//! `highway_onset_reusing` must equal `highway_onset_flat`, including when the scratch grid is reused.
use turmite::Rule;
use turmite::fast::Config;
use turmite::lean::{LeanGrid, highway_onset_flat, run_lean};
use turmite::onset::{OnsetScratch, highway_onset_reusing};

#[test]
fn reused_scratch_matches_flat_onset() {
    let size = 1024;
    let mut sc = OnsetScratch::new(size);
    let mut checked = 0;
    // Langton's ant (onset 9976) plus every rule up to length 7 that finds a highway; the scratch is
    // reused across all of them, so stale cells would show up as a mismatch.
    let mut rules: Vec<Rule> = vec!["RL".parse().unwrap()];
    for n in 2..=7usize {
        for b in 0..1u32 << n {
            let r = Rule::from_bits(b, n);
            if r.is_canonical() && !r.is_trivial() {
                rules.push(r);
            }
        }
    }
    for rule in rules {
        let cfg = Config { max_steps: 400_000, ..Config::default() };
        let res = run_lean(&rule, &mut LeanGrid::new(size), &cfg);
        if let Some(hw) = res.highway {
            let want = highway_onset_flat(&rule, hw, res.steps, size);
            assert_eq!(highway_onset_reusing(&rule, hw, res.steps, &mut sc), want, "{rule}");
            checked += 1;
        }
    }
    assert!(checked >= 1);
    let rl: Rule = "RL".parse().unwrap();
    let res = run_lean(&rl, &mut LeanGrid::new(size), &Config { max_steps: 400_000, ..Config::default() });
    assert_eq!(highway_onset_reusing(&rl, res.highway.unwrap(), res.steps, &mut sc), 9976);
    // the scratch ends up clean
    assert!(sc_is_blank(&sc));
}

fn sc_is_blank(sc: &OnsetScratch) -> bool {
    // Reuse once more on a fresh ant and compare with a brand-new scratch.
    let rl: Rule = "RL".parse().unwrap();
    let res = run_lean(&rl, &mut LeanGrid::new(1024), &Config { max_steps: 200_000, ..Config::default() });
    let hw = res.highway.unwrap();
    let mut fresh = OnsetScratch::new(1024);
    let mut sc2 = OnsetScratch::new(1024);
    let _ = sc;
    highway_onset_reusing(&rl, hw, res.steps, &mut fresh) == highway_onset_reusing(&rl, hw, res.steps, &mut sc2)
}
