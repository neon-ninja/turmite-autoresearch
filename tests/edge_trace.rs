//! `edge_trace::trace` against (a) `probe_walker`'s last-octave window and (b) a naive HashMap brute force of every field,
//! on blank boards (ants, sparse background, rotor, random walk) and on a prefilled torus band.

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::edge_trace::*;
use turmite::probe::probe_walker;
use turmite::walkers::{AntWalker, RandomWalk, Rotor, SparseBgAnt, Walker, mix64};
use turmite::{DIRS, Rule};

type P = (i64, i64);

/// Obvious, slow re-implementation of `trace` (same semantics, no incremental bookkeeping).
fn brute<W: Walker>(mut walker: W, lay: Layout, prefill: &dyn Fn(usize, usize) -> Option<u8>, start: (usize, usize, u8), log2: u32) -> EdgeTrace {
    let (w, h) = (lay.w as i64, lay.h as i64);
    let (cx, cy) = (w / 2, h / 2);
    let norm = |p: P| if lay.wrap { (p.0.rem_euclid(w), p.1.rem_euclid(h)) } else { p };
    let mut vis: HashSet<P> = HashSet::new();
    let mut col: HashMap<P, u8> = HashMap::new();
    for gy in 0..h {
        for gx in 0..w {
            let init = if W::RANDOM_INIT { walker.initial_cell(gx - cx, gy - cy) } else { 0 };
            match prefill(gx as usize, gy as usize) {
                Some(c) => {
                    vis.insert((gx, gy));
                    col.insert((gx, gy), if c == u8::MAX { init } else { c });
                }
                None => {
                    col.insert((gx, gy), init);
                }
            }
        }
    }
    let nb4 = |p: P| -> Vec<P> { (0..4).map(|d| norm((p.0 + DIRS[d].0 as i64, p.1 + DIRS[d].1 as i64))).collect() };
    let fresh_count = |vis: &HashSet<P>, p: P| nb4(p).iter().filter(|q| !vis.contains(q)).count() as u64;
    let on_b = |vis: &HashSet<P>, p: P| vis.contains(&p) && fresh_count(vis, p) > 0;
    let mut pos: P = (start.0 as i64, start.1 as i64);
    let mut hd = start.2;
    let mut last_t: HashMap<P, u64> = HashMap::new();
    let mut nb: HashMap<P, u64> = HashMap::new();
    let mut tracked: HashSet<P> = HashSet::new();
    let mut tr = EdgeTrace {
        by_colour: vec![Tally::default(); KC],
        boundary_steps: 0,
        births: 0,
        fresh_nbrs: 0,
        lag: vec![vec![0; LAGC]; LAGK],
        nb_resolved: vec![0; NBC],
        nb_censored: vec![0; NBC],
        chain: vec![0; CHC],
        area: 0,
        perimeter: 0,
        area_t0: 0,
        escaped: false,
    };
    let total = 1u64 << log2;
    let t0 = total / 2;
    let mut run = 0usize;
    for t in 1..=total {
        let in_win = t > t0;
        let b = on_b(&vis, pos);
        let c = col[&pos];
        if b {
            let e = nb.entry(pos).or_insert(0);
            *e = (*e + 1).min(0x7FFF);
            if in_win {
                tr.boundary_steps += 1;
                tr.fresh_nbrs += fresh_count(&vis, pos);
                let e = &mut tr.by_colour[(c as usize).min(KC - 1)];
                e.edge += 1;
                let fresh = |d: usize| {
                    let q = norm((pos.0 + DIRS[d & 3].0 as i64, pos.1 + DIRS[d & 3].1 as i64));
                    !vis.contains(&q)
                };
                e.ahead += fresh(hd as usize) as u64;
                e.left += fresh(hd as usize + 3) as u64;
                e.right += fresh(hd as usize + 1) as u64;
                if let Some(&lt) = last_t.get(&pos) {
                    tr.lag[(c as usize).min(LAGK - 1)][((t - lt) as usize).min(LAGC - 1)] += 1;
                }
            }
        }
        last_t.insert(pos, t);
        let mut cell = c;
        walker.step(&mut cell, &mut hd);
        col.insert(pos, cell);
        pos = norm((pos.0 + DIRS[hd as usize].0 as i64, pos.1 + DIRS[hd as usize].1 as i64));
        let born = !vis.contains(&pos);
        if born {
            vis.insert(pos);
            if in_win {
                tr.births += 1;
                tr.by_colour[(c as usize).min(KC - 1)].births += 1;
                tracked.insert(pos);
            }
        }
        let gone: Vec<P> = tracked.iter().copied().filter(|&p| !on_b(&vis, p)).collect();
        for p in gone {
            tr.nb_resolved[(nb.get(&p).copied().unwrap_or(0) as usize).min(NBC - 1)] += 1;
            tracked.remove(&p);
        }
        if t == t0 {
            tr.area_t0 = vis.len() as u64;
        }
        if in_win {
            if born {
                run += 1;
            } else if run > 0 {
                tr.chain[run.min(CHC - 1)] += 1;
                run = 0;
            }
        }
        if !lay.wrap && (pos.0 <= 2 || pos.1 <= 2 || pos.0 >= w - 2 || pos.1 >= h - 2) {
            tr.escaped = true;
            break;
        }
    }
    if run > 0 {
        tr.chain[run.min(CHC - 1)] += 1;
    }
    for p in &tracked {
        tr.nb_censored[(nb.get(p).copied().unwrap_or(0) as usize).min(NBC - 1)] += 1;
    }
    tr.area = vis.len() as u64;
    tr.perimeter = vis.iter().filter(|&&p| fresh_count(&vis, p) > 0).count() as u64;
    tr
}

