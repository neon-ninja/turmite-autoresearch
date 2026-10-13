//! Colour statistics of the visited region at time checkpoints (A1/A6/C3 of the 2026-10-13 frontier).
//!
//! At each checkpoint t, over the cells the ant has stood on (including the
//! start and its current cell): the number of such cells, the number on the
//! boundary (a visited cell with a never-visited 4-neighbour), and the
//! histogram of current colours over all visited cells and over boundary cells.
//! Deliberately the plain loop of `reference`, on a flat board with a visited
//! bitmap; `tests/colours.rs` checks it against the HashMap oracle.

use crate::{DIRS, Rule};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub t: u64,
    pub area: u64,
    pub boundary: u64,
    pub hist_all: Vec<u64>,
    pub hist_boundary: Vec<u64>,
}

/// Returns the snapshots taken and whether the ant got within 2 cells of the board edge (stops there).
pub fn colour_hists(rule: &Rule, size: usize, checkpoints: &[u64]) -> (Vec<Snapshot>, bool) {
    let n = rule.len();
    let mut cells = vec![0u8; size * size];
    let mut seen = vec![false; size * size];
    let c0 = size / 2;
    let (mut x, mut y, mut h) = (c0, c0, 0u8);
    seen[y * size + x] = true;
    let mut t = 0u64;
    let mut out = vec![];
    for &cp in checkpoints {
        while t < cp {
            let i = y * size + x;
            let c = cells[i] as usize;
            h = (h + rule.0[c].delta()) & 3;
            cells[i] = ((c + 1) % n) as u8;
            let (dx, dy) = DIRS[h as usize];
            x = x.wrapping_add_signed(dx as isize);
            y = y.wrapping_add_signed(dy as isize);
            if x < 2 || y < 2 || x + 2 >= size || y + 2 >= size {
                return (out, true);
            }
            seen[y * size + x] = true;
            t += 1;
        }
        let mut s = Snapshot { t, area: 0, boundary: 0, hist_all: vec![0; n], hist_boundary: vec![0; n] };
        for yy in 1..size - 1 {
            for xx in 1..size - 1 {
                let i = yy * size + xx;
                if !seen[i] {
                    continue;
                }
                s.area += 1;
                s.hist_all[cells[i] as usize] += 1;
                if !(seen[i - 1] && seen[i + 1] && seen[i - size] && seen[i + size]) {
                    s.boundary += 1;
                    s.hist_boundary[cells[i] as usize] += 1;
                }
            }
        }
        out.push(s);
    }
    (out, false)
}
