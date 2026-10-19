//! Independent HashMap eviction oracle and exact boundary-clock selection.
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::edge_renewal::{self as er, Clock};
use turmite::edge_trace::Layout;
use turmite::local_memory::{LocalMemoryAnt, Reset};
use turmite::walkers::{AntWalker, Walker, mix64, splitmix_next};
use turmite::{DIRS, Rule};

fn compare(rule: &Rule, radius: Option<usize>, seed: u64, policy: Reset, steps: usize) {
    let lay = Layout {
        w: 16,
        h: 24,
        wrap: true,
    };
    let mut fast = LocalMemoryAnt::new(rule, lay, (8, 12), radius, seed, policy);
    let mut dense = vec![0u8; lay.w * lay.h];
    let mut field = HashMap::new();
    for y in 0..lay.h {
        for x in 0..lay.w {
            let c = if y < 12 {
                0
            } else {
                (mix64(seed ^ ((x as u64) << 32 | y as u64)) % rule.len() as u64) as u8
            };
            dense[y * lay.w + x] = c;
            field.insert((x as i64, y as i64), c);
        }
    }
    let norm = |x: i64, y: i64| (x.rem_euclid(lay.w as i64), y.rem_euclid(lay.h as i64));
    let mut cache = HashMap::new();
    let mut read = HashSet::new();
    let mut last_read = HashMap::new();
    let (mut x, mut y, mut hd, mut actual_hd) = (8i64, 12i64, 0u8, 0u8);
    let (mut rng, mut resets, mut changed) = (seed, 0u64, 0u64);
    for step in 1..=steps {
        let p = norm(x, y);
        let i = p.1 as usize * lay.w + p.0 as usize;
        let mut c = field[&p];
        if radius.is_some() {
            if let Some(&cached) = cache.get(&(x, y)) {
                c = cached;
            } else if read.contains(&p) {
                let old = c;
                c = match policy {
                    Reset::Uniform => (splitmix_next(&mut rng) % rule.len() as u64) as u8,
                    Reset::Zero => 0,
                    Reset::Word(k) => {
                        // Construct candidates from the rule afresh, independently of the
                        // production walker's precomputed equivalence-class table.
                        let choices: Vec<_> = (0..rule.len())
                            .filter(|&j| {
                                (0..k).all(|a| {
                                    rule.0[(j + a) % rule.len()]
                                        == rule.0[(old as usize + a) % rule.len()]
                                })
                            })
                            .collect();
                        choices[(splitmix_next(&mut rng) % choices.len() as u64) as usize] as u8
                    }
                    Reset::Next => {
                        let choices: Vec<_> = (0..rule.len())
                            .filter(|&j| {
                                rule.0[(j + 1) % rule.len()]
                                    == rule.0[(old as usize + 1) % rule.len()]
                            })
                            .collect();
                        choices[(splitmix_next(&mut rng) % choices.len() as u64) as usize] as u8
                    }
                };
                if let Reset::Word(k) = policy {
                    assert!((0..k).all(|j| rule.0[(old as usize + j) % rule.len()]
                        == rule.0[(c as usize + j) % rule.len()]));
                }
                resets += 1;
                changed += u64::from(old != c);
            }
            read.insert(p);
        }
        fast.prepare(&mut dense[i], actual_hd);
        assert_eq!(dense[i], c, "pre-read phase differs at {x},{y}");
        assert_eq!(fast.exposure.resets.get(), resets);
        assert_eq!(fast.exposure.changed.get(), changed);
        if let Reset::Word(k) = policy {
            assert!(!fast.exposure.last_turn_changed.get());
            if k >= 2 {
                assert!(!fast.exposure.last_next_turn_changed.get());
            }
        }
        if policy == Reset::Next {
            assert!(!fast.exposure.last_next_turn_changed.get());
        }
        if fast.exposure.last_reset.get() {
            let gap = step - last_read[&p];
            assert!(
                gap >= 4 * (radius.unwrap() + 1),
                "forgot a phase before a possible spatial excursion"
            );
        }
        last_read.insert(p, step);
        hd = (hd + rule.0[c as usize].delta()) & 3;
        c = ((c as usize + 1) % rule.len()) as u8;
        field.insert(p, c);
        if radius.is_some() {
            cache.insert((x, y), c);
        }
        x += DIRS[hd as usize].0 as i64;
        y += DIRS[hd as usize].1 as i64;
        if let Some(r) = radius {
            cache.retain(|&(cx, cy), _| (cx - x).abs() <= r as i64 && (cy - y).abs() <= r as i64);
        }
        fast.step(&mut dense[i], &mut actual_hd);
        assert_eq!(dense[i], c);
        assert_eq!((fast.x, fast.y, actual_hd), (x, y, hd));
    }
}

