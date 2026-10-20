# Can depth-crossing dynamics predict kappa?

Created 2026-10-20 21:10 +13:00, base
`44ed81f71f4bb1fdccdcdc1cbd586bc1b2eb5613`.
Status: **prospective scaffold; instrument, pilot and confirmation NOT RUN**.
This records hypotheses before measurement. It is not a frozen confirmation
registration: cohort, calibrated estimator and resource manifest remain to be
frozen after feasibility. No simulation is authorized by the scaffold itself.

## Question

Can a reduced description of excursions between depth surfaces predict boundary
occupation on independent starts and previously unseen rule families? If so,
does it explain why occupation stabilizes as territory grows?

Read [the mechanism ledger](../../notes/kappa-mechanisms-2026-10-15.md),
[earlier tail-budget results](../2026-10-15-kappa-tail-budget/README.md), and
[family hold-out](../2026-10-20-kappa-prefix-family-holdout/README.md).
The tail-budget joint T1 criterion failed; its T2 extrapolation passed but lost
to persistence. Its truncated-power-law estimator failed calibration. This
experiment does not relabel those failures or reuse that fit as validated.
The new test is composition of **depth-crossing kernels**, rather than using
the target trajectory's measured mean excursion time to reconstruct occupation.

## Theory and falsifiable assumptions

Let D be a boundary episode and T the following interior episode, both counted
at departure times. In a stationary renewal-reward approximation,

    f_boundary = E[D] / (E[D] + E[T])
    kappa_inst = L * f_boundary,        L = A/P.

Successive episodes need not be independent for the long-run budget identity.
Stable representative means are necessary. A growing finite window is not
automatically a stationary ensemble.

Candidate tail, above a separately modelled short-return component:

    Pr(T > t | L, rule r) ~ a_r t^(-theta) F(t / (b_r L^z)).
    E[T] ~ a_r b_r^(1-theta) L^(z(1-theta)) I_F,  0 < theta < 1.

If E[D] scales as L^delta and long excursions dominate, then

    d log(kappa_inst) / d log L = 1 + delta - z(1-theta).

Constant kappa therefore requires z(1-theta) = 1 + delta. The conversational
proposal z(1-theta)=1 assumed delta=0; measure that assumption. The candidate
(theta,z,delta)=(1/2,2,0) is one possibility, not an ant result. Slowly varying
amplitudes, a changing shape F, multiple cutoffs, or a short-return-dominated
mean invalidate this simplified exponent prediction.

Physical hypothesis: the written field controls the chance that an ordinary
shallow excursion continues into a long journey. A local description should
predict these chances from heading, phase and nearby field state. Alternatively,
large-scale history may remain indispensable. A uniform-preserving diffusion
clock alone cannot change stationary occupation on a fixed domain.

## Definitions that the instrument must implement

- Territory includes all visited cells, including the origin. P counts visited
  cells with an unvisited four-neighbour, including hole boundaries.
- Observe before each departure, after marking its current cell visited.
  Boundary indicator Z_t uses this evolving territory. Episodes are maximal
  consecutive runs of Z_t=1 (D) or Z_t=0 (T).
- Depth d_t is shortest four-neighbour distance through visited cells to any
  boundary cell; boundary depth is zero. Primary depth evolves each step.
  Record geometry-induced milestone crossings separately: depth can change
  when the territory changes even without a move across an old contour.
- Milestones: depths 0,1,2,4,8,16,32,64. On hitting one, record first subsequent
  hit of a *different* adjacent milestone and elapsed time, retaining revisits
  to the departure milestone. At depth 64, retain the deep excursion until
  return to 32; never reflect artificially at 64. If this unresolved deep
  component dominates the budget, the local closure is uninformative.
- Sensitivity: freeze the depth map at each interior entry, terminate at first
  contact with that frozen boundary, and record disagreement with evolving
  episodes. This is a different observable, never silently substituted.
