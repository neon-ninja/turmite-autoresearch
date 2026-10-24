# Square-ring fillers: why `R^k L R^(k+2)...` fills the plane at 2(k+1) steps per cell

Registered 2026-10-25 10:31 (+13:00), base commit `b942a82`. **No registered measurement has been run** when this file is
first committed. Exploration done before registration is listed in full under "What I looked at before registering".
Author: Corvid (AI research agent, Claude). Open questions addressed: Q20 (filling mechanism of the board fillers),
Q15 (are the 289 census "slow fillers" one class?), and the link to Q23 (spiral builders) in
`notes/open-questions-hypotheses.md`.

## Question

The late-highway board check (`experiments/2026-10-21-late-highway-board-check/`) found that 25 "late highways" were board
fillers of the form `R^k L R R...` (k = 1..5) that cover a 32768 board at `2(k+1)` steps per cell, and that its registered
onset formula `2(k+1) G^2` missed by a perimeter term (B1 failed at 1%: shortfall 1.3%, 0.6%, 0.3%). The census
"builders" (`experiments/2026-10-13-b4-unfitted-rules/results/b4_builders.txt`) are 289 rules that fill a square at about
2-6 steps per bounding-box cell, never examined as a class. Nothing in the record explains *why* these rules fill, why the
number is `2(k+1)`, or which rules can do it.

## Proposed mechanism (registered)

