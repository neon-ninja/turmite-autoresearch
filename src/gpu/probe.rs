//! GPU probe: `probe::probe_walker` for thousands of ants at once (plain rules and `PatchedAnt` seeds).
//!
//! Contract: for every [`ProbeJob`], [`GpuProber::run`] returns exactly the [`ProbeResult`] of
//! `probe::probe(rule, grid, min_log2, max_log2)` (no patch seed) or
//! `probe::probe_walker(PatchedAnt::new(rule, seed, 2), grid, min_log2, max_log2)`, with every field
//! equal (floats included: both paths build windows with `probe::finish_window`). A job's result does
//! not depend on lane count, slice length, pool pressure or the other jobs.
//!
//! Differences from the sweep kernel ([`super::GpuSweeper`]): no heading ring or highway detection;
//! a 16-bit cell (colour byte + "unvisited neighbours + 1" byte, 0 = never visited, so an all-zero tile
//! is blank and unvisited); the visited-region bookkeeping and the octave-window sums live in the lane
//! state; the kernel closes windows itself and writes one [`Rec`] per window to a per-lane array, so
//! slices need not align with window boundaries. Scheduling (lane refill between slices, results by job
//! index, tile-pool exhaustion requeues the job and halves the active-lane cap) is the sweeper's.

use super::{GpuError, MAX_COLOURS, TILE_DIM, err};
use crate::probe::{ProbeResult, WindowSums, finish_window, window_sums_exact};
use crate::{Rule, Turn};
use cudarc::driver::{CudaContext, CudaFunction, CudaSlice, CudaStream, DeviceRepr, LaunchConfig, PushKernelArg, ValidAsZeroBits};
use cudarc::nvrtc::{CompileOptions, compile_ptx_with_opts};
use std::collections::VecDeque;
use std::sync::Arc;

const KERNELS: &str = include_str!("probe.cu");
const TILE_CELLS: usize = TILE_DIM * TILE_DIM;
const TILE_BYTES: usize = TILE_CELLS * 2;
/// Radius of the random patch of `PatchedAnt` as used by `bin/probe` (`RULE@SEED`).
pub const PATCH_RADIUS: i64 = 2;

const ST_BUDGET: u32 = 1;
const ST_ESCAPED: u32 = 2;
const ST_OOM: u32 = 3;
const ST_IDLE: u32 = 255;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct PLane {
    steps: u64,
    budget: u64,
    seed: u64,
    job: u32,
    status: u32,
    x: i32,
    y: i32,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    h: u32,
    k: u32,
    started: u32,
    n: u32,
    turn: [u32; 4],
    has_patch: u32,
    pad: u32,
    area: u64,
    perimeter: u64,
    sx: i64,
    sy: i64,
    sxx: i64,
    newc: u64,
    bsteps: u64,
    fresh: u64,
    ax: i64,
    ay: i64,
    axx: u64,
}
// SAFETY: plain-old-data mirrored in probe.cu (`struct PLane`); all-zero is a valid value.
unsafe impl DeviceRepr for PLane {}
unsafe impl ValidAsZeroBits for PLane {}

/// One closed window as the kernel writes it.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct Rec {
    area: u64,
    perimeter: u64,
    newc: u64,
    bsteps: u64,
    fresh: u64,
    sx: i64,
    sy: i64,
    sxx: i64,
    ax: i64,
    ay: i64,
    axx: u64,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}
unsafe impl DeviceRepr for Rec {}
unsafe impl ValidAsZeroBits for Rec {}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct Assign {
    budget: u64,
    seed: u64,
    lane: u32,
    job: u32,
    turn: [u32; 4],
    n: u32,
    has_patch: u32,
}
unsafe impl DeviceRepr for Assign {}
unsafe impl ValidAsZeroBits for Assign {}

const _: () = assert!(std::mem::size_of::<PLane>() == 184);
const _: () = assert!(std::mem::size_of::<Rec>() == 104);
const _: () = assert!(std::mem::size_of::<Assign>() == 48);

/// One probe run: the ant on a blank board (`patch_seed: None`) or with the random 5x5 patch of
/// `PatchedAnt::new(rule, seed, 2)` at the start. Runs 2^max_log2 steps.
#[derive(Clone, Debug)]
pub struct ProbeJob {
    pub rule: Rule,
    pub patch_seed: Option<u64>,
    pub max_log2: u32,
}

#[derive(Clone, Debug)]
pub struct ProbeConfig {
    pub device: usize,
    /// Ants resident at once.
    pub lanes: usize,
    /// Board side, as in `probe::probe(.., size, ..)` (the ant stops at |x| >= size/2 - 2).
    pub grid: usize,
    /// Tile pool size in 8 KiB tiles. `None`: all free VRAM minus a margin.
    pub pool_tiles: Option<usize>,
    /// Steps per kernel launch.
    pub slice_steps: u64,
    /// Windows with k >= min_log2 are returned (`bin/probe` uses 8). At least 1.
    pub min_log2: u32,
    /// Largest `max_log2` any job may have (sizes the per-lane window records).
    pub max_log2_cap: u32,
}

