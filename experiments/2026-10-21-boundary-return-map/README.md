# Is there a compact boundary state? Colour-intervention test of the return map

Created 2026-10-21 07:30 +13:00, base `bd60f4792a57c627d290531963e2e45fddb19fbe` (dirty workspace: other sweeps' `h1e11` chunks are untouched by this experiment).
Status: **REGISTERED, nothing measured yet.** Predictions below are fixed before any sampler run.

## Question

A "return map" `R: B -> B` takes the ant's state when it leaves the boundary (a visited cell with an unvisited
4-neighbour) to its state at the next boundary arrival. A proof or scaling argument that works on `R` instead of the
full ant needs a summary `s` of the departure configuration for which `R` is well defined, or at least a Markov kernel.
Which summary? Two structural facts constrain the answer:

1. While the ant is interior the **territory cannot change**, so the only hidden state is the colour field on visited
   cells (plus position and heading). An interior excursion is a deterministic function of (position, heading, colours).
2. Earlier work found the interior time budget is dominated by deep excursions (first-passage README: depth-64 carries
   more than half of E[T] in 35 of 46 rules). Exact closure on a finite window therefore cannot be expected to be
   enough; the question is how far out the colour field matters.

This experiment measures that reach directly by **intervention**, not by fitting a state partition. (A maximum-likelihood
chain scored on its own data cannot fail, which is why the first-passage closure test was uninformative; here the
quantity is a causal effect of replacing colours, with controls.)

### Relation to the pasted proposal (no new data used)

The pasted heuristic (width L, boundary-hit rate, `dL/dt ~ 1/L^2`, hence `L ~ t^(1/3)`) is the repo's existing identity
`dA/dt = c * kappa * P/A` with uniform boundary access (`notes/cube-root-hypothesis.md`, 2026-10-10): with `A ~ L^2`,
`P ~ L`, constant `kappa` gives `dA/dt ~ 1/L` and `L ~ t^(1/3)`. Its *diffusive* version (return time to the boundary
`~ L^2`, flat boundary dwell `E[D] ~ 1`) gives boundary fraction `f ~ L^-2`, `dA/dt ~ L^-2`, `L ~ t^(1/4)`: the
internal-DLA exponent already named in the cube-root note. The census has `kappa ~ const`, i.e. `f ~ 1/L`, so
`E[T]` must scale as `L`, not `L^2`; the diffusive reading is already contradicted by the census. `E[T] ~ L` is
*bookkeeping* given constant kappa and flat `E[D]` (2026-10-21 correction in `notes/kappa-mechanisms-2026-10-15.md`);
this experiment does not cite it as support.

## Instrument

`examples/boundary_return_map_sampler.rs` (written after this registration). For each run (`RULE@SEED`, patched start
radius 2, the first-passage walker) the ant runs on an exact board until area first reaches each band lower edge
A0 in {2^15, 2^17}; within the band `[A0, A0*17/16)` departures (boundary -> interior transitions) are sampled
(first `K = 48` departures spaced by at least 64 steps, a deterministic rule, no randomness). At each sampled departure:

* Crop the territory (bounding box plus 1) and the colour field. `inner` = visited cells with all four neighbours
  visited. The interior episode ends at the first step that lands on a visited non-`inner` cell. Cap `2^22` steps;
  hitting the cap is censored and recorded.
* **Baseline** episode: outcome `(T, dx, dy, arrival heading, arrival colour)`, set of cells visited, maximum depth.
* **Intervention** at radius `r` in {0, 1, 2, 4, 8, 16, 32, 64, 128}: every visited cell at Chebyshev distance `> r` from the
  departure cell has its colour replaced by a **within-depth-class shuffle** (depth = BFS distance through the
  territory to the nearest boundary, classes 0..63 and "64+"; the shuffle is a seeded Fisher-Yates over the cells of each
  class, so the per-class colour multiset is preserved exactly). `M = 8` replicas per (departure, r). Territory,
  position, heading and all colours within radius `r` are unchanged. Replicas run to the next boundary arrival.
* Interventions that change nothing (shuffle fixed point) are retained, not discarded; identical outcomes are the point.

