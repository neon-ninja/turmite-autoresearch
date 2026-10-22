//! Equal-budget comparison of attack-search strategies for the falsification harness
//! (experiment `2026-10-22-falsify-adaptive-search`).
//!
//! usage: falsify_search OUT_DIR [CLAIMS.txt] [--reps 30] [--budget 200]
//!
//! Arms, each given exactly `budget` evaluations per replicate on claims C1..C3 of the harness:
//! - `random`: uniform draws from the harness's own start generators (patch, mono, half, stripe, sparse, obstacle);
//! - `old`: the harness's one-cell hill-climb (`adaptive_chain`);
//! - `evolve`: random draws for the first quarter of the budget, then a (mu + lambda) search with multi-scale
//!   mutations that include the obstacle's parameters.
//! Everything is keyed by (claim, arm, replicate), so output does not depend on the thread count.

use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::process::Command;
use turmite::Rule;
use turmite::falsify::*;
use turmite::walkers::{mix64, splitmix_next};

const UNION: [Class; 6] = [Class::Patch, Class::Mono, Class::Half, Class::Stripe, Class::Sparse, Class::Obstacle];
const ELITES: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Arm {
    Random,
    Old,
    Evolve,
    Evolve2,
}

impl Arm {
    fn name(self) -> &'static str {
        match self {
            Arm::Random => "random",
            Arm::Old => "old",
            Arm::Evolve => "evolve",
            Arm::Evolve2 => "evolve2",
        }
    }
    fn salt(self) -> u64 {
        match self {
            Arm::Random => 0x72616e64,
            Arm::Old => 0x6f6c64,
            Arm::Evolve => 0x65766f6c,
            Arm::Evolve2 => 0x65766f32,
        }
    }
}

struct Ctx<'a> {
    rule: &'a Rule,
    base: &'a Baseline,
    size: usize,
    horizon: u64,
    claim_seed: u64,
}

/// Attacker's score: the escape step of a baseline-type highway; twice the horizon for a start that never
/// resolved; four times the horizon for a highway of a different type (a counterexample).
fn score(r: &Run, base: &Baseline, horizon: u64) -> f64 {
    match categorise(r, base) {
        Cat::HwSame => r.steps as f64,
        Cat::HwOther => 4.0 * horizon as f64,
        Cat::Edge | Cat::Unresolved => 2.0 * horizon as f64,
    }
}

#[derive(Default, Clone, Debug)]
struct Outcome1 {
    max: f64,
    steps_at_max: u64,
    other: u32,
    unresolved: u32,
    evals: u32,
}

impl Outcome1 {
    fn push(&mut self, s: f64, r: &Run, base: &Baseline) {
        if s > self.max {
            self.max = s;
            self.steps_at_max = r.steps;
        }
        match categorise(r, base) {
            Cat::HwOther => self.other += 1,
            Cat::Edge | Cat::Unresolved => self.unresolved += 1,
            Cat::HwSame => {}
        }
        self.evals += 1;
    }
}

fn random_trial(c: &Ctx, rep: u32, k: u32, budget: u32, arm_salt: u64) -> Trial {
    let idx = rep * budget + k;
    let class = UNION[(mix64(c.claim_seed ^ arm_salt ^ idx as u64) % UNION.len() as u64) as usize];
    make_trial(c.rule, class, idx, c.claim_seed ^ arm_salt, c.size, c.horizon, c.base)
}

fn eval(c: &Ctx, t: &Trial) -> (f64, Run) {
    let r = run_trial(c.rule, t).run;
    (score(&r, c.base, c.horizon), r)
}

