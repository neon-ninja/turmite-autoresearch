//! `BiasedRotor` under the probe against a HashMap brute force (area at each octave end).

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::DIRS;
use turmite::probe::probe_walker;
use turmite::walkers::{BiasedRotor, biased_arrow};

proptest! {
    #[test]
    fn area_matches_brute_force(seed in 0u64..1000, bias in 0u32..=100) {
        let b = bias as f64 / 100.0;
        let w = BiasedRotor::new(seed, b);
        let res = probe_walker(w, 1024, 4, 12);
        prop_assume!(!res.escaped);
        let q = BiasedRotor::new(seed, b).bias_q32;
        let mut arrows: HashMap<(i32, i32), u8> = HashMap::new();
        let mut pos = (0i32, 0i32);
        let mut seen: HashSet<(i32, i32)> = HashSet::from([pos]);
        let mut t = 0u64;
        for win in &res.windows {
            while t < win.t_end {
                let a = arrows.entry(pos).or_insert_with(|| biased_arrow(seed, q, pos.0 as i64, pos.1 as i64));
                *a = (*a + 1) % 4;
                let (dx, dy) = DIRS[*a as usize];
                pos = (pos.0 + dx, pos.1 + dy);
                seen.insert(pos);
                t += 1;
            }
            prop_assert_eq!(win.area, seen.len() as u64);
        }
    }
}
