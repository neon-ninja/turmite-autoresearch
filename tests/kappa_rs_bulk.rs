//! `bulk::fingerprint` against an obvious O(n * lag) oracle on random trajectories, and `bulk::run_bulk`
//! against the reference ant (positions, colours, headings, depth mask, reuse flags) and against `probe`
//! (the step at which the area first reaches A0).

use proptest::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use turmite::bulk::*;
use turmite::colourflux::MAXC;
use turmite::probe::probe;
use turmite::reference::Ant;
use turmite::walkers::{PersistentWalk, Rotor, mix64};
use turmite::{DIRS, Rule};

fn all_bulk(tr: &Traj, a: usize, b: usize) -> bool {
    tr.bulk[a..=b].iter().all(|&v| v)
}

/// Direct restatement of the definitions in the W5 addendum, no run-length bookkeeping.
fn brute(tr: &Traj) -> Fingerprint {
    let n = tr.len();
    let s = |j: usize| -> i64 {
        match tr.dh[j] {
            1 => 1,
            3 => -1,
            _ => 0,
        }
    };
    let mut fp = Fingerprint {
        n_rec: n as u64,
        n_bulk: 0,
        sum_s: 0,
        sum_s2: 0,
        turn_s: vec![0; MAXLAG + 1],
        turn_n: vec![0; MAXLAG + 1],
        msd_sum: [0; 3],
        msd_n: [0; 3],
        head_sum: 0,
        head_n: 0,
        ret_hit: 0,
        ret_elig: 0,
        gap_hist: [0; GAP_BINS],
        n_reuse: 0,
        dh_hist: [0; 4],
        col_steps: [0; MAXC],
    };
    for j in 0..n {
        if !tr.bulk[j] {
            continue;
        }
        fp.n_bulk += 1;
        fp.sum_s += s(j);
        fp.sum_s2 += s(j) * s(j);
        fp.dh_hist[tr.dh[j] as usize] += 1;
        fp.col_steps[tr.colour[j] as usize] += 1;
        fp.n_reuse += tr.reuse[j] as u64;
        for m in 1..=MAXLAG {
            if j >= m && all_bulk(tr, j - m, j) {
                fp.turn_s[m] += s(j) * s(j - m);
                fp.turn_n[m] += 1;
            }
        }
        for (a, &l) in MSD_LAGS.iter().enumerate() {
            if j >= l && all_bulk(tr, j - l, j) {
                let (dx, dy) = ((tr.x[j] - tr.x[j - l]) as i64, (tr.y[j] - tr.y[j - l]) as i64);
                fp.msd_sum[a] += (dx * dx + dy * dy) as u64;
                fp.msd_n[a] += 1;
            }
        }
        if j >= HEAD_LAG && all_bulk(tr, j - HEAD_LAG, j) {
            let d = (tr.head[j] as i64 - tr.head[j - HEAD_LAG] as i64).rem_euclid(4);
            fp.head_sum += [1, 0, -1, 0][d as usize];
            fp.head_n += 1;
        }
    }
    for i in 0..n {
        let next = (i + 1..n).find(|&j| tr.cell[j] == tr.cell[i]);
        if let Some(j) = next {
            if all_bulk(tr, i, j) {
                fp.gap_hist[(j - i).ilog2() as usize] += 1;
            }
        }
        if i + RET_WIN < n && all_bulk(tr, i, i + RET_WIN) {
            fp.ret_elig += 1;
            if next.is_some_and(|j| j - i <= RET_WIN) {
                fp.ret_hit += 1;
            }
        }
    }
    fp
}

fn synth(seed: u64, n: usize, stay: f64) -> Traj {
    let mut st = seed;
    let mut rnd = move || {
        st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix64(st)
    };
    let mut tr = Traj::default();
    let (mut x, mut y, mut h) = (0i32, 0i32, 0u8);
    let mut bulk = true;
    for _ in 0..n {
        let r = rnd();
        if (r >> 40) as f64 / (1u64 << 24) as f64 > stay {
            bulk = !bulk;
        }
        // Mostly bulk: flip back quickly out of non-bulk stretches.
        if !bulk && r & 7 == 0 {
            bulk = true;
        }
        let dh = match (r >> 8) & 7 {
            0 | 1 | 2 => 1,
            3 | 4 | 5 => 3,
            6 => 0,
            _ => 2,
        } as u8;
        tr.x.push(x);
        tr.y.push(y);
        tr.cell.push(((x & 15) + 16 * (y & 15)) as u32);
        tr.colour.push(((r >> 16) % 5) as u8);
        tr.bulk.push(bulk);
        tr.reuse.push((r >> 20) & 3 == 0);
        h = (h + dh) & 3;
        tr.dh.push(dh);
        tr.head.push(h);
        x += DIRS[h as usize].0;
        y += DIRS[h as usize].1;
    }
    tr
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn fingerprint_matches_brute(seed in 0u64..100_000, n in 50usize..1500, stay in 0.9f64..0.9999) {
        let tr = synth(seed, n, stay);
        prop_assert_eq!(fingerprint(&tr, 256), brute(&tr));
    }
}

#[test]
fn fingerprint_matches_brute_long_lags() {
    // n > 4096 with very long bulk runs so the 2^12 lag has pairs.
    let tr = synth(7, 6000, 0.99995);
    let fp = fingerprint(&tr, 256);
    assert!(fp.msd_n[2] > 0 && fp.turn_n[256] > 0 && fp.head_n > 0, "{:?}", fp.msd_n);
    assert_eq!(fp, brute(&tr));
}

