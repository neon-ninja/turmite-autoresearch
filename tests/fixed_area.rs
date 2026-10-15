//! `fixed_area`: producer vs a brute-force visited-set recount, and the accounting state machine
//! vs an offline oracle built from full per-step arrays (run-length encode, scan for tau, intersect).
//! The accountant is also checked against hand-worked cases and the free invariants.

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::fixed_area::*;
use turmite::reference;
use turmite::walkers::{AntWalker, PatchedAnt, RandomWalk, Rotor, Walker};
use turmite::{DIRS, Rule};

// ---------- offline oracle for the accountant (b') ----------

fn offline(z: &[bool], a: &[u64], p: &[u64], g: &[Geom], bands: &[(u64, u64)]) -> Vec<BandOut> {
    let tt = z.len() - 1; // states 0..=tt, departures 0..tt
    // maximal runs of equal label over executed departures
    let mut runs: Vec<(u64, u64, bool)> = vec![];
    let mut s = 0;
    for t in 1..=tt {
        if z[t] != z[s] {
            runs.push((s as u64, t as u64, z[s]));
            s = t;
        }
    }
    if tt > 0 && s < tt {
        runs.push((s as u64, tt as u64, z[s]));
    }
    let start_obs = |s: u64| s > 0;
    let end_obs = |e: u64, zz: bool| z[e as usize] != zz;
    bands
        .iter()
        .map(|&(lo, hi)| {
            let mut b = BandOut { lo, hi, ..Default::default() };
            let Some(ts) = (0..=tt).find(|&t| a[t] >= lo) else { return b };
            let th = (0..=tt).find(|&t| a[t] >= hi);
            let te = th.unwrap_or(tt);
            assert_eq!(a[ts], lo);
            if let Some(th) = th {
                assert_eq!(a[th], hi);
            }
            b.reached_lo = true;
            b.reached_hi = th.is_some();
            b.t_start = ts as u64;
            b.t_end = te as u64;
            b.area_start = a[ts];
            b.area_end = a[te];
            b.perim_start = p[ts];
            b.perim_end = p[te];
            b.geom_start = g[ts];
            b.geom_end = g[te];
            for t in ts..te {
                b.steps += 1;
                if z[t] {
                    b.b_dep += 1;
                } else {
                    b.i_dep += 1;
                    if a[t + 1] > a[t] {
                        b.fresh_from_interior += 1;
                    }
                }
                b.sum_pa += p[t] as f64 / a[t] as f64;
            }
            let (ts, te) = (ts as u64, te as u64);
            for (j, &(rs, re, rz)) in runs.iter().enumerate() {
                let (lo_i, hi_i) = (rs.max(ts), re.min(te));
                if lo_i >= hi_i {
                    continue;
                }
                let complete = start_obs(rs) && end_obs(re, rz) && rs >= ts && re <= te;
                let st = if rz { &mut b.b } else { &mut b.i };
                if complete {
                    let d = re - rs;
                    st.complete += 1;
                    st.complete_steps += d;
                    st.bin_n[d.ilog2() as usize] += 1;
                    st.bin_steps[d.ilog2() as usize] += d;
                    if !rz {
                        b.i_durations.push(d as u32);
                    }
                } else {
                    st.clipped_frags += 1;
                    st.clipped_steps += hi_i - lo_i;
                    st.clipped_max = st.clipped_max.max(hi_i - lo_i);
                }
                // cycle: this run is I, previous run is B, both complete, whole cycle in band
                if complete && !rz && j > 0 {
                    let (ps, pe, pz) = runs[j - 1];
                    assert_eq!(pe, rs);
                    if pz && start_obs(ps) && ps >= ts && re <= te {
                        b.cycles += 1;
                        b.cycle_d_sum += pe - ps;
                        b.cycle_t_sum += re - rs;
                    }
                }
            }
            b
        })
        .collect()
}

