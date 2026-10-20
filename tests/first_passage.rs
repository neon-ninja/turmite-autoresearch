//! `first_plet totlet t_done: u64 = out.tallies.values().map(|p| p.time).sum();l_done: u64 = out.tallies.iter().filter(|(k, _)| k.2.is_some()).map(|(_, p)| p.n).sum();ssert!(o.tallies.keys().all(|k| k.2.is_none()));ssage`: the incremental depth field against a from-scratch BFS after every
//! step, the observer against `fixed_area` and against offline recomputation from the
//! recorded state stream, hand fixtures, and clone-as-checkpoint determinism.

use proptest::prelude::*;
use std::collections::VecDeque;
use turmite::fixed_area::{self, Stop};
use turmite::first_passage::*;
use turmite::walkers::{PatchedAnt, RandomWalk, Rotor, Walker};
use turmite::Rule;

/// BFS from every boundary cell over visited cells, clamped at CLAMP.
fn oracle(f: &DepthField) -> Vec<u8> {
    let n = f.size;
    let mut d = vec![u8::MAX; n * n];
    let mut q = VecDeque::new();
    for i in 0..n * n {
        if f.region.on_boundary(i) {
            d[i] = 0;
            q.push_back(i);
        }
    }
    while let Some(i) = q.pop_front() {
        for o in [-(n as isize), 1, n as isize, -1] {
            let j = (i as isize + o) as usize;
            if f.region.visited(j) && d[j] == u8::MAX {
                d[j] = d[i].saturating_add(1).min(CLAMP);
                q.push_back(j);
            }
        }
    }
    d
}

fn assert_depths(f: &DepthField) {
    let o = oracle(f);
    for i in 0..f.size * f.size {
        if f.region.visited(i) {
            assert_eq!(f.depth(i), o[i], "cell {i}");
        }
    }
    assert_eq!(f.decreases, 0, "an insertion lowered a depth");
}

fn check_every_step<W: Walker + Clone>(w: W, steps: u64) -> Output {
    let mut s = Sim::new(w, 128, u64::MAX / 4, &[(10, 20)]);
    for _ in 0..steps {
        s.advance(Some(1));
        assert_depths(&s.field);
    }
    s.finish()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]
    #[test]
    fn depth_matches_bfs_every_step_ants(bits in prop::collection::vec(any::<bool>(), 2..9), seed in any::<u64>()) {
        let rule: Rule = bits.iter().map(|&b| if b { 'R' } else { 'L' }).collect::<String>().parse().unwrap();
        let out = check_every_step(PatchedAnt::new(&rule, seed, 2), 1500);
        prop_assert_eq!(out.lipschitz_violations, 0);
        prop_assert_eq!(out.depth_decreases, 0);
    }

    #[test]
    fn depth_matches_bfs_every_step_walkers(seed in any::<u64>()) {
        check_every_step(RandomWalk { state: seed }, 1500);
        check_every_step(Rotor { seed }, 1500);
    }
}

/// Fill a block in serpentine order directly through the field: exercises depth > 64 (clamp).
#[test]
fn clamp_and_deep_depths() {
    let size = 512;
    let mut f = DepthField::new(size);
    let c = 256usize;
    let side = 170;
    let mut n = 0;
    for r in 0..side {
        for k in 0..side {
            let col = if r % 2 == 0 { k } else { side - 1 - k };
            let (x, y) = (col as i64, r as i64);
            f.visit((c + r) * size + c + col, x, y);
            n += 1;
            if n % 4001 == 0 {
                assert_depths(&f);
            }
        }
    }
    assert_depths(&f);
    assert!((0..size * size).any(|i| f.region.visited(i) && f.depth(i) == CLAMP), "clamp never reached");
    assert_eq!(f.decreases, 0);
}

/// A walker that follows a fixed heading script (hand fixtures).
#[derive(Clone)]
struct Script {
    h: Vec<u8>,
    i: usize,
}
impl Walker for Script {
    const RANDOM_INIT: bool = false;
    fn initial_cell(&self, _: i64, _: i64) -> u8 {
        0
    }
    fn step(&mut self, _: &mut u8, heading: &mut u8) {
        *heading = self.h[self.i % self.h.len()];
        self.i += 1;
    }
}

