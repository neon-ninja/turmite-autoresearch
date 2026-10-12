//! An instrumented ant for studying how chaotic patterns grow.
//!
//! Besides cell colours it tracks the *visited* region (every cell the ant has
//! ever stood on, whatever its colour now) and, incrementally:
//! - `area`: number of visited cells,
//! - `perimeter`: visited cells with at least one unvisited 4-neighbour,
//! - per window: steps starting on a perimeter cell, and steps that enter a
//!   never-visited cell,
//! - sums for the visited set's radius of gyration and for the ant's own
//!   time-averaged spread, to check whether it is spread evenly.
//!
//! Windows are octaves of time: [2^k, 2^(k+1)).

use crate::walkers::{AntWalker, Walker};
use crate::{DIRS, Rule};

#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    /// Window covers steps t in (t_start, t_end].
    pub t_start: u64,
    pub t_end: u64,
    pub area: u64,
    pub perimeter: u64,
    pub new_cells: u64,
    pub boundary_steps: u64,
    /// Sum, over steps starting on a boundary cell, of that cell's number of
    /// unvisited neighbours. If the ant's heading were uncorrelated with the
    /// edge, new_cells / boundary_steps would be about this / (4 * boundary_steps).
    pub fresh_nbrs: u64,
    /// Radius of gyration squared of the visited set at t_end.
    pub rg2: f64,
    /// Time-average of the ant's squared distance from the visited set's
    /// centroid (centroid taken at t_end), over the window.
    pub ant_r2: f64,
    /// Variance of the ant's position about its own time-average over the
    /// window: the spread of where it actually spends its time. Rare long
    /// excursions barely move this.
    pub ant_var: f64,
    /// Bounding box of the visited set at t_end.
    pub width: u32,
    pub height: u32,
}

pub struct ProbeResult {
    pub windows: Vec<Window>,
    /// The ant left the board before the last window finished.
    pub escaped: bool,
}

const UNVISITED: u8 = u8::MAX;

/// The visited region, updated one newly visited cell at a time.
struct Region {
    /// For visited cells: count of unvisited 4-neighbours. UNVISITED otherwise.
    unv: Vec<u8>,
    nbr: [isize; 4],
    area: u64,
    perimeter: u64,
    // Sums over visited cells, coordinates relative to the start.
    sx: i64,
    sy: i64,
    sxx: i64,
    min: (i64, i64),
    max: (i64, i64),
}

impl Region {
    fn new(size: usize) -> Region {
        let s = size as isize;
        Region {
            unv: vec![UNVISITED; size * size],
            nbr: [-s, 1, s, -1],
            area: 0,
            perimeter: 0,
            sx: 0,
            sy: 0,
            sxx: 0,
            min: (0, 0),
            max: (0, 0),
        }
    }

    fn visited(&self, idx: usize) -> bool {
        self.unv[idx] != UNVISITED
    }

    fn on_boundary(&self, idx: usize) -> bool {
        self.visited(idx) && self.unv[idx] > 0
    }

    fn fresh_neighbours(&self, idx: usize) -> u64 {
        if self.visited(idx) { self.unv[idx] as u64 } else { 4 }
    }

    fn visit(&mut self, idx: usize, x: i64, y: i64) {
        debug_assert!(!self.visited(idx));
        let mut u = 0u8;
        for d in self.nbr {
            let j = (idx as isize + d) as usize;
            if self.unv[j] == UNVISITED {
                u += 1;
            } else {
                self.unv[j] -= 1;
                if self.unv[j] == 0 {
                    self.perimeter -= 1;
                }
            }
        }
        self.unv[idx] = u;
        if u > 0 {
            self.perimeter += 1;
        }
        self.area += 1;
        self.sx += x;
        self.sy += y;
        self.sxx += x * x + y * y;
        self.min = (self.min.0.min(x), self.min.1.min(y));
        self.max = (self.max.0.max(x), self.max.1.max(y));
    }

    fn centroid(&self) -> (f64, f64) {
        let a = self.area as f64;
        (self.sx as f64 / a, self.sy as f64 / a)
    }

    fn rg2(&self) -> f64 {
        let (mx, my) = self.centroid();
        self.sxx as f64 / self.area as f64 - mx * mx - my * my
    }
}

/// Run `rule` for 2^max_log2 steps on a size x size board, reporting windows
/// (2^k, 2^(k+1)] for min_log2 <= k < max_log2. Stops early if the ant gets
/// within two cells of the edge.
pub fn probe(rule: &Rule, size: usize, min_log2: u32, max_log2: u32) -> ProbeResult {
    probe_walker(AntWalker::new(rule), size, min_log2, max_log2)
}

/// As `probe`, for any walker.
pub fn probe_walker<W: Walker>(mut walker: W, size: usize, min_log2: u32, max_log2: u32) -> ProbeResult {
    assert!(min_log2 >= 1);
    let c = (size / 2) as i64;
    let mut colour = vec![0u8; size * size];
    if W::RANDOM_INIT {
        for (i, cell) in colour.iter_mut().enumerate() {
            let (gx, gy) = ((i % size) as i64, (i / size) as i64);
            *cell = walker.initial_cell(gx - c, gy - c);
        }
    }
    let mut region = Region::new(size);
    let s = size as isize;

    let (mut x, mut y, mut h) = (0i64, 0i64, 0u8);
    let mut idx = (c * size as i64 + c) as usize;
    region.visit(idx, 0, 0);

    let mut windows = vec![];
    let mut escaped = false;
    let mut t: u64 = 0;
    for k in 0..max_log2 {
        let (t_start, t_end) = (1u64 << k, 1u64 << (k + 1));
        let (mut new_cells, mut boundary_steps, mut fresh_nbrs) = (0u64, 0u64, 0u64);
        let (mut ax, mut ay, mut axx) = (0f64, 0f64, 0f64);
        while t < t_end {
            // A step can only enter fresh ground if it starts on the boundary.
            if region.on_boundary(idx) {
                boundary_steps += 1;
                fresh_nbrs += region.fresh_neighbours(idx);
            }
            walker.step(&mut colour[idx], &mut h);
            let (dx, dy) = DIRS[h as usize];
            x += dx as i64;
            y += dy as i64;
            idx = (idx as isize + dy as isize * s + dx as isize) as usize;
            t += 1;
            if !region.visited(idx) {
                new_cells += 1;
                region.visit(idx, x, y);
            }
            let (fx, fy) = (x as f64, y as f64);
            ax += fx;
            ay += fy;
            axx += fx * fx + fy * fy;
            if x.abs() >= c - 2 || y.abs() >= c - 2 {
                escaped = true;
                break;
            }
        }
        if escaped {
            break;
        }
        if k >= min_log2 {
            let (mx, my) = region.centroid();
            let m = (t_end - t_start) as f64;
            // <|p - m|^2> = <|p|^2> - 2 m.<p> + |m|^2
            let ant_r2 = axx / m - 2.0 * (mx * ax + my * ay) / m + mx * mx + my * my;
            let ant_var = axx / m - (ax / m).powi(2) - (ay / m).powi(2);
            windows.push(Window {
                t_start,
                t_end,
                area: region.area,
                perimeter: region.perimeter,
                new_cells,
                boundary_steps,
                fresh_nbrs,
                rg2: region.rg2(),
                ant_r2,
                ant_var,
                width: (region.max.0 - region.min.0 + 1) as u32,
                height: (region.max.1 - region.min.1 + 1) as u32,
            });
        }
    }
    ProbeResult { windows, escaped }
}