fn mutate(c: &Ctx, parent: &Trial, st: &mut u64, idx: u32) -> Trial {
    let n = c.rule.len() as u64;
    let mut t = parent.clone();
    t.class = Class::Adaptive;
    t.index = idx;
    t.seed = trial_seed(c.claim_seed, c.rule, Class::Adaptive, idx);
    let mut m: BTreeMap<(i32, i32), u8> = t.init.cells.iter().copied().collect();
    let pick = |st: &mut u64, m: &BTreeMap<(i32, i32), u8>| -> Option<(i32, i32)> {
        (!m.is_empty()).then(|| *m.keys().nth((splitmix_next(st) % m.len() as u64) as usize).unwrap())
    };
    let pos = |st: &mut u64| ((splitmix_next(st) % 25) as i32 - 12, (splitmix_next(st) % 25) as i32 - 12);
    for _ in 0..1 + splitmix_next(st) % 3 {
        match splitmix_next(st) % 7 {
            0 => {
                // recolour (colour 0 removes the cell)
                if let Some(p) = pick(st, &m) {
                    let col = (splitmix_next(st) % n) as u8;
                    if col == 0 { m.remove(&p); } else { m.insert(p, col); }
                }
            }
            1 => {
                if let Some(p) = pick(st, &m) { m.remove(&p); }
            }
            2 => {
                let p = pos(st);
                m.insert(p, 1 + (splitmix_next(st) % (n - 1)) as u8);
            }
            3 => {
                let (dx, dy) = ((splitmix_next(st) % 5) as i32 - 2, (splitmix_next(st) % 5) as i32 - 2);
                m = m.into_iter().map(|((x, y), v)| ((x + dx, y + dy), v)).filter(|((x, y), _)| x.abs() <= 40 && y.abs() <= 40).collect();
            }
            4 => t.init.heading = (splitmix_next(st) % 4) as u8,
            5 => {
                // 3x3 stamp of random colours
                let (cx, cy) = pos(st);
                for dy in 0..3 {
                    for dx in 0..3 {
                        let col = (splitmix_next(st) % n) as u8;
                        if col == 0 { m.remove(&(cx + dx, cy + dy)); } else { m.insert((cx + dx, cy + dy), col); }
                    }
                }
            }
            _ => {
                let scale = (c.base.period as i64 * 4).max(1000);
                t.perturb = match (&t.perturb, splitmix_next(st) % 5) {
                    (None, _) | (Some(_), 0) => make_trial(c.rule, Class::Obstacle, idx, c.claim_seed, c.size, c.horizon, c.base).perturb,
                    (Some(p), 1) => Some(Perturb { at: (p.at as i64 + (splitmix_next(st) % (2 * scale as u64 + 1)) as i64 - scale).max(1) as u64, ..p.clone() }),
                    (Some(p), 2) => Some(Perturb { ahead: (p.ahead + (splitmix_next(st) % 17) as i32 - 8).clamp(0, 200), ..p.clone() }),
                    (Some(p), 3) => Some(Perturb { radius: (p.radius + (splitmix_next(st) % 3) as i32 - 1).clamp(0, 6), seed: splitmix_next(st), ..p.clone() }),
                    (Some(_), _) => None,
                };
            }
        }
    }
    t.init.cells = m.into_iter().collect();
    t
}

fn run_arm(c: &Ctx, arm: Arm, rep: u32, budget: u32) -> Outcome1 {
    let mut o = Outcome1::default();
    match arm {
        Arm::Random => {
            for k in 0..budget {
                let t = random_trial(c, rep, k, budget, arm.salt());
                let (s, r) = eval(c, &t);
                o.push(s, &r, c.base);
            }
        }
        Arm::Old => {
            for res in adaptive_chain(c.rule, Kind::Highway, rep, budget, c.claim_seed ^ arm.salt(), c.size, c.horizon, c.base) {
                o.push(score(&res.run, c.base, c.horizon), &res.run, c.base);
            }
        }
        Arm::Evolve | Arm::Evolve2 => {
            let mut st = mix64(c.claim_seed ^ arm.salt() ^ rep as u64);
            let mut pop: Vec<(f64, Trial)> = vec![];
            let seeding = budget / 4;
            for k in 0..budget {
                let fresh = arm == Arm::Evolve2 && k >= seeding && splitmix_next(&mut st) % 4 == 0;
                let t = if k < seeding || fresh {
                    random_trial(c, rep, k, budget, arm.salt())
                } else {
                    let parent = pop[(splitmix_next(&mut st) % pop.len() as u64) as usize].1.clone();
                    mutate(c, &parent, &mut st, rep * budget + k)
                };
                let (s, r) = eval(c, &t);
                o.push(s, &r, c.base);
                pop.push((s, t));
                if k + 1 >= seeding || pop.len() > ELITES {
                    // stable sort keeps the older of equal scores
                    pop.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                    pop.truncate(ELITES);
                }
            }
        }
    }
    o
}

fn norm_sf(z: f64) -> f64 {
    // Abramowitz & Stegun 7.1.26, |error| < 1.5e-7
    let t = 1.0 / (1.0 + 0.3275911 * (z.abs() / 2f64.sqrt()));
    let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let erfc_half = poly * (-(z * z) / 2.0).exp(); // = erfc(|z|/sqrt2)
    if z >= 0.0 { 0.5 * erfc_half } else { 1.0 - 0.5 * erfc_half }
}

/// One-sided Mann-Whitney: P(x tends larger than y), normal approximation with tie correction.
fn mann_whitney(x: &[f64], y: &[f64]) -> (f64, f64) {
    let (n1, n2) = (x.len() as f64, y.len() as f64);
    let u: f64 = x.iter().map(|a| y.iter().map(|b| if a > b { 1.0 } else if a == b { 0.5 } else { 0.0 }).sum::<f64>()).sum();
    let mut all: Vec<f64> = x.iter().chain(y).copied().collect();
    all.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let nn = n1 + n2;
    let mut tie = 0.0;
    let mut i = 0;
    while i < all.len() {
        let j = all[i..].iter().take_while(|&&v| v == all[i]).count();
        tie += (j as f64).powi(3) - j as f64;
        i += j;
    }
    let var = n1 * n2 / 12.0 * ((nn + 1.0) - tie / (nn * (nn - 1.0)));
    let z = (u - n1 * n2 / 2.0) / var.sqrt();
    (u / (n1 * n2), norm_sf(z))
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (s[(s.len() - 1) / 2] + s[s.len() / 2]) / 2.0
}

