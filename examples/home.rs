//! Does the ant ever return to the initial state (blank board, origin, facing north)?
use turmite::{DIRS, Rule};
fn main() {
    let max: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    for r in std::env::args().skip(2) {
        let rule: Rule = r.parse().unwrap();
        const S: i64 = 4096;
        let mut cells = vec![0u8; (S * S) as usize];
        let n = rule.len() as u8;
        let (mut x, mut y, mut h) = (S / 2, S / 2, 0u8);
        let mut nonzero: u64 = 0;
        let (mut minx, mut maxx, mut miny, mut maxy) = (x, x, y, y);
        let mut found = None;
        let mut checkpoints = vec![];
        for t in 1..=max {
            let i = (y * S + x) as usize;
            let c = cells[i];
            h = (h + rule.0[c as usize].delta()) & 3;
            let nc = (c + 1) % n;
            cells[i] = nc;
            if c == 0 { nonzero += 1 } else if nc == 0 { nonzero -= 1 }
            let (dx, dy) = DIRS[h as usize];
            x += dx as i64; y += dy as i64;
            minx = minx.min(x); maxx = maxx.max(x); miny = miny.min(y); maxy = maxy.max(y);
            if nonzero == 0 && h == 0 && x == S / 2 && y == S / 2 { found = Some(t); break; }
            if t.is_power_of_two() && t >= 1 << 16 { checkpoints.push((t, maxx - minx + 1, maxy - miny + 1)); }
        }
        let tail: Vec<_> = checkpoints.iter().rev().take(4).rev().collect();
        println!("{r:>13} returns home at {found:?}  extent at 2^k: {tail:?}");
    }
}