**The square-ring crawler.** The filler's boundary is a three-row profile that a periodic "crawler" walks along. Write
`a = k`, `b = k+1`, `c = 2k+1`, `d = 2k+2` for visit counts (a cell's colour is its visit count mod n; `0` = unvisited).
Coordinates: x east, y south (as `DIRS`), x mod 4 taken in 0..3. The explicit configuration `C_k` (crawler on the north
side of the territory, moving west) is:

| row | x < 0 ("ahead") | x = 0 | x >= 1 ("behind") |
|---|---|---|---|
| y <= -2 | 0 | 0 | 0 |
| y = -1 | 0 | 0 | a if x mod 4 in {1,2}, else 0 |
| y = 0 | a if x mod 4 in {2,3}, else 0 | a (ant here, heading N) | c if x mod 4 in {1,2}, else b |
| y = 1 | c if x mod 4 in {2,3}, else b | d | d |
| y >= 2 | d | d | d |

So "ahead" the boundary rows read (blank, a-pairs, c/b-pairs, d...), "behind" they read (a-pairs, c/b-pairs, d, d...): the
crawler moves the profile out by one row. Per 4 columns the visit counts rise by `(2k+1) + (4k+2) + (2k+2) + 2 + 1 =
8(k+1)`.

**Why the rule must read `R^k L R^(k+1)`.** Exploration (below) shows the crawler's period decomposes into 2k+2 blocks of 4
steps, most of which are closed 2x2 square loops (four R turns that return the ant to its start cell). A fresh 2x2
plaquette is looped while its three new cells read 0..k-1 (turn R); at count k they turn L and eject the ant to the next
plaquette, while the fourth (shared) cell reads k+1..2k+1 during those loops (turn R). Every cell is therefore read at each
count 0..2k+1 exactly once (k+1 times as a fresh cell, k+1 times as the shared cell) and ends at d = 2(k+1) visits. The
interior colour d is read only at corners, so the turn `T(2k+2)` matters only there. Here `T(v) = rule[v mod n]`.

Membership predicate **Pi(k)**: `T(v) = R` for all `v` in `[0, 2k+2]` except `T(k) = L` (for n >= 2k+3 this is the prefix
`R^k L R^(k+2)`; shorter rules qualify by wrap-around, e.g. `RLRR`, `RRLRR`, `RRLRRR`, which are single-L rules).

## Predictions (registered before any run)

**P1 (explicit crawler, all k).** For every k = 1..64, under the minimal rule `R^k L R^(k+1)` and under 4 further rules with
the same `T(0..2k+1)` and random suffixes (seed 20261025), `C_k` evolves after exactly `8(k+1)` steps into `C_k` translated
by (-4, 0) with the ant at (-4, 0) heading N, for 16 consecutive periods, reading each count 0..2k+1 exactly 4 times per
period and nothing else. Confidence 90% (k = 1..3 partly seen in exploration; k >= 4 unseen as a construction). **Controls:**
flipping any single `T(v)`, v in [0, 2k+1], breaks the translation within one period (expected by construction, a check on
the instrument); flipping `T(v)` for v >= 2k+2 leaves the trajectory unchanged. Falsified by any k <= 64 that fails.

**P2 (exact revolution clock).** For each of the 25 census board fillers (k = 1..5; list in
`experiments/2026-10-21-late-highway-board-check/results/classify.txt`), a blank-board run of 2^27 steps has, after its
onset, an exact clock in the registered sense of `experiments/2026-10-23-sweeper-generalisation/scripts/clock.py` (cycle P = 1
in a direction, >= 95% of cycles with zero residual, p <= 6) with slope **a = 8(k+1)**, in each of the four directions,
with p in {1, 2}. Window: the last half of the run (2^26..2^27). Confidence 85% for a = 8(k+1) in all 25; 70% for p <= 2.
The offsets b are not predicted (they include the ring's aspect offset, which depends on the seed).

**P3 (footprint).** In the same window, for every one of the 25 rules: the maximum count read is 2k+2; each count 0..2k+1
accounts for a share within 2% (relative) of 1/(2k+2) of the reads; count 2k+2 accounts for < 0.5%; new cells per step are
within 2% of 1/(2k+2). Confidence 85%.

**P4 (board exit steps, closes the B1 failure).** For `RLRRRRL` (k = 1), `RRLRR` (k = 2), `RRRLRRRRR` (k = 3), the exit steps
E(G) on boards G = 1024, 2048, 4096 (committed: `.../2026-10-21-late-highway-board-check/results/board_{1024,2048,4096}.csv`,
not yet looked at beyond the 3-figure values quoted in that README) determine a quadratic `E = alpha G^2 + beta G + gamma`.
Predictions: (i) alpha = 2(k+1) **exactly** (as a rational number); (ii) the same quadratic predicts, to the step, E(8192)
and E(16384) (new `sweep_lean` runs) and E(32768) (the census record in
`experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/results/h1e10/` or a rerun). Reason: front f advances once per
revolution, revolution r lasts `8(k+1) s_r + b_(r mod 2)` with `s_r` linear in r, and the ant leaves the board exactly at
the front-advance event f = G/2 (same exit side and parity for all G = 2^j). Confidence 70% (fails if the exit side or
parity changes with G, or the clock is not exact by f = 512). The registered B1 shortfall is then `-beta/(2(k+1)G)`.

**P5 (the corner reads T(2k+2)).** Take the state of `RLRRRL` (k = 1) at t0 = 1e6, `RRLRRRR` (k = 2) at t0 = 2e6,
`RRRLRRRRR` (k = 3) at t0 = 3e6 (mid-fill, from blank), and continue (a) with a rule identical on counts 0..2k+2
(control: trajectory identical to the original for 1e8 steps), (b) with `T(2k+2)` flipped to L and nothing else in
0..2k+1 changed (`RLRRLL`, `RRLRRRL`, `RRRLRRRRL`; counts carried over as integers). Prediction (b): the ring breaks at the
first corner: within one revolution the ant reads a count > 2k+2 or the front stops advancing every revolution.
Confidence 75%. (a) is a check on P3 (an identical trajectory follows if the footprint stays in [0, 2k+2]).

**P6 (census membership, Q15).** A footprint instrument runs every rule of the union U = {304 census builders
(`experiments/2026-10-13-census-sweeps/results/census.csv`, cls = builder)} plus {all canonical rules of length <= 12 in
`sweep_2_12_100M.csv` satisfying Pi(k) for some k >= 1} from a blank 16384 board for 2^27 steps and classifies the final
window W = 2^24 steps:
- **ring(k)**: max count read <= 2k+2, each count 0..2k+1 has share within 5% (relative) of 1/(2k+2), count 2k+2 < 1%,
  new cells per step within 5% of 1/(2k+2);
- **two-visit**: counts 0 and 1 together >= 90% of reads;
- **other** (includes runs that left the board before 2^27; their last full window is classified and flagged).

Predictions: (a) **necessity**: every rule classified ring(k) satisfies Pi(k) for the same k: 100% expected, confidence 90%;
(b) **sufficiency rate**: among the Pi rules (361 expected by enumeration), the fraction classified ring(k) is between 30% and
90% (point guess 60%; no mechanism predicts which seeds reach the ring); (c) **Q15**: among the 304 census builders, at least
50% are ring(k) (point guess 65%) and between 10% and 30% are two-visit; so Q15's "one class" fails, but a majority is one
mechanism. Falsified if (a) has any exception, or ring(k) < 50% of builders.

What each outcome changes: P1 + P3 hold -> the 2(k+1) law and the family are explained by the crawler (Q20 answered for the
straight edge). P2 + P4 hold -> the board-filler "late highways" are fully predicted, including B1's perimeter term. P5 (b)
holds -> the corner gadget needs `T(2k+2) = R`, which completes the predicate. P6 fails (a) -> another mechanism produces the
same footprint, and the crawler is not the only ring. P6 (c) -> Q15 answered in counts.

## What I looked at before registering (exploration, disclosed)

All with a throwaway Python ant (scratchpad, not committed), blank board, heading N:
- `RLRRRL` to 4e5 and 2e6 steps: 97% of cells have exactly 4 visits; reads after 1e6 steps are counts 0..3 (about 25% each)
  plus 621 reads of count 4; xmax front advances once per ~11e3 steps with dt alternating (+72, -8) around a slope of 32 per
  advance (about 16 per unit of side), i.e. a ~ 16 = 8(k+1) for k = 1 was **seen** before registering.
- `RRLRRRR`, `RRLRR` to 1e6: spc 5.96, reads 0..5 about equal, ~240 reads of 6. `RRRLRRRRR`, `RRRRLRRRRRR`: one period each.
- Non-fillers from blank at 1e6 steps: `RLRR`, `RLRRR`, `RRLRRR` (spc 34-57); `RRLRRRRL` at 1.5e6 was not in the ring regime.
  These satisfy Pi, so blank-board onset is not implied by Pi.
- `RLLLLLRRL`, `RLLLLRRL` (~4.2 steps per cell, reads of 0 and 1 about 56%, long tail) and `RLLLRRRL` (reads of 0, 1 = 98%)
  are not ring rules.
- Crawler read sequences for k = 1, 2, 3 (period 16, 24, 32; only count k turns L) and edge snapshots for k = 1, 3, from which
  `C_k` above was written down. The translation of `C_k` itself has **not** been checked.
- I have not looked at the census builder list beyond the 12 rows printed in `b4_builders.txt`, at the exit steps in
  `board_*.csv` beyond the README's 3-figure values, or at any census row of a Pi rule.

## Pruning-index preflight

- Index revision `b942a82854b4d61222301959b9895060d2543d3f`. Commands: `rg --files notes/pruning-index/entries` (one entry);
  `rg -n -i -e filler -e builder -e crawler -e square -e RLRRR notes/pruning-index/entries` (no output).
- Candidate: `NR-20261022-additive-laws-order-blind` only (additive conservation laws for rules of length 2..8). Its
  objective is conservation laws, not growth mechanisms or classification; it does not cover this test.
- Decision: **run**. No signature covers fillers, rings or the builder class.
- Budget: CPU only, 4 cores. P1 seconds; P2 25 runs x 2^27 steps; P4 six runs <= 1.1e10 steps total; P5 six runs of 1e8 steps;
  P6 <= 700 runs x 2^27 steps (~1e11 steps, < 1 h). No census expansion, no runs beyond these.

## Relation to `notes/ACTIVE.md`

ACTIVE (2026-10-23) puts boundary access first and defers new proof programmes. This is a bounded regime-validation item
(Priority 2: board artefacts and which census rules are outside the cube-root regime) that answers the open Q20/Q15 with
cheap, exact tests. It is not a Lean proof programme: P1 is a finite computation per k plus a written argument. Deviation
noted in `NOTEBOOK.md`.

## Commands

Filled in as run (see `NOTEBOOK.md` for timestamps and hashes).

## Results

(none yet; appended below without editing the registration)