#[test]
fn one_cell_and_straight_line() {
    let mut s = Sim::new(Script { h: vec![1], i: 0 }, 64, 20, &[(5, 9)]);
    s.advance(Some(0));
    assert_eq!(s.field.region.area, 0); // nothing observed yet
    s.advance(None);
    let o = s.finish();
    // a line never has interior cells: no segments leave milestone 0
    assert!(o.tallies.keys().all(|k| k.2.is_none()));
    assert_eq!(o.bands[0].eps[1].overlap, 0);
    assert_eq!(o.stop, Some(Stop::Budget));
}

/// 3x3 square then the centre: the centre is born with all neighbours visited and has depth 1.
#[test]
fn square_fixture() {
    // N=0,E=1,S=2,W=3. start (0,0): E,E,S,S,W,W,N,E(->(1,1)) centre
    let h = vec![1, 1, 2, 2, 3, 3, 0, 1, 2];
    let mut s = Sim::new(Script { h, i: 0 }, 64, 9, &[(2, 100)]).with_trace();
    s.advance(None);
    let tr = s.trace.clone().unwrap();
    // states 0..=9; state 8 is the centre
    assert_eq!(tr[8].1, 9, "area");
    assert_eq!(tr[8].3, 1, "centre depth");
    assert!(!tr[8].0, "centre is interior");
    assert!(tr[7].0, "ring cell is boundary");
    let o = s.finish();
    assert_eq!(o.depth_decreases, 0);
    assert_eq!(o.lipschitz_violations, 0);
}

fn ant_sim(rule: &str, seed: u64, size: usize, cap: u64, bands: &[(u64, u64)]) -> Sim<PatchedAnt> {
    let r: Rule = rule.parse().unwrap();
    Sim::new(PatchedAnt::new(&r, seed, 2), size, cap, bands)
}

/// States agree with `fixed_area::drive`, and band aggregates with `fixed_area::run`.
#[test]
fn matches_fixed_area() {
    for (rule, seed) in [("RL", 1u64), ("LLRR", 7), ("RRLLLRLLLRRR", 3), ("LRRRRRLLR", 11)] {
        let r: Rule = rule.parse().unwrap();
        let bands = [(40u64, 80u64), (200, 300)];
        let mut s = ant_sim(rule, seed, 2048, 300_000, &bands).with_trace();
        s.advance(None);
        let tr = s.trace.clone().unwrap();
        let mut k = 0usize;
        fixed_area::drive(PatchedAnt::new(&r, seed, 2), 2048, tr.len() as u64 - 1, |st| {
            let (z, a, p, _, g) = tr[k];
            assert_eq!((st.z, st.area, st.perimeter, st.geom), (z, a, p, g), "{rule} t={k}");
            k += 1;
            false
        });
        assert_eq!(k, tr.len());
        let out = s.finish();
        let fa = fixed_area::run(PatchedAnt::new(&r, seed, 2), 2048, 300_000, &bands);
        for (b, f) in out.bands.iter().zip(&fa.bands) {
            if !f.reached_hi {
                continue;
            }
            assert_eq!((b.t_start, b.t_end, b.area_start, b.area_end, b.perim_end), (f.t_start, f.t_end, f.area_start, f.area_end, f.perim_end));
            assert_eq!((b.steps, b.b_dep, b.i_dep), (f.steps, f.b_dep, f.i_dep));
            assert_eq!(b.sum_pa.to_bits(), f.sum_pa.to_bits(), "sum_pa {rule}");
            assert_eq!(b.eps[0].overlap, f.b_dep);
            assert_eq!(b.eps[1].overlap, f.i_dep);
            assert_eq!(b.seg_overlap, b.steps, "segment time covers the band exactly");
        }
        // interior episodes never change the territory
        for w in tr.windows(2) {
            if !w[0].0 && !w[1].0 {
                assert_eq!(w[0].1, w[1].1, "area changed inside an interior episode");
            }
        }
    }
}

