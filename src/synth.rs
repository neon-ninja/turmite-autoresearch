//! Colour fields that match a source field's local pattern statistics.
//!
//! A [`Patch`] is a rectangle of symbols. Symbols `0..n` are colours and may
//! move; symbol `n` marks a cell that was never visited and is fixed. The
//! synthesiser only ever swaps two movable cells, so the colour counts are
//! preserved exactly. It anneals towards a target histogram of window
//! patterns (for example all oriented 2x2 blocks), minimising the L1 distance
//! between the current and target histograms.
//!
//! [`histogram`] is the obvious full recount and is the oracle; [`Annealer`]
//! keeps the distance up to date incrementally and is tested against it.

use std::collections::HashMap;

use crate::walkers::splitmix_next;

#[derive(Clone, Debug, PartialEq)]
pub struct Patch {
    pub w: usize,
    pub h: usize,
    /// Number of colours; symbol `n` is "never visited".
    pub n: u8,
    pub cells: Vec<u8>,
}

impl Patch {
    pub fn movable(&self, i: usize) -> bool {
        self.cells[i] < self.n
    }
}

/// Window shapes (width, height) for each level of the ladder.
pub fn shapes(level: &str) -> Vec<(usize, usize)> {
    match level {
        "pairs" => vec![(2, 1), (1, 2)],
        "b2" => vec![(2, 2)],
        "b3" => vec![(3, 3)],
        _ => panic!("unknown level {level}"),
    }
}

/// Pattern key: shape index in the top bits, then the symbols in base n+1.
fn key(p: &Patch, shape_ix: usize, (sw, sh): (usize, usize), x: usize, y: usize) -> u64 {
    let base = p.n as u64 + 1;
    let mut k = 0u64;
    for dy in 0..sh {
        for dx in 0..sw {
            k = k * base + p.cells[(y + dy) * p.w + x + dx] as u64;
        }
    }
    ((shape_ix as u64) << 56) | k
}

/// Full histogram of every window that fits inside the patch.
pub fn histogram(p: &Patch, shapes: &[(usize, usize)]) -> HashMap<u64, i64> {
    let mut h = HashMap::new();
    for (s, &(sw, sh)) in shapes.iter().enumerate() {
        for y in 0..=p.h - sh {
            for x in 0..=p.w - sw {
                *h.entry(key(p, s, (sw, sh), x, y)).or_insert(0) += 1;
            }
        }
    }
    h
}

pub fn l1(a: &HashMap<u64, i64>, b: &HashMap<u64, i64>) -> i64 {
    let mut d: i64 = a.iter().map(|(k, &v)| (v - b.get(k).copied().unwrap_or(0)).abs()).sum();
    d += b.iter().filter(|(k, _)| !a.contains_key(k)).map(|(_, &v)| v.abs()).sum::<i64>();
    d
}

/// Uniform random permutation of the movable cells' colours.
pub fn shuffle(p: &mut Patch, seed: &mut u64) {
    let idx: Vec<usize> = (0..p.cells.len()).filter(|&i| p.movable(i)).collect();
    for a in (1..idx.len()).rev() {
        let b = (splitmix_next(seed) % (a as u64 + 1)) as usize;
        p.cells.swap(idx[a], idx[b]);
    }
}

/// Permute colours only among movable cells whose distance from (cx, cy)
/// falls in the same ring of width `ring`.
pub fn radial_shuffle(p: &mut Patch, cx: f64, cy: f64, ring: f64, seed: &mut u64) {
    let mut rings: HashMap<i64, Vec<usize>> = HashMap::new();
    for i in (0..p.cells.len()).filter(|&i| p.movable(i)) {
        let (x, y) = ((i % p.w) as f64, (i / p.w) as f64);
        rings.entry(((x - cx).hypot(y - cy) / ring) as i64).or_default().push(i);
    }
    let mut keys: Vec<_> = rings.keys().copied().collect();
    keys.sort();
    for k in keys {
        let idx = &rings[&k];
        for a in (1..idx.len()).rev() {
            let b = (splitmix_next(seed) % (a as u64 + 1)) as usize;
            p.cells.swap(idx[a], idx[b]);
        }
    }
}

