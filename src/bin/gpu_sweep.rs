//! GPU census sweep: `sweep_lean` on a CUDA device (src/gpu). Same positional arguments, same CSV.
//!
//! usage: gpu_sweep MIN MAX MAX_STEPS [GRID] [flags]
//!    or: gpu_sweep - - MAX_STEPS [GRID] [flags]     (jobs on stdin, one per line: `RULE` or `RULE SEED`)
//!
//! flags:
//!   --bg-seeds A..B   cross every rule with background seeds A..=B (adds a `bg_seed` column;
//!                     seeded runs have no `onset`, which is only defined for the blank background)
//!   --lanes N         resident ants (default 8192)
//!   --slice N         steps per kernel launch (default 1048576)
//!   --pool-gb G       tile pool size in GiB (default: all free VRAM minus a margin)
//!   --device K        CUDA device ordinal (default 0; the name is printed on stderr)
//!   --stats FILE      write a one-line JSON summary (jobs, steps, wall seconds, steps/s, device, lanes)
//!
//! With no seeds the output is byte-identical to `sweep_lean` for the same arguments.
//! Needs `--features cuda` and `libnvrtc.so.12` on LD_LIBRARY_PATH.

use rayon::prelude::*;
use std::io::Write;
use turmite::Rule;
use turmite::fast::{Config, Stop};
use turmite::gpu::{GpuConfig, GpuSweeper, Job};
use turmite::onset::{OnsetScratch, highway_onset_reusing};

