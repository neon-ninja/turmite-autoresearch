//! Lab for the tether ("home") questions (frontier items 11-16, 22). Does not touch the knockout
//! map code. All randomness is seeded; output is deterministic.
//!
//! usage:
//!   tetherlab screen AGE TRIAL SEED   < rules  : item 15. grow each rule to AGE steps, then
//!        run a fresh ant (home, heading north) for TRIAL steps on (a) the real field and (b) a
//!        shuffled field; CSV rule,area,extent,spread_real,near_real,spread_shuf,near_shuf
//!   tetherlab dose RULE AGE TRIAL SEEDS MODE...: items 13, 14, 16. MODE in
//!        real | shuffle | keep:F | class:K | parity:P | ctrl:M | disc:R ; CSV rule,age,mode,seed,spread,near,maxd,escaped
//!   tetherlab flip RULE AGE CONT NPERT NFLIP SEED : item 11. grow to AGE, then clone the *whole* state
//!        (ant included) and flip NFLIP random visited cells (colour += 1); continue CONT steps
//!        for both; CSV rule,pert,flipped_dist,first_contact,damage_at_1e4_1e5_1e6_after_contact,
//!        spread_last_quarter_perturbed,spread_last_quarter_base
//!   tetherlab form RULE TOTAL SEEDS : item 12. patched-start ants (5x5 random patch, seeds 1..SEEDS),
//!        windowed RMS distance from the start in windows of 2^15 steps; CSV rule,seed,onset,rms_final,...
use std::io::BufRead;
use turmite::walkers::{PatchedAnt, Walker, splitmix_next};
use turmite::{DIRS, Rule};

const S: usize = 2048;
const C: i64 = (S / 2) as i64;
/// Number of colours of the rule under study (set by `dose`; used by the recolour modes).
static NC: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(2);

#[derive(Clone)]
struct State {
    colour: Vec<u8>,
    visited: Vec<bool>,
    x: i64,
    y: i64,
    h: u8,
    t: u64,
}

fn blank() -> State {
    let mut v = vec![false; S * S];
    v[(C as usize) * S + C as usize] = true;
    State { colour: vec![0; S * S], visited: v, x: C, y: C, h: 0, t: 0 }
}

/// Run `steps` more steps; returns false if the ant came within 3 cells of the edge (state then undefined).
fn advance(s: &mut State, turn: &[u8], n: u8, steps: u64) -> bool {
    for _ in 0..steps {
        let i = (s.y as usize) * S + s.x as usize;
        let c = s.colour[i];
        s.h = (s.h + turn[c as usize]) & 3;
        s.colour[i] = if c + 1 == n { 0 } else { c + 1 };
        let (dx, dy) = DIRS[s.h as usize];
        s.x += dx as i64;
        s.y += dy as i64;
        s.visited[(s.y as usize) * S + s.x as usize] = true;
        s.t += 1;
        if s.x < 3 || s.y < 3 || s.x > S as i64 - 4 || s.y > S as i64 - 4 {
            return false;
        }
    }
    true
}

struct Trial {
    spread: f64,
    near: f64,
    maxd: f64,
    escaped: bool,
}

