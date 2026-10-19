//! Exact ant phase updates with an explicitly bounded colour cache.
//! Forgotten phases are reset only on a later read; visited geometry is external.

use crate::edge_trace::Layout;
use crate::walkers::{AntWalker, Walker, splitmix_next};
use crate::{DIRS, Rule};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    Uniform,
    Zero,
}

#[derive(Clone, Debug, Default)]
pub struct Exposure {
    pub resets: Rc<Cell<u64>>,
    pub changed: Rc<Cell<u64>>,
    pub last_reset: Rc<Cell<bool>>,
    pub last_changed: Rc<Cell<bool>>,
}

/// A ring cache for a square moving one lattice step at a time. The global
/// read-once bitset distinguishes first reads from forgotten re-reads; it
/// contains no phase information. No empirical reference path is consulted.
pub struct LocalMemoryAnt {
    ant: AntWalker,
    n: u64,
    radius: Option<usize>,
    side: usize,
    cache: Vec<Option<u8>>,
    read_once: Vec<bool>,
    layout: Layout,
    pub x: i64,
    pub y: i64,
    rng: u64,
    policy: Reset,
    prepared: bool,
    pub exposure: Exposure,
}

impl LocalMemoryAnt {
    pub fn new(
        rule: &Rule,
        layout: Layout,
        start: (usize, usize),
        radius: Option<usize>,
        seed: u64,
        policy: Reset,
    ) -> Self {
        let side = radius.map_or(1, |r| 2 * r + 1);
        assert!(side < layout.w && side < layout.h);
        Self {
            ant: AntWalker::new(rule),
            n: rule.len() as u64,
            radius,
            side,
            cache: vec![None; side * side],
            read_once: vec![false; layout.w * layout.h],
            layout,
            x: start.0 as i64,
            y: start.1 as i64,
            rng: seed,
            policy,
            prepared: false,
            exposure: Exposure::default(),
        }
    }

    fn index(&self, x: i64, y: i64) -> usize {
        y.rem_euclid(self.side as i64) as usize * self.side
            + x.rem_euclid(self.side as i64) as usize
    }

    pub fn prepare(&mut self, cell: &mut u8, _: u8) {
        assert!(!self.prepared, "prepare called twice without a step");
        self.prepared = true;
        self.exposure.last_reset.set(false);
        self.exposure.last_changed.set(false);
        if self.radius.is_none() {
            return;
        }
        let i = self.index(self.x, self.y);
        let global = self.y.rem_euclid(self.layout.h as i64) as usize * self.layout.w
            + self.x.rem_euclid(self.layout.w as i64) as usize;
        if let Some(c) = self.cache[i] {
            assert_eq!(*cell, c, "resident phase diverged from generated field");
        } else if self.read_once[global] {
            let old = *cell;
            *cell = match self.policy {
                Reset::Uniform => (splitmix_next(&mut self.rng) % self.n) as u8,
                Reset::Zero => 0,
            };
            self.exposure.resets.set(self.exposure.resets.get() + 1);
            self.exposure.last_reset.set(true);
            self.exposure.last_changed.set(old != *cell);
            self.exposure
                .changed
                .set(self.exposure.changed.get() + u64::from(old != *cell));
        }
        self.read_once[global] = true;
        self.cache[i] = Some(*cell);
    }
}

impl Walker for LocalMemoryAnt {
    const RANDOM_INIT: bool = false;
    fn initial_cell(&self, _: i64, _: i64) -> u8 {
        0
    }

    fn step(&mut self, cell: &mut u8, heading: &mut u8) {
        assert!(self.prepared, "finite memory requires a pre-read hook");
        self.prepared = false;
        self.ant.step(cell, heading);
        if self.radius.is_some() {
            let i = self.index(self.x, self.y);
            self.cache[i] = Some(*cell);
        }
        let d = DIRS[*heading as usize];
        self.x += d.0 as i64;
        self.y += d.1 as i64;
        if let Some(r) = self.radius {
            let r = r as i64;
            // Incoming and outgoing strips alias in the ring. Clear exactly
            // that strip; cells staying within the new patch keep their phase.
            if d.0 != 0 {
                let column = (self.x + d.0 as i64 * r).rem_euclid(self.side as i64) as usize;
                for row in self.cache.chunks_exact_mut(self.side) {
                    row[column] = None;
                }
            } else {
                let row = (self.y + d.1 as i64 * r).rem_euclid(self.side as i64) as usize;
                self.cache[row * self.side..(row + 1) * self.side].fill(None);
            }
        }
    }
}
