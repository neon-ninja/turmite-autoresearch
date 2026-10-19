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
