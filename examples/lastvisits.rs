//! Item 6 pilot: how "periodic" is the interior? Run T = 2^k steps, then count visits per cell
//! over the next window of L = m * A steps (A = cells visited so far; m = 4 for the rotor,
//! m = n for an ant of n colours; also m = 4 for ants). Report the histogram of window visit
//! counts over cells visited at least once in the window and not on the boundary, and the
//! fraction with exactly m visits. Rotor must give ~0.89 (Priezzhev heuristic, calibration.md).
//!
//! usage: lastvisits K SPEC...   (SPEC: RULE, RULE@SEED, rotor:SEED)
use turmite::walkers::{AntWalker, PatchedAnt, Rotor, Walker};
use turmite::{DIRS, Rule};

fn run<W: Walker>(mut w: W, n: usize, k: u32, label: &str) {
    let size = 4096usize;
    let c = (size / 2) as i64;
    let mut colour = vec![0u8; size * size];
    if W::RANDOM_INIT {
        for (i, cell) in colour.iter_mut().enumerate() {
            *cell = w.initial_cell((i % size) as i64 - c, (i / size) as i64 - c);
        }
    }
    let mut seen = vec![false; size * size];
    let mut idx = (c as usize) * size + c as usize;
    seen[idx] = true;
    let mut area = 1u64;
    let mut h = 0u8;
    let s = size as isize;
    let mut step = |idx: &mut usize, seen: &mut Vec<bool>, area: &mut u64, cnt: Option<&mut Vec<u16>>, w: &mut W, colour: &mut Vec<u8>, h: &mut u8| {
        w.step(&mut colour[*idx], h);
        let (dx, dy) = DIRS[*h as usize];
        *idx = (*idx as isize + dy as isize * s + dx as isize) as usize;
        if !seen[*idx] {
            seen[*idx] = true;
            *area += 1;
        }
        if let Some(cn) = cnt {
            cn[*idx] = cn[*idx].saturating_add(1);
        }
    };
    for _ in 0..1u64 << k {
        step(&mut idx, &mut seen, &mut area, None, &mut w, &mut colour, &mut h);
    }
    for m in [4usize, n] {
        // restore nothing: windows are consecutive, each uses its own counter and the dynamics continue
        let a0 = area;
        let mut cnt = vec![0u16; size * size];
        let seen_before = seen.clone();
        for _ in 0..(m as u64 * a0) {
            step(&mut idx, &mut seen, &mut area, Some(&mut cnt), &mut w, &mut colour, &mut h);
        }
        // interior: seen before the window and all 4 neighbours seen before the window
        let mut hist = std::collections::BTreeMap::new();
        let (mut tot, mut exact) = (0u64, 0u64);
        for y in 1..size - 1 {
            for x in 1..size - 1 {
                let i = y * size + x;
                if seen_before[i] && seen_before[i - 1] && seen_before[i + 1] && seen_before[i - size] && seen_before[i + size] {
                    tot += 1;
                    *hist.entry(cnt[i]).or_insert(0u64) += 1;
                    if cnt[i] as usize == m {
                        exact += 1;
                    }
                }
            }
        }
        let mut top: Vec<_> = hist.iter().map(|(a, b)| (*b, *a)).collect();
        top.sort_by(|a, b| b.cmp(a));
        let top: Vec<String> = top.iter().take(4).map(|(b, a)| format!("{a}:{:.3}", *b as f64 / tot as f64)).collect();
        println!("{label} k={k} A0={a0} window=m*A m={m} interior={tot} frac(exactly m)={:.4} top counts {}", exact as f64 / tot as f64, top.join(" "));
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let k: u32 = a[1].parse().unwrap();
    for r in &a[2..] {
        if let Some(seed) = r.strip_prefix("rotor:") {
            run(Rotor { seed: seed.parse().unwrap() }, 4, k, r);
        } else if let Some((rule, seed)) = r.split_once('@') {
            let ru: Rule = rule.parse().unwrap();
            run(PatchedAnt::new(&ru, seed.parse().unwrap(), 2), ru.len(), k, r);
        } else {
            let ru: Rule = r.parse().unwrap();
            run(AntWalker::new(&ru), ru.len(), k, r);
        }
    }
}
