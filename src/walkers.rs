//! Walkers the probe can drive. Each owns the rule for how a cell's state and
//! the walker's heading change on a visit; the probe owns the bookkeeping.
//!
//! - `AntWalker`: the generalised Langton's ant. Turn relative to heading by
//!   rule[colour], then advance the colour. Board starts blank.
//! - `Rotor`: the Eulerian (rotor-router) walker of Priezzhev et al. and Kapri &
//!   Dhar: rotate the cell's arrow 90 degrees clockwise, then step along it.
//!   Arrows start i.i.d. uniform, derived from a seed and the cell position so
//!   that independent implementations agree.
//! - `RandomWalk`: simple random walk, a negative control.
//!
//! Directions use the crate's encoding: 0 N, 1 E, 2 S, 3 W (screen coordinates,
//! so N -> E -> S -> W is clockwise).

use crate::Rule;

/// SplitMix64 finaliser: a good 64-bit mixing function.
pub fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Next value of a SplitMix64 stream.
pub fn splitmix_next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    mix64(*state)
}

/// The initial arrow (0..4) at (x, y), relative to the start, for a seed.
pub fn initial_arrow(seed: u64, x: i64, y: i64) -> u8 {
    let key = ((x as i32 as u32 as u64) << 32) | (y as i32 as u32 as u64);
    (mix64(mix64(seed ^ 0xA076_1D64_78BD_642F) ^ key) & 3) as u8
}

pub trait Walker {
    /// Whether unvisited cells start in a non-zero, position-dependent state.
    const RANDOM_INIT: bool;
    fn initial_cell(&self, x: i64, y: i64) -> u8;
    /// Update the cell under the walker and its heading for one step.
    fn step(&mut self, cell: &mut u8, heading: &mut u8);
}

pub struct AntWalker {
    turn: Vec<u8>,
    next: Vec<u8>,
}

impl AntWalker {
    pub fn new(rule: &Rule) -> AntWalker {
        let n = rule.len();
        AntWalker {
            turn: rule.0.iter().map(|t| t.delta()).collect(),
            next: (0..n).map(|i| ((i + 1) % n) as u8).collect(),
        }
    }
}

impl Walker for AntWalker {
    const RANDOM_INIT: bool = false;
    fn initial_cell(&self, _: i64, _: i64) -> u8 {
        0
    }
    #[inline(always)]
    fn step(&mut self, cell: &mut u8, heading: &mut u8) {
        let c = *cell as usize;
        *heading = (*heading + self.turn[c]) & 3;
        *cell = self.next[c];
    }
}

pub struct Rotor {
    pub seed: u64,
}

impl Walker for Rotor {
    const RANDOM_INIT: bool = true;
    fn initial_cell(&self, x: i64, y: i64) -> u8 {
        initial_arrow(self.seed, x, y)
    }
    #[inline(always)]
    fn step(&mut self, cell: &mut u8, heading: &mut u8) {
        *cell = (*cell + 1) & 3;
        *heading = *cell;
    }
}

pub struct RandomWalk {
    pub state: u64,
}

impl Walker for RandomWalk {
    const RANDOM_INIT: bool = false;
    fn initial_cell(&self, _: i64, _: i64) -> u8 {
        0
    }
    #[inline(always)]
    fn step(&mut self, _cell: &mut u8, heading: &mut u8) {
        *heading = (splitmix_next(&mut self.state) & 3) as u8;
    }
}

/// A generalised ant whose board starts blank except for a small square patch
/// of random colours around the start. Different seeds give independent runs
/// of the same rule, which measures how much a fitted exponent scatters for
/// one rule.
pub struct PatchedAnt {
    pub ant: AntWalker,
    pub n: u8,
    pub seed: u64,
    /// Half-width of the random patch.
    pub radius: i64,
}

impl PatchedAnt {
    pub fn new(rule: &Rule, seed: u64, radius: i64) -> PatchedAnt {
        PatchedAnt { ant: AntWalker::new(rule), n: rule.len() as u8, seed, radius }
    }
}

