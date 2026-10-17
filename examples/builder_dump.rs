//! G1 follow-up (notes/g1-builder-structure-2026-10-14.md): run a rule from a blank board and dump what the ant does late.
//! usage: builder_dump RULE T0 LEN OUTPREFIX [GRID=32768] [WAKE=800]
//! Writes OUTPREFIX.traj.csv (x,y,h per step for LEN steps starting at T0, offsets from the centre) and, at T0+LEN,
//! OUTPREFIX.wake.bin: raw colour bytes (255 = unvisited) of the columns x in [ax-WAKE/4, ax+WAKE) over all rows
//! (header line "x0 x1 size" then rows y-major). Same plain loop as `ribbon_trace`.
use std::io::Write;
use turmite::{DIRS, Rule};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let t0: u64 = a[2].parse().unwrap();
    let len: u64 = a[3].parse().unwrap();
    let out = &a[4];
    let size: usize = a.get(5).map_or(32768, |v| v.parse().unwrap());
    let wake: i64 = a.get(6).map_or(800, |v| v.parse().unwrap());
    let n = rule.len();
    let turn: Vec<u8> = rule.0.iter().map(|t| t.delta()).collect();
    const SEEN: u8 = 0x80;
    let mut cells = vec![0u8; size * size];
    let c = (size / 2) as i64;
    let (mut x, mut y, mut h) = (c, c, 0u8);
    let mut traj = std::io::BufWriter::new(std::fs::File::create(format!("{out}.traj.csv")).unwrap());
    writeln!(traj, "x,y,h").unwrap();
    for t in 0..t0 + len {
        let i = (y as usize) * size + x as usize;
        let col = (cells[i] & !SEEN) as usize;
        h = (h + turn[col]) & 3;
        cells[i] = SEEN | ((col + 1) % n) as u8;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if t >= t0 {
            writeln!(traj, "{},{},{h}", x - c, y - c).unwrap();
        }
        if x < 1 || y < 1 || x >= size as i64 - 1 || y >= size as i64 - 1 {
            eprintln!("left board at {t}");
            break;
        }
    }
    let (x0, x1) = ((x - wake / 4).max(0), (x + wake).min(size as i64));
    let mut f = std::io::BufWriter::new(std::fs::File::create(format!("{out}.wake.bin")).unwrap());
    writeln!(f, "{x0} {x1} {size}").unwrap();
    let mut row = vec![0u8; (x1 - x0) as usize];
    for yy in 0..size {
        for (k, xx) in (x0..x1).enumerate() {
            let v = cells[yy * size + xx as usize];
            row[k] = if v & SEEN == 0 { 255 } else { v & !SEEN };
        }
        f.write_all(&row).unwrap();
    }
}
