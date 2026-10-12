//! snapshot RULE STEPS OUT : dump the 8192^2 grid to OUT and positions every
//! STEPS/2000 steps to OUT.traj.csv. Stops early if the ant leaves the grid.
use std::io::Write;
use turmite::{DIRS, Rule};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let steps: u64 = a[2].parse().unwrap();
    const S: i64 = 8192;
    let mut cells = vec![0u8; (S * S) as usize];
    let n = rule.len() as u8;
    let (mut x, mut y, mut h) = (S / 2, S / 2, 0u8);
    let sample = (steps / 2000).max(1);
    let mut traj = std::fs::File::create(format!("{}.traj.csv", a[3])).unwrap();
    writeln!(traj, "t,x,y").unwrap();
    for t in 1..=steps {
        let i = (y * S + x) as usize;
        let c = cells[i];
        h = (h + rule.0[c as usize].delta()) & 3;
        cells[i] = (c + 1) % n;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if t % sample == 0 {
            writeln!(traj, "{t},{},{}", x - S / 2, y - S / 2).unwrap();
        }
        if !(0..S).contains(&x) || !(0..S).contains(&y) {
            println!("escaped at {t}");
            break;
        }
    }
    std::fs::write(&a[3], &cells).unwrap();
}
