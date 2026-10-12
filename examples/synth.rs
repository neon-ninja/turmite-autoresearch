//! synth GRIDFILE N LEVEL SEED SWEEPS OUTFILE [CX CY]
//! Make a field with the same colour counts as GRIDFILE (from examples/dump.rs,
//! 255 = never visited, N colours) and matched local statistics:
//!   shuffle  colours permuted at random among visited cells
//!   radial   permuted within rings of width 2 around (CX, CY), relative to home
//!   pairs    horizontal and vertical neighbour-pair histograms matched
//!   b2       oriented 2x2 block histogram matched
//!   b3       oriented 3x3 block histogram matched
//! Matched levels start from a shuffle and anneal for SWEEPS sweeps.
//! Writes OUTFILE in the same format and prints one CSV line:
//! level,seed,windows,distinct,cost0,cost,same_place
use turmite::synth::{Annealer, Patch, histogram, radial_shuffle, shapes, shuffle};

const S: usize = 4096;
const PAD: usize = 3;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read(&a[1]).unwrap();
    let n: u8 = a[2].parse().unwrap();
    let level = a[3].as_str();
    let seed0: u64 = a[4].parse().unwrap();
    let sweeps: usize = a[5].parse().unwrap();
    let (mut x0, mut y0, mut x1, mut y1) = (S, S, 0, 0);
    for (i, &c) in raw.iter().enumerate() {
        if c != 255 {
            let (x, y) = (i % S, i / S);
            (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
        }
    }
    let (ox, oy) = (x0 - PAD, y0 - PAD);
    let (w, h) = (x1 - x0 + 1 + 2 * PAD, y1 - y0 + 1 + 2 * PAD);
    let mut cells = vec![n; w * h];
    for y in 0..h {
        for x in 0..w {
            let c = raw[(y + oy) * S + x + ox];
            if c != 255 {
                assert!(c < n);
                cells[y * w + x] = c;
            }
        }
    }
    let src = Patch { w, h, n, cells };
    let mut p = src.clone();
    let mut seed = seed0.wrapping_mul(0x9E3779B97F4A7C15) ^ 0x5EED;
    let (mut windows, mut distinct, mut cost0, mut cost) = (0, 0, 0, 0);
    match level {
        "shuffle" => shuffle(&mut p, &mut seed),
        "radial" => {
            let cx = (S / 2 - ox) as f64 + a[7].parse::<f64>().unwrap();
            let cy = (S / 2 - oy) as f64 + a[8].parse::<f64>().unwrap();
            radial_shuffle(&mut p, cx, cy, 2.0, &mut seed);
        }
        _ => {
            let sh = shapes(level);
            let target = histogram(&src, &sh);
            windows = target.values().sum::<i64>();
            distinct = target.len();
            shuffle(&mut p, &mut seed);
            let mut an = Annealer::new(p, &target, sh);
            cost0 = an.cost;
            let t0: f64 = std::env::var("SYNTH_T0").map(|v| v.parse().unwrap()).unwrap_or(3.0);
            an.anneal(t0, 0.05, sweeps, &mut seed);
            cost = an.cost;
            p = an.p;
        }
    }
    let movable = (0..src.cells.len()).filter(|&i| src.movable(i)).count();
    let same = (0..src.cells.len()).filter(|&i| src.movable(i) && src.cells[i] == p.cells[i]).count();
    let mut out = raw.clone();
    for y in 0..h {
        for x in 0..w {
            let c = p.cells[y * w + x];
            if c < n {
                out[(y + oy) * S + x + ox] = c;
            }
        }
    }
    std::fs::write(&a[6], out).unwrap();
    println!("{level},{seed0},{windows},{distinct},{cost0},{cost},{:.4}", same as f64 / movable as f64);
}
