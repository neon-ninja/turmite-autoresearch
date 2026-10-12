//! Frontier item 23: do distinct canonical rules ever give identical
//! trajectories? Hashes the first T positions of every canonical non-trivial
//! rule of length MIN..=MAX and groups collisions. Also checks that the
//! mirror (L<->R) of each rule is the x-reflected trajectory.
//!
//! usage: equiv MIN MAX T
use std::collections::HashMap;
use turmite::reference::Ant;
use turmite::{Rule, Turn};

fn traj_hash(rule: &Rule, t: usize, mirror: bool) -> u64 {
    let mut a = Ant::new(rule.clone());
    let mut h: u64 = 0xcbf29ce484222325;
    for _ in 0..t {
        a.step();
        let x = if mirror { -a.pos.0 } else { a.pos.0 };
        for v in [x as i64, a.pos.1 as i64] {
            h ^= v as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (lo, hi, t): (usize, usize, usize) = (a[1].parse().unwrap(), a[2].parse().unwrap(), a[3].parse().unwrap());
    let mut groups: HashMap<u64, Vec<String>> = HashMap::new();
    let mut total = 0;
    let mut mirror_bad = 0;
    for n in lo..=hi {
        for b in 0..1u32 << n {
            let r = Rule::from_bits(b, n);
            if !r.is_canonical() || r.is_trivial() {
                continue;
            }
            total += 1;
            groups.entry(traj_hash(&r, t, false)).or_default().push(r.to_string());
            let m = Rule(r.0.iter().map(|x| if *x == Turn::L { Turn::R } else { Turn::L }).collect());
            // Mirror ant starts heading north too; reflecting x maps R-turns to L-turns.
            if traj_hash(&m, t, false) != traj_hash(&r, t, true) {
                mirror_bad += 1;
            }
        }
    }
    let mut coll: Vec<_> = groups.values().filter(|g| g.len() > 1).collect();
    coll.sort();
    println!("rules {total}, distinct trajectories {}, mirror mismatches {mirror_bad}", groups.len());
    let mut periodic = 0;
    for g in &coll {
        println!("{}", g.join(" "));
    }
    for g in &coll {
        let _ = &mut periodic;
    }
}