#[test]
fn turning_path_return_bound() {
    // For a closed prefix, axes alternate and each has t/4 moves of either sign.
    // Exhausting all words also covers every prefix of each word.
    for word in 0u32..(1 << 16) {
        let (mut x, mut y, mut hd, mut largest) = (0i32, 0i32, 0usize, 0i32);
        for t in 1..=16 {
            hd = (hd + if word & (1 << (t - 1)) == 0 { 1 } else { 3 }) & 3;
            x += DIRS[hd].0;
            y += DIRS[hd].1;
            largest = largest.max(x.abs()).max(y.abs());
            if (x, y) == (0, 0) {
                assert_eq!(t % 4, 0);
                assert!(largest <= t / 4);
            }
        }
    }
}

#[test]
fn evicted_phase_aliases_with_same_word_have_identical_futures() {
    let rule: Rule = "RRRRLLRRLLLLRR".parse().unwrap();
    let lay = Layout {
        w: 64,
        h: 64,
        wrap: true,
    };
    for (policy, offset, k) in [
        (Reset::Word(1), 0usize, 1usize),
        (Reset::Word(2), 0, 2),
        (Reset::Next, 1, 1),
    ] {
        let (a, b) = (0..rule.len())
            .flat_map(|a| (a + 1..rule.len()).map(move |b| (a, b)))
            .find(|&(a, b)| {
                (0..k).all(|j| {
                    rule.0[(a + offset + j) % rule.len()] == rule.0[(b + offset + j) % rule.len()]
                }) && if offset == 1 {
                    rule.0[a] != rule.0[b]
                } else {
                    rule.0[(a + k) % rule.len()] != rule.0[(b + k) % rule.len()]
                }
            })
            .unwrap();
        let mut left = LocalMemoryAnt::new(&rule, lay, (32, 32), Some(0), 17, policy);
        let mut right = LocalMemoryAnt::new(&rule, lay, (32, 32), Some(0), 17, policy);
        let (mut lc, mut rc) = (vec![0u8; 4096], vec![0u8; 4096]);
        let (mut lh, mut rh) = (0u8, 0u8);
        for t in 0..260 {
            assert_eq!((left.x, left.y, lh), (right.x, right.y, rh));
            let i = left.y.rem_euclid(64) as usize * 64 + left.x.rem_euclid(64) as usize;
            if t == 4 {
                // Four initial blank reads trace a square. The source is a
                // forgotten re-read. Its retained word, not latent phase, is
                // now the only allowed distinguishing input to sampling.
                assert_eq!((left.x, left.y), (32, 32));
                lc[i] = a as u8;
                rc[i] = b as u8;
            }
            left.prepare(&mut lc[i], lh);
            right.prepare(&mut rc[i], rh);
            assert_eq!(lc[i], rc[i]);
            left.step(&mut lc[i], &mut lh);
            right.step(&mut rc[i], &mut rh);
        }
    }
}

