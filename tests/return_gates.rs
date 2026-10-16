//! return_gates: the incremental rim/interior instrument against a from-scratch oracle that
//! recomputes the whole depth map (BFS from every boundary cell) at every step, plus
//! statistic calibration on synthetic exposure/event tables.
use proptest::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use turmite::fixed_area::{Geom, State};
use turmite::return_gates::{self, Key, Tally, TILE};
use turmite::walkers::{PatchedAnt, Walker};
use turmite::{reference, field_age, Rule, DIRS};

type P = (i32, i32);

fn full_depth(seen: &HashSet<P>) -> HashMap<P, u32> {
    let mut d = HashMap::new();
    let mut q = VecDeque::new();
    for &p in seen {
        if DIRS.iter().any(|&(dx, dy)| !seen.contains(&(p.0 + dx, p.1 + dy))) {
            d.insert(p, 0);
            q.push_back(p);
        }
    }
    while let Some(p) = q.pop_front() {
        for &(dx, dy) in &DIRS {
            let n = (p.0 + dx, p.1 + dy);
            if seen.contains(&n) && !d.contains_key(&n) {
                d.insert(n, d[&p] + 1);
                q.push_back(n);
            }
        }
    }
    d
}

struct St {
    pos: P,
    heading: u8,
    visited: HashSet<P>,
    depth: HashMap<P, u32>,
}

fn tile(p: P) -> (i32, i32) {
    ((p.0 as i64).div_euclid(TILE) as i32, (p.1 as i64).div_euclid(TILE) as i32)
}

fn oracle(states: &[St], bands: &[(u64, u64)]) -> Vec<Tally> {
    let mut out = vec![Tally::default(); bands.len()];
    let rim = |d: &HashMap<P, u32>, v: &HashSet<P>, p: P| !v.contains(&p) || d[&p] <= 4;
    let mut open: Option<(usize, u8, usize)> = None;
    let mut brun: Option<u64> = None;
    let mut prev_interior = false;
    for t in 1..states.len() {
        let (a, b) = (&states[t - 1], &states[t]);
        let area = a.visited.len() as u64;
        let k = bands.iter().position(|&(lo, hi)| area >= lo && area < hi);
        let l_prev = rim(&a.depth, &a.visited, a.pos);
        let l_a = rim(&a.depth, &a.visited, b.pos);
        let l_new = b.depth[&b.pos] <= 4;
        let z_prev = a.depth[&a.pos] == 0;
        if let Some(k) = k {
            let o = &mut out[k];
            o.steps += 1;
            o.depth_steps[a.depth[&a.pos].min(5) as usize] += 1;
            for (h, &(dx, dy)) in DIRS.iter().enumerate() {
                let n = (a.pos.0 + dx, a.pos.1 + dy);
                if rim(&a.depth, &a.visited, n) != l_prev {
                    let (kind, cell) = if l_prev { (0u8, a.pos) } else { (1u8, n) };
                    let (tx, ty) = tile(cell);
                    *o.exposure.entry((tx, ty, h as u8, kind)).or_insert(0) += 1;
                }
            }
            if z_prev {
                brun = match brun { Some(n) => Some(n + 1), None if prev_interior => Some(1), None => None };
            } else if let Some(n) = brun.take() {
                o.b_runs += 1;
                o.b_run_steps += n;
            }
            prev_interior = !z_prev;
            if l_a != l_prev {
                let (kind, cell) = if l_prev { (0u8, a.pos) } else { (1u8, b.pos) };
                let (tx, ty) = tile(cell);
                *o.events.entry((tx, ty, b.heading, kind)).or_insert(0) += 1;
                if kind == 0 { o.exits += 1 } else { o.entries += 1 }
            }
            if l_new != l_a {
                if l_new { o.reclass_in += 1 } else { o.reclass_out += 1 }
            }
            if l_prev && !l_a && !l_new { open = Some((t, b.heading, k)); }
        } else {
            brun = None;
            prev_interior = false;
        }
        if l_new {
            if let Some((t0, h, kb)) = open.take() {
                out[kb].exc_n[h as usize] += 1;
                out[kb].exc_steps[h as usize] += (t - t0) as u64;
            }
        }
    }
    if let Some((_, _, k)) = open { out[k].exc_unresolved += 1; }
    out
}

fn trajectory(rule: &Rule, seed: u64, steps: u64) -> Vec<St> {
    let init = PatchedAnt::new(rule, seed, 2);
    let mut ant = reference::Ant::new(rule.clone());
    for y in -2..=2 { for x in -2..=2 {
        let c = init.initial_cell(x, y);
        if c != 0 { ant.cells.insert((x as i32, y as i32), c); }
    }}
    let mut visited = HashSet::from([(0, 0)]);
    let mut v = Vec::new();
    for _ in 0..=steps {
        let depth = full_depth(&visited);
        v.push(St { pos: ant.pos, heading: ant.heading, visited: visited.clone(), depth });
        ant.step();
        visited.insert(ant.pos);
    }
    v
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]
    #[test]
    fn incremental_depth_and_crossings_match_from_scratch_oracle(
        bits in prop::collection::vec(any::<bool>(), 2..12), seed in any::<u64>()) {
        let rule: Rule = bits.iter().map(|b| if *b { 'R' } else { 'L' }).collect::<String>().parse().unwrap();
        let bands = [(6u64, 30u64), (30, 90), (90, 400)];
        let mut obs = return_gates::Obs::new(&bands, 128);
        let mut last = 0u64;
        field_age::drive(PatchedAnt::new(&rule, seed, 2), 128, 700, |f| {
            obs.feed(&f);
            last = f.state.t;
            false
        });
        let got = obs.finish();
        let want = oracle(&trajectory(&rule, seed, last), &bands);
        prop_assert_eq!(got, want);
    }
}

