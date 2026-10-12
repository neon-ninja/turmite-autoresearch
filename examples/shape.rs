//! Territory shape statistics (frontier items 5 and 7).
//!
//! usage: shape MAX_LOG2 [GRID] < specs.txt > shape.csv
//! Each spec line: an ant rule, `RULE@SEED` (random 5x5 patch), `rotor:SEED`.
//! One `cp` row per (spec, checkpoint t = 2^k, k = 12..=MAX_LOG2 step 2):
//!   A      visited cells                     rg2    radius of gyration^2 of the visited set
//!   aniso  (l1-l2)/(l1+l2) of the visited set's second-moment tensor (0 = isotropic)
//!   rbar   mean over 128 angular bins of the outermost visited cell's distance from the centroid
//!   rsd    sd of that profile / rbar      h2,h4,h8  relative amplitude of angular harmonics m = 2, 4, 8
//!   Wsurf  boundary width: sd of the radius (about the centroid) of surface cells (K&D definition)
//!   fill   A / (pi rbar^2)
//! One `prof` row per spec at t = 2^MAX_LOG2: mean visits per cell in 20 radial shells of width
//! 0.075 * R_eq (R_eq = sqrt(A/pi)), the time-density profile in units of t / A.
//! Deterministic: the only randomness is the seed in the spec.
use rayon::prelude::*;
use std::io::Write;
use turmite::walkers::{AntWalker, PatchedAnt, Rotor, Walker};
use turmite::{DIRS, Rule};

fn run<W: Walker>(mut w: W, label: &str, size: usize, max_log2: u32) -> String {
    let c = (size / 2) as i64;
    let mut colour = vec![0u8; size * size];
    if W::RANDOM_INIT {
        for (i, cell) in colour.iter_mut().enumerate() {
            *cell = w.initial_cell((i % size) as i64 - c, (i / size) as i64 - c);
        }
    }
    let mut visits = vec![0u32; size * size];
    let s = size as isize;
    let (mut x, mut y, mut h) = (0i64, 0i64, 0u8);
    let mut idx = (c * size as i64 + c) as usize;
    visits[idx] = 1;
    let mut out = String::new();
    let mut t: u64 = 0;
    let mut k = 12;
    while k <= max_log2 {
        let target = 1u64 << k;
        while t < target {
            w.step(&mut colour[idx], &mut h);
            let (dx, dy) = DIRS[h as usize];
            x += dx as i64;
            y += dy as i64;
            idx = (idx as isize + dy as isize * s + dx as isize) as usize;
            visits[idx] += 1;
            t += 1;
            if x.abs() >= c - 2 || y.abs() >= c - 2 {
                return out + &format!("# {label} escaped at t={t}\n");
            }
        }
        out += &checkpoint(&visits, size, label, k, k == max_log2);
        k += 2;
    }
    out
}

fn checkpoint(visits: &[u32], size: usize, label: &str, k: u32, profile: bool) -> String {
    let (mut a, mut sx, mut sy, mut sxx, mut syy, mut sxy) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
    for y in 0..size {
        for x in 0..size {
            if visits[y * size + x] > 0 {
                let (fx, fy) = (x as f64, y as f64);
                a += 1.0;
                sx += fx;
                sy += fy;
                sxx += fx * fx;
                syy += fy * fy;
                sxy += fx * fy;
            }
        }
    }
    let (mx, my) = (sx / a, sy / a);
    let (vxx, vyy, vxy) = (sxx / a - mx * mx, syy / a - my * my, sxy / a - mx * my);
    let tr = vxx + vyy;
    let disc = (((vxx - vyy) / 2.0).powi(2) + vxy * vxy).sqrt();
    let (l1, l2) = (tr / 2.0 + disc, tr / 2.0 - disc);
    let aniso = (l1 - l2) / (l1 + l2);
    const NB: usize = 128;
    let mut rmax = [0f64; NB];
    let (mut ssum, mut s2, mut ns) = (0f64, 0f64, 0f64);
    for y in 1..size - 1 {
        for x in 1..size - 1 {
            let i = y * size + x;
            if visits[i] == 0 {
                continue;
            }
            let (fx, fy) = (x as f64 - mx, y as f64 - my);
            let r = (fx * fx + fy * fy).sqrt();
            let th = fy.atan2(fx) + std::f64::consts::PI;
            let b = ((th / (2.0 * std::f64::consts::PI)) * NB as f64) as usize % NB;
            if r > rmax[b] {
                rmax[b] = r;
            }
            if visits[i - 1] == 0 || visits[i + 1] == 0 || visits[i - size] == 0 || visits[i + size] == 0 {
                ssum += r;
                s2 += r * r;
                ns += 1.0;
            }
        }
    }
    let wsurf = (s2 / ns - (ssum / ns).powi(2)).max(0.0).sqrt();
    let rbar = rmax.iter().sum::<f64>() / NB as f64;
    let rsd = (rmax.iter().map(|r| (r - rbar).powi(2)).sum::<f64>() / NB as f64).sqrt() / rbar;
    let harm = |m: usize| {
        let (mut re, mut im) = (0.0, 0.0);
        for (b, r) in rmax.iter().enumerate() {
            let ang = 2.0 * std::f64::consts::PI * m as f64 * (b as f64 + 0.5) / NB as f64;
            re += r * ang.cos();
            im += r * ang.sin();
        }
        2.0 * (re * re + im * im).sqrt() / NB as f64 / rbar
    };
    let fill = a / (std::f64::consts::PI * rbar * rbar);
    let rg2 = tr;
    let mut o = format!(
        "cp,{label},{k},{a},{rg2:.3},{aniso:.5},{rbar:.3},{rsd:.5},{:.5},{:.5},{:.5},{wsurf:.4},{fill:.5}\n",
        harm(2),
        harm(4),
        harm(8)
    );
    if profile {
        let req = (a / std::f64::consts::PI).sqrt();
        let mut sum = [0f64; 20];
        let mut cnt = [0f64; 20];
        for y in 0..size {
            for x in 0..size {
                let (fx, fy) = (x as f64 - mx, y as f64 - my);
                let b = ((fx * fx + fy * fy).sqrt() / (0.075 * req)) as usize;
                if b < 20 {
                    sum[b] += visits[y * size + x] as f64;
                    cnt[b] += 1.0;
                }
            }
        }
        let t = (1u64 << k) as f64;
        let prof: Vec<String> = (0..20).map(|b| format!("{:.4}", sum[b] / cnt[b].max(1.0) / (t / a))).collect();
        o += &format!("prof,{label},{k},{}\n", prof.join(";"));
    }
    o
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let max_log2: u32 = a[1].parse().unwrap();
    let size: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(4096);
    let specs: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
    let blocks: Vec<String> = specs
        .par_iter()
        .map(|r| {
            if let Some(seed) = r.strip_prefix("rotor:") {
                run(Rotor { seed: seed.parse().unwrap() }, r, size, max_log2)
            } else if let Some((rule, seed)) = r.split_once('@') {
                run(PatchedAnt::new(&rule.parse::<Rule>().unwrap(), seed.parse().unwrap(), 2), r, size, max_log2)
            } else {
                run(AntWalker::new(&r.parse::<Rule>().unwrap()), r, size, max_log2)
            }
        })
        .collect();
    let mut out = std::io::stdout().lock();
    writeln!(out, "kind,spec,k,A,rg2,aniso,rbar,rsd,h2,h4,h8,Wsurf,fill").unwrap();
    for b in blocks {
        out.write_all(b.as_bytes()).unwrap();
    }
}