fn main() {
    let mut pos: Vec<String> = Vec::new();
    let mut gpu = GpuConfig::default();
    let mut seeds: Option<(u64, u64)> = None;
    let mut pool_gb: Option<f64> = None;
    let mut stats_path: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = |name: &str| args.next().unwrap_or_else(|| panic!("{name} needs a value"));
        match a.as_str() {
            "--bg-seeds" => {
                let v = val("--bg-seeds");
                let (lo, hi) = v.split_once("..").expect("--bg-seeds A..B");
                seeds = Some((lo.parse().unwrap(), hi.parse().unwrap()));
            }
            "--lanes" => gpu.lanes = val("--lanes").parse().unwrap(),
            "--slice" => gpu.slice_steps = val("--slice").parse().unwrap(),
            "--pool-gb" => pool_gb = Some(val("--pool-gb").parse().unwrap()),
            "--device" => gpu.device = val("--device").parse().unwrap(),
            "--stats" => stats_path = Some(val("--stats")),
            _ => pos.push(a),
        }
    }
    let max_steps: u64 = pos[2].parse().unwrap();
    gpu.grid = pos.get(3).map(|g| g.parse().unwrap()).unwrap_or(8192);
    if let Some(g) = pool_gb {
        gpu.pool_tiles = Some((g * (1u64 << 30) as f64 / 4096.0) as usize);
    }
    let det = Config::default();
    gpu.history = det.history;
    gpu.max_period = det.max_period;

    // (rule, explicit seed from stdin)
    let rules: Vec<(Rule, Option<u64>)> = if pos[0] == "-" {
        std::io::stdin()
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let mut it = l.split_whitespace();
                let rule = it.next().unwrap().parse().unwrap();
                (rule, it.next().map(|s| s.parse().unwrap()))
            })
            .collect()
    } else {
        let (lo, hi): (usize, usize) = (pos[0].parse().unwrap(), pos[1].parse().unwrap());
        (lo..=hi)
            .flat_map(|n| (0..1u32 << n).map(move |b| Rule::from_bits(b, n)))
            .filter(|r| r.is_canonical() && !r.is_trivial())
            .map(|r| (r, None))
            .collect()
    };
    let jobs: Vec<Job> = rules
        .iter()
        .flat_map(|(r, s)| match (s, seeds) {
            (Some(s), _) => vec![(r, Some(*s))],
            (None, Some((a, b))) => (a..=b).map(|s| (r, Some(s))).collect(),
            (None, None) => vec![(r, None)],
        })
        .map(|(r, s)| Job { rule: r.clone(), bg_seed: s, max_steps })
        .collect();
    let seeded = jobs.iter().any(|j| j.bg_seed.is_some());
    eprintln!("{} jobs", jobs.len());

    let t_start = std::time::Instant::now();
    let mut sweeper = GpuSweeper::new(gpu.clone()).unwrap_or_else(|e| panic!("{e}"));
    eprintln!(
        "device {} | {} lanes | grid {} | pool {} tiles ({} MiB) | slice {}",
        sweeper.device_name,
        gpu.lanes,
        gpu.grid,
        sweeper.pool_tiles,
        sweeper.pool_tiles * 4096 >> 20,
        gpu.slice_steps
    );
    eprintln!("setup {:.2}s", t_start.elapsed().as_secs_f64());
    let total = jobs.len();
    let mut last = 0usize;
    let res = sweeper
        .run(&jobs, |done| {
            if done / 500 > last / 500 || done == total {
                eprintln!("{done}");
            }
            last = done;
        })
        .unwrap_or_else(|e| panic!("{e}"));
    let st = sweeper.stats.clone();
    eprintln!("host detection {:.2}s (partly overlapped), blocked on device {:.2}s", st.host_detect_secs, st.device_wait_secs);
    eprintln!(
        "{} steps in {:.2}s on the device path = {:.3e} steps/s ({} launches, {} pool requeues)",
        st.total_steps,
        st.wall_secs,
        st.total_steps as f64 / st.wall_secs,
        st.launches,
        st.oom_requeues
    );

    let t_onset = std::time::Instant::now();
    // Onset replays are CPU work (blank background only); done in parallel, results kept in job order.
    let onsets: Vec<String> = jobs
        .par_iter()
        .zip(res.par_iter())
        .map_init(
            || OnsetScratch::new(gpu.grid),
            |sc, (j, r)| match (r.highway, j.bg_seed) {
                (Some(h), None) => highway_onset_reusing(&j.rule, h, r.steps, sc).to_string(),
                _ => String::new(),
            },
        )
        .collect();

    eprintln!("onset replays {:.2}s", t_onset.elapsed().as_secs_f64());
    let mut out = std::io::stdout().lock();
    if seeded {
        writeln!(out, "rule,bg_seed,n,stop,steps,period,dx,dy,onset,width,height,tail_period").unwrap();
    } else {
        writeln!(out, "rule,n,stop,steps,period,dx,dy,onset,width,height,tail_period").unwrap();
    }
    for ((j, r), onset) in jobs.iter().zip(&res).zip(&onsets) {
        let (period, dx, dy) = match r.highway {
            Some(h) => (h.period.to_string(), h.dx.to_string(), h.dy.to_string()),
            None => Default::default(),
        };
        let stop = match r.stop {
            Stop::Budget => "budget",
            Stop::Escaped => "escaped",
        };
        let tail = r.periodic_tail.map(|h| h.period.to_string()).unwrap_or_default();
        let seed_col = if seeded { format!("{},", j.bg_seed.map(|s| s.to_string()).unwrap_or_default()) } else { String::new() };
        let onset = if r.highway.is_some() { onset.as_str() } else { "" };
        writeln!(
            out,
            "{},{seed_col}{},{stop},{},{period},{dx},{dy},{onset},{},{},{tail}",
            j.rule,
            j.rule.len(),
            r.steps,
            r.extent.0,
            r.extent.1
        )
        .unwrap();
    }
    if let Some(p) = stats_path {
        let line = format!(
            "{{\"jobs\":{},\"total_steps\":{},\"wall_secs\":{:.4},\"steps_per_sec\":{:.4e},\"device\":\"{}\",\"lanes\":{},\"grid\":{},\"slice\":{},\"max_steps\":{max_steps},\"pool_tiles\":{},\"launches\":{},\"oom_requeues\":{}}}\n",
            st.jobs,
            st.total_steps,
            st.wall_secs,
            st.total_steps as f64 / st.wall_secs,
            sweeper.device_name,
            gpu.lanes,
            gpu.grid,
            gpu.slice_steps,
            sweeper.pool_tiles,
            st.launches,
            st.oom_requeues
        );
        std::fs::write(p, line).unwrap();
    }
}
