//! Excursion geometry in size-free units (T1b, notes/excursion-tomography-2026-10-13.md section 7).
//!
//! usage: sectors MAX_LOG2 GRID OUT_PREFIX < walkers.txt
//! writes OUT_PREFIX_windows.csv and OUT_PREFIX_bins.csv
//!
//! Same boundary/interior state machine as `excursions.rs`, but each completed
//! excursion also records its angular displacement about the visited-set
//! centroid, its Euclidean displacement and depth in cells, and is tallied in a
//! log2 bin of its length. Window accounting is exact: interior steps equal the
//! in-window part of completed excursions plus the open part at the window end.

use rayon::prelude::*;
use std::io::Write;
use turmite::DIRS;
use turmite::Rule;
use turmite::walkers::{AntWalker, PatchedAnt, RandomWalk, Rotor, Walker};

const UNVISITED: u8 = u8::MAX;
pub const NBINS: usize = 48;

#[derive(Clone, Debug, Default)]
pub struct Bin {
    pub count: u64,
    pub time_sum: u64,
    pub dtheta_sum: f64,
    pub dcells_sum: f64,
    pub depth_sum: f64,
    pub depth_max: f64,
}

#[derive(Clone, Debug)]
pub struct SectorWindow {
    pub k: u32,
    pub area: u64,
    pub perimeter: u64,
    pub new_cells: u64,
    pub boundary_steps: u64,
    pub interior_steps: u64,
    /// In-window interior steps of excursions that completed in this window.
    pub part_completed: u64,
    /// In-window interior steps of the excursion still open at the window end.
    pub part_open: u64,
    pub open_len: u64,
    pub bins: Vec<Bin>,
}

impl SectorWindow {
    pub fn conserved(&self) -> bool {
        self.part_completed + self.part_open == self.interior_steps
            && self.boundary_steps + self.interior_steps == 1u64 << self.k
    }
}

struct Region {
    unv: Vec<u8>,
    nbr: [isize; 4],
    area: u64,
    perimeter: u64,
    sx: i64,
    sy: i64,
}

impl Region {
    fn new(size: usize) -> Self {
        let s = size as isize;
        Region { unv: vec![UNVISITED; size * size], nbr: [-s, 1, s, -1], area: 0, perimeter: 0, sx: 0, sy: 0 }
    }
    #[inline(always)]
    fn visited(&self, idx: usize) -> bool {
        self.unv[idx] != UNVISITED
    }
    #[inline(always)]
    fn on_boundary(&self, idx: usize) -> bool {
        self.visited(idx) && self.unv[idx] > 0
    }
    fn visit(&mut self, idx: usize, x: i64, y: i64) {
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
    }
    fn centroid(&self) -> (f64, f64) {
        let a = self.area as f64;
        (self.sx as f64 / a, self.sy as f64 / a)
    }
}

/// Angle in [0, pi] between two points as seen from `c`.
pub fn angle_between(a: (i64, i64), b: (i64, i64), c: (f64, f64)) -> f64 {
    let (ax, ay) = (a.0 as f64 - c.0, a.1 as f64 - c.1);
    let (bx, by) = (b.0 as f64 - c.0, b.1 as f64 - c.1);
    (ax * by - ay * bx).abs().atan2(ax * bx + ay * by)
}

pub fn len_bin(l: u64) -> usize {
    (63 - l.leading_zeros() as usize).min(NBINS - 1)
}

fn dist(a: (i64, i64), c: (f64, f64)) -> f64 {
    ((a.0 as f64 - c.0).powi(2) + (a.1 as f64 - c.1).powi(2)).sqrt()
}

