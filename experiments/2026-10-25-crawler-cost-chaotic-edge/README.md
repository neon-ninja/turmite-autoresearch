# Is a chaotic edge a broken crawler? The crawler's cost per birth as a ceiling for the c law

Registered 2026-10-25 23:30 (+13:00), base commit `56aac50`. **No registered measurement has been run** when this file
is first committed. Everything I looked at beforehand is listed under "Seen before registering".
Author: Kea (AI research agent, Claude). Open questions: the unexplained c-law coefficients (`a ~ 1.36`, `b ~ 0.70`) and
"why letters 5-8 matter" (`experiments/2026-10-19-kappa-chiral-edge`, C4; `experiments/2026-10-19-c-law-mechanism`,
closing entry), and Q20/Q15 (`notes/open-questions-hypotheses.md`).

## Question

The square-ring crawler (`experiments/2026-10-25-square-ring-fillers`, lemma proved for all k >= 1) is an exact,
ordered solution for a growing straight edge. Every cell it creates costs `2(k+1)` steps. The chaotic rules obey an
empirical edge law, `1/c = 1.363 + 0.696 m`, where `c` = births / boundary steps and `m` is the opening same-letter run
(the lead). The law transfers to fresh rules and to a flat band with an i.i.d. interior, and it does **not** depend on
the interior's colours (c-law-mechanism Stage 2c: all-zero, young-only and old-only interiors change c by 0.3%). So `c`
is set by an edge layer that the ant builds for itself. The crawler is one such layer, built perfectly. Does the
crawler's exact accounting say anything about the chaotic one?

## The crawler's edge accounting (derived from the lemma; checked numerically in P0)

Use the edge tracer's definitions (`src/edge_trace.rs`). A *boundary read* starts on a visited cell with an unvisited
4-neighbour, and a *birth* is a step onto a never-visited cell. `r_j` = boundary reads at colour `j` per birth, so
`1/c = sum_j r_j`.

The crawler's period has `8(k+1)` steps and 4 births: a chain of three (plaquette P, three fresh cells and one shared
cell) and a chain of one (plaquette Q, one fresh cell and three shared cells). Each fresh cell is read at counts
`0..k-1` while its plaquette is circled, then at `k` (the L that ejects the ant), all while it still has an unvisited
neighbour. Its later reads `k+1..2k+1` happen as a *shared* cell of the next pass's plaquettes. By then the next row
has been born, so all of these reads but one in four are interior. Hence

    r_j = 1 (0 <= j <= k),   r_(k+1) = 1/4,   r_j = 0 (j >= k+2),   1/c_crawler = k + 5/4,

with birth chains of length 1 and 3, half each, and none of length 2. A blank-board tracer run at 2^22 steps showed
exactly this profile for `R^5 L R^6` and `R^7 L R^8` (see "Seen before registering").

Set this against the chaotic law at `m = k`:

    crawler: 1/c = 1.25 + 1.00 m          chaotic: 1/c = 1.363 + 0.696 m

The intercepts agree within 0.11. The slope is the naive "2x2 loop" slope `b = 1`, which C3 of the chiral-edge
experiment rejected for chaotic rules (that picture also had `a = 0`; the crawler shows that `b = 1` comes with
`a = 5/4`, not 0). So, read through the crawler, **a chaotic edge spends 30% fewer boundary reads per lead step than a
perfect crawler**, and its intercept is the crawler's.

## Hypothesis H-crawl (registered)

A chaotic edge is a fragmented crawler. Fresh plaquettes are circled as in the crawler, at a cost of one boundary read
per birth per circulation. A circulation breaks when a non-fresh (shared) cell of the plaquette reads a letter other
than `s_0`. In the crawler, the shared cells are young edge cells at counts `m+1..2m+1`, and the rule supplies `s_0` at
exactly those counts: the membership predicate `Pi(m)` (`T(v) = R` for `v` in `[0, 2m+2]` except `T(m) = L`; the
corner reads `2m+2`). The 30% shortfall is the circulation lost to unsupported shared cells. The crawler,
`1/c = m + 5/4`, is the ceiling that full support reaches.

This makes two kinds of prediction that the lead law alone does not:

1. **Crawler limit.** Chaotic rules that satisfy `Pi(m)` have the crawler's full local turn table, but they do not
   settle into a ring. There are 40 such stationary census blobs with `m >= 2`, and all 40 are "other" in the
   square-ring P6 classification to about 1.3e8 steps. Their edge should be crawler-like, with `1/c` near
   `m + 5/4`, above the lead law.
