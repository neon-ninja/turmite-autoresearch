//! Finite-colour-memory pilot, boundary-event clock (no learned transition table).
//! band RULE INTERIOR_SEED full|uniform:R|zero:R|turn:R|next:R|word2:R RESET_SEED WARMUP READS MAX_LOG2
//! control rotor|rw SEED WARMUP READS MAX_LOG2
//! Deterministic JSON on stdout; no timings inside scientific output.
use turmite::Rule;
use turmite::edge_renewal::{self as er, Clock};
use turmite::edge_trace::Layout;
use turmite::local_memory::{Exposure, LocalMemoryAnt, Reset};
use turmite::walkers::{RandomWalk, Rotor, Walker, mix64};

fn run<W: Walker>(
    walker: W,
    lay: Layout,
    prefill: &dyn Fn(usize, usize) -> Option<u8>,
    start: (usize, usize, u8),
    clock: Clock,
    n: usize,
    exposure: Exposure,
    prepare: impl FnMut(&mut W, &mut u8, u8),
) {
    let mut phase_reads = vec![0u64; n];
    let mut phase_births = vec![0u64; n];
    let (mut resets, mut changes) = (0u64, 0u64);
    let mut reset_lag = [0u64; 4];
    let (mut turn_changes, mut next_changes, mut ambiguous) = (0u64, 0u64, 0u64);
    let out = er::trace_clock(walker, None, lay, prefill, start, clock, prepare, |e| {
        phase_reads[e.state.colour as usize] += 1;
        phase_births[e.state.colour as usize] += u64::from(e.born);
        resets += u64::from(exposure.last_reset.get());
        reset_lag[e.state.lag as usize] += u64::from(exposure.last_reset.get());
        changes += u64::from(exposure.last_changed.get());
        turn_changes += u64::from(exposure.last_turn_changed.get());
        next_changes += u64::from(exposure.last_next_turn_changed.get());
        ambiguous += u64::from(exposure.last_reset.get() && exposure.last_class_size.get() > 1);
    });
    assert_eq!(phase_reads.iter().sum::<u64>(), out.boundary_reads);
    assert_eq!(phase_births.iter().sum::<u64>(), out.births);
    assert_eq!(reset_lag[0], 0);
    println!(
        "{{\"steps\":{},\"start_step\":{},\"window_steps\":{},\"window_complete\":{},\"boundary_reads\":{},\"births\":{},\"lag\":{:?},\"chains\":{:?},\"left_chain_censored\":{},\"right_chain_censored\":{},\"area_t0\":{},\"area\":{},\"perimeter\":{},\"escaped\":{},\"resets_total\":{},\"changed_total\":{},\"boundary_resets\":{},\"boundary_changes\":{},\"boundary_reset_lag\":{:?},\"phase_reads\":{:?},\"phase_births\":{:?},\"boundary_turn_changes\":{},\"boundary_next_turn_changes\":{},\"boundary_ambiguous_resets\":{}}}",
        out.steps,
        out.start_step,
        out.window_steps,
        out.window_complete,
        out.boundary_reads,
        out.births,
        out.lag,
        out.chains,
        out.left_chain_censored,
        out.right_chain_censored,
        out.area_t0,
        out.area,
        out.perimeter,
        out.escaped,
        exposure.resets.get(),
        exposure.changed.get(),
        resets,
        changes,
        reset_lag,
        phase_reads,
        phase_births,
        turn_changes,
        next_changes,
        ambiguous
    );
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a[1].as_str() {
        "band" => {
            let rule: Rule = a[2].parse().unwrap();
            let interior: u64 = a[3].parse().unwrap();
            let (radius, policy) = if a[4] == "full" {
                (None, Reset::Uniform)
            } else {
                let (kind, r) = a[4].split_once(':').unwrap();
                (
                    Some(r.parse::<usize>().unwrap()),
                    match kind {
                        "uniform" => Reset::Uniform,
                        "zero" => Reset::Zero,
                        "turn" => Reset::Word(1),
                        "next" => Reset::Next,
                        "word2" => Reset::Word(2),
                        _ => panic!("unknown reset policy"),
                    },
                )
            };
            let reset_seed = a[5].parse().unwrap();
            let clock = Clock::Boundary {
                warmup: a[6].parse().unwrap(),
                reads: a[7].parse().unwrap(),
                max_steps: 1u64 << a[8].parse::<u32>().unwrap(),
            };
            let lay = Layout {
                w: 256,
                h: 4096,
                wrap: true,
            };
            let pre = |x: usize, y: usize| {
                if y < 2048 {
                    None
                } else {
                    Some(
                        (mix64(mix64(interior) ^ ((x as u64) << 32 | y as u64)) % rule.len() as u64)
                            as u8,
                    )
                }
            };
            let walker = LocalMemoryAnt::new(&rule, lay, (128, 2048), radius, reset_seed, policy);
            let stats = walker.exposure.clone();
            run(
                walker,
                lay,
                &pre,
                (128, 2048, 0),
                clock,
                rule.len(),
                stats,
                |w, c, h| w.prepare(c, h),
            );
        }
        "control" => {
            let seed = a[3].parse().unwrap();
            let clock = Clock::Boundary {
                warmup: a[4].parse().unwrap(),
                reads: a[5].parse().unwrap(),
                max_steps: 1u64 << a[6].parse::<u32>().unwrap(),
            };
            let lay = Layout {
                w: 4096,
                h: 4096,
                wrap: false,
            };
            let pre = |x, y| {
                if (x, y) == (2048, 2048) {
                    Some(u8::MAX)
                } else {
                    None
                }
            };
            match a[2].as_str() {
                "rotor" => run(
                    Rotor { seed },
                    lay,
                    &pre,
                    (2048, 2048, 0),
                    clock,
                    4,
                    Exposure::default(),
                    |_, _, _| {},
                ),
                "rw" => run(
                    RandomWalk { state: seed },
                    lay,
                    &pre,
                    (2048, 2048, 0),
                    clock,
                    4,
                    Exposure::default(),
                    |_, _, _| {},
                ),
                _ => panic!("unknown control"),
            }
        }
        _ => panic!("expected band or control"),
    }
}
