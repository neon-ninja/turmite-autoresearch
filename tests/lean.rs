//! `run_lean` must agree with `fast::run` on every field of `RunResult` (frontier item 31).
use proptest::prelude::*;
use turmite::Rule;
use turmite::fast::{self, Config, Grid};
use turmite::lean::{LeanGrid, run_lean};

proptest! {
    #[test]
    fn lean_equals_fast(bits in 1u32..4096, n in 2usize..12, size in 24usize..200, steps in 1u64..60_000, hist in 100usize..30_000) {
        let rule = Rule::from_bits(bits, n);
        let cfg = Config { max_steps: steps, checkpoint: 1_000, max_period: 5_000, history: hist };
        let mut g = Grid::new(size);
        let a = fast::run(&rule, &mut g, &cfg);
        let mut lg = LeanGrid::new(size);
        let b = run_lean(&rule, &mut lg, &cfg);
        prop_assert_eq!(a.steps, b.steps);
        prop_assert_eq!(a.stop, b.stop);
        prop_assert_eq!(a.pos, b.pos);
        prop_assert_eq!(a.heading, b.heading);
        prop_assert_eq!(a.highway, b.highway);
        prop_assert_eq!(a.periodic_tail, b.periodic_tail);
        prop_assert_eq!(a.extent, b.extent);
    }
}

#[test]
fn langton_highway_matches() {
    let rule: Rule = "RL".parse().unwrap();
    let cfg = Config { max_steps: 200_000, ..Config::default() };
    let a = fast::run(&rule, &mut Grid::new(512), &cfg);
    let b = run_lean(&rule, &mut LeanGrid::new(512), &cfg);
    assert_eq!((a.steps, a.pos, a.heading, a.highway, a.extent), (b.steps, b.pos, b.heading, b.highway, b.extent));
    assert!(b.highway.is_some());
}

#[test]
fn flat_onset_matches_hashmap_onset() {
    for r in ["RL", "RRL", "RLRL", "RRLL"] {
        let rule: Rule = r.parse().unwrap();
        let cfg = Config { max_steps: 400_000, ..Config::default() };
        let res = run_lean(&rule, &mut LeanGrid::new(1024), &cfg);
        if let Some(hw) = res.highway {
            assert_eq!(turmite::lean::highway_onset_flat(&rule, hw, res.steps, 1024), fast::highway_onset(&rule, hw, res.steps), "{r}");
        }
    }
    // Langton's ant: the published onset is 9976
    let rule: Rule = "RL".parse().unwrap();
    let res = run_lean(&rule, &mut LeanGrid::new(1024), &Config { max_steps: 400_000, ..Config::default() });
    assert_eq!(turmite::lean::highway_onset_flat(&rule, res.highway.unwrap(), res.steps, 1024), 9976);
}
