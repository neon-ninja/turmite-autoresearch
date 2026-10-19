# Does the prefix signal survive holding out whole prefix families? (2026-10-20)

## Question
`../2026-10-20-kappa-prefix-additive/` could not separate higher-order interaction structure from memorisation of prefix families
shared by train and test. If the signal is structure, a model should predict rules from families it has never seen; if it is
memorisation, only the lookup should work on seen families and it should collapse on unseen ones.

## Design (registered before running, 2026-10-20 07:10 +13:00, git 7339dc9)
- Data: the 2185 stationary blob census rules (`../2026-10-13-probe-growth-census/results/probe_summary.csv`, target ln kappa).
  (The 162 fresh rules share their 6-letter prefix families with the census, so they cannot test unseen families.)
- **Family** = first 6 letters. **Grouped CV:** 8 folds in which each fold holds out whole families (families assigned to folds
  by a seeded shuffle, seed 20261020); **random CV:** 8 folds of random rules (same seed), where test rules' families are in training.
  Out-of-fold strict R^2 against the training-fold mean, pooled over folds.
- Models: **P8** (8-letter prefix lookup with fallback to shorter keys, >= 3 rules); **A2** (ridge, penalty 1.0, 7 position
  indicators + 21 pairwise products, as in the sibling experiment); **A** (additive ridge).
- Bootstrap over rules for 95% intervals (2000 resamples).

## Predictions (written before the run)
- **D1:** P8 has random-CV R^2 >= 0.45 and grouped-CV R^2 < 0.30 (memorisation of families).
- **D2:** A2 grouped-CV R^2 is within 0.10 of its random-CV R^2 (its signal, if any, is not family memorisation).
- **D3:** A2 grouped-CV R^2 exceeds P8 grouped-CV R^2.

Failures are reported as failures; choices made after seeing results are labelled post hoc.

## Reproduce
`uv run experiments/2026-10-20-kappa-prefix-family-holdout/scripts/holdout.py` -> `results/holdout.txt`, `results/holdout.json`.
