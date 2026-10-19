# Which prefix positions carry kappa? Per-position ablation (2026-10-20)

## Question
`../2026-10-20-kappa-prefix-runs/` showed the prefix signal is distributed over the opening (first four runs 0.48, eight letters 0.62).
Which of letters 1-8 carry it?

## Design (registered before running, 2026-10-20 05:20 +13:00, git f40da67)
- Same train / test as the sibling experiments: 2185 stationary blob census rules, 162 fresh stationary rules, disjoint; strict
  held-out R^2 against the training mean; lookup P8 (first 8 letters, fall back to shorter keys with >= 3 training rules).
- **Ablation of position j (j = 1..8):** the key ignores letter j (replaced by `*` in the key for all depths that include it).
- **Group-count control:** ablating a position merges groups, which changes the bias-variance trade-off independently of
  information. Control: permute the letters at position j among training rules (200 permutations, seed 20261020) and re-run the
  lookup unchanged on the test rules; this keeps the number of groups and destroys the information at j. Report the mean control R^2.
- Loss = R^2(P8) - R^2(ablated). Bootstrap over test rules (2000 resamples) for the ablation loss.

## Predictions (written before the run)
- **A1:** position 1 (always R in this cohort) has loss within 0.01 of 0.
- **A2:** each of positions 2, 3, 4 has ablation loss >= 0.05.
- **A3:** no position in 5-8 has a larger ablation loss than the largest loss among positions 2-4 (information is front-loaded).
- **A4:** for each position the permutation-control loss is at least as large as the masking loss minus 0.05 (masking is not
  hiding a large information effect behind regrouping; failure means the masking and the control disagree).

Failures are reported as failures; anything chosen after seeing the results is labelled post hoc.

## Reproduce
`uv run experiments/2026-10-20-kappa-prefix-ablation/scripts/ablate.py` -> `results/ablate.txt`, `results/ablate.json`.