/// Offline recomputation of episode and milestone tallies from the recorded stream.
#[test]
fn matches_offline_recomputation() {
    for (rule, seed) in [("RL", 5u64), ("LLRR", 2), ("RRLLLRLLLRRR", 9), ("RLLRLRRL", 4)] {
        let bands = [(30u64, 60u64), (150, 260)];
        let mut s = ant_sim(rule, seed, 2048, 400_000, &bands).with_trace();
        s.advance(None);
        let tr = s.trace.clone().unwrap();
        let tt = tr.len() - 1;
        let out = s.finish();
        for (k, &(lo, hi)) in bands.iter().enumerate() {
            let b = &out.bands[k];
            let Some(ts) = (0..=tt).find(|&t| tr[t].1 >= lo) else { assert!(!b.reached_lo); continue };
            let th = (0..=tt).find(|&t| tr[t].1 >= hi);
            let te = th.unwrap_or(tt);
            // runs
            let mut exp = [[0u64; 4]; 2]; // [label][entered_complete, steps, overlap, left]
            let mut st = 0;
            for t in 1..=tt + 1 {
                if t == tt + 1 || tr[t].0 != tr[st].0 {
                    let (s0, e, z) = (st, t.min(tt), tr[st].0);
                    let resolved = t <= tt;
                    let lab = if z { 0 } else { 1 };
                    let ov = e.min(te).saturating_sub(s0.max(ts));
                    exp[lab][2] += ov as u64;
                    let entered = s0 > 0 && s0 >= ts && s0 < te.max(if th.is_none() { e } else { te });
                    if entered && resolved {
                        exp[lab][0] += 1;
                        exp[lab][1] += (e - s0) as u64;
                    } else if !entered && ov > 0 {
                        exp[lab][3] += 1;
                    }
                    st = t.min(tt);
                    if t > tt { break; }
                }
            }
            for lab in 0..2 {
                let e = &b.eps[lab];
                assert_eq!([e.entered_complete, e.entered_complete_steps, e.overlap, e.left_fragments], exp[lab], "{rule} band {k} label {lab}");
            }
            let _ = (lo, hi);
        }
        // milestone chain recomputed from the depth stream, tallied by (start band, from, to)
        let band_of = |st: usize| -> Option<usize> {
            bands.iter().position(|&(lo, hi)| {
                let Some(ts) = (0..=tt).find(|&t| tr[t].1 >= lo) else { return false };
                let te = (0..=tt).find(|&t| tr[t].1 >= hi).unwrap_or(tt + 1);
                ts <= st && st < te
            })
        };
        let mut exp: std::collections::BTreeMap<(usize, usize, Option<usize>), (u64, u64)> = Default::default();
        let (mut cur, mut st) = (0usize, 0usize);
        for t in 1..=tt {
            if let Some(j) = MILESTONES.iter().position(|&m| m == tr[t].3) {
                if j != cur {
                    if let Some(b) = band_of(st) {
                        let e = exp.entry((b, cur, Some(j))).or_default();
                        e.0 += 1;
                        e.1 += (t - st) as u64;
                    }
                    cur = j;
                    st = t;
                }
            }
        }
        if let Some(b) = band_of(st) {
            let e = exp.entry((b, cur, None)).or_default();
            e.0 += 1;
            e.1 += (tt - st) as u64;
        }
        let mut got: std::collections::BTreeMap<(usize, usize, Option<usize>), (u64, u64)> = Default::default();
        for (&(b, a, n), p) in &out.tallies {
            let e = got.entry((b, a.m as usize, n.map(|k| k.m as usize))).or_default();
            e.0 += p.n;
            e.1 += p.time;
        }
        assert_eq!(got, exp, "{rule}");
        assert_eq!(out.overflowed, 0);
    }
}


/// Clone is a checkpoint: stepping in chunks, or cloning mid-run and continuing, gives identical output.
#[test]
fn checkpoint_by_clone_is_exact() {
    let bands = [(60u64, 140u64), (300, 420)];
    let whole = {
        let mut s = ant_sim("RRLLLRLLLRRR", 17, 1024, 3_000_000, &bands);
        s.advance(None);
        s.finish()
    };
    let mut s = ant_sim("RRLLLRLLLRRR", 17, 1024, 3_000_000, &bands);
    let mut snapshots = Vec::new();
    while s.advance(Some(97)).is_none() {
        snapshots.push(s.clone());
    }
    assert_eq!(s.finish(), whole);
    assert!(snapshots.len() > 5);
    for snap in snapshots.iter().step_by(snapshots.len() / 4 + 1) {
        let mut c = snap.clone();
        c.advance(None);
        assert_eq!(c.finish(), whole);
    }
}

#[test]
fn geometry_jumps_occur_and_physics_is_lipschitz() {
    let mut jumps = 0;
    for seed in 0..20u64 {
        let o = {
            let mut s = Sim::new(RandomWalk { state: seed }, 256, 40_000, &[(20, 200)]);
            s.advance(None);
            s.finish()
        };
        assert_eq!(o.lipschitz_violations, 0);
        assert_eq!(o.depth_decreases, 0);
        jumps += o.jump_steps;
    }
    assert!(jumps > 0, "no geometry jump observed in 20 random walks");
}
