//! Speed of fast::run vs lean::run_lean. usage: leanbench RULE STEPS GRID
use std::time::Instant;
use turmite::Rule;
use turmite::fast::{self, Config, Grid};
use turmite::lean::{LeanGrid, run_lean};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: Rule = a[1].parse().unwrap();
    let steps: u64 = a[2].parse().unwrap();
    let size: usize = a[3].parse().unwrap();
    let cfg = Config { max_steps: steps, ..Config::default() };
    let t = Instant::now();
    let r1 = fast::run(&rule, &mut Grid::new(size), &cfg);
    let d1 = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let r2 = run_lean(&rule, &mut LeanGrid::new(size), &cfg);
    let d2 = t.elapsed().as_secs_f64();
    assert_eq!((r1.steps, r1.pos, r1.heading, r1.extent), (r2.steps, r2.pos, r2.heading, r2.extent));
    println!("{rule} {steps} steps: fast {:.2}s ({:.0} Msteps/s)  lean {:.2}s ({:.0} Msteps/s)  speedup {:.2}x", d1, steps as f64 / d1 / 1e6, d2, steps as f64 / d2 / 1e6, d1 / d2);
}
