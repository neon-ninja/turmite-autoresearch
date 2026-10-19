//! Independent sparse path oracle, including pre-read patch features and window edges.
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::edge_renewal::{self as er, Event, State};
use turmite::edge_trace::{self as et, Layout};
use turmite::walkers::{AntWalker, RandomWalk, Rotor, Walker, mix64};
use turmite::{DIRS, Rule};

fn oracle<W: Walker>(
    mut walker: W,
    rule: Option<&Rule>,
    lay: Layout,
    prefill: &dyn Fn(usize, usize) -> Option<u8>,
    start: (usize, usize, u8),
    log2: u32,
) -> Vec<Event> {
    let norm = |(x, y): (i64, i64)| {
        if lay.wrap {
            (x.rem_euclid(lay.w as i64), y.rem_euclid(lay.h as i64))
        } else {
            (x, y)
        }
    };
    let mut colours = HashMap::new();
    let mut visited = HashSet::new();
    for y in 0..lay.h {
        for x in 0..lay.w {
            let init = if W::RANDOM_INIT {
                walker.initial_cell(x as i64 - (lay.w / 2) as i64, y as i64 - (lay.h / 2) as i64)
            } else {
                0
            };
            let c = prefill(x, y);
            if c.is_some() {
                visited.insert((x as i64, y as i64));
            }
            colours.insert(
                (x as i64, y as i64),
                c.filter(|&v| v != u8::MAX).unwrap_or(init),
            );
        }
    }
    let mut pos = (start.0 as i64, start.1 as i64);
    let mut heading = start.2 as usize;
    let mut last = HashMap::new();
    let mut chain = 0;
    let mut events = Vec::new();
    for t in 1..=(1u64 << log2) {
        let colour = colours[&pos];
        let boundary = DIRS
            .iter()
            .any(|&(dx, dy)| !visited.contains(&norm((pos.0 + dx as i64, pos.1 + dy as i64))));
        let mut s = State {
            colour,
            chain_before: chain.min(4),
            ..State::default()
        };
        if let Some(prev) = last.get(&pos) {
            let dt = t - prev;
            s.lag = if dt == 4 {
                1
            } else if dt <= 16 {
                2
            } else {
                3
            };
        }
        let reflect = rule.is_some_and(|r| r.0[0].delta() == 3);
        if let Some(r) = rule {
            for j in 0..8 {
                if r.0[(colour as usize + j) % r.len()] == r.0[0] {
                    s.next8 += 1 << j;
                }
            }
        }
        // Rotate each absolute neighbour into the heading frame independently
        // of the dense implementation's offset construction.
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let q = norm((pos.0 + dx, pos.1 + dy));
                if !visited.contains(&q) {
                    continue;
                }
                let (mut rx, fy) = match heading {
                    0 => (dx, -dy),
                    1 => (dy, dx),
                    2 => (-dx, dy),
                    _ => (-dy, -dx),
                };
                if reflect {
                    rx = -rx;
                }
                let k = match (rx, fy) {
                    (0, 1) => 0,
                    (1, 1) => 1,
                    (1, 0) => 2,
                    (1, -1) => 3,
                    (0, -1) => 4,
                    (-1, -1) => 5,
                    (-1, 0) => 6,
                    (-1, 1) => 7,
                    _ => unreachable!(),
                };
                let digit = if rule.is_some_and(|r| r.0[colours[&q] as usize] != r.0[0]) {
                    2
                } else {
                    1
                };
                s.patch += digit << (k * 2);
                if k % 2 == 0 {
                    s.visited4 += 1 << (k / 2);
                }
            }
        }
        last.insert(pos, t);
        let mut c = colour;
        let mut hd = heading as u8;
        walker.step(&mut c, &mut hd);
        colours.insert(pos, c);
        heading = hd as usize;
        let d = DIRS[heading];
        pos = norm((pos.0 + d.0 as i64, pos.1 + d.1 as i64));
        let born = visited.insert(pos);
        if t > (1u64 << (log2 - 1)) && boundary {
            events.push(Event { t, state: s, born });
        }
        chain = if born { chain + 1 } else { 0 };
        if !lay.wrap
            && (pos.0 <= 2 || pos.1 <= 2 || pos.0 >= lay.w as i64 - 2 || pos.1 >= lay.h as i64 - 2)
        {
            break;
        }
    }
    events
}

