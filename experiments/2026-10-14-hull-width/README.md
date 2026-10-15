# hull-width: is the ants' wide edge coastline roughness or porosity?

Registered 2026-10-14 22:15 NZDT, base commit `797704a` (dirty tree), **before any
`hull_width` run** (only its oracle unit test, `cargo test --release --example hull_width`,
was run: a synthetic square with two holes, expected counts hand-derived).

## Question

`W_surf` (sd of the radius of surface cells, Kapri-Dhar definition) grows as ~t^0.22-0.25 for
ants against ~t^0.10-0.13 for the rotor (`2026-10-13-a4-boundary-width`). The surface set is every
visited cell with an unvisited 4-neighbour, so it includes the rims of interior holes
(`notes/frontier-results.md`, P7: "not separated"). Territory fill is 0.76-0.98. Two readings:

- **H-hole (porosity).** The ants' extra width is lake/pocket rims scattered through the bulk;
  the outer coastline grows like the rotor's.
- **H-rough (interface).** The outer coastline itself roughens about twice as fast.

## Design

Instrument `examples/hull_width.rs`: exterior = unvisited cells 4-connected to the grid border;
*outer* surface cells touch the exterior, *hole* surface cells touch only enclosed unvisited
cells. Per checkpoint k = 16, 18, ..., 30 it reports A, P_all/P_outer/P_hole, number and area of
holes, W_all, W_out, the roughness of the angular profile r(theta) (512 bins, Fourier modes
m <= 2 removed) with its band-summed spectrum (bands 3-4 ... 129-256, in units of rbar^2), and
box counts of the outer cells at box sizes 1..64.

Cohort: the six A4 rules (RLR, RLRLRRRLLR, RLLLRLLRRLR, RLRLLRRLL, RLRLRRRRLR, RLLRLLRLRRRL), fresh
patched-start seeds 221-224 (24 runs, `RULE@SEED`), and rotor seeds 221-228 (8 runs), 8192^2
grid, `MAX_LOG2 = 30`. Runs that leave the grid end early and are listed, not dropped silently.
Calibration: the rotor is the reference walker; the synthetic-oracle test checks the hull split.

Estimators (fixed now): local exponent = least-squares log-log slope over k = 22, 24, 26, 28, 30
(per run); medians over runs, with between-run sd and a rule-level bootstrap (seed 20261014) for
ants. Spectrum exponent: slope of log2(band power) on band index over bands 5-8 ... 65-128
(five bands), equal to -2 zeta_loc for a self-affine profile; hull dimension D_h: slope of
log N against log(1/s) over s = 2 ... 32. All at the median over runs at k = 28 and 30.
The outermost-cell-per-bin profile is an extremal statistic and biased for small R; that bias is
the same for both walkers and is read off the rotor.

## Predictions (fixed now; not to be edited after the run)

Prior: I lean H-rough (about 60%) because the same ratio ~2 appears in the temporal and in the
radius-based fits, and holes are a small share of area (fill 0.76-0.98 is the loosest bound).

1. **Calibration.** Rotor W_out exponent lies in [0.08, 0.14]. If not, the instrument or the
   window is suspect and nothing below is interpreted.
2. **Holes are ants' business.** Median P_hole/P_all at k = 28: ants >= 3x the rotor's, and ants
   >= 0.10.
