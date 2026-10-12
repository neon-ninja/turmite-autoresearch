//! The obviously-correct ant: an unbounded sparse grid in a HashMap.

use crate::{DIRS, Rule};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Ant {
    pub rule: Rule,
    pub cells: HashMap<(i32, i32), u8>,
    pub pos: (i32, i32),
    pub heading: u8,
    pub steps: u64,
}

impl Ant {
    pub fn new(rule: Rule) -> Ant {
        Ant {
            rule,
            cells: HashMap::new(),
            pos: (0, 0),
            heading: 0,
            steps: 0,
        }
    }

    pub fn colour_at(&self, p: (i32, i32)) -> u8 {
        self.cells.get(&p).copied().unwrap_or(0)
    }

    pub fn step(&mut self) {
        let n = self.rule.len() as u8;
        let c = self.colour_at(self.pos);
        self.heading = (self.heading + self.rule.0[c as usize].delta()) % 4;
        let next = (c + 1) % n;
        if next == 0 {
            self.cells.remove(&self.pos);
        } else {
            self.cells.insert(self.pos, next);
        }
        let (dx, dy) = DIRS[self.heading as usize];
        self.pos = (self.pos.0 + dx, self.pos.1 + dy);
        self.steps += 1;
    }

    /// Exactly undoes one `step`. The dynamics are reversible, which makes a
    /// strong property test: run forward k steps, back k steps, get the origin.
    pub fn unstep(&mut self) {
        let n = self.rule.len() as u8;
        let (dx, dy) = DIRS[self.heading as usize];
        self.pos = (self.pos.0 - dx, self.pos.1 - dy);
        let c = (self.colour_at(self.pos) + n - 1) % n;
        if c == 0 {
            self.cells.remove(&self.pos);
        } else {
            self.cells.insert(self.pos, c);
        }
        self.heading = (self.heading + 4 - self.rule.0[c as usize].delta()) % 4;
        self.steps -= 1;
    }

    pub fn run(&mut self, steps: u64) {
        for _ in 0..steps {
            self.step();
        }
    }
}

/// The obviously-correct rotor (Eulerian) walker: arrows in a HashMap,
/// created on first visit from the same seeded function the fast probe uses.
/// Rule (Kapri & Dhar): rotate the arrow 90 degrees clockwise, then step along it.
#[derive(Clone, Debug)]
pub struct Rotor {
    pub seed: u64,
    pub arrows: HashMap<(i32, i32), u8>,
    pub pos: (i32, i32),
    pub steps: u64,
}

impl Rotor {
    pub fn new(seed: u64) -> Rotor {
        Rotor { seed, arrows: HashMap::new(), pos: (0, 0), steps: 0 }
    }

    pub fn step(&mut self) {
        let (x, y) = self.pos;
        let seed = self.seed;
        let arrow = self.arrows.entry(self.pos).or_insert_with(|| crate::walkers::initial_arrow(seed, x as i64, y as i64));
        *arrow = (*arrow + 1) % 4;
        let (dx, dy) = DIRS[*arrow as usize];
        self.pos = (x + dx, y + dy);
        self.steps += 1;
    }
}

/// Simple random walk driven by a SplitMix64 stream.
#[derive(Clone, Debug)]
pub struct RandomWalk {
    pub state: u64,
    pub pos: (i32, i32),
    pub steps: u64,
}

impl RandomWalk {
    pub fn new(seed: u64) -> RandomWalk {
        RandomWalk { state: seed, pos: (0, 0), steps: 0 }
    }

    pub fn step(&mut self) {
        let d = (crate::walkers::splitmix_next(&mut self.state) % 4) as usize;
        let (dx, dy) = DIRS[d];
        self.pos = (self.pos.0 + dx, self.pos.1 + dy);
        self.steps += 1;
    }
}
