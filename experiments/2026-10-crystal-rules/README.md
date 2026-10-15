# crystal-rules: why do some rules draw diagonal "crystal" lattices?

Written 2026-10-14 19:03 (+13:00), commit 6ce3793. **Status: designed, nothing run yet.**
Every hypothesis and prediction below is registered before any measurement; append
results under "Results", do not edit this section.

## Observation that prompts it
Two 30M-step renders (user screenshots, `Downloads/RLLLLRRL_30M.png`,
`Downloads/LRRRRRLLR_30M.png`; a third rule, RLLLLLLLRRLRR, was named but not shown):
RLLLLRRL (2624x2630) and LRRRRRLLR (2824x2824). Both show

1. thin straight rays at 45 degrees to the image axes, hundreds to ~1500 cells long,
   which close into nested rotated rectangles/diamonds;
2. dense blobs at some corners/crossings, sparse elsewhere;
3. almost no re-use (busiest cell 164 / 139 visits in 30M steps) and a bounding box
   that is entirely "visited" because the rays sweep it.

This is not the t^(1/3) blob class. RLLLLRRL is already on record as a late grid
escape with no recognised highway (B5, `notes/frontier-2026-10-13-results.md`), and the
G1 breakout analysis found a phase switch to ~4 steps/cell
(`notes/research-frontier-2026-10-13-pm.md`, G1/K3). So the question may be the same
one as K3 / Q15 (frame builders): "crystal" may just be the frame-builder mode seen
at larger scale. Part of this experiment is to find out whether it is.

Assumptions (untested): (a) the renders are not rotated, so "45 degrees" is a diagonal
of the lattice, i.e. a staircase path (check in `experiments/2026-10-13-occupancy-pages/scripts/render.py` before S1);
(b) all three rules show the same phenomenon (S0 tests it).

## Hypotheses (mechanisms that make different predictions)
- **H-ray (glider in a medium).** A ray is a periodic translating pattern: the ant moves
  through background laid down by earlier passes, repeating a short motif of period p
  (a diagonal "highway" whose background happens to be compatible). Rays end when the
  ant meets a cell whose colour breaks the motif. Predicts: the ray's colour motif is
  the same in every ray (up to the 4 rotations), p is small, only a subset of the
  rule's colours appears in rays, and a ray replayed alone on a blank board does NOT
  propagate (it needs its background).
- **H-spin (spinner + launcher).** The long same-turn run (L^4 in RLLLLRRL, R^5 in
  LRRRRRLLR, L^7 in RLLLLLLLRRLRR) makes the ant circle a 2x2 loop while cells walk
  through the run's colours; it leaves the loop when the cells reach the end of the
  run. Blobs are the spin sites, rays what the ant does between them. Predicts: node
  size grows with run length k; crystal-ness depends on the run, not on the tail
  letters; Hamming-1 mutations inside the run destroy it, mutations in the tail
  mostly don't.
- **H-reverse (reversibility).** The dynamics are time-reversible, so a ray and the
  ant's later return along it are the same path in opposite directions; rectangles
  are the ant retracing earlier structure. Predicts: a large fraction of ray length is
  later traversed in reverse, and the 164-visit maximum is retracing.
- **H-frame (K3).** Crystals are frame builders in the K3 sense: perimeter patrols,
  width ~ t^(1/2), 1D skeleton. Predicts alpha_w = 0.50 +- 0.03 and 2-6 steps per
  new cell in ray mode. Distinguishable from the others by the exponent and by whether
  rays are self-similar across scales.

These are not exclusive (H-spin could produce rays whose substance is H-ray); the
stages are designed so each result moves belief between them.

## Instruments (to build, with oracle tests per AGENTS.md)
- **Ray segmentation**, `examples/crystal_rays.rs`: run a rule on a blank 8192^2 board
  with the `fast` loop and record positions. Split the trajectory into maximal
  windows that are a straight diagonal (|dx| = |dy| within 1 cell over >= 128 steps,
  steps per cell <= 8). Output per ray: start t, length, heading (4 diagonals),
  duration, colour sequence over 2 periods, terminating cell colour, and whether it
  overlaps an earlier ray's cells (and in which direction). Property test: the
  segmentation of `reference` and `fast` trajectories agrees step for step; a
  synthetic piecewise-diagonal path recovers its planted rays.
- **Crystal score** (census, cheap, grid-only): fraction of visited cells lying on a
  45-degree run of visited cells of length >= 30, minus the same for axial runs.
  Speed-agnostic (classic ant highways move 2 cells per 104 steps, so trajectory
  straightness would be wrong here).
