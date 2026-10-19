//! rulepat_scan LOG2_STEPS [SIZE] < walkers.txt > out.csv
//! One row of "plateau" descriptors per walker (rule, `rotor:SEED`, `rw:SEED`), in input order.
//! Visit counts are kept per cell (u16, saturating); descriptors are computed in-process so
//! nothing bulky is written. Deterministic regardless of thread count (rows keyed by input line).
//! Columns: steps, escaped, area, width, height, area_q (area at t/4, for a local exponent),
//!   mode_visits (m), plateau (share of visited cells with m visits), plateau_colour (modal
//!   colour/state among those cells), plateau2 (share with <= 2 visits), mean_visits,
//!   max_visits, alt_frac (steps whose turn differs from the previous step's, ants only),
//!   long_alt_runs (maximal alternating-turn runs of length >= 32, ants only).
use rayon::prelude::*;
use std::io::Write;
use turmite::walkers::{AntWalker, RandomWalk, Rotor, Walker};
use turmite::{DIRS, Rule};

fn run<W: Walker>(mut w: W, size: usize, steps: u64, ant_turns: Option<&[u8]>, n_state: usize) -> String {
    let c = (size / 2) as i64;
    let mut col = vec![0u8; size * size];
    if W::RANDOM_INIT {
        for (i, cell) in col.iter_mut().enumerate() {
            *cell = w.initial_cell((i % size) as i64 - c, (i / size) as i64 - c);
        }
    }
    let mut vis = vec![0u16; size * size];
    let s = size as isize;
    let (mut x, mut y, mut h) = (c, c, 0u8);
    let (mut minx, mut maxx, mut miny, mut maxy) = (x, x, y, y);
    let mut area = 1u64;
    let mut area_q = 0u64;
    let mut idx = (y * size as i64 + x) as usize;
    vis[idx] = 1;
    let (mut alt, mut run, mut long_runs, mut prev_turn) = (0u64, 0u64, 0u64, 0u8);
    let mut done = steps;
    let mut escaped = false;
    for t in 1..=steps {
        let before = h;
        w.step(&mut col[idx], &mut h);
        if ant_turns.is_some() {
            let d = (h + 4 - before) & 3;
            if d != prev_turn { alt += 1; run += 1 } else { if run >= 32 { long_runs += 1 } run = 0 }
            prev_turn = d;
        }
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if x < 0 || y < 0 || x >= size as i64 || y >= size as i64 { escaped = true; done = t; break; }
        idx = (idx as isize + dy as isize * s + dx as isize) as usize;
        if vis[idx] == 0 { area += 1; minx = minx.min(x); maxx = maxx.max(x); miny = miny.min(y); maxy = maxy.max(y); }
        vis[idx] = vis[idx].saturating_add(1);
        if t == steps / 4 { area_q = area; }
    }
    // The ant's final cell was entered but its state not yet updated: harmless for a descriptor.
    let mut hist = vec![0u64; 65536];
    for &v in &vis { if v > 0 { hist[v as usize] += 1 } }
    let (m, &cnt) = hist.iter().enumerate().skip(1).max_by_key(|&(i, &c)| (c, std::cmp::Reverse(i))).unwrap();
    let mut ctally = vec![0u64; n_state.max(4)];
    for (i, &v) in vis.iter().enumerate() { if v as usize == m { ctally[col[i] as usize] += 1 } }
    let pc = ctally.iter().enumerate().max_by_key(|&(i, &c)| (c, std::cmp::Reverse(i))).unwrap().0;
    let le2 = hist[1] + hist[2];
    let total: u64 = hist.iter().enumerate().map(|(i, &c)| i as u64 * c).sum();
    let maxv = hist.iter().rposition(|&c| c > 0).unwrap();
    format!("{done},{escaped},{area},{},{},{area_q},{m},{:.4},{pc},{:.4},{:.3},{maxv},{:.4},{long_runs}",
        maxx - minx + 1, maxy - miny + 1, cnt as f64 / area as f64, le2 as f64 / area as f64,
        total as f64 / area as f64, alt as f64 / done as f64)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let steps = 1u64 << a[1].parse::<u32>().unwrap();
    let size: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(8192);
    let lines: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
    let rows: Vec<String> = lines.par_iter().map(|r| {
        let body = if let Some(sd) = r.strip_prefix("rotor:") {
            run(Rotor { seed: sd.parse().unwrap() }, size, steps, None, 4)
        } else if let Some(sd) = r.strip_prefix("rw:") {
            run(RandomWalk { state: sd.parse().unwrap() }, size, steps, None, 4)
        } else {
            let rule: Rule = r.parse().unwrap();
            let n = rule.len();
            let turns: Vec<u8> = rule.0.iter().map(|t| t.delta()).collect();
            run(AntWalker::new(&rule), size, steps, Some(&turns), n)
        };
        format!("{r},{body}\n")
    }).collect();
    let mut o = std::io::stdout().lock();
    writeln!(o, "rule,steps,escaped,area,width,height,area_q,mode_visits,plateau,plateau_colour,plateau2,mean_visits,max_visits,alt_frac,long_alt_runs").unwrap();
    for r in rows { o.write_all(r.as_bytes()).unwrap() }
}
