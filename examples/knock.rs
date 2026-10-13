//! knock GRIDFILE RULE STEPS base OUTDIR
//! knock GRIDFILE RULE STEPS jobs JOBFILE OUT.csv
//!
//! Knockout runs on a final colour field written by examples/dump.rs (255 =
//! never visited, treated as blank). A fresh ant starts at home heading
//! north, as in examples/transplant.rs, and runs STEPS steps.
//!
//! base: run the intact field; write OUTDIR/occ.u32 and OUTDIR/route.u32
//!   (4096^2 little-endian step counts over the last half of the run; route
//!   counts only steps spent outside r <= 10 of home, i.e. on excursions)
//!   and print the outcome row.
//! jobs: JOBFILE has lines `kind,x,y,w,h,seed` (x,y relative to home, top-left
//!   corner). kind = patch: permute the colours of the visited cells inside the
//!   w*h rectangle among themselves. kind = swap: pick w*h visited cells
//!   uniformly at random anywhere and permute them among themselves (x,y
//!   ignored). Jobs run in parallel; one CSV row each, in job order.
//!
//! Outcome columns, over the last 16 of 64 blocks: near_home (share of time
//! within 10 cells of home), own (within 10 of the previous block's mean),
//! spread (RMS distance from the block's own mean), mean position.
use rayon::prelude::*;
use std::io::Write;
use turmite::{DIRS, Rule};

const S: usize = 4096;
const C: i64 = (S / 2) as i64;

struct Out {
    near_home: f64,
    own: f64,
    spread: f64,
    mx: f64,
    my: f64,
    ok: bool,
}