- **Calibration (AGENTS.md rule 5)** before use: positive control RL (one diagonal
  highway), RLRRRL (frame builder, axial lines not diagonal), a seeded random walk and
  the rotor walker (`src/walkers.rs`) as negative controls. Report the false-positive
  rate of the crystal score on the 3,512 census rules (`experiments/2026-10-13-census-sweeps/results/non_highway.txt`) at the
  chosen threshold, and re-do the calibration if the threshold changes.

## Stages (each informs a decision)

**S0. Reproduce.** Regenerate RLLLLRRL, LRRRRRLLR, RLLLLLLLRRLRR on a blank board to
2^25 (deterministic, no seed). Confirm the renders match the screenshots and compute
the crystal score. Decision: are these three one phenomenon?

**S1. Anatomy of rays (H-ray vs H-spin vs H-reverse vs H-frame).** The three rules to
2^28: ray length distribution (log bins, tail exponent), heading set, colour motif and
period, steps per new cell in rays vs overall, fraction of ray length re-traversed in
reverse, what ends a ray (terminating colour, distance to the nearest earlier ray
along the heading), and the bounding-box width exponent 2^16..2^28 (`src/bin/probe.rs`).

**S2. Cause by intervention (the discriminating step).**
- *Ray transplant*: lift a mid-ray state (ant + strip of width 2p+4 around the ray
  axis) from the S1 run onto a board with (i) the original background, (ii) background
  reset to 0, (iii) background colours shuffled (seed 20261014). Metric: propagation
  length in units of the strip length. Machinery: `examples/transplant.rs`.
  Separates a self-sustaining highway from a medium-dependent ray.
- *Ray-end knockout*: at a terminating cell flip single cells in a 5x5 window and
  rerun; does the ray continue? Machinery: `examples/knock.rs`.
- *Spinner isolation*: blank board with the first k colours preset so that the ant
  spins; exit time and heading as function of k and the tail.

**S3. Mutation map (H-spin).** For each of the three rules, all Hamming-1 and Hamming-2
neighbours plus single-letter insertions/deletions in the long run (a few hundred rules
each); crystal score at 2^24; heat map of which positions matter. Control: the same scan
for 3 blob-class rules of similar length (RRRRRLLLLLL, RLLLLRRRRLR, RLRLLL), giving the
background rate at which a mutation changes the class.

**S4. Census (generalisation).** Crystal score for the 3,512 census rules at 2^24 and
for the unclassified rules of length <= 11. Features: max same-turn run k, net turn per
colour cycle (sum of turns mod 4), n, word symmetry (palindrome, reverse-complement).
Decision tree / logistic with leave-prefix-family-out cross-validation (rules sharing
a prefix have identical early trajectories, see the B5 note, so a random split
overstates independence). Decision: is "crystal" a clean class, and what predicts it?

## Predictions (fixed now)
1. S0: all three rules have >= 90% of visited cells on 45-degree runs by the crystal
   score and no recognised highway at 2^28. If one is axial or blob-like, the grouping
   is wrong and S1-S4 split into classes.
2. S1, H-ray: >= 80% of ray length (cells) shares one motif up to the 4 rotations,
   period p in [4, 40]. More than 5 motifs, or p > 100, falsifies H-ray.
3. S1, H-frame: bounding-box exponent 0.50 +- 0.05 over 2^20..2^28 for all three; if it
   is 0.33 +- 0.05, they are slow-filling blobs and the crystal picture is a
   finite-time crossover. My prior: 40% for 0.50, 40% in between (0.40-0.47), 20% for 0.33.
4. S1, H-reverse: <= 20% of ray length re-traversed in reverse (my belief: the 164
   max-visits are node blobs). > 50% supports H-reverse.
5. S2: transplanted ray with original background propagates >= 10x the strip length in
   >= 80% of trials; with background 0 it propagates < 1x in >= 80% of trials
   (medium-dependent). If the blank-background ray also propagates, it is a genuine
   highway and B5's "no recognised highway" is a detector failure (period > window).
6. S3: Hamming-1 mutations at positions inside the long run drop the crystal score
   below the 5th percentile of the control-scan scores in >= 60% of cases; mutations
   in the tail in <= 30%. Equal sensitivity falsifies H-spin.
7. S4: max same-turn run >= 4 in >= 80% of crystal-class rules, but < 50% of rules with
   run >= 4 are crystal (necessary, not sufficient). More than 20% of crystal rules
   with max run <= 3 falsifies H-spin as stated.

## What would show the framing is wrong
- No short motif (large motif count): "crystal" is the visual effect of long ballistic
  transits at scale, and the experiment becomes one about transit statistics.
