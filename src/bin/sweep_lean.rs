//! As `sweep`, but with the lean ant (src/lean.rs, identical results, about 2x faster).
//! usage: sweep_lean MIN MAX MAX_STEPS [GRID]   or   sweep_lean - - MAX_STEPS [GRID]  (rules on stdin)
//! usage: sweep MIN MAX MAX_STEPS [GRID]
//!    or: sweep - - MAX_STEPS [GRID]   (rules, one per line, on stdin)

use rayon::prelude::*;
use std::io::Write;
use turmite::Rule;
use turmite::fast::{self, Config, Stop};
use turmite::lean::{Ckpt, LeanGrid, highway_onset_flat, run_lean_ckpt};

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
    let done = std::sync::atomic::AtomicUsize::new(0);
    let rows: Vec<String> = rules
        .par_iter()
        .map_init(
            || LeanGrid::new(grid_size),
            |grid, rule| {
                let ck = ckpt_dir.as_ref().map(|d| Ckpt { path: format!("{d}/{rule}_{grid_size}.ckpt").into(), every: ckpt_every });
                let res = run_lean_ckpt(rule, grid, &cfg, ck.as_ref());
                let (period, dx, dy, onset) = match res.highway {
                    Some(h) => (
                        h.period.to_string(),
                        h.dx.to_string(),
                        h.dy.to_string(),
                        highway_onset_flat(rule, h, res.steps, grid_size).to_string(),
                    ),
                    None => Default::default(),
                };
                let stop = match res.stop {
                    Stop::Budget => "budget",
                    Stop::Escaped => "escaped",
                };
                let tail = res.periodic_tail.map(|h| h.period.to_string()).unwrap_or_default();
                let k = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if k % 500 == 0 {
                    eprintln!("{k}");
                }
                format!(
                    "{rule},{},{stop},{},{period},{dx},{dy},{onset},{},{},{tail}",
                    rule.len(),
                    res.steps,
                    res.extent.0,
                    res.extent.1
                )
            },
        )
        .collect();

    let mut out = std::io::stdout().lock();
    writeln!(out, "rule,n,stop,steps,period,dx,dy,onset,width,height,tail_period").unwrap();
    for r in rows {
        writeln!(out, "{r}").unwrap();
    }
}
