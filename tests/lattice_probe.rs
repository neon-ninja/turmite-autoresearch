//! `lattice_probe` against (G1) `probe::probe` on the square lattice, (G2) an obvious HashMap re-implementation on all three
//! lattices and all three movers, and (G3) single-letter loop lengths read off the visited area.

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::DIRS;
use turmite::Rule;
use turmite::ext::{HEX_DIRS, HexSim, Sim, TriSim, hex_turns, tri_step};
use turmite::lattice_probe::{MOORE_DIRS, *};
use turmite::probe::probe;
use turmite::walkers::splitmix_next;

type P = (i64, i64);

fn neighbours(lat: Lattice, p: P) -> Vec<P> {
    match lat {
        Lattice::Square => DIRS.iter().map(|d| (p.0 + d.0 as i64, p.1 + d.1 as i64)).collect(),
        Lattice::Hex => HEX_DIRS.iter().map(|d| (p.0 + d.0 as i64, p.1 + d.1 as i64)).collect(),
        Lattice::Moore => MOORE_DIRS.iter().map(|d| (p.0 + d.0 as i64, p.1 + d.1 as i64)).collect(),
        Lattice::Tri => {
            let v = if (p.0 + p.1).rem_euclid(2) == 0 { 1 } else { -1 };
            vec![(p.0 - 1, p.1), (p.0 + 1, p.1), (p.0, p.1 + v)]
        }
    }
}

/// Slow reference: HashMap colours, visited set, perimeter recounted from scratch at each window end.
fn naive(lat: Lattice, mover: &Mover, size: usize, min_log2: u32, max_log2: u32) -> LResult {
    let c = (size / 2) as i64;
    let mut vis: HashSet<P> = HashSet::new();
    let mut col: HashMap<P, u8> = HashMap::new();
    let mut pos: P = (0, 0);
    vis.insert(pos);
    let (mut min, mut max) = ((0i64, 0i64), (0i64, 0i64));
    let tcode = |ch: char| -> i64 {
        match (lat, ch) {
            (Lattice::Square, 'R') => 1,
            (Lattice::Square, 'L') => 3,
            (Lattice::Hex, 'r') => 1,
            (Lattice::Hex, 'R') => 2,
            (Lattice::Hex, 'l') => 5,
            (Lattice::Hex, 'L') => 4,
            (Lattice::Moore, 'r') => 1,
            (Lattice::Moore, 'R') => 2,
            (Lattice::Moore, 'l') => 7,
            (Lattice::Moore, 'L') => 6,
            (Lattice::Tri, 'L') => 2,
            (Lattice::Tri, 'R') => -2,
            _ => panic!("bad letter"),
        }
    };
    let turns: Vec<i64> = match mover {
        Mover::Ant(r) => r.chars().map(tcode).collect(),
        _ => vec![],
    };
    let mut h: i64 = if lat == Lattice::Tri { 4 } else { 0 };
    let mut rng = match mover {
        Mover::RandomWalk(s) | Mover::MeanField(s) => *s,
        _ => 0,
    };
    let mut list: Vec<P> = vec![pos];
    let mut windows = vec![];
    let mut escaped = false;
    let mut t = 0u64;
    'outer: for k in 0..max_log2 {
        let (mut nc, mut bs, mut fr) = (0u64, 0u64, 0u64);
        while t < (1u64 << (k + 1)) {
            if let Mover::MeanField(_) = mover {
                pos = list[(splitmix_next(&mut rng) % list.len() as u64) as usize];
            }
            let fresh = neighbours(lat, pos).iter().filter(|q| !vis.contains(q)).count() as u64;
            if fresh > 0 {
                bs += 1;
                fr += fresh;
            }
            match mover {
                Mover::Ant(_) => {
                    let cc = col.entry(pos).or_insert(0);
                    let turn = turns[*cc as usize];
                    *cc = ((*cc as usize + 1) % turns.len()) as u8;
                    match lat {
                        Lattice::Square => {
                            h = (h + turn).rem_euclid(4);
                            pos = (pos.0 + DIRS[h as usize].0 as i64, pos.1 + DIRS[h as usize].1 as i64);
                        }
                        Lattice::Hex => {
                            h = (h + turn).rem_euclid(6);
                            pos = (pos.0 + HEX_DIRS[h as usize].0 as i64, pos.1 + HEX_DIRS[h as usize].1 as i64);
                        }
                        Lattice::Moore => {
                            h = (h + turn).rem_euclid(8);
                            pos = (pos.0 + MOORE_DIRS[h as usize].0 as i64, pos.1 + MOORE_DIRS[h as usize].1 as i64);
                        }
                        Lattice::Tri => {
                            let exit = (h + turn).rem_euclid(6);
                            let (dx, dy) = tri_step((pos.0 + pos.1).rem_euclid(2) == 0, exit as u8);
                            pos = (pos.0 + dx, pos.1 + dy);
                            h = (exit + 3) % 6;
                        }
                    }
                }
                _ => {
                    let nb = neighbours(lat, pos);
                    pos = nb[(splitmix_next(&mut rng) % nb.len() as u64) as usize];
                }
            }
            t += 1;
            if vis.insert(pos) {
                nc += 1;
                list.push(pos);
                min = (min.0.min(pos.0), min.1.min(pos.1));
                max = (max.0.max(pos.0), max.1.max(pos.1));
            }
            if pos.0.abs() >= c - 2 || pos.1.abs() >= c - 2 {
                escaped = true;
                break 'outer;
            }
        }
        if k >= min_log2 {
            let perimeter = vis.iter().filter(|p| neighbours(lat, **p).iter().any(|q| !vis.contains(q))).count() as u64;
            windows.push(LWindow {
                k,
                area: vis.len() as u64,
                perimeter,
                new_cells: nc,
                boundary_steps: bs,
                fresh_nbrs: fr,
                width: (max.0 - min.0 + 1) as u32,
                height: (max.1 - min.1 + 1) as u32,
            });
        }
    }
    LResult { windows, escaped }
}