fn feed_all(z: &[bool], a: &[u64], p: &[u64], g: &[Geom], bands: &[(u64, u64)]) -> Vec<BandOut> {
    let mut acc = Accountant::new(bands);
    for t in 0..z.len() {
        acc.feed(&State { t: t as u64, z: z[t], area: a[t], perimeter: p[t], geom: g[t] });
    }
    acc.finish()
}

fn geoms(n: usize) -> Vec<Geom> {
    (0..n).map(|t| Geom { width: t as u32 + 1, height: 2 * t as u32 + 1, sx: t as i64, sy: -(t as i64), sxx: (t * t) as i64 }).collect()
}

/// Synthetic stream obeying the geometric constraints: area grows by 0/1, only from a B departure.
fn synth() -> impl Strategy<Value = (Vec<bool>, Vec<u64>, Vec<u64>, Vec<(u64, u64)>)> {
    (0usize..260, 0.05f64..0.9, 0.0f64..1.0, any::<u64>(), 1usize..4, any::<bool>()).prop_map(|(tt, flip, grow, seed, nb, chain)| {
        let mut st = seed | 1;
        let mut rnd = move || {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            (st >> 11) as f64 / (1u64 << 53) as f64
        };
        let mut z = vec![true];
        for _ in 0..tt {
            let last = *z.last().unwrap();
            z.push(if rnd() < flip { !last } else { last });
        }
        let mut a = vec![1u64];
        for t in 0..tt {
            let inc = z[t] && rnd() < grow;
            a.push(a[t] + inc as u64);
        }
        let p: Vec<u64> = a.iter().map(|&x| 1 + (rnd() * x as f64) as u64).collect();
        let amax = *a.last().unwrap();
        // 2*nb boundaries (or nb+1 if chained) spread through 2..=amax+2 so some bands go unreached.
        let n = if chain { nb + 1 } else { 2 * nb };
        let mut vals: Vec<u64> = (0..n).map(|_| 2 + (rnd() * (amax + 1) as f64) as u64).collect();
        vals.sort();
        vals.dedup();
        let mut bands = vec![];
        if chain {
            for w in vals.windows(2) {
                bands.push((w[0], w[1]));
            }
        } else {
            for w in vals.chunks(2) {
                if w.len() == 2 {
                    bands.push((w[0], w[1]));
                }
            }
        }
        (z, a, p, bands)
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(600))]
    #[test]
    fn accountant_matches_offline((z, a, p, bands) in synth()) {
        let g = geoms(z.len());
        let on = feed_all(&z, &a, &p, &g, &bands);
        let off = offline(&z, &a, &p, &g, &bands);
        prop_assert_eq!(&on, &off);
        for b in &on {
            // exact integer conservation, in both paths
            prop_assert_eq!(b.steps, b.b_dep + b.i_dep);
            prop_assert_eq!(b.b.steps(), b.b_dep);
            prop_assert_eq!(b.i.steps(), b.i_dep);
            prop_assert_eq!(b.fresh_from_interior, 0);
            if b.reached_lo {
                prop_assert_eq!(b.area_start, b.lo);
                prop_assert_eq!(b.steps, b.t_end - b.t_start);
                prop_assert!(b.new_cells() <= b.b_dep, "every new cell is entered from a boundary departure");
                prop_assert_eq!(b.i_durations.len() as u64, b.i.complete);
            }
            if b.reached_hi {
                prop_assert_eq!(b.area_end, b.hi);
            }
        }
    }
}

// ---------- hand-worked cases ----------