Why the shuffle is a strong perturbation: it destroys spatial structure outside `r` while preserving each depth class's
colour marginals. It measures "how much of the outcome is determined by the ball `r`", not a small sensitivity. A
small-perturbation arm (single-cell flip at distance rho) is not in this registration.

Interpretation limit (fixed before measuring): a change under the shuffle may come from a specific colour or from
loss of spatial structure (stripes, crystal regions); this design cannot separate them. P1 and `r*` therefore state
how far out *structure* matters. A large reach is not evidence that distant individual cells control the ant; that
needs the single-cell flip arm, which is unregistered.

Sampling (revised 07:50, before any pilot): departures are stratified by the baseline episode length
(`T <= 3`, `4..31`, `32..511`, `>= 512`), up to `K` per stratum per band, so rare long excursions are represented.
The live ant keeps an undo log of overwritten colours and the departure field is rebuilt once the length is known;
the rebuilt baseline must reproduce the live arrival (control C1b). Per-band histograms of `T` over *all* departures
give the stratum weights for population estimates.

### Statistics

* `Y = log2(1 + T)`. For each (rule, band, r), `ICC_r` is the one-way random-effects ICC across departures with the `M`
  replicas as within-group draws (ANOVA estimator, pooled over seeds; between-seed sd reported).
* `ICC_0` (everything but the current cell perturbed) is the geometry-and-state floor: departures differ in territory,
  position, heading even when colours are fully randomised. `G(r) = (ICC_r - ICC_0) / (1 - ICC_0)` is the fraction of
  colour-dependent outcome variance captured by the ball. `r* = ` smallest `r` with `G(r) >= 0.8`, interpolated in
  `log r`; reported as `r*/L`, `L = A/P` at the band lower edge.
* Exactness: identical return `ID_r` = fraction of replicas with identical `(T, dx, dy, heading, colour)`.
  `Utilde_r` = fraction of replicas whose baseline path never visits a cell whose **turn** the intervention changed.
  `E_r = ID_r - Utilde_r` (excess identity: coincidence plus same-turn revisit sensitivity). Secondary coarse match:
  `|arrival - baseline arrival|_1 <= 4` and `T/T_base in [1/2, 2]`.

### Controls

* C1 (exactness). (a) `r` larger than the territory: all replicas identical, `ID = 1`. (b) C1b: the baseline episode
  replayed on the cropped board reproduces the live ant (full-board simulation of the same interval: same `T`,
  arrival cell, heading, colour); the sampler aborts on a mismatch.
* C2 (random walk). The `rw:SEED` walker ignores colours: all replicas must be identical at every `r` (`ID = 1`). Fails
  => instrument broken.
* C3 (ICC null). Reassign replica sets to random other departures: `ICC` must be 0 within its sampling sd.
* C4 (rotor walker). `rotor:SEED`: arrows are hidden state and are rewritten each visit; report `G(r)` for contrast with
  the ant, no sign prediction beyond C2/C3.

## Registered predictions (07:30 +13:00)

Cohort: pilot = 12 rules (every 4th of the 48 sorted rules of `2026-10-20-kappa-first-passage/results/confirm_specs.txt`,
seeds 202610208001-2); confirmation = the other 36 rules, same two seeds, frozen after the pilot. Pilot results may
refine instrument details but not these predictions.

* **P1 (no compact local boundary state).** At A0 = 2^17, the median over confirmation rules of `r*/L` is `>= 0.25`, and
  the pooled log-log slope `gamma` of `r*` against `L` (paired bands, per-rule difference, median over rules) lies in
  [0.7, 1.3]. Reading: the colour field relevant to the next return extends over a fixed fraction of the territory
  scale, so no finite boundary summary is sufficient and the return map is not Markov on any state of bounded radius.
* **P1-alt (a compact state exists).** Median `r*/L <= 0.1` at both bands and median `gamma <= 0.4`. This would be the
  headline: a bounded-radius state making `R` approximately Markov, and the next step would be to enumerate it.
  Anything between (0.1 < median `r*/L` < 0.25, or gamma in (0.4, 0.7) or > 1.3) is reported as inconclusive,
  not as a rescue of either.
* **P2 (dynamics does not forget what it reads).** The median over (rule, band, r) of `E_r` is `<= 0.02` where
  `Utilde_r < 0.9` (enough exposure to measure). A median above 0.1 would mean exposed excursions often return exactly
  anyway and the "exposure" accounting is not the right dependence measure; report as such.
