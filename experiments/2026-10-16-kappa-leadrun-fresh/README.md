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

(pending)