#[test]
fn hand_worked_band() {
    //         t: 0     1     2     3     4     5     6     7     8     9
    let z = [true, true, false, false, true, false, false, true, true, false];
    let a = [1, 2, 2, 2, 2, 3, 3, 3, 4, 4];
    let p = [1, 2, 2, 2, 2, 3, 3, 3, 4, 4];
    let g = geoms(10);
    let out = feed_all(&z, &a, &p, &g, &[(2, 4)]);
    let b = &out[0];
    assert_eq!((b.t_start, b.t_end, b.steps), (1, 8, 7));
    assert!(b.reached_lo && b.reached_hi);
    assert_eq!((b.b_dep, b.i_dep), (3, 4));
    // B: [0,2) clipped (run start, in band only t=1), [4,5) complete, [7,9) clipped (in band t=7)
    assert_eq!((b.b.complete, b.b.complete_steps, b.b.clipped_frags, b.b.clipped_steps), (1, 1, 2, 2));
    assert_eq!((b.i.complete, b.i.complete_steps, b.i.clipped_frags), (2, 4, 0));
    assert_eq!(b.i_durations, vec![2, 2]);
    assert_eq!((b.b.bin_n[0], b.i.bin_n[1]), (1, 2));
    assert_eq!((b.cycles, b.cycle_d_sum, b.cycle_t_sum), (1, 1, 2));
    assert_eq!(b, &offline(&z, &a, &p, &g, &[(2, 4)])[0]);
}

#[test]
fn single_episode_spanning_both_edges_and_no_cycles() {
    // always boundary, area rises every step: one B run covers the whole band
    let z = [true; 12];
    let a: Vec<u64> = (1..=12).collect();
    let g = geoms(12);
    let out = feed_all(&z, &a, &a, &g, &[(3, 8)]);
    let b = &out[0];
    assert_eq!((b.t_start, b.t_end, b.steps, b.b_dep, b.i_dep), (2, 7, 5, 5, 0));
    assert_eq!((b.b.complete, b.b.clipped_frags, b.b.clipped_steps, b.b.clipped_max), (0, 1, 5, 5));
    assert_eq!((b.cycles, b.i.steps()), (0, 0));
    assert_eq!(b.new_cells(), 5);
}

#[test]
fn transition_exactly_at_band_hit_times() {
    // B->I transition lands exactly on tau(lo) and I->B exactly on tau(hi)
    //        t: 0     1     2      3      4     5
    let z = [true, true, false, false, true, true];
    let a = [1, 2, 3, 3, 3, 4];
    let g = geoms(6);
    let out = feed_all(&z, &a, &a, &g, &[(3, 4)]);
    let b = &out[0];
    // band: tau(3)=2, tau(4)=5 -> departures 2,3,4: I,I,B
    assert_eq!((b.t_start, b.t_end), (2, 5));
    assert_eq!((b.i.complete, b.i.complete_steps), (1, 2)); // I run [2,4) inside, both transitions observed
    assert_eq!((b.b.complete, b.b.clipped_frags, b.b.clipped_steps), (0, 1, 1)); // B run [4,5) cut by tau(hi)...
    assert_eq!(b, &offline(&z, &a, &a, &g, &[(3, 4)])[0]);
}

#[test]
fn budget_stop_leaves_partial_band_and_unreached_band_is_null() {
    let z = [true, false, false, true, false];
    let a = [1, 2, 2, 2, 3];
    let g = geoms(5);
    let out = feed_all(&z, &a, &a, &g, &[(2, 9), (20, 30)]);
    assert!(out[0].reached_lo && !out[0].reached_hi);
    assert_eq!((out[0].t_end, out[0].steps), (4, 3));
    assert!(!out[1].reached_lo);
    assert_eq!(out[1], BandOut { lo: 20, hi: 30, ..Default::default() });
    assert_eq!(out, offline(&z, &a, &a, &g, &[(2, 9), (20, 30)]));
}

#[test]
fn nearest_rank_quantile() {
    let v: Vec<u32> = (1..=1000).rev().collect();
    assert_eq!(nearest_rank(&v, 0.999), Some(999));
    assert_eq!(nearest_rank(&v, 1.0), Some(1000));
    assert_eq!(nearest_rank(&[5, 1, 9, 3], 0.5), Some(3));
    assert_eq!(nearest_rank(&[], 0.5), None);
}