/// Run `steps` steps; stats over the last quarter: RMS distance from that quarter's mean position
/// (spread), fraction of time within 10 cells of home (C, C), max distance from home overall.
fn trial(s: &mut State, turn: &[u8], n: u8, steps: u64) -> Trial {
    let q = steps / 4;
    if !advance(s, turn, n, steps - q) {
        return Trial { spread: f64::NAN, near: 0.0, maxd: f64::INFINITY, escaped: true };
    }
    let (mut sx, mut sy, mut sr2, mut near, mut maxd2) = (0f64, 0f64, 0f64, 0u64, 0i64);
    for _ in 0..q {
        if !advance(s, turn, n, 1) {
            return Trial { spread: f64::NAN, near: 0.0, maxd: f64::INFINITY, escaped: true };
        }
        let (rx, ry) = (s.x - C, s.y - C);
        sx += rx as f64;
        sy += ry as f64;
        sr2 += (rx * rx + ry * ry) as f64;
        near += (rx * rx + ry * ry <= 100) as u64;
        maxd2 = maxd2.max(rx * rx + ry * ry);
    }
    let qf = q as f64;
    Trial { spread: (sr2 / qf - (sx / qf).powi(2) - (sy / qf).powi(2)).max(0.0).sqrt(), near: near as f64 / qf, maxd: (maxd2 as f64).sqrt(), escaped: false }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        splitmix_next(&mut self.0)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

fn cells(visited: &[bool]) -> Vec<usize> {
    (0..S * S).filter(|&i| visited[i]).collect()
}

/// Shuffle the colours among the given cell indices.
fn shuffle_among(colour: &mut [u8], idx: &[usize], rng: &mut Rng) {
    let mut vals: Vec<u8> = idx.iter().map(|&i| colour[i]).collect();
    rng.shuffle(&mut vals);
    for (k, &i) in idx.iter().enumerate() {
        colour[i] = vals[k];
    }
}

/// A fresh ant at home (heading north) on a modified copy of the grown field.
fn modified(base: &State, mode: &str, rng: &mut Rng) -> State {
    let mut s = base.clone();
    s.x = C;
    s.y = C;
    s.h = 0;
    s.t = 0;
    let vis = cells(&base.visited);
    let (kind, arg) = mode.split_once(':').unwrap_or((mode, ""));
    match kind {
        "real" => {}
        "shuffle" => shuffle_among(&mut s.colour, &vis, rng),
        "keep" => {
            let f: f64 = arg.parse().unwrap();
            let moved: Vec<usize> = vis.iter().copied().filter(|_| (rng.next() as f64 / u64::MAX as f64) >= f).collect();
            shuffle_among(&mut s.colour, &moved, rng);
        }
        "ctrl" => {
            let m: usize = arg.parse().unwrap();
            let mut v = vis.clone();
            rng.shuffle(&mut v);
            v.truncate(m);
            shuffle_among(&mut s.colour, &v, rng);
        }
        "ctrlclass" => {
            // random control matched to class:K in the expected number of changed cells
            let k: u8 = arg.parse().unwrap();
            let cnt = vis.iter().filter(|&&i| base.colour[i] == k).count() as f64;
            let a = vis.len() as f64;
            let mut counts = vec![0f64; 256];
            for &i in &vis { counts[base.colour[i] as usize] += 1.0; }
            let same: f64 = counts.iter().map(|c| (c / a) * (c / a)).sum();
            let m = ((2.0 * cnt) / (1.0 - same)).round() as usize;
            let mut v = vis.clone();
            rng.shuffle(&mut v);
            v.truncate(m.min(vis.len()));
            shuffle_among(&mut s.colour, &v, rng);
        }
        "class" => {
            // every cell of colour k swaps with a random cell of another colour
            let k: u8 = arg.parse().unwrap();
            let ks: Vec<usize> = vis.iter().copied().filter(|&i| base.colour[i] == k).collect();
            let mut others: Vec<usize> = vis.iter().copied().filter(|&i| base.colour[i] != k).collect();
            rng.shuffle(&mut others);
            for (j, &i) in ks.iter().enumerate() {
                if j < others.len() {
                    s.colour.swap(i, others[j]);
                }
            }
        }
        "parity" => {
            let p: usize = arg.parse().unwrap();
            let idx: Vec<usize> = vis.iter().copied().filter(|&i| ((i % S) + (i / S)) & 1 == p).collect();
            shuffle_among(&mut s.colour, &idx, rng);
        }
        "recolour" | "recolourctrl" => {
            // recolour (to a uniformly random different colour) every cell of colour K, or the same number of random cells
            let k: u8 = arg.parse().unwrap();
            let n = NC.load(std::sync::atomic::Ordering::Relaxed);
            let mut targets: Vec<usize> = vis.iter().copied().filter(|&i| base.colour[i] == k).collect();
            if kind == "recolourctrl" {
                let m = targets.len();
                let mut v = vis.clone();
                rng.shuffle(&mut v);
                v.truncate(m);
                targets = v;
            }
            for i in targets {
                let old = s.colour[i] as u64;
                s.colour[i] = ((old + 1 + rng.next() % (n - 1)) % n) as u8;
            }
        }
        "recolourlist" => {
            // recolour the cells listed in the file `arg` (lines "x,y", relative to home) to a uniformly random different colour
            let n = NC.load(std::sync::atomic::Ordering::Relaxed);
            for line in std::fs::read_to_string(arg).unwrap().lines() {
                let Some((xs, ys)) = line.split_once(',') else { continue };
                let (x, y): (i64, i64) = (xs.trim().parse().unwrap(), ys.trim().parse().unwrap());
                let i = ((C + y) as usize) * S + (C + x) as usize;
                if base.visited[i] {
                    let old = s.colour[i] as u64;
                    s.colour[i] = ((old + 1 + rng.next() % (n - 1)) % n) as u8;
                }
            }
        }
        "core" => {
            // shuffle only the cells within radius R of home
            let r: f64 = arg.parse().unwrap();
            let idx: Vec<usize> = vis.iter().copied().filter(|&i| {
                let (dx, dy) = ((i % S) as f64 - C as f64, (i / S) as f64 - C as f64);
                (dx * dx + dy * dy).sqrt() <= r
            }).collect();
            shuffle_among(&mut s.colour, &idx, rng);
        }
        "disc" => {
            let r: f64 = arg.parse().unwrap();
            let idx: Vec<usize> = vis
                .iter()
                .copied()
                .filter(|&i| {
                    let (dx, dy) = ((i % S) as f64 - C as f64, (i / S) as f64 - C as f64);
                    (dx * dx + dy * dy).sqrt() > r
                })
                .collect();
            shuffle_among(&mut s.colour, &idx, rng);
        }
        other => panic!("unknown mode {other}"),
    }
    s
}

fn tables(rule: &Rule) -> (Vec<u8>, u8) {
    (rule.0.iter().map(|t| t.delta()).collect(), rule.len() as u8)
}

fn grow(rule: &Rule, age: u64) -> Option<State> {
    let (turn, n) = tables(rule);
    let mut s = blank();
    if advance(&mut s, &turn, n, age) { Some(s) } else { None }
}

fn extent(v: &[bool]) -> (usize, usize) {
    let (mut mn, mut mx, mut a) = ((S, S), (0, 0), 0);
    for y in 0..S {
        for x in 0..S {
            if v[y * S + x] {
                a += 1;
                mn = (mn.0.min(x), mn.1.min(y));
                mx = (mx.0.max(x), mx.1.max(y));
            }
        }
    }
    (a, (mx.0 - mn.0 + 1).max(mx.1 - mn.1 + 1))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a[1].as_str() {
        "screen" => {
            let (age, steps, seed): (u64, u64, u64) = (a[2].parse().unwrap(), a[3].parse().unwrap(), a[4].parse().unwrap());
            let rules: Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
            use rayon::prelude::*;
            let rows: Vec<String> = rules
                .par_iter()
                .map(|r| {
                    let rule: Rule = r.parse().unwrap();
                    let (turn, n) = tables(&rule);
                    let Some(base) = grow(&rule, age) else { return format!("{r},escaped,,,,,") };
                    let (area, ext) = extent(&base.visited);
                    let mut rng = Rng(seed ^ 0xABCDEF);
                    let mut real = modified(&base, "real", &mut rng);
                    let mut shuf = modified(&base, "shuffle", &mut rng);
                    let tr = trial(&mut real, &turn, n, steps);
                    let ts = trial(&mut shuf, &turn, n, steps);
                    format!("{r},{area},{ext},{:.2},{:.3},{:.2},{:.3}", tr.spread, tr.near, ts.spread, ts.near)
                })
                .collect();
            println!("rule,area,extent,spread_real,near_real,spread_shuf,near_shuf");
            for r in rows {
                println!("{r}");
            }
        }
        "dose" => {
            let rule: Rule = a[2].parse().unwrap();
            let (age, steps, seeds): (u64, u64, u64) = (a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap());
            let (turn, n) = tables(&rule);
            NC.store(n as u64, std::sync::atomic::Ordering::Relaxed);
            let base = grow(&rule, age).expect("grew off the board");
            let modes: Vec<String> = a[6..].to_vec();
            let (area, ext) = extent(&base.visited);
            eprintln!("{rule} age {age}: area {area} extent {ext}");
            use rayon::prelude::*;
            let jobs: Vec<(String, u64)> = modes.iter().flat_map(|m| (1..=seeds).map(move |s| (m.clone(), s))).collect();
            let rows: Vec<String> = jobs
                .par_iter()
                .map(|(m, seed)| {
                    let mut rng = Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) ^ 0x1234);
                    let mut s = modified(&base, m, &mut rng);
                    let t = trial(&mut s, &turn, n, steps);
                    format!("{rule},{age},{m},{seed},{:.2},{:.3},{:.1},{}", t.spread, t.near, t.maxd, t.escaped)
                })
                .collect();
            println!("rule,age,mode,seed,spread,near,maxd,escaped");
            for r in rows {
                println!("{r}");
            }
        }
        "describe" => {
            let rule: Rule = a[2].parse().unwrap();
            let age: u64 = a[3].parse().unwrap();
            let base = grow(&rule, age).expect("off board");
            let vis = cells(&base.visited);
            println!("rule,colour,count,mean_r,median_r,frac_within_10,frac_within_20");
            for k in 0..rule.len() as u8 {
                let mut rs: Vec<f64> = vis.iter().filter(|&&i| base.colour[i] == k).map(|&i| (((i % S) as f64 - C as f64).powi(2) + ((i / S) as f64 - C as f64).powi(2)).sqrt()).collect();
                if rs.is_empty() { continue; }
                rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let n = rs.len() as f64;
                println!("{rule},{k},{},{:.1},{:.1},{:.3},{:.3}", rs.len(), rs.iter().sum::<f64>() / n, rs[rs.len() / 2], rs.iter().filter(|&&r| r <= 10.0).count() as f64 / n, rs.iter().filter(|&&r| r <= 20.0).count() as f64 / n);
            }
        }
        "flip" => {
            let rule: Rule = a[2].parse().unwrap();
            let (age, cont, npert, nflip, seed): (u64, u64, usize, usize, u64) =
                (a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap(), a[6].parse().unwrap(), a[7].parse().unwrap());
            let (turn, n) = tables(&rule);
            let base = grow(&rule, age).expect("grew off the board");
            let vis = cells(&base.visited);
            use rayon::prelude::*;
            let rows: Vec<String> = (0..npert)
                .into_par_iter()
                .map(|p| {
                    let mut rng = Rng(seed.wrapping_add(p as u64).wrapping_mul(0xD1B54A32D192ED03));
                    let mut a = base.clone();
                    let mut b = base.clone();
                    let mut dists = vec![];
                    for _ in 0..nflip {
                        let i = vis[rng.below(vis.len())];
                        b.colour[i] = (b.colour[i] + 1) % n;
                        dists.push((((i % S) as f64 - C as f64).powi(2) + ((i / S) as f64 - C as f64).powi(2)).sqrt());
                    }
                    // run both in lockstep until the first step at which their states differ in position,
                    // then record the damage (number of differing cells) at fixed lags after contact.
                    let mut contact: Option<u64> = None;
                    let lags = [10_000u64, 100_000, 1_000_000];
                    let mut damage = [f64::NAN; 3];
                    let diff = |a: &State, b: &State| a.colour.iter().zip(&b.colour).filter(|(x, y)| x != y).count();
                    let mut t = 0u64;
                    let (mut sx, mut sy, mut sr2, mut q) = ((0f64, 0f64), (0f64, 0f64), (0f64, 0f64), 0u64);
                    let qs = cont - cont / 4;
                    let mut ok = true;
                    while t < cont && ok {
                        ok = advance(&mut a, &turn, n, 1) && advance(&mut b, &turn, n, 1);
                        t += 1;
                        if contact.is_none() && (a.x != b.x || a.y != b.y || a.h != b.h) {
                            contact = Some(t);
                        }
                        if let Some(c0) = contact {
                            for (j, l) in lags.iter().enumerate() {
                                if t == c0 + l {
                                    damage[j] = diff(&a, &b) as f64;
                                }
                            }
                        }
                        if t > qs {
                            q += 1;
                            let (ax, ay, bx, by) = ((a.x - C) as f64, (a.y - C) as f64, (b.x - C) as f64, (b.y - C) as f64);
                            sx.0 += ax;
                            sx.1 += bx;
                            sy.0 += ay;
                            sy.1 += by;
                            sr2.0 += ax * ax + ay * ay;
                            sr2.1 += bx * bx + by * by;
                        }
                    }
                    let qf = q.max(1) as f64;
                    let sp = |s: f64, y: f64, r: f64| (r / qf - (s / qf).powi(2) - (y / qf).powi(2)).max(0.0).sqrt();
                    let dmean = dists.iter().sum::<f64>() / dists.len() as f64;
                    format!(
                        "{rule},{p},{dmean:.1},{},{},{},{},{:.2},{:.2},{ok}",
                        contact.map(|c| c.to_string()).unwrap_or("none".into()),
                        damage[0],
                        damage[1],
                        damage[2],
                        sp(sx.1, sy.1, sr2.1),
                        sp(sx.0, sy.0, sr2.0)
                    )
                })
                .collect();
            println!("rule,pert,flip_dist,first_contact,dmg_1e4,dmg_1e5,dmg_1e6,spread_perturbed,spread_base,ok");
            for r in rows {
                println!("{r}");
            }
        }
        "formage" => {
            // item 12: when does the field first support a home? Patched starts (seed 1..SEEDS; seed 0 = blank); at each age
            // 2^k, k = KLO..=KHI, snapshot, run a fresh ant on the real and on a shuffled copy. usage: formage RULE SEEDS KLO KHI TRIAL
            let rule: Rule = a[2].parse().unwrap();
            let (seeds, klo, khi, steps): (u64, u32, u32, u64) = (a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap(), a[6].parse().unwrap());
            let (turn, n) = tables(&rule);
            use rayon::prelude::*;
            let rows: Vec<Vec<String>> = (0..=seeds).into_par_iter().map(|seed| {
                let mut s = blank();
                if seed > 0 {
                    let w = PatchedAnt::new(&rule, seed, 2);
                    for y in 0..S { for x in 0..S { s.colour[y * S + x] = w.initial_cell(x as i64 - C, y as i64 - C); } }
                }
                let mut out = vec![];
                for k in klo..=khi {
                    let todo = (1u64 << k) - s.t;
                    if !advance(&mut s, &turn, n, todo) { break; }
                    let (area, ext) = extent(&s.visited);
                    let mut rng = Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) ^ k as u64);
                    let mut real = modified(&s, "real", &mut rng);
                    let mut shuf = modified(&s, "shuffle", &mut rng);
                    let tr = trial(&mut real, &turn, n, steps);
                    let ts = trial(&mut shuf, &turn, n, steps);
                    out.push(format!("{rule},{seed},{k},{area},{ext},{:.2},{:.2}", tr.spread, ts.spread));
                }
                out
            }).collect();
            println!("rule,seed,k,area,extent,spread_real,spread_shuf");
            for r in rows { for l in r { println!("{l}"); } }
        }
        "form" => {
            let rule: Rule = a[2].parse().unwrap();
            let (total, seeds): (u64, u64) = (a[3].parse().unwrap(), a[4].parse().unwrap());
            let (turn, n) = tables(&rule);
            let w_log: u32 = a.get(5).map(|s| s.parse().unwrap()).unwrap_or(15);
            let W: u64 = 1 << w_log;
            use rayon::prelude::*;
            let rows: Vec<String> = (1..=seeds)
                .into_par_iter()
                .map(|seed| {
                    let w = PatchedAnt::new(&rule, seed, 2);
                    let mut s = blank();
                    for y in 0..S {
                        for x in 0..S {
                            s.colour[y * S + x] = w.initial_cell(x as i64 - C, y as i64 - C);
                        }
                    }
                    let mut rms = vec![];
                    let mut ok = true;
                    for _ in 0..total / W {
                        let (mut sr2, mut ok2) = (0f64, true);
                        for _ in 0..W {
                            ok2 &= advance(&mut s, &turn, n, 1);
                            if !ok2 {
                                break;
                            }
                            sr2 += ((s.x - C).pow(2) + (s.y - C).pow(2)) as f64;
                        }
                        if !ok2 {
                            ok = false;
                            break;
                        }
                        rms.push((sr2 / W as f64).sqrt());
                    }
                    if !ok || rms.len() < 8 {
                        return format!("{rule},{seed},escaped,,,,");
                    }
                    let tail = &rms[rms.len() * 3 / 4..];
                    let mut sorted = tail.to_vec();
                    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let home = sorted[sorted.len() / 2];
                    let mx = sorted[sorted.len() - 1];
                    // onset: first window index after which every window has RMS <= 1.5 * max(home, 5)
                    let thr = 1.5 * home.max(5.0);
                    let mut onset = rms.len();
                    while onset > 0 && rms[onset - 1] <= thr {
                        onset -= 1;
                    }
                    let (area, ext) = extent(&s.visited);
                    // fraction of windows at home (RMS <= thr) per octave of window index: octave j covers windows [2^j, 2^(j+1))
                    let mut pj = vec![];
                    let mut j = 0;
                    while (1usize << j) < rms.len() {
                        let (lo, hi) = (1usize << j, (1usize << (j + 1)).min(rms.len()));
                        pj.push(format!("{:.2}", rms[lo..hi].iter().filter(|&&r| r <= thr).count() as f64 / (hi - lo) as f64));
                        j += 1;
                    }
                    format!("{rule},{seed},{},{home:.1},{mx:.1},{area},{ext},{}", onset as u64 * W, pj.join(";"))
                })
                .collect();
            println!("rule,seed,onset_steps,rms_home_median_last_quarter,rms_max_last_quarter,area,extent,p_home_by_octave_of_window_index");
            for r in rows {
                println!("{r}");
            }
        }
        other => panic!("unknown command {other}"),
    }
}
