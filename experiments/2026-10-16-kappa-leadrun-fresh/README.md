# kappa leading-run test on fresh, lead-stratified rules

Registered 2026-10-16 (+13:00), base commit `d4fd349`, **before any probe run on these rules**.
Follow-up to `../2026-10-16-kappa-prefix-decode` (post hoc finding: on the 162 stationary
pool rules, the number of identical opening letters ("leading run" `lead`) gives +0.33
held-out R^2 over a ln A(22) spline, beating the prefix-8 lookup's +0.20; residual rises
~+0.1 per step, same slope in census and pool).

## Question

Is the leading-run effect on ln kappa real and out-of-sample, at fixed area? The decode
result was found on data already scored, and `lead` correlates -0.56 with ln A(22).

## Design

300 fresh rules, lengths 13/14, stratified on `lead` = 1..6 (25 rules per lead x length cell,
`R^lead L` + uniform random tail; `scripts/sample.py`, numpy default_rng(20261016 + 1)). Excludes every
W3b/W3c rule (census is length <= 12). Eight patched seeds 601..608 each = 2400 runs,
`probe 27 8192`, same instrument and `per_rule` pipeline as W3c (core band
alpha_w in [0.25, 0.42], kappa > 0, >= 6 core-band runs, stationary = |g_k| < 0.05).
Stratification de-correlates `lead` from rule composition; lead-1 would be 50% of an
unstratified sample, lead >= 4 about 12%.

Model (fixed now): trained **only on the 2183 stationary census rules**, as in decode:
ln kappa = spline(ln A(22)) [knots at 10/25/50/75/90% quantiles, as `prefix_audit.basis`]
+ b * min(lead, 6). Evaluated on the fresh stationary rules. gain = held-out R^2 gain of the
lead term over the spline-only model, strict against the census training mean, bootstrap
over fresh rules (2000, seed 20261016).

## Predictions (thresholds are my design choices)

- **L1** gain >= 0.15 and 95% CI lower bound > 0 (decode's post hoc +0.33 is optimistic; expect shrinkage). ~65%.
- **L2** the census-fitted slope b lies within 0.07-0.13 per step, and the slope refit on fresh
  rules (same spline + lead, OLS) has CI containing b_census and excluding 0. ~60%.
- **L3** (confound check) split fresh rules into ln A(22) tertiles; the within-tertile
  slope of the residual on lead is > 0 in all three and the pooled within-tertile estimate has CI above 0. ~70%.
- **L4** (control) lead shuffled across fresh rules (200 permutations): median gain within +-0.02,
  95th pct <= +0.05.
- **L5** stationarity and core-band retention do not depend on lead: share of rules kept
  differs by < 0.20 between lead 1 and lead 6 strata. If violated, selection biases L1-L3; report only.
- **L6** (descriptive, no threshold) compare `lead` against R-count-in-first-8 and letter-2
  alone (decode's ablations) as fresh-sample gains.

A failure of L1 and L3 together means the decode finding was an area/composition proxy
that did not survive fresh data.

## Commands (WSL, repo root)

```bash
uv run experiments/2026-10-16-kappa-leadrun-fresh/scripts/sample.py      # already run, results/rules.txt, specs.txt
bash experiments/2026-10-16-kappa-leadrun-fresh/run.sh                   # ~22 min, dumps/probe.csv (gitignored)
uv run experiments/2026-10-16-kappa-leadrun-fresh/scripts/analyse.py     # results/analysis.txt, analysis.json
```

## Results

Run 2026-10-16 ~20:15, commit 58b4f38 (dirty tree), seed 20261016, 2000 bootstraps; `results/analysis.txt`, `analysis.json`.
2400 runs, 365 runs escaped the board, 183/300 rules kept (>= 6 core-band runs), 176 stationary (test set). Per lead: 34/32/34/31/26/19.

| pred | outcome |
|---|---|
| L1 gain >= 0.15, CI > 0 | **HELD**: +0.651 [0.498, 0.828] (decode's post hoc +0.33 was not optimistic; here the spline-only baseline is weak, R^2 0.124, because stratifying on lead puts many large-lead rules in the test set; corr(lead, ln A22) = -0.72) |
| L2 census slope in 0.07-0.13 and fresh refit CI covers it | **FAILED as registered**: census slope is +0.195/step (my range was wrong: it came from the unconditioned residual table, not the spline-adjusted regression); fresh refit +0.152 [0.129, 0.174], excludes 0.195, so the slope shrinks ~22% out of sample but is clearly > 0 |
| L3 within-ln A tertile slopes > 0, pooled CI > 0 | **HELD**: tertile slopes 0.151 / 0.155 / 0.149, pooled +0.152 [0.128, 0.177] |
| L4 shuffled lead gain median within +-0.02, p95 <= 0.05 | **FAILED as registered**: median -1.43. The criterion was mis-specified: shuffling lead keeps the large census coefficient and injects variance, so gain is very negative, not ~0. It shows the lead term carries the signal; it is not the "pipeline gives ~0" control I intended. A proper null would refit the coefficient on shuffled lead (not done). |
| L5 retention differs < 0.20 (lead 1 vs 6) | **FAILED**: stationary share 0.68 (lead 1) falling monotonically to 0.38 (lead 6); kept share 0.74 -> 0.40. Selection depends on lead (mostly board escapes / too few core-band runs). L1-L3 hold on survivors only. |
| L6 alternatives | lead +0.651 [0.50, 0.83] vs letter-2-is-R +0.319 [0.25, 0.40] vs R-count-first-8 +0.290 [0.20, 0.39]: lead is better than both, CIs separate from the R-count at the top end only mildly |

Mean residual by lead (fresh, vs census spline-only): -0.148, +0.053, +0.127, +0.188, +0.377, +0.418 (census/pool table: -0.11 .. +0.45): same monotone shape.

**Conclusion.** The leading-run effect replicates on fresh, lead-stratified rules, and it holds at fixed area (L3), so it is not simply the ln A(22) proxy. Not established: causation (no mutation test), and survivor bias (L5 failed: high-lead rules escape or drop out more, so the 0.15-0.20 slope is for rules that stay on the board; selection could inflate or deflate it). Two registered predictions (L2 range, L4 control) were badly specified by me and are reported as failed, not repaired.
