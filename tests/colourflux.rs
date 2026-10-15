//! `colourflux::flux_walker` against (a) a HashMap oracle that recomputes every field from scratch
//! for any `Walker`, (b) `probe_walker` window fields, and (c) the reference ant for the dynamics.

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::colourflux::{FluxWindow, MAXC, flux_walker};
use turmite::probe::probe_walker;
use turmite::reference::Ant;
use turmite::walkers::{AntWalker, PatchedAnt, PersistentWalk, Rotor, Walker};
use turmite::{DIRS, Rule};

fn on_boundary(seen: &HashSet<(i64, i64)>, (x, y): (i64, i64)) -> bool {
    DIRS.iter().any(|&(dx, dy)| !seen.contains(&(x + dx as i64, y + dy as i64)))
}

/// Obvious recount, one HashMap of cell colours, no incremental state.
fn brute<W: Walker>(mut w: W, min_log2: u32, max_log2: u32) -> Vec<FluxWindow> {
    let mut cells: HashMap<(i64, i64), u8> = HashMap::new();
    let mut seen: HashSet<(i64, i64)> = HashSet::from([(0, 0)]);
    let (mut pos, mut h, mut t) = ((0i64, 0i64), 0u8, 0u64);
    let mut out = vec![];
    for k in 0..max_log2 {
        let (t_start, t_end) = (1u64 << k, 1u64 << (k + 1));
        let mut f = FluxWindow {
            t_start,
            t_end,
            area: 0,
            perimeter: 0,
            new_cells: 0,
            boundary_steps: 0,
            steps_by_colour: [0; MAXC],
            boundary_by_colour: [0; MAXC],
            new_by_colour: [0; MAXC],
            hist_all: [0; MAXC],
            hist_boundary: [0; MAXC],
        };
        while t < t_end {
            let cell = cells.entry(pos).or_insert_with(|| w.initial_cell(pos.0, pos.1));
            let col = *cell as usize;
            f.steps_by_colour[col] += 1;
            let was_b = on_boundary(&seen, pos);
            if was_b {
                f.boundary_steps += 1;
                f.boundary_by_colour[col] += 1;
            }
            w.step(cell, &mut h);
            let (dx, dy) = DIRS[h as usize];
            pos = (pos.0 + dx as i64, pos.1 + dy as i64);
            t += 1;
            if seen.insert(pos) {
                f.new_cells += 1;
                f.new_by_colour[col] += 1;
            }
        }
        if k >= min_log2 {
            f.area = seen.len() as u64;
            for &p in &seen {
                let col = cells.get(&p).copied().unwrap_or_else(|| w.initial_cell(p.0, p.1)) as usize;
                f.hist_all[col] += 1;
                if on_boundary(&seen, p) {
                    f.perimeter += 1;
                    f.hist_boundary[col] += 1;
                }
            }
            out.push(f);
        }
    }
    out
}

fn sums(f: &FluxWindow) -> [u64; 5] {
    [
        f.steps_by_colour.iter().sum(),
        f.boundary_by_colour.iter().sum(),
        f.new_by_colour.iter().sum(),
        f.hist_all.iter().sum(),
        f.hist_boundary.iter().sum(),
    ]
}

fn check_identities<W: Walker>(mk: impl Fn() -> W, size: usize, max_log2: u32) -> Result<(), TestCaseError> {
    let fl = flux_walker(mk(), size, 2, max_log2);
    prop_assume!(!fl.escaped);
    let pr = probe_walker(mk(), size, 2, max_log2);
    prop_assert_eq!(fl.windows.len(), pr.windows.len());
    for (f, p) in fl.windows.iter().zip(&pr.windows) {
        let s = sums(f);
        prop_assert_eq!(s[0], f.t_end - f.t_start);
        prop_assert_eq!(s[1], f.boundary_steps);
        prop_assert_eq!(s[2], f.new_cells);
        prop_assert_eq!(s[3], f.area);
        prop_assert_eq!(s[4], f.perimeter);
        prop_assert_eq!((f.area, f.perimeter, f.new_cells, f.boundary_steps), (p.area, p.perimeter, p.new_cells, p.boundary_steps));
    }
    let bf = brute(mk(), 2, max_log2);
    prop_assert_eq!(&fl.windows, &bf);
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]

    #[test]
    fn plain_ant_matches_oracle_and_probe(bits in 0u32..(1 << 9), n in 2usize..=9, k in 6u32..12) {
        let rule = Rule::from_bits(bits, n);
        check_identities(|| AntWalker::new(&rule), 512, k)?;
    }

    #[test]
    fn patched_ant_matches_oracle_and_probe(bits in 0u32..(1 << 9), n in 2usize..=9, seed in 0u64..1000, k in 6u32..12) {
        let rule = Rule::from_bits(bits, n);
        check_identities(|| PatchedAnt::new(&rule, seed, 2), 512, k)?;
    }

    #[test]
    fn rotor_matches_oracle_and_probe(seed in 0u64..1000, k in 6u32..12) {
        check_identities(|| Rotor { seed }, 512, k)?;
    }

    #[test]
    fn persistent_walk_matches_oracle_and_probe(seed in 0u64..1000, k in 6u32..11) {
        check_identities(|| PersistentWalk::new(seed, 0.5), 512, k)?;
    }

    /// The dynamics themselves (colour before each step) against the reference ant.
    #[test]
    fn colour_counts_match_reference_ant(bits in 0u32..(1 << 8), n in 2usize..=8, k in 5u32..11) {
        let rule = Rule::from_bits(bits, n);
        let fl = flux_walker(AntWalker::new(&rule), 512, 1, k);
        prop_assume!(!fl.escaped);
        let mut ant = Ant::new(rule);
        // Window k = 0 consumes steps 1 and 2 (probe's convention), so recorded windows k >= 1 are contiguous from step 3.
        ant.step();
        ant.step();
        for f in &fl.windows {
            let mut counts = [0u64; MAXC];
            while ant.steps < f.t_end {
                counts[ant.colour_at(ant.pos) as usize] += 1;
                ant.step();
            }
            prop_assert_eq!(counts, f.steps_by_colour);
        }
    }
}

#[test]
fn rl_first_steps_by_colour() {
    // Langton's ant on a blank board: step 1 starts on colour 0 and is a boundary cell that enters fresh ground.
    let fl = flux_walker(AntWalker::new(&"RL".parse().unwrap()), 64, 1, 4);
    assert!(!fl.escaped);
    let w = &fl.windows[0]; // k = 1, steps 3..4
    assert_eq!(w.t_end - w.t_start, 2);
    assert_eq!(w.steps_by_colour.iter().sum::<u64>(), 2);
}
