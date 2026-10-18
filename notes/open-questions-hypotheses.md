# Open questions: hypotheses before measuring

Written 2026-10-10 10:30 NZDT. Data in hand: probe_summary.csv (c, kappa, ...
per rule). No new measurements yet.

## Q1. Why is the ant well mixed inside its territory?

H1. Inside, the ant is a random walker in a random environment. Each interior
cell has been visited many times, so its colour (visits mod n) is close to
uniformly distributed and nearly independent of its neighbours'. The ant then
turns R with probability q = (number of R in the rule) / n on each step,
independently of earlier turns. That is a persistent random walk, and a
reflecting wall plus a symmetric walk gives a uniform stationary density.

Predictions:
- P1.1 Interior colours are close to uniform over 0..n-1, with weak
  neighbour correlation.
- P1.2 The fraction of R turns the ant actually makes is close to #R/n.
- P1.3 Turns are nearly uncorrelated, and the mean squared displacement grows
  linearly with lag, with slope (1 - a) / (1 + a), where a = (2q - 1)^2.
  (Derivation: each turn multiplies the velocity by +i or -i; the average
  factor is i(2q - 1), so velocities k steps apart correlate as
  Re[(i(2q-1))^k], which vanishes for odd k and is (-a)^(k/2) for even k.
  Summing gives MSD / lag = 1 + 2 * sum_m (-a)^m = (1 - a) / (1 + a).)
- P1.4 Mixing time across the blob ~ R^2 / D, which is much less than the
  time to grow, ~ R^3. So the walker has time to spread out.

H1' (for tethered, log-growing rules). Something breaks the random environment
near home, for example a colour field that is not uniform, giving an inward
drift. If the drift v is constant and diffusion is D, occupancy falls off as
exp(-r v / D), which matches the exponential profile already seen.
- P1.5 Tethered rules have a measurable inward radial drift, and D / v matches
  the decay length of their occupancy profile.

## Q2. Why is c about 0.44, and why does it vary?

H2. Edge cells are young: most have been visited once or twice, so their
colour is 1 or 2. On an edge cell the ant's turn is therefore set mostly by
rule[1] (and rule[2]), not by a random colour. If the low colours turn the same
way as colour 0 (R), the ant tends to curl back inside, so c is low. If they
turn the opposite way (L), the ant zig-zags outward, so c is high.

Predictions:
- P2.1 Among all rules, c depends strongly on rule[1] and rule[2], much more
  than on later letters.
- P2.2 Edge cells' colours are concentrated on 1 and 2.
- P2.3 A local model (an ant on a straight edge, cells beyond it blank, edge
  cells with the measured colour mix, interior colours uniform) predicts c per
  rule better than the random-heading estimate (fresh neighbours / 4).

## Added 2026-10-13 from the frontier work (see notes/frontier-results.md)

- Q3. Why do 556 tethered rules share only ~28 discrete home types (spread values equal to 0.3% across different rules and territories)? What invariant of the rule labels them?
- Q4. Why are the rare colours (6, 8, 10 / 0, 6, 8) the critical cells of the hard rules' homes, and are they rare in the other 112 rules of the same class?
- Q5. Is RRLLLLLLR (CV of A/t^(2/3) rising with t, alpha_w 0.38 +- 0.07) truly non-self-averaging, and with how many 'phases'?
- Q6. Why does the ant's visit density equal the rotor's cone 3 t/A (1 - r/R_eq) to a few per cent? A derivation would also fix the 2/3 exponent.
- Q7. Why is the ants' boundary width exponent (0.25-0.27 at 2^28) above the rotor's (0.13-0.16 under the same estimator)?

## Added 2026-10-13 (later) from `notes/frontier-2026-10-13-results.md`