pub fn run_sectors<W: Walker>(mut walker: W, size: usize, min_log2: u32, max_log2: u32) -> (Vec<SectorWindow>, bool) {
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

    let mut in_exc = false;
    let mut start = (0i64, 0i64);
    let mut exc_len = 0u64; // 1 + interior steps made so far while in an excursion
    let mut min_r = f64::MAX;
    let mut carry; // interior steps of the open excursion made before this window

    for k in 0..max_log2 {
        let t_end = 1u64 << (k + 1);
        let record = k >= min_log2;
        let (mut boundary_steps, mut interior_steps, mut new_cells) = (0u64, 0u64, 0u64);
        let mut part_completed = 0u64;
        let mut bins = vec![Bin::default(); NBINS];
        carry = if in_exc { exc_len - 1 } else { 0 };
        let mut first_open = in_exc; // the open excursion began before this window

        while t < t_end {
            let was_boundary = region.on_boundary(idx);
            let prev = (x, y);
            if was_boundary {
                boundary_steps += 1;
            } else {
                interior_steps += 1;
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
            let is_boundary = region.on_boundary(idx);
            let cen = region.centroid();

            if was_boundary && !is_boundary {
                in_exc = true;
                start = prev;
                exc_len = 1;
                min_r = dist((x, y), cen);
                first_open = false;
                carry = 0;
            } else if !was_boundary && !is_boundary {
                exc_len += 1;
                min_r = min_r.min(dist((x, y), cen));
            } else if !was_boundary && is_boundary {
                // exc_len counts the pending first interior step, so it equals L.
                let l = exc_len;
                part_completed += l - if first_open { carry } else { 0 };
                first_open = false;
                carry = 0;
                if record {
                    let b = &mut bins[len_bin(l)];
                    b.count += 1;
                    b.time_sum += l;
                    b.dtheta_sum += angle_between(start, (x, y), cen);
                    b.dcells_sum += dist(start, (x as f64, y as f64));
                    let depth = (dist(start, cen) - min_r).max(0.0);
                    b.depth_sum += depth;
                    b.depth_max = b.depth_max.max(depth);
                }
                in_exc = false;
            }

            if x.abs() >= c - 2 || y.abs() >= c - 2 {
                escaped = true;
                break;
            }
        }
        if escaped {
            break;
        }
        if record {
            let open_len = if in_exc { exc_len - 1 } else { 0 };
            let part_open = if in_exc { open_len - carry } else { 0 };
            windows.push(SectorWindow {
                k,
                area: region.area,
                perimeter: region.perimeter,
                new_cells,
                boundary_steps,
                interior_steps,
                part_completed,
                part_open,
                open_len,
                bins,
            });
        }
    }
    (windows, escaped)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let max_log2: u32 = a.get(1).expect("usage: sectors MAX_LOG2 GRID OUT_PREFIX").parse().unwrap();
    let size: usize = a.get(2).expect("GRID").parse().unwrap();
    let prefix = a.get(3).expect("OUT_PREFIX");
    let specs: Vec<String> =
        std::io::stdin().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
    eprintln!("{} specs to run to 2^{}", specs.len(), max_log2);

    let done = std::sync::atomic::AtomicUsize::new(0);
    let results: Vec<(String, Vec<SectorWindow>, bool)> = specs
        .par_iter()
        .map(|spec| {
            let (w, esc) = if let Some(seed) = spec.strip_prefix("rotor:") {
                run_sectors(Rotor { seed: seed.parse().unwrap() }, size, 22, max_log2)
            } else if let Some(seed) = spec.strip_prefix("rw:") {
                run_sectors(RandomWalk { state: seed.parse().unwrap() }, size, 22, max_log2)
            } else if let Some((rule, seed)) = spec.split_once('@') {
                run_sectors(PatchedAnt::new(&rule.parse::<Rule>().unwrap(), seed.parse().unwrap(), 2), size, 22, max_log2)
            } else {
                run_sectors(AntWalker::new(&spec.parse::<Rule>().unwrap()), size, 22, max_log2)
            };
            let n = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if n % 20 == 0 || n == specs.len() {
                eprintln!("{}/{}", n, specs.len());
            }
            (spec.clone(), w, esc)
        })
        .collect();

    let mut wf = std::io::BufWriter::new(std::fs::File::create(format!("{prefix}_windows.csv")).unwrap());
    let mut bf = std::io::BufWriter::new(std::fs::File::create(format!("{prefix}_bins.csv")).unwrap());
    writeln!(wf, "spec,k,area,perimeter,new_cells,boundary_steps,interior_steps,part_completed,part_open,open_len,kappa,c,escaped,conserved").unwrap();
    writeln!(bf, "spec,k,bin,count,time_sum,dtheta_sum,dcells_sum,depth_sum,depth_max").unwrap();
    for (spec, windows, esc) in &results {
        for w in windows {
            let steps = (1u64 << w.k) as f64;
            let kappa = (w.boundary_steps as f64 / steps) / (w.perimeter as f64 / w.area as f64);
            let cc = if w.boundary_steps > 0 { w.new_cells as f64 / w.boundary_steps as f64 } else { 0.0 };
            writeln!(
                wf,
                "{},{},{},{},{},{},{},{},{},{},{:.4},{:.4},{},{}",
                spec, w.k, w.area, w.perimeter, w.new_cells, w.boundary_steps, w.interior_steps, w.part_completed,
                w.part_open, w.open_len, kappa, cc, esc, w.conserved()
            )
            .unwrap();
            for (i, b) in w.bins.iter().enumerate().filter(|(_, b)| b.count > 0) {
                writeln!(
                    bf,
                    "{},{},{},{},{},{:.6},{:.4},{:.4},{:.4}",
                    spec, w.k, i, b.count, b.time_sum, b.dtheta_sum, b.dcells_sum, b.depth_sum, b.depth_max
                )
                .unwrap();
            }
        }
    }
}
