//! `bin/probe` on a CUDA device, for plain rules and patched ants only.
//!
//! usage: gpu_probe MAX_LOG2 [GRID] [flags] < specs > probe.csv
//! Each input line is `RULE` or `RULE@SEED` (the ant with a random 5x5 patch at the start). Other
//! walkers of `probe` (rotor:, rw:, sbg:, ...) are not ported and are rejected before anything runs.
//! The CSV is byte-identical to `probe MAX_LOG2 GRID` on the same lines (`tests/gpu_probe.rs`).
//!
//! flags:
//!   --lanes N     resident ants (default 4096; all of them need tile-pool room, see the README)
//!   --slice N     steps per kernel launch (default 1048576)
//!   --pool-gb G   tile pool size in GiB (default: all free VRAM minus a margin)
//!   --device K    CUDA device ordinal (default 0)
//!   --stats FILE  write a one-line JSON summary
//!
//! Needs `--features cuda` and `libnvrtc.so.12` on LD_LIBRARY_PATH.

use std::io::Write;
use turmite::Rule;
use turmite::gpu::{GpuProber, ProbeConfig, ProbeJob};
use turmite::probe::{CSV_HEADER, csv_rows};

fn main() {
    let mut pos: Vec<String> = Vec::new();
    let mut cfg = ProbeConfig::default();
    let mut pool_gb: Option<f64> = None;
    let mut stats_path: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = |name: &str| args.next().unwrap_or_else(|| panic!("{name} needs a value"));
        match a.as_str() {
            "--lanes" => cfg.lanes = val("--lanes").parse().unwrap(),
            "--slice" => cfg.slice_steps = val("--slice").parse().unwrap(),
            "--pool-gb" => pool_gb = Some(val("--pool-gb").parse().unwrap()),
            "--device" => cfg.device = val("--device").parse().unwrap(),
            "--stats" => stats_path = Some(val("--stats")),
            _ => pos.push(a),
        }
    }
    let max_log2: u32 = pos.first().expect("usage: gpu_probe MAX_LOG2 [GRID] < specs").parse().unwrap();
    cfg.grid = pos.get(1).map(|s| s.parse().unwrap()).unwrap_or(8192);
    cfg.max_log2_cap = max_log2;
    cfg.min_log2 = 8; // as hard-coded in bin/probe
    if let Some(g) = pool_gb {
        cfg.pool_tiles = Some((g * (1u64 << 30) as f64 / 8192.0) as usize);
    }
    let specs: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
    let jobs: Vec<ProbeJob> = specs
        .iter()
        .map(|s| {
            let (rule, seed) = match s.split_once('@') {
                Some((r, seed)) => (r, Some(seed.parse::<u64>().unwrap_or_else(|_| panic!("bad seed in spec {s:?}")))),
                None => (s.as_str(), None),
            };
            let rule: Rule = rule.parse().unwrap_or_else(|_| panic!("spec {s:?} is not RULE or RULE@SEED (other walkers are CPU-only: use `probe`)"));
            ProbeJob { rule, patch_seed: seed, max_log2 }
        })
        .collect();
    eprintln!("{} jobs", jobs.len());
    let t = std::time::Instant::now();
    let mut prober = GpuProber::new(cfg.clone()).unwrap_or_else(|e| panic!("{e}"));
    eprintln!(
        "device {} | {} lanes | grid {} | pool {} tiles ({} MiB) | slice {} | setup {:.2}s",
        prober.device_name,
        cfg.lanes,
        cfg.grid,
        prober.pool_tiles,
        prober.pool_tiles * 8192 >> 20,
        cfg.slice_steps,
        t.elapsed().as_secs_f64()
    );
    let total = jobs.len();
    let mut last = 0usize;
    let res = prober
        .run(&jobs, |done| {
            if done / 200 > last / 200 || done == total {
                eprintln!("{done}");
            }
            last = done;
        })
        .unwrap_or_else(|e| panic!("{e}"));
    let st = prober.stats.clone();
    eprintln!(
        "{} steps in {:.2}s = {:.3e} steps/s ({} launches, {} pool requeues)",
        st.total_steps,
        st.wall_secs,
        st.total_steps as f64 / st.wall_secs,
        st.launches,
        st.oom_requeues
    );
    let mut out = std::io::stdout().lock();
    writeln!(out, "{CSV_HEADER}").unwrap();
    for (s, r) in specs.iter().zip(&res) {
        out.write_all(csv_rows(s, r).as_bytes()).unwrap();
    }
    if let Some(p) = stats_path {
        let line = format!(
            "{{\"jobs\":{},\"total_steps\":{},\"wall_secs\":{:.4},\"steps_per_sec\":{:.4e},\"device\":\"{}\",\"lanes\":{},\"grid\":{},\"slice\":{},\"max_log2\":{max_log2},\"pool_tiles\":{},\"launches\":{},\"oom_requeues\":{}}}\n",
            st.jobs,
            st.total_steps,
            st.wall_secs,
            st.total_steps as f64 / st.wall_secs,
            prober.device_name,
            cfg.lanes,
            cfg.grid,
            cfg.slice_steps,
            prober.pool_tiles,
            st.launches,
            st.oom_requeues
        );
        std::fs::write(p, line).unwrap();
    }
}
