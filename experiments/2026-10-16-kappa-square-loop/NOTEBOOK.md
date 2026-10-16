# Lab notebook: kappa and the square-loop trap

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-16 20:55 (+13:00) · 9dce376 · intent
Scaffolded this folder at the user's request ("scaffold experiment directories with detailed notes for subsequent researchers to evaluate your predictions"). The hypothesis comes from `../2026-10-16-kappa-leadrun-fresh/NOTEBOOK.md`, entry 20:45. No instrument, no probe data yet. P1-P4 are copied into the README; C1-C3 are added there, written before any loop data but after P1-P4, and flagged as such. The sibling `../2026-10-16-kappa-loop-surgery/` holds the causal test (draft).

### 2026-10-16 20:57 (+13:00) · 9dce376 · observation (scripts/p1_sanity.py, results/p1_sanity.txt)
P1: 240/240 lead-k rules (k = 1..6, both chiralities, 20 random 8-letter tails each, tail preceded by a letter that breaks the lead, seed 20261016) repeat with exactly k laps of period 4 from t = 0. The simulator is pure Python written independently of `src/`: unbounded dict board, start heading north, R = clockwise, colour `c` turns by `rule[c]` then advances to `(c+1) % n`. Not yet cross-checked against `reference`; do that in the identity tests.

### 2026-10-16 20:58 (+13:00) · 9dce376 · decision
The reported kappa is the window k = 26 value (steps 2^26..2^27). I read this in `experiments/_lib/analyse_probe.py` (KLO=20, KHI=26, `kappa=float(g["kappa"][-1])`), not k = 22; ln A uses k = 22. My 20:45 wording "in the probe window" was ambiguous. The README now fixes k = 26 as primary, 22 and 24 as secondary. The sampler therefore must run to 2^27 steps like the parent probe (~22 min for 2400 runs). No prediction's content changed.

### 2026-10-16 20:59 (+13:00) · 9dce376 · assumption
(a) Chirality symmetry: `L^k R...` is the mirror of `R^k L...`. P1 covers both on the blank board only. (b) In the probe window the relevant loops use cells of colour < lead, not only all-fresh cells, so the lap count there is lead minus the highest colour among the four cells, not k. Check with the lap-length histogram. (c) The parent's `PatchedAnt` background (random 5x5 patch, colour 0 elsewhere) means frontier cells have colour 0 when first entered. All untested.

### 2026-10-16 21:00 (+13:00) · 9dce376 · question
Is a same-turn 4-loop the right unit? Longer closed paths (period 6, 8) with mixed turns exist on mixed colours. If P2 holds only weakly, check those before dropping the idea. Open for the implementer.

### 2026-10-16 20:56 (+13:00) · 9dce376 · deviation (corrects: timestamps of entries above)
The entries above carry times 20:55-21:02, written while the wall clock was at about 20:50-20:54, so some are stamped slightly in the future. Order is correct; treat the stamps as approximate. Also: the parent mutant probe run finished 20:53 (`../2026-10-16-kappa-leadrun-fresh/dumps/mutants.done` exists); `analyse_mutants.py` had not been run when this was written.

### 2026-10-16 20:58 (+13:00) · 9dce376 · observation
The parent mutant analysis exists now (`../2026-10-16-kappa-leadrun-fresh/results/mutants_analysis.txt`): M1, M2 failed as registered, M3 held, area-matched pairs and regression b = +0.158 agree with the lead slope. HANDOFF step 3 updated to say so; it no longer gates on a placeholder.
