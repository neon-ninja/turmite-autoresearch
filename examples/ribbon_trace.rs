//! G1 (notes/research-frontier-2026-10-13-pm.md): windowed anatomy of a non-highway grid breakout.
//! usage: ribbon_trace RULE FINE_START END [FINE=1000000] [COARSE=100000000] [GRID=32768] > csv
//! From a blank board; one row per window (COARSE steps before FINE_START, FINE after). Per window: new cells (first visits), steps per
//! new cell, bounding box of the *newly visited* cells (their x/y extent is the "ribbon thickness" of what was built in the window),
//! the ant's own extent and end position. Same dynamics loop as `escape_trace` (plain flat board); stops if the ant leaves the board.
use turmite::{DIRS, Rule};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let fine_start: u64 = a[2].parse().unwrap();
    let end: u64 = a[3].parse().unwrap();
    let fine: u64 = a.get(4).map_or(1_000_000, |v| v.parse().unwrap());
    let coarse: u64 = a.get(5).map_or(100_000_000, |v| v.parse().unwrap());
    let size: usize = a.get(6).map_or(32768, |v| v.parse().unwrap());
    let n = rule.len();
    let turn: Vec<u8> = rule.0.iter().map(|t| t.delta()).collect();
    const SEEN: u8 = 0x80; // visited flag in the top bit; colour in the low bits
    let mut cells = vec![0u8; size * size];
    let c = (size / 2) as i64;
    let (mut x, mut y, mut h) = (c, c, 0u8);
    println!("t_end,new_cells,steps_per_new,new_x0,new_x1,new_y0,new_y1,ant_x0,ant_x1,ant_y0,ant_y1,x,y");
    let (mut w_new, mut wn) = (0u64, [i64::MAX, i64::MIN, i64::MAX, i64::MIN]);
    let mut wa = [i64::MAX, i64::MIN, i64::MAX, i64::MIN];
    let mut t = 0u64;
    let mut win = coarse;
    let mut win_start = 0u64;
    while t < end {
        let i = (y as usize) * size + x as usize;
        let v = cells[i];
        if v & SEEN == 0 {
            w_new += 1;
            wn = [wn[0].min(x - c), wn[1].max(x - c), wn[2].min(y - c), wn[3].max(y - c)];
        }
        let col = (v & !SEEN) as usize;
        h = (h + turn[col]) & 3;
        cells[i] = SEEN | ((col + 1) % n) as u8;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        t += 1;
        wa = [wa[0].min(x - c), wa[1].max(x - c), wa[2].min(y - c), wa[3].max(y - c)];
        let off = x < 1 || y < 1 || x >= size as i64 - 1 || y >= size as i64 - 1;
        if t >= win_start + win || off {
            let spn = if w_new > 0 { (t - win_start) as f64 / w_new as f64 } else { f64::INFINITY };
            println!("{t},{w_new},{spn:.3},{},{},{},{},{},{},{},{},{},{}", wn[0], wn[1], wn[2], wn[3], wa[0], wa[1], wa[2], wa[3], x - c, y - c);
            if off {
                eprintln!("left the board at step {t}");
                break;
            }
            win_start = t;
            if t >= fine_start { win = fine; }
            w_new = 0;
            wn = [i64::MAX, i64::MIN, i64::MAX, i64::MIN];
            wa = [i64::MAX, i64::MIN, i64::MAX, i64::MIN];
        }
    }
}