Answered or sharpened there: Q3 (no single rule feature labels the home types: A5), Q4 (rare colours are rare in 99.8% of tethered rules and 5% of blob rules, but the self-healing explanation is reversed: the most-visited cells hurt most: A6), Q5 (RRLLLLLLR: CV rises through 2^29, unimodal; rising CV occurs in ~5% of random rules, mostly RLLL-prefixed: A7), Q6 (cone follows from uniform occupancy + the exponent, and correctly fails on a random background: A3), Q7 (ants 0.24 vs rotor 0.13 still falling, ants trend inconclusive: A4).
New, unexplained:
- Q8. RLRLLL (an item-4 "core" rule) leaves a 32768 board at ~1.3e10 steps with no highway: an ~1e4-cell thin structure along the edge (bounding box 3,377 at 1e10, 18,175 at 1.3e10). What is it, and are the other "escaped without highway" cases (4.6% of the 10^10 survivors, 1.2% in (1e10, 1e11]) the same thing?
- Q9. The tether of RRRRRLLLLLL is intermittent in age: a fresh ant started from the 2^25 field at home heading north is not tethered (near_home 0.061) while 2^22, 2^24, 2^26 are (0.43, 0.47, 0.36). What state does the original ant carry that the field plus "home, north" does not?
- Q10. In the rotor with north-biased arrows (C2b/C2c) the territory width at 2^28 grows +25% at bias 0.15 and 2.6x at 0.25, while kappa and c change by < 10% and the width exponent not at all. Which factor of the identity dA/dt = c kappa P/A carries the amplitude, and why does the area exponent rise (0.715 to 0.787) while the width exponent does not (elongation untested)?
- Q11. Why is the area-exponent ICC across rules (0.34) so much larger than the width-exponent ICC (0.12) (B2)? Rule identity matters for area but little for width.
- Q12. The 3D ants' extent exponent is 0.234 and volume exponent 0.771 at 2^28-2^32, so volume/extent^3 is still rising as ~t^0.07: what is the 3D limit shape (D2)?
- Q13. Why does the corruption of the most-visited cells hurt most, with rare colours doubling the damage at equal visit quartile (A6)? How much of "visit rate" is just distance from home?
- Q14. Does contiguity reduce the damage per critical cell (patch ~0.65x the loss per rare cell of a swap, E4), or is that a location effect?
- Q15. The census "builders" (304) are mostly non-highway frame builders with steps per bounding-box cell ~2-6; are they one class, and does it match the sqrt(t) turmite cluster (D1)?
- Q16. The late-highway onset rate fell from 4.7% to 1.25% per decade at 1e10-1e11 (B5): is the set of late highways finite, and what is the distribution of onset times within a prefix family?

See `notes/research-frontier-2026-10-13-pm.md` for the Phase 3 hypotheses, tests, and pre-registered predictions addressing Q8–Q16 and the open mechanisms of kappa.

## Added 2026-10-15: independent-area cutoff test

The cutoff–kappa lead survives assigning area matching, kappa and cutoff to
three disjoint seed groups: concordance 0.730 [0.605,0.842] at k=22 and
0.757 [0.617,0.881] at k=25, 44 rules, rule-bootstrap intervals (2000 draws,
seed 20261016). Registered thresholds held; synthetic controls passed. Recipe:
`uv run experiments/2026-10-15-independent-area/scripts/analyse.py`;
[registration, caveats and next test](../experiments/2026-10-15-independent-area/README.md),
[output with provenance](../experiments/2026-10-15-independent-area/results/analysis.json).
This rules out shared-trajectory area matching as a sufficient explanation;
it does not remove noisy-area confounding or establish causality. The earlier
noise-floor gate remains failed. Open: does cutoff predict kappa at truly
fixed area in new trajectories, and can an intervention separate cause from
two responses to the same field property?

Prospective follow-up avenues, recorded 2026-10-15:
[fixed-area sampling, cycle accounting and mutation-to-tail tests](cutoff-research-avenues-2026-10-15.md).
These are research proposals; detailed predictions and analysis choices must
be registered in experiment READMEs before new measurements.

## Q8 follow-up, 2026-10-15: registered archival comparison
[G1 README](../experiments/2026-10-13-g1-ribbon-breakout/README.md) and
[append-only notebook](../experiments/2026-10-13-g1-ribbon-breakout/NOTEBOOK.md)
record the test and failed transfer prediction. RLRLLL has efficient, elongated
new-cell window boxes in 281/298 complete breakout windows, versus 0/676 before
onset; median efficiency 4.022, transverse box 54 cells. The original <=12-cell
prediction fails. This does not establish a period or a surface-wave mechanism.
Across completed census chunks, 0/37 earlier and 0/3 later no-highway escapes
have global box aspect >=2, falsifying the >=75% transfer prediction. Global
boxes are insufficient to test local ribbon dynamics (recognised highways also
usually have nearly square global boxes). Later census coverage is 324/732,
not complete; RLRLLL's later chunk is unfinished. Local-window comparison of
RLLLLRRL, RLLLLRLRRL and RLLLRLRLLL remains the discriminating next test.
Reproduce: `uv run experiments/2026-10-13-g1-ribbon-breakout/scripts/compare_survivors.py`;
JSON and per-rule CSV in that experiment's results include input hashes.

