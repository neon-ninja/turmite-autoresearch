//! Highway onset replay that reuses its grid between calls.
//!
//! Same result as `lean::highway_onset_flat`, but that function allocates and zeroes a fresh
//! `(size+2)^2` grid per call (67 MB at the census size 8192), which dominated the wall time of a
//! sweep once the ants themselves were fast. Here a worker keeps one [`OnsetScratch`] and each replay
//! clears only the rectangle it touched. Checked against `highway_onset_flat` in `tests/onset.rs`.

use crate::detect::Highway;
use crate::{DIRS, Rule};

pub struct OnsetScratch {
    size: usize,
    cells: Vec<u8>,
}

impl OnsetScratch {
    /// `size` is the grid side the original run used (as for `highway_onset_flat`).
    pub fn new(size: usize) -> OnsetScratch {
        OnsetScratch { size, cells: vec![0; (size + 2) * (size + 2)] }
    }
}

/// First step s after which pos[t] - pos[t-p] == d for all t >= s + p (see `fast::highway_onset`).
pub fn highway_onset_reusing(rule: &Rule, hw: Highway, until: u64, sc: &mut OnsetScratch) -> u64 {
    let n = rule.len();
    let p = hw.period as usize;
    let size = sc.size;
    let s = size + 2;
    let c = size / 2;
    let turn: Vec<u8> = rule.0.iter().map(|t| t.delta()).collect();
    let (mut x, mut y) = ((c + 1) as i64, (c + 1) as i64);
    let (mut minx, mut maxx, mut miny, mut maxy) = (x, x, y, y);
    let mut ring = vec![(0i32, 0i32); p];
    let mut h = 0u8;
    let mut last_bad: u64 = 0;
    for step in 1..=until {
        let idx = y as usize * s + x as usize;
        let b = sc.cells[idx] as usize;
        h = (h + turn[b]) & 3;
        sc.cells[idx] = ((b + 1) % n) as u8;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        let pos = ((x - 1 - c as i64) as i32, (y - 1 - c as i64) as i32);
        let slot = (step as usize) % p;
        let prev = ring[slot];
        ring[slot] = pos;
        if step < p as u64 || (pos.0 - prev.0, pos.1 - prev.1) != (hw.dx, hw.dy) {
            last_bad = step;
        }
        if x < 1 || y < 1 || x >= s as i64 - 1 || y >= s as i64 - 1 {
            break; // left the grid (only possible on the final step of an escaped run)
        }
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
    }
    // Every written cell is inside the box of visited in-grid positions.
    for yy in miny..=maxy {
        let row = yy as usize * s;
        sc.cells[row + minx as usize..=row + maxx as usize].fill(0);
    }
    (last_bad + 1).saturating_sub(p as u64)
}
