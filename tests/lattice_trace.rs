//! `lattice_trace` against (G1) `edge_trace::trace_blank` on the square lattice and (G2) an obvious HashMap
//! re-implementation on all three lattices.

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::DIRS;
use turmite::Rule;
use turmite::edge_trace::trace_blank;
use turmite::ext::{HEX_DIRS, hex_turns, tri_step};
use turmite::lattice_probe::Lattice;
use turmite::lattice_trace::*;
use turmite::walkers::AntWalker;

type P = (i64, i64);

fn neighbours(lat: Lattice, p: P) -> Vec<P> {
    match lat {
        Lattice::Square => DIRS.iter().map(|d| (p.0 + d.0 as i64, p.1 + d.1 as i64)).collect(),
        Lattice::Hex => HEX_DIRS.iter().map(|d| (p.0 + d.0 as i64, p.1 + d.1 as i64)).collect(),
        Lattice::Tri => {
            let v = if (p.0 + p.1).rem_euclid(2) == 0 { 1 } else { -1 };
            vec![(p.0 - 1, p.1), (p.0 + 1, p.1), (p.0, p.1 + v)]
        }
    }
}

fn naive(lat: Lattice, rule: &str, size: usize, log2: u32) -> LTrace {
    let c = (size / 2) as i64;
    let turns: Vec<i64> = match lat {
        Lattice::Square => rule.chars().map(|ch| if ch == 'R' { 1 } else { 3 }).collect(),
        Lattice::Hex => hex_turns(rule).unwrap().into_iter().map(|x| x as i64).collect(),
        Lattice::Tri => rule.chars().map(|ch| if ch == 'L' { 2 } else { 4 }).collect(),
    };
    let ell = match (lat, rule.chars().next().unwrap()) {
        (Lattice::Square, _) => 4u32,
        (Lattice::Tri, _) => 6,
        (Lattice::Hex, 'r' | 'l') => 6,
        _ => 3,
    };
    let (mut run, mut run_b) = (0usize, 0u64);
    let mut h: i64 = if lat == Lattice::Tri { 4 } else { 0 };
    let mut pos: P = (0, 0);
    let mut vis: HashSet<P> = HashSet::from([pos]);
    let mut col: HashMap<P, usize> = HashMap::new();
    let mut last: HashMap<P, u64> = HashMap::new();
    let mut nbc: HashMap<P, usize> = HashMap::new(); // tracked cells -> boundary reads so far
    let mut tr = LTrace {
        by_colour: vec![Tally::default(); KC],
        boundary_steps: 0,
        births: 0,
        lag: vec![vec![0; LAGC]; LAGK],
        nb_resolved: vec![0; NBC],
        nb_censored: vec![0; NBC],
        hole_births: 0,
        loop_len: ell,
        run_count: vec![0; RUNC],
        run_boundary: vec![0; RUNC],
        area: 1,
        escaped: false,
    };
    let total = 1u64 << log2;
    for t in 1..=total {
        let win = t > total / 2;
        let ccol = *col.get(&pos).unwrap_or(&0);
        let on_b = neighbours(lat, pos).iter().any(|q| !vis.contains(q));
        if on_b {
            if let Some(v) = nbc.get_mut(&pos) {
                *v += 1;
            }
            if win {
                tr.boundary_steps += 1;
                tr.by_colour[ccol.min(KC - 1)].edge += 1;
                if let Some(&lt) = last.get(&pos) {
                    tr.lag[ccol.min(LAGK - 1)][((t - lt) as usize).min(LAGC - 1)] += 1;
                }
            }
        }
        if win {
            if last.get(&pos).is_some_and(|&lt| t - lt == ell as u64) {
                run += 1;
                run_b += on_b as u64;
            } else if run > 0 {
                tr.run_count[run.min(RUNC - 1)] += 1;
                tr.run_boundary[run.min(RUNC - 1)] += run_b;
                run = 0;
                run_b = 0;
            }
        }
        last.insert(pos, t);
        col.insert(pos, (ccol + 1) % turns.len());
        let turn = turns[ccol];
        match lat {
            Lattice::Square => {
                h = (h + turn).rem_euclid(4);
                pos = (pos.0 + DIRS[h as usize].0 as i64, pos.1 + DIRS[h as usize].1 as i64);
            }
            Lattice::Hex => {
                h = (h + turn).rem_euclid(6);
                pos = (pos.0 + HEX_DIRS[h as usize].0 as i64, pos.1 + HEX_DIRS[h as usize].1 as i64);
            }
            Lattice::Tri => {
                let exit = (h + turn).rem_euclid(6);
                let (dx, dy) = tri_step((pos.0 + pos.1).rem_euclid(2) == 0, exit as u8);
                pos = (pos.0 + dx, pos.1 + dy);
                h = (exit + 3) % 6;
            }
        }
        if vis.insert(pos) {
            tr.area += 1;
            if win {
                tr.births += 1;
                tr.by_colour[ccol.min(KC - 1)].births += 1;
                nbc.insert(pos, 0);
                if neighbours(lat, pos).iter().all(|q| vis.contains(q)) {
                    tr.hole_births += 1;
                }
            }
            // cells that just became interior are resolved (the new cell included)
            let mut cand = neighbours(lat, pos);
            cand.push(pos);
            for q in cand {
                if nbc.contains_key(&q) && neighbours(lat, q).iter().all(|r| vis.contains(r)) {
                    let v = nbc.remove(&q).unwrap();
                    tr.nb_resolved[v.min(NBC - 1)] += 1;
                }
            }
        }
        if pos.0.abs() >= c - 2 || pos.1.abs() >= c - 2 {
            tr.escaped = true;
            break;
        }
    }
    if run > 0 {
        tr.run_count[run.min(RUNC - 1)] += 1;
        tr.run_boundary[run.min(RUNC - 1)] += run_b;
    }
    for (_, v) in nbc {
        tr.nb_censored[v.min(NBC - 1)] += 1;
    }
    tr
}