// ---------- producer oracle (a'): HashMap board, HashSet visited recount ----------

struct Stream {
    z: Vec<bool>,
    a: Vec<u64>,
    p: Vec<u64>,
    g: Vec<Geom>,
    pos: Vec<(i64, i64)>,
}

fn oracle_stream<W: Walker>(mut w: W, steps: u64) -> Stream {
    let mut col: HashMap<(i64, i64), u8> = HashMap::new();
    let mut vis: HashSet<(i64, i64)> = HashSet::new();
    let (mut pos, mut h) = ((0i64, 0i64), 0u8);
    vis.insert(pos);
    let mut s = Stream { z: vec![], a: vec![], p: vec![], g: vec![], pos: vec![] };
    let nb = |q: (i64, i64)| DIRS.map(|(dx, dy)| (q.0 + dx as i64, q.1 + dy as i64));
    for t in 0..=steps {
        s.z.push(nb(pos).iter().any(|q| !vis.contains(q)));
        s.a.push(vis.len() as u64);
        s.p.push(vis.iter().filter(|&&q| nb(q).iter().any(|r| !vis.contains(r))).count() as u64);
        let (minx, maxx) = (vis.iter().map(|q| q.0).min().unwrap(), vis.iter().map(|q| q.0).max().unwrap());
        let (miny, maxy) = (vis.iter().map(|q| q.1).min().unwrap(), vis.iter().map(|q| q.1).max().unwrap());
        s.g.push(Geom {
            width: (maxx - minx + 1) as u32,
            height: (maxy - miny + 1) as u32,
            sx: vis.iter().map(|q| q.0).sum(),
            sy: vis.iter().map(|q| q.1).sum(),
            sxx: vis.iter().map(|q| q.0 * q.0 + q.1 * q.1).sum(),
        });
        s.pos.push(pos);
        if t == steps {
            break;
        }
        let cell = col.entry(pos).or_insert_with(|| if W::RANDOM_INIT { w.initial_cell(pos.0, pos.1) } else { 0 });
        w.step(cell, &mut h);
        let (dx, dy) = DIRS[h as usize];
        pos = (pos.0 + dx as i64, pos.1 + dy as i64);
        vis.insert(pos);
    }
    s
}

fn fast_stream<W: Walker>(w: W, steps: u64, size: usize) -> (Stream, Stop) {
    let mut s = Stream { z: vec![], a: vec![], p: vec![], g: vec![], pos: vec![] };
    let (stop, _) = drive(w, size, steps, |st| {
        s.z.push(st.z);
        s.a.push(st.area);
        s.p.push(st.perimeter);
        s.g.push(st.geom);
        false
    });
    (s, stop)
}

fn same_stream(f: &Stream, o: &Stream) {
    assert_eq!(f.z, o.z[..f.z.len()], "labels");
    assert_eq!(f.a, o.a[..f.a.len()], "areas");
    assert_eq!(f.p, o.p[..f.p.len()], "perimeters");
    assert_eq!(f.g, o.g[..f.g.len()], "geometry");
}

fn arb_rule() -> impl Strategy<Value = Rule> {
    proptest::collection::vec(prop_oneof![Just('L'), Just('R')], 2..8)
        .prop_filter("needs both turns", |v| v.contains(&'L') && v.contains(&'R'))
        .prop_map(|v| v.into_iter().collect::<String>().parse().unwrap())
}

const BANDS: [(u64, u64); 3] = [(4, 9), (9, 30), (45, 70)];

