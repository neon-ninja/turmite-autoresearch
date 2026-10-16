//! `prefix_exposure`: the producer (incremental depth map, fresh flags, colour reads) against a
//! brute-force HashMap oracle with a full BFS at every step; the accountant against an offline
//! oracle and a hand-worked stream; the producer's boundary stream against `fixed_area`; and the
//! README controls (clone identity, base/mutant divergence, joint colour relabelling).

use proptest::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use turmite::fixed_area::{self, Stop};
use turmite::prefix_exposure::*;
use turmite::walkers::{AntWalker, PatchedAnt, RandomWalk, Rotor, Walker};
use turmite::{DIRS, Rule, Turn};

fn collect<W: Walker>(walker: W, size: usize, cap: u64, radius: i64) -> (Vec<Dep>, Stop) {
    let mut v = Vec::new();
    let (stop, t) = drive(walker, size, cap, radius, |d| {
        v.push(*d);
        false
    });
    assert_eq!(t as usize + 1, v.len());
    (v, stop)
}

// ---------- brute-force oracle for the producer ----------

/// States 0..=steps of a patched ant, every field recomputed from sets and maps. Depth is a full
/// multi-source BFS over the visited set from all boundary cells, capped at 5.
fn oracle(rule: &Rule, seed: u64, steps: u64) -> Vec<Dep> {
    let init = PatchedAnt::new(rule, seed, 2);
    let n = rule.len() as u8;
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut colour: HashMap<(i64, i64), u8> = HashMap::new();
    let mut wraps: HashMap<(i64, i64), u8> = HashMap::new();
    let (mut pos, mut h) = ((0i64, 0i64), 0u8);
    colour.insert(pos, init.initial_cell(0, 0));
    visited.insert(pos);
    let mut fresh = true;
    let mut out = Vec::new();
    let nbrs = |p: (i64, i64)| DIRS.map(|(dx, dy)| (p.0 + dx as i64, p.1 + dy as i64));
    for t in 0..=steps {
        let boundary: Vec<(i64, i64)> = visited.iter().copied().filter(|&p| nbrs(p).iter().any(|q| !visited.contains(q))).collect();
        let mut dist: HashMap<(i64, i64), u8> = boundary.iter().map(|&p| (p, 0)).collect();
        let mut q: VecDeque<_> = boundary.iter().copied().collect();
        while let Some(p) = q.pop_front() {
            let dp = dist[&p];
            for r in nbrs(p) {
                if visited.contains(&r) && !dist.contains_key(&r) {
                    dist.insert(r, dp + 1);
                    q.push_back(r);
                }
            }
        }
        out.push(Dep {
            t,
            z: dist[&pos] == 0,
            area: visited.len() as u64,
            perimeter: boundary.len() as u64,
            colour: colour[&pos],
            depth: dist[&pos].min(5),
            cycle: wraps.get(&pos).copied().unwrap_or(0).min(3),
            fresh,
            heading: h,
            x: pos.0,
            y: pos.1,
        });
        if t == steps {
            break;
        }
        let c = colour[&pos];
        h = (h + rule.0[c as usize].delta()) % 4;
        let nc = (c + 1) % n;
        colour.insert(pos, nc);
        if nc == 0 {
            *wraps.entry(pos).or_insert(0) = wraps.get(&pos).copied().unwrap_or(0).saturating_add(1);
        }
        pos = (pos.0 + DIRS[h as usize].0 as i64, pos.1 + DIRS[h as usize].1 as i64);
        fresh = !visited.contains(&pos);
        if fresh {
            visited.insert(pos);
            colour.insert(pos, init.initial_cell(pos.0, pos.1));
        }
    }
    out
}