#[test]
fn hole_split_matches_set_flood_fill() {
    // run long enough that closed holes exist for most rules; compare to a set-based fill
    for (rule, seed) in [("RL", 1u64), ("RLLR", 2), ("LLRR", 3), ("RRLLLRLLLRRR", 4)] {
        let rule: Rule = rule.parse().unwrap();
        let mut got = (0, 0);
        let mut seen = HashSet::new();
        let steps = 6000u64;
        let mut ant = reference::Ant::new(rule.clone());
        let init = PatchedAnt::new(&rule, seed, 2);
        for y in -2..=2 { for x in -2..=2 { let c = init.initial_cell(x, y); if c != 0 { ant.cells.insert((x as i32, y as i32), c); } } }
        seen.insert(ant.pos);
        let mut stop_t = 0;
        field_age::drive(PatchedAnt::new(&rule, seed, 2), 256, steps, |f| {
            if f.state.t == steps { got = return_gates::hole_split(&f); stop_t = f.state.t; }
            false
        });
        for _ in 0..stop_t { ant.step(); seen.insert(ant.pos); }
        let (x0, x1) = (seen.iter().map(|p| p.0).min().unwrap() - 1, seen.iter().map(|p| p.0).max().unwrap() + 1);
        let (y0, y1) = (seen.iter().map(|p| p.1).min().unwrap() - 1, seen.iter().map(|p| p.1).max().unwrap() + 1);
        let mut out = HashSet::from([(x0, y0)]);
        let mut q = VecDeque::from([(x0, y0)]);
        while let Some(p) = q.pop_front() {
            for &(dx, dy) in &DIRS {
                let n = (p.0 + dx, p.1 + dy);
                if n.0 < x0 || n.0 > x1 || n.1 < y0 || n.1 > y1 { continue; }
                if !seen.contains(&n) && out.insert(n) { q.push_back(n); }
            }
        }
        let (mut ext, mut hole) = (0u64, 0u64);
        for &p in &seen {
            let unv: Vec<P> = DIRS.iter().map(|&(dx, dy)| (p.0 + dx, p.1 + dy)).filter(|n| !seen.contains(n)).collect();
            if unv.is_empty() { continue; }
            if unv.iter().any(|n| out.contains(n)) { ext += 1 } else { hole += 1 }
        }
        eprintln!("hole_split {rule:?}: ext {ext} hole {hole}");
        assert_eq!(got, (ext, hole), "{rule:?}");
    }
}

/// Splitmix stream for synthetic tables (independent of the crate's null sampler).
fn next(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
    turmite::walkers::mix64(*s)
}

fn synthetic(seed: u64, bins: usize, events: u64, planted: Option<(usize, f64)>) -> Tally {
    let mut s = seed;
    let mut t = Tally::default();
    let mut weight = Vec::new();
    for b in 0..bins {
        let e = 100 + next(&mut s) % 900;
        let key: Key = ((b / 64) as i32, (b % 64) as i32, 0, 0);
        t.exposure.insert(key, e);
        let boost = planted.map_or(1.0, |(m, f)| if b % m == 0 { f } else { 1.0 });
        weight.push(e as f64 * boost);
    }
    let total: f64 = weight.iter().sum();
    let cum: Vec<f64> = weight.iter().scan(0.0, |a, w| { *a += w / total; Some(*a) }).collect();
    for _ in 0..events {
        let u = (next(&mut s) >> 11) as f64 / (1u64 << 53) as f64;
        let b = cum.partition_point(|&c| c < u).min(bins - 1);
        *t.events.entry(((b / 64) as i32, (b % 64) as i32, 0, 0)).or_insert(0) += 1;
    }
    t
}

#[test]
fn concentration_rejects_uniform_flow_and_detects_planted_gates() {
    let (mut null_g, mut planted_g) = (Vec::new(), Vec::new());
    for r in 0..30u64 {
        let u = synthetic(1000 + r, 4000, 60_000, None);
        let (o, m, _, _, _) = return_gates::concentration(&u, 77 + r, 60);
        null_g.push(o - m);
        let p = synthetic(2000 + r, 4000, 60_000, Some((100, 8.0)));
        let (o, m, _, _, _) = return_gates::concentration(&p, 77 + r, 60);
        planted_g.push(o - m);
    }
    let mean = null_g.iter().sum::<f64>() / 30.0;
    let sd = (null_g.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / 29.0).sqrt();
    assert!(mean.abs() < 4.0 * sd / 30f64.sqrt() + 1e-3, "uniform flow biased: {mean} sd {sd}");
    let pm = planted_g.iter().sum::<f64>() / 30.0;
    assert!(pm > 0.03 && pm > 20.0 * sd, "planted gates missed: {pm} vs null sd {sd}");
}

#[test]
fn return_spread_needs_two_headings() {
    assert!(return_gates::return_spread(&[500, 0, 0, 0], &[5000, 0, 0, 0], 200).is_nan());
    let v = return_gates::return_spread(&[300, 300, 10, 0], &[3000, 30000, 999, 0], 200);
    assert!((v - 10f64.ln()).abs() < 1e-12);
}

#[test]
fn state_struct_is_still_constructible() {
    // guards the field_age/fixed_area API this instrument sits on
    let _ = State { t: 0, z: true, area: 1, perimeter: 4, geom: Geom::default() };
    let _: BTreeMap<Key, u64> = BTreeMap::new();
}
