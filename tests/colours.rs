//! `colours::colour_hists` against a brute-force recount on the HashMap oracle.

use proptest::prelude::*;
use std::collections::HashSet;
use turmite::Rule;
use turmite::colours::colour_hists;
use turmite::reference::Ant;

proptest! {
    #[test]
    fn matches_reference(bits in 0u32..(1 << 9), n in 2usize..=9, k in 6u32..11) {
        let rule = Rule::from_bits(bits, n);
        let cps: Vec<u64> = vec![1 << (k - 2), 1 << (k - 1), 1 << k];
        let (snaps, escaped) = colour_hists(&rule, 512, &cps);
        prop_assume!(!escaped);
        let mut ant = Ant::new(rule);
        let mut seen: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
        for (snap, &cp) in snaps.iter().zip(&cps) {
            while ant.steps < cp {
                ant.step();
                seen.insert(ant.pos);
            }
            let mut hist_all = vec![0u64; n];
            let mut hist_b = vec![0u64; n];
            let (mut area, mut bnd) = (0u64, 0u64);
            for &(x, y) in &seen {
                let c = ant.colour_at((x, y)) as usize;
                area += 1;
                hist_all[c] += 1;
                if [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().any(|(dx, dy)| !seen.contains(&(x + dx, y + dy))) {
                    bnd += 1;
                    hist_b[c] += 1;
                }
            }
            prop_assert_eq!(snap.t, cp);
            prop_assert_eq!(snap.area, area);
            prop_assert_eq!(snap.boundary, bnd);
            prop_assert_eq!(&snap.hist_all, &hist_all);
            prop_assert_eq!(&snap.hist_boundary, &hist_b);
        }
    }
}
