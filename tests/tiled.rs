//! The tiled backend (src/tiled.rs) against `reference::Ant` (the oracle), `fast::run` and `lean::run_lean`.
//! Whole logical states are compared, not just hashes; tile size, cache bound and inactive format must not matter.
use proptest::prelude::*;
use std::collections::BTreeMap;
use turmite::fast::{self, Config, Grid, Stop};
use turmite::lean::{Ckpt, LeanGrid, highway_onset_flat, run_lean, run_lean_ckpt};
use turmite::reference::Ant;
use turmite::tiled::*;
use turmite::{Rule, Turn};

fn cfg(max_steps: u64) -> Config {
    Config { max_steps, checkpoint: 1_000, max_period: 5_000, history: 20_000 }
}

fn debug(r: &fast::RunResult) -> String {
    format!("{r:?}")
}

/// Rule with `n` colours from a bit pattern wider than 32 bits (bit i set: colour i turns R).
fn rule_n(bits: u64, n: usize) -> Rule {
    Rule((0..n).map(|i| if (bits.rotate_left(i as u32 / 7) >> (i % 64)) & 1 == 1 { Turn::R } else { Turn::L }).collect())
}

fn tcfg(tile_log2: u32, cache: Option<usize>, inactive: Inactive) -> TileConfig {
    TileConfig { tile_log2, cache_tiles: cache, inactive }
}

/// The oracle's complete state after `steps`: lean-encoded byte for every cell that is not blank (colour, bit 7 = the ant
/// has stood on it before this step, which is when lean writes it).
fn oracle_state(rule: &Rule, steps: u64) -> (BTreeMap<(i64, i64), u8>, (i32, i32), u8) {
    let mut a = Ant::new(rule.clone());
    let mut touched = std::collections::BTreeSet::new();
    for _ in 0..steps {
        touched.insert(a.pos);
        a.step();
    }
    let mut m = BTreeMap::new();
    for p in touched {
        m.insert((p.1 as i64, p.0 as i64), a.colour_at(p) | 0x80); // keyed (y, x) like logical_cells' order
    }
    (m, a.pos, a.heading)
}

fn tiled_state(g: &TiledGrid) -> BTreeMap<(i64, i64), u8> {
    g.logical_cells().into_iter().map(|((x, y), b)| ((y, x), b)).collect()
}

