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

#[derive(Clone, Debug)]
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

}

/// Raw sums for one finished window. Both the CPU probe and the GPU probe fill this in and call
/// `finish_window`, so the floating-point formulas (and operation order) exist exactly once.
/// `ax`, `ay`, `axx` are integer-valued sums of the ant's x, y and x^2 + y^2 over the window's steps;
/// they are exact in f64 while they stay below 2^53 (`window_sums_exact` checks the bound).
pub struct WindowSums {
    pub k: u32,
    pub area: u64,
    pub perimeter: u64,
    pub new_cells: u64,
    pub boundary_steps: u64,
    pub fresh_nbrs: u64,
    /// Sums over the visited set (coordinates relative to the start).
    pub sx: i64,
    pub sy: i64,
    pub sxx: i64,
    pub ax: f64,
    pub ay: f64,
    pub axx: f64,
    pub min: (i64, i64),
    pub max: (i64, i64),
}

pub fn finish_window(w: &WindowSums) -> Window {
    let (t_start, t_end) = (1u64 << w.k, 1u64 << (w.k + 1));
    let a = w.area as f64;
    let (mx, my) = (w.sx as f64 / a, w.sy as f64 / a);
    let m = (t_end - t_start) as f64;
    let (ax, ay, axx) = (w.ax, w.ay, w.axx);
    // <|p - m|^2> = <|p|^2> - 2 m.<p> + |m|^2
    let ant_r2 = axx / m - 2.0 * (mx * ax + my * ay) / m + mx * mx + my * my;
    let ant_var = axx / m - (ax / m).powi(2) - (ay / m).powi(2);
    Window {
        t_start,
        t_end,
        area: w.area,
        perimeter: w.perimeter,
        new_cells: w.new_cells,
        boundary_steps: w.boundary_steps,
        fresh_nbrs: w.fresh_nbrs,
        rg2: w.sxx as f64 / a - mx * mx - my * my,
        ant_r2,
        ant_var,
        width: (w.max.0 - w.min.0 + 1) as u32,
        height: (w.max.1 - w.min.1 + 1) as u32,
    }
}

/// Whether a window of 2^max_log2 steps on a board of side `size` keeps `ax`, `ay`, `axx` below 2^53,
/// where integer sums in f64 (CPU) and in i64 (GPU) agree exactly. The ant stays within |x|, |y| < size/2.
pub fn window_sums_exact(size: usize, max_log2: u32) -> bool {
    let r = (size / 2) as f64;
    (max_log2 as f64).exp2() * 2.0 * r * r < 53f64.exp2()
}

/// The CSV header of `bin/probe` and `bin/gpu_probe`.
pub const CSV_HEADER: &str =
    "rule,k,t,area,perimeter,new_cells,boundary_steps,fresh_nbrs,escaped,rg2,ant_r2,ant_var,width,height";

/// One CSV line (with trailing newline) per emitted window of a run, in the format of `bin/probe`.
pub fn csv_rows(spec: &str, res: &ProbeResult) -> String {
    res.windows
        .iter()
        .map(|w| {
            format!(
                "{spec},{},{},{},{},{},{},{},{},{:.3},{:.3},{:.3},{},{}\n",
                w.t_end.trailing_zeros(),
                w.t_end,
                w.area,
                w.perimeter,
                w.new_cells,
                w.boundary_steps,
                w.fresh_nbrs,
                res.escaped,
                w.rg2,
                w.ant_r2,
                w.ant_var,
                w.width,
                w.height
            )
        })
        .collect()
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
        let t_end = 1u64 << (k + 1);
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
            windows.push(finish_window(&WindowSums {
                k,
                area: region.area,
                perimeter: region.perimeter,
                new_cells,
                boundary_steps,
                fresh_nbrs,
                sx: region.sx,
                sy: region.sy,
                sxx: region.sxx,
                ax,
                ay,
                axx,
                min: region.min,
                max: region.max,
            }));
        }
    }
    ProbeResult { windows, escaped }
}
