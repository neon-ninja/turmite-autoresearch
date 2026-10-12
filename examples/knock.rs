//! knock GRIDFILE RULE STEPS base OUTDIR
//! knock GRIDFILE RULE STEPS jobs JOBFILE OUT.csv
//!
//! Knockout runs on a final colour field written by examples/dump.rs (255 =
//! never visited, treated as blank). A fresh ant starts at home heading
//! north, as in examples/transplant.rs, and runs STEPS steps.
//!
//! base: run the intact field; write OUTDIR/occ.u32 and OUTDIR/route.u32
//!   (4096^2 little-endian step counts over the last half of the run; route
//!   counts only steps spent outside r <= 10 of home, i.e. on excursions)
//!   and print the outcome row.
//! jobs: JOBFILE has lines `kind,x,y,w,h,seed` (x,y relative to home, top-left
//!   corner). kind = patch: permute the colours of the visited cells inside the
//!   w*h rectangle among themselves. kind = swap: pick w*h visited cells
//!   uniformly at random anywhere and permute them among themselves (x,y
//!   ignored). Jobs run in parallel; one CSV row each, in job order.
//!
//! Outcome columns, over the last 16 of 64 blocks: near_home (share of time
//! within 10 cells of home), own (within 10 of the previous block's mean),
//! spread (RMS distance from the block's own mean), mean position.
use rayon::prelude::*;
use std::io::Write;
use turmite::{DIRS, Rule};

const S: usize = 4096;
const C: i64 = (S / 2) as i64;

struct Out {
    near_home: f64,
    own: f64,
    spread: f64,
    mx: f64,
    my: f64,
    ok: bool,
}