pub struct Annealer {
    pub p: Patch,
    shapes: Vec<(usize, usize)>,
    /// current histogram minus target histogram (zero entries may linger)
    diff: HashMap<u64, i64>,
    pub cost: i64,
    movable: Vec<usize>,
    buf: Vec<(usize, usize, usize)>,
}

impl Annealer {
    pub fn new(p: Patch, target: &HashMap<u64, i64>, shapes: Vec<(usize, usize)>) -> Annealer {
        let mut diff = histogram(&p, &shapes);
        for (k, v) in target {
            *diff.entry(*k).or_insert(0) -= v;
        }
        diff.retain(|_, v| *v != 0);
        let cost = diff.values().map(|v| v.abs()).sum();
        let movable = (0..p.cells.len()).filter(|&i| p.movable(i)).collect();
        Annealer { p, shapes, diff, cost, movable, buf: Vec::new() }
    }

    /// All windows (shape, x, y) containing cell i or cell j, deduplicated.
    fn affected(&mut self, i: usize, j: usize) {
        self.buf.clear();
        for (s, &(sw, sh)) in self.shapes.iter().enumerate() {
            for c in [i, j] {
                let (cx, cy) = (c % self.p.w, c / self.p.w);
                for y in cy.saturating_sub(sh - 1)..=cy.min(self.p.h - sh) {
                    for x in cx.saturating_sub(sw - 1)..=cx.min(self.p.w - sw) {
                        self.buf.push((s, x, y));
                    }
                }
            }
        }
        self.buf.sort_unstable();
        self.buf.dedup();
    }

    fn tally(&mut self, sign: i64) -> i64 {
        let mut delta = 0;
        for &(s, x, y) in &self.buf {
            let k = key(&self.p, s, self.shapes[s], x, y);
            let e = self.diff.entry(k).or_insert(0);
            delta -= e.abs();
            *e += sign;
            delta += e.abs();
            if *e == 0 {
                // missing means zero; keeps the map from growing without bound
                self.diff.remove(&k);
            }
        }
        delta
    }

    /// Swap cells i and j and return the change in cost.
    pub fn swap(&mut self, i: usize, j: usize) -> i64 {
        self.affected(i, j);
        let mut d = self.tally(-1);
        self.p.cells.swap(i, j);
        d += self.tally(1);
        self.cost += d;
        d
    }

    /// One Metropolis proposal at temperature t. Half the partners are local
    /// (within 3 cells), half anywhere. Returns true if accepted.
    pub fn propose(&mut self, t: f64, seed: &mut u64) -> bool {
        let m = self.movable.len() as u64;
        let i = self.movable[(splitmix_next(seed) % m) as usize];
        let j = if splitmix_next(seed) & 1 == 0 {
            self.movable[(splitmix_next(seed) % m) as usize]
        } else {
            let r = splitmix_next(seed);
            let dx = (r % 7) as i64 - 3;
            let dy = ((r >> 8) % 7) as i64 - 3;
            let (x, y) = ((i % self.p.w) as i64 + dx, (i / self.p.w) as i64 + dy);
            if x < 0 || y < 0 || x >= self.p.w as i64 || y >= self.p.h as i64 {
                return false;
            }
            let j = y as usize * self.p.w + x as usize;
            if !self.p.movable(j) {
                return false;
            }
            j
        };
        if self.p.cells[i] == self.p.cells[j] {
            return false;
        }
        let d = self.swap(i, j);
        if d <= 0 {
            return true;
        }
        let u = (splitmix_next(seed) >> 11) as f64 / (1u64 << 53) as f64;
        if u < (-(d as f64) / t).exp() {
            return true;
        }
        self.swap(i, j);
        false
    }

    /// Geometric cooling from t0 to t1 over `sweeps` sweeps of
    /// (number of movable cells) proposals. Stops early at cost 0.
    pub fn entries(&self) -> usize {
        self.diff.len()
    }

