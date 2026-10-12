//! occupancy RULE K OUT : run 2^K steps on a 4096 board; write two u32 arrays
//! (4096^2 each, little-endian) to OUT: time spent per cell during the last
//! octave (2^(K-1), 2^K], then a visited flag (1/0) at the end.
use turmite::{DIRS, Rule};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let k: u32 = a[2].parse().unwrap();
    const S: usize = 4096;
    let n = rule.len() as u8;
    let mut colour = vec![0u8; S * S];
    let mut visited = vec![0u32; S * S];
    let mut time = vec![0u32; S * S];
    let (mut x, mut y, mut h) = (S / 2, S / 2, 0u8);
    visited[y * S + x] = 1;
    let half = 1u64 << (k - 1);
    for t in 1..=(1u64 << k) {
        let i = y * S + x;
        let c = colour[i];
        h = (h + rule.0[c as usize].delta()) & 3;
        colour[i] = (c + 1) % n;
        let (dx, dy) = DIRS[h as usize];
        x = x.wrapping_add_signed(dx as isize);
        y = y.wrapping_add_signed(dy as isize);
        assert!(x < S && y < S, "left the board at step {t}");
        let j = y * S + x;
        visited[j] = 1;
        if t > half {
            time[j] += 1;
        }
    }
    let mut out = Vec::with_capacity(S * S * 8);
    for v in time.iter().chain(visited.iter()) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    std::fs::write(&a[3], out).unwrap();
}