/// Manhattan distance to the nearest unvisited cell, by BFS outward from the unvisited ring (the production code
/// goes inward from the boundary cells, so this is a different computation).
fn depth_oracle(visited: &HashSet<(i32, i32)>, margin: i32) -> HashMap<(i32, i32), i32> {
    let (x0, x1) = (visited.iter().map(|p| p.0).min().unwrap() - margin, visited.iter().map(|p| p.0).max().unwrap() + margin);
    let (y0, y1) = (visited.iter().map(|p| p.1).min().unwrap() - margin, visited.iter().map(|p| p.1).max().unwrap() + margin);
    let mut d: HashMap<(i32, i32), i32> = HashMap::new();
    let mut q = VecDeque::new();
    for x in x0..=x1 {
        for y in y0..=y1 {
            if !visited.contains(&(x, y)) {
                d.insert((x, y), 0);
                q.push_back((x, y));
            }
        }
    }
    while let Some(p) = q.pop_front() {
        let dp = d[&p];
        for (dx, dy) in DIRS {
            let r = (p.0 + dx, p.1 + dy);
            if r.0 < x0 || r.0 > x1 || r.1 < y0 || r.1 > y1 || d.contains_key(&r) {
                continue;
            }
            d.insert(r, dp + 1);
            q.push_back(r);
        }
    }
    d
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn run_bulk_matches_reference_ant(bits in 0u32..(1 << 9), n in 2usize..=9, a0 in 30u64..400, t_rec in 300usize..3000, d0 in 1u32..4) {
        let rule = Rule::from_bits(bits, n);
        let params = BulkParams { size: 512, a0, t_rec, d0, max_steps: 1 << 22 };
        let run = run_bulk(turmite::walkers::AntWalker::new(&rule), &params, true);
        prop_assume!(run.status == Status::Ok);
        let tr = run.traj.as_ref().unwrap();

        let mut ant = Ant::new(rule.clone());
        let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
        let mut starts: Vec<(i32, i32)> = vec![];
        while (visited.len() as u64) < a0 {
            starts.push(ant.pos);
            ant.step();
            visited.insert(ant.pos);
        }
        prop_assert_eq!(run.t_star, ant.steps);
        prop_assert_eq!(run.area0, visited.len() as u64);
        let snap0 = visited.clone();
        let depth = depth_oracle(&snap0, d0 as i32 + 3);
        let bulk_cell = |p: (i32, i32)| snap0.contains(&p) && depth[&p] >= d0 as i32 + 1;

        let mut last: HashMap<(i32, i32), u64> = starts.iter().enumerate().map(|(i, &p)| (p, i as u64 + 1)).collect();
        let mut h_prev = ant.heading;
        for i in 0..t_rec {
            let p = ant.pos;
            prop_assert_eq!((tr.x[i], tr.y[i]), p);
            prop_assert_eq!(tr.cell[i] as usize, (256 + p.1 as i64) as usize * 512 + (256 + p.0 as i64) as usize);
            prop_assert_eq!(tr.colour[i], ant.colour_at(p));
            prop_assert_eq!(tr.bulk[i], bulk_cell(p));
            let step_no = ant.steps + 1;
            let reuse = last.get(&p).is_some_and(|&l| step_no - l <= REUSE_WIN);
            prop_assert_eq!(tr.reuse[i], reuse);
            last.insert(p, step_no);
            ant.step();
            visited.insert(ant.pos);
            prop_assert_eq!(tr.head[i], ant.heading);
            prop_assert_eq!(tr.dh[i], (ant.heading + 4 - h_prev) & 3);
            h_prev = ant.heading;
        }
        prop_assert_eq!(run.area_end, visited.len() as u64);
        let mut mask = 0u64;
        let mut core = [0u64; MAXC];
        // Frozen mask: recompute from the A0 snapshot.
        let mut snap: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
        let mut a2 = Ant::new(rule.clone());
        while (snap.len() as u64) < a0 {
            a2.step();
            snap.insert(a2.pos);
        }
        let d0map = depth_oracle(&snap, d0 as i32 + 3);
        for &p in &snap {
            if d0map[&p] >= d0 as i32 + 1 {
                mask += 1;
                core[ant.colour_at(p) as usize] += 1;
            }
        }
        prop_assert_eq!(run.mask_cells, mask);
        prop_assert_eq!(run.core_hist, core);
        prop_assert_eq!(run.fp.as_ref().unwrap(), &brute(tr));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn first_hit_of_a0_agrees_with_probe(bits in 0u32..(1 << 9), n in 2usize..=9, a0 in 20u64..2000) {
        let rule = Rule::from_bits(bits, n);
        let pr = probe(&rule, 1024, 1, 14);
        prop_assume!(!pr.escaped);
        let run = run_bulk(turmite::walkers::AntWalker::new(&rule), &BulkParams { size: 1024, a0, t_rec: 10, d0: 2, max_steps: 1 << 14 }, false);
        for w in &pr.windows {
            // probe reports area at the end of each octave window; A0 is first reached by t_star.
            prop_assert_eq!(w.area >= a0, run.status != Status::NoA0 && run.t_star <= w.t_end);
        }
    }
}

#[test]
fn rotor_and_pw_trajectories_are_self_consistent() {
    let params = BulkParams { size: 1024, a0: 3000, t_rec: 5000, d0: 5, max_steps: 1 << 22 };
    for run in [run_bulk(Rotor { seed: 3 }, &params, true), run_bulk(PersistentWalk::new(5, 0.5), &params, true)] {
        assert_eq!(run.status, Status::Ok);
        let tr = run.traj.as_ref().unwrap();
        let mut h = tr.head[0];
        for i in 1..tr.len() {
            assert_eq!(tr.head[i], (h + tr.dh[i]) & 3);
            h = tr.head[i];
        }
        assert_eq!(run.fp.as_ref().unwrap(), &brute(tr));
        assert!(run.area_end >= run.area0);
    }
}