## Appended 2026-10-15: open items left by the experiments closed that day (nothing here has been measured)

- **kappa at matched area (`experiments/2026-10-14-kappa-matched-area`).** A residual rule effect on the excursion cutoff at fixed area is not established: 2 of 3 registered noise-floor
  criteria failed. Open and unregistered: rule-level regression of ln_ratio on the kappa residual given ln A with rule random effects, split seeds, all 44 rules, no matching.
- **Prefix effect on kappa (`experiments/2026-10-kappa-rule-space`, W3c).** The size-controlled gain of the prefix-8 lookup replicated on stationary rules (+0.240, CI 0.148-0.347) but is ~0 in the all-rules
  cohort when trained on all census rules (+0.031 / +0.042 on stationary test rules); stationary-trained it is +0.240. Open: does the non-stationary census tail simply degrade the lookup (w3c_cross.txt, exploratory), or is the prefix effect specific to stationary dynamics? Not run: W5 (area-matched bulk fingerprint), an area-matched W6 mutation scan, the 200-base W6 (about 8 CPU-hours, needs a registered addendum).
- **Ants' wide edge (`experiments/2026-10-14-hull-width`).** The extra width carries 2-6x the long-wavelength power of the rotor, but the high-wavenumber deficit and a rotor-stability control failed. Open: real-space S(dtheta), band power versus k
  (needs a rerun that dumps r(theta) at k = 24..30), larger census-sampled rule set.
- **Crystal rules (`experiments/2026-10-crystal-rules`).** Ray motifs are not one rigid colour motif (P2 falsified); most ray length is an alternating-turn staircase (~2 steps/cell) over a background whose alternate cells carry opposite-turn colours (exploratory, sublattice picture not verified; unrelated to the 50-86 steps/cell colour-2 ground highway). Open and not run: ray transplant onto reset/shuffled
  background, ray-end knockouts, spinner isolation, exact periodicity of the ground highway, what ends a ray, why the sweep builds ground 2, why the opposite-turn two-sublattice background forms.

## Added 2026-10-15: crystal ground-phase intervention