fn first_mismatch(a: &[Dep], b: &[Dep]) -> Option<usize> {
    a.iter().zip(b).position(|(x, y)| x != y)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(160))]

    #[test]
    fn producer_matches_oracle(bits in any::<u32>(), n in 2usize..=14, seed in any::<u64>(), steps in 0u64..900) {
        let rule = Rule::from_bits(bits, n);
        let (deps, stop) = collect(PatchedAnt::new(&rule, seed, 2), 64, steps, UPDATE_RADIUS);
        prop_assert!(matches!(stop, Stop::Budget | Stop::Escaped));
        let want = oracle(&rule, seed, deps.len() as u64 - 1);
        prop_assert_eq!(first_mismatch(&deps, &want), None);
    }
}

/// The ball radius is proved by the test, not argued: radius 5 never disagrees with the oracle on a
/// fixed battery, radius 3 does.
#[test]
fn update_radius_is_sufficient_and_the_test_has_power() {
    let mut seed = 0x1234_5678u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        seed >> 20
    };
    let (mut bad5, mut bad4, mut bad3) = (0, 0, 0);
    for _ in 0..120 {
        let n = 3 + (next() % 10) as usize;
        let rule = Rule::from_bits(next() as u32, n);
        let s = next();
        for (radius, bad) in [(5, &mut bad5), (4, &mut bad4), (3, &mut bad3)] {
            let (deps, _) = collect(PatchedAnt::new(&rule, s, 2), 64, 1500, radius);
            if first_mismatch(&deps, &oracle(&rule, s, deps.len() as u64 - 1)).is_some() {
                *bad += 1;
            }
        }
    }
    eprintln!("depth-map radius sensitivity over 120 cases: radius5 bad={bad5} radius4 bad={bad4} radius3 bad={bad3}");
    assert_eq!(bad5, 0);
    assert!(bad4 > 0, "radius 5 is tight: 4 already fails");
    assert!(bad3 > 0, "a too-small update radius should be detectable");
}

// ---------- accountant vs offline oracle ----------