fn run_checked(rule: &Rule, size: usize, steps: u64, tc: TileConfig) -> (fast::RunResult, TiledGrid) {
    let mut g = TiledGrid::new(size, tc).unwrap();
    let r = run_tiled(rule, &mut g, &cfg(steps));
    g.check_invariants().unwrap();
    (r, g)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(120))]

    /// Full state against the oracle: every colour, every touched flag, position, heading, step count, escape timing.
    #[test]
    fn equals_reference_state(bits in any::<u64>(), n in 1usize..=40, size in 1usize..=45, steps in 0u64..6_000, tl in 2u32..=5, cap in 2usize..=6, packed in any::<bool>()) {
        let rule = rule_n(bits, n);
        let inactive = if packed { Inactive::Packed } else { Inactive::Raw };
        let (r, g) = run_checked(&rule, size, steps, tcfg(tl, Some(cap), inactive));
        let (want, pos, heading) = oracle_state(&rule, r.steps);
        prop_assert_eq!(r.pos, pos);
        prop_assert_eq!(r.heading, heading);
        if r.stop == Stop::Budget {
            prop_assert_eq!(r.steps, steps);
        } else {
            prop_assert!(r.steps <= steps);
        }
        // Compare the state the run left in the tiles (the trail replay has restored it).
        prop_assert_eq!(tiled_state(&g), want);
    }

    /// Same RunResult as the dense lean ant, every field, and the same cells, for any tile size and cache bound.
    #[test]
    fn equals_lean(bits in any::<u64>(), n in 2usize..=12, size in 1usize..=70, steps in 1u64..40_000, tl in 2u32..=6, cap in 2usize..=9, packed in any::<bool>(), hist in 100usize..20_000) {
        let rule = rule_n(bits, n);
        let c = Config { max_steps: steps, checkpoint: 1_000, max_period: 5_000, history: hist };
        let mut lg = LeanGrid::new(size);
        let want = run_lean(&rule, &mut lg, &c);
        let inactive = if packed { Inactive::Packed } else { Inactive::Raw };
        let mut g = TiledGrid::new(size, tcfg(tl, Some(cap), inactive)).unwrap();
        let got = run_tiled(&rule, &mut g, &c);
        prop_assert_eq!(debug(&got), debug(&want));
        g.check_invariants().unwrap();
        let ext = (size / 2) as i64;
        for y in -ext..size as i64 - ext {
            for x in -ext..size as i64 - ext {
                prop_assert_eq!(g.byte_at(x, y), lg.cell_byte(x, y), "cell ({}, {})", x, y);
            }
        }
    }

    /// Colours agree with `fast::run`'s grid, which is a separate dense implementation.
    #[test]
    fn colours_equal_fast(bits in 1u32..4096, n in 2usize..12, size in 24usize..120, steps in 1u64..50_000) {
        let rule = Rule::from_bits(bits, n);
        let mut fg = Grid::new(size);
        let want = fast::run(&rule, &mut fg, &cfg(steps));
        let (got, g) = run_checked(&rule, size, steps, tcfg(4, Some(3), Inactive::Packed));
        // fast::run and lean agree except when the budget ends exactly on the escaping step (tested below), so skip that case.
        prop_assume!(!(want.stop == Stop::Escaped && want.steps == steps));
        prop_assert_eq!((got.steps, got.stop, got.pos, got.heading, got.extent), (want.steps, want.stop, want.pos, want.heading, want.extent));
        let c = (size / 2) as i64;
        for y in -c..size as i64 - c {
            for x in -c..size as i64 - c {
                prop_assert_eq!(g.byte_at(x, y).unwrap() & 127, fg.get(x as i32, y as i32));
            }
        }
    }

    /// Tile size, cache bound and inactive format change cost, never the result or the final state.
    #[test]
    fn configuration_independent(bits in any::<u64>(), n in 2usize..=9, size in 8usize..=90, steps in 1u64..30_000) {
        let rule = rule_n(bits, n);
        let mut base: Option<(String, BTreeMap<(i64, i64), u8>)> = None;
        for tl in [2u32, 3, 5, 7] {
            for cache in [Some(2), Some(3), Some(8), None] {
                for inactive in [Inactive::Raw, Inactive::Packed] {
                    let (r, g) = run_checked(&rule, size, steps, tcfg(tl, cache, inactive));
                    let cur = (debug(&r), tiled_state(&g));
                    match &base {
                        None => base = Some(cur),
                        Some(b) => prop_assert_eq!(&cur, b, "tile_log2 {} cache {:?} {:?}", tl, cache, inactive),
                    }
                }
            }
        }
    }

    /// One grid reused for many rules (different n, so different packing widths) behaves like a fresh grid each time.
    #[test]
    fn reuse_across_rules(seeds in proptest::collection::vec((any::<u64>(), 1usize..=70, 100u64..8_000), 2..6), size in 16usize..80, packed in any::<bool>()) {
        let inactive = if packed { Inactive::Packed } else { Inactive::Raw };
        let mut g = TiledGrid::new(size, tcfg(3, Some(4), inactive)).unwrap();
        for (bits, n, steps) in seeds {
            let rule = rule_n(bits, n);
            let got = run_tiled(&rule, &mut g, &cfg(steps));
            g.check_invariants().unwrap();
            let (fresh, fg) = run_checked(&rule, size, steps, tcfg(3, Some(4), inactive));
            prop_assert_eq!(debug(&got), debug(&fresh));
            prop_assert_eq!(tiled_state(&g), tiled_state(&fg));
            if n >= 2 {
                prop_assert_eq!(debug(&got), debug(&run_lean(&rule, &mut LeanGrid::new(size), &cfg(steps))));
            }
        }
    }

    /// Packing round trip for every colour count, including the widths where the field straddles bytes.
    #[test]
    fn pack_round_trip(n in 1usize..=99, tl in 2u32..=6, seed in any::<u64>()) {
        let t = 1usize << tl;
        let w = colour_width(n);
        let mut s = seed;
        let cells: Vec<u8> = (0..t * t)
            .map(|_| {
                s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let col = ((s >> 33) as usize % n) as u8;
                col | if (s >> 20) & 1 == 1 { 0x80 } else { 0 }
            })
            .collect();
        let p = pack_cells(&cells, t, w);
        prop_assert_eq!(p.len(), packed_len(t, w));
        prop_assert_eq!(unpack_cells(&p, t, w, n).unwrap(), cells);
    }
}

