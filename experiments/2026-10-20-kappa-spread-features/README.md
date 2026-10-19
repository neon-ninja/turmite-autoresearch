# What carries the rule-determined spread of ln kappa? Prefix vs suffix vs composition (2026-10-20)

## Question
`../2026-10-20-kappa-spread-budget/` found that the first eight rule letters predict about half of the variance of ln kappa
(held-out R^2 0.55-0.63 on rule means) and that about a third is rule-determined but unexplained. Is the predictable part
specific to the **start** of the rule string (consistent with the lead-run / early-evolution story), or would other string
summaries do as well (end of the string, letter composition)?

## Design (registered before running, 2026-10-20 03:10 +13:00, git 91a2243)
- **Train:** census rules in `../2026-10-13-probe-growth-census/results/probe_summary.csv`, blob class, `kappa > 0`,
  `|g_k| < 0.05` (stationary), ln kappa as target. **Test:** the fresh stationary rules (13-14 letters, 162) with ln kappa
  `target` from `../2026-10-kappa-rule-space/results/prefix_audit_predictions.csv` (one row per rule, cohort `stationary`,
  encoding `loo`, area `linear`). Train and test are disjoint by rule. The training filter approximates the audit's
  (2183 rules there); the exact count is reported.
- **Models** (all area-free, rule string only; strict held-out R^2 against the training mean, as in W3c):
  - **P8:** mean of ln kappa over training rules sharing the first 8 letters, falling back to shorter prefixes with >= 3 rules (the W3c lookup).
  - **S8:** same with the **last** 8 letters (suffix), same fallback.
  - **C:** ordinary least squares on composition: length, count of R, number of runs, longest run of L, longest run of R.
  - **P8+C:** P8 prediction plus C as a second regressor (OLS on training predictions made leave-one-out within training).
- **Uncertainty:** bootstrap over test rules, 2000 resamples, seed 20261020.

## Predictions (written before the run)
- **F1:** P8 test R^2 is within 0.1 of the W3c value 0.633 (the training cohort differs slightly).
- **F2:** S8 test R^2 is at least 0.2 below P8 (the predictable part is specific to the start of the string).
- **F3:** C test R^2 < 0.2.
- **F4:** P8+C improves on P8 by less than 0.05 (composition adds nothing beyond the prefix).

Any of these failing is reported as failed. Choices made after seeing results will be labelled post hoc.

## Reproduce
`uv run experiments/2026-10-20-kappa-spread-features/scripts/features.py` -> `results/features.txt`, `results/features.json`.
