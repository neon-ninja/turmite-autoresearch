//! Growth curves: for each rule on stdin, the touched extent (max of width
//! and height) at t = 2^16, 2^17, ..., up to MAX_STEPS. Stops at the edge.
//!
//! usage: growth MAX_STEPS [GRID] < rules.txt > growth.csv

use rayon::prelude::*;
use std::io::Write;
use turmite::{DIRS, Rule};

fn curve(rule: &Rule, cells: &mut [u8], size: usize, max_steps: u64) -> Vec<(u64, u32)> {
    cells.fill(0);
    let n = rule.len();
    let next: Vec<u8> = (0..n).map(|i| ((i + 1) % n) as u8).collect();
    let turn: Vec<u8> = rule.0.iter().map(|t| t.delta()).collect();
    let c = size / 2;
    let (mut x, mut y, mut h) = (c, c, 0u8);
    let (mut minx, mut maxx, mut miny, mut maxy) = (c, c, c, c);
    let mut out = vec![];
    let mut mark: u64 = 1 << 16;
    for t in 1..=max_steps {
        let i = y * size + x;
        let col = cells[i] as usize;
        h = (h + turn[col]) & 3;
        cells[i] = next[col];
        let (dx, dy) = DIRS[h as usize];
        x = x.wrapping_add_signed(dx as isize);
        y = y.wrapping_add_signed(dy as isize);
        if x >= size || y >= size {
            break;
        }
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
        if t == mark {
            out.push((t, (maxx - minx + 1).max(maxy - miny + 1) as u32));
            mark <<= 1;
        }
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let max_steps: u64 = a[1].parse().unwrap();
    let size: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(8192);
    let rules: Vec<Rule> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect();
    let rows: Vec<String> = rules
        .par_iter()
        .map_init(
            || vec![0u8; size * size],
            |cells, r| {
                let pts = curve(r, cells, size, max_steps);
                let s: Vec<String> = pts.iter().map(|(t, e)| format!("{}:{e}", t.trailing_zeros())).collect();
                format!("{r},{}", s.join(" "))
            },
        )
        .collect();
    let mut out = std::io::stdout().lock();
    writeln!(out, "rule,curve").unwrap();
    for r in rows {
        writeln!(out, "{r}").unwrap();
    }
}
