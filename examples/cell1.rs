//! Item 10: single-cell refinement of the knockout map. Does not touch the knockout code.
//!
//! cell1 GRIDFILE RULE STEPS OUTPREFIX X1,Y1 X2,Y2 ...
//!   Windows are 3x3 top-left corners relative to home (as in knock.rs). Protocol of knock.rs: fresh ant at
//!   home heading north on the dumped field (255 = blank), STEPS steps, near_home over the last quarter of 64 blocks.
//!   Writes OUTPREFIX_scan.csv   x,y,old,new,near_home        every visited cell within 1 of a window, every other colour
//!          OUTPREFIX_null.csv   x,y,old,new,near_home        200 random visited cells, random other colour (seed fixed)
//!          OUTPREFIX_greedy.csv k,x,y,new,near_home          cumulative: the k most damaging cells each at its worst colour
//!          OUTPREFIX_lone.csv   x,y,old,new,near_home        second greedy round: top cell fixed at its worst colour, rescan all scan cells
use rayon::prelude::*;
use std::io::Write;
use turmite::{DIRS, Rule};

const S: usize = 4096;
const C: i64 = (S / 2) as i64;

fn near_home(colour: &mut [u8], rule: &Rule, steps: u64) -> f64 {
    let n = rule.len() as u8;
    let (mut x, mut y, mut h) = (C, C, 0u8);
    let block = (steps / 64).max(1);
    let (mut nh, mut acc, mut nb) = (0u64, 0f64, 0f64);
    for t in 1..=steps {
        let i = (y as usize) * S + x as usize;
        let col = colour[i];
        h = (h + rule.0[col as usize].delta()) & 3;
        colour[i] = (col + 1) % n;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if x <= 0 || y <= 0 || x >= S as i64 - 1 || y >= S as i64 - 1 {
            return f64::NAN;
        }
        let (rx, ry) = (x - C, y - C);
        nh += (rx * rx + ry * ry <= 100) as u64;
        if t % block == 0 {
            if t > block * 48 {
                acc += nh as f64 / block as f64;
                nb += 1.0;
            }
            nh = 0;
        }
    }
    acc / nb
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn idx(x: i64, y: i64) -> usize {
    ((C + y) as usize) * S + (C + x) as usize
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read(&a[1]).unwrap();
    let rule: Rule = a[2].parse().unwrap();
    let steps: u64 = a[3].parse().unwrap();
    let pre = &a[4];
    let n = rule.len() as u8;
    let base: Vec<u8> = raw.iter().map(|&c| if c == 255 { 0 } else { c }).collect();
    let run = |edits: &[(usize, u8)]| -> f64 {
        let mut c = base.clone();
        for &(i, v) in edits {
            c[i] = v;
        }
        near_home(&mut c, &rule, steps)
    };
    let b0 = run(&[]);
    println!("baseline near_home {b0:.4}");

    let mut cells: Vec<(i64, i64)> = vec![];
    for w in &a[5..] {
        let (wx, wy) = w.split_once(',').unwrap();
        let (wx, wy): (i64, i64) = (wx.parse().unwrap(), wy.parse().unwrap());
        for y in wy - 1..wy + 4 {
            for x in wx - 1..wx + 4 {
                if raw[idx(x, y)] != 255 && !cells.contains(&(x, y)) {
                    cells.push((x, y));
                }
            }
        }
    }
    let jobs: Vec<(i64, i64, u8, u8)> = cells
        .iter()
        .flat_map(|&(x, y)| {
            let old = raw[idx(x, y)];
            (0..n).filter(move |&v| v != old).map(move |v| (x, y, old, v))
        })
        .collect();
    let res: Vec<f64> = jobs.par_iter().map(|&(x, y, _, v)| run(&[(idx(x, y), v)])).collect();
    let mut f = std::fs::File::create(format!("{pre}_scan.csv")).unwrap();
    writeln!(f, "x,y,old,new,near_home").unwrap();
    for (j, r) in jobs.iter().zip(&res) {
        writeln!(f, "{},{},{},{},{:.4}", j.0, j.1, j.2, j.3, r).unwrap();
    }

    // null: random visited cells anywhere, random other colour
    let visited: Vec<usize> = (0..S * S).filter(|&i| raw[i] != 255).collect();
    let mut st = 0x5EED_u64;
    let nulljobs: Vec<(usize, u8)> = (0..200)
        .map(|_| {
            let i = visited[(splitmix(&mut st) % visited.len() as u64) as usize];
            let old = raw[i];
            let v = ((old as u64 + 1 + splitmix(&mut st) % (n as u64 - 1)) % n as u64) as u8;
            (i, v)
        })
        .collect();
    let nres: Vec<f64> = nulljobs.par_iter().map(|&(i, v)| run(&[(i, v)])).collect();
    let mut f = std::fs::File::create(format!("{pre}_null.csv")).unwrap();
    writeln!(f, "x,y,old,new,near_home").unwrap();
    for ((i, v), r) in nulljobs.iter().zip(&nres) {
        let (x, y) = ((*i % S) as i64 - C, (*i / S) as i64 - C);
        writeln!(f, "{x},{y},{},{v},{r:.4}", raw[*i]).unwrap();
    }

    // greedy cumulative: each cell at its worst single recolouring, ranked by that loss
    let mut best: Vec<(f64, i64, i64, u8)> = vec![];
    for &(x, y) in &cells {
        let (mut w, mut wv) = (f64::INFINITY, 0u8);
        for (j, r) in jobs.iter().zip(&res) {
            if (j.0, j.1) == (x, y) && *r < w {
                w = *r;
                wv = j.3;
            }
        }
        best.push((w, x, y, wv));
    }
    best.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap());
    let ks: Vec<usize> = (1..=best.len().min(15)).collect();
    let gres: Vec<f64> = ks
        .par_iter()
        .map(|&k| {
            let e: Vec<(usize, u8)> = best[..k].iter().map(|b| (idx(b.1, b.2), b.3)).collect();
            run(&e)
        })
        .collect();
    let mut f = std::fs::File::create(format!("{pre}_greedy.csv")).unwrap();
    writeln!(f, "k,x,y,new,near_home").unwrap();
    for (k, r) in ks.iter().zip(&gres) {
        let b = &best[k - 1];
        writeln!(f, "{k},{},{},{},{r:.4}", b.1, b.2, b.3).unwrap();
    }

    // second round: fix the single most damaging recolouring, rescan every other scan cell
    let top = &best[0];
    let top_e = (idx(top.1, top.2), top.3);
    let jobs2: Vec<&(i64, i64, u8, u8)> = jobs.iter().filter(|j| (j.0, j.1) != (top.1, top.2)).collect();
    let res2: Vec<f64> = jobs2.par_iter().map(|j| run(&[top_e, (idx(j.0, j.1), j.3)])).collect();
    let mut f = std::fs::File::create(format!("{pre}_lone.csv")).unwrap();
    writeln!(f, "x,y,old,new,near_home").unwrap();
    for (j, r) in jobs2.iter().zip(&res2) {
        writeln!(f, "{},{},{},{},{:.4}", j.0, j.1, j.2, j.3, r).unwrap();
    }
}
