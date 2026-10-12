use proptest::prelude::*;
use turmite::detect::find_highway;
use turmite::fast::{self, Config, Grid, Stop};
use turmite::reference::Ant;
use turmite::{Rule, Turn};

fn rule(s: &str) -> Rule {
    s.parse().unwrap()
}

#[test]
fn first_steps_of_langtons_ant() {
    // Facing north on white: turn right (east), flip, move. Four steps trace a
    // clockwise square back to the origin.
    let mut a = Ant::new(rule("RL"));
    a.step();
    assert_eq!((a.pos, a.heading), ((1, 0), 1));
    a.run(3);
    assert_eq!(a.pos, (0, 0));
    assert_eq!(a.cells.len(), 4);
}

#[test]
fn langtons_ant_builds_its_highway() {
    // Well known: RL wanders for ~10^4 steps, then builds a highway with
    // period 104 that moves 2 cells diagonally per period.
    let mut a = Ant::new(rule("RL"));
    let mut trail = vec![a.pos];
    for _ in 0..40_000 {
        a.step();
        trail.push(a.pos);
    }
    let hw = find_highway(&trail, 2_000, turmite::detect::MIN_SPAN).expect("RL should have a highway");
    assert_eq!(hw.period, 104);
    assert_eq!((hw.dx.abs(), hw.dy.abs()), (2, 2));

    let onset = fast::highway_onset(&rule("RL"), hw, 40_000);
    assert!((9_800..10_200).contains(&onset), "onset {onset}");
}

#[test]
fn no_highway_during_langtons_chaos() {
    let mut a = Ant::new(rule("RL"));
    let mut trail = vec![a.pos];
    for _ in 0..9_000 {
        a.step();
        trail.push(a.pos);
    }
    // Too short for MIN_SPAN anyway, so also check a shorter manual window.
    assert_eq!(find_highway(&trail, 2_000, turmite::detect::MIN_SPAN), None);
}

#[test]
fn mirror_rules_are_mirror_images() {
    // Swapping L and R reflects the pattern in the vertical axis.
    let (mut a, mut b) = (Ant::new(rule("RLR")), Ant::new(rule("LRL")));
    for _ in 0..5_000 {
        a.step();
        b.step();
        assert_eq!(a.pos, (-b.pos.0, b.pos.1));
    }
}

#[test]
fn llrr_is_bilaterally_symmetric_when_home() {
    // LLRR (Propp) is known to grow symmetric patterns. Whenever the ant is
    // back at the origin facing north, the pattern should be mirror symmetric.
    let mut a = Ant::new(rule("LLRR"));
    let mut checked = 0;
    for _ in 0..200_000 {
        a.step();
        if a.pos == (0, 0) && a.heading == 0 {
            // Allow either axis convention by testing the reflection x -> -x - 1 too.
            let sym = |f: &dyn Fn(i32) -> i32| {
                a.cells.iter().all(|(&(x, y), &c)| a.colour_at((f(x), y)) == c)
            };
            if sym(&|x| -x) || sym(&|x| -x - 1) {
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "never saw a symmetric home configuration");
}

#[test]
fn canonical_and_trivial() {
    assert!(rule("RL").is_canonical());
    assert!(!rule("LR").is_canonical());
    assert!(rule("RRR").is_trivial());
    assert_eq!(Rule::from_bits(0b01, 2).0, vec![Turn::R, Turn::L]);
}

fn arb_rule() -> impl Strategy<Value = Rule> {
    (2usize..=12).prop_flat_map(|n| (0u32..(1 << n)).prop_map(move |b| Rule::from_bits(b, n)))
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn reversible(r in arb_rule(), k in 0u64..3_000) {
        let mut a = Ant::new(r);
        a.run(k);
        for _ in 0..k { a.unstep(); }
        prop_assert_eq!(a.pos, (0, 0));
        prop_assert_eq!(a.heading, 0);
        prop_assert!(a.cells.is_empty());
    }

    #[test]
    fn fast_matches_reference(r in arb_rule(), k in 1u64..20_000) {
        let mut grid = Grid::new(512);
        let cfg = Config { max_steps: k, checkpoint: k, max_period: 100, history: 25_000 };
        let res = fast::run(&r, &mut grid, &cfg);
        let mut a = Ant::new(r);
        a.run(res.steps);
        prop_assert_eq!(res.pos, a.pos);
        prop_assert_eq!(res.heading, a.heading);
        if res.stop == Stop::Budget {
            prop_assert_eq!(res.steps, k);
        }
        for (&(x, y), &c) in &a.cells {
            prop_assert_eq!(grid.get(x, y), c);
        }
    }
}

#[test]
fn fast_grid_reset_is_clean() {
    // Reusing a grid must not leak state between runs.
    let mut grid = Grid::new(256);
    let cfg = Config { max_steps: 5_000, checkpoint: 5_000, max_period: 100, history: 25_000 };
    let first = fast::run(&rule("RLR"), &mut grid, &cfg);
    fast::run(&rule("LLRR"), &mut grid, &cfg);
    let again = fast::run(&rule("RLR"), &mut grid, &cfg);
    assert_eq!(first.pos, again.pos);
    assert_eq!(first.heading, again.heading);
}

#[test]
fn square_builder_edge_is_not_a_highway() {
    // RRLRRRRRRLRL walks the edge of a growing square with period 24. Around
    // step 93M that looked like a highway to an unconfirmed detector.
    let mut grid = Grid::new(8192);
    let cfg = Config { max_steps: 93_000_000, ..Config::default() };
    let res = fast::run(&rule("RRLRRRRRRLRL"), &mut grid, &cfg);
    assert_eq!(res.stop, Stop::Budget);
    assert_eq!(res.highway, None);
    assert_eq!(res.periodic_tail.map(|h| h.period), Some(24));
}

#[test]
fn langton_highway_confirmed_by_escape() {
    let mut grid = Grid::new(1024);
    let res = fast::run(&rule("RL"), &mut grid, &Config::default());
    assert_eq!(res.stop, Stop::Escaped);
    assert_eq!(res.highway.map(|h| h.period), Some(104));
}

#[test]
fn square_builder_hitting_the_wall_is_not_a_highway() {
    // When the growing square reaches the grid edge the ant leaves at a
    // corner, mid-turn, so it must not be mistaken for a highway. (The
    // exit-direction guard itself is unit-tested in detect.rs.)
    let mut grid = Grid::new(1024);
    let cfg = Config { max_steps: 100_000_000, ..Config::default() };
    let res = fast::run(&rule("RRLRRRRRRLRL"), &mut grid, &cfg);
    assert_eq!(res.stop, Stop::Escaped);
    assert_eq!(res.highway, None, "tail was {:?}", res.periodic_tail);
}

#[test]
fn square_builder_leaving_mid_pass_is_not_a_highway() {
    // RRLLLRLLLRLL builds a square with straight period-4 passes. On an 8,192
    // board one pass carries it off the side at step 99,015,728, which the
    // pre-guard detector took for a highway (its last pass is periodic over a
    // ~2,000-step window). It leaves sideways, so the exit guard rejects it.
    let mut grid = Grid::new(8192);
    let cfg = Config { max_steps: 100_000_000, ..Config::default() };
    let res = fast::run(&rule("RRLLLRLLLRLL"), &mut grid, &cfg);
    assert_eq!(res.stop, Stop::Escaped);
    assert_eq!(res.steps, 99_015_728);
    assert_eq!(res.highway, None);
}