3. **Decisive.** Ants' median W_out exponent (k >= 22): >= 0.18 supports H-rough; <= 0.15
   supports H-hole (gap mostly porosity); in between is reported as mixed. The ratio
   (ants W_out exponent)/(rotor's) must be reported either way.
4. **Spectrum.** Ants' spectrum exponent zeta_loc exceeds the rotor's by >= 0.10 at k = 28 and 30
   (H-rough); under H-hole the two differ by < 0.10.
5. **Hull dimension.** Both D_h lie in [1.0, 1.4]; ants' exceeds the rotor's by >= 0.05.
6. **Decomposition check.** beta_out (log P_outer on log A, k >= 22) is within 0.03 of beta_all for
   both walkers (holes a roughly constant share of the edge). Not met means holes are a changing
   share, which would add a drifting factor to the edge in the three-factor decomposition.

## Decision rules

- 1 fails: stop, debug the instrument (compare W_all with `shape`'s Wsurf on the same specs; they
  must agree to 4 digits).
- 3 and 4 both H-rough: the wider edge is a property of the outer interface; next step is
  equal-time height correlations on the hull (KPZ-type exponents), not more windows.
- 3 and 4 both H-hole: revise the paper's edge claims to "porosity"; investigate hole statistics.
- 3 and 4 disagree, or 3 mixed: report as unresolved; do not summarise as either.

## Commands (WSL, from `/mnt/f/Code/turmite`)

```bash
export CARGO_TARGET_DIR=~/turmite-target
cargo test --release --example hull_width
cargo build --release --example hull_width
bash experiments/2026-10-14-hull-width/scripts/run_hull.sh
uv run experiments/2026-10-14-hull-width/scripts/hull_stats.py
```

## Status

Registered; run pending. Outputs go to `results/` (`hull_ants.csv`, `hull_rotor.csv`, `*.log`,
`hull.done`, `hull_stats.txt`). Check agreement with `shape` before interpreting (decision rule 1).

## Results (2026-10-14 22:55 NZDT; predictions above unchanged)

Run: `scripts/run_hull.sh` (commit `797704a`, dirty tree; 24 ant runs, 8 rotor runs, no escapes,
k = 16..30), then `scripts/hull_stats.py` -> `results/hull_stats.txt`. Decision rule 1 cross-check:
W_all equals `shape`'s Wsurf on 160 matched (spec, k) rows to 4 digits (max diff 0.0000).

| prediction | outcome |
|---|---|
| 1 calibration: rotor W_out exponent in [0.08, 0.14] | **held**, 0.135 (sd 0.062 over 8 seeds; edge of the band) |
| 2 holes: ants P_hole/P_all >= 0.10 and >= 3x rotor | **FAILED, direction reversed:** ants 0.038, rotor 0.252 (k=28) |
| 3 ants W_out exponent (k>=22) >= 0.18 -> H-rough | **held:** 0.239 (sd 0.049, rule-bootstrap [0.220, 0.247]); rotor 0.135; ratio 1.77. W_all exponents: ants 0.239, rotor 0.130 |
| 4 spectrum zeta_loc, ants - rotor >= 0.10 | met numerically (+0.183 at k=28, +0.200 at k=30; ants 0.63/0.60, rotor 0.44/0.40), **but see below: the direction of my reasoning was wrong** |
| 5 hull dimension: ants >= rotor + 0.05, both in [1.0, 1.4] | **FAILED, reversed:** ants 1.07/1.06, rotor 1.16/1.17 |
| 6 beta_out within 0.03 of beta_all | **held:** max difference 0.017 (ants), 0.016 (rotor) |

Reading (interpretation). The ants' wide edge is **not** porosity: removing hole rims leaves its
exponent unchanged (0.239 -> 0.239), and ants have few hole-rim cells (4%). The rotor is the
holey one (25% of its surface cells are hole rims), yet its W_out and W_all exponents also agree
(0.135, 0.130), so the surface-width comparison in the paper was not biased by holes in either
walker. H-hole is rejected; H-rough, as defined by prediction 3, holds.
Prediction 4 was badly specified: a larger spectral exponent means a *smoother* profile at small
scale, not a rougher one, and box counting agrees (ants' hull dimension is lower). Together they say
the ants' outer hull has fewer small-scale kinks than the rotor's yet a larger overall width, i.e.
the extra width sits at long wavelengths (lobes, notches), not in fine roughness. I did not register
that reading, so it is a hypothesis, not a result. Caveats: 6 rules x 4 seeds (the A4 rules, chosen
earlier and not sampled from the census); the r(theta) profile is the outermost cell per bin (extremal,
misses overhangs; zeta_loc over bins 5-8..65-128 uses only 5 bands, rotor sd 0.08-0.11); the
exterior is 4-connected (diagonal channels count as closed, unchecked per the notebook assumption);
all exponents are finite-window (k >= 22); D_h over s = 2..32 is a short range.
Next: equal-time height correlations/structure function of r(theta) at fixed k (registered first),
and a larger rule sample from the census.
