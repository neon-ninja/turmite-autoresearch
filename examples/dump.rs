//! dump RULE K OUTDIR : run 2^K steps on a 4096 board, then write
//!   OUTDIR/RULE.traj  positions (i16 x, i16 y, relative to start) for every
//!                     step of the last octave, plus the position before it
//!   OUTDIR/RULE.grid  final colours (u8, 4096^2), with 255 marking cells
//!                     never visited
use std::io::Write;
use turmite::{DIRS, Rule};

const S: usize = 4096;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let k: u32 = a[2].parse().unwrap();
    let dir = std::path::Path::new(&a[3]);
    let n = rule.len() as u8;
    let mut colour = vec![0u8; S * S];
    let mut visited = vec![false; S * S];
    let (mut x, mut y, mut h) = (S / 2, S / 2, 0u8);
    visited[y * S + x] = true;
    let half = 1u64 << (k - 1);
    let mut traj = Vec::with_capacity((half as usize + 1) * 4);
    let c = (S / 2) as i64;
    let mut push = |x: usize, y: usize, traj: &mut Vec<u8>| {
        traj.extend_from_slice(&((x as i64 - c) as i16).to_le_bytes());
        traj.extend_from_slice(&((y as i64 - c) as i16).to_le_bytes());
    };
    for t in 1..=(1u64 << k) {
        if t == half + 1 {
            push(x, y, &mut traj);
        }
        let i = y * S + x;
        let col = colour[i];
        h = (h + rule.0[col as usize].delta()) & 3;
        colour[i] = (col + 1) % n;
        let (dx, dy) = DIRS[h as usize];
        x = x.wrapping_add_signed(dx as isize);
        y = y.wrapping_add_signed(dy as isize);
        assert!(x < S && y < S, "left the board");
        visited[y * S + x] = true;
        if t > half {
            push(x, y, &mut traj);
        }
    }
    let grid: Vec<u8> = colour.iter().zip(&visited).map(|(&c, &v)| if v { c } else { 255 }).collect();
    std::fs::File::create(dir.join(format!("{}.traj", a[1]))).unwrap().write_all(&traj).unwrap();
    std::fs::File::create(dir.join(format!("{}.grid", a[1]))).unwrap().write_all(&grid).unwrap();
}
