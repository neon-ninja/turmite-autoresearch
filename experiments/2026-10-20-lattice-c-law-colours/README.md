# Why does the c-law slope b fall with loop length? Circulations per birth versus reads per circulation

Registered 2026-10-20 00:55 (+13:00), base commit `6049d6a` plus the uncommitted instrument (`src/lattice_trace.rs`,
`tests/lattice_trace.rs`, `examples/lattice_c_law_colours.rs`), committed together with this file before any run on the
960-ant list. **Nothing in this folder has been run on that list.** Disclosure of what was seen before: the lattice
c-law results (`experiments/2026-10-20-lattice-c-law`, slopes b and intercepts per stratum, per-lead medians), and on the
*square* lattice only, the per-colour and lag-4 findings of `experiments/2026-10-19-c-law-mechanism` (r_k decays ~0.88 per
read; 48-78% of circulating reads return after exactly 4 steps). No per-colour, lag or run statistic has been looked at
on the triangular or hexagonal lattice. The tracer's own gates (below) ran on short random rules only.

## Question

`1/c = a + b * lead` holds on three lattices (R^2 0.94-0.95 square and triangular), but b falls with the loop that the lead
letter closes: 0.86 (hex R, loop 3), 0.70 (square, 4), 0.34 (triangular, 6), 0.28 (hex r, 6; per-lead medians, since the
OLS is destroyed by one rule). The lattice study's first-principles reading, `b = l_b / N_b` (boundary cells per lap over
births per circulation episode), was never measured on either side, and its three hypotheses (b grows with l, constant,
~4/z) are all disproven. A fitted b(l) would be a fourth guess. Instead: **measure the two factors of b.**

## Definitions (frozen)

- Everything as in `experiments/2026-10-20-lattice-c-law` (windows, boundary step, birth, lead, strata, `alpha_w` blob filter, kept set).
  The tracer `src/lattice_trace.rs` uses the last octave only (steps (2^26, 2^27]), identical to the probe's window k = 26.
- **Return**: a step that starts on a cell last read exactly `ell` steps earlier; `ell` = loop length of the rule's first
  letter: square 4, triangular 6, hex `r`/`l` 6, hex `R`/`L` 3 (checked by stepping in `tests/lattice_probe.rs`, gate G3 of the parent).
- **Circulation**: a maximal run of consecutive returns (any colour, boundary or not) of length >= `ell` (one full lap
  re-read). `eps` = circulations per birth. `R` = mean run length of circulations (lengths capped at 63). `C` = total steps
  inside circulation runs per birth = `eps * R`. `Cb` = the same counting only boundary steps in those runs.
- **Slopes** are Theil-Sen over kept rules (primary, robust to the 1/c = 390 outlier) and the slope of per-lead medians
  (secondary), leads 1..5 for 1/c, and m = lead for C, Cb, eps, R. OLS is reported but is not primary.
- `b_med` = slope of per-lead medians of 1/c, recomputed from this run (not copied).

## Gates (instrument; a fail voids the dependent claims)

- **T1** (done before this registration, `cargo test --release --test lattice_trace`): square `LTrace` equals `edge_trace::trace_blank`
  in every field on 30 random rules; hex, triangular and square equal a HashMap re-implementation on 30 random rules each;
  two one-line mutations (lag off by one, return lag `ell + 1`) were each caught by the tests.
- **T2** on the 960-ant list: for every rule not escaped on the 8192 board, `births` and `boundary_steps` equal `ants.csv` window k = 26
  exactly, and `sum_k edge_k = boundary_steps`.
- **T3** `1/c` from the tracer equals the parent's to 1e-12 (follows from T2; reported anyway).

## Predictions (kept rules, per stratum; my odds in brackets)

- **P1 (circulation accounts for the lead slope).** `Cb` per-lead-median slope is within [0.8, 1.2] x `b_med` in all four strata. [45%]
  (If this fails the reads that grow with the lead are not in lag-`ell` runs, and the loop picture of the slope is incomplete off the square lattice.)
- **P2 (reads per circulation).** `R` per-lead-median slope divided by `ell` lies in [0.5, 1.1] in all four strata (this is `l_b / l`:
  the fraction of a loop that is boundary and survives a lap). [50%]
- **P3 (the discriminator).** `b = eps * lambda` with `lambda = dR/dm`. The drop of b with loop length is carried by `eps`, not by
  `lambda`: `eps` at lead 3 (stratum median over kept rules) is lower on triangular than on square by a factor >= 2.0, and lower
  on hex r than hex R by a factor >= 3.0. [55%]
  Rival (lambda carries it, N_b/l roughly constant): ratios of `eps` within [0.7, 1.4] of 1, with `lambda/ell` falling instead. Reported with the measured vector either way.
- **P4 (primary robust slope).** Theil-Sen b is within the parent's OLS bootstrap CI on square, triangular and hex R, and in [0.15, 0.45] on hex r. [60%]
- **P5 (hex ring centre).** Births into a cell whose neighbours are all already visited (`hole_births`, e.g. a 6-ring centre) per birth are >= 2x
  higher on hex r-lead than hex R-lead (median over kept rules), because only the r ring encloses a cell. [40%]
- **P6 (letter-typed hex lead).** Splitting hex R-lead kept rules by the killing letter (the first letter different from the lead:
  `r`, `l`, `L`) and fitting `1/c ~ lead` per type raises the mean R^2 weighted by n from 0.69 to >= 0.80. [40%]
  Not registered: any claim about U-turn letters (turn 2 on square, 3 on hex); a second stage with a new loop length (period 2) would need its own registration.

## Decision rule

P3 is the headline. Supported if both ratios pass; "rival" if both eps ratios are in [0.7, 1.4]; otherwise undecided with
the measured vector. A gate failure (T2) on a lattice means no claim on that lattice.

## Commands

```
bash experiments/2026-10-20-lattice-c-law-colours/scripts/run.sh   # fresh target dir, sha256 logged, writes results/traces.jsonl
uv run experiments/2026-10-20-lattice-c-law-colours/scripts/analyse.py
```

## Results

(not yet run)