#[test]
fn pack_rejects_damage() {
    let t = 8;
    for n in [2usize, 3, 5, 100.min(99)] {
        let w = colour_width(n);
        let cells = vec![(n - 1) as u8; t * t];
        let p = pack_cells(&cells, t, w);
        assert!(unpack_cells(&p[..p.len() - 1], t, w, n).is_err(), "truncated n={n}");
        let mut longer = p.clone();
        longer.push(0);
        assert!(unpack_cells(&longer, t, w, n).is_err(), "padded n={n}");
        // the same payload read as a rule with fewer colours has an out-of-range colour
        if n > 2 {
            assert!(unpack_cells(&p, t, w, n - 1).is_err(), "colour range n={n}");
        }
    }
    assert_eq!(colour_width(1), 0);
    assert_eq!(packed_len(8, 0), 8, "one colour: touched plane only");
}

#[test]
fn packing_width_transitions_and_raw_fallback() {
    // Packed is smaller than raw up to 64 colours and falls back to raw from 65 (7 colour bits + 1 touched bit = 8).
    for (n, packs) in [(1, true), (2, true), (3, true), (4, true), (5, true), (8, true), (9, true), (16, true), (17, true), (32, true), (33, true), (64, true), (65, false), (99, false)] {
        let w = colour_width(n);
        assert_eq!(packed_len(64, w) < 64 * 64, packs, "n={n} w={w}");
    }
    // the grid really stores the 65-colour rule raw while in Packed mode, and the run is still exact
    let rule = rule_n(0x1234_5678_9abc_def1, 70);
    let (r, g) = run_checked(&rule, 60, 30_000, tcfg(4, Some(2), Inactive::Packed));
    let m = g.memory();
    assert_eq!(m.tiles_packed, 0);
    assert!(m.tiles_raw_inactive > 0);
    assert_eq!(debug(&r), debug(&run_lean(&rule, &mut LeanGrid::new(60), &cfg(30_000))));
}

#[test]
fn one_colour_rule() {
    // A single colour: colour 0 forever, so only the touched flag distinguishes visited cells (the ant spins in a 2x2 square).
    for turn in [Turn::R, Turn::L] {
        let rule = Rule(vec![turn]);
        for tl in [2u32, 4] {
            let (r, g) = run_checked(&rule, 9, 1_001, tcfg(tl, Some(2), Inactive::Packed));
            let want = oracle_state(&rule, 1_001);
            assert_eq!(r.pos, want.1);
            assert_eq!(tiled_state(&g), want.0);
            assert_eq!(g.logical_cells().len(), 4);
            assert_eq!(r.extent, (2, 2));
        }
    }
}

#[test]
fn visited_zero_colour_cells_are_not_forgotten() {
    // RL: after one pass cells return to colour 0 yet are visited; they must survive eviction as touched.
    let rule: Rule = "RLR".parse().unwrap();
    let (_, g) = run_checked(&rule, 64, 9_000, tcfg(2, Some(2), Inactive::Packed));
    let zeros = g.logical_cells().iter().filter(|(_, b)| b & 127 == 0 && b & 0x80 != 0).count();
    assert!(zeros > 0, "expected visited cells with colour 0");
    assert_eq!(tiled_state(&g), oracle_state(&rule, 9_000).0);
    assert!(g.stats().evictions > 100, "tiny cache should have been thrashed: {:?}", g.stats());
}

#[test]
fn escape_through_every_edge_has_lean_timing() {
    // Long runs on small boards, odd and even: the escape step, the ring position and the extent must equal lean's.
    let mut sides = std::collections::BTreeSet::new();
    for size in [1usize, 2, 3, 5, 8, 9, 15, 16, 17, 31] {
        for bits in 0u32..64 {
            for n in 2usize..=5 {
                let rule = Rule::from_bits(bits, n);
                let c = cfg(20_000);
                let want = run_lean(&rule, &mut LeanGrid::new(size), &c);
                for (tl, cap) in [(2, 2), (3, 3), (4, 2)] {
                    let mut g = TiledGrid::new(size, tcfg(tl, Some(cap), Inactive::Packed)).unwrap();
                    let got = run_tiled(&rule, &mut g, &c);
                    assert_eq!(debug(&got), debug(&want), "{rule} size {size} T=2^{tl}");
                }
                if want.stop == Stop::Escaped {
                    let half = (size / 2) as i32;
                    let (x, y) = want.pos;
                    sides.insert(if x < -half { "west" } else if x >= size as i32 - half { "east" } else if y < -half { "north" } else { "south" });
                }
            }
        }
    }
    assert_eq!(sides.len(), 4, "every board edge exercised, got {sides:?}");
}

