//! kappa_uniform_baseline SEED0 NSEEDS LOG2N OUT.csv
//!
//! Mean-field walker: every step the walker is resampled uniformly over the
//! visited cells, then steps to a uniform random 4-neighbour. Its instantaneous
//! edge occupation is P/A by construction, so the probe's kappa for it is the
//! reference "uniform" value under the probe's windowing. Bookkeeping copies
//! src/probe.rs (octave windows (2^k, 2^(k+1)], B = steps starting on a visited
//! cell with an unvisited 4-neighbour, P and A read at window end).
//! Output rows: seed,k,area,perimeter,new_cells,boundary_steps.
use rayon::prelude::*;
use std::io::Write;
use turmite::walkers::splitmix_next;

const SIZE: usize = 4096;
const UNV: u8 = u8::MAX;

fn run(seed: u64, log2n: u32) -> Vec<[u64; 6]> {
    let nbr: [isize; 4] = [-(SIZE as isize), 1, SIZE as isize, -1];
    let mut unv = vec![UNV; SIZE * SIZE];
    let mut cells: Vec<usize> = Vec::new();
    let (mut area, mut perim) = (0u64, 0u64);
    let mut st = seed.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ 0x2545_F491_4F6C_DD1D;
    let visit = |idx: usize, unv: &mut Vec<u8>, cells: &mut Vec<usize>, area: &mut u64, perim: &mut u64| {
        let mut u = 0u8;
        for d in nbr {
            let j = (idx as isize + d) as usize;
            if unv[j] == UNV {
                u += 1;
            } else {
                unv[j] -= 1;
                if unv[j] == 0 {
                    *perim -= 1;
                }
            }
        }
        unv[idx] = u;
        if u > 0 {
            *perim += 1;
        }
        *area += 1;
        cells.push(idx);
    };
    let start = (SIZE / 2) * SIZE + SIZE / 2;
    visit(start, &mut unv, &mut cells, &mut area, &mut perim);
    let mut out = vec![];
    let mut t = 0u64;
    for k in 0..log2n {
        let t_end = 1u64 << (k + 1);
        let (mut new_cells, mut b) = (0u64, 0u64);
        while t < t_end {
            let r = splitmix_next(&mut st);
            let idx = cells[((r >> 8) % cells.len() as u64) as usize];
            if unv[idx] > 0 {
                b += 1;
            }
            let j = (idx as isize + nbr[(r & 3) as usize]) as usize;
            t += 1;
            if unv[j] == UNV {
                new_cells += 1;
                visit(j, &mut unv, &mut cells, &mut area, &mut perim);
            }
        }
        out.push([seed, k as u64, area, perim, new_cells, b]);
    }
    // Oracle recount: brute-force area and boundary count must match the incremental ones.
    let brute = cells.iter().filter(|&&i| unv[i] > 0).count() as u64;
    let real = cells.iter().filter(|&&i| nbr.iter().any(|d| unv[(i as isize + d) as usize] == UNV)).count() as u64;
    assert_eq!(cells.len() as u64, area);
    assert_eq!(brute, perim);
    assert_eq!(real, perim);
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (s0, n, log2n): (u64, u64, u32) = (a[1].parse().unwrap(), a[2].parse().unwrap(), a[3].parse().unwrap());
    let rows: Vec<Vec<[u64; 6]>> = (0..n).into_par_iter().map(|i| run(s0 + i, log2n)).collect();
    let mut f = std::fs::File::create(&a[4]).unwrap();
    writeln!(f, "seed,k,area,perimeter,new_cells,boundary_steps").unwrap();
    for r in rows.iter().flatten() {
        writeln!(f, "{},{},{},{},{},{}", r[0], r[1], r[2], r[3], r[4], r[5]).unwrap();
    }
}