#[test]
fn unique_complete_words_recover_full_ant_even_without_cache() {
    for text in ["RL", "RRL", "RRLL", "RRRLRLL"] {
        let rule: Rule = text.parse().unwrap();
        for a in 0..rule.len() {
            for b in a + 1..rule.len() {
                assert!(
                    (0..rule.len())
                        .any(|j| rule.0[(a + j) % rule.len()] != rule.0[(b + j) % rule.len()])
                );
            }
        }
        let lay = Layout {
            w: 64,
            h: 64,
            wrap: true,
        };
        let pre = |x, y| if (x, y) == (32, 32) { Some(0) } else { None };
        let clock = Clock::Boundary {
            warmup: 16,
            reads: 256,
            max_steps: 8192,
        };
        let w = LocalMemoryAnt::new(&rule, lay, (32, 32), Some(0), 123, Reset::Word(rule.len()));
        let stats = w.exposure.clone();
        let actual = er::trace_clock(
            w,
            Some(&rule),
            lay,
            &pre,
            (32, 32, 0),
            clock,
            |w, c, h| w.prepare(c, h),
            |_| {},
        );
        let expected = er::trace_clock(
            AntWalker::new(&rule),
            Some(&rule),
            lay,
            &pre,
            (32, 32, 0),
            clock,
            |_, _, _| {},
            |_| {},
        );
        assert_eq!(actual, expected);
        assert_eq!(stats.changed.get(), 0);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]
    #[test]
    fn ring_equals_independent_retention(bits in 0u32..256, n in 2usize..=8, r in 0usize..=4, seed in 0u64..10000, zero in any::<bool>()) {
        let rule = Rule::from_bits(bits,n);
        compare(&rule,Some(r),seed,if zero { Reset::Zero } else { Reset::Uniform },2048);
        compare(&rule,None,seed,Reset::Uniform,512);
        compare(&rule,Some(r),seed,Reset::Word(1),2048);
        compare(&rule,Some(r),seed,Reset::Word(2),2048);
        compare(&rule,Some(r),seed,Reset::Next,2048);
    }
    #[test]
    fn full_mode_equals_ant(bits in 0u32..256, n in 2usize..=8, seed in 0u64..1000) {
        let rule = Rule::from_bits(bits,n);
        let lay = Layout { w:32,h:64,wrap:true };
        let pre = |x:usize,y:usize| if y>=32 { Some((mix64(seed ^ ((x as u64)<<32 | y as u64)) % n as u64) as u8) } else { None };
        let clock = Clock::Boundary { warmup:32,reads:256,max_steps:8192 };
        let full = LocalMemoryAnt::new(&rule,lay,(16,32),None,seed,Reset::Uniform);
        let stats = full.exposure.clone();
        let a = er::trace_clock(full,Some(&rule),lay,&pre,(16,32,0),clock,|w,c,h|w.prepare(c,h), |_|{});
        let b = er::trace_clock(AntWalker::new(&rule),Some(&rule),lay,&pre,(16,32,0),clock,|_,_,_|{}, |_|{});
        prop_assert_eq!(a,b);
        prop_assert_eq!(stats.resets.get(),0);
    }
}

#[test]
fn known_loop_and_no_eviction_controls() {
    // In sixteen steps any two read sites are at most sixteen cells apart;
    // radius 31 must preserve the complete phase history on this path.
    let rule: Rule = "RRLRLL".parse().unwrap();
    let lay = Layout {
        w: 128,
        h: 128,
        wrap: false,
    };
    let pre = |x, y| if (x, y) == (64, 64) { Some(0) } else { None };
    let w = LocalMemoryAnt::new(&rule, lay, (64, 64), Some(31), 3, Reset::Uniform);
    let stats = w.exposure.clone();
    let clock = Clock::Boundary {
        warmup: 0,
        reads: 16,
        max_steps: 16,
    };
    let actual = er::trace_clock(
        w,
        None,
        lay,
        &pre,
        (64, 64, 0),
        clock,
        |w, c, h| w.prepare(c, h),
        |_| {},
    );
    let expected = er::trace_clock(
        AntWalker::new(&rule),
        None,
        lay,
        &pre,
        (64, 64, 0),
        clock,
        |_, _, _| {},
        |_| {},
    );
    assert_eq!(actual, expected);
    assert_eq!(stats.resets.get(), 0);
    for rs in ["RR", "LL"] {
        let rule: Rule = rs.parse().unwrap();
        let lay = Layout {
            w: 32,
            h: 32,
            wrap: false,
        };
        let pre = |x, y| if (x, y) == (16, 16) { Some(0) } else { None };
        for r in [0, 1, 4] {
            let w = LocalMemoryAnt::new(&rule, lay, (16, 16), Some(r), 99, Reset::Uniform);
            let exposure = w.exposure.clone();
            let s = er::trace_clock(
                w,
                Some(&rule),
                lay,
                &pre,
                (16, 16, 0),
                Clock::Boundary {
                    warmup: 64,
                    reads: 128,
                    max_steps: 256,
                },
                |w, c, h| w.prepare(c, h),
                |e| {
                    assert_eq!(e.state.lag, 1);
                    assert!(!e.born);
                },
            );
            assert!(s.window_complete);
            assert_eq!(s.boundary_reads, 128);
            assert_eq!(s.births, 0);
            assert_eq!(s.lag, [0, 128, 0, 0]);
            if r >= 1 {
                assert_eq!(exposure.resets.get(), 0);
            } else {
                assert_eq!(exposure.resets.get(), 188);
            }
        }
    }
}