2. **A lead-dependent letter window.** Letters at positions `p` in `[m+1, 2m+1]` (the shared-cell window) should
   matter more for `c` than letters at `p >= 2m+3`, and the window should move outward as `m` grows. This is a
   candidate mechanism for C4's "information keeps accumulating with each early letter", with a horizon that depends
   on `m`.

## Competing explanations

- **H-young.** Circulation is supported by shared cells that are themselves young (colour `< m`), so letters after
  the lead hardly matter for circulation. `Pi` membership is then irrelevant: Pi-blobs sit on the lead law, and
  single-letter flips show no lead-dependent window.
- **K3-generic** (c-law-mechanism Stage 1). The influence of a letter decays smoothly with its position through a
  fitted survival kernel that does not depend on `m` beyond the lead. This predicts no step at `2m+1` and no outward
  movement with `m`.
- **Composition.** `Pi(m)` rules carry many `s_0` letters, so any "more R letters means more circulation" effect would
  mimic prediction 1. P1b controls for this by matching composition. The flip design (P2) changes composition by
  exactly one letter in every arm.

## Seen before registering (disclosure)

- The whole record of the c-law experiments (chiral-edge, c-law-mechanism, c-law-renewal, both lattice c-law
  experiments), the square-ring fillers README and results, and the P6 classification of the 40 Pi-blobs (class labels
  only).
- **Exploratory tracer runs** (2026-10-25 ~23:00; `c_law_mechanism_trace blank 22 2048`, minimal rules
  `R^k L R^(k+1)`, k = 1..8; output not kept): `1/c` = 2.320, 3.175, 4.089, 5.149, 6.241, 7.061, 8.231, 8.972. For
  k = 5 and 7 (ring fillers), the r-profile was exactly the crawler's. For the others (not fillers), chain-2 events made
  up about 10-30% of chains. **Three of the 40 P1 rules (`RRLRRR`, `RRRLRRRR`, `RRRRLRRRRR`) were in this run.** Their
  tracer `1/c` lies at about 0.8-0.9 of the way from the lead law to `m + 5/4`. This is why I give P1 high odds. P1 is
  also scored without these three rules. I have not looked at the census `c` of any P1 rule, or at any tracer output
  for the other 37.
- Cohort counts by lead and `Pi` membership (rule strings only, no outcome).

## Predictions (registered before any run; my honest odds)

**P0 calibration (exact).**
- **P0a.** An independent Python simulation of the crawler configuration `C_k` on a strip (k = 1..8, 64 periods) gives
  exactly `r_j = 1` (j <= k), `r_(k+1) = 1/4`, 0 otherwise, `1/c = k + 5/4`, and chains {1: 1/2, 3: 1/2}. ~90%.
- **P0b.** The edge tracer on the 25 census board fillers (blank, 2^27 steps, board 8192, window [2^26, 2^27], the
  square-ring P2 setting, where all 25 are in the ring regime) gives `|1/c - (k + 5/4)| <= 0.05` and a chain-2 share
  `<= 0.02` for every rule. ~80%. A failure here would mean the instrument and the derivation disagree, and P1-P3 would
  be read with that caveat.

**P1 crawler limit (census `c`, `probe_summary.csv`, seen-but-never-analysed data).** Cohort: the 40 stationary census
blobs (`cls == blob`, `c > 0`, `|g_k| < 0.05`) with `Pi(m)` and `m >= 2`. Write `LL(m) = 1.363 + 0.696 min(m, 8)` and
`phi = (1/c - LL(m)) / (m + 5/4 - LL(m))`, so 0 = on the lead law and 1 = on the crawler line.
- **P1a.** median `phi` >= 0.5 (all 40, and separately without the 3 seen rules). ~70%.
- **P1b.** Composition-matched controls are stationary blobs with the same `m`, the same `n` and the same number of
  `R`s that are not `Pi`. For each Pi rule I draw up to 3 controls (seed 20261025); if none exist, I relax `n` by ±1,
  then the R-count by ±1. The test: median `phi`(Pi) minus median `phi`(controls) >= 0.3, with a 95% bootstrap CI over
  Pi rules (2000 resamples, seed 20261025; controls resampled with their Pi rule) whose lower bound is above 0. ~65%.
- **P1c (ceiling).** At most 10% of the 40 have `1/c > m + 5/4 + 0.10`. ~50%. This is a bold claim: no fragmented edge
  spends more boundary reads per birth than a perfect crawler. Hole rims could break it.

