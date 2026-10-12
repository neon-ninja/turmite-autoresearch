//! The probe's incremental bookkeeping, checked against brute force on the
//! reference walkers (ant, rotor, random walk).

use proptest::prelude::*;
use std::collections::HashSet;
use turmite::Rule;
use turmite::probe::{Window, probe, probe_walker};
use turmite::reference::{Ant, RandomWalk, Rotor};
use turmite::walkers;

/// The little a brute-force recount needs from a reference walker.
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

fn brute(rule: Rule, min_log2: u32, max_log2: u32) -> Vec<Window> {
    brute_walk(Ant::new(rule), min_log2, max_log2)
}

/// Recompute every Window field from scratch, the slow obvious way.
fn brute_walk(mut ant: impl RefWalker, min_log2: u32, max_log2: u32) -> Vec<Window> {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let on_boundary = |v: &HashSet<(i32, i32)>, (x, y): (i32, i32)| {
        v.contains(&(x, y)) && [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().any(|(dx, dy)| !v.contains(&(x + dx, y + dy)))
    };
    let mut out = vec![];
    let mut positions = vec![];
    for k in 0..max_log2 {
        let (t_start, t_end) = (1u64 << k, 1u64 << (k + 1));
        let (mut new_cells, mut boundary_steps, mut fresh_nbrs) = (0, 0, 0);
        positions.clear();
        while ant.steps() < t_end {
            if on_boundary(&visited, ant.pos()) {
                boundary_steps += 1;
                let (x, y) = ant.pos();
                fresh_nbrs += [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().filter(|(dx, dy)| !visited.contains(&(x + dx, y + dy))).count() as u64;
            }
            ant.step();
            if visited.insert(ant.pos()) {
                new_cells += 1;
            }
            positions.push(ant.pos());
        }
        if k < min_log2 {
            continue;
        }
        let a = visited.len() as f64;
        let mx = visited.iter().map(|p| p.0 as f64).sum::<f64>() / a;
        let my = visited.iter().map(|p| p.1 as f64).sum::<f64>() / a;
        let d2 = |p: &(i32, i32)| (p.0 as f64 - mx).powi(2) + (p.1 as f64 - my).powi(2);
        let xs = visited.iter().map(|p| p.0);
        let ys = visited.iter().map(|p| p.1);
        out.push(Window {
            t_start,
            t_end,
            area: visited.len() as u64,
            perimeter: visited.iter().filter(|&&p| on_boundary(&visited, p)).count() as u64,
            new_cells,
            boundary_steps,
            fresh_nbrs,
            rg2: visited.iter().map(d2).sum::<f64>() / a,
            ant_r2: positions.iter().map(d2).sum::<f64>() / positions.len() as f64,
            ant_var: {
                let m = positions.len() as f64;
                let px = positions.iter().map(|p| p.0 as f64).sum::<f64>() / m;
                let py = positions.iter().map(|p| p.1 as f64).sum::<f64>() / m;
                positions.iter().map(|p| (p.0 as f64 - px).powi(2) + (p.1 as f64 - py).powi(2)).sum::<f64>() / m
            },
            width: (xs.clone().max().unwrap() - xs.min().unwrap() + 1) as u32,
            height: (ys.clone().max().unwrap() - ys.min().unwrap() + 1) as u32,
        });
    }
    out
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-6 * (1.0 + a.abs().max(b.abs()))
}

fn assert_same(fast: &[Window], slow: &[Window]) {
    assert_eq!(fast.len(), slow.len());
    for (f, s) in fast.iter().zip(slow) {
        let ints = |w: &Window| (w.t_start, w.t_end, w.area, w.perimeter, w.new_cells, w.boundary_steps, w.fresh_nbrs, w.width, w.height);
        assert_eq!(ints(f), ints(s), "window ending {}", s.t_end);
        assert!(close(f.rg2, s.rg2), "rg2 {} vs {} at {}", f.rg2, s.rg2, s.t_end);
        assert!(close(f.ant_r2, s.ant_r2), "ant_r2 {} vs {} at {}", f.ant_r2, s.ant_r2, s.t_end);
        assert!(close(f.ant_var, s.ant_var), "ant_var {} vs {} at {}", f.ant_var, s.ant_var, s.t_end);
    }
}

#[test]
fn langton_first_windows() {
    let r: Rule = "RL".parse().unwrap();
    let res = probe(&r, 512, 1, 14);
    assert!(!res.escaped);
    assert_same(&res.windows, &brute(r, 1, 14));
    // After 4 steps RL has visited exactly a 2x2 square, all on the boundary.
    let w = &res.windows[0];
    assert_eq!((w.t_end, w.area, w.perimeter), (4, 4, 4));
}

#[test]
fn accounting_identity() {
    // Every new cell is entered from a boundary cell, so new <= boundary steps,
    // and the areas telescope.
    let r: Rule = "RLR".parse().unwrap();
    let res = probe(&r, 2048, 1, 20);
    let mut prev_area = res.windows[0].area - res.windows[0].new_cells;
    for w in &res.windows {
        assert!(w.new_cells <= w.boundary_steps);
        assert_eq!(w.area, prev_area + w.new_cells);
        prev_area = w.area;
    }
}

fn arb_rule() -> impl Strategy<Value = Rule> {
    (2usize..=12).prop_flat_map(|n| (0u32..(1 << n)).prop_map(move |b| Rule::from_bits(b, n)))
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]
    #[test]
    fn probe_matches_brute_force(r in arb_rule()) {
        // 2^13 steps can't reach the edge of a 1024 board.
        let res = probe(&r, 1024, 1, 13);
        prop_assert!(!res.escaped);
        assert_same(&res.windows, &brute(r, 1, 13));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]
    #[test]
    fn rotor_probe_matches_brute_force(seed in any::<u64>()) {
        let res = probe_walker(walkers::Rotor { seed }, 512, 1, 14);
        prop_assert!(!res.escaped);
        assert_same(&res.windows, &brute_walk(Rotor::new(seed), 1, 14));
    }

    #[test]
    fn random_walk_probe_matches_brute_force(seed in any::<u64>()) {
        let res = probe_walker(walkers::RandomWalk { state: seed }, 1024, 1, 14);
        prop_assert!(!res.escaped);
        assert_same(&res.windows, &brute_walk(RandomWalk::new(seed), 1, 14));
    }
}

#[test]
fn rotor_follows_kapri_dhar_rule() {
    // Hand-checked first steps: at the origin the arrow rotates clockwise
    // (N -> E -> S -> W -> N) and the walker steps along the new arrow.
    let seed = 7;
    let a0 = walkers::initial_arrow(seed, 0, 0);
    let mut r = Rotor::new(seed);
    r.step();
    let expect = turmite::DIRS[((a0 + 1) % 4) as usize];
    assert_eq!(r.pos, expect);
    assert_eq!(r.arrows[&(0, 0)], (a0 + 1) % 4);
}

#[test]
fn rotor_visits_every_site_of_a_small_disc() {
    // Bou-Rabee & Peres: with uniform random rotors the walk is recurrent and
    // visits every site. After 2^20 steps every site within 50 of the origin
    // should have been visited, for several seeds.
    for seed in 0..5 {
        let mut r = Rotor::new(seed);
        let mut seen = HashSet::new();
        for _ in 0..(1 << 20) {
            r.step();
            seen.insert(r.pos);
        }
        for x in -50..=50 {
            for y in -50..=50 {
                if x * x + y * y <= 2500 {
                    assert!(seen.contains(&(x, y)), "seed {seed}: ({x},{y}) unvisited");
                }
            }
        }
    }
}

#[test]
fn patched_ant_with_blank_patch_is_the_plain_ant() {
    // A patch of radius -1 is empty, so the run must equal the plain ant's.
    let r: Rule = "RLLR".parse().unwrap();
    let plain = probe(&r, 1024, 1, 14);
    let patched = probe_walker(walkers::PatchedAnt::new(&r, 99, -1), 1024, 1, 14);
    assert_same(&patched.windows, &plain.windows);
}

#[test]
fn patched_ant_seeds_differ() {
    let r: Rule = "RLR".parse().unwrap();
    let a = probe_walker(walkers::PatchedAnt::new(&r, 1, 2), 1024, 1, 14);
    let b = probe_walker(walkers::PatchedAnt::new(&r, 2, 2), 1024, 1, 14);
    assert_ne!(a.windows.last().unwrap().area, b.windows.last().unwrap().area);
}