fn letters(alphabet: &'static str, lo: usize, hi: usize) -> impl Strategy<Value = String> {
    proptest::collection::vec(proptest::sample::select(alphabet.chars().collect::<Vec<_>>()), lo..hi).prop_map(|v| v.into_iter().collect())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    #[test]
    fn g1_square_matches_edge_trace(rule in letters("LR", 2, 12)) {
        let r: Rule = rule.parse().unwrap();
        let a = trace_blank(AntWalker::new(&r), 256, 13);
        let b = lattice_trace(Lattice::Square, &rule, 256, 13);
        prop_assert_eq!(a.escaped, b.escaped);
        prop_assert_eq!(a.births, b.births);
        prop_assert_eq!(a.boundary_steps, b.boundary_steps);
        prop_assert_eq!(a.area, b.area);
        for c in 0..KC {
            prop_assert_eq!((a.by_colour[c].edge, a.by_colour[c].births), (b.by_colour[c].edge, b.by_colour[c].births));
        }
        prop_assert_eq!(a.lag, b.lag);
        prop_assert_eq!(a.nb_resolved, b.nb_resolved);
        prop_assert_eq!(a.nb_censored, b.nb_censored);
    }

    #[test]
    fn g2_square(rule in letters("LR", 2, 12)) {
        prop_assert_eq!(lattice_trace(Lattice::Square, &rule, 128, 13), naive(Lattice::Square, &rule, 128, 13));
    }

    #[test]
    fn g2_hex(rule in letters("rlRL", 2, 12)) {
        prop_assert_eq!(lattice_trace(Lattice::Hex, &rule, 128, 13), naive(Lattice::Hex, &rule, 128, 13));
    }

    #[test]
    fn g2_tri(rule in letters("LR", 2, 12)) {
        prop_assert_eq!(lattice_trace(Lattice::Tri, &rule, 128, 13), naive(Lattice::Tri, &rule, 128, 13));
    }
}