**P2 window (new tracer runs, single-letter flips).** Parents: stationary census blobs with `n` in 10..12, not `Pi`,
30 per `m` in {1, 2, 3} (seed 20261025). Mutants: each parent with one letter flipped at every position `p` in
`[m+1, n-1]`. The lead and the canonical first letter are unchanged; non-primitive mutants are dropped. Tracer:
`blank 24 4096`, window [2^23, 2^24]. A run is kept if it does not escape, has births >= 200 and has
1.3 <= area/area_t0 <= 1.9 (the c-law V filter). Effect `e = |1/c_mut - 1/c_par| / (1/c_par)`. Position classes:
window `W = [m+1, 2m+1]`, corner `2m+2`, far `F = [2m+3, n-1]`.
- **N0 (noise gate).** Parents are also run at 2^25, and the gate is median `|1/c(2^25) - 1/c(2^24)| / (1/c(2^24))`
  <= 0.5 × median `e` over all kept W mutants. If it fails, P2 is "unresolved (finite-window noise)", not a verdict.
- **W1.** For each `m` in {1, 2, 3}, over parents with at least one kept W mutant and one kept F mutant: median of
  `mean e(W) / mean e(F)` >= 2. Scored per `m`. ~35% for each of m = 2 and 3; ~25% for m = 1 (the k = 1 crawler has no
  circulation loops, phases 3 and 6 are empty, so the window can only act through the transfer between plaquettes).
- **W2 (the window moves with m).** At relative positions `q = p - m` in {3, 4}, which lie inside the window for
  `m = 3` and outside it for `m = 1`: median `e`(m = 3) >= 1.5 × median `e`(m = 1). Both K3-generic (fixed in `q`) and
  an absolute-position decay predict a ratio of 1 or less. ~30%.
- **W3 (sign).** Pooled over m = 2 and 3, kept W mutants. Among flips that turn `s_bar` into `s_0`, the fraction with
  `Delta(1/c) > 0` is >= 0.70. Among flips that turn `s_0` into `s_bar`, the fraction with `Delta(1/c) < 0` is >= 0.70.
  ~40%.
- Reported, not scored: kept fraction by position class and `m` (survivor selection), mean signed `Delta` by `q` and
  `m`, and the corner position.

**P3 mechanism (tracer, the P1 cohort and its controls).** Same tracer settings as P2.
- **R1.** For each rule, `rbar` = mean `r_j` over the circulating colours `j = 1..m-1`. The test: median `rbar`(Pi)
  minus median `rbar`(controls) >= 0.05. ~50%.
- **R2.** The chain-2 share is lower in Pi rules than in controls by >= 0.05 (median difference), meaning their
  plaquettes look more like the crawler's 3 + 1 pattern. ~45%.
- **R3 (replication of P1a on the tracer).** median `phi`, computed from tracer `1/c`, >= 0.5. ~65%.

## Decision rule and readings

- P1a, P1b and R3 hold: full shared-cell support pushes chaotic `c` toward the crawler line. H-crawl's crawler limit
  stands. H-young is rejected for Pi rules.
- If, in addition, W1 holds for at least two `m` and W2 holds: the window is positional and moves with `m`, a mechanism
  for C4's "letters 5-8 matter". Next steps would be (i) a closed formula for `b` from the support probability, and
  (ii) a flat-band test with the window letters as the controlled variable.
- P1 holds but W fails: support matters as a property of the whole table, not through a sharp window. The positional
  story is wrong in detail.
- P1a or P1b fails: the crawler is an isolated ordered solution and does not organise the chaotic edge's cost. That
  outcome would go into the pruning index as an empirical record scoped to these rules, instrument and horizons.
- P0b fails: report the disagreement first, and stop interpreting P1-P3.

## Pruning-index preflight

- Index revision `56aac506a75228ea148c1d6dea753877eecd3f4d`. Queries: `rg --files notes/pruning-index/entries`;
  `rg -n -i -e crawler -e "birth rate" -e "c law|c-law" -e "boundary steps per birth" -e "Pi\(k\)" -e lead
  notes/pruning-index/entries`. I also read the titles and predicates of both entries.
- Candidates: no matches. `NR-20261022-additive-laws-order-blind` concerns additive invariants (a different
  objective). `NR-20261025-kappa-exposure-classes-exposed-edge` concerns kappa by exposure class, not c and not letter
  positions.
- Decision: run. This does not repeat any recorded test. The closest earlier work is H-loop (square-loop,
  2026-10-16: kappa, interior loop rate) and the c-law kernels K3/K4. Both were fitted kernels without a crawler
  ceiling or a lead-dependent window.
- Budget: at most about 1,200 tracer runs of 2^24 to 2^27 steps (under 1e11 steps), about 30 minutes on 4 cores.
  Nothing larger without a new registration.

## Commands

Filled in as run.
