//! One run, streamed events or compact state/transition counts.
//! band RULE SEED LOG2 W H FRESH_ROWS [counts|events|summary]
//! blank RULE|rotor:SEED|rw:SEED LOG2 SIZE [counts|events|summary]
//! CSV on stdout (or summary JSON in summary mode); summary JSON always on stderr.
use std::collections::BTreeMap;
use std::io::{self, Write};
use turmite::Rule;
use turmite::edge_renewal::{self as er, Event, State, Summary};
use turmite::edge_trace::Layout;
use turmite::walkers::{AntWalker, RandomWalk, Rotor, Walker, mix64};

fn key(s: State) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        s.colour, s.next8, s.visited4, s.patch, s.lag, s.chain_before
    )
}

fn run<W: Walker>(
    walker: W,
    rule: Option<&Rule>,
    lay: Layout,
    prefill: &dyn Fn(usize, usize) -> Option<u8>,
    start: (usize, usize, u8),
    l: u32,
    mode: &str,
) -> Summary {
    assert!(["summary", "counts", "events"].contains(&mode));
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    let mut states: BTreeMap<State, (u64, u64)> = BTreeMap::new();
    // Keep the gap category and duration sum; neither is used as a source feature.
    let mut edges: BTreeMap<(State, State, u8), (u64, u64)> = BTreeMap::new();
    let mut previous: Option<Event> = None;
    if mode == "events" {
        writeln!(
            output,
            "t,colour,next8,visited4,patch,lag,chain_before,born"
        )
        .unwrap();
    }
    let summary = er::trace(walker, rule, lay, prefill, start, l, |e| {
        if mode == "events" {
            let s = e.state;
            writeln!(
                output,
                "{},{},{},{},{},{},{},{}",
                e.t,
                s.colour,
                s.next8,
                s.visited4,
                s.patch,
                s.lag,
                s.chain_before,
                u8::from(e.born)
            )
            .unwrap();
        } else if mode == "counts" {
            let count = states.entry(e.state).or_default();
            count.0 += 1;
            count.1 += u64::from(e.born);
            if let Some(p) = previous {
                let dt = e.t - p.t;
                let gap = if dt == 1 {
                    0
                } else if dt == 4 {
                    1
                } else if dt <= 16 {
                    2
                } else {
                    3
                };
                let edge = edges.entry((p.state, e.state, gap)).or_default();
                edge.0 += 1;
                edge.1 += dt;
            }
        }
        previous = Some(e);
    });
    if mode == "counts" {
        writeln!(output, "kind,source,target,gap,count,births,elapsed").unwrap();
        for (s, (count, births)) in &states {
            writeln!(output, "state,{},,,{count},{births},", key(*s)).unwrap();
        }
        for ((a, b, gap), (count, elapsed)) in &edges {
            writeln!(
                output,
                "edge,{},{},{gap},{count},,{elapsed}",
                key(*a),
                key(*b)
            )
            .unwrap();
        }
        // A terminal source has no observed next boundary event. Never close it to the first.
        if let Some(p) = previous {
            writeln!(
                output,
                "censored,{},,,1,,{}",
                key(p.state),
                summary.steps - p.t
            )
            .unwrap();
        }
    }
    output.flush().unwrap();
    summary
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let summary = match a[1].as_str() {
        "band" => {
            let r: Rule = a[2].parse().unwrap();
            let seed: u64 = a[3].parse().unwrap();
            let l = a[4].parse().unwrap();
            let (w, h, rows) = (
                a[5].parse().unwrap(),
                a[6].parse().unwrap(),
                a[7].parse::<usize>().unwrap(),
            );
            assert!(rows > 0 && rows < h);
            let prefill = |x: usize, y: usize| {
                if y < rows {
                    None
                } else {
                    Some(
                        (mix64(mix64(seed) ^ ((x as u64) << 32 | y as u64)) % r.len() as u64) as u8,
                    )
                }
            };
            run(
                AntWalker::new(&r),
                Some(&r),
                Layout { w, h, wrap: true },
                &prefill,
                (w / 2, rows, 0),
                l,
                a.get(8).map_or("counts", String::as_str),
            )
        }
        "blank" => {
            let l = a[3].parse().unwrap();
            let w: usize = a[4].parse().unwrap();
            let c = w / 2;
            let prefill = |x, y| {
                if (x, y) == (c, c) {
                    Some(u8::MAX)
                } else {
                    None
                }
            };
            let lay = Layout {
                w,
                h: w,
                wrap: false,
            };
            let mode = a.get(5).map_or("counts", String::as_str);
            if let Some(seed) = a[2].strip_prefix("rotor:") {
                run(
                    Rotor {
                        seed: seed.parse().unwrap(),
                    },
                    None,
                    lay,
                    &prefill,
                    (c, c, 0),
                    l,
                    mode,
                )
            } else if let Some(seed) = a[2].strip_prefix("rw:") {
                run(
                    RandomWalk {
                        state: seed.parse().unwrap(),
                    },
                    None,
                    lay,
                    &prefill,
                    (c, c, 0),
                    l,
                    mode,
                )
            } else {
                let r: Rule = a[2].parse().unwrap();
                run(
                    AntWalker::new(&r),
                    Some(&r),
                    lay,
                    &prefill,
                    (c, c, 0),
                    l,
                    mode,
                )
            }
        }
        _ => panic!("expected band or blank"),
    };
    let json = format!(
        "{{\"steps\":{},\"boundary_reads\":{},\"births\":{},\"lag\":{:?},\"chains\":{:?},\"left_chain_censored\":{},\"right_chain_censored\":{},\"area_t0\":{},\"area\":{},\"perimeter\":{},\"escaped\":{}}}",
        summary.steps,
        summary.boundary_reads,
        summary.births,
        summary.lag,
        summary.chains,
        summary.left_chain_censored,
        summary.right_chain_censored,
        summary.area_t0,
        summary.area,
        summary.perimeter,
        summary.escaped
    );
    eprintln!("{json}");
    if a.last().is_some_and(|s| s == "summary") {
        println!("{json}");
    }
}