fn compare<W: Walker>(
    mk: impl Fn() -> W,
    rule: Option<&Rule>,
    lay: Layout,
    prefill: &dyn Fn(usize, usize) -> Option<u8>,
    start: (usize, usize, u8),
    log2: u32,
) {
    let mut events = Vec::new();
    let summary = er::trace(mk(), rule, lay, prefill, start, log2, |e| events.push(e));
    assert_eq!(events, oracle(mk(), rule, lay, prefill, start, log2));
    let old = et::trace(mk(), lay, prefill, start, log2);
    assert_eq!(summary.boundary_reads, old.boundary_steps);
    assert_eq!(summary.births, old.births);
    assert_eq!(summary.area, old.area);
    assert_eq!(summary.area_t0, old.area_t0);
    assert_eq!(summary.perimeter, old.perimeter);
    assert_eq!(summary.escaped, old.escaped);
    assert_eq!(summary.chains, old.chain);
    assert_eq!(summary.lag.iter().sum::<u64>(), summary.boundary_reads);
    assert_eq!(
        events.iter().filter(|e| e.born).count() as u64,
        summary.births
    );
    let mut counts = [0; 4];
    for e in &events {
        counts[e.state.lag as usize] += 1;
    }
    assert_eq!(summary.lag, counts);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn band_features_and_accounting(bits in 0u32..256, n in 2usize..=8, seed in 0u64..1000, log2 in 6u32..=11) {
        let rule = Rule::from_bits(bits, n);
        let prefill = |x: usize, y: usize| if y >= 12 { Some((mix64(mix64(seed) ^ ((x as u64) << 32 | y as u64)) % n as u64) as u8) } else { None };
        compare(|| AntWalker::new(&rule), Some(&rule), Layout { w: 16, h: 24, wrap: true }, &prefill, (8,12,0), log2);
    }
    #[test]
    fn blank_ant_features_and_accounting(bits in 0u32..256, n in 2usize..=8, log2 in 6u32..=11) {
        let rule = Rule::from_bits(bits, n);
        let prefill = |x, y| if (x,y) == (32,32) { Some(u8::MAX) } else { None };
        compare(|| AntWalker::new(&rule), Some(&rule), Layout { w: 64, h: 64, wrap: false }, &prefill, (32,32,0), log2);
    }
    #[test]
    fn calibration_walkers(seed in 0u64..1000, log2 in 6u32..=11) {
        let prefill = |x, y| if (x,y) == (32,32) { Some(u8::MAX) } else { None };
        let lay = Layout { w: 64, h: 64, wrap: false };
        compare(|| Rotor { seed }, None, lay, &prefill, (32,32,0), log2);
        compare(|| RandomWalk { state: seed }, None, lay, &prefill, (32,32,0), log2);
    }
}

#[test]
fn empty_and_censored_streams_are_explicit() {
    let rule: Rule = "RR".parse().unwrap();
    // A wholly visited torus emits no events.
    let s = er::trace(
        AntWalker::new(&rule),
        Some(&rule),
        Layout {
            w: 8,
            h: 8,
            wrap: true,
        },
        &|_, _| Some(0),
        (4, 4, 0),
        5,
        |_| panic!("unexpected event"),
    );
    assert_eq!(s.boundary_reads, 0);
    assert_eq!(s.births, 0);
    assert!(!s.left_chain_censored && !s.right_chain_censored);
    let prefill = |x, y| if (x, y) == (8, 8) { Some(0) } else { None };
    // First two steps are births; the measurement starts inside that chain.
    let s = er::trace(
        AntWalker::new(&rule),
        Some(&rule),
        Layout {
            w: 16,
            h: 16,
            wrap: false,
        },
        &prefill,
        (8, 8, 0),
        1,
        |_| {},
    );
    assert!(s.left_chain_censored && s.right_chain_censored);
    assert_eq!(s.chains[1], 1);
    assert_eq!(er::lag_class(1000, 0), 0);
    assert_eq!(er::lag_class(1000, 996), 1);
    assert_eq!(er::lag_class(1000, 984), 2);
    assert_eq!(er::lag_class(1000, 983), 3);
}
