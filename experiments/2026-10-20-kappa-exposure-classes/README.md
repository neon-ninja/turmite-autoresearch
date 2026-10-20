# Where on the edge is the ant's boundary deficit? Boundary occupation by exposure class

Owner: Turmite Research Agent (AI research agent). Branch `turmite-research-agent/onboarding`.

**Registered 2026-10-20 09:46 UTC (22:46 +13:00), base commit `ba38590b33e32c8c0a7078d390cee8c11aa2ab20`,
before any measurement on the rule sample below.** Instrument validation runs done before
registration are listed in `NOTEBOOK.md` (grid 512, 2^16-2^20 steps, oracle checks only; the only
class values I looked at were two windows of the highway rule `RL`).

## Question

Ants spend less time on boundary cells than a walker spread uniformly over the visited set:
census median kappa/kappa_unif = 0.814 over 2185 stationary rules
(`experiments/2026-10-16-kappa-uniform-baseline/results/census_spread.txt`), while every walker
measured so far sits above uniform (memoryless 1.24-1.32, rotor 1.69, mean-field 1.01;
`experiments/2026-10-19-kappa-walker-reference/`). No mechanism explains the location of that
distribution (`notes/ACTIVE.md`, "kappa/kappa_unif distribution, location").

The boundary P is a mixture: a visited cell with u = 1, 2, 3 (rarely 4) unvisited 4-neighbours.
Flat edge cells have u = 1; corners, bays and protruding tips have u >= 2. This experiment asks
**which kind of boundary cell the deficit lives on**. It is the "outer edge versus internal holes"
style of definition audit proposed in `notes/kappa-mechanisms-2026-10-15.md`, cut along exposure
rather than hole/outer (holes are only ~4% of ant boundary cells, `experiments/2026-10-14-hull-width/`).

## Instrument

`examples/kappa_exposure_classes.rs`. Stepping and region bookkeeping copy `src/probe.rs`. Per
octave window (2^(k-1), 2^k] (k labelled as in `bin/probe`, i.e. by the window's end exponent):

- `b_u`: steps starting on a class-u cell (sum over u is the probe's `boundary_steps`);
- `f_u`: of those, steps starting on the cell the walker entered as a new cell on the previous
  step (the forced first departure from a newborn cell);
- `m_u`: sum over the window's steps of P_u(t)/A(t) at step start;
- `p_u`: class counts at window end.

Convention-free occupation ratio: **rho_u = sum b_u / sum m_u**. For a walker uniform over the
visited set at every instant, E[rho_u] = 1 for every class, with no end-of-window reference (the
probe's kappa divides by end-of-window P/A, which is why its uniform value is ~1.11). Returns-only
version: **rho_u^ret = sum (b_u - f_u) / sum m_u**. Total rho pools all u.
Main statistic: **Q = rho_1^ret / rho_2+^ret** (2+ pools u = 2, 3, 4).

Oracle: `--check` compares every window with `probe::probe_walker` (area, perimeter, new_cells,
boundary_steps, and fresh_nbrs = sum u b_u) and recounts p_u by scanning the board; it panics on any
mismatch. A mutant (class clipped at 2) is caught. The mean-field walker `mf:SEED` reproduces
`examples/kappa_uniform_baseline.rs` window for window (area, perimeter, new_cells, boundary_steps).

## Sample and design (fixed before measuring)

- Rules: 64 drawn from the 2185 stationary census rules (blob, |g_k| < 0.05) by
  `scripts/select_rules.py` (numpy `default_rng(20261020)`), selecting on class columns only.
  `results/specs.txt` sha256 `b55889c9866770196a024811437c34aabf6ceb9baa1d997e3485d57bf53b93b5`.
- Per rule: one blank-board run `RULE` (same trajectory as the census) and four patched starts
  `RULE@1..4` (5x5 random patch, as `bin/probe`). Rule-level values are the mean over the four
  patched starts; between-seed sd reported. The blank-board run is used for the reproduction check
  and P2b, and reported separately.
- Calibration walkers: `mf:1..8` (uniform by construction), `rotor:1..8`, `rw:1..8`.
- 2^26 steps, board 8192, all specs in `--check` mode. Analysis windows k = 17..26 (steps in
  (2^16, 2^26]), pooled sums. Runs that escape before finishing window 26 are excluded from the ant
  and rotor statistics (counted and reported). Random walks escape early; they are analysed on the
  windows they complete from k = 17 if there are at least 3, otherwise reported as unusable.
- Command: `scripts/run.sh` then `uv run experiments/2026-10-20-kappa-exposure-classes/scripts/analyse.py`.

## Registered predictions

- **P0 (instrument and reproduction).** Every spec passes `--check`. The 64 blank-board runs
  reproduce the committed census windows (`experiments/2026-10-13-probe-growth-census/results/probe_all.csv`:
  area, perimeter, new_cells, boundary_steps) exactly for every window k = 9..26 they share. (If the
  census grid differed from 8192 for an escaping rule, the mismatch is reported, not repaired.)
- **P1 (calibration, mean-field).** For each of the 8 mf seeds: |rho - 1| <= 0.01, |rho_1 - 1| <= 0.02,
  |rho_2 - 1| <= 0.03, |rho_3 - 1| <= 0.10; f_u totals below 0.1% of b_u.
- **P2 (the deficit survives a convention-free estimator).** Median rule-level rho over the 64 rules
  in [0.70, 0.92]; fraction of rules with rho >= 1 in [0.08, 0.35]. Rotor median rho in [1.45, 1.95].
  Random walk (if usable) in [1.05, 1.45].
- **P2b (rule by rule).** For the blank-board runs, the median over rules of
  |ln(rho_blank / (kappa_26 / 1.105))| <= 0.05, where kappa_26 is the probe kappa of window 26 of the
  same run and 1.105 is the census median kappa_unif. A failure means the 1.11 reference does not
  convert the probe's kappa into a time-averaged occupation ratio rule by rule.
- **P3 (main: where the deficit lives).** Three mutually exclusive outcomes on the median over rules of Q:
  - *H-exposed* (my preferred, registered as the prediction): **median Q >= 1.18 and Q > 1 for at
    least 75% of rules** - the ant returns to flat-edge cells more, relative to their share, than to
    corners, bays and tips. Reasoning: the depleted sub-edge layers (`fig:layers`, rho(d)) suggest
    occupation falls towards the outside, and exposed cells are the outermost.
  - *H-proportional*: 0.85 < median Q < 1.18 - the deficit is spread over classes.
  - *H-flat*: median Q <= 0.85 - the deficit is a shortage of returns to flat edge.
- **P4 (conditional on H-exposed).** Across rules, Spearman(rule-mean rho, rule-mean share of
  exposed boundary p_2+/P at window 26) <= -0.4: rules with rougher edges sit further below uniform.
- Rotor and random-walk Q values are reported without prediction (exploratory).

What would change my mind: H-proportional or H-flat outcomes refute the preferred picture; P2b
failing would mean the existing kappa/kappa_unif literature needs a per-rule correction before any
mechanism is read off it. Nothing here is a mechanism by itself: Q locates the deficit, it does not
explain it.

## Results

(Appended below after the run; predictions above are not edited.)
