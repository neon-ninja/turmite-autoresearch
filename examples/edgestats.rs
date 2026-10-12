//! edgestats K RULE... : for each rule, run 2^K steps from a blank board and,
//! over the last octave, break down steps that start on an edge cell (visited,
//! with an unvisited neighbour) by the colour of that cell: how many such
//! steps, and how many entered fresh ground. Also: of the edge steps, how
//! often the cell ahead / to the left / to the right was fresh.
use turmite::{DIRS, Rule};

const S: usize = 4096;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let k: u32 = a[1].parse().unwrap();
    println!("rule,colour,turn,edge_steps,new_cells,ahead_fresh,left_fresh,right_fresh");
    for r in &a[2..] {
        let rule: Rule = r.parse().unwrap();
        let n = rule.len();
        let mut colour = vec![0u8; S * S];
        let mut visited = vec![false; S * S];
        let (mut x, mut y, mut h) = (S / 2, S / 2, 0u8);
        visited[y * S + x] = true;
        let idx = |x: usize, y: usize| y * S + x;
        let step_from = |x: usize, y: usize, h: u8| {
            let (dx, dy) = DIRS[h as usize];
            (x.wrapping_add_signed(dx as isize), y.wrapping_add_signed(dy as isize))
        };
        // per colour: edge steps, new, ahead fresh, left fresh, right fresh
        let mut tally = vec![[0u64; 5]; n];
        let half = 1u64 << (k - 1);
        for t in 1..=(1u64 << k) {
            let i = idx(x, y);
            let c = colour[i] as usize;
            let fresh = |h: u8| {
                let (nx, ny) = step_from(x, y, h);
                !visited[idx(nx, ny)]
            };
            let on_edge = (0..4).any(|d| fresh(d));
            let (ahead, left, right) = (fresh(h), fresh((h + 3) & 3), fresh((h + 1) & 3));
            h = (h + rule.0[c].delta()) & 3;
            colour[i] = ((c + 1) % n) as u8;
            (x, y) = step_from(x, y, h);
            assert!(x > 0 && y > 0 && x < S - 1 && y < S - 1, "{r} reached the edge of the board");
            let j = idx(x, y);
            let new = !visited[j];
            visited[j] = true;
            if t > half && on_edge {
                let e = &mut tally[c];
                e[0] += 1;
                e[1] += new as u64;
                e[2] += ahead as u64;
                e[3] += left as u64;
                e[4] += right as u64;
            }
        }
        for (c, e) in tally.iter().enumerate() {
            let turn = match rule.0[c] { turmite::Turn::L => 'L', turmite::Turn::R => 'R' };
            println!("{r},{c},{turn},{},{},{},{},{}", e[0], e[1], e[2], e[3], e[4]);
        }
    }
}
