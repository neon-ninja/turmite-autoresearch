//! Is the late-time motion an exact periodic orbit?  usage: orbit AGE LEN RULE...
//! Grows each rule from a blank board for AGE steps, records LEN further positions, and looks for the smallest p such that
//! pos[i+p] == pos[i] (and the heading) for all i in the first LEN/2 positions. Prints period, centred orbit signature
//! (number of distinct cells, bounding box, RMS about the mean) and whether the displacement over a period is zero.
use turmite::{DIRS, Rule};
const S: usize = 2048;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (age, len): (u64, usize) = (a[1].parse().unwrap(), a[2].parse().unwrap());
    for r in &a[3..] {
        let rule: Rule = r.parse().unwrap();
        let n = rule.len() as u8;
        let turn: Vec<u8> = rule.0.iter().map(|t| t.delta()).collect();
        let mut colour = vec![0u8; S * S];
        let (mut x, mut y, mut h) = ((S / 2) as i64, (S / 2) as i64, 0u8);
        let mut pos: Vec<(i64, i64, u8)> = Vec::with_capacity(len);
        let mut ok = true;
        for t in 0..age + len as u64 {
            let i = y as usize * S + x as usize;
            let c = colour[i];
            h = (h + turn[c as usize]) & 3;
            colour[i] = if c + 1 == n { 0 } else { c + 1 };
            let (dx, dy) = DIRS[h as usize];
            x += dx as i64;
            y += dy as i64;
            if x < 3 || y < 3 || x > S as i64 - 4 || y > S as i64 - 4 {
                ok = false;
                break;
            }
            if t >= age {
                pos.push((x, y, h));
            }
        }
        if !ok {
            println!("{r}: left the board");
            continue;
        }
        let half = len / 2;
        let mut period = None;
        for p in 1..=len - half {
            if pos[p] == pos[0] && (0..half).all(|i| pos[i + p] == pos[i]) {
                period = Some(p);
                break;
            }
        }
        let tail = &pos[..period.unwrap_or(half)];
        let m = tail.len() as f64;
        let (mx, my) = (tail.iter().map(|p| p.0 as f64).sum::<f64>() / m, tail.iter().map(|p| p.1 as f64).sum::<f64>() / m);
        let rms = (tail.iter().map(|p| (p.0 as f64 - mx).powi(2) + (p.1 as f64 - my).powi(2)).sum::<f64>() / m).sqrt();
        let mut cells: Vec<(i64, i64)> = tail.iter().map(|p| (p.0, p.1)).collect();
        cells.sort();
        cells.dedup();
        let (x0, x1) = (cells.iter().map(|c| c.0).min().unwrap(), cells.iter().map(|c| c.0).max().unwrap());
        let (y0, y1) = (cells.iter().map(|c| c.1).min().unwrap(), cells.iter().map(|c| c.1).max().unwrap());
        println!("{r}: period {:?} distinct cells {} bbox {}x{} centre ({:.1},{:.1}) rms about mean {:.2} (over {} steps)", period, cells.len(), x1 - x0 + 1, y1 - y0 + 1, mx - (S / 2) as f64, my - (S / 2) as f64, rms, tail.len());
    }
}