- Ray length is fixed by the rule, not the geometry: correlation of ray length with the
  distance to the nearest earlier ray along its heading < 0.2 (> 0.5 means geometry
  sets it).

## Reproduce (planned; nothing run yet)
All via WSL from `/mnt/f/Code/turmite`:
```bash
cargo test --release --test crystal      # oracle test for the instrument
cargo run --release --example crystal_rays -- RLLLLRRL 28 experiments/2026-10-crystal-rules/dumps
uv run experiments/2026-10-crystal-rules/scripts/ray_stats.py
```
Budget: S0-S2 minutes; S3 ~1,000 rules x 2^24, minutes; S4 3,512 x 2^24, about an hour
on 6 cores with `rayon`. Deterministic; results keyed by rule; the only RNG is the
shuffled-background control (seed 20261014).

## Results
(none yet)

### Results so far (2026-10-14 19:38 (+1300), commit 6ce3793, uncommitted work)
Predictions above are unedited; verdicts here. Deviations (the crystal-score definition, the census path) are in NOTEBOOK.md.
- **S0 (done).** The renders are reproduced (max visits 164, 139). They are visit-count heat maps of a box that is ~100% visited: the "red" is a uniform ground state (visited 2x, colour 2: 80-83% of visited cells in all three rules). **Prediction 1 as written was mis-specified** (">= 90% of visited cells on 45-degree runs") and is not meaningful; the replacement (diag-run cover of non-ground cells, 0.96-0.98 vs 0.45 for the control RLRRRL) holds, but its "score > 3" threshold failed (1.95-2.32). The "no recognised highway at 2^28" part was not tested.
- **S1 (partly done).** Width exponent 0.495 / 0.503 / 0.502 over 2^20..2^28 (prediction 3: 0.50 +- 0.05, **held**, H-frame supported; also true for the control RLRRRL 0.499, so the exponent does not separate crystals from flat frame builders). Not done: ray length distribution, motif period, re-traversal fraction (prediction 2 and 4 untested), what ends a ray.
- **S2 (partly done).** A lone ant on a uniform ground of colour 2 is a true highway in all three rules (50 / 58 / 86 steps per diagonal cell), not on a blank board, and not for the control. This supports H-ray with the ground as the medium (prediction 5's logic, tested on a uniform board instead of a transplanted strip: blank-background propagation < 1x **held**; ray with original background not tested). A ground square of side 600-1200 reflects the ant: it stays inside while the box grows to 2-3.5x its side. Not done: transplant of an actual ray, ray-end knockouts, spinner isolation.
- **S3 (done).** Prediction 6 (run positions sensitive, tail not) **falsified**: all positions are sensitive; crystals are isolated under point mutation (0/142 Hamming-2). A different structure appeared: the family R L^k R R L is crystal-like (by the ground-fraction proxy, not the grid score) for k = 4..15; R L^k RRLRR for k >= 6; R L^k RRR LLL only for k=4. See the NOTEBOOK correction.
- **S4 (done, census length <= 12).** Prediction 7: all 8 crystal rules have max same-turn run >= 3 and 7 of 8 have a run >= 4 (the 8th, RLLLRRRLRLRR, has 3): held at 87.5% (n = 8; threshold >= 80%). Every crystal has ground (2x, colour 2); no ground-4x/6x rule is crystal. Necessary-not-sufficient part not computed.
- **Open:** why exactly ground 2 and a lone-ant highway on it are produced by the sweep (the 4-5 steps/cell fill is the same frame-builder mechanism as K3), what the ray end looks like, and whether k >= 4 is a threshold in the colours that turn L after the ground colour.

### Paired local-patch experiment registered 2026-10-14 22:15 (+1300), 7eff085
Select the first 12 nonoverlapping 4096-step windows within 2^25 with absolute dx >=64 and abs(abs(dx)-abs(dy)) <=4. Preserve the starting ant and 33x33 patch; set the exterior uniformly to colour 0 or 2. Replay 4096 steps, reporting displacement and the exact trajectory-prefix length against the intact baseline. Prediction: colour-2 exterior produces greater diagonal displacement than colour-0 in at least 9 of 12 paired samples per rule. This tests local-state propagation, not the originally proposed long-strip transplant or ray termination. Selection depends only on baseline displacement.

### Local-patch outcome (2026-10-14 22:16 +1300)
Prediction passed: 12/12 paired windows for each of the three rules travelled farther diagonally with colour-2 exterior than blank exterior. Inputs and SHA256: results/local_patch_summary.json; producer scripts/local_patch_summary.py under uv. The new trajectory loop passed its reference-oracle property test in release mode (results/trace_oracle_v2.log). This does not validate the old loops retroactively. Exact highway periods and boundary reflection remain unproven.
