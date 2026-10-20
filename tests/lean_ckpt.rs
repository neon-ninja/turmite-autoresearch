//! Checkpointed lean runs equal uninterrupted ones, including when extended from a budget-stop checkpoint.
use proptest::prelude::*;
use turmite::Rule;
use turmite::fast::Config;
use turmite::lean::{Ckpt, LeanGrid, run_lean, run_lean_ckpt};

fn cfg(max_steps: u64) -> Config {
    Config { max_steps, ..Config::default() }
}

fn same(a: &turmite::fast::RunResult, b: &turmite::fast::RunResult) -> bool {
    format!("{a:?}") == format!("{b:?}")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(40))]
    #[test]
    fn extend_and_resume_match_straight_run(bits in 0u32..256, n in 3usize..9, first in 1u64..60_000, total in 60_000u64..200_000, every in 1u64..50_000) {
        let rule = Rule::from_bits(bits & ((1 << n) - 1), n);
        prop_assume!(!rule.is_trivial());
        let dir = std::env::temp_dir().join(format!("turmite-ckpt-{}-{bits}-{n}-{first}-{total}-{every}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ck = Ckpt { path: dir.join("a.ckpt"), every };
        let mut g = LeanGrid::new(1024);
        // Stage 1 stops at `first`, stage 2 extends to `total` from the saved state.
        run_lean_ckpt(&rule, &mut g, &cfg(first), Some(&ck));
        let mut g2 = LeanGrid::new(1024);
        let ext = run_lean_ckpt(&rule, &mut g2, &cfg(total), Some(&ck));
        let straight = run_lean(&rule, &mut LeanGrid::new(1024), &cfg(total));
        prop_assert!(same(&ext, &straight), "{ext:?} vs {straight:?}");
        std::fs::remove_dir_all(&dir).ok();
    }
}

#[test]
fn foreign_or_damaged_checkpoint_is_ignored() {
    let dir = std::env::temp_dir().join(format!("turmite-ckpt-bad-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join("x.ckpt");
    let (a, b): (Rule, Rule) = ("RLR".parse().unwrap(), "RRL".parse().unwrap());
    let ck = Ckpt { path: p.clone(), every: 10_000 };
    run_lean_ckpt(&a, &mut LeanGrid::new(512), &cfg(50_000), Some(&ck));
    // other rule: same file must not be used
    let r = run_lean_ckpt(&b, &mut LeanGrid::new(512), &cfg(80_000), Some(&ck));
    assert!(same(&r, &run_lean(&b, &mut LeanGrid::new(512), &cfg(80_000))));
    // truncated file
    let bytes = std::fs::read(&p).unwrap();
    std::fs::write(&p, &bytes[..bytes.len() / 2]).unwrap();
    let r = run_lean_ckpt(&b, &mut LeanGrid::new(512), &cfg(80_000), Some(&ck));
    assert!(same(&r, &run_lean(&b, &mut LeanGrid::new(512), &cfg(80_000))));
    std::fs::remove_dir_all(&dir).ok();
}