fn naive(deps: &[Dep], bands: &[(u64, u64)], w: u64, ws: &[u64]) -> (Vec<BandOut>, Whole) {
    let big = deps.len() - 1; // last state; departures 0..big are executed
    let mut out = Vec::new();
    for &(lo, hi) in bands {
        let mut b = BandOut {
            lo,
            hi,
            reached_lo: false,
            reached_hi: false,
            t_start: 0,
            t_end: 0,
            area_start: 0,
            area_end: 0,
            perim_start: 0,
            perim_end: 0,
            steps: 0,
            b_dep: 0,
            i_dep: 0,
            sum_pa: 0.0,
            reads: vec![0; MAX_COLOURS * DEPTH_CLASSES * CYCLE_CLASSES],
            fresh_colour: [0; MAX_COLOURS],
            launch: Launch::default(),
            ep_n: 0,
            ep_long_n: vec![0; ws.len()],
            tail_steps: vec![0; ws.len()],
        };
        if let Some(ts) = deps.iter().position(|d| d.area >= lo) {
            let th = deps.iter().position(|d| d.area >= hi);
            let te = th.unwrap_or(big);
            b.reached_lo = true;
            b.reached_hi = th.is_some();
            (b.t_start, b.t_end) = (ts as u64, te as u64);
            (b.area_start, b.area_end) = (deps[ts].area, deps[te].area);
            (b.perim_start, b.perim_end) = (deps[ts].perimeter, deps[te].perimeter);
            for f in ts..te {
                let d = &deps[f];
                b.steps += 1;
                b.sum_pa += d.perimeter as f64 / d.area as f64;
                if d.z { b.b_dep += 1 } else { b.i_dep += 1 }
                b.reads[reads_idx(d.colour as usize, d.depth as usize, d.cycle as usize)] += 1;
                if !d.fresh {
                    continue;
                }
                b.fresh_colour[d.colour as usize] += 1;
                let w = w as usize;
                let last = big.min(f + w);
                let long = (f + 1..=last).any(|s0| deps[s0 - 1].z && s0 + w - 1 <= big && (s0..s0 + w).all(|s| !deps[s].z));
                if big >= f + 2 * w - 1 {
                    let l = &mut b.launch;
                    l.n += 1;
                    l.n_interior_start += u64::from(!d.z);
                    l.left += u64::from((f + 1..=f + w).any(|s| !deps[s].z));
                    l.long += u64::from(long);
                    l.overlap += (f + 1..=f + w).filter(|&s| deps[s].fresh).count() as u64;
                    let (dx, dy) = (deps[f + w].x - d.x, deps[f + w].y - d.y);
                    let (ex, ey) = DIRS[d.heading as usize];
                    let (rx, ry) = DIRS[(d.heading as usize + 1) % 4];
                    l.sum_fwd += dx * ex as i64 + dy * ey as i64;
                    l.sum_lat += dx * rx as i64 + dy * ry as i64;
                    l.sum_r2 += (dx * dx + dy * dy) as u64;
                } else {
                    b.launch.cens += 1;
                    b.launch.cens_long += u64::from(long);
                }
            }
            let mut s = 0;
            while s <= big {
                if deps[s].z {
                    s += 1;
                    continue;
                }
                let mut e = s;
                while e <= big && !deps[e].z {
                    e += 1;
                }
                let len = (e - s) as u64;
                let indep = (s.max(ts)..e.min(te)).count() as u64;
                let begins = s >= ts && s < te;
                b.ep_n += u64::from(begins);
                for (i, &wv) in ws.iter().enumerate() {
                    if len >= wv {
                        b.ep_long_n[i] += u64::from(begins);
                        b.tail_steps[i] += indep;
                    }
                }
                s = e;
            }
        }
        out.push(b);
    }
    let mut whole = Whole { first_read: [None; MAX_COLOURS], reads: [0; MAX_COLOURS], open_episode: None };
    if !deps[big].z {
        let s = (0..=big).rev().take_while(|&t| !deps[t].z).last().unwrap();
        let begin = out.iter().position(|b: &BandOut| b.reached_lo && (b.t_start as usize) <= s && s < b.t_end as usize);
        whole.open_episode = Some((begin, (big + 1 - s) as u64));
    }
    for d in &deps[..big] {
        whole.reads[d.colour as usize] += 1;
        whole.first_read[d.colour as usize].get_or_insert((d.t, d.area, d.depth));
    }
    (out, whole)
}

fn account(deps: &[Dep], bands: &[(u64, u64)], w: u64, ws: &[u64]) -> (Vec<BandOut>, Whole) {
    let mut acc = Accountant::new(bands, w, ws);
    for d in deps {
        acc.feed(d);
    }
    acc.finish()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(160))]

    #[test]
    fn accountant_matches_offline(bits in any::<u32>(), n in 2usize..=14, seed in any::<u64>(), steps in 0u64..1200,
                                  lo1 in 2u64..40, len1 in 1u64..60, gap in 0u64..30, lo_len2 in 1u64..120, w in 1u64..=9) {
        let rule = Rule::from_bits(bits, n);
        let (deps, _) = collect(PatchedAnt::new(&rule, seed, 2), 64, steps, UPDATE_RADIUS);
        let bands = [(lo1, lo1 + len1), (lo1 + len1 + gap, lo1 + len1 + gap + lo_len2)];
        let ws = [1, 2, 3, 5, 40];
        let (got, whole) = account(&deps, &bands, w, &ws);
        let (want, want_whole) = naive(&deps, &bands, w, &ws);
        prop_assert_eq!(got, want);
        prop_assert_eq!(whole, want_whole);
    }
}

