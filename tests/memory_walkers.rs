//! Independent sparse references: cookie exhaustion and shared undirected edges.
use std::collections::{BTreeMap, HashSet};
use proptest::prelude::*;
use turmite::{DIRS, walkers::{self, Walker}};

proptest! {
    #![proptest_config(ProptestConfig { cases: 24, rng_seed: proptest::test_runner::RngSeed::Fixed(20261013), ..ProptestConfig::default() })]
    #[test]
    fn cookies_match_reference(seed in any::<u64>(), random in any::<bool>(), bias in 0u32..=4) {
        let mut w = walkers::CookieWalk::new(seed, bias as f64 / 4.0, random);
        let mut cells = BTreeMap::new();
        let mut used = HashSet::new();
        let mut state = seed;
        let mut pos = (0i64, 0i64);
        let mut h = 0;
        for _ in 0..8192 {
            let r = walkers::splitmix_next(&mut state);
            let arrow = if random { walkers::initial_arrow(seed, pos.0, pos.1) } else { 1 };
            let fresh = used.insert(pos);
            let d = if fresh && (r >> 32) < bias as u64 * (1u64 << 30) { arrow } else { (r % 4) as u8 };
            let cell = cells.entry(pos).or_insert_with(|| w.initial_cell(pos.0, pos.1));
            w.step(cell, &mut h);
            prop_assert_eq!(h, d);
            prop_assert_eq!(*cell, arrow | 4);
            let (dx, dy) = DIRS[d as usize];
            pos = (pos.0 + dx as i64, pos.1 + dy as i64);
        }
    }

    #[test]
    fn reinforced_edges_match_reference(seed in any::<u64>(), increment in 0u64..=4) {
        let mut w = walkers::EdgeReinforcedWalk::new(seed, increment);
        let mut weights = BTreeMap::new();
        let mut pos = (0i64, 0i64);
        let mut state = seed;
        let mut h = 0;
        for _ in 0..8192 {
            let mut candidates = vec![];
            let mut total = 0u64;
            for (dx, dy) in DIRS {
                let q = (pos.0 + dx as i64, pos.1 + dy as i64);
                let e = if pos < q { (pos, q) } else { (q, pos) };
                total += *weights.get(&e).unwrap_or(&1);
                candidates.push((e, q, total));
            }
            let limit = total.wrapping_neg() % total;
            let draw = loop {
                let r = walkers::splitmix_next(&mut state);
                if r >= limit { break r % total; }
            };
            let d = candidates.iter().position(|c| draw < c.2).unwrap();
            let (edge, q, _) = candidates[d];
            *weights.entry(edge).or_insert(1) += increment;
            w.step(&mut 0, &mut h);
            prop_assert_eq!(h as usize, d);
            pos = q;
        }
    }
}

#[test]
fn zero_cookie_bias_is_identical_to_random_walk() {
    for random in [false, true] {
        let a = turmite::probe::probe_walker(walkers::CookieWalk::new(7, 0.0, random), 512, 8, 14);
        let b = turmite::probe::probe_walker(walkers::RandomWalk { state: 7 }, 512, 8, 14);
        assert_eq!(a.windows, b.windows);
        assert_eq!(a.escaped, b.escaped);
    }
}
