//! Colour histograms of the visited region and of its boundary at octave checkpoints.
//! usage: colourstats MIN_LOG2 MAX_LOG2 [GRID] < rules.txt > colourstats.csv
//! One row per (rule, k): rule,k,area,boundary,hist_all,hist_boundary (histograms colon-separated).
//! Rule order in the output is input order (deterministic whatever the thread count).

use rayon::prelude::*;
use std::io::Write;
use turmite::Rule;
use turmite::colours::colour_hists;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (lo, hi): (u32, u32) = (a[1].parse().unwrap(), a[2].parse().unwrap());
    let size: usize = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(8192);
    let rules: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
    let cps: Vec<u64> = (lo..=hi).map(|k| 1u64 << k).collect();
    let join = |h: &[u64]| h.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(":");
    let blocks: Vec<String> = rules
        .par_iter()
        .map(|r| {
            let (snaps, escaped) = colour_hists(&r.parse::<Rule>().unwrap(), size, &cps);
            let mut s = String::new();
            for (k, sn) in (lo..).zip(&snaps) {
                s += &format!("{r},{k},{},{},{},{}\n", sn.area, sn.boundary, join(&sn.hist_all), join(&sn.hist_boundary));
            }
            if escaped {
                s += &format!("{r},escaped,,,,\n");
            }
            s
        })
        .collect();
    let mut out = std::io::stdout().lock();
    writeln!(out, "rule,k,area,boundary,hist_all,hist_boundary").unwrap();
    for b in blocks {
        out.write_all(b.as_bytes()).unwrap();
    }
}