fn run(colour: &mut [u8], rule: &Rule, steps: u64, mut counts: Option<(&mut Vec<u32>, &mut Vec<u32>, &mut Vec<u32>)>) -> Out {
    // E3 (2026-10-13): excursion buffer for the homeward-leg counts (ret.u32)
    let mut exc: Vec<(usize, i64)> = vec![];
    let n = rule.len() as u8;
    // E2 (2026-10-13): radius of the near_home measure; the default 10 reproduces the original runs exactly.
    let near_r: i64 = std::env::var("NEAR_R").ok().map_or(10, |v| v.parse().unwrap());
    // E1: START_H (0 = north, 1 = east, ...) lets a rotated field be run with the correspondingly rotated start heading; default 0.
    let h0: u8 = std::env::var("START_H").ok().map_or(0, |v| v.parse().unwrap());
    let (mut x, mut y, mut h) = (C, C, h0);
    let block = (steps / 64).max(1);
    let (mut sx, mut sy, mut sr2) = (0f64, 0f64, 0f64);
    let (mut nh, mut np) = (0u64, 0u64);
    let mut prev = (0f64, 0f64);
    let (mut a_nh, mut a_np, mut a_sp, mut a_mx, mut a_my, mut nb) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
    for t in 1..=steps {
        let i = (y as usize) * S + x as usize;
        let col = colour[i];
        h = (h + rule.0[col as usize].delta()) & 3;
        colour[i] = (col + 1) % n;
        let (dx, dy) = DIRS[h as usize];
        x += dx as i64;
        y += dy as i64;
        if x <= 0 || y <= 0 || x >= S as i64 - 1 || y >= S as i64 - 1 {
            return Out { near_home: f64::NAN, own: f64::NAN, spread: f64::NAN, mx: f64::NAN, my: f64::NAN, ok: false };
        }
        let (rx, ry) = (x - C, y - C);
        let r2 = rx * rx + ry * ry;
        sx += rx as f64;
        sy += ry as f64;
        sr2 += r2 as f64;
        nh += (r2 <= near_r * near_r) as u64;
        np += ((rx as f64 - prev.0).powi(2) + (ry as f64 - prev.1).powi(2) <= 100.0) as u64;
        if let Some((occ, route, ret)) = counts.as_mut() {
            if t > steps / 2 {
                let j = (y as usize) * S + x as usize;
                occ[j] += 1;
                if r2 > 100 {
                    route[j] += 1;
                    exc.push((j, r2));
                } else if !exc.is_empty() {
                    // back inside r <= 10: the steps after the farthest point of this excursion are its homeward leg
                    let m = (0..exc.len()).max_by_key(|&i| (exc[i].1, std::cmp::Reverse(i))).unwrap();
                    for &(jj, _) in &exc[m..] {
                        ret[jj] += 1;
                    }
                    exc.clear();
                }
            }
        }
        if t % block == 0 {
            let b = block as f64;
            if t > block * 48 {
                a_nh += nh as f64 / b;
                a_np += np as f64 / b;
                a_sp += (sr2 / b - (sx / b).powi(2) - (sy / b).powi(2)).max(0.0).sqrt();
                a_mx += sx / b;
                a_my += sy / b;
                nb += 1.0;
            }
            prev = (sx / b, sy / b);
            (sx, sy, sr2, nh, np) = (0.0, 0.0, 0.0, 0, 0);
        }
    }
    Out { near_home: a_nh / nb, own: a_np / nb, spread: a_sp / nb, mx: a_mx / nb, my: a_my / nb, ok: true }
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn shuffle<T>(v: &mut [T], st: &mut u64) {
    for i in (1..v.len()).rev() {
        let j = (splitmix(st) % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
}

fn load(raw: &[u8]) -> Vec<u8> {
    raw.iter().map(|&c| if c == 255 { 0 } else { c }).collect()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read(&a[1]).unwrap();
    let rule: Rule = a[2].parse().unwrap();
    let steps: u64 = a[3].parse().unwrap();
    let hdr = "near_home,own,spread,mx,my";
    match a[4].as_str() {
        "base" => {
            let mut colour = load(&raw);
            let mut occ = vec![0u32; S * S];
            let mut route = vec![0u32; S * S];
            let mut ret = vec![0u32; S * S];
            let o = run(&mut colour, &rule, steps, Some((&mut occ, &mut route, &mut ret)));
            let dir = std::path::Path::new(&a[5]);
            for (name, v) in [("occ.u32", &occ), ("route.u32", &route), ("ret.u32", &ret)] {
                let bytes: Vec<u8> = v.iter().flat_map(|c| c.to_le_bytes()).collect();
                std::fs::File::create(dir.join(name)).unwrap().write_all(&bytes).unwrap();
            }
            println!("{hdr}\n{:.4},{:.4},{:.3},{:.2},{:.2}", o.near_home, o.own, o.spread, o.mx, o.my);
        }
        "jobs" => {
            let jobs: Vec<(String, i64, i64, i64, i64, u64)> = std::fs::read_to_string(&a[5])
                .unwrap()
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| {
                    let f: Vec<&str> = l.split(',').collect();
                    (f[0].to_string(), f[1].parse().unwrap(), f[2].parse().unwrap(), f[3].parse().unwrap(), f[4].parse().unwrap(), f[5].parse().unwrap())
                })
                .collect();
            let visited: Vec<usize> = (0..S * S).filter(|&i| raw[i] != 255).collect();
            let base = load(&raw);
            let rare: Vec<u8> = std::env::var("RARE").ok().map_or(vec![], |v| v.split(',').map(|c| c.parse().unwrap()).collect());
            let have_rare = std::env::var("RARE").is_ok();
            // A6 mechanism (2026-10-13): kind "strat" corrupts w*h cells drawn from pool[class=x][occupancy quartile=y], where class 1 = base colour in RARE,
            // and occupancy comes from the intact run's occ.u32 (env STRAT_OCC); "corrupt" = a uniformly random different colour.
            let mut pools: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; 4]; 2];
            // E3b: kind "strat2": excursion cells (route > 0) split by homeward share ret/route (class 1: >= 0.6, 0: <= 0.4) and route-count quartile.
            let mut pools2: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; 4]; 2];
            if let (Ok(pr), Ok(pt)) = (std::env::var("STRAT_ROUTE"), std::env::var("STRAT_RET")) {
                let rd = |p: &str| -> Vec<u32> { std::fs::read(p).unwrap().chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect() };
                let (route, ret) = (rd(&pr), rd(&pt));
                let mut vals: Vec<u32> = visited.iter().map(|&i| route[i]).filter(|&v| v > 0).collect();
                vals.sort();
                let q = [vals[vals.len() / 4], vals[vals.len() / 2], vals[3 * vals.len() / 4]];
                let mut bins: Vec<Vec<(f64, usize)>> = vec![vec![]; 4];
                for &i in &visited {
                    if route[i] == 0 {
                        continue;
                    }
                    bins[q.iter().filter(|&&t| route[i] > t).count()].push((ret[i] as f64 / route[i] as f64, i));
                }
                // within each route quartile, split at the median homeward share (class 1 = upper half)
                for (b, v) in bins.iter_mut().enumerate() {
                    v.sort_by(|a, c| a.0.partial_cmp(&c.0).unwrap().then(a.1.cmp(&c.1)));
                    let h = v.len() / 2;
                    for (k, &(_, i)) in v.iter().enumerate() {
                        pools2[(k >= h) as usize][b].push(i);
                    }
                }
                eprintln!("strat2 quartile thresholds {:?}; pool sizes {:?}", q, pools2.iter().map(|c| c.iter().map(|b| b.len()).collect::<Vec<_>>()).collect::<Vec<_>>());
            }
            if let Ok(p) = std::env::var("STRAT_OCC") {
                let raw_occ = std::fs::read(&p).unwrap();
                let occ: Vec<u32> = raw_occ.chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
                let mut vals: Vec<u32> = visited.iter().map(|&i| occ[i]).filter(|&v| v > 0).collect();
                vals.sort();
                let q = [vals[vals.len() / 4], vals[vals.len() / 2], vals[3 * vals.len() / 4]];
                for &i in &visited {
                    if occ[i] == 0 {
                        continue;
                    }
                    let bin = q.iter().filter(|&&t| occ[i] > t).count();
                    let class = rare.contains(&base[i]) as usize;
                    pools[class][bin].push(i);
                }
                eprintln!("strat quartile thresholds {:?}; pool sizes {:?}", q, pools.iter().map(|c| c.iter().map(|b| b.len()).collect::<Vec<_>>()).collect::<Vec<_>>());
            }
            let rows: Vec<String> = jobs
                .par_iter()
                .map(|(kind, x, y, w, h, seed)| {
                    let mut st = seed.wrapping_mul(0x2545F4914F6CDD1D) ^ ((*x as u64) << 20) ^ ((*y as u64) << 40) ^ 0xABCDEF;
                    let idx: Vec<usize> = if kind == "strat" || kind == "strat2" {
                        let cnt = (*w * *h) as usize;
                        let mut pool = if kind == "strat" { pools[*x as usize][*y as usize].clone() } else { pools2[*x as usize][*y as usize].clone() };
                        for k in 0..cnt {
                            let j = k + (splitmix(&mut st) % (pool.len() - k) as u64) as usize;
                            pool.swap(k, j);
                        }
                        pool.truncate(cnt);
                        pool
                    } else if kind == "patch" {
                        let mut v = vec![];
                        for yy in *y..*y + *h {
                            for xx in *x..*x + *w {
                                let i = ((C + yy) as usize) * S + (C + xx) as usize;
                                if raw[i] != 255 {
                                    v.push(i);
                                }
                            }
                        }
                        v
                    } else {
                        let cnt = (*w * *h) as usize;
                        let mut pool = visited.clone();
                        for k in 0..cnt {
                            let j = k + (splitmix(&mut st) % (pool.len() - k) as u64) as usize;
                            pool.swap(k, j);
                        }
                        pool.truncate(cnt);
                        pool
                    };
                    let mut colour = base.clone();
                    let mut vals: Vec<u8> = idx.iter().map(|&i| colour[i]).collect();
                    if kind == "strat" || kind == "strat2" {
                        let nn = rule.len() as u64;
                        for v in vals.iter_mut() {
                            *v = ((*v as u64 + 1 + splitmix(&mut st) % (nn - 1)) % nn) as u8;
                        }
                    } else {
                        shuffle(&mut vals, &mut st);
                    }
                    let mut changed = 0usize;
                    // E4 (2026-10-13): with RARE=c1,c2,.. also count cells that held a rare colour and
                    // were changed (lost) and cells that received a rare colour from elsewhere (gained).
                    let (mut lost, mut gained) = (0usize, 0usize);
                    for (k, &i) in idx.iter().enumerate() {
                        changed += (colour[i] != vals[k]) as usize;
                        if colour[i] != vals[k] {
                            lost += rare.contains(&colour[i]) as usize;
                            gained += rare.contains(&vals[k]) as usize;
                        }
                        colour[i] = vals[k];
                    }
                    let o = run(&mut colour, &rule, steps, None);
                    let extra = if have_rare { format!(",{lost},{gained}") } else { String::new() };
                    format!(
                        "{kind},{x},{y},{w},{h},{seed},{},{:.4},{:.4},{:.4},{:.3},{:.2},{:.2},{}{extra}",
                        idx.len(),
                        changed as f64 / idx.len().max(1) as f64,
                        o.near_home,
                        o.own,
                        o.spread,
                        o.mx,
                        o.my,
                        o.ok as u8
                    )
                })
                .collect();
            let mut f = std::fs::File::create(&a[6]).unwrap();
            writeln!(f, "kind,x,y,w,h,seed,nvis,changed,{hdr},ok{}", if have_rare { ",rare_lost,rare_gained" } else { "" }).unwrap();
            for r in rows {
                writeln!(f, "{r}").unwrap();
            }
        }
        m => panic!("unknown mode {m}"),
    }
}
