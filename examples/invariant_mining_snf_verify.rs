//! Oracle check for the laws mined by `experiments/_lib/lawmod.py`.
//!
//! cargo run --release --example invariant_mining_snf_verify -- LAWS.csv STEPS STARTS SEED > out.csv
//!
//! LAWS.csv columns: rule,family,m,expect,a,b,e,d,phi,w  (phi = "h:s:v;...", w = "s:c:v;...").
//! Each law is applied to `STARTS` seeded random starts of `reference::Ant` (random heading, position,
//! and a random finite patch on a blank field) for `STEPS` steps; the change of the law's value across
//! every step is computed from the oracle's actual before/after states. Families F2/F3 are only applied
//! to starts on the manifold h + x + y even (they are proved there only); A0/F1 get half off-manifold
//! starts. At the end of each start the value is also recomputed from scratch and compared with the
//! initial value, so a bookkeeping error in the incremental sum cannot hide.
//! Output: one line per law, `line,violations` (violations = number of starts with any residual).
use std::collections::HashMap;
use turmite::{Rule, reference::Ant, walkers::splitmix_next};

struct Law {
    rule: Rule,
    n: usize,
    family: String,
    m: i64,
    a: i64,
    b: i64,
    e: i64,
    d: i64,
    phi: HashMap<(usize, usize), i64>,
    w: HashMap<(usize, usize), i64>,
}

fn class(family: &str, x: i32, y: i32) -> usize {
    match family {
        "F2" => (x + y).rem_euclid(2) as usize,
        "F3" => (x.rem_euclid(2) * 2 + y.rem_euclid(2)) as usize,
        _ => 0,
    }
}

fn kv_pairs(s: &str, width: usize) -> Vec<Vec<i64>> {
    s.split(';').filter(|t| !t.is_empty()).map(|t| {
        let v: Vec<i64> = t.split(':').map(|x| x.parse().unwrap()).collect();
        assert_eq!(v.len(), width);
        v
    }).collect()
}

fn parse(line: &str) -> Law {
    let f: Vec<&str> = line.split(',').collect();
    assert_eq!(f.len(), 10, "bad law line {line}");
    let rule: Rule = f[0].parse().unwrap();
    let phi = kv_pairs(f[8], 3).into_iter().map(|v| ((v[0] as usize, v[1] as usize), v[2])).collect();
    let w = kv_pairs(f[9], 3).into_iter().map(|v| ((v[0] as usize, v[1] as usize), v[2])).collect();
    Law {
        n: rule.len(), rule, family: f[1].to_string(), m: f[2].parse().unwrap(),
        a: f[4].parse().unwrap(), b: f[5].parse().unwrap(), e: f[6].parse().unwrap(), d: f[7].parse().unwrap(),
        phi, w,
    }
}

impl Law {
    fn phi_at(&self, h: usize, x: i32, y: i32) -> i64 {
        if self.family == "A0" { return self.d * h as i64; }
        *self.phi.get(&(h, class(&self.family, x, y))).unwrap_or(&0)
    }
    fn w_at(&self, x: i32, y: i32, c: usize) -> i64 {
        if c == 0 { 0 } else { *self.w.get(&(class(&self.family, x, y), c)).unwrap_or(&0) }
    }
    fn value(&self, ant: &Ant, steps: i64) -> i64 {
        let cells: i64 = ant.cells.iter().map(|(&(x, y), &c)| self.w_at(x, y, c as usize)).sum();
        self.a * ant.pos.0 as i64 + self.b * ant.pos.1 as i64 + self.e * steps
            + self.phi_at(ant.heading as usize, ant.pos.0, ant.pos.1) + cells
    }
}

fn rnd(state: &mut u64, k: u64) -> u64 { splitmix_next(state) % k }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 5, "usage: LAWS.csv STEPS STARTS SEED");
    let text = std::fs::read_to_string(&args[1]).unwrap();
    let steps: usize = args[2].parse().unwrap();
    let starts: u64 = args[3].parse().unwrap();
    let seed: u64 = args[4].parse().unwrap();
    let laws: Vec<(usize, Law, bool)> = text.lines().enumerate()
        .filter(|(_, l)| !l.is_empty() && !l.starts_with('#') && !l.starts_with("rule,"))
        .map(|(i, l)| { let f: Vec<&str> = l.split(',').collect(); (i, parse(l), f[3] == "1") }).collect();
    let mut by_rule: HashMap<String, Vec<usize>> = HashMap::new();
    for (j, (_, l, _)) in laws.iter().enumerate() { by_rule.entry(l.rule.to_string()).or_default().push(j); }
    let mut violations = vec![0u32; laws.len()];
    let mut applied = vec![0u32; laws.len()];
    let mut rules: Vec<_> = by_rule.keys().cloned().collect();
    rules.sort();
    for rname in rules {
        let idxs = &by_rule[&rname];
        let rule: Rule = rname.parse().unwrap();
        // The stream depends on (seed, rule, start) only, so results do not depend on the laws file's contents.
        let mut rule_key = seed;
        for ch in rname.bytes() { rule_key = rule_key.wrapping_mul(1099511628211).wrapping_add(ch as u64); }
        for s in 0..starts {
            let mut st = rule_key ^ s.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            splitmix_next(&mut st);
            let on_manifold = s % 2 == 0;
            let mut ant = Ant::new(rule.clone());
            ant.heading = rnd(&mut st, 4) as u8;
            let (mut x, y) = (rnd(&mut st, 7) as i32 - 3, rnd(&mut st, 7) as i32 - 3);
            if on_manifold && (ant.heading as i32 + x + y).rem_euclid(2) != 0 { x += 1; }
            ant.pos = (x, y);
            for cx in -4..=4 { for cy in -4..=4 {
                if rnd(&mut st, 2) == 0 { let c = rnd(&mut st, rule.len() as u64) as u8; if c != 0 { ant.cells.insert((cx, cy), c); } }
            }}
            let active: Vec<usize> = idxs.iter().copied()
                .filter(|&j| on_manifold || matches!(laws[j].1.family.as_str(), "A0" | "F1")).collect();
            let start_vals: Vec<i64> = active.iter().map(|&j| laws[j].1.value(&ant, 0)).collect();
            let mut bad = vec![false; active.len()];
            for _ in 0..steps {
                let (px, py) = ant.pos;
                let h = ant.heading as usize;
                let c = ant.colour_at(ant.pos) as usize;
                ant.step();
                let (qx, qy) = ant.pos;
                let h2 = ant.heading as usize;
                for (k, &j) in active.iter().enumerate() {
                    if bad[k] { continue; }
                    let l = &laws[j].1;
                    let delta = l.a * (qx - px) as i64 + l.b * (qy - py) as i64 + l.e
                        + l.phi_at(h2, qx, qy) - l.phi_at(h, px, py)
                        + l.w_at(px, py, (c + 1) % l.n) - l.w_at(px, py, c);
                    if delta.rem_euclid(l.m) != 0 { bad[k] = true; }
                }
            }
            for (k, &j) in active.iter().enumerate() {
                let l = &laws[j].1;
                if (l.value(&ant, ant.steps as i64) - start_vals[k]).rem_euclid(l.m) != 0 { bad[k] = true; }
                applied[j] += 1;
                if bad[k] { violations[j] += 1; }
            }
        }
    }
    println!("line,expect,applied,violations");
    for (j, (i, _, expect)) in laws.iter().enumerate() {
        println!("{i},{},{},{}", u8::from(*expect), applied[j], violations[j]);
    }
}