/// Worked by hand (see NOTEBOOK.md): w = 2, one band [2, 4).
#[test]
fn accountant_hand_worked() {
    let d = |t, z, area, fresh, colour, heading, x, y| Dep { t, z, area, perimeter: 3, colour, depth: if z { 0 } else { 2 }, cycle: 0, fresh, heading, x, y };
    let deps = [
        d(0, true, 1, true, 0, 0, 0, 0),
        d(1, true, 2, true, 0, 1, 1, 0),
        d(2, false, 2, false, 1, 1, 2, 0),
        d(3, false, 3, true, 0, 1, 3, 0),
        d(4, false, 3, false, 1, 1, 4, 0),
        d(5, true, 4, true, 0, 1, 4, 1),
        d(6, true, 4, false, 1, 1, 4, 2),
    ];
    let (bands, _) = account(&deps, &[(2, 4)], 2, &[1, 2, 3, 4]);
    let b = &bands[0];
    assert!(b.reached_lo && b.reached_hi);
    assert_eq!((b.t_start, b.t_end, b.area_start, b.area_end), (1, 5, 2, 4));
    assert_eq!((b.steps, b.b_dep, b.i_dep), (4, 1, 3));
    assert_eq!((b.perim_start, b.perim_end, b.sum_pa), (3, 3, 5.0));
    assert_eq!(b.fresh_colour[0], 2);
    assert_eq!(b.reads_of(0, 0), 1);
    assert_eq!(b.reads_of(0, 2), 1);
    assert_eq!(b.reads_of(1, 2), 2);
    let l = &b.launch;
    assert_eq!((l.n, l.n_interior_start, l.left, l.long, l.overlap), (2, 1, 2, 1, 2));
    assert_eq!((l.sum_fwd, l.sum_lat, l.sum_r2), (3, 1, 6));
    assert_eq!((l.cens, l.cens_long), (0, 0));
    // one interior episode, states 2..=4 (3 states), first departure in the band, all 3 departures in the band
    assert_eq!((b.ep_n, b.ep_long_n.clone(), b.tail_steps.clone()), (1, vec![1, 1, 1, 0], vec![3, 3, 3, 0]));
}

#[test]
fn unresolved_windows_are_censored_not_counted() {
    // Fresh at f = 1, run stops at state 3 < f + 2w - 1 = 4 (w = 2).
    let d = |t, z, area, fresh| Dep { t, z, area, perimeter: 1, colour: 0, depth: 0, cycle: 0, fresh, heading: 1, x: t as i64, y: 0 };
    let deps = [d(0, true, 1, true), d(1, true, 2, true), d(2, false, 2, false), d(3, false, 2, false)];
    let (bands, whole) = account(&deps, &[(2, 9)], 2, &[1, 2, 3]);
    let l = &bands[0].launch;
    assert_eq!(whole.open_episode, Some((Some(0), 2)));
    assert_eq!((bands[0].ep_n, bands[0].ep_long_n.clone(), bands[0].tail_steps.clone()), (1, vec![1, 1, 0], vec![1, 1, 0]));
    assert_eq!((l.n, l.cens), (0, 1));
    assert_eq!(l.cens_long, 1, "an episode that started at state 2 already has 2 interior states");
}

// ---------- producer vs fixed_area ----------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(80))]

    #[test]
    fn boundary_stream_matches_fixed_area(bits in any::<u32>(), n in 2usize..=14, seed in any::<u64>(), lo in 2u64..60, len in 1u64..120) {
        let rule = Rule::from_bits(bits, n);
        let bands = [(lo, lo + len)];
        let a = fixed_area::run(PatchedAnt::new(&rule, seed, 2), 512, 60_000, &bands);
        let b = run(PatchedAnt::new(&rule, seed, 2), 512, 60_000, &bands, 8, &[1, 4]);
        prop_assume!(a.stop != Stop::Escaped && b.stop != Stop::Escaped);
        let (x, y) = (&a.bands[0], &b.bands[0]);
        prop_assert_eq!((x.reached_lo, x.reached_hi), (y.reached_lo, y.reached_hi));
        if x.reached_lo {
            prop_assert_eq!((x.t_start, x.t_end, x.area_start, x.area_end), (y.t_start, y.t_end, y.area_start, y.area_end));
            prop_assert_eq!((x.steps, x.b_dep, x.i_dep), (y.steps, y.b_dep, y.i_dep));
        }
    }
}