impl Default for ProbeConfig {
    fn default() -> ProbeConfig {
        ProbeConfig { device: 0, lanes: 4096, grid: 8192, pool_tiles: None, slice_steps: 1 << 20, min_log2: 8, max_log2_cap: 32 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProbeStats {
    pub jobs: usize,
    pub total_steps: u64,
    pub launches: usize,
    pub oom_requeues: usize,
    pub wall_secs: f64,
}

pub struct GpuProber {
    pub cfg: ProbeConfig,
    pub device_name: String,
    pub pool_tiles: usize,
    pub stats: ProbeStats,
    stream: Arc<CudaStream>,
    f_step: CudaFunction,
    f_retire: CudaFunction,
    f_assign: CudaFunction,
    dir_w: usize,
    lanes_dev: CudaSlice<PLane>,
    dirs: CudaSlice<u32>,
    pool: CudaSlice<u16>,
    freelist: CudaSlice<u32>,
    free_count: CudaSlice<i32>,
    recs: CudaSlice<Rec>,
    assign_dev: CudaSlice<Assign>,
    ids_dev: CudaSlice<u32>,
}

const STAGE_LANES: usize = 1024;

impl GpuProber {
    pub fn new(cfg: ProbeConfig) -> Result<GpuProber, GpuError> {
        if cfg.lanes == 0 || cfg.grid < 16 || cfg.slice_steps == 0 || cfg.min_log2 == 0 || cfg.max_log2_cap == 0 || cfg.max_log2_cap > 40 {
            return Err(GpuError("lanes, slice_steps, min_log2, max_log2_cap must be positive (cap <= 40), grid >= 16".into()));
        }
        let ctx = CudaContext::new(cfg.device).map_err(err("open device"))?;
        let device_name = ctx.name().map_err(err("device name"))?;
        let ptx = compile_ptx_with_opts(KERNELS, CompileOptions { arch: Some("sm_89"), ..Default::default() }).map_err(err("nvrtc compile"))?;
        let module = ctx.load_module(ptx).map_err(err("load module"))?;
        let get = |n: &str| module.load_function(n).map_err(err(n));
        let (f_step, f_retire, f_assign, f_init) = (get("pstep_kernel")?, get("pretire_kernel")?, get("passign_kernel")?, get("pinit_freelist")?);
        let stream = ctx.default_stream();

        let dir_w = cfg.grid.div_ceil(TILE_DIM);
        let lanes = cfg.lanes;
        let max_win = cfg.max_log2_cap as usize;
        let fixed_bytes = lanes * (dir_w * dir_w * 4 + std::mem::size_of::<PLane>() + 48 + 4 + max_win * std::mem::size_of::<Rec>());
        let (free, _total) = ctx.mem_get_info().map_err(err("mem_get_info"))?;
        let budget = free.saturating_sub(fixed_bytes + (256 << 20));
        let pool_tiles = cfg.pool_tiles.unwrap_or(budget / (TILE_BYTES + 4));
        if pool_tiles == 0 || pool_tiles >= u32::MAX as usize {
            return Err(GpuError(format!(
                "no room for a tile pool ({} MiB free, {} MiB needed for {lanes} lanes): lower lanes",
                free >> 20,
                fixed_bytes >> 20
            )));
        }
        let lanes_dev = {
            let mut l = stream.alloc_zeros::<PLane>(lanes).map_err(err("alloc lanes"))?;
            let idle = vec![PLane { status: ST_IDLE, ..PLane::default() }; lanes];
            stream.memcpy_htod(&idle, &mut l).map_err(err("init lanes"))?;
            l
        };
        let dirs = stream.alloc_zeros::<u32>(lanes * dir_w * dir_w).map_err(err("alloc dirs"))?;
        let pool = stream.alloc_zeros::<u16>(pool_tiles * TILE_CELLS).map_err(err("alloc pool"))?;
        let mut freelist = stream.alloc_zeros::<u32>(pool_tiles).map_err(err("alloc freelist"))?;
        let mut free_count = stream.alloc_zeros::<i32>(1).map_err(err("alloc free_count"))?;
        stream.memcpy_htod(&[pool_tiles as i32], &mut free_count).map_err(err("init free_count"))?;
        let recs = stream.alloc_zeros::<Rec>(lanes * max_win).map_err(err("alloc recs"))?;
        let assign_dev = stream.alloc_zeros::<Assign>(lanes).map_err(err("alloc assign"))?;
        let ids_dev = stream.alloc_zeros::<u32>(STAGE_LANES.min(lanes)).map_err(err("alloc ids"))?;
        let n = pool_tiles as u32;
        let mut b = stream.launch_builder(&f_init);
        b.arg(&mut freelist).arg(&n);
        // SAFETY: matches `pinit_freelist(unsigned*, unsigned)`.
        unsafe { b.launch(LaunchConfig::for_num_elems(n)) }.map_err(err("init freelist"))?;
        stream.synchronize().map_err(err("sync"))?;
        Ok(GpuProber {
            cfg,
            device_name,
            pool_tiles,
            stats: ProbeStats::default(),
            stream,
            f_step,
            f_retire,
            f_assign,
            dir_w,
            lanes_dev,
            dirs,
            pool,
            freelist,
            free_count,
            recs,
            assign_dev,
            ids_dev,
        })
    }

    /// Run every job; `results[i]` belongs to `jobs[i]`. `progress` gets the number of finished jobs.
    pub fn run(&mut self, jobs: &[ProbeJob], mut progress: impl FnMut(usize)) -> Result<Vec<ProbeResult>, GpuError> {
        let size = self.cfg.grid;
        for (i, j) in jobs.iter().enumerate() {
            let n = j.rule.len();
            if !(1..=MAX_COLOURS).contains(&n) {
                return Err(GpuError(format!("job {i}: {n} colours (supported: 1..={MAX_COLOURS})")));
            }
            if j.max_log2 == 0 || j.max_log2 > self.cfg.max_log2_cap {
                return Err(GpuError(format!("job {i}: max_log2 {} outside 1..={}", j.max_log2, self.cfg.max_log2_cap)));
            }
            if !window_sums_exact(size, j.max_log2) {
                return Err(GpuError(format!("job {i}: window sums could exceed 2^53 on grid {size} at 2^{} steps (f64 would no longer be exact)", j.max_log2)));
            }
        }
        let t0 = std::time::Instant::now();
        let lanes = self.cfg.lanes;
        let min_log2 = self.cfg.min_log2;
        let max_win = self.cfg.max_log2_cap as usize;
        let mut results: Vec<Option<ProbeResult>> = vec![None; jobs.len()];
        let mut queue: VecDeque<usize> = (0..jobs.len()).collect();
        let mut lane_job: Vec<Option<usize>> = vec![None; lanes];
        let mut free_lanes: Vec<usize> = (0..lanes).rev().collect();
        let (mut active, mut max_active, mut done) = (0usize, lanes, 0usize);
        let mut host_lanes = vec![PLane::default(); lanes];
        let mut stats = ProbeStats { jobs: jobs.len(), ..ProbeStats::default() };

        loop {
            let mut recs_in: Vec<Assign> = Vec::new();
            while active < max_active {
                let Some(&job) = queue.front() else { break };
                let Some(lane) = free_lanes.pop() else { break };
                queue.pop_front();
                let j = &jobs[job];
                let mut turn = [0u32; 4];
                for (k, t) in j.rule.0.iter().enumerate() {
                    if *t == Turn::R {
                        turn[k / 32] |= 1 << (k % 32);
                    }
                }
                recs_in.push(Assign {
                    budget: 1u64 << j.max_log2,
                    seed: j.patch_seed.unwrap_or(0),
                    lane: lane as u32,
                    job: job as u32,
                    turn,
                    n: j.rule.len() as u32,
                    has_patch: j.patch_seed.is_some() as u32,
                });
                lane_job[lane] = Some(job);
                active += 1;
            }
            if !recs_in.is_empty() {
                let mut dst = self.assign_dev.slice_mut(0..recs_in.len());
                self.stream.memcpy_htod(&recs_in, &mut dst).map_err(err("upload jobs"))?;
                let (k, sz) = (recs_in.len() as i32, size as u32);
                let mut b = self.stream.launch_builder(&self.f_assign);
                b.arg(&mut self.lanes_dev).arg(&self.assign_dev).arg(&k).arg(&sz);
                // SAFETY: matches `passign_kernel(PLane*, const Assign*, int, unsigned)`.
                unsafe { b.launch(LaunchConfig::for_num_elems(k as u32)) }.map_err(err("assign"))?;
            }
            if active == 0 {
                break;
            }
            let active_at_launch = active;
            {
                let (nl, sz, dw, mw, slice) = (lanes as i32, size as u32, self.dir_w as u32, max_win as u32, self.cfg.slice_steps);
                let mut b = self.stream.launch_builder(&self.f_step);
                b.arg(&mut self.lanes_dev)
                    .arg(&nl)
                    .arg(&mut self.dirs)
                    .arg(&mut self.pool)
                    .arg(&self.freelist)
                    .arg(&mut self.free_count)
                    .arg(&mut self.recs)
                    .arg(&sz)
                    .arg(&dw)
                    .arg(&mw)
                    .arg(&slice);
                let cfg = LaunchConfig { grid_dim: ((lanes as u32).div_ceil(128), 1, 1), block_dim: (128, 1, 1), shared_mem_bytes: 0 };
                // SAFETY: matches `pstep_kernel(...)` in probe.cu argument for argument.
                unsafe { b.launch(cfg) }.map_err(err("step"))?;
                stats.launches += 1;
            }
            self.stream.memcpy_dtoh(&self.lanes_dev, &mut host_lanes).map_err(err("download lanes"))?;
            let finished: Vec<usize> =
                (0..lanes).filter(|&l| lane_job[l].is_some() && (ST_BUDGET..=ST_OOM).contains(&host_lanes[l].status)).collect();
            if finished.is_empty() {
                continue;
            }
            let mut oom_jobs = Vec::new();
            for chunk in finished.chunks(STAGE_LANES.min(lanes)) {
                for &l in chunk {
                    let job = lane_job[l].take().expect("finished lane has a job");
                    let lane = host_lanes[l];
                    free_lanes.push(l);
                    active -= 1;
                    if lane.status == ST_OOM {
                        oom_jobs.push(job);
                        continue;
                    }
                    let mut buf = vec![Rec::default(); lane.k as usize];
                    if !buf.is_empty() {
                        let src = self.recs.slice(l * max_win..l * max_win + buf.len());
                        self.stream.memcpy_dtoh(&src, &mut buf).map_err(err("download records"))?;
                    }
                    results[job] = Some(to_result(&lane, &buf, min_log2));
                    done += 1;
                }
                let ids: Vec<u32> = chunk.iter().map(|&l| l as u32).collect();
                let mut dst = self.ids_dev.slice_mut(0..ids.len());
                self.stream.memcpy_htod(&ids, &mut dst).map_err(err("upload ids"))?;
                let de = (self.dir_w * self.dir_w) as u32;
                let mut b = self.stream.launch_builder(&self.f_retire);
                b.arg(&mut self.lanes_dev)
                    .arg(&self.ids_dev)
                    .arg(&mut self.dirs)
                    .arg(&mut self.pool)
                    .arg(&mut self.freelist)
                    .arg(&mut self.free_count)
                    .arg(&de);
                let cfg = LaunchConfig { grid_dim: (ids.len() as u32, 1, 1), block_dim: (256, 1, 1), shared_mem_bytes: 0 };
                // SAFETY: matches `pretire_kernel(...)` in probe.cu.
                unsafe { b.launch(cfg) }.map_err(err("retire"))?;
            }
            progress(done);
            if !oom_jobs.is_empty() {
                if active_at_launch == 1 {
                    return Err(GpuError(format!(
                        "job {} ({}) needs more than the whole tile pool ({} tiles = {} MiB); raise pool_tiles",
                        oom_jobs[0],
                        jobs[oom_jobs[0]].rule,
                        self.pool_tiles,
                        self.pool_tiles * TILE_BYTES >> 20
                    )));
                }
                stats.oom_requeues += oom_jobs.len();
                max_active = (active_at_launch / 2).max(1);
                for j in oom_jobs.into_iter().rev() {
                    queue.push_front(j);
                }
            }
        }
        self.stream.synchronize().map_err(err("sync"))?;
        stats.wall_secs = t0.elapsed().as_secs_f64();
        let out: Vec<ProbeResult> = results.into_iter().map(|r| r.expect("every job produced a result")).collect();
        stats.total_steps = jobs.iter().zip(&out).map(|(j, r)| if r.escaped { 0 } else { 1u64 << j.max_log2 }).sum();
        self.stats = stats;
        Ok(out)
    }
}

/// Windows k in min_log2..closed, built with the same function as the CPU probe.
fn to_result(lane: &PLane, recs: &[Rec], min_log2: u32) -> ProbeResult {
    let windows = recs
        .iter()
        .enumerate()
        .filter(|(k, _)| *k as u32 >= min_log2)
        .map(|(k, r)| {
            finish_window(&WindowSums {
                k: k as u32,
                area: r.area,
                perimeter: r.perimeter,
                new_cells: r.newc,
                boundary_steps: r.bsteps,
                fresh_nbrs: r.fresh,
                sx: r.sx,
                sy: r.sy,
                sxx: r.sxx,
                ax: r.ax as f64,
                ay: r.ay as f64,
                axx: r.axx as f64,
                min: (r.minx as i64, r.miny as i64),
                max: (r.maxx as i64, r.maxy as i64),
            })
        })
        .collect();
    ProbeResult { windows, escaped: lane.status == ST_ESCAPED }
}
