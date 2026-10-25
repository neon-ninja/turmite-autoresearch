# Q19: is the late-highway yield per decade flat? A fourth decade, (1e11, 1e12]

Registered 2026-10-25, base `df07dfe`, branch `ccr-9a56d193-javkym`. Status: **REGISTERED; design and power analysis done, no 1e12 run started.**

## Question and registered prediction

Q19 (`notes/open-questions-hypotheses.md`): the corrected genuine late-highway yields among survivors are 12/854 = 1.41% in
(1e8, 1e9], 16/807 = 1.98% in (1e9, 1e10] and 12/732 = 1.64% in (1e10, 1e11]
(`experiments/2026-10-21-late-highway-board-check/`). Is the per-decade yield constant ("chaotic" is a transient for a fixed fraction
of rules per decade, without end), or slowly falling?

* **H-flat (my prior, ~55%):** per-decade yield p = 40/2393 = 1.67% (pooled), so a random m of the 702 survivors at 1e11 should
  contain 0.0167 m highways with onset in (1e11, 1e12].
* **H-fall:** yield falls by a factor f per decade, f in {0.7, 0.5, 0.3}, so p = 1.17%, 0.83%, 0.50%.
* **Genuine** means the `late-highway-board-check` criteria: not a board filler (escape footprint < 0.99 of the board side in both
  directions), and the same period, drift and onset on a second, larger board.
* **Decision rule (frozen now):** exact one-sided binomial test of the observed count X in the run sample of size m against
  H-flat with p = 0.0167, alpha = 0.05 (reject flat if X <= cut(m)). Report X/m with an exact (Clopper-Pearson) 95% interval
  regardless. Not rejecting is **not** support for flat; say "inconclusive at this power".
* **Power (`scripts/power.py`, arithmetic only):** the test is weak.

| m run | E[X] if flat | cut | power vs x0.7 | vs x0.5 | vs x0.3 | core-hours (2 / 1.5 / 1 e8 steps/s) |
|---|---|---|---|---|---|---|
| 48 | 0.8 | none | 0.00 | 0.00 | 0.00 | 67 / 89 / 133 |
| 100 | 1.7 | none | 0.00 | 0.00 | 0.00 | 139 / 185 / 278 |
| 200 | 3.3 | 0 | 0.09 | 0.19 | 0.37 | 278 / 370 / 556 |
| 350 | 5.9 | 1 | 0.08 | 0.21 | 0.48 | 486 / 648 / 972 |
| 702 | 11.7 | 5 | 0.17 | 0.47 | 0.86 | 975 / 1,300 / 1,950 |

  **Consequence, stated before spending compute:** even the full 702 at 1e12 has under 50% power against a halving per decade;
  only a collapse (x0.3 or worse) is likely to be seen. Partial samples can only estimate the yield (wide interval), not test it.
  The honest deliverable of any feasible stage is an interval for p, to be pooled with the three earlier decades in a
  log-linear trend fit (slope of log yield per decade with its interval), not a flat/falling verdict.
* **Further registered predictions (measurement checks, not hypotheses):**
  T1: tiled `sweep_lean` is byte-identical to dense on the first rules of the sample (met on 2 rules at 3e9, below).
  T2: throughput at 3e9 steps is 1.5-2.2e8 steps/s/core with 4 parallel workers and peak RSS < 100 MiB each (met: 179-188 Msteps/s, 11-16 MiB).
  T3 (open): throughput at 1e12 stays >= 1.0e8 steps/s/core and the footprint on a 65536 board stays below 0.99 of the side for
  the non-filler survivors (the 1e11 widths are 5.5k-12k cells; cube-root growth predicts 1.2-2.6x more at 1e12, i.e. under 31k).

## Pruning-index preflight

* Index revision: `df07dfe58faa93579d1b27a2436c4f0f7c0141db`. Commands: `git rev-parse HEAD`; `rg --files notes/pruning-index/entries`;
  `rg -n -i -e highway -e late -e yield -e unresolved -e decade notes/pruning-index/entries`.
* Candidates: the only entry is `NR-20261022-additive-laws-order-blind`; it matches "highway/late" words only in unrelated
  context (additive conservation laws, objective: order-sensitive invariants). No match for late-highway yield. A no-match is a
  recorded outcome, not evidence of novelty.