// ---------- invariants and controls ----------

#[test]
fn band_invariants_and_clone_identity() {
    let rule: Rule = "RLLRLLRRLRLRL".parse().unwrap();
    let go = || run(PatchedAnt::new(&rule, 99, 2), 512, 4_000_000, &[(2000, 2200), (3000, 3500)], 64, &[1, 16, 64, 256]);
    let (a, b) = (go(), go());
    assert_eq!(format!("{a:?}"), format!("{b:?}"), "unchanged-rule clones must agree exactly");
    for band in &a.bands {
        assert!(band.reached_hi, "{band:?}");
        assert_eq!(band.steps, band.b_dep + band.i_dep);
        assert_eq!(band.steps, band.reads.iter().sum::<u64>());
        let depth0: u64 = (0..MAX_COLOURS).map(|c| band.reads_of(c, 0)).sum();
        assert_eq!(depth0, band.b_dep, "depth 0 is exactly the boundary label");
        assert_eq!(band.fresh_colour.iter().sum::<u64>(), band.hi - band.lo, "one fresh departure per new cell");
        assert_eq!(band.launch.n + band.launch.cens, band.hi - band.lo);
        assert_eq!(band.tail_steps[0], band.i_dep, "threshold 1 captures every interior departure");
        assert_eq!(band.ep_long_n[0], band.ep_n);
        assert!(band.tail_steps.windows(2).all(|p| p[0] >= p[1]) && band.ep_long_n.windows(2).all(|p| p[0] >= p[1]));
        assert!(band.launch.long <= band.launch.left + band.launch.n_interior_start);
    }
    assert_eq!(a.whole.reads.iter().sum::<u64>(), a.t_final, "the final state has no executed departure");
}

fn mutate(rule: &Rule, j: usize) -> Rule {
    let mut v = rule.0.clone();
    v[j] = if v[j] == Turn::L { Turn::R } else { Turn::L };
    Rule(v)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    /// Base and mutant agree through the state at which the base first reads the altered colour, and
    /// differ at the next one; first_read gives that state.
    #[test]
    fn mutant_diverges_exactly_at_first_read(bits in any::<u32>(), n in 3usize..=14, jj in any::<usize>(), seed in any::<u64>()) {
        let base = Rule::from_bits(bits, n);
        let j = jj % n;
        let mutant = mutate(&base, j);
        let (b, _) = collect(PatchedAnt::new(&base, seed, 2), 128, 1500, UPDATE_RADIUS);
        let (m, _) = collect(PatchedAnt::new(&mutant, seed, 2), 128, 1500, UPDATE_RADIUS);
        let (_, whole) = account(&b, &[(2, 3)], 4, &[1]);
        match whole.first_read[j] {
            None => prop_assert_eq!(first_mismatch(&b, &m).filter(|&i| i + 1 < b.len().min(m.len())), None),
            Some((t, _, _)) => {
                let t = t as usize;
                prop_assert_eq!(first_mismatch(&b[..=t], &m[..=t]), None);
                prop_assert!(b.len() > t + 1 && m.len() > t + 1);
                prop_assert_ne!((b[t + 1].x, b[t + 1].y), (m[t + 1].x, m[t + 1].y));
            }
        }
    }
}

/// Joint relabelling c -> c - s of the rule and of every field colour (blank cells included)
/// reproduces the trajectory; the reads are the same events under permuted colour names.
struct Relabelled {
    ant: AntWalker,
    orig: PatchedAnt,
    n: u8,
    s: u8,
}