#[test]
fn budget_ending_on_the_escaping_step_is_lean_compatible() {
    // When max_steps is exactly the step that leaves the board, lean (and so the tiled backend) reports Budget with the ring
    // position and an extent that counts the ring cell (wrong by one, or wrapped to 0), whereas fast::run reports Escaped. This
    // pins that existing quirk; it is not endorsed and not fixed here, because fixing it would change sweep CSVs.
    let rule: Rule = "RL".parse().unwrap();
    for size in [8usize, 9, 10] {
        let escape = fast::run(&rule, &mut Grid::new(size), &cfg(100_000)).steps;
        for m in [escape - 1, escape, escape + 1] {
            let want = run_lean(&rule, &mut LeanGrid::new(size), &cfg(m));
            let (got, _) = run_checked(&rule, size, m, tcfg(2, Some(2), Inactive::Packed));
            assert_eq!(debug(&got), debug(&want), "size {size} max {m}");
        }
        let at = run_lean(&rule, &mut LeanGrid::new(size), &cfg(escape));
        assert_eq!(at.stop, Stop::Budget);
        assert_ne!(at.extent, fast::run(&rule, &mut Grid::new(size), &cfg(escape)).extent);
    }
}

#[test]
fn langton_highway_across_many_tiles() {
    let rule: Rule = "RL".parse().unwrap();
    let c = Config { max_steps: 200_000, ..Config::default() };
    let want = run_lean(&rule, &mut LeanGrid::new(512), &c);
    assert!(want.highway.is_some());
    for (tl, cap, inactive) in [(3, Some(2), Inactive::Packed), (6, Some(4), Inactive::Packed), (6, None, Inactive::Raw), (5, Some(2), Inactive::Raw)] {
        let (got, g) = run_checked(&rule, 512, 200_000, tcfg(tl, cap, inactive));
        assert_eq!(debug(&got), debug(&want));
        if cap.is_some() {
            assert!(g.stats().evictions > 0);
        }
    }
}

#[test]
fn onset_matches_flat_and_published() {
    for r in ["RL", "RRL", "RLRL", "RRLL"] {
        let rule: Rule = r.parse().unwrap();
        let c = Config { max_steps: 400_000, ..Config::default() };
        let res = run_lean(&rule, &mut LeanGrid::new(1024), &c);
        if let Some(hw) = res.highway {
            let mut g = TiledGrid::new(1024, tcfg(5, Some(3), Inactive::Packed)).unwrap();
            assert_eq!(highway_onset_tiled(&rule, hw, res.steps, &mut g), highway_onset_flat(&rule, hw, res.steps, 1024), "{r}");
            g.check_invariants().unwrap();
        }
    }
    let rule: Rule = "RL".parse().unwrap();
    let res = run_lean(&rule, &mut LeanGrid::new(1024), &Config { max_steps: 400_000, ..Config::default() });
    let mut g = TiledGrid::new(1024, TileConfig::default()).unwrap();
    assert_eq!(highway_onset_tiled(&rule, res.highway.unwrap(), res.steps, &mut g), 9976);
}

#[test]
fn config_is_validated() {
    let bad = |size: usize, tc: TileConfig| matches!(TiledGrid::new(size, tc), Err(TileError::BadConfig(_)));
    assert!(bad(0, TileConfig::default()));
    assert!(bad(1 << 31, TileConfig::default()));
    assert!(bad(64, tcfg(1, None, Inactive::Raw)));
    assert!(bad(64, tcfg(13, None, Inactive::Raw)));
    assert!(bad(64, tcfg(4, Some(1), Inactive::Raw)));
    assert!(bad(64, tcfg(4, Some(0), Inactive::Raw)));
    assert!(TiledGrid::new(64, tcfg(4, Some(MIN_CACHE_TILES), Inactive::Raw)).is_ok());
    // a huge board costs nothing until it is used
    let mut g = TiledGrid::new(1 << 28, tcfg(6, Some(8), Inactive::Packed)).unwrap();
    let r = run_tiled(&"RL".parse().unwrap(), &mut g, &cfg(5_000));
    assert_eq!(r.stop, Stop::Budget);
    assert!(g.memory().total_bytes() < 4 << 20);
}

