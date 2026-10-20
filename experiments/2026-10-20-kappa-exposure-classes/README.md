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

### Run and analysis — appended 2026-10-20 09:51 UTC

Run: `scripts/run.sh` at commit `1593b61` (clean `src`/`examples`), 2026-10-20 09:46:44 UTC, 81 s on
8 threads; binary sha256 `0e6df95c...192e9f`, `results/classes.csv` sha256 `50ca6573...84f79` (full
hashes in `results/run.log`). All 344 specs ran in `--check` mode without a panic. A rerun with
`RAYON_NUM_THREADS=3` (no `--check`, into gitignored `scratch/`) is byte-identical.
Analysis: `uv run experiments/2026-10-20-kappa-exposure-classes/scripts/analyse.py` ->
`results/analysis.txt`, `results/analysis.json`, `results/rule_stats.csv`.

| prediction | registered | measured | verdict |
|---|---|---|---|
| P0 reproduction | every shared census window identical | 1152 windows (k = 9..26) over 64/64 blank-board rules, 0 mismatches | **held** |
| P1 mean-field | \|rho-1\| <= .01, \|rho_1-1\| <= .02, \|rho_2-1\| <= .03, \|rho_3-1\| <= .10 | all 8 seeds within (rho 0.9985-1.0020) | **held** |
| P2 ant median rho | [0.70, 0.92] | 0.874 (rule IQR 0.719-1.072) | held |
| P2 fraction rho >= 1 | [0.08, 0.35] | 0.312 | held |
| P2 rotor median rho | [1.45, 1.95] | 1.798 (8/8 seeds, sd 0.004) | held |
| P2 rw median rho | [1.05, 1.45] | 1.315 (8/8 usable, 7-8 windows each before escape) | held |
| P2b rule-by-rule conversion | median \|ln(rho_blank/(kappa_26/1.105))\| <= 0.05 | 0.036; ratio 5-95%: 0.931-1.086 | **held** |
| P3 H-exposed | median Q >= 1.18 **and** Q > 1 in >= 75% of rules | median Q 1.349 [rule bootstrap 95% 1.091, 1.593]; Q > 1 in 64% | **FAILED** (second clause) |
| P4 | Spearman(rho, exposed share) <= -0.4 | +0.197 (p 0.12) | **failed** (wrong sign) |

Sample: 256 patched runs, 5 excluded (escaped before window 26), so some rules have fewer than
4 seeds (minimum 1); all 64 rules retained. Between-seed sd (median over rules): rho 0.023,
Q 0.011, rho_1^ret 0.014, rho_2+^ret 0.011. Q is a reproducible property of a rule (seed sd 0.011)
with a wide spread between rules (5-95%: 0.58-2.38), so the P3 failure is not seed noise.

Class-resolved medians over rules: rho_1 0.66, rho_2+ 1.57; returns only: rho_1^ret 0.59,
rho_2+^ret 0.41. The first departure from a newborn cell accounts for a median 41% of ant boundary
steps (rotor 45%, rw 41%), close to c, as the bookkeeping requires.

Calibration walkers (exploratory, no prediction): rotor Q 1.60 (sd 0.003), rho_1^ret 1.14,
rho_2+^ret 0.71; random walk Q 1.03, rho_1^ret 0.78, rho_2+^ret 0.76; mean-field Q 1.00.

### Post hoc exploration (unregistered) — `scripts/explore.py` -> `results/explore.txt`

- Across rules, rho follows flat-edge returns (Spearman with rho_1^ret +0.93) much more than
  exposed returns (+0.66), and is unrelated to Q (+0.05) or to the exposed share of P (+0.20).
  Flat cells are ~72% of the ant's time-averaged boundary (w_1 0.72), so part of this is composition.
- Decomposing rho into contributions per unit time-averaged P/A (medians): ants: birth steps 0.33,
  flat returns 0.43, exposed returns 0.11 (sum 0.87); rotor 0.82 / 0.72 / 0.26 (1.80);
  rw 0.54 / 0.51 / 0.26 (1.31); mean-field 0 / 0.60 / 0.40 (1.00).
- Returns to exposed cells are below uniform for every non-uniform walker (rotor 0.71, rw 0.76,
  ant 0.41): reaching corners and tips by return rather than by birth is rare in general, not an
  ant peculiarity. The ant-specific feature is the flat edge: rho_1^ret 0.59 against 1.14 (rotor) and 0.78 (rw).
- Only 3 of 64 sampled rules have census c >= 0.6 (the crust proxy); all three have Q < 1 (0.50 median).
  Too few to say anything.

- **Board fillers (surprise).** Three patched runs left the blob regime without escaping:
  `RLLLRLLL@2` (births/step 0.4998 in window 26, area 3.35e7) and `RLRRRRLRR@1`, `@3` (both exactly
  0.250116, area 1.68e7). All other ant runs have births/step <= 0.0101. Both rules are stationary
  blobs from a blank board (their blank runs reproduce the census). The registered plan excluded only
  escapes, so these runs are in the registered rule means; they make two rule means absurd
  (rho ~ 140-150). Sensitivity with them dropped (post hoc, same seeds and windows): median rho 0.874,
  fraction rho >= 1 0.281, median Q 1.349 [1.099, 1.594], Q > 1 in 64.1%, Spearman(rho, exposed
  share) +0.21, Spearman(rho, rho_1^ret) +0.92, (rho, rho_2+^ret) +0.61, (rho, Q) +0.00. **No verdict
  changes.** The figure in `notebook.ipynb` (`results/returns_by_class.png`) excludes these three runs.
  After exclusion every ant rule has rho_1^ret <= 1.01, while the rotor is at 1.14.

### Interpretation and limits

Observations: the convention-free ratio confirms the deficit (ants 0.87 x uniform, rotor 1.80,
rw 1.31; mean-field exactly 1), and P2b shows that kappa / 1.105 recovers it rule by rule to a
median 3.6%. The registered H-exposed picture fails: the median of Q is above 1.18, but a third of
the rules have Q < 1, and edge exposure does not track the deficit across rules (P4 has the
wrong sign). Interpretation (post hoc, needs a registered test): the rule-to-rule variation of
boundary occupation is carried by **returns to flat edge**, and that is where ants differ most from
the rotor and random walk. Q mostly records a property common to all non-uniform walkers: corners
and tips are reached by births, not by returns.
Limits: one cohort of 64 stationary rules, windows (2^16, 2^26], one board size; exposure is
confounded with age, colour and depth, which were not recorded; the exploratory split was chosen
after seeing the data.

### Next test (proposed, not registered)

Register on a fresh rule sample: (i) Spearman(rho, rho_1^ret) >= 0.85 and rho_1^ret explains more
between-rule variance of ln rho than the exposed-return term after removing the composition
effect (hold w_1 fixed by reweighting); (ii) resolve flat-edge returns by the age of the edge
cell (steps since birth), to test whether the ant's flat-edge shortfall is concentrated in
recently born edge (a short-horizon effect) or spread over old edge (a long-excursion effect).
