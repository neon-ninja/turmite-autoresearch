# Lab notebook: falsification harness

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-22 21:30 (+13:00) · cbe7b40 · intent
Build a harness that attacks a registered claim (perturbed starts, asymmetric colourings, obstacles, off-centre
boards, adaptive search) instead of re-running it longer. Motivation: "chaotic" / "stabilised" labels have rested
on one horizon on the blank board.

### 2026-10-22 21:31 (+13:00) · cbe7b40 · decision
Own seeded runner (`falsify::simulate`) rather than `fast::run`: `Grid::reset` clears only the previous bounding box,
so seeded cells would leak between runs, and `fast::highway_onset` replays from a blank board so it is wrong for
perturbed starts (onset is therefore used only for the blank baseline). `Trail` in `fast.rs` made `pub(crate)`.

### 2026-10-22 21:32 (+13:00) · cbe7b40 · observation
`cargo test --release --test falsify`: 13/13 pass first time. That was suspicious, so mutants were tried:
dropping the start heading is caught by two tests; shifting the obstacle direction window by one step (4001 -> 4000)
is NOT caught; a 10x shift (4001 -> 401) is caught only after widening `radius` to 0..7, `ahead` to 0..15 and
`at` to < 6000 in the property test. The one-step mutant is nearly equivalent (it changes the direction vector by one
step's displacement) and stays uncaught. Test was strengthened for the large mutant.

### 2026-10-22 21:34 (+13:00) · cbe7b40 · decision
`horizon` in a claim is the author's horizon; the harness attacks to horizon * 4^ladder and reports highways by
horizon * 4^k, flagging refutations that appear only after the stated horizon. Added the `blank` class so the claim
as literally stated is always one of the attacks.

### 2026-10-22 21:36 (+13:00) · cbe7b40 · assumption
Predictions P1-P7 in README.md are registered before any run. `RRLR` chosen for P6 because the census records its
highway (384; -4,-4) at onset 256240 (`experiments/2026-10-13-census-sweeps/results/confirmed_v2.csv`).