fn tmpdir(tag: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("turmite-tiled-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Stop, extend and resume equals one straight run (and the dense lean run); the checkpoint bytes do not depend on the
    /// cache bound, the inactive format or the eviction history.
    #[test]
    fn checkpoint_resume_exact(bits in any::<u64>(), n in 2usize..=9, first in 1u64..30_000, total in 30_000u64..90_000, every in 1u64..20_000, size in 60usize..260) {
        let rule = rule_n(bits, n);
        let dir = tmpdir(&format!("{bits:x}-{n}-{first}-{total}-{every}-{size}"));
        let ck = |name: &str| Ckpt { path: dir.join(name), every };
        let straight = run_lean(&rule, &mut LeanGrid::new(size), &cfg(total));
        let mut files = vec![];
        for (i, (cache, inactive)) in [(Some(2), Inactive::Packed), (None, Inactive::Raw), (Some(5), Inactive::Raw)].into_iter().enumerate() {
            let name = format!("c{i}.tkpt");
            let mut g1 = TiledGrid::new(size, tcfg(3, cache, inactive)).unwrap();
            run_tiled_ckpt(&rule, &mut g1, &cfg(first), Some(&ck(&name))).unwrap();
            files.push(std::fs::read(dir.join(&name)).ok()); // absent when the run escaped before any save
            // resume in a different configuration than the one that wrote the file
            let mut g2 = TiledGrid::new(size, tcfg(3, Some(3 + i), if i % 2 == 0 { Inactive::Raw } else { Inactive::Packed })).unwrap();
            let ext = run_tiled_ckpt(&rule, &mut g2, &cfg(total), Some(&ck(&name))).unwrap();
            prop_assert_eq!(debug(&ext), debug(&straight));
        }
        prop_assert_eq!(&files[0], &files[1]);
        prop_assert_eq!(&files[0], &files[2]);
        std::fs::remove_dir_all(&dir).ok();
    }
}

#[test]
fn checkpoint_rejects_bad_files_clearly() {
    let dir = tmpdir("bad");
    let p = dir.join("x.tkpt");
    let (a, b): (Rule, Rule) = ("RLR".parse().unwrap(), "RRL".parse().unwrap());
    let ck = Ckpt { path: p.clone(), every: 10_000 };
    let mut g = TiledGrid::new(256, tcfg(4, Some(4), Inactive::Packed)).unwrap();
    run_tiled_ckpt(&a, &mut g, &cfg(50_000), Some(&ck)).unwrap();
    let good = std::fs::read(&p).unwrap();

    // another rule, board or tile size: incompatible, and the file is left alone
    assert!(matches!(run_tiled_ckpt(&b, &mut g, &cfg(80_000), Some(&ck)), Err(TileError::Incompatible(_))));
    let mut g2 = TiledGrid::new(250, tcfg(4, Some(4), Inactive::Packed)).unwrap();
    assert!(matches!(run_tiled_ckpt(&a, &mut g2, &cfg(80_000), Some(&ck)), Err(TileError::Incompatible(_))));
    let mut g3 = TiledGrid::new(256, tcfg(5, Some(4), Inactive::Packed)).unwrap();
    assert!(matches!(run_tiled_ckpt(&a, &mut g3, &cfg(80_000), Some(&ck)), Err(TileError::Incompatible(_))));
    assert_eq!(std::fs::read(&p).unwrap(), good);

    // truncated, bit-flipped, wrong magic, future version, empty
    let mut cases: Vec<(&str, Vec<u8>)> = vec![("truncated", good[..good.len() / 2].to_vec()), ("short", good[..6].to_vec()), ("empty", vec![])];
    let mut flipped = good.clone();
    let mid = flipped.len() / 2;
    flipped[mid] ^= 0x10;
    cases.push(("flipped", flipped));
    let mut magic = good.clone();
    magic[0] = b'X';
    cases.push(("magic", magic));
    for (name, bytes) in cases {
        std::fs::write(&p, &bytes).unwrap();
        let e = run_tiled_ckpt(&a, &mut g, &cfg(80_000), Some(&ck)).unwrap_err();
        assert!(matches!(e, TileError::Corrupt(_)), "{name}: {e}");
        assert!(!e.to_string().is_empty());
    }
    // a future version with a valid checksum is incompatible, not corrupt
    let mut v = good[..good.len() - 8].to_vec();
    v[4] = 99;
    let sum = v.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3));
    v.extend_from_slice(&sum.to_le_bytes());
    std::fs::write(&p, &v).unwrap();
    assert!(matches!(run_tiled_ckpt(&a, &mut g, &cfg(80_000), Some(&ck)), Err(TileError::Incompatible(_))));

    // a checkpoint beyond the budget is ignored like lean's, and the run restarts
    std::fs::write(&p, &good).unwrap();
    let r = run_tiled_ckpt(&a, &mut g, &cfg(20_000), Some(&ck)).unwrap();
    assert_eq!(debug(&r), debug(&run_lean(&a, &mut LeanGrid::new(256), &cfg(20_000))));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn tiled_checkpoint_does_not_collide_with_dense() {
    let dir = tmpdir("collide");
    let rule: Rule = "RLR".parse().unwrap();
    let ck = Ckpt { path: dir.join("same-name"), every: 5_000 };
    run_lean_ckpt(&rule, &mut LeanGrid::new(128), &cfg(20_000), Some(&ck));
    // the dense file is not a tiled checkpoint: rejected, never misread
    let mut g = TiledGrid::new(128, TileConfig::default()).unwrap();
    assert!(matches!(run_tiled_ckpt(&rule, &mut g, &cfg(30_000), Some(&ck)), Err(TileError::Corrupt(_))));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn resume_after_interruption_on_a_roomy_board() {
    // Deterministic companion of `checkpoint_resume_exact`, whose small boards often escape before the first save: here every
    // run reaches the budget, the checkpoint is written and read back (tiny cache, packed), and the end state matches lean.
    let dir = tmpdir("roomy");
    for r in ["RLR", "RRLL", "LLRRRLRLRLLR", "RLLRRRLLRLRL"] {
        let rule: Rule = r.parse().unwrap();
        let ck = Ckpt { path: dir.join(format!("{r}.tkpt")), every: 7_000 };
        let straight = run_lean(&rule, &mut LeanGrid::new(512), &cfg(120_000));
        let mut g1 = TiledGrid::new(512, tcfg(5, Some(2), Inactive::Packed)).unwrap();
        run_tiled_ckpt(&rule, &mut g1, &cfg(20_000), Some(&ck)).unwrap();
        assert!(ck.path.exists());
        let mut g2 = TiledGrid::new(512, tcfg(5, Some(5), Inactive::Raw)).unwrap();
        let resumed = run_tiled_ckpt(&rule, &mut g2, &cfg(120_000), Some(&ck)).unwrap();
        assert_eq!(debug(&resumed), debug(&straight), "{r}");
        // and the state is the oracle's, not merely the summary
        let (want, ..) = oracle_state(&rule, resumed.steps);
        assert_eq!(tiled_state(&g2), want, "{r}");
    }
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn tiny_board_cases_that_caught_a_dropped_touched_bit() {
    // Shrunk counterexamples from a deliberate mutation of the codec (touched plane written as zeros), kept as plain tests.
    // Small boards, 2x2 tiles and a 2-tile cache make every packed tile mostly off-board or touched-only.
    for (bits, n, size, steps) in [(1u64, 2usize, 3usize, 7u64), (1, 2, 24, 85), (1670443992101384193, 2, 8, 6)] {
        let rule = rule_n(bits, n);
        let (r, g) = run_checked(&rule, size, steps, tcfg(2, Some(2), Inactive::Packed));
        let (want, pos, heading) = oracle_state(&rule, r.steps);
        assert_eq!((r.pos, r.heading), (pos, heading));
        assert_eq!(tiled_state(&g), want);
        assert_eq!(debug(&r), debug(&run_lean(&rule, &mut LeanGrid::new(size), &cfg(steps))));
    }
}