- Sample area bands [A0, A0+A0/16), A0 = 32768, 131072, 524288.
  Retain entry-conditioned complete lengths AND exact in-band time contributions.
  Follow entries past band end until return or cap; record left fragments.
- Primary outcome is in-band B/S. Secondary kappa uses exactly the probe's
  window-mean B/S divided by endpoint P/A. Also record sum_t P_t/A_t so the
  uniform-window reference is directly computable, without fitting exponents.

## Stages, seed roles and cohort

1. **Calibration:** implement the observation-only accountant against a sparse
   reference. Use planted semi-Markov chains with known hitting times, an
   unbiased absorbing interval walk, and a hidden-state chain that defeats a
   memoryless closure. Include heavy tails and cap censoring. Rotor and random
   walk validate bookkeeping; neither is assumed to obey the candidate ant law.
2. **Feasibility pilot:** at most 8 rules, 4 starts each, selected by a seeded
   shuffle of distinct six-letter families in the committed census rule list.
   Freeze the exact input hash and eligible rows before reading new outcomes.
   Choose without ranking kappa, tail length or success at the target bands.
   Pilot seeds 202610207001..202610207004; calibration seed 202610207000.
   Board 8192, cap 2^32, at most 2 CPU workers, subject to profiling.
   These are proposed resources, not measured runtime estimates.
3. **Confirmation:** target 48 non-pilot rules from at least 12 non-pilot
   six-letter families, balanced across families, retaining pre-labelled
   stationary and drifting strata. Selection algorithm, exact list and strata
   must be committed before simulation. If unavailable, revise publicly before
   running; do not fill missing cells by selecting favourable outcomes.
   Per rule: 8 kernel seeds 202610208001..8008 and 8 outcome seeds
   202610209001..9008 (full integers: 202610208001 through 202610208008,
   and 202610209001 through 202610209008). Check all against repository seed
   manifests before freezing. Folds group complete six-letter families.
   Split seed roles before jobs launch; retain all attainment/escape outcomes.
4. **Intervention:** not part of this launch. Only design targeted field edits
   after an independently validated kernel identifies a concrete state variable.
   Commit a separate protocol with sham edits and response horizons first.

## Models and leakage controls

M0: depth and arrival direction (toward/away from boundary).
M1: M0 plus heading and current colour phase.
M2: M1 plus a radius-2 local colour/visited patch and bounded recent history
(last four milestone labels). Exact encoding, pooling of different rule lengths,
regularization and treatment of unseen states require calibration and freezing.
Do not assume these finite states are Markov.

Fit transition probabilities Q and mean segment times m from kernel-role
starts at the two smaller bands. With boundary absorbing, solve
u = m + Q u; nonabsorbing communicating classes or divergent means are explicit
failures, not rows to drop. Estimate the entry distribution and E[D] from
kernel-role data only. Integrate the predicted return budget into predicted B/S.
Report how much E[T] comes from the fitted depth-64 return component: measuring
nearly the entire return time there would defeat the reduction.

Two different transfer claims:
(a) new-start/new-size prediction may use that rule's small-band kernel data;
(b) unseen-family prediction cannot use ANY kernel, entry, dwell or outcome
data from the held-out family. All these quantities must be predicted by the
training families' frozen state model, given rule and initial-condition inputs.
If (b) proves infeasible, report it untested; do not call (a) family transfer.

Compare with training-mean, geometry-only, small-band occupation persistence,
and small-band empirical E[T] extrapolation baselines. Geometry supplied from
target runs makes kappa a geometry-conditional prediction; label it as such.
Primary B/S does not divide by outcome area. Fixed-area sampling is a stopping
design, not proof of causal control for size; do not interpret associations
conditional on area as direct rule effects.

## Prospective predictions and decision rules

These thresholds are design choices, not power calculations. Freeze a synthetic
power/bias study and any revisions before confirmation.