* Scope: all 702 survivors of the 1e11 sweep, unchanged rule family, blank background, plain ant, objective = yield of new highways.
* Decision: **run** (no signature covers it). Compute/stopping budget: see "Stages"; a null at partial power does not warrant a dead-end signature.

## Design

* **Population:** the 702 `stop=budget` rows of `experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/results/h1e11/chunk_*.csv`
  -> `results/survivors_1e11.txt` (sha256 `4496fd16...b5f74c`). Note it is a 32768-board census: survivors are rules that neither
  escaped nor showed a highway by 1e11.
* **Sample:** nested prefixes of `results/sample_order.txt`, rules sorted by sha256("q19-fourth-decade:" + rule)
  (`scripts/order.py`; sha256 `70ca50da...cdd721f41`). Any prefix is a uniform random subsample, and stages can be pooled without re-drawing.
* **Instrument:** `sweep_lean` with `TURMITE_GRID=tiled` (sparse tiles, unlimited cache), 65536 board, 1e12 steps, one thread per
  rule, four rules at once, checkpoints every 5e9 (`scratch/ckpt`, gitignored). Tiled is required: a dense 65536 board is 4.3 GB
  per worker and 15 GB RAM would allow 3.
* **Runs restart from step 0** (the 1e11 chunks left no committed checkpoints), so the cost is the full 1e12 per rule: ~1.5 h per rule per core at 1.85e8 steps/s.
* **Confirmation of any hit:** re-run on a 131072 board (tiled) and compare period, drift and onset (the `confirm` criterion);
  classify escapes by footprint as in `late-highway-board-check/scripts/classify.py`.
* **Known blind spot:** a rule whose structure is a highway fed by something outside the 65536 board is still an escape, not a
  highway; rules with a >1e12 onset are censored. This measures the yield in one decade, not the tail.

## Stages (compute is the decision for the user, not for the agent)

| stage | rules (prefix) | wall on 4 cores | what it can say |
|---|---|---|---|
| A | 1-48 | ~18 h | throughput/footprint at 1e12 (T3), interval for p with E[X] = 0.8: essentially "how many to expect" |
| B | 49-200 | ~57 h | with A: m = 200, E[X] = 3.3, cut 0, power 0.19 vs x0.5 |
| C | 201-702 | ~190 h | full population, power 0.47 vs x0.5; a GPU is not usable (no batch instrument) |

## Reproduction

```bash
uv run --frozen experiments/2026-10-25-late-highway-fourth-decade/scripts/power.py
python3 experiments/2026-10-25-late-highway-fourth-decade/scripts/order.py
cargo build --release --locked --bin sweep_lean
bash experiments/2026-10-25-late-highway-fourth-decade/scripts/run_stage.sh FIRST LAST     # 1-based inclusive range of sample_order.txt
```
`run_stage.sh` is resumable: finished rules (`results/runs/RULE.csv`) are skipped and interrupted ones resume from `scratch/ckpt`.
Provenance (commit, binary sha256, start/end) is appended to `results/runs/stage.log`.

## Results

### Calibration at 3e9 steps (2026-10-25; commit `df07dfe`, binary sha256 `c9dee7ca...cc7e1`, rustc 1.97.0)
First four rules of `sample_order.txt` (RRRRLLLRL, RRRRLRLLRL, RLRLLLRRLR, RRLRLLLRL), tiled backend, 65536 board, four processes in parallel,
one thread each: 188 / 182 / 184 / 179 Msteps/s, peak RSS 11-16 MiB, all `budget` with widths 1.7k-2.7k (`results/throughput_check_3e9.csv`).
Dense output equals tiled byte for byte for the first two rules. n = 4: a throughput calibration only, no highway statistic.
Stage A-C results: **none yet.**

## Pruning-index outcome and next question

None yet. A null result here would be a well-controlled but under-powered observation (see table); it must not be recorded as a dead end.
Next questions this sets up: Q15/Q20/Q23 (what builds the late structures) and whether a cheaper observable than the full 1e12 run
(shared-prefix trajectory deduplication, or an onset-hazard model fitted to the 40 known onsets) can add power for free.
