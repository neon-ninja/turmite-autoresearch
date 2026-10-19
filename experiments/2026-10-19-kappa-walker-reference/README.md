# kappa of the calibration walkers against their own uniform reference

Registered 2026-10-19 (+13), base commit 6de71f2 (dirty workspace from other
experiments; this folder is untracked and self-contained), **before any new run**.

## Question

`experiments/2026-10-16-kappa-uniform-baseline` showed the probe's kappa for a walker that is
uniform on its territory at every instant is `kappa_unif(a, beta) = 2 * int_{1/2}^{1} x^(-a(1-beta)) dx`
(1.110 at a = 2/3, beta = 1/2), and restated the ant (median census kappa/kappa_unif = 0.81,
5-95% 0.44-1.33, IQR 0.66-0.96, n = 2185 stationary blob rules, single run each). The rotor
(kappa ~2.0) and memoryless walkers (1.3-1.45) were only compared against 1.11, which is
justified only for a ~ 2/3, beta ~ 1/2 (deviation entry 2026-10-16 01:00). Recompute their
ratios with **each walker's own measured (a, beta)**, then ask whether the ant location is ant-specific.

## Prior numbers (from EXISTING data, computed before the new runs)

`scripts/prior_ab.py` on `2026-10-13-a1-kappa/results/a1d_probe.csv` (pw, 6 seeds) and
`2026-10-13-calib-instrument-calibration/results/calib_rotor.csv` (rotor, 64 seeds), windows k = 12..22:

| walker | a | beta | kappa | kappa_unif(a,beta) | ratio |
|---|---|---|---|---|---|
| pw:0.25 | 0.891 | 0.882 | 1.254 | 1.033 | 1.21 |
| pw:0.5  | 0.913 | 0.907 | 1.358 | 1.027 | 1.32 |
| pw:0.75 | 0.930 | 0.931 | 1.287 | 1.020 | 1.26 |
| rotor   | 0.724 | 0.564 | 1.856 | 1.104 | 1.68 |

**Correction to the premise in the task/deviation entry.** "Memoryless walkers have a ~ 0.95 and a
reference near 1.16" used the beta = 1/2 column of the formula (a = 0.95 -> 1.159). Their territory
is porous and the perimeter (visited cells with an unvisited 4-neighbour, holes included) scales
as P ~ A^0.9, so a(1-beta) ~ 0.09 and the reference is **~1.02-1.03**, below 1.11, not above.
Recomputing therefore moves the memoryless ratios *up* (1.2-1.3 vs 1.2-1.3 against 1.11: unchanged
to within 0.1), not down.

## Predictions (registered)

New runs, fresh seeds, same probe: ratio_w = kappa_w / kappa_unif(a_w, beta_w), with a_w, beta_w
fitted per seed (log-log, windows k = 12..22) from the same run as kappa (kappa = mean over the
same windows of the per-window kappa, as in `scripts/analyse.py` of the baseline experiment).

1. Memoryless walkers (`pw:0.25`, `pw:0.5`, `pw:0.75`, `rw`): a in 0.88-0.96, beta in 0.86-0.95,
   kappa_unif in 1.01-1.05, ratio in 1.15-1.40 (rw has no prior run in this window; I expect it
   alongside pw:0.5). Between-seed sd of the ratio <= 0.1.
2. Rotor: a in 0.68-0.74, beta in 0.53-0.58, kappa_unif ~1.10, ratio 1.6-1.8 over k = 12..22. Over k = 20..26
   (the window the census uses), kappa ~2.0 and ratio ~1.8; the difference between the two windows
   is reported as the window bias.
3. Mean-field walker (anchor, already measured, not rerun): ratio 1.01 (1.118 / 1.107).
4. Location test. Ant median 0.81, IQR 0.66-0.96, 5-95% 0.44-1.33.
   - **Ant-specific location** is supported if every non-uniform walker's ratio is above the ant
     IQR (> 0.96) by more than 3 between-seed sd, and none lies in 0.66-0.96. I predict this: all
     walkers 1.2-1.8.
   - **Same ratio for all walkers** would be shown by walker ratios inside 0.66-0.96 (the ant IQR).
     I predict this does not happen.
   - **Not discriminating** if some walkers sit in 0.96-1.33. That is inside the ant 5-95% range
     (so "outside the ant range" is the wrong test for those), but above the median. I predict the
     memoryless walkers land here (1.2-1.3) and only the rotor is outside (> 1.33). If so: the
     verdict is that the ant is *lower than every walker tested* and that walkers do not
     discriminate within 0.96-1.33; it says nothing about mechanism.
   - Rule-level caveat: the ant ratios are single run, rule-dependent, noise inflated
     (ln-kappa reliability ~0.6). Only a walker-ratio far from the ant IQR carries weight.

## Design

- `rw`, `pw:0.25`, `pw:0.5`, `pw:0.75`: seeds 2026101901..08, `probe 23 16384` (board 16384, 2^23 steps;
  windows k = 12..22). Escaped windows are dropped and counted; seeds with any escape are reported.