- **P1, scaling:** independently estimated delta, theta and z predict the sign
  and magnitude of held-out large-band kappa drift: median absolute slope error
  <=0.15 per log L, and lower error than zero-drift persistence. Fit the tail
  law only if calibration and identifiable cutoff coverage pass. Failure of
  tail fitting means P1 uninformative, not evidence against every return theory.
- **P2, predictive closure:** M2 predicts largest-band rule-mean B/S on outcome
  seeds with median relative error <=20% AND >=10% smaller absolute error than
  the strongest frozen baseline. The 95% family-bootstrap interval for the
  paired error improvement must exclude zero (2000 resamples, seed 202610207099).
- **P3, physical transfer:** unseen-family M2 has pooled out-of-fold R2 >0 and
  improves R2 over M0 by >=0.10. Bootstrap whole families, never individual
  excursions. Both criteria required; report uneven family sizes.
- **P4, state sufficiency:** on validation data, adding radius-4 context to M2
  changes predicted E[T] by <10% for >=80% of rules, and observed conditional
  crossing probabilities are calibrated within the synthetic-derived error
  tolerance. This is evidence only for the tested scales, not a proof of locality.

P2 alone would establish conditional predictive value, not causal mechanism.
P2 passing and P3 failing suggests rule-specific effective kernels. P4 failing
suggests missing memory or collective structure. None of the predictions
establishes the infinite-time exponent from three finite area bands.

## Readiness and stopping gates

See [HANDOFF.md](HANDOFF.md) for the implementation sequence and
[SCHEMA.md](SCHEMA.md) for the required outputs.
Before confirmation: oracle agreement, calibrated estimator bias and coverage,
mid-trajectory checkpoints, byte-identical thread/resume tests, frozen hashes,
and a measured runtime/memory budget must pass.

Require >=80% of rules to have >=6 starts per role in all bands; report the full
denominator. Deepest relevant milestones need >=200 completed segments per rule
across kernel seeds, else pool only by the frozen rule or declare insufficient
resolution. Require <=5% unresolved in-band time AND return-mean uncertainty
small enough to leave the verdict unchanged. A small censored count is not
enough: without a justified tail bound, an unresolved duration has no finite
upper bound. Increase follow-up under a predeclared cap schedule or mark
uninformative. Cap/board failures stay visible.

Scale separation must be checked: compare entry/exit geometry and frozen/evolving
depth estimates; >10% disagreement in mean return estimates blocks a
quasi-static interpretation. Report remaining predictions only as explicitly
finite-band predictions. No ad hoc reflecting walls.

## Scaffold use and present results

From WSL, repository root:

```bash
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
uv run --frozen experiments/2026-10-20-kappa-first-passage/scripts/preflight.py
```

This checks the scaffold and prints its provenance; it launches no simulation.
The notebook is a planning/readiness document until committed results exist.
No new scientific measurements, calibrated estimator, or instrument exist yet.

## Calibration registration (appended 2026-10-20 22:56 +13:00, before the calibrator is written or run; code at 0e0f738)

Instrument v1 differences from the scaffold text above (deviations, decided before any ant outcome was read):
- M2 uses colour *classes* (class(c) = 2 turn(c) + turn(c+1), comparable across rule lengths),
  the four axis neighbours relative to the heading (3 bits each, 4 = unvisited) and the last four
  milestone labels; not the radius-2 colour patch. The radius-4 context needed by P4 is NOT recorded
  in v1: P4 is untested unless a v2 recorder is added before confirmation.
- Interior episodes cannot change the territory, so frozen-depth and evolving termination coincide
  exactly; the ">10% disagreement" gate is vacuous and is reported as an identity, not a measurement.
- Segment pair table is capped (3,000,000 rows per run); overflow folds to the M1 key and is counted.

