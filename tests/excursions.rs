//! Property tests for excursion tomography comparing fast against brute force oracle.

use proptest::prelude::*;
use std::collections::HashSet;
use turmite::Rule;
use turmite::reference::{Ant, RandomWalk, Rotor};
use turmite::walkers::AntWalker;

trait RefWalker {
    fn pos(&self) -> (i32, i32);
    fn steps(&self) -> u64;
    fn step(&mut self);
}

macro_rules! ref_walker {
    ($t:ty) => {
        impl RefWalker for $t {
            fn pos(&self) -> (i32, i32) { self.pos }
            fn steps(&self) -> u64 { self.steps }
            fn step(&mut self) { <$t>::step(self) }
        }
    };
}
ref_walker!(Ant);
ref_walker!(Rotor);
ref_walker!(RandomWalk);

#[derive(Debug, PartialEq)]
struct BruteWindow {
    k: u32,
    boundary_steps: u64,
    interior_steps: u64,
    n_completed: u64,
    l_max: u64,
    l_sum: u64,
}

fn brute_excursions(mut ant: impl RefWalker, min_log2: u32, max_log2: u32) -> Vec<BruteWindow> {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let on_boundary = |v: &HashSet<(i32, i32)>, (x, y): (i32, i32)| {
        v.contains(&(x, y)) && [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().any(|(dx, dy)| !v.contains(&(x + dx, y + dy)))
    };

    let mut out = vec![];
    let mut exc_len = 0u64;

    for k in 0..max_log2 {
        let (_t_start, t_end) = (1u64 << k, 1u64 << (k + 1));
        let record = k >= min_log2;

        let mut boundary_steps = 0u64;
        let mut interior_steps = 0u64;
        let mut completed = vec![];

        while ant.steps() < t_end {
            let was_b = on_boundary(&visited, ant.pos());
            if was_b {
                boundary_steps += 1;
            } else {
                interior_steps += 1;
            }

            ant.step();
            visited.insert(ant.pos());
            let is_b = on_boundary(&visited, ant.pos());

            if was_b && !is_b {
                exc_len = 1;
            } else if !was_b && !is_b {
                exc_len += 1;
            } else if !was_b && is_b {
                if record {
                    completed.push(exc_len);
                }
                exc_len = 0;
            }
        }

        if record {
            let l_max = completed.iter().copied().max().unwrap_or(0);
            let l_sum: u64 = completed.iter().sum();
            out.push(BruteWindow {
                k,
                boundary_steps,
                interior_steps,
                n_completed: completed.len() as u64,
                l_max,
                l_sum,
            });
        }
    }

    out
}

#[path = "../examples/excursions.rs"]
mod excursions_mod;

#[test]
fn langton_excursions_match_brute_force() {
    let rule: Rule = "RL".parse().unwrap();
    let brute = brute_excursions(Ant::new(rule.clone()), 8, 13);
    let (fast, escaped) = excursions_mod::run_excursions(AntWalker::new(&rule), 1024, 8, 13);
    assert!(!escaped);
    assert_eq!(brute.len(), fast.len());

    for (b, f) in brute.iter().zip(fast.iter()) {
        assert_eq!(b.k, f.k);
        assert_eq!(b.boundary_steps, f.boundary_steps);
        assert_eq!(b.interior_steps, f.interior_steps);
        assert_eq!(b.boundary_steps + b.interior_steps, 1u64 << b.k);
        assert_eq!(b.n_completed, f.n_completed);
        assert_eq!(b.l_max, f.l_max);
    }
}

#[test]
fn rotor_excursions_match_brute_force() {
    let brute = brute_excursions(Rotor::new(42), 10, 15);
    let (fast, escaped) = excursions_mod::run_excursions(turmite::walkers::Rotor { seed: 42 }, 1024, 10, 15);
    assert!(!escaped);
    assert_eq!(brute.len(), fast.len());

    for (b, f) in brute.iter().zip(fast.iter()) {
        assert_eq!(b.k, f.k);
        assert_eq!(b.boundary_steps, f.boundary_steps);
        assert_eq!(b.interior_steps, f.interior_steps);
        assert_eq!(b.boundary_steps + b.interior_steps, 1u64 << b.k);
        assert_eq!(b.n_completed, f.n_completed);
        assert_eq!(b.l_max, f.l_max);
    }
}

#[test]
fn random_walk_excursions_match_brute_force() {
    let brute = brute_excursions(RandomWalk::new(99), 10, 14);
    let (fast, escaped) = excursions_mod::run_excursions(turmite::walkers::RandomWalk { state: 99 }, 1024, 10, 14);
    assert!(!escaped);
    assert_eq!(brute.len(), fast.len());

    for (b, f) in brute.iter().zip(fast.iter()) {
        assert_eq!(b.k, f.k);
        assert_eq!(b.boundary_steps, f.boundary_steps);
        assert_eq!(b.interior_steps, f.interior_steps);
        assert_eq!(b.boundary_steps + b.interior_steps, 1u64 << b.k);
        assert_eq!(b.n_completed, f.n_completed);
        assert_eq!(b.l_max, f.l_max);
    }
}

proptest! {
    #[test]
    fn various_rules_match_brute_force(
        rule_str in "(R|L){3,6}"
    ) {
        if let Ok(rule) = rule_str.parse::<Rule>() {
            let brute = brute_excursions(Ant::new(rule.clone()), 8, 13);
            let (fast, escaped) = excursions_mod::run_excursions(AntWalker::new(&rule), 512, 8, 13);
            if !escaped {
                prop_assert_eq!(brute.len(), fast.len());
                for (b, f) in brute.iter().zip(fast.iter()) {
                    prop_assert_eq!(b.boundary_steps, f.boundary_steps);
                    prop_assert_eq!(b.interior_steps, f.interior_steps);
                    prop_assert_eq!(b.boundary_steps + b.interior_steps, 1u64 << b.k);
                    prop_assert_eq!(b.n_completed, f.n_completed);
                    prop_assert_eq!(b.l_max, f.l_max);
                }
            }
        }
    }
}
