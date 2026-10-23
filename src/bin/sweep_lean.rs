//! As `sweep`, but with the lean ant (src/lean.rs, identical results, about 2x faster).
//! usage: sweep_lean MIN MAX MAX_STEPS [GRID]   or   sweep_lean - - MAX_STEPS [GRID]  (rules on stdin)
//! usage: sweep MIN MAX MAX_STEPS [GRID]
//!    or: sweep - - MAX_STEPS [GRID]   (rules, one per line, on stdin)
//!
//! Optional environment (the CSV on stdout is byte-identical whatever is set; see src/tiled.rs):
//!   TURMITE_CKPT_DIR=dir [TURMITE_CKPT_EVERY=steps]   mid-trajectory checkpoints (dense: `.ckpt`, tiled: `.tkpt`)
//!   TURMITE_GRID=dense|tiled                           storage backend, default dense
//!   TURMITE_TILE_LOG2=k                                tiled: tile side 2^k, default 6
//!   TURMITE_CACHE_TILES=N|unlimited                    tiled: resident-tile bound (N >= 2), default unlimited (sparse only)
//!   TURMITE_INACTIVE=raw|packed                        tiled: how non-resident tiles are stored, default packed
//!   TURMITE_GRID_STATS=file.json                       backend configuration, cache counters, memory, peak RSS (never in the CSV)

use rayon::prelude::*;
use std::io::Write;
use turmite::Rule;
use turmite::fast::{Config, RunResult, Stop};
use turmite::lean::{Ckpt, LeanGrid, highway_onset_flat, run_lean_ckpt};
use turmite::tiled::{Inactive, MemoryReport, TileConfig, TileStats, TiledGrid, highway_onset_tiled, run_tiled_ckpt};

enum Worker {
    Dense(LeanGrid),
    Tiled(TiledGrid),
}

struct RuleStats {
    rule: String,
    run: TileStats,
    mem: MemoryReport,
    onset_peak_total: Option<usize>,
}

fn env_or_die<T>(name: &str, default: T, parse: impl Fn(&str) -> Option<T>, expect: &str) -> T {
    match std::env::var(name) {
        Err(_) => default,
        Ok(v) => parse(&v).unwrap_or_else(|| {
            eprintln!("{name}={v:?} is not valid: expected {expect}");
            std::process::exit(2)
        }),
    }
}