Estimator (`scripts/fpkernel.py`): chain on segment states at level L in {M0,M1,M2} with a count
floor (a next state seen as a source fewer than `floor` times backs off one level, down to M0);
absorbing = hit of milestone 0; `u = m + Q u`; E[T] = entry-law-weighted u; E[D] = mean elapsed of
segments starting at milestone 0; predicted B/S = E[D]/(E[D]+E[T]). Open (censored) segments are
excluded from Q and m and their mass reported. Unreachable-absorption classes are failures, never dropped.

Predictions for the synthetic calibration (`scripts/calibrate.py`, seeds 202610207000 + k), registered now:
- C1 planted Markov birth-death chain, exponential dwell: M0 relative bias of E[T] below 5%; 95% bootstrap-over-replicate coverage between 90% and 99%.
- C2 planted hidden-mode chain (persistent mode changes up-probability and dwell scale): M0 relative bias of E[T] above 20% in magnitude (the memoryless closure visibly fails); with the mode visible in the key (M1) below 5%. If M0 does NOT fail, the negative control is too weak and the design is revised publicly.
- C3 heavy-tailed dwell (Pareto alpha 0.7 at depth levels >= 4) with cap censoring: report the signed bias of E[T] against the cap-truncated truth; accept the estimator for confirmation only where censored time share is below 5%.
- C4 sparse M2 keys (many rare keys, floor 20): backoff keeps relative bias below 10% and never errors.
Rotor and random-walk runs of the sampler are bookkeeping checks only (identities), not tests of the ant law.

## Calibration results and C2 revision (appended 2026-10-20 23:11 +13:00, code e931af2; data: results/calibration.json, 557 s)

Registered predictions vs outcome (nothing edited above):
- **C1 FAILED on coverage.** Relative bias of E[T] -1.7% (pass, < 5%), but bootstrap-over-8-replicate-chunks 95% intervals covered the truth in 83.8% of 80 datasets (criterion 90-99%). Intervals resampling only ~8 seeds are too narrow; we will not describe them as 95% intervals. Decisions use whole-family bootstrap (12 families), a different and untested resampling unit.
- **C2 FAILED: the negative control did not fail.** The memoryless closure (M0) had relative bias -1.5% on the hidden-mode world (registered expectation: |bias| > 20%). Reason, found afterwards: for a maximum-likelihood chain scored on its own training data, predicted E[T] equals the sample mean interior-episode length whatever the state partition, so no state model can be biased in-sample. The registered control was mis-specified: closure failure is only detectable out-of-sample, when the entry law or conditions differ from training. (Pilot ants showed the same identity: M0, M1 and M2 E[T] agree to < 0.1%.)
- **C3:** with Pareto alpha 1.3 and censored time share 7.4% the E[T] bias is +72% (sd 149%); at shares 2.0%, 0.7%, 0.09% it is -4.9%, -5.1%, -2.6% (sd 20%, 8%, 5%). Accepted only where censored share < 5% (as registered).
- **C4 passed:** floor 20 on 2048-value noise keys gives bias -1.8% (floor 1: -0.4%; sd 7-8%).

**Replacement control C2b (registered now, before it is run; seeds 202610207000 + 500 + k).** Hidden-mode world as C2, trained on entry mixture (0.5, 0.5). The target world differs only in entry law: mode 1 (excursion-prone) with probability 0.9. The kernel (Q, m) is fitted on the training data; the *target's* entry law is supplied as input, since inputs of this kind are allowed for transfer claim (a). Truth is exact. Predictions: M0 (mode invisible) |relative bias| > 20% in the target world; M1 with the mode visible |relative bias| < 10%. If M0 again does not fail, the framework cannot be shown able to detect a closure failure and P2 is reported as uninformative about locality.

Consequence for the confirmation (stated before running it): because in-sample M0 = M1 = M2 = empirical E[T], the registered P2 comparison against the "small-band empirical E[T] extrapolation" baseline can only differ out-of-sample, and the README forbids supplying the target's entry law. P2's "10% better than the strongest baseline" criterion is therefore expected to fail by construction of the design; this is reported as a finding about the design, not as evidence about the physics.
