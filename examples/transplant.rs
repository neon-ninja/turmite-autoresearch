//! transplant GRIDFILE RULE MODE X0 Y0 STEPS SEED
//! Load a final colour field written by examples/dump.rs (255 = never
//! visited, treated as blank), optionally modify it, put a fresh ant (heading
//! north) at (X0, Y0) relative to the original home, and run.
//! MODE: real     use the field as is
//!       shuffled permute colours randomly among the visited cells
//!       frozen   as real, but colours never change (a static environment)
//! Prints, every STEPS/64 steps: t, mean x, mean y over the block, and the
//! fraction of the block spent within 10 cells of home, of the start, and of
//! the previous block's mean position (a tether anywhere shows up there),
//! and the RMS distance from the block's own mean position (its spread).
use turmite::{DIRS, Rule};

const S: usize = 4096;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read(&a[1]).unwrap();
    let rule: Rule = a[2].parse().unwrap();
    let mode = a[3].as_str();
    let (x0, y0): (i64, i64) = (a[4].parse().unwrap(), a[5].parse().unwrap());
    let steps: u64 = a[6].parse().unwrap();
    let mut seed: u64 = a[7].parse().unwrap();
    let n = rule.len() as u8;
    let mut colour: Vec<u8> = raw.iter().map(|&c| if c == 255 { 0 } else { c }).collect();
    if mode == "shuffled" {
        let idx: Vec<usize> = (0..S * S).filter(|&i| raw[i] != 255).collect();
        let mut vals: Vec<u8> = idx.iter().map(|&i| colour[i]).collect();
        // Fisher-Yates with splitmix64
        let mut next = || {
            seed = seed.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = seed;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        };
        for i in (1..vals.len()).rev() {
            let j = (next() % (i as u64 + 1)) as usize;
            vals.swap(i, j);
        }
        for (k, &i) in idx.iter().enumerate() {
            colour[i] = vals[k];
        }
    }
    let frozen = mode == "frozen";
    let c = (S / 2) as i64;
    let (mut x, mut y, mut h) = (c + x0, c + y0, 0u8);
    let block = (steps / 64).max(1);
    let (mut sx, mut sy, mut near_home, mut near_start, mut near_prev) = (0f64, 0f64, 0u64, 0u64, 0u64);
    let mut sr2 = 0f64;
    let mut prev = (x0 as f64, y0 as f64);
    println!("t,mean_x,mean_y,near_home,near_start,near_prev,spread");
    for t in 1..=steps {
        let i = (y as usize) * S + x as usize;
        let col = colour[i];
        h = (h + rule.0[col as usize].delta()) & 3;
        if !frozen {
            colour[i] = (col + 1) % n;
        }
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        assert!(x > 0 && y > 0 && x < S as i64 - 1 && y < S as i64 - 1, "left the board");
        let (rx, ry) = (x - c, y - c);
        sx += rx as f64;
        sy += ry as f64;
        sr2 += (rx * rx + ry * ry) as f64;
        near_home += (rx * rx + ry * ry <= 100) as u64;
        near_start += ((rx - x0).pow(2) + (ry - y0).pow(2) <= 100) as u64;
        near_prev += ((rx as f64 - prev.0).powi(2) + (ry as f64 - prev.1).powi(2) <= 100.0) as u64;
        if t % block == 0 {
            let b = block as f64;
            let spread = (sr2 / b - (sx / b).powi(2) - (sy / b).powi(2)).max(0.0).sqrt();
            println!("{t},{:.1},{:.1},{:.3},{:.3},{:.3},{spread:.2}", sx / b, sy / b, near_home as f64 / b, near_start as f64 / b, near_prev as f64 / b);
            prev = (sx / b, sy / b);
            (sx, sy, near_home, near_start, near_prev) = (0.0, 0.0, 0, 0, 0);
            sr2 = 0.0;
        }
    }
}