fn git(args: &[&str]) -> String {
    Command::new("git").args(args).output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_dir = args.first().expect("usage: falsify_search OUT_DIR [CLAIMS.txt] [--reps N] [--budget N]");
    let flag = |name: &str, d: u32| args.iter().position(|a| a == name).map_or(d, |i| args[i + 1].parse().unwrap());
    let (reps, budget) = (flag("--reps", 30), flag("--budget", 200));
    let claims_path = args.get(1).filter(|a| !a.starts_with("--")).cloned().unwrap_or("experiments/2026-10-22-falsification-harness/claims.txt".into());
    let claims: Vec<Claim> = std::fs::read_to_string(&claims_path)
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| Claim::parse(l).unwrap())
        .filter(|c| c.kind == Kind::Highway)
        .collect();

    let mut rows: Vec<(String, Arm, u32, Outcome1)> = vec![];
    for claim in &claims {
        let base = baseline(&claim.rule, claim.size, claim.horizon);
        let ctx = Ctx { rule: &claim.rule, base: &base, size: claim.size, horizon: claim.horizon, claim_seed: claim.seed };
        let jobs: Vec<(Arm, u32)> = [Arm::Random, Arm::Old, Arm::Evolve, Arm::Evolve2].into_iter().flat_map(|a| (0..reps).map(move |r| (a, r))).collect();
        let done: Vec<(String, Arm, u32, Outcome1)> = jobs.par_iter().map(|&(a, r)| (claim.id.clone(), a, r, run_arm(&ctx, a, r, budget))).collect();
        rows.extend(done);
    }
    rows.sort_by(|a, b| (&a.0, a.1, a.2).cmp(&(&b.0, b.1, b.2)));

    let dirty = git(&["status", "--porcelain", "--", "src", "tests"]);
    let prov = format!(
        "# cmd: falsify_search {}\n# commit: {}\n# uncommitted src/tests files: {}\n# reps={reps} budget={budget}\n",
        args.join(" "),
        git(&["rev-parse", "HEAD"]),
        if dirty.is_empty() { "none".to_string() } else { dirty.replace('\n', " | ") }
    );
    let mut csv = prov.clone();
    csv.push_str("claim,arm,rep,max_score,steps_at_max,other_type,unresolved,evals\n");
    for (id, a, r, o) in &rows {
        let _ = writeln!(csv, "{id},{},{r},{},{},{},{},{}", a.name(), o.max, o.steps_at_max, o.other, o.unresolved, o.evals);
    }
    std::fs::create_dir_all(out_dir).unwrap();
    std::fs::write(format!("{out_dir}/search.csv"), csv).unwrap();

    let mut s = prov;
    let _ = writeln!(s, "max score per replicate = longest escape step; 2*horizon = unresolved; 4*horizon = different highway type");
    for claim in &claims {
        let get = |a: Arm| -> Vec<&Outcome1> { rows.iter().filter(|r| r.0 == claim.id && r.1 == a).map(|r| &r.3).collect() };
        let m = |a: Arm| get(a).iter().map(|o| o.max).collect::<Vec<_>>();
        let (rnd, old, evo, ev2) = (m(Arm::Random), m(Arm::Old), m(Arm::Evolve), m(Arm::Evolve2));
        let _ = writeln!(s, "\n{} (rule {})", claim.id, claim.rule);
        for (a, v) in [(Arm::Random, &rnd), (Arm::Old, &old), (Arm::Evolve, &evo), (Arm::Evolve2, &ev2)] {
            let other_reps = get(a).iter().filter(|o| o.other > 0).count();
            let _ = writeln!(s, "  {:<7} median max {:>9.0}  max {:>9.0}  replicates with a different-type highway: {other_reps}/{reps}", a.name(), median(v), v.iter().cloned().fold(0.0, f64::max));
        }
        for (name, x, y) in [("evolve > random", &evo, &rnd), ("old > random", &old, &rnd), ("evolve > old", &evo, &old), ("evolve2 > random", &ev2, &rnd), ("evolve2 > evolve", &ev2, &evo)] {
            let (auc, p) = mann_whitney(x, y);
            let _ = writeln!(s, "  {name:<16} P(x>y)={auc:.3}  one-sided p={p:.2e}");
        }
    }
    std::fs::write(format!("{out_dir}/search_summary.txt"), &s).unwrap();
    print!("{s}");
}