fn letters(alphabet: &'static str, lo: usize, hi: usize) -> impl Strategy<Value = String> {
    proptest::collection::vec(proptest::sample::select(alphabet.chars().collect::<Vec<_>>()), lo..hi).prop_map(|v| v.into_iter().collect())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(40))]

    /// G1: identical to `probe.rs` on the square lattice (fields the lattice probe produces).
    #[test]
    fn g1_square_matches_probe(rule in letters("LR", 2, 10)) {
        let r: Rule = rule.parse().unwrap();
        let a = probe(&r, 256, 1, 14);
        let b = lattice_probe(Lattice::Square, &Mover::Ant(rule.clone()), 256, 1, 14);
        prop_assert_eq!(a.escaped, b.escaped);
        prop_assert_eq!(a.windows.len(), b.windows.len());
        for (x, y) in a.windows.iter().zip(&b.windows) {
            prop_assert_eq!((x.area, x.perimeter, x.new_cells, x.boundary_steps, x.fresh_nbrs, x.width, x.height),
                            (y.area, y.perimeter, y.new_cells, y.boundary_steps, y.fresh_nbrs, y.width, y.height));
        }
    }

    #[test]
    fn g2_square_ant(rule in letters("LR", 2, 12)) {
        let m = Mover::Ant(rule);
        let (a, b) = (lattice_probe(Lattice::Square, &m, 128, 1, 12), naive(Lattice::Square, &m, 128, 1, 12));
        prop_assert_eq!(a.windows, b.windows);
        prop_assert_eq!(a.escaped, b.escaped);
    }

    #[test]
    fn g2_hex_ant(rule in letters("rlRL", 2, 12)) {
        let m = Mover::Ant(rule);
        let (a, b) = (lattice_probe(Lattice::Hex, &m, 128, 1, 12), naive(Lattice::Hex, &m, 128, 1, 12));
        prop_assert_eq!(a.windows, b.windows);
        prop_assert_eq!(a.escaped, b.escaped);
    }

    #[test]
    fn g2_tri_ant(rule in letters("LR", 2, 12)) {
        let m = Mover::Ant(rule);
        let (a, b) = (lattice_probe(Lattice::Tri, &m, 128, 1, 12), naive(Lattice::Tri, &m, 128, 1, 12));
        prop_assert_eq!(a.windows, b.windows);
        prop_assert_eq!(a.escaped, b.escaped);
    }

    #[test]
    fn g2_moore_ant(rule in letters("rlRL", 2, 12)) {
        let m = Mover::Ant(rule);
        let (a, b) = (lattice_probe(Lattice::Moore, &m, 128, 1, 12), naive(Lattice::Moore, &m, 128, 1, 12));
        prop_assert_eq!(a.windows, b.windows);
        prop_assert_eq!(a.escaped, b.escaped);
    }

    #[test]
    fn g2_random_movers(seed in 1u64..1000, lat in 0usize..4, mf in proptest::bool::ANY) {
        let lat = [Lattice::Square, Lattice::Hex, Lattice::Tri, Lattice::Moore][lat];
        let m = if mf { Mover::MeanField(seed) } else { Mover::RandomWalk(seed) };
        let (a, b) = (lattice_probe(lat, &m, 128, 1, 11), naive(lat, &m, 128, 1, 11));
        prop_assert_eq!(a.windows, b.windows);
        prop_assert_eq!(a.escaped, b.escaped);
    }
}

