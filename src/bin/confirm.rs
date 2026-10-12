//! Re-verify highways: run each rule on two board sizes and keep it only if
//! both runs find a highway with the same period, drift and onset. A real
//! highway's onset is a property of the rule; an impostor (a builder's
//! straight pass that happens to cross the board edge) depends on the board.
//!
//! usage: confirm MAX_STEPS < rules.txt > confirmed.csv

use rayon::prelude::*;
use std::io::Write;
use turmite::Rule;
use turmite::fast::{self, Config, Grid};

fn main() {
    let max_steps: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let rules: Vec<Rule> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect();
    let cfg = Config {
        max_steps,
        ..Config::default()
    };
    let rows: Vec<String> = rules
        .par_iter()
        .map(|r| {
            let run = |size: usize| {
                let mut g = Grid::new(size);
                let res = fast::run(r, &mut g, &cfg);
                res.highway.map(|h| (h, fast::highway_onset(r, h, res.steps)))
            };
            let (a, b) = (run(8192), run(16384));
            let ok = matches!((a, b), (Some(x), Some(y)) if x == y);
            let f = |o: Option<(turmite::detect::Highway, u64)>| match o {
                Some((h, on)) => format!("{},{},{},{on}", h.period, h.dx, h.dy),
                None => ",,,".into(),
            };
            format!("{r},{},{ok},{},{}", r.len(), f(a), f(b))
        })
        .collect();
    let mut out = std::io::stdout().lock();
    writeln!(out, "rule,n,confirmed,period,dx,dy,onset,period16,dx16,dy16,onset16").unwrap();
    for r in rows {
        writeln!(out, "{r}").unwrap();
    }
}