The registered local colour-2 reset failed its strong staircase-disruption/termination predictions in all three crystal rules (16 paired states each). Reset to blank advanced geometric termination in 3/16, 6/16, 8/16 states, while mean full-window alternating-turn fractions increased; phase advance to 3 never advanced termination. Colours 2 and 3 specify the same turn, limiting that control to revisit effects. See [protocol, commands and caveats](../experiments/2026-10-crystal-rules/README.md#s2-phase-reset-results--2026-10-15-2123-1300) and [generated summary](../experiments/2026-10-crystal-rules/results/phase_summary.txt). Open: are ray ends caused by specific encountered defects rather than the abundant ground phase? A targeted endpoint intervention with explicit exposure measurement remains unrun.

## Research priorities for the paper, 2026-10-15

[Paper-gap research avenues](paper-gaps-research-avenues-2026-10-15.md) records
prospective fixed-area prefix tests, complete boundary/interior time accounting,
paired mutation challenges, crystal edit-exposure and endpoint tests, edge-profile
mode persistence, and stronger forecast baselines. These are proposals, not
preregistrations or new measurements. The first priority combines fixed-area
sampling with episode accounting to distinguish prediction from mechanism.

### Kappa prefix robustness follow-up (2026-10-15 22:40 +13:00)
The [prefix audit](../experiments/2026-10-kappa-rule-space/README.md#prefix-audit-addendum) retains the W3c stationary lead: five-fold lookup encoding plus a five-knot nonlinear ln A(22) control gives +0.201 held-out R2 gain [0.103,0.311] on 162 rules, seed 20261015 and 2000 rule bootstraps. Removing five largest positive contributors leaves +0.150. All-rules gain +0.041 [-0.018,0.103] remains weak. This is an audit of inspected data, not fresh confirmation or a mechanism; one area snapshot may leave growth-history confounding. Reproduce with uv run experiments/2026-10-kappa-rule-space/scripts/prefix_audit.py; input/source hashes in results/prefix_audit.json. Next discrimination: matched-area mutation intervention with early-colour exposure measurements.

## Kappa mechanism plans — 2026-10-15, prospective

[Mechanism brainstorming](kappa-mechanisms-2026-10-15.md) separates four live
possibilities: excursion survival mass, prefix-dependent fresh-cell return
kernels, sparse directional return gates, and field maturation. Each now has
an experiment scaffold with draft predictions, controls and a handoff. Start
with tail-budget and age observations; gate interventions depend on reliable
predictive evidence. These are plans only, not new results or registrations;
all earlier failed predictions retain their verdicts.

### Tail budget / field age instrument pilots — 2026-10-15 23:38 +13:00

See the [mechanism continuation](kappa-mechanisms-2026-10-15.md#instrumentpilot-continuation--2026-10-15-2338-1300)
and the [tail](../experiments/2026-10-15-kappa-tail-budget/results/REPORT.md) /
[age](../experiments/2026-10-15-kappa-field-age/results/REPORT.md) reports.
Both observation-only instruments now pass reference and synthetic accounting
checks. All primary bands were reached; two drifting rules miss the largest
age band. Reserve and original age cohort support gates fail. Median log age
has weak small-band seed-half precision. No T1/T2/A1/A2/A3 scientific prediction
has been tested; neither mechanism is established or retired. Next: calibrate
and freeze actual estimators and role/fold manifests, add state checkpoints,
and resolve cohort/precision/cap choices without dropping inconvenient rules.

- **kappa uniform reference (2026-10-16, `experiments/2026-10-16-kappa-uniform-baseline`).** The probe's kappa for a uniform-by-construction walker is ~1.11, not 1 (window-mean B/S over end-of-window P/A). Paper line "kappa < 1 below uniform" should be restated relative to 1.11. Still open: why the ants sit near 0.81× this reference (rotor and memoryless ratios need their own a, β); no mechanism established.

- **kappa residual vs cutoff ratio, rule-level mixed model (2026-10-16, `experiments/2026-10-15-kappa-cutoff-mixed-effects`).** Registered random-intercept REML (own implementation, calibrated on synthetic data and a label-shuffle control), all 44 rules, kappa/ln A from even seeds and ln_ratio from odd seeds, seed 20261016. The registered hypothesis FAILED: standardised effect -0.275 (k=22) and -0.194 (k=25) vs the registered <= -0.30; permutation p 0.020 / 0.096; held-out dR2 +0.118 / +0.073 but bootstrap fraction positive 0.875 / 0.866; pooled-k slope ~0. The sign is negative everywhere, but the matched-pair 76-82% overstated it. ln_ratio is heavy-tailed (a minority of rules has seed sd 2-4 in ln_ratio) and the Gaussian model is misspecified. Post hoc (unregistered) the 27-29 rules with stable cutoffs meet every threshold (Std -0.43 / -0.39); that needs a registered replication (k=23, 24) before it counts. No T2 on this basis. Reproduce: `uv run experiments/2026-10-15-kappa-cutoff-mixed-effects/scripts/{calibrate,fit,posthoc}.py`.

### Kappa prefix-exposure confirmation — 2026-10-16 (`experiments/2026-10-15-kappa-prefix-exposure`)
Registered and run (10752 specs, 24 bases x 7 arms x 64 seeds; 10 informative validation bases). The early-letter effect on B/S at fixed area replicates on fresh mutants (P3: ratio of median |d ln B/S| early/late 9.0, CI 1.27-27.6) and is sign-mixed in kappa (P5: 47% positive, mean -0.11 vs median |shift| 0.33), as W6m found. The fresh-cell long-excursion launch rate does NOT carry it out of sample: P1 FAILED (held-out R2 0.105 vs 0.20, CI -0.21..0.32). P2 (exposure features over position) FAILED on its interval bar: delta R2 +0.215, CI [-0.002, 0.281]. Open: is the exposure gain real (needs more bases), and why do early mutants fail to reach the band more often (valid 0.82 vs 0.73-0.93)? Details and caveats in the experiment README.

- **kappa prefix decoded to a leading run (2026-10-16, `experiments/2026-10-16-kappa-prefix-decode`, exploratory, post hoc).** On the ln A(22)-spline residual, stationary rules, the prefix-8 lookup (+0.20 held-out) is beaten by one number, the length of the opening same-letter run (+0.33 [0.24,0.44]); residual rises ~+0.1 ln kappa per leading step in census and pool alike; letters 9+ and rule length/R-fraction carry nothing. Registered D1 (additive fails) and D5 (tree splits late) FAILED; own-prefix-rule hypothesis recovers only 22-69%. Caveats: lead run vs ln A(22) corr -0.56 (growth-history confound possible), A-series chosen on scored data. Next: registered fresh-rule test and fixed-area leading-run mutation dose-response.

### Kappa return gates, Stage 1 — 2026-10-16 (`experiments/2026-10-15-kappa-return-gates`)
Registered and run: incremental rim/interior depth instrument (oracle-tested), 24 rules x (64 predictor + 32 target) seeds, bands A = 32768 and 131072. Adding two gate features (crossing-location concentration g1, exit-heading return spread g2) to a 4-feature depth-profile baseline did NOT improve held-out-seed boundary occupation: DeltaR2 -0.039 [-0.118, +0.006] and -0.0004 [-0.033, +0.031] (criterion +0.10, plus bootstrap and permutation). Verdict UNINFORMATIVE, not a clean rejection: the baseline (which excludes boundary occupancy itself) already has held-out R2 0.88 / 0.95, so band 1 has no headroom, and g2 fails split-seed reliability at band 0 (0.56). Open: which independent, pre-window quantity can explain what the depth-profile ratios (essentially the time budget) reproduce? Gates are neither supported nor retired; G2 (matched gate/non-gate edits) is unrun and should not be launched on this evidence. Edge case reported: 3 predictor runs with 2-6 events gave a NaN null (sensitivity: same conclusion).

### Leading-run kappa effect: fresh rules and single-flip mutants — 2026-10-16/18 (`experiments/2026-10-16-kappa-leadrun-fresh`)
Part A (300 fresh lead-stratified rules, 176 stationary): the opening same-letter run predicts ln kappa out of sample and at fixed ln A(22) (L1 gain +0.651 [0.498, 0.828]; shuffled-label null ~0; slope ~0.15 per lead step, concave in lead). Survivor selection is a caveat (stationary share 0.68 at lead 1 vs 0.38 at lead 6).
Part B (417 single-flip mutants of 157 bases, 257 kept-and-stationary pairs, run 2026-10-18 after the 2026-10-16 launch died at 600/3336): M3 HELD (shortening the lead: d ln kappa -0.198 [-0.290, -0.110], area unchanged). M1 and M2 FAILED as registered (extend arm -0.062 [-0.152, +0.025]; extend minus late-flip control -0.056 [-0.159, +0.040]) because lengthening the lead also cuts ln A22 by ~0.29, which cancels the kappa change. Per the README's criterion, causality is NOT established; an earlier "causal at matched area" write-up was withdrawn (bd8c489). Caveats: ln kappa contains ln A, retention differs by arm (extend 0.53), pooled CIs ignore shared bases, no arm flips an early letter without changing the lead. Open: why lengthening the lead shrinks the pattern but shortening it does not; next test (proposed, not registered) is an early flip that leaves the lead unchanged, area-adjusted. Reproduce: `uv run experiments/2026-10-16-kappa-leadrun-fresh/scripts/analyse_mutants.py`.

### Kappa chiral-edge hypothesis refuted; lead sets c — 2026-10-19 (`experiments/2026-10-19-kappa-chiral-edge`)
Registered hypothesis: the opening same-letter run acts on fresh ground (young edge cells) as a chiral edge layer and raises kappa. Stage 1, within-necklace (cyclic rotations, composition fixed; census, 536 necklaces / 2358 blob rules): slope of ln kappa on lead -0.007 [-0.037, 0.017] (stationary +0.044 [0.027, 0.060]); N1 and N4 FAILED. The cyclic-maximum-run alternative also failed (wrong sign); census-wide ln kappa on lead has R^2 0.001, so the cross-sectional "lead effect" exists only conditional on ln A(22). What does hold: within a necklace the opening run sets c, the births per boundary step, almost deterministically (ln c slope -0.243, R^2 0.83). Stage 3 (registered): 1/c = 1.363 + 0.696 lead fitted on 2185 census rules predicts 176 fresh rules at R^2 0.877, median relative error 4.1% (C1, C2, C3 held; naive 2x2-loop picture a = 0, b = 1 rejected; C4 failed: prefix lookups keep gaining out to 8 letters). Mechanism of a ~ 1.36, b ~ 0.70 unexplained. Open: reconcile with the leading-run M3 (conditional-on-area effect); sbg dose-response of the c law (unregistered, not run). Reproduce: `uv run experiments/2026-10-19-kappa-chiral-edge/scripts/{necklace_lead,cyclic_run,c_by_lead,c_law,c_explore}.py`.
