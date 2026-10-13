//! True self-avoiding walk on the square lattice (E1): from the current cell choose one of the 4
//! neighbours with weight exp(-g * visits(neighbour)). Comparison walker for "is stochastic board
//! memory enough for the cube root?" (notes/frontier-2026-10-13-results.md, E1).
//!
//! usage: tsaw MAX_LOG2 [GRID] < specs.txt > tsaw.csv     (spec: `G:SEED`, g = 0 is the memoryless random walk)
//! One row per (spec, k) for k = 12..=MAX_LOG2 (octave windows): A visited cells, P cells with an unvisited
//! 4-neighbour, S steps in the window, B steps in the window made from such an edge cell, kappa = (B/S)/(P/A)
//! with P, A at the window end (the probe's convention). Deterministic given the seed (splitmix64 stream).
use rayon::prelude::*;
use std::io::Write;
use turmite::walkers::mix64;

fn run(g: f64, seed: u64, size: usize, max_log2: u32) -> String {
    let w: Vec<f64> = (0..64).map(|v| (-g * v as f64).exp()).collect();
    let s = size as isize;
    let mut visits = vec![0u8; size * size];
    let mut idx = (size / 2 * size + size / 2) as isize;
    visits[idx as usize] = 1;
    let mut area: u64 = 1;
    let mut rng = mix64(seed ^ 0x2545_F491_4F6C_DD1D);
    let nb = [-s, 1, s, -1];
    let (mut t, mut b_win, mut k) = (0u64, 0u64, 12u32);
    let mut out = String::new();
    let lim = (size / 2 - 2) as isize;
    let mut prev_t = 0u64;
    while k <= max_log2 {
        let target = 1u64 << k;
        while t < target {
            // edge flag of the current cell, before moving
            let i = idx as usize;
            if nb.iter().any(|d| visits[(idx + d) as usize] == 0) { b_win += 1; }
            let mut ws = [0f64; 4];
            let mut tot = 0.0;
            for (j, d) in nb.iter().enumerate() { ws[j] = w[visits[(idx + d) as usize].min(63) as usize]; tot += ws[j]; }
            rng = rng.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut u = (mix64(rng) >> 11) as f64 / (1u64 << 53) as f64 * tot;
            let mut pick = 3;
            for j in 0..4 { if u < ws[j] { pick = j; break; } u -= ws[j]; }
            let _ = i;
            idx += nb[pick];
            let c = idx as usize;
            if visits[c] == 0 { area += 1; }
            visits[c] = visits[c].saturating_add(1);
            t += 1;
            let (x, y) = (idx % s - s / 2, idx / s - s / 2);
            if x.abs() >= lim || y.abs() >= lim { return out + &format!("# {g}:{seed} escaped at t={t}\n"); }
        }
        let mut p = 0u64;
        for y in 1..size - 1 { for x in 1..size - 1 {
            let c = y * size + x;
            if visits[c] > 0 && nb.iter().any(|d| visits[(c as isize + d) as usize] == 0) { p += 1; }
        } }
        let sw = (t - prev_t) as f64;
        let kappa = (b_win as f64 / sw) / (p as f64 / area as f64);
        out += &format!("{g}:{seed},{k},{t},{area},{p},{},{b_win},{kappa:.4}\n", t - prev_t);
        prev_t = t; b_win = 0; k += 1;
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let max_log2: u32 = a[1].parse().unwrap();
    let size: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(4096);
    let specs: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
    let blocks: Vec<String> = specs.par_iter().map(|sp| {
        let (g, seed) = sp.split_once(':').unwrap();
        run(g.parse().unwrap(), seed.parse().unwrap(), size, max_log2)
    }).collect();
    let mut o = std::io::stdout().lock();
    writeln!(o, "spec,k,t,A,P,S,B,kappa").unwrap();
    for b in blocks { write!(o, "{b}").unwrap(); }
}
