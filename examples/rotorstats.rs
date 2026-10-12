//! rotorstats SEEDS SEED0 OUTDIR [MAXN] [alt] : reproduce Kapri & Dhar (2009) measurements
//! for the rotor (Eulerian) walker with i.i.d. uniform initial arrows.
//!
//! For each seed, runs to N = MAXN (default 1e7, at least 1e6) on a 2048 board and records at checkpoints
//! N in {1e4, 3e4, 1e5, 3e5, 1e6, 3e6, 1e7}:
//!   R2  squared distance of the walker from the origin
//!   S   number of distinct visited sites
//!   W   boundary width: sqrt(mean over surface sites (|x| - <R>)^2), surface
//!       = visited sites with an unvisited 4-neighbour, <R> their mean |x|
//! and, at N = 1e6, adds this seed's visit counts n_N(x) (arrivals after each
//! step; the starting site is not counted as a visit) into a running sum, so
//! OUTDIR/visits_1e6.f64 holds sum over seeds of n_N(x) on a 1024 x 1024 crop.
//! Also at N = 1e6: the fraction of visited sites visited exactly 4 times in
//! the last 4 S steps (the Priezzhev heuristic), via a second pass.
//! Writes OUTDIR/checkpoints.csv (one row per seed and checkpoint).
use rayon::prelude::*;
use std::io::Write;
use turmite::DIRS;
use turmite::walkers::initial_arrow;

const B: usize = 2048;
const CROP: usize = 1024;
const CHECKS: [u64; 7] = [10_000, 30_000, 100_000, 300_000, 1_000_000, 3_000_000, 10_000_000];

struct Board {
    arrow: Vec<u8>,
    visits: Vec<u32>,
}

/// Use a different arrow generator (top two bits of a SplitMix64 stream in
/// raster order), to check results don't depend on the generator.
static ALT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

impl Board {
    fn new(seed: u64) -> Board {
        let c = (B / 2) as i64;
        let arrow = if ALT.load(std::sync::atomic::Ordering::Relaxed) {
            let mut st = seed.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ 0x8CB9_2BA7_2F3D_8DD7;
            (0..B * B).map(|_| (turmite::walkers::splitmix_next(&mut st) >> 62) as u8).collect()
        } else {
            (0..B * B).map(|i| initial_arrow(seed, (i % B) as i64 - c, (i / B) as i64 - c)).collect()
        };
        Board { arrow, visits: vec![0; B * B] }
    }
}

fn boundary_width(visits: &[u32]) -> f64 {
    let c = (B / 2) as f64;
    let mut rs = vec![];
    for y in 1..B - 1 {
        for x in 1..B - 1 {
            let i = y * B + x;
            if visits[i] == 0 {
                continue;
            }
            if visits[i - 1] == 0 || visits[i + 1] == 0 || visits[i - B] == 0 || visits[i + B] == 0 {
                rs.push(((x as f64 - c).powi(2) + (y as f64 - c).powi(2)).sqrt());
            }
        }
    }
    let m = rs.iter().sum::<f64>() / rs.len() as f64;
    (rs.iter().map(|r| (r - m).powi(2)).sum::<f64>() / rs.len() as f64).sqrt()
}