- `rotor`: seeds 2026101901..08, `probe 27 8192`; fit over k = 12..22 and k = 20..26 separately.
- No estimator or fit change relative to the baseline experiment's `analyse.py` (same formulas). A
  secondary, descriptive-only estimate with per-window local (a_k, beta_k) from adjacent window ends
  is also reported, because a single power law is a poor description of porous walkers.

## Commands / results

Filled in as it runs (`NOTEBOOK.md`, `results/`).

## Results (2026-10-19, run at commit 34fbf45; workspace dirty with unrelated changes, no Rust changed)

Commands (WSL, repo root): `bash experiments/2026-10-19-kappa-walker-reference/scripts/run.sh` (builds `probe`; specs in `results/specs_mem.txt`,
`results/specs_rotor.txt`; seeds 2026101901..08 for each of rw, pw:0.25, pw:0.5, pw:0.75, rotor; memoryless
`probe 23 16384`, rotor `probe 27 8192`; outputs `results/probe_mem.csv`, `results/probe_rotor.csv`),
then `uv run experiments/2026-10-19-kappa-walker-reference/scripts/analyse.py` -> `results/analysis.txt`. 8 seeds per walker, no escaped windows.
Mean (between-seed sd); a, beta fitted per seed, then averaged.

| walker (windows) | a | beta | kappa | kappa_unif(a,beta) | ratio | vs ant (med 0.81, IQR 0.66-0.96, 5-95 0.44-1.33) |
|---|---|---|---|---|---|---|
| rw (12..22) | 0.920 (.028) | 0.917 (.041) | 1.307 (.030) | 1.024 | **1.28** (.04) | above IQR, inside 5-95 |
| pw:0.25 | 0.903 (.017) | 0.898 (.021) | 1.271 (.027) | 1.029 | **1.24** (.03) | same |
| pw:0.5 | 0.920 (.030) | 0.909 (.037) | 1.350 (.036) | 1.026 | **1.32** (.04) | same, at the 95% edge |
| pw:0.75 | 0.911 (.026) | 0.907 (.027) | 1.287 (.051) | 1.026 | **1.26** (.06) | same |
| rotor (12..22) | 0.730 (.020) | 0.575 (.030) | 1.858 (.039) | 1.102 | **1.69** (.04) | above ant 95% |
| rotor (20..26) | 0.685 (.006) | 0.530 (.026) | 1.989 (.034) | 1.106 | **1.80** (.03) | above ant 95% |
| mean-field (baseline expt) | 0.677 | 0.520 | 1.118 (.009) | 1.107 | 1.01 (.01) | anchor |

The local-exponent reference (per-window a_k, beta_k from adjacent window ends) agrees with the
power-law reference to <= 0.006 in every row (columns in `analysis.txt`).

### Against the registered predictions
1. Memoryless: a 0.90-0.92 (pred 0.88-0.96), beta 0.90-0.92 (0.86-0.95), kappa_unif 1.02-1.03 (1.01-1.05),
   ratio 1.24-1.32 (1.15-1.40), sd <= 0.06 (<= 0.1): **held**.
2. Rotor: a, beta, ratio inside the registered ranges in both windows (1.69 over 12..22, 1.80 over 20..26): **held**.
   Window bias of the ratio between the two windows is 0.11 (rotor a still relaxing towards 2/3).
3. Location test: every non-uniform walker is above the ant IQR (by 5-25 between-seed sd of the walker); none is in
   0.66-0.96. By the registered criterion the ant location is *lower than every walker tested*.
   As registered, the memoryless walkers fall in the not-discriminating band 0.96-1.33 (inside the ant 5-95%
   range; about 20% of ant rules are >= 1); only the rotor is outside the ant range.

### What this does and does not show
- The recomputed ratios: memoryless ~1.25-1.3 (not changed by the correction: their reference is 1.03, not 1.16, because
  beta ~0.9), rotor 1.7-1.8. The ant median 0.81 is not the value a generic walker gets; uniform-on-territory
  (mean-field) is 1.0, and no non-uniform walker tested sits below 1.
- It does not say why. The walkers are above 1 and the typical ant is below 1; the ant range reaches 1.33 and the
  memoryless walkers sit at its top edge, so 'inside the ant range' is a weak statement. Accounting plus a control.
- Caveats: memoryless fits use k = 12..22 (board limits; a ~0.92 is not asymptotic, the census uses later windows);
  the rotor shows the window effect is ~0.1 in the ratio. Ant ratios are single-run and rule-dependent; walker
  ratios are 8-seed means, so the comparison is walker mean vs ant distribution, not like with like.
  Window labelling: probe CSV k is the window end (t = 2^k), the baseline mean-field CSV labels by window start, so
  'k=12..22' there is ends 2^13..2^23; same estimator, different label (corrected in the notebook).
- Next: a walker with the ant's a ~ 2/3, beta ~ 1/2 but intermediate memory (noisy rotor `nrotor:EPS`, Arm B of A1) to
  see where between 1.8 and 1.0 the ratio crosses the ant median; that discriminates 'memory in the environment
  lowers kappa' from 'it is ant-specific', which is not tested here.
