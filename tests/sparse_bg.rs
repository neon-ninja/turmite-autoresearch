//! `SparseBgAnt` under the probe against a HashMap brute force (area at each octave end).

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::probe::probe_walker;
use turmite::walkers::{SparseBgAnt, sparse_bg_colour};
use turmite::{DIRS, Rule};

proptest! {
    #[test]
    fn area_matches_brute_force(bits in 0u32..(1 << 7), n in 2usize..=7, seed in 0u64..1000, dens in 0u32..=60) {
        let rule = Rule::from_bits(bits, n);
        let d = dens as f64 / 100.0;
        let w = SparseBgAnt::new(&rule, seed, d);
        let q = w.density_q32;
        let res = probe_walker(w, 1024, 4, 12);
        prop_assume!(!res.escaped);
        let mut cells: HashMap<(i32, i32), u8> = HashMap::new();
        let (mut pos, mut h) = ((0i32, 0i32), 0u8);
        let mut seen: HashSet<(i32, i32)> = HashSet::from([pos]);
        let mut t = 0u64;
        for win in &res.windows {
            while t < win.t_end {
                let c = *cells.entry(pos).or_insert_with(|| sparse_bg_colour(seed, q, n as u8, pos.0 as i64, pos.1 as i64));
                h = (h + rule.0[c as usize].delta()) & 3;
                cells.insert(pos, (c + 1) % n as u8);
                let (dx, dy) = DIRS[h as usize];
                pos = (pos.0 + dx, pos.1 + dy);
                seen.insert(pos);
                t += 1;
            }
            prop_assert_eq!(win.area, seen.len() as u64);
        }
    }
}
