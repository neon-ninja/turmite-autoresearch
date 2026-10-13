//! What is the ant doing when a run "escapes without a highway"? usage: escape_trace RULE ESCAPE_STEP [LAST=4000000] [GRID=32768]
//! Runs from a blank board to ESCAPE_STEP - LAST, then records LAST positions and looks for the smallest period p (<= LAST/4) with
//! pos[i+p] - pos[i] constant over the recorded stretch (a highway with any period) and reports the displacement per period.
//! Plain loop of `reference` on a flat board (checked against nothing new; it is the same dynamics).
use turmite::{DIRS, Rule};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let esc: u64 = a[2].parse().unwrap();
    let last: usize = a.get(3).map_or(4_000_000, |v| v.parse().unwrap());
    let size: usize = a.get(4).map_or(32768, |v| v.parse().unwrap());
    let n = rule.len();
    let turn: Vec<u8> = rule.0.iter().map(|t| t.delta()).collect();
    let mut cells = vec![0u8; size * size];
    let c = size / 2;
    let (mut x, mut y, mut h) = (c as i64, c as i64, 0u8);
    let start = esc - last as u64;
    let (mut minx, mut maxx, mut miny, mut maxy) = (x, x, y, y);
    let mut pos: Vec<(i32, i32)> = Vec::with_capacity(last);
    for t in 0..esc {
        let i = (y as usize) * size + x as usize;
        let col = cells[i] as usize;
        h = (h + turn[col]) & 3;
        cells[i] = ((col + 1) % n) as u8;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if x < 1 || y < 1 || x >= size as i64 - 1 || y >= size as i64 - 1 {
            println!("left the board at step {}", t + 1);
            break;
        }
        if t < start {
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(y);
            maxy = maxy.max(y);
        } else {
            pos.push(((x - c as i64) as i32, (y - c as i64) as i32));
        }
    }
    println!("{} steps recorded; bounding box before the recorded stretch: x {}..{} y {}..{} (relative to start)", pos.len(), minx - c as i64, maxx - c as i64, miny - c as i64, maxy - c as i64);
    if pos.len() < 100 {
        return;
    }
    println!("first recorded position {:?}, last {:?}, net displacement {:?}", pos[0], pos[pos.len() - 1], (pos[pos.len() - 1].0 - pos[0].0, pos[pos.len() - 1].1 - pos[0].1));
    let m = pos.len();
    let half = m / 2;
    for p in 1..=(m / 4) {
        let d = (pos[p].0 - pos[0].0, pos[p].1 - pos[0].1);
        if (0..half).all(|i| (pos[i + p].0 - pos[i].0, pos[i + p].1 - pos[i].1) == d) {
            println!("periodic motion: smallest period {p}, displacement per period {:?} (checked over the first {half} positions)", d);
            let j = pos.len() - p - 1;
            let still = (j..j + 1).all(|i| (pos[i + p].0 - pos[i].0, pos[i + p].1 - pos[i].1) == d);
            println!("same displacement at the end of the recorded stretch: {still}");
            return;
        }
    }
    println!("no exact period <= {} over the first half of the recorded stretch", m / 4);
    // coarse trajectory so a human can see it
    for k in (0..m).step_by(m / 8) {
        println!("  t_rel {k}: {:?}", pos[k]);
    }
}