* **P3 (controls).** C1b and C2 pass exactly (no tolerance); C3 within 2 sampling sd. If any fails, nothing else is
  reported as a finding.
* **P4 (deep excursions carry the reach).** Splitting departures by baseline max depth into `<= 8` and `> 8`:
  `G(r=8)` is `>= 0.8` for the shallow group in the median rule and `<= 0.5` for the deep group. (This is the weakest
  prediction: it is the mechanism for P1, not independent of it.)

What would change my mind: a rule class where `r*/L` is far below 0.1 (e.g. tethered, crystal rules) would say the
global dependence is rule-specific and the vocabulary should split by `r*`; a pilot where `ICC_0` already exceeds
0.9 would mean the departure geometry rather than colours drives the outcome and `G` is ill-conditioned (then report
the colour-dependent variance in absolute terms).

## Pilot results and amendment (2026-10-21 ~09:00 +13:00; before the confirmation run)

Pilot: `scripts/run_pilot.sh`, commit `67f7d032742843c82b7a4fa2237ede5e2e877b30`, binary sha256 prefix `56bdf30c5c7e97f4`,
log `results/pilot.log`, analysis `results/pilot_analysis.json` (`scripts/analyze.py`). 12 rules x 2 seeds + rotor:301-302 + rw:301.
**RLRLLLL did not reach area 2^17 within the 4e9-step budget (both seeds): ineligible, not an instrument failure.** 25 of 27 specs finished.
Exploratory only; P1-P4 are tested on the confirmation cohort.

* Controls: live check (C1b) 2851/2851 departures; rw:301 identical at every r (C2); r >> territory identical; ICC null (C3)
  mean 0.011, sd 0.019 over 196 values (max |.| 0.112, not within 2 sd for every cell: single cells with few departures are noisy).
* Population-weighted `G(r)` (weights = stratum shares of all departures) reaches 0.8 at `r*` of about 1-7, `r*/L` 0.02-0.11
  (median 0.08 at A0=2^15, 0.05 at 2^17), per-rule gamma median 0.07 over 10 rules (range -0.55..2.75). Taken alone this
  looks like P1-alt (a compact state). Rotor controls look similar (r*/L 0.05-0.12): the number mostly reflects that most
  departures are short excursions.
* Split by baseline maximum depth: shallow (<= 8) median `G(8) = 0.98`; deep (> 8) median `G(8) = 0.16`, `G(16) = 0.54`,
  `G(32) = 0.85`. This split conditions on the outcome (a deep excursion has necessarily left a small ball), so it is
  partly tautological and is not evidence of P4.
* Time-weighted `G` (each departure weighted by its baseline duration, i.e. its share of interior time) gives
  `r*_t/L` of 0.7-1.4, or beyond the largest radius (128, L about 100) in 6 of 11 rule-bands at A0=2^15 and 8 of 11 at 2^17.

Interpretation (pilot-level): the ensemble of excursions is two populations. Most departures return after a short loop whose
outcome is fixed by colours within a few cells; the interior time is carried by rare long excursions whose outcome depends on
structure out to a fixed fraction of the territory. This explains why the registered P1 (population-weighted) is likely to
come out "compact" while saying little about the time budget.

**Amendment, registered before the confirmation run (pilot-informed, so not blind):**

* P1 and P1-alt stay as registered and will be reported as they come out.
* **P5 (time-weighted reach).** Median over confirmation rules of `r*_t/L` is `>= 0.5` at both bands, with "beyond the
  largest radius" counting as `>= 0.5` and flagged. Falsifier: median `<= 0.2` at both bands. Radius 256 is added to the grid so
  that fewer cases are censored (pilot grid ended at 128). The pilot suggests P5 will hold; if it does the headline is that
  *a compact local return map describes typical excursions and cannot describe the time budget*, which is a statement
  about the structure of shuffles, not single-cell control (see the interpretation limit above).
* The shuffle-vs-single-cell limitation stands; the single-cell flip arm remains unregistered.

Confirmation command: `scripts/run_confirm.sh` (cohort `results/confirm_specs.txt`, 36 rules x 2 seeds).

## Deviations and results

(confirmation results appended below once run)