fn run(colour: &mut [u8], rule: &Rule, steps: u64, mut counts: Option<(&mut Vec<u32>, &mut Vec<u32>)>) -> Out {
    let n = rule.len() as u8;
    let (mut x, mut y, mut h) = (C, C, 0u8);
    let block = (steps / 64).max(1);
    let (mut sx, mut sy, mut sr2) = (0f64, 0f64, 0f64);
    let (mut nh, mut np) = (0u64, 0u64);
    let mut prev = (0f64, 0f64);
    let (mut a_nh, mut a_np, mut a_sp, mut a_mx, mut a_my, mut nb) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
    for t in 1..=steps {
        let i = (y as usize) * S + x as usize;
        let col = colour[i];
        h = (h + rule.0[col as usize].delta()) & 3;
        colour[i] = (col + 1) % n;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if x <= 0 || y <= 0 || x >= S as i64 - 1 || y >= S as i64 - 1 {
            return Out { near_home: f64::NAN, own: f64::NAN, spread: f64::NAN, mx: f64::NAN, my: f64::NAN, ok: false };
        }
        let (rx, ry) = (x - C, y - C);
        let r2 = rx * rx + ry * ry;
        sx += rx as f64;
        sy += ry as f64;
        sr2 += r2 as f64;
        nh += (r2 <= 100) as u64;
        np += ((rx as f64 - prev.0).powi(2) + (ry as f64 - prev.1).powi(2) <= 100.0) as u64;
        if let Some((occ, route)) = counts.as_mut() {
            if t > steps / 2 {
                let j = (y as usize) * S + x as usize;
                occ[j] += 1;
                if r2 > 100 {
                    route[j] += 1;
                }
            }
        }
        if t % block == 0 {
            let b = block as f64;
            if t > block * 48 {
                a_nh += nh as f64 / b;
                a_np += np as f64 / b;
                a_sp += (sr2 / b - (sx / b).powi(2) - (sy / b).powi(2)).max(0.0).sqrt();
                a_mx += sx / b;
                a_my += sy / b;
                nb += 1.0;
            }
            prev = (sx / b, sy / b);
            (sx, sy, sr2, nh, np) = (0.0, 0.0, 0.0, 0, 0);
        }
    }
    Out { near_home: a_nh / nb, own: a_np / nb, spread: a_sp / nb, mx: a_mx / nb, my: a_my / nb, ok: true }
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn shuffle<T>(v: &mut [T], st: &mut u64) {
    for i in (1..v.len()).rev() {
        let j = (splitmix(st) % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
}

fn load(raw: &[u8]) -> Vec<u8> {
    raw.iter().map(|&c| if c == 255 { 0 } else { c }).collect()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read(&a[1]).unwrap();
    let rule: Rule = a[2].parse().unwrap();
    let steps: u64 = a[3].parse().unwrap();
    let hdr = "near_home,own,spread,mx,my";
    match a[4].as_str() {
        "base" => {
            let mut colour = load(&raw);
            let mut occ = vec![0u32; S * S];
            let mut route = vec![0u32; S * S];
            let o = run(&mut colour, &rule, steps, Some((&mut occ, &mut route)));
            let dir = std::path::Path::new(&a[5]);
            for (name, v) in [("occ.u32", &occ), ("route.u32", &route)] {
                let bytes: Vec<u8> = v.iter().flat_map(|c| c.to_le_bytes()).collect();
                std::fs::File::create(dir.join(name)).unwrap().write_all(&bytes).unwrap();
            }
            println!("{hdr}\n{:.4},{:.4},{:.3},{:.2},{:.2}", o.near_home, o.own, o.spread, o.mx, o.my);
        }
        "jobs" => {
            let jobs: Vec<(String, i64, i64, i64, i64, u64)> = std::fs::read_to_string(&a[5])
                .unwrap()
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| {
                    let f: Vec<&str> = l.split(',').collect();
                    (f[0].to_string(), f[1].parse().unwrap(), f[2].parse().unwrap(), f[3].parse().unwrap(), f[4].parse().unwrap(), f[5].parse().unwrap())
                })
                .collect();
            let visited: Vec<usize> = (0..S * S).filter(|&i| raw[i] != 255).collect();
            let base = load(&raw);
            let rows: Vec<String> = jobs
                .par_iter()
                .map(|(kind, x, y, w, h, seed)| {
                    let mut st = seed.wrapping_mul(0x2545F4914F6CDD1D) ^ ((*x as u64) << 20) ^ ((*y as u64) << 40) ^ 0xABCDEF;
                    let idx: Vec<usize> = if kind == "patch" {
                        let mut v = vec![];
                        for yy in *y..*y + *h {
                            for xx in *x..*x + *w {
                                let i = ((C + yy) as usize) * S + (C + xx) as usize;
                                if raw[i] != 255 {
                                    v.push(i);
                                }
                            }
                        }
                        v
                    } else {
                        let cnt = (*w * *h) as usize;
                        let mut pool = visited.clone();
                        for k in 0..cnt {
                            let j = k + (splitmix(&mut st) % (pool.len() - k) as u64) as usize;
                            pool.swap(k, j);
                        }
                        pool.truncate(cnt);
                        pool
                    };
                    let mut colour = base.clone();
                    let mut vals: Vec<u8> = idx.iter().map(|&i| colour[i]).collect();
                    shuffle(&mut vals, &mut st);
                    let mut changed = 0usize;
                    for (k, &i) in idx.iter().enumerate() {
                        changed += (colour[i] != vals[k]) as usize;
                        colour[i] = vals[k];
                    }
                    let o = run(&mut colour, &rule, steps, None);
                    format!(
                        "{kind},{x},{y},{w},{h},{seed},{},{:.4},{:.4},{:.4},{:.3},{:.2},{:.2},{}",
                        idx.len(),
                        changed as f64 / idx.len().max(1) as f64,
                        o.near_home,
                        o.own,
                        o.spread,
                        o.mx,
                        o.my,
                        o.ok as u8
                    )
                })
                .collect();
            let mut f = std::fs::File::create(&a[6]).unwrap();
            writeln!(f, "kind,x,y,w,h,seed,nvis,changed,{hdr},ok").unwrap();
            for r in rows {
                writeln!(f, "{r}").unwrap();
            }
        }
        m => panic!("unknown mode {m}"),
    }
}