    pub fn anneal(&mut self, t0: f64, t1: f64, sweeps: usize, seed: &mut u64) {
        let per = self.movable.len();
        for s in 0..sweeps {
            let t = t0 * (t1 / t0).powf(s as f64 / (sweeps.max(2) - 1) as f64);
            for _ in 0..per {
                self.propose(t, seed);
            }
            if self.cost == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn patch(w: usize, h: usize, n: u8, seed: u64) -> Patch {
        let mut s = seed;
        // a blank border and a few blank holes, like a real territory
        let cells = (0..w * h)
            .map(|i| {
                let (x, y) = (i % w, i / w);
                let r = splitmix_next(&mut s);
                if x == 0 || y == 0 || x == w - 1 || y == h - 1 || r % 9 == 0 { n } else { (r % n as u64) as u8 }
            })
            .collect();
        Patch { w, h, n, cells }
    }

    fn counts(p: &Patch) -> Vec<usize> {
        (0..=p.n).map(|c| p.cells.iter().filter(|&&x| x == c).count()).collect()
    }

    proptest! {
        #[test]
        fn incremental_cost_matches_recount(seed in any::<u64>(), lv in 0usize..3, n in 2u8..6,
                                            w in 4usize..12, h in 4usize..12) {
            let level = ["pairs", "b2", "b3"][lv];
            let src = patch(w, h, n, seed);
            let target = histogram(&src, &shapes(level));
            let mut start = src.clone();
            let mut s = seed ^ 0xABCD;
            shuffle(&mut start, &mut s);
            let mut a = Annealer::new(start, &target, shapes(level));
            prop_assert_eq!(a.cost, l1(&histogram(&a.p, &shapes(level)), &target));
            for k in 0..300 {
                a.propose(if k % 2 == 0 { 0.5 } else { 5.0 }, &mut s);
                prop_assert_eq!(a.cost, l1(&histogram(&a.p, &shapes(level)), &target));
            }
            prop_assert_eq!(counts(&a.p), counts(&src));
            for i in 0..src.cells.len() {
                if !src.movable(i) { prop_assert_eq!(a.p.cells[i], src.n); }
            }
        }

        #[test]
        fn radial_shuffle_keeps_ring_contents(seed in any::<u64>()) {
            let src = patch(15, 15, 4, seed);
            let mut p = src.clone();
            let mut s = seed;
            radial_shuffle(&mut p, 7.0, 7.0, 2.0, &mut s);
            let ring = |i: usize| (((i % 15) as f64 - 7.0).hypot((i / 15) as f64 - 7.0) / 2.0) as i64;
            for r in 0..8 {
                let mut a: Vec<u8> = (0..225).filter(|&i| ring(i) == r).map(|i| src.cells[i]).collect();
                let mut b: Vec<u8> = (0..225).filter(|&i| ring(i) == r).map(|i| p.cells[i]).collect();
                a.sort(); b.sort();
                prop_assert_eq!(a, b);
            }
        }
    }

    #[test]
    fn annealing_reaches_target_on_a_small_field() {
        let src = patch(10, 10, 3, 7);
        let target = histogram(&src, &shapes("pairs"));
        let mut start = src.clone();
        let mut s = 99;
        shuffle(&mut start, &mut s);
        let mut a = Annealer::new(start, &target, shapes("pairs"));
        let c0 = a.cost;
        a.anneal(2.0, 0.05, 400, &mut s);
        assert!(a.cost < c0 / 4, "{c0} -> {}", a.cost);
    }

    #[test]
    fn a_wrong_window_is_caught() {
        // the recount must see a change anywhere in the window
        let mut p = patch(6, 6, 3, 1);
        let h0 = histogram(&p, &shapes("b3"));
        let i = 2 * 6 + 2;
        p.cells[i] = (p.cells[i] + 1) % 3;
        assert_ne!(l1(&histogram(&p, &shapes("b3")), &h0), 0);
    }
}
