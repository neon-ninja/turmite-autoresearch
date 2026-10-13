//! Oracle test for the T1b sector instrument: a HashSet-based brute force
//! recomputes every excursion's length, angle, displacement and depth.

use proptest::prelude::*;
use std::collections::HashSet;
use turmite::Rule;
use turmite::reference::{Ant, RandomWalk, Rotor};
use turmite::walkers::AntWalker;

#[path = "../examples/sectors.rs"]
#[allow(dead_code)]
mod sectors_mod;
use sectors_mod::{NBINS, angle_between, len_bin};

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

#[derive(Default, Clone)]
struct BruteBin {
    count: u64,
    time: u64,
    dtheta: f64,
    dcells: f64,
    depth: f64,
}

struct BruteWindow {
    k: u32,
    interior: u64,
    bins: Vec<BruteBin>,
}

fn brute(mut ant: impl RefWalker, min_log2: u32, max_log2: u32) -> Vec<BruteWindow> {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let (mut sx, mut sy) = (0i64, 0i64);
    let on_b = |v: &HashSet<(i32, i32)>, (x, y): (i32, i32)| {
        v.contains(&(x, y)) && [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().any(|(dx, dy)| !v.contains(&(x + dx, y + dy)))
    };
    let centroid = |v: &HashSet<(i32, i32)>, sx: i64, sy: i64| (sx as f64 / v.len() as f64, sy as f64 / v.len() as f64);
    let rad = |p: (i32, i32), c: (f64, f64)| ((p.0 as f64 - c.0).powi(2) + (p.1 as f64 - c.1).powi(2)).sqrt();

    let mut out = vec![];
    let (mut start, mut len, mut min_r) = ((0, 0), 0u64, 0f64);
    for k in 0..max_log2 {
        let t_end = 1u64 << (k + 1);
        let mut bins = vec![BruteBin::default(); NBINS];
        let mut interior = 0;
        while ant.steps() < t_end {
            let prev = ant.pos();
            let was_b = on_b(&visited, prev);
            if !was_b {
                interior += 1;
            }
            ant.step();
            let p = ant.pos();
            if visited.insert(p) {
                sx += p.0 as i64;
                sy += p.1 as i64;
            }
            let is_b = on_b(&visited, p);
            let cen = centroid(&visited, sx, sy);
            if was_b && !is_b {
                start = prev;
                len = 1;
                min_r = rad(p, cen);
            } else if !was_b && !is_b {
                len += 1;
                min_r = min_r.min(rad(p, cen));
            } else if !was_b && is_b && k >= min_log2 {
                let b = &mut bins[len_bin(len)];
                b.count += 1;
                b.time += len;
                b.dtheta += angle_between((start.0 as i64, start.1 as i64), (p.0 as i64, p.1 as i64), cen);
                b.dcells += (((p.0 - start.0) as f64).powi(2) + ((p.1 - start.1) as f64).powi(2)).sqrt();
                b.depth += (rad(start, cen) - min_r).max(0.0);
            }
        }
        if k >= min_log2 {
            out.push(BruteWindow { k, interior, bins });
        }
    }
    out
}

fn nonvacuous(fast: &[sectors_mod::SectorWindow]) {
    assert!(fast.iter().any(|w| w.bins.iter().any(|b| b.count > 0 && b.dtheta_sum > 0.0)), "vacuous");
}

fn compare(brute: &[BruteWindow], fast: &[sectors_mod::SectorWindow]) {
    assert_eq!(brute.len(), fast.len());
    for (b, f) in brute.iter().zip(fast) {
        assert_eq!(b.k, f.k);
        assert_eq!(b.interior, f.interior_steps);
        assert!(f.conserved(), "window {} not conserved: {:?}", f.k, (f.part_completed, f.part_open, f.interior_steps));
        for (bb, fb) in b.bins.iter().zip(&f.bins) {
            assert_eq!(bb.count, fb.count);
            assert_eq!(bb.time, fb.time_sum);
            assert!((bb.dtheta - fb.dtheta_sum).abs() <= 1e-9 * (1.0 + bb.dtheta));
            assert!((bb.dcells - fb.dcells_sum).abs() <= 1e-9 * (1.0 + bb.dcells));
            assert!((bb.depth - fb.depth_sum).abs() <= 1e-9 * (1.0 + bb.depth));
        }
    }
}

#[test]
fn angle_known_values() {
    let c = (0.0, 0.0);
    assert!((angle_between((1, 0), (0, 1), c) - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    assert!((angle_between((1, 0), (-1, 0), c) - std::f64::consts::PI).abs() < 1e-12);
    assert!(angle_between((3, 4), (3, 4), c).abs() < 1e-12);
    assert!((angle_between((0, 1), (1, 0), c) - std::f64::consts::FRAC_PI_2).abs() < 1e-12); // symmetric
    assert_eq!(len_bin(1), 0);
    assert_eq!(len_bin(7), 2);
    assert_eq!(len_bin(8), 3);
}

#[test]
fn langton_matches_brute() {
    let rule: Rule = "RL".parse().unwrap();
    let (fast, esc) = sectors_mod::run_sectors(AntWalker::new(&rule), 1024, 8, 14);
    assert!(!esc);
    nonvacuous(&fast);
    compare(&brute(Ant::new(rule), 8, 14), &fast);
}

#[test]
fn rotor_matches_brute() {
    let (fast, esc) = sectors_mod::run_sectors(turmite::walkers::Rotor { seed: 42 }, 1024, 10, 15);
    assert!(!esc);
    nonvacuous(&fast);
    compare(&brute(Rotor::new(42), 10, 15), &fast);
}

#[test]
fn random_walk_matches_brute() {
    let (fast, esc) = sectors_mod::run_sectors(turmite::walkers::RandomWalk { state: 99 }, 1024, 10, 14);
    assert!(!esc);
    nonvacuous(&fast);
    compare(&brute(RandomWalk::new(99), 10, 14), &fast);
}

proptest! {
    #[test]
    fn rules_match_brute(rule_str in "(R|L){3,6}") {
        if let Ok(rule) = rule_str.parse::<Rule>() {
            let (fast, esc) = sectors_mod::run_sectors(AntWalker::new(&rule), 512, 8, 13);
            if !esc {
                compare(&brute(Ant::new(rule), 8, 13), &fast);
            }
        }
    }
}
