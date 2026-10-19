# Which part of the prefix carries kappa? Run structure of the opening (2026-10-20)

## Question
`../2026-10-20-kappa-spread-features/` found the predictable part of ln kappa is in the first 8 letters (held-out R^2 0.62) and not in the
last 8 or in composition. Earlier work (`../2026-10-16-kappa-leadrun-fresh/`) tied ln kappa to the *lead* (length of the initial run of R,
slope +0.15 to +0.24 per step, with a collider caveat). How much of the prefix signal is run structure of the opening?

## Design (registered before running, 2026-10-20 03:50 +13:00, git 014f2c2)
- Same train / test as the sibling experiment: 2185 stationary blob census rules (`probe_summary.csv`, ln kappa), 162 fresh stationary
  rules (`prefix_audit_predictions.csv`, `target`, cohort stationary / loo / linear), disjoint by rule; strict held-out R^2 against the training mean.
- A rule is split into maximal runs of equal letters (e.g. `RRLLLRL` -> R2 L3 R1 L1). Key `Rk` = the lengths of the first k runs
  (k = 1, 2, 3, 4), so `R1` is the lead alone. Group-mean lookup on the key, falling back to fewer runs with >= 3 training rules, then
  the training mean. Reference: the 8-letter prefix lookup P8 from the sibling experiment, recomputed here.
- Bootstrap over test rules, 2000 resamples, seed 20261020.

## Predictions (written before the run)
- **H1:** the lead alone (`R1`) has test R^2 >= 0.15.
- **H2:** the first two runs (`R2`) reach R^2 >= 0.40.
- **H3:** the first three runs (`R3`) come within 0.10 of P8.
- **H4:** no run-structure key exceeds P8 by more than 0.05 (the 8-letter prefix contains them).

Failures are reported as failures. Anything chosen after seeing results is labelled post hoc.

## Reproduce
`uv run experiments/2026-10-20-kappa-prefix-runs/scripts/runs.py` -> `results/runs.txt`, `results/runs.json`.
