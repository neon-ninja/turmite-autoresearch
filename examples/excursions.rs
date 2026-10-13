//! Excursion tomography (T1 in notes/research-frontier-2026-10-13-pm.md and notes/excursion-tomography-2026-10-13.md).
//!
//! usage: excursions MAX_LOG2 [GRID] < walkers.txt > excursions.csv
//!
//! Measures the distribution of interior excursion durations, inward depths,
//! return displacements, and boundary contact dwell times across octave windows.

use rayon::prelude::*;
use std::io::Write;
use turmite::Rule;
use turmite::walkers::{AntWalker, PatchedAnt, RandomWalk, Rotor, Walker};
use turmite::DIRS;

const UNVISITED: u8 = u8::MAX;

#[derive(Clone, Debug)]
pub struct ExcursionSummary {
    pub len: u64,
    pub disp: f64,
    pub depth: f64,
}

#[derive(Clone, Debug)]
pub struct ExcursionWindow {
    pub k: u32,
    pub t_end: u64,
    pub area: u64,
    pub perimeter: u64,
    pub new_cells: u64,
    pub boundary_steps: u64,
    pub interior_steps: u64,
    pub n_completed: u64,
    pub l_mean: f64,
    pub l_med: f64,
    pub l_p90: f64,
    pub l_p99: f64,
    pub l_max: u64,
    pub top1_share: f64,
    pub top5_share: f64,
    pub unresolved_steps: u64,
    pub unresolved_share: f64,
    pub disp_med: f64,
    pub disp_mean: f64,
    pub depth_med: f64,
    pub depth_mean: f64,
    pub edge_episodes: u64,
    pub edge_dwell_mean: f64,
    pub kappa: f64,
    pub c: f64,
}

struct FastRegion {
    unv: Vec<u8>,
    nbr: [isize; 4],
    pub area: u64,
    pub perimeter: u64,
    pub sx: i64,
    pub sy: i64,
    pub min: (i64, i64),
    pub max: (i64, i64),
}

impl FastRegion {
    fn new(size: usize) -> Self {
        let s = size as isize;
        FastRegion {
            unv: vec![UNVISITED; size * size],
            nbr: [-s, 1, s, -1],
            area: 0,
            perimeter: 0,
            sx: 0,
            sy: 0,
            min: (0, 0),
            max: (0, 0),
        }
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
        self.min = (self.min.0.min(x), self.min.1.min(y));
        self.max = (self.max.0.max(x), self.max.1.max(y));
    }

    fn centroid(&self) -> (f64, f64) {
        let a = self.area as f64;
        (self.sx as f64 / a, self.sy as f64 / a)
    }
}