fn check_run<W: Walker>(mk: impl Fn() -> W, steps: u64) {
    let (f, stop) = fast_stream(mk(), steps, 1024);
    assert_eq!(stop, Stop::Budget);
    let o = oracle_stream(mk(), steps);
    same_stream(&f, &o);
    // Full pipeline vs offline oracle on the recounted arrays, truncated where run() stops.
    let r = run(mk(), 1024, steps, &BANDS);
    let n = r.t_final as usize + 1;
    let g = &o.g[..n];
    let off = offline(&o.z[..n], &o.a[..n], &o.p[..n], g, &BANDS);
    assert_eq!(r.bands, off);
    if r.stop == Stop::Sink {
        assert_eq!(r.area_final, BANDS[2].1);
        assert!(r.bands.iter().all(|b| b.reached_hi));
    } else {
        assert_eq!(r.stop, Stop::Budget);
        assert_eq!(r.t_final, steps);
    }
    for b in &r.bands {
        assert_eq!(b.fresh_from_interior, 0);
        assert_eq!((b.b.steps(), b.i.steps()), (b.b_dep, b.i_dep));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(40))]
    #[test]
    fn ant_pipeline_matches_oracle(r in arb_rule()) {
        check_run(|| AntWalker::new(&r), 400);
    }
    #[test]
    fn patched_ant_pipeline_matches_oracle(r in arb_rule(), seed in any::<u64>()) {
        check_run(|| PatchedAnt::new(&r, seed, 2), 400);
    }
    #[test]
    fn rotor_pipeline_matches_oracle(seed in any::<u64>()) {
        check_run(|| Rotor { seed }, 400);
    }
    #[test]
    fn random_walk_pipeline_matches_oracle(seed in any::<u64>()) {
        check_run(|| RandomWalk { state: seed }, 400);
    }
}

#[test]
fn oracle_positions_match_reference_walkers() {
    let r: Rule = "RLLR".parse().unwrap();
    let o = oracle_stream(AntWalker::new(&r), 500);
    let mut a = reference::Ant::new(r);
    for t in 0..=500 {
        assert_eq!(o.pos[t], (a.pos.0 as i64, a.pos.1 as i64), "ant t={t}");
        a.step();
    }
    let o = oracle_stream(Rotor { seed: 7 }, 500);
    let mut a = reference::Rotor::new(7);
    for t in 0..=500 {
        assert_eq!(o.pos[t], (a.pos.0 as i64, a.pos.1 as i64), "rotor t={t}");
        a.step();
    }
    let o = oracle_stream(RandomWalk { state: 7 }, 500);
    let mut a = reference::RandomWalk::new(7);
    for t in 0..=500 {
        assert_eq!(o.pos[t], (a.pos.0 as i64, a.pos.1 as i64), "rw t={t}");
        a.step();
    }
}

#[test]
fn escape_ends_the_run_at_the_first_edge_state() {
    // 20x20 board: c = 10, escape once |x| or |y| >= 8
    let o = oracle_stream(RandomWalk { state: 3 }, 5000);
    let first = o.pos.iter().position(|&(x, y)| x.abs() >= 8 || y.abs() >= 8).expect("walk reaches the edge");
    let r = run(RandomWalk { state: 3 }, 20, 5000, &[(5, 12), (400, 500)]);
    assert_eq!(r.stop, Stop::Escaped);
    assert_eq!(r.t_final as usize, first);
    let n = first + 1;
    assert_eq!(r.bands, offline(&o.z[..n], &o.a[..n], &o.p[..n], &o.g[..n], &[(5, 12), (400, 500)]));
    assert!(!r.bands[1].reached_lo);
}

#[test]
fn a_fresh_area_hit_can_fill_an_interior_hole() {
    // First-area-hit sampling does not guarantee B at the band edge: the
    // last fresh cell can have four already-visited neighbours.
    use turmite::fixed_area::Region;
    let size = 16;
    let mut region = Region::new(size);
    let index = |x: i64, y: i64| ((y + 8) * size as i64 + x + 8) as usize;
    for (x, y) in [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)] {
        region.visit(index(x, y), x, y);
    }
    assert!(!region.visited(index(1, 1)));
    region.visit(index(1, 1), 1, 1);
    assert_eq!(region.area, 9);
    assert!(!region.on_boundary(index(1, 1)));
    assert_eq!(region.perimeter, 8);
}