impl Walker for Relabelled {
    const RANDOM_INIT: bool = true;
    fn initial_cell(&self, x: i64, y: i64) -> u8 {
        (self.orig.initial_cell(x, y) + self.n - self.s) % self.n
    }
    fn step(&mut self, cell: &mut u8, heading: &mut u8) {
        self.ant.step(cell, heading);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn joint_relabelling_reproduces_trajectories(bits in any::<u32>(), n in 3usize..=14, s in 1usize..14, seed in any::<u64>()) {
        let s = s % n;
        let rule = Rule::from_bits(bits, n);
        let rotated = Rule((0..n).map(|k| rule.0[(k + s) % n]).collect());
        let w = Relabelled { ant: AntWalker::new(&rotated), orig: PatchedAnt::new(&rule, seed, 2), n: n as u8, s: s as u8 };
        let (a, _) = collect(PatchedAnt::new(&rule, seed, 2), 96, 1500, UPDATE_RADIUS);
        let (b, _) = collect(w, 96, 1500, UPDATE_RADIUS);
        prop_assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            prop_assert_eq!((x.z, x.area, x.depth, x.fresh, x.heading, x.x, x.y), (y.z, y.area, y.depth, y.fresh, y.heading, y.x, y.y));
            prop_assert_eq!((x.colour as usize + n - s) % n, y.colour as usize);
        }
    }
}

// ---------- known answer without any BFS ----------

struct Scripted {
    moves: Vec<u8>,
    i: usize,
}

impl Walker for Scripted {
    const RANDOM_INIT: bool = false;
    fn initial_cell(&self, _: i64, _: i64) -> u8 {
        0
    }
    fn step(&mut self, _: &mut u8, heading: &mut u8) {
        *heading = self.moves[self.i];
        self.i += 1;
    }
}

/// Snake-fill an 11x11 block, then walk inside it. After the last new cell the depth of every
/// state is the analytic distance to the block edge, capped at 5.
#[test]
fn depth_known_answer_on_a_filled_block() {
    const K: i64 = 11;
    let (e, s, w, n) = (1u8, 2u8, 3u8, 0u8);
    let mut moves = Vec::new();
    for y in 0..K {
        moves.extend(std::iter::repeat_n(if y % 2 == 0 { e } else { w }, (K - 1) as usize));
        if y < K - 1 {
            moves.push(s);
        }
    }
    let fill = moves.len(); // arrives at the last new cell after this many moves
    moves.extend(std::iter::repeat_n(w, 5));
    moves.extend(std::iter::repeat_n(n, 5));
    moves.extend(std::iter::repeat_n(s, 5));
    moves.extend(std::iter::repeat_n(e, 5));
    let steps = moves.len() as u64;
    let (deps, stop) = collect(Scripted { moves, i: 0 }, 64, steps, UPDATE_RADIUS);
    assert_eq!(stop, Stop::Budget);
    assert_eq!(fill as i64, K * K - 1);
    assert_eq!(deps.iter().filter(|d| d.fresh).count() as i64, K * K, "origin plus 120 arrivals");
    assert!(deps.iter().take(fill + 1).skip(1).all(|d| d.fresh));
    assert!(deps.iter().skip(fill + 1).all(|d| !d.fresh));
    for d in &deps[fill..] {
        let edge = d.x.min(d.y).min(K - 1 - d.x).min(K - 1 - d.y);
        assert!((0..K).contains(&d.x) && (0..K).contains(&d.y));
        assert_eq!(d.depth as i64, edge.min(5), "state {} at ({}, {})", d.t, d.x, d.y);
        assert_eq!(d.z, edge == 0);
    }
}

// ---------- calibration walkers run and obey the invariants ----------

#[test]
fn control_walkers_obey_the_invariants() {
    for (name, out) in [
        ("rotor", run(Rotor { seed: 5 }, 512, 2_000_000, &[(200, 400)], 16, &[1, 8])),
        ("randomwalk", run(RandomWalk { state: 5 }, 512, 2_000_000, &[(200, 400)], 16, &[1, 8])),
    ] {
        let b = &out.bands[0];
        assert!(b.reached_hi, "{name}: {out:?}");
        assert_eq!(b.steps, b.b_dep + b.i_dep);
        assert_eq!(b.steps, b.reads.iter().sum::<u64>());
        assert_eq!(b.launch.n + b.launch.cens, b.hi - b.lo);
    }
}