#[test]
fn boundary_clock_equals_offline_prefix_selection() {
    let rule: Rule = "RRLRLL".parse().unwrap();
    let lay = Layout {
        w: 128,
        h: 128,
        wrap: false,
    };
    let pre = |x, y| if (x, y) == (64, 64) { Some(0) } else { None };
    let mut stream = Vec::new();
    er::trace_clock(
        AntWalker::new(&rule),
        Some(&rule),
        lay,
        &pre,
        (64, 64, 0),
        Clock::Boundary {
            warmup: 0,
            reads: u64::MAX,
            max_steps: 4096,
        },
        |_, _, _| {},
        |e| stream.push(e),
    );
    for warmup in [0usize, 3, 32] {
        for count in [1usize, 4, 64] {
            assert!(stream.len() >= warmup + count);
            let expected = &stream[warmup..warmup + count];
            let mut got = Vec::new();
            let s = er::trace_clock(
                AntWalker::new(&rule),
                Some(&rule),
                lay,
                &pre,
                (64, 64, 0),
                Clock::Boundary {
                    warmup: warmup as u64,
                    reads: count as u64,
                    max_steps: 4096,
                },
                |_, _, _| {},
                |e| got.push(e),
            );
            assert_eq!(got, expected);
            let start = if warmup == 0 { 0 } else { stream[warmup - 1].t };
            let end = expected.last().unwrap().t;
            assert_eq!(s.start_step, start);
            assert_eq!(s.steps, end);
            assert_eq!(s.window_steps, end - start);
            assert!(s.window_complete);
            assert_eq!(
                s.area_t0,
                1 + stream[..warmup].iter().filter(|e| e.born).count() as u64
            );
            assert_eq!(
                s.area,
                1 + stream[..warmup + count].iter().filter(|e| e.born).count() as u64
            );
            let mut lag = [0u64; 4];
            for e in expected {
                lag[e.state.lag as usize] += 1;
            }
            assert_eq!(s.lag, lag);
            assert_eq!(s.births, expected.iter().filter(|e| e.born).count() as u64);
            let mut chain = vec![0u64; 32];
            let mut run = 0usize;
            let mut last = start;
            for e in expected {
                if e.t > last + 1 && run > 0 {
                    chain[run.min(31)] += 1;
                    run = 0;
                }
                if e.born {
                    run += 1;
                } else if run > 0 {
                    chain[run.min(31)] += 1;
                    run = 0;
                }
                last = e.t;
            }
            if run > 0 {
                chain[run.min(31)] += 1;
            }
            assert_eq!(s.chains, chain);
            assert_eq!(s.right_chain_censored, expected.last().unwrap().born);
            let left = warmup > 0
                && stream[warmup - 1].born
                && expected[0].born
                && expected[0].t == start + 1;
            assert_eq!(s.left_chain_censored, left);
        }
    }
    let s = er::trace_clock(
        AntWalker::new(&rule),
        Some(&rule),
        lay,
        &pre,
        (64, 64, 0),
        Clock::Boundary {
            warmup: 10000,
            reads: 64,
            max_steps: 16,
        },
        |_, _, _| {},
        |_| {},
    );
    assert!(!s.window_complete);
    assert_eq!(s.boundary_reads, 0);
    assert_eq!(s.window_steps, 0);
}