impl Walker for PatchedAnt {
    const RANDOM_INIT: bool = true;
    fn initial_cell(&self, x: i64, y: i64) -> u8 {
        if x.abs() <= self.radius && y.abs() <= self.radius {
            let key = ((x as i32 as u32 as u64) << 32) | (y as i32 as u32 as u64);
            (mix64(mix64(self.seed ^ 0x5851_F42D_4C95_7F2D) ^ key) % self.n as u64) as u8
        } else {
            0
        }
    }
    #[inline(always)]
    fn step(&mut self, cell: &mut u8, heading: &mut u8) {
        self.ant.step(cell, heading);
    }
}

/// Rotor walker whose initial arrows are biased towards north (C2 of the 2026-10-13 frontier):
/// with probability `bias` (a fraction of 2^32, so the run is exactly reproducible) the arrow is north,
/// otherwise uniform. `bias = 0` is not bit-identical to `Rotor` (different hash stream), only equal in law.
pub struct BiasedRotor {
    pub seed: u64,
    pub bias_q32: u64,
}

impl BiasedRotor {
    pub fn new(seed: u64, bias: f64) -> BiasedRotor {
        BiasedRotor { seed, bias_q32: (bias * 4294967296.0) as u64 }
    }
}

/// The initial arrow of `BiasedRotor` at (x, y); shared with the brute-force test.
pub fn biased_arrow(seed: u64, bias_q32: u64, x: i64, y: i64) -> u8 {
    let key = ((x as i32 as u32 as u64) << 32) | (y as i32 as u32 as u64);
    let h = mix64(mix64(seed ^ 0x3C6E_F372_FE94_F82B) ^ key);
    if (h >> 32) < bias_q32 { 0 } else { (h & 3) as u8 }
}

impl Walker for BiasedRotor {
    const RANDOM_INIT: bool = true;
    fn initial_cell(&self, x: i64, y: i64) -> u8 {
        biased_arrow(self.seed, self.bias_q32, x, y)
    }
    #[inline(always)]
    fn step(&mut self, cell: &mut u8, heading: &mut u8) {
        *cell = (*cell + 1) & 3;
        *heading = *cell;
    }
}

/// Generalised ant on a sparse random background (B2b of the 2026-10-13 frontier): each cell independently starts in a uniformly random
/// non-zero colour with probability `density` (a fraction of 2^32, so runs are exactly reproducible), otherwise blank.
pub struct SparseBgAnt {
    pub ant: AntWalker,
    pub n: u8,
    pub seed: u64,
    pub density_q32: u64,
}

impl SparseBgAnt {
    pub fn new(rule: &Rule, seed: u64, density: f64) -> SparseBgAnt {
        SparseBgAnt { ant: AntWalker::new(rule), n: rule.len() as u8, seed, density_q32: (density * 4294967296.0) as u64 }
    }
}

/// Initial colour of `SparseBgAnt` at (x, y); shared with the brute-force test.
pub fn sparse_bg_colour(seed: u64, density_q32: u64, n: u8, x: i64, y: i64) -> u8 {
    let key = ((x as i32 as u32 as u64) << 32) | (y as i32 as u32 as u64);
    let h = mix64(mix64(seed ^ 0x7F4A_7C15_9E37_79B9) ^ key);
    if (h >> 32) < density_q32 { 1 + ((h & 0xFFFF_FFFF) % (n as u64 - 1)) as u8 } else { 0 }
}

impl Walker for SparseBgAnt {
    const RANDOM_INIT: bool = true;
    fn initial_cell(&self, x: i64, y: i64) -> u8 {
        sparse_bg_colour(self.seed, self.density_q32, self.n, x, y)
    }
    #[inline(always)]
    fn step(&mut self, cell: &mut u8, heading: &mut u8) {
        self.ant.step(cell, heading);
    }
}
