//! rulepat_dump RULE STEPS OUTPREFIX : run the ant on an 8192^2 board and write
//!   OUTPREFIX.col  colours (u8, 8192^2)       OUTPREFIX.vis  visit counts (u16 saturating)
//! plus a one-line JSON of motion statistics on stdout. Stops if the ant leaves the board.
//! "diag" steps are steps whose turn differs from the previous step's turn (a
//! staircase move: L,R,L,R...); "straight-ish" are repeats (L,L / R,R: tight loops).
use std::io::Write;
use turmite::{DIRS, Rule};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let steps: u64 = a[2].parse().unwrap();
    const S: i64 = 8192;
    let n = rule.len();
    let mut cells = vec![0u8; (S * S) as usize];
    let mut vis = vec![0u16; (S * S) as usize];
    let (mut x, mut y, mut h) = (S / 2, S / 2, 0u8);
    let mut prev_turn = 0u8;
    let mut alt = 0u64;
    let mut per_colour = vec![0u64; n];
    // lengths of maximal runs of alternating turns (diagonal runs)
    let mut run = 0u64;
    let mut run_hist = vec![0u64; 24];
    let mut done = steps;
    for t in 1..=steps {
        let i = (y * S + x) as usize;
        let c = cells[i] as usize;
        per_colour[c] += 1;
        let d = rule.0[c].delta();
        if d != prev_turn { alt += 1; run += 1; } else {
            if run > 0 { run_hist[(64 - run.leading_zeros() as usize).min(23)] += 1; }
            run = 0;
        }
        prev_turn = d;
        h = (h + d) & 3;
        cells[i] = ((c + 1) % n) as u8;
        vis[i] = vis[i].saturating_add(1);
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if !(0..S).contains(&x) || !(0..S).contains(&y) { done = t; break; }
    }
    std::fs::write(format!("{}.col", a[3]), &cells).unwrap();
    let vb: Vec<u8> = vis.iter().flat_map(|v| v.to_le_bytes()).collect();
    std::fs::write(format!("{}.vis", a[3]), &vb).unwrap();
    let mut o = std::io::stdout().lock();
    writeln!(o, "{{\"rule\":\"{}\",\"steps\":{},\"alt_frac\":{:.5},\"per_colour\":{:?},\"run_log2_hist\":{:?}}}",
        a[1], done, alt as f64 / done as f64, per_colour, run_hist).unwrap();
}