fn blank_prefill(size: usize) -> impl Fn(usize, usize) -> Option<u8> {
    let c = size / 2;
    move |x, y| if (x, y) == (c, c) { Some(u8::MAX) } else { None }
}

fn check_blank<W: Walker>(mk: impl Fn() -> W, size: usize, log2: u32) -> Result<(), TestCaseError> {
    let a = trace_blank(mk(), size, log2);
    prop_assume!(!a.escaped);
    // (a) the probe's last octave
    let pr = probe_walker(mk(), size, log2 - 1, log2);
    prop_assert!(!pr.escaped);
    let win = pr.windows.last().unwrap();
    prop_assert_eq!(a.births, win.new_cells);
    prop_assert_eq!(a.boundary_steps, win.boundary_steps);
    prop_assert_eq!(a.fresh_nbrs, win.fresh_nbrs);
    prop_assert_eq!(a.area, win.area);
    prop_assert_eq!(a.perimeter, win.perimeter);
    // (b) brute force
    let c = size / 2;
    let b = brute(mk(), Layout { w: size, h: size, wrap: false }, &blank_prefill(size), (c, c, 0), log2);
    prop_assert_eq!(a, b);
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(40))]
    #[test]
    fn ant_matches_probe_and_brute(bits in 0u32..(1 << 7), n in 2usize..=7, log2 in 9u32..=12) {
        let rule = Rule::from_bits(bits, n);
        check_blank(|| AntWalker::new(&rule), 256, log2)?;
    }

    #[test]
    fn sparse_bg_matches(bits in 0u32..(1 << 7), n in 3usize..=7, seed in 0u64..1000, dens in 0u32..=90, log2 in 9u32..=12) {
        let rule = Rule::from_bits(bits, n);
        check_blank(|| SparseBgAnt::new(&rule, seed, dens as f64 / 100.0), 256, log2)?;
    }

    #[test]
    fn rotor_matches(seed in 0u64..1000, log2 in 9u32..=12) {
        check_blank(|| Rotor { seed }, 256, log2)?;
    }

    #[test]
    fn random_walk_matches(seed in 0u64..1000, log2 in 9u32..=12) {
        check_blank(|| RandomWalk { state: seed }, 512, log2)?;
    }

    /// Torus with a fresh band (rows 0..16) between two visited i.i.d.-coloured regions; both fronts wrap.
    #[test]
    fn torus_band_matches_brute(bits in 0u32..(1 << 7), n in 2usize..=7, seed in 0u64..1000, log2 in 9u32..=12) {
        let rule = Rule::from_bits(bits, n);
        let nn = n as u64;
        let prefill = move |x: usize, y: usize| -> Option<u8> {
            if y >= 16 { Some((mix64(seed ^ ((x as u64) << 20 | y as u64)) % nn) as u8) } else { None }
        };
        let lay = Layout { w: 24, h: 40, wrap: true };
        let a = trace(AntWalker::new(&rule), lay, &prefill, (5, 16, 0), log2);
        let b = brute(AntWalker::new(&rule), lay, &prefill, (5, 16, 0), log2);
        prop_assert_eq!(a, b);
    }
}