pub fn run_excursions<W: Walker>(
    mut walker: W,
    size: usize,
    min_log2: u32,
    max_log2: u32,
) -> (Vec<ExcursionWindow>, bool) {
    let c = (size / 2) as i64;
    let mut colour = vec![0u8; size * size];
    if W::RANDOM_INIT {
        for (i, cell) in colour.iter_mut().enumerate() {
            let (gx, gy) = ((i % size) as i64, (i / size) as i64);
            *cell = walker.initial_cell(gx - c, gy - c);
        }
    }
    let mut region = FastRegion::new(size);
    let s = size as isize;

    let (mut x, mut y, mut h) = (0i64, 0i64, 0u8);
    let mut idx = (c * size as i64 + c) as usize;
    region.visit(idx, 0, 0);

    let mut windows = vec![];
    let mut escaped = false;
    let mut t: u64 = 0;

    // Excursion state
    let mut in_excursion = false;
    let mut exc_start_x = 0i64;
    let mut exc_start_y = 0i64;
    let mut exc_len = 0u64;
    let mut exc_min_r = f64::MAX;

    // Track active excursion steps within current window
    let mut current_exc_steps_in_window = 0u64;

    for k in 0..max_log2 {
        let (t_start, t_end) = (1u64 << k, 1u64 << (k + 1));
        let record = k >= min_log2;

        let mut boundary_steps = 0u64;
        let mut interior_steps = 0u64;
        let mut new_cells = 0u64;
        let mut edge_episodes = 0u64;

        let mut completed_excursions = Vec::new();
        current_exc_steps_in_window = 0;

        while t < t_end {
            let was_boundary = region.on_boundary(idx);
            let prev_x = x;
            let prev_y = y;

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

            // Centroid coordinates for depth
            let (cx, cy) = region.centroid();
            let r_curr = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();

            if was_boundary && !is_boundary {
                // Stepped off the boundary into interior: start excursion
                in_excursion = true;
                exc_start_x = prev_x;
                exc_start_y = prev_y;
                exc_len = 1;
                exc_min_r = r_curr;
                current_exc_steps_in_window = 1;
            } else if !was_boundary && !is_boundary {
                // Continuing interior excursion
                exc_len += 1;
                exc_min_r = exc_min_r.min(r_curr);
                current_exc_steps_in_window += 1;
            } else if !was_boundary && is_boundary {
                // Returned to boundary: complete excursion
                let start_r = ((exc_start_x as f64 - cx).powi(2) + (exc_start_y as f64 - cy).powi(2)).sqrt();
                let disp = ((x as f64 - exc_start_x as f64).powi(2) + (y as f64 - exc_start_y as f64).powi(2)).sqrt();
                let depth = (start_r - exc_min_r).max(0.0);

                if record {
                    completed_excursions.push(ExcursionSummary {
                        len: exc_len,
                        disp,
                        depth,
                    });
                }
                in_excursion = false;
                current_exc_steps_in_window = 0;
                edge_episodes += 1;
            } else {
                // was_boundary && is_boundary
                // Walker remains on edge
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
            let r_eq = (region.area as f64 / std::f64::consts::PI).sqrt().max(1.0);
            let n_completed = completed_excursions.len() as u64;

            // Sort completed excursions by length
            completed_excursions.sort_by_key(|e| e.len);

            let l_med = if n_completed > 0 {
                completed_excursions[(n_completed / 2) as usize].len as f64
            } else {
                0.0
            };

            let l_p90 = if n_completed > 0 {
                let idx = ((n_completed as f64 * 0.90) as usize).min(completed_excursions.len() - 1);
                completed_excursions[idx].len as f64
            } else {
                0.0
            };

            let l_p99 = if n_completed > 0 {
                let idx = ((n_completed as f64 * 0.99) as usize).min(completed_excursions.len() - 1);
                completed_excursions[idx].len as f64
            } else {
                0.0
            };

            let l_max = completed_excursions.last().map(|e| e.len).unwrap_or(0);

            let l_sum: u64 = completed_excursions.iter().map(|e| e.len).sum();
            let l_mean = if n_completed > 0 {
                l_sum as f64 / n_completed as f64
            } else {
                0.0
            };

            // Top 1% and Top 5% time shares
            let top1_count = ((n_completed as f64 * 0.01).ceil() as usize).max(if n_completed > 0 { 1 } else { 0 });
            let top1_sum: u64 = if n_completed > 0 {
                completed_excursions[completed_excursions.len().saturating_sub(top1_count)..]
                    .iter()
                    .map(|e| e.len)
                    .sum()
            } else {
                0
            };

            let top5_count = ((n_completed as f64 * 0.05).ceil() as usize).max(if n_completed > 0 { 1 } else { 0 });
            let top5_sum: u64 = if n_completed > 0 {
                completed_excursions[completed_excursions.len().saturating_sub(top5_count)..]
                    .iter()
                    .map(|e| e.len)
                    .sum()
            } else {
                0
            };

            let total_interior = interior_steps.max(1) as f64;
            let top1_share = top1_sum as f64 / total_interior;
            let top5_share = top5_sum as f64 / total_interior;

            // Unresolved steps (active excursion crossing the window boundary)
            let unresolved_steps = if in_excursion { current_exc_steps_in_window } else { 0 };
            let unresolved_share = unresolved_steps as f64 / total_interior;

            // Displacements normalized by R_eq
            let mut disps: Vec<f64> = completed_excursions.iter().map(|e| e.disp / r_eq).collect();
            disps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let disp_med = if !disps.is_empty() { disps[disps.len() / 2] } else { 0.0 };
            let disp_mean = if !disps.is_empty() { disps.iter().sum::<f64>() / disps.len() as f64 } else { 0.0 };

            // Depths normalized by R_eq
            let mut depths: Vec<f64> = completed_excursions.iter().map(|e| e.depth / r_eq).collect();
            depths.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let depth_med = if !depths.is_empty() { depths[depths.len() / 2] } else { 0.0 };
            let depth_mean = if !depths.is_empty() { depths.iter().sum::<f64>() / depths.len() as f64 } else { 0.0 };

            let edge_dwell_mean = if edge_episodes > 0 {
                boundary_steps as f64 / edge_episodes as f64
            } else {
                boundary_steps as f64
            };

            let steps = (t_end - t_start) as f64;
            let kappa = (boundary_steps as f64 / steps) / (region.perimeter as f64 / region.area as f64);
            let c = if boundary_steps > 0 { new_cells as f64 / boundary_steps as f64 } else { 0.0 };

            windows.push(ExcursionWindow {
                k,
                t_end,
                area: region.area,
                perimeter: region.perimeter,
                new_cells,
                boundary_steps,
                interior_steps,
                n_completed,
                l_mean,
                l_med,
                l_p90,
                l_p99,
                l_max,
                top1_share,
                top5_share,
                unresolved_steps,
                unresolved_share,
                disp_med,
                disp_mean,
                depth_med,
                depth_mean,
                edge_episodes,
                edge_dwell_mean,
                kappa,
                c,
            });
        }
    }

    (windows, escaped)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let max_log2: u32 = a.get(1).expect("usage: excursions MAX_LOG2 [GRID]").parse().unwrap();
    let size: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(8192);

    let specs: Vec<String> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    eprintln!("{} specs to run to 2^{}", specs.len(), max_log2);

    let done = std::sync::atomic::AtomicUsize::new(0);
    let rows: Vec<String> = specs
        .par_iter()
        .map(|spec| {
            let (windows, escaped) = if let Some(seed) = spec.strip_prefix("rotor:") {
                run_excursions(Rotor { seed: seed.parse().unwrap() }, size, 22, max_log2)
            } else if let Some(seed) = spec.strip_prefix("rw:") {
                run_excursions(RandomWalk { state: seed.parse().unwrap() }, size, 22, max_log2)
            } else if let Some((rule, seed)) = spec.split_once('@') {
                run_excursions(
                    PatchedAnt::new(&rule.parse::<Rule>().unwrap(), seed.parse().unwrap(), 2),
                    size,
                    22,
                    max_log2,
                )
            } else {
                run_excursions(
                    AntWalker::new(&spec.parse::<Rule>().unwrap()),
                    size,
                    22,
                    max_log2,
                )
            };

            let count = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if count % 20 == 0 || count == specs.len() {
                eprintln!("{}/{}", count, specs.len());
            }

            windows
                .iter()
                .map(|w| {
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{:.2},{:.2},{:.2},{:.2},{},{:.4},{:.4},{},{:.4},{:.4},{:.4},{:.4},{:.4},{},{:.3},{:.4},{:.4},{}\n",
                        spec,
                        w.k,
                        w.t_end,
                        w.area,
                        w.perimeter,
                        w.new_cells,
                        w.boundary_steps,
                        w.interior_steps,
                        w.n_completed,
                        w.l_max,
                        w.l_mean,
                        w.l_med,
                        w.l_p90,
                        w.l_p99,
                        w.top1_share,
                        w.top5_share,
                        w.unresolved_steps,
                        w.unresolved_share,
                        w.disp_med,
                        w.disp_mean,
                        w.depth_med,
                        w.depth_mean,
                        w.edge_episodes,
                        w.edge_dwell_mean,
                        w.kappa,
                        w.c,
                        escaped,
                        (w.boundary_steps + w.interior_steps) == (1u64 << w.k),
                    )
                })
                .collect()
        })
        .collect();

    let mut out = std::io::stdout().lock();
    writeln!(
        out,
        "spec,k,t_end,area,perimeter,new_cells,boundary_steps,interior_steps,n_completed,l_max,l_mean,l_med,l_p90,l_p99,top1_share,top5_share,unresolved_steps,unresolved_share,disp_med,disp_mean,depth_med,depth_mean,edge_episodes,edge_dwell_mean,kappa,c,escaped,exact_budget"
    )
    .unwrap();

    for r in rows {
        out.write_all(r.as_bytes()).unwrap();
    }
}
