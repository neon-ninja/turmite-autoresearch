//! kappa_tail_budget_meanfield SEED0 NSEEDS CAP LO-HI,LO-HI,... OUT.csv THREADS
//!
//! Known-answer calibration of the tail-budget renewal estimator. The walker is
//! the mean-field one of kappa_uniform_baseline (resample uniformly over visited
//! cells each step, then step to a uniform 4-neighbour), so its instantaneous
//! boundary occupation is exactly P/A and kappa_inst must come out at 1. The
//! state stream feeds the real `Accountant` and `Tails`; the Walker trait cannot
//! teleport, hence the separate driver. Row per (seed, band).
use rayon::prelude::*;
use std::io::Write;
use turmite::field_age::Tails;
use turmite::fixed_area::{Accountant, Region, State};
use turmite::walkers::splitmix_next;

const SIZE: usize = 4096;

struct Row {
    seed: u64,
    band: usize,
    reached_hi: bool,
    steps: u64,
    b_dep: u64,
    sum_pa: f64,
    b_n: u64,
    b_steps: u64,
    t_n: u64,
    t_steps: u64,
    unresolved: u64,
    t_final: u64,
    bins_n: [u64; 64],
    bins_steps: [u64; 64],
}

fn run(seed: u64, cap: u64, bands: &[(u64, u64)]) -> Vec<Row> {
    let c = (SIZE / 2) as i64;
    let nbr: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut region = Region::new(SIZE);
    let mut cells: Vec<usize> = Vec::new();
    let mut st = seed.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ 0x2545_F491_4F6C_DD1D;
    let mut acc = Accountant::new(bands);
    let mut tails = Tails::new(bands);
    let start = (c as usize) * SIZE + c as usize;
    region.visit(start, 0, 0);
    cells.push(start);
    let mut t = 0u64;
    loop {
        let r = splitmix_next(&mut st);
        let idx = cells[((r >> 8) % cells.len() as u64) as usize];
        let state = State { t, z: region.on_boundary(idx), area: region.area, perimeter: region.perimeter, geom: region.geom() };
        acc.feed(&state);
        tails.feed(state);
        if (acc.done() && tails.done()) || t == cap {
            break;
        }
        let (x, y) = ((idx % SIZE) as i64 - c, (idx / SIZE) as i64 - c);
        let (dx, dy) = nbr[(r & 3) as usize];
        let (nx, ny) = (x + dx, y + dy);
        assert!(nx.abs() < c - 2 && ny.abs() < c - 2, "board escape");
        let j = ((ny + c) as usize) * SIZE + (nx + c) as usize;
        if !region.visited(j) {
            region.visit(j, nx, ny);
            cells.push(j);
        }
        t += 1;
    }
    let bo = acc.finish();
    let tl = tails.finish();
    bo.iter()
        .zip(tl.iter())
        .enumerate()
        .map(|(k, (b, tl))| Row {
            seed,
            band: k,
            reached_hi: b.reached_hi,
            steps: b.steps,
            b_dep: b.b_dep,
            sum_pa: b.sum_pa,
            b_n: b.b.complete,
            b_steps: b.b.complete_steps,
            t_n: tl.entered_n.iter().sum(),
            t_steps: tl.entered_steps.iter().sum(),
            unresolved: tl.unresolved_overlap,
            t_final: t,
            bins_n: tl.entered_n,
            bins_steps: tl.entered_steps,
        })
        .collect()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    assert_eq!(a.len(), 7, "SEED0 NSEEDS CAP LO-HI,... OUT.csv THREADS");
    let (s0, n, cap): (u64, u64, u64) = (a[1].parse().unwrap(), a[2].parse().unwrap(), a[3].parse().unwrap());
    let bands: Vec<(u64, u64)> = a[4]
        .split(',')
        .map(|b| {
            let (l, h) = b.split_once('-').unwrap();
            (l.parse().unwrap(), h.parse().unwrap())
        })
        .collect();
    rayon::ThreadPoolBuilder::new().num_threads(a[6].parse().unwrap()).build_global().unwrap();
    // Results are keyed by seed and collected in input order, so thread count cannot change the bytes.
    let rows: Vec<Vec<Row>> = (0..n).into_par_iter().map(|i| run(s0 + i, cap, &bands)).collect();
    let mut f = std::fs::File::create(&a[5]).unwrap();
    writeln!(f, "seed,band,reached_hi,steps,b_dep,sum_pa,b_complete_n,b_complete_steps,t_entered_n,t_entered_steps,unresolved_overlap,t_final").unwrap();
    for r in rows.iter().flatten() {
        writeln!(
            f,
            "{},{},{},{},{},{:.17e},{},{},{},{},{},{}",
            r.seed, r.band, r.reached_hi as u8, r.steps, r.b_dep, r.sum_pa, r.b_n, r.b_steps, r.t_n, r.t_steps, r.unresolved, r.t_final
        )
        .unwrap();
    }
    // Per-log2-bin entered-episode counts and time sums (the inputs of the survival-family fit).
    let mut f = std::fs::File::create(a[5].replace(".csv", ".bins.csv")).unwrap();
    writeln!(f, "seed,band,bin,entry_count,entry_time_sum").unwrap();
    for r in rows.iter().flatten() {
        for b in 0..64 {
            if r.bins_n[b] > 0 {
                writeln!(f, "{},{},{},{},{}", r.seed, r.band, b, r.bins_n[b], r.bins_steps[b]).unwrap();
            }
        }
    }
}