fn peak_rss_kb() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/self/status").ok()?;
    s.lines().find_map(|l| l.strip_prefix("VmHWM:")).and_then(|v| v.trim().trim_end_matches("kB").trim().parse().ok())
}

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let max_steps: u64 = raw[2].parse().unwrap();
    let grid_size: usize = raw.get(3).map(|g| g.parse().unwrap()).unwrap_or(8192);

    let rules: Vec<Rule> = if raw[0] == "-" {
        std::io::stdin()
            .lines()
            .map(|l| l.unwrap().trim().parse().unwrap())
            .collect()
    } else {
        let (lo, hi): (usize, usize) = (raw[0].parse().unwrap(), raw[1].parse().unwrap());
        (lo..=hi)
            .flat_map(|n| (0..1u32 << n).map(move |b| Rule::from_bits(b, n)))
            .filter(|r| r.is_canonical() && !r.is_trivial())
            .collect()
    };
    eprintln!("{} rules", rules.len());

    let cfg = Config {
        max_steps,
        ..Config::default()
    };
    // Optional mid-trajectory checkpoints: TURMITE_CKPT_DIR=dir [TURMITE_CKPT_EVERY=steps, default 1e9]. A rerun resumes,
    // and a larger MAX_STEPS extends finished runs instead of restarting them (same CSV as an uninterrupted run).
    let ckpt_dir = std::env::var("TURMITE_CKPT_DIR").ok();
    let ckpt_every: u64 = std::env::var("TURMITE_CKPT_EVERY").ok().map_or(1_000_000_000, |v| v.parse().unwrap());
    if let Some(d) = &ckpt_dir {
        std::fs::create_dir_all(d).unwrap();
    }

    let tiled: bool = env_or_die("TURMITE_GRID", false, |v| match v { "dense" => Some(false), "tiled" => Some(true), _ => None }, "dense or tiled");
    let tile_cfg = TileConfig {
        tile_log2: env_or_die("TURMITE_TILE_LOG2", 6, |v| v.parse().ok(), "an integer"),
        cache_tiles: env_or_die("TURMITE_CACHE_TILES", None, |v| if v == "unlimited" { Some(None) } else { v.parse().ok().map(Some) }, "N or unlimited"),
        inactive: env_or_die("TURMITE_INACTIVE", Inactive::Packed, |v| match v { "raw" => Some(Inactive::Raw), "packed" => Some(Inactive::Packed), _ => None }, "raw or packed"),
    };
    let stats_path = std::env::var("TURMITE_GRID_STATS").ok();
    if tiled {
        if let Err(e) = TiledGrid::new(grid_size, tile_cfg) {
            eprintln!("{e}");
            std::process::exit(2);
        }
    } else if ["TURMITE_TILE_LOG2", "TURMITE_CACHE_TILES", "TURMITE_INACTIVE"].iter().any(|k| std::env::var_os(k).is_some()) {
        eprintln!("TURMITE_TILE_LOG2, TURMITE_CACHE_TILES and TURMITE_INACTIVE need TURMITE_GRID=tiled");
        std::process::exit(2);
    }

    let t0 = std::time::Instant::now();
    let done = std::sync::atomic::AtomicUsize::new(0);
    let rows: Vec<(String, Option<RuleStats>)> = rules
        .par_iter()
        .map_init(
            || if tiled { Worker::Tiled(TiledGrid::new(grid_size, tile_cfg).unwrap()) } else { Worker::Dense(LeanGrid::new(grid_size)) },
            |worker, rule| {
                let (res, onset, stat): (RunResult, Option<u64>, Option<RuleStats>) = match worker {
                    Worker::Dense(grid) => {
                        let ck = ckpt_dir.as_ref().map(|d| Ckpt { path: format!("{d}/{rule}_{grid_size}.ckpt").into(), every: ckpt_every });
                        let res = run_lean_ckpt(rule, grid, &cfg, ck.as_ref());
                        let onset = res.highway.map(|h| highway_onset_flat(rule, h, res.steps, grid_size));
                        (res, onset, None)
                    }
                    Worker::Tiled(grid) => {
                        let ck = ckpt_dir.as_ref().map(|d| Ckpt {
                            path: format!("{d}/{rule}_{grid_size}_t{}.tkpt", tile_cfg.tile_log2).into(),
                            every: ckpt_every,
                        });
                        let res = run_tiled_ckpt(rule, grid, &cfg, ck.as_ref()).unwrap_or_else(|e| {
                            eprintln!("rule {rule}: {e}");
                            std::process::exit(1)
                        });
                        let (run, mem) = (grid.stats().clone(), grid.memory());
                        let onset = res.highway.map(|h| highway_onset_tiled(rule, h, res.steps, grid));
                        let onset_peak_total = onset.map(|_| grid.stats().peak_total_bytes);
                        (res, onset, Some(RuleStats { rule: rule.to_string(), run, mem, onset_peak_total }))
                    }
                };
                let (period, dx, dy) = match res.highway {
                    Some(h) => (h.period.to_string(), h.dx.to_string(), h.dy.to_string()),
                    None => Default::default(),
                };
                let onset = onset.map(|o| o.to_string()).unwrap_or_default();
                let stop = match res.stop {
                    Stop::Budget => "budget",
                    Stop::Escaped => "escaped",
                };
                let tail = res.periodic_tail.map(|h| h.period.to_string()).unwrap_or_default();
                let k = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if k % 500 == 0 {
                    eprintln!("{k}");
                }
                (
                    format!(
                        "{rule},{},{stop},{},{period},{dx},{dy},{onset},{},{},{tail}",
                        rule.len(),
                        res.steps,
                        res.extent.0,
                        res.extent.1
                    ),
                    stat,
                )
            },
        )
        .collect();
    let wall = t0.elapsed().as_secs_f64();

    let mut out = std::io::stdout().lock();
    writeln!(out, "rule,n,stop,steps,period,dx,dy,onset,width,height,tail_period").unwrap();
    for (r, _) in &rows {
        writeln!(out, "{r}").unwrap();
    }

    if let Some(path) = stats_path {
        let threads = rayon::current_num_threads();
        let null = || "null".to_string();
        let mut j = format!(
            "{{\n \"backend\": \"{}\", \"grid\": {grid_size}, \"max_steps\": {max_steps}, \"rules\": {}, \"threads\": {threads}, \"wall_s\": {wall:.3}, \"peak_rss_kb\": {},\n",
            if tiled { "tiled" } else { "dense" },
            rules.len(),
            peak_rss_kb().map_or_else(null, |v| v.to_string()),
        );
        j.push_str(&format!(" \"dense_logical_bytes_per_worker\": {},\n", (grid_size + 2) * (grid_size + 2)));
        if tiled {
            j.push_str(&format!(
                " \"tile_log2\": {}, \"cache_tiles\": {}, \"inactive\": \"{:?}\",\n \"per_rule\": [\n",
                tile_cfg.tile_log2,
                tile_cfg.cache_tiles.map_or_else(null, |c| c.to_string()),
                tile_cfg.inactive
            ));
            let items: Vec<String> = rows
                .iter()
                .filter_map(|(_, s)| s.as_ref())
                .map(|s| {
                    format!(
                        "  {{\"rule\": \"{}\", \"transitions\": {}, \"hits\": {}, \"misses\": {}, \"creations\": {}, \"evictions\": {}, \"blank_drops\": {}, \
                         \"packed_stores\": {}, \"raw_stores\": {}, \"bytes_encoded\": {}, \"bytes_decoded\": {}, \"peak_resident_tiles\": {}, \"peak_inactive_bytes\": {}, \
                         \"peak_total_bytes\": {}, \"end_tiles\": {}, \"end_resident_tiles\": {}, \"end_packed_bytes\": {}, \"end_raw_inactive_bytes\": {}, \
                         \"end_resident_bytes\": {}, \"end_directory_bytes\": {}, \"end_cache_meta_bytes\": {}, \"onset_peak_total_bytes\": {}}}",
                        s.rule, s.run.transitions, s.run.hits, s.run.misses, s.run.creations, s.run.evictions, s.run.blank_drops,
                        s.run.packed_stores, s.run.raw_stores, s.run.bytes_encoded, s.run.bytes_decoded, s.run.peak_resident_tiles,
                        s.run.peak_inactive_bytes, s.run.peak_total_bytes, s.mem.tiles_total, s.mem.tiles_resident, s.mem.packed_bytes,
                        s.mem.raw_inactive_bytes, s.mem.resident_bytes, s.mem.directory_bytes, s.mem.cache_meta_bytes,
                        s.onset_peak_total.map_or_else(null, |v| v.to_string())
                    )
                })
                .collect();
            j.push_str(&items.join(",\n"));
            j.push_str("\n ]\n");
        } else {
            j.push_str(" \"per_rule\": []\n");
        }
        j.push_str("}\n");
        std::fs::write(&path, j).unwrap();
    }
}