/// Run one seed; returns checkpoint rows, its visit counts at 1e6 (cropped)
/// and the exact-4 fraction.
fn run(seed: u64, maxn: u64) -> (Vec<String>, Vec<u32>, f64) {
    let mut b = Board::new(seed);
    let (mut x, mut y) = (B / 2, B / 2);
    let mut s: u64 = 1; // the origin counts as visited (as a site), not as a visit
    let mut rows = vec![];
    let mut crop = vec![];
    let mut s_at_1e6 = 0;
    let mut ci = 0;
    for t in 1..=maxn {
        let i = y * B + x;
        b.arrow[i] = (b.arrow[i] + 1) & 3;
        let (dx, dy) = DIRS[b.arrow[i] as usize];
        x = x.wrapping_add_signed(dx as isize);
        y = y.wrapping_add_signed(dy as isize);
        assert!(x > 0 && y > 0 && x < B - 1 && y < B - 1, "seed {seed} left the board");
        let j = y * B + x;
        if b.visits[j] == 0 && j != (B / 2) * B + B / 2 {
            s += 1;
        }
        b.visits[j] += 1;
        if ci < CHECKS.len() && t == CHECKS[ci] {
            let (rx, ry) = (x as f64 - (B / 2) as f64, y as f64 - (B / 2) as f64);
            let mut v = b.visits.clone();
            v[(B / 2) * B + B / 2] += 1; // the origin is a visited site for S and the boundary
            let w = boundary_width(&v);
            rows.push(format!("{seed},{t},{},{s},{w:.4}", rx * rx + ry * ry));
            if t == 1_000_000 {
                s_at_1e6 = s;
                let o = (B - CROP) / 2;
                crop = (0..CROP).flat_map(|yy| (0..CROP).map(move |xx| (yy, xx))).map(|(yy, xx)| b.visits[(yy + o) * B + xx + o]).collect();
            }
            ci += 1;
        }
    }
    // Second pass: visits in the window (1e6 - 4S, 1e6].
    let start = 1_000_000u64.saturating_sub(4 * s_at_1e6);
    let mut b2 = Board::new(seed);
    let (mut x, mut y) = (B / 2, B / 2);
    let mut win = vec![0u32; B * B];
    for t in 1..=1_000_000u64 {
        let i = y * B + x;
        b2.arrow[i] = (b2.arrow[i] + 1) & 3;
        let (dx, dy) = DIRS[b2.arrow[i] as usize];
        x = x.wrapping_add_signed(dx as isize);
        y = y.wrapping_add_signed(dy as isize);
        if t > start {
            win[y * B + x] += 1;
        }
        b2.visits[y * B + x] += 1;
    }
    b2.visits[(B / 2) * B + B / 2] += 1;
    let visited = b2.visits.iter().filter(|&&v| v > 0).count();
    let exactly4 = b2.visits.iter().zip(&win).filter(|&(&v, &w)| v > 0 && w == 4).count();
    (rows, crop, exactly4 as f64 / visited as f64)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u64 = a[1].parse().unwrap();
    let seed0: u64 = a[2].parse().unwrap();
    let dir = std::path::Path::new(&a[3]);
    let maxn: u64 = a.get(4).map(|v| v.parse().unwrap()).unwrap_or(10_000_000);
    assert!(maxn >= 1_000_000);
    if a.get(5).map(|v| v == "alt").unwrap_or(false) {
        ALT.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    // Accumulate per thread and reduce, so memory stays at a few crops
    // regardless of the number of seeds.
    type Acc = (Vec<f64>, Vec<String>, Vec<(u64, f64)>);
    let empty = || -> Acc { (vec![0f64; CROP * CROP], vec![], vec![]) };
    let (sum, rows, mut ex4): Acc = (seed0..seed0 + n)
        .into_par_iter()
        .fold(empty, |mut acc, seed| {
            let (rows, crop, e4) = run(seed, maxn);
            for (s, &v) in acc.0.iter_mut().zip(&crop) {
                *s += v as f64;
            }
            acc.1.extend(rows);
            acc.2.push((seed, e4));
            acc
        })
        .reduce(empty, |mut x, y| {
            for (s, v) in x.0.iter_mut().zip(&y.0) {
                *s += v;
            }
            x.1.extend(y.1);
            x.2.extend(y.2);
            x
        });
    ex4.sort_by_key(|e| e.0);
    let mut f = std::fs::File::create(dir.join(format!("checkpoints_{seed0}.csv"))).unwrap();
    writeln!(f, "seed,N,R2,S,W").unwrap();
    for r in &rows {
        writeln!(f, "{r}").unwrap();
    }
    let ex4: Vec<f64> = ex4.into_iter().map(|e| e.1).collect();
    let bytes: Vec<u8> = sum.iter().flat_map(|v| v.to_le_bytes()).collect();
    std::fs::write(dir.join(format!("visits_1e6_{seed0}.f64")), bytes).unwrap();
    std::fs::write(dir.join(format!("exactly4_{seed0}.txt")), ex4.iter().map(|v| format!("{v}\n")).collect::<String>()).unwrap();
    eprintln!("done {n} seeds from {seed0}");
}