/// The ant dynamics agree with the existing simulators (`HexSim`, `TriSim`): area after every power-of-two step count.
#[test]
fn ant_area_matches_ext_sims() {
    for rule in ["rRlL", "RRrlL", "rrRLl", "RLRRLLRL"] {
        let mut sim = HexSim::new(hex_turns(rule).unwrap(), 512);
        let res = lattice_probe(Lattice::Hex, &Mover::Ant(rule.into()), 512, 1, 14);
        let mut t = 0u64;
        for w in &res.windows {
            while t < 1u64 << (w.k + 1) {
                sim.step();
                t += 1;
            }
            assert_eq!(sim.area(), w.area, "hex {rule} k={}", w.k);
        }
    }
    for rule in ["RL", "RRLRLLR", "LLRRRLRL"] {
        let r: Rule = rule.parse().unwrap();
        let mut sim = TriSim::new(&r, 512);
        let res = lattice_probe(Lattice::Tri, &Mover::Ant(rule.into()), 512, 1, 14);
        let mut t = 0u64;
        for w in &res.windows {
            while t < 1u64 << (w.k + 1) {
                sim.step();
                t += 1;
            }
            assert_eq!(sim.area(), w.area, "tri {rule} k={}", w.k);
        }
    }
}

/// G3: a one-letter rule cycles forever; the visited area is the cycle length.
#[test]
fn g3_loop_lengths() {
    let area = |lat, letter: &str| {
        let r = lattice_probe(lat, &Mover::Ant(letter.into()), 64, 3, 6);
        assert!(!r.escaped);
        r.windows.last().unwrap().area
    };
    assert_eq!(area(Lattice::Square, "R"), 4);
    assert_eq!(area(Lattice::Square, "L"), 4);
    assert_eq!(area(Lattice::Hex, "r"), 6);
    assert_eq!(area(Lattice::Hex, "l"), 6);
    assert_eq!(area(Lattice::Hex, "R"), 3);
    assert_eq!(area(Lattice::Hex, "L"), 3);
    assert_eq!(area(Lattice::Moore, "r"), 8);
    assert_eq!(area(Lattice::Moore, "l"), 8);
    assert_eq!(area(Lattice::Moore, "R"), 4);
    assert_eq!(area(Lattice::Moore, "L"), 4);
    assert_eq!(area(Lattice::Tri, "R"), 6);
    assert_eq!(area(Lattice::Tri, "L"), 6);
}
