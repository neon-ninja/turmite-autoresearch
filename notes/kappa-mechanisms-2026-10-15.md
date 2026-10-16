# What could explain kappa? Mechanisms and discriminating experiments

Written 2026-10-15 22:54 +13:00; base commit
`b3a0f762ac2230b5df92188206a4d3a1081897be`, dirty workspace.
Scope: brainstorming and prospective research plans, requested by the user.
No new measurement, literature review or claim of novelty. The numerical
thresholds in the linked plans are proposed design choices, not findings.
Plans are **drafts, not frozen preregistrations**.

## What needs explaining

With S steps, B boundary steps, visited area A and perimeter-cell count P,
the probe reports kappa = (B/S)/(P_end/A_end). The boundary is the visited
cells having an unvisited four-neighbour, including boundaries of holes.
It is not the convex hull. Explain both the rule-specific level and why it
stops drifting for some rules. These may be different problems.

The identity dA/dt = c kappa P/A does not predict kappa independently.
Nor does reconstructing B/S from the same trajectory's return times.
A useful explanation needs an independently measured mechanism, a quantitative
prediction on held-out rules or starts, and an intervention that can break it.
Keep three levels distinct: accounting, prediction, causal mechanism.

## Evidence we must not erase

- [Original mechanism tests](kappa-experiment-design.md): simple rule features,
  edge residence and matched-pair crust depletion did not explain kappa.
  Noisy-rotor kappa rose during crossover, contrary to the proposed monotone
  interpolation. Memory matters, but that does not identify its mechanism.
- [Excursion tomography corrections](excursion-tomography-2026-10-13.md):
  normalized short chords were a size artefact; rare-tail share and the
  early-to-late forecast failed their registered tests. A median excursion
  of one step gives little discrimination. Redoing these correlations is
  not a new mechanism.
- [Fixed-area cycles](../experiments/2026-10-15-fixed-area-cycles/README.md):
  primary association failed its both-band criterion, secondary prediction
  passed, and the narrower-band sensitivity passed. The predictive feature
  was an extreme quantile, not the time mass in the tail. This is the main
  reason to pursue a time-budget model, not to call it established.
- [Prefix audit](../experiments/2026-10-kappa-rule-space/README.md): prefix
  prediction survives flexible area adjustment within stationary rules;
  all-rules transfer is weak. The audit is on inspected data, and a single
  area measurement does not control field history. W5's short-lag bulk
  measures supplied too little predictive gain; long-lag reliability was
  inadequate. Do not call long-range transport ruled out.
- [Matched-area noise floor](../experiments/2026-10-14-kappa-matched-area/README.md):
  its gate failed. These plans do not reopen a transplant on that selected
  pair set by silently changing the gate.

## 1. Tail mass, not just tail extent — first priority

Think of edge episodes of duration D alternating with interior episodes T.
For a stationary renewal approximation, boundary occupation is
E[D]/(E[D]+E[T]). No independence between successive cycles is required for
the long-run ratio, but existence of stable means and representative sampling
is essential. Clipped finite windows require explicit fragment accounting.

The useful unknown is what sets E[T]. For integer T,
E[T] = sum over u >= 0 of Pr(T > u). A cutoff alone omits the survival
amplitude and shape. A rare-trap mixture could have
E[T] = (1-p) mu_short + p mu_trap. Different rules could change p, trap
duration, or both, even when all have a similar top-percent time share.
This explains why a failed top-1% contrast need not kill a time-budget theory.

Test a compact survival model on separate seeds and predict occupation on
fresh ones. Then ask whether its parameters transfer across area bands.
Do not use a measured held-out E[T] to 'predict' its own kappa.
[Plan: tail budget](../experiments/2026-10-15-kappa-tail-budget/README.md).

## 2. The prefix is a boundary preparation programme

A fresh cell encounters colours in their update order. Early rule letters
may therefore prepare local tracks before high colours are reached. The
relevant feature could be a first-return kernel over position, heading and
colour, rather than the frequency of L/R or a prefix lookup itself.
The prefix could set the probability of entering a long interior excursion
without changing the ordinary one-step return or edge dwell.

This is stronger than 'early letters matter': predict mutation effects from
how often the altered colour is actually read and what return transition
it changes. A late letter with comparable exposure is a useful control.
Compare birth-neighbourhood exposure with bulk exposure. Cyclic rotation
of a rule alone changes the physical initial background; joint rotation of
rule and colour labels is an exact relabelling control, not an intervention.
[Plan: prefix exposure](../experiments/2026-10-15-kappa-prefix-exposure/README.md).

## 3. Sparse gates couple the active rim to interior traps

A depleted layer's average occupancy need not measure its permeability.
Two fields with the same depth profile could differ in the number and
direction of a few recurrent crossings. This suggests a rim/interior
transition model: rim-to-interior rate a, interior-to-rim rate b, with
rim occupation b/(a+b). A rim of depth more than zero is not the probe's
boundary; the model also needs independently measured within-rim boundary
occupation. Rates fitted to the same occupations merely restate the answer.

Look for spatial concentration and directional asymmetry of crossings,
then test whether disrupting observed gates changes return time more than
matched edits elsewhere. Geometry, colour phase and incoming heading could
all carry the gates; separating them is part of the experiment. This differs
from the failed crust-depth model and the failed angular-return statistic.
[Plan: return gates](../experiments/2026-10-15-kappa-return-gates/README.md).

## 4. A slowly maturing field controls return hazards

Equal area need not imply equal age: cells can have different discovery
times, visit counts, colour phases and fractions of a completed colour cycle.
An apparent rule-specific kappa may partly reflect where each rule sits on
a slow maturation trajectory. Prefixes might predict that trajectory rather
than a permanent transport coefficient. A stationary occupation ratio could
arise when the age/phase distribution in the active layer becomes stationary.

Measure hazards conditional on cell age and visit count, using independent
seed groups and several fixed-area bands. Compare size/shape, age, and prefix
models on exactly the same held-out rules. Nonstationary rules are a declared
second stratum, not exclusions selected after inspecting target kappa.
[Plan: field age](../experiments/2026-10-15-kappa-field-age/README.md).

## Other ideas worth retaining, but not launching yet

- **Finite-state scattering at a flat edge.** Enumerate a small strip's
  exit/return map under the rule; predict a launch probability for long
  excursions. Attractive route to a rule-only calculation, but a uniform
  distribution over edge states is unjustified. Prefix exposure should first
  reveal which states a real field supplies. Periodic strip cycles and
  artificial walls can dominate; report them instead of declaring a bulk law.
- **Effective accessible volume.** Uniform occupation on only a fraction of
  the visited field changes kappa even without edge attraction. Estimate
  accessibility on an earlier block and predict a later block. Same-window
  occupation-weighted volume risks an identity. Gates/age plans can test
  whether inactive cells are structurally inaccessible or simply slow.
- **Loop cancellation and directional balance.** A local imbalance accumulated
  across colour cycles could control persistence of traps. Derive a bound
  or invariant on signed exits, calibrated on the rotor, before another
  generic feature search. The ant reads heading as well as colour, so a
  rotor-style Eulerian identity cannot be assumed.
- **Outer edge versus internal holes.** Kappa could mix two boundary populations
  with different return hazards. Label both in the gate instrument and report
  their time budgets. This is a definition audit; do not replace P by hull
  perimeter and call the resulting quantity the old kappa.
- **Dynamic scaling of the entire return law.** If
  Pr(T>u|A) approximately equals a(A) F(u/tau(A)), then E[T] scales as
  a(A) tau(A) times the integral of F (with a separate short-time part).
  For stationary kappa, E[T]/E[D] should approach A/(kappa P)-1 under
  quasi-static geometry. A proposed collapse must predict this scaling,
  including amplitude; visually collapsing a quantile is insufficient.

## Order and stopping rules

Start with tail-budget accounting and field-age instrumentation; both extend
the existing fixed-area sampler without changing ant dynamics. Use a pilot
only for instrument reliability and cost, then freeze each confirmatory plan.
Next test prefix exposure with paired mutations. Gate edits are last, after
a gate statistic generalizes across seeds and above its noise floor.

The four directories are independent scopes, not permission to launch four
large sweeps. Each has a draft plan, schema, notebook and handoff. No runtime
estimate is claimed before profiling. A null prediction with adequate
reliability retires that mechanism at the tested scales; a failed reliability
gate means uninformative, not a scientific null. Preserve failures and regime
changes in the final accounting. A successful mechanism should eventually
predict kappa from rule and independently sampled field statistics, rather
than requiring the full target trajectory.

## Instrument/pilot continuation — 2026-10-15 23:38 +13:00

The first two plans now have shared observation-only instrumentation, independent
reference/property tests, synthetic calibration, and prospectively registered
feasibility pilots. These are **not scientific confirmation** of T1/T2 or A1/A2/A3.
Base b3a0f76, dirty source hashes recorded; later shared HEAD 3d00123 is unrelated.

Each pilot sampled 44 supported rules x four fresh starts at areas 32768, 131072
and 524288 (width A/16), cap 2^30, board 4096. Tail seeds 2026101501..1504; age
2026101511..1514. All starts reached both primary bands. Tail reached the third
band 176/176; age 168/176, with all eight misses in two preclassified drifting
rules. No escapes or unresolved in-band I time. Both pilots fail their quarter-
cap reserve gate. Age's original 48-rule cohort is unsupported: existing pool
has only 23 drifting rules, with 22 exact length/prefix-matched pairs.

Descriptive seed-half repeatability is high for tail probability >=64
(0.957/0.962/0.985) but lower for mean entry duration (0.656/0.635/0.847).
Median log age is weak at the smaller bands (0.413/0.321; 0.718 at the largest
among 42 complete-case rules); immature fraction and cycle-count variance are
more repeatable. These are instrument-precision findings, not associations with
kappa. Do not discard the median-age feature or missing drifting rules to obtain
a favorable scientific verdict.

Reproducible summaries/commands: [tail report](../experiments/2026-10-15-kappa-tail-budget/results/REPORT.md)
and [age report](../experiments/2026-10-15-kappa-field-age/results/REPORT.md), with
source/input/seed manifests and executed notebooks in each folder. Release tests
and sampled one-versus-six-thread replay pass; finished-pilot resume preserves
bytes. Mid-trajectory state checkpoints and fitted scientific estimator calibration
are still absent. Resolve those requirements and the cohort/reserve failures,
then freeze scientific roles/folds/models before new confirmation outcomes.

## Uniform-occupation baseline — 2026-10-16 00:45 +13:00 (accounting, not mechanism)

[experiments/2026-10-16-kappa-uniform-baseline](../experiments/2026-10-16-kappa-uniform-baseline/README.md).
The probe divides a window-mean B/S by the end-of-window P/A, so a walker uniform
on its territory at every instant gets kappa = 2∫_{1/2}^{1} x^(-a(1-β)) dx = 1.110
(a=2/3, β=1/2). A mean-field walker (resampled uniformly over visited cells each
step) measured 1.118 ± 0.009 (8 seeds, a=0.677, β=0.520; formula 1.107), as registered.
Consequence: "kappa < 1 means below uniform" should read "below ~1.11"; the ant's
~0.9 is ~0.81× uniform. Two kinds of claim stay separate: the *reference value*
(prediction-level, confirmed) and the *ant's level* (still unexplained). No
mechanism was tested; the four plans above remain the route and remain blocked
on their reserve-gate and checkpoint requirements.

## Return-gates Stage 1 outcome — 2026-10-16

Section 3 was tested in [experiments/2026-10-15-kappa-return-gates](../experiments/2026-10-15-kappa-return-gates/README.md)
(registration and results at the bottom of its README). Two gate summaries added no held-out predictive value beyond a
depth-profile baseline (DeltaR2 <= 0 at both bands), but the baseline alone reached R2 0.88-0.95, so the verdict is
UNINFORMATIVE (headroom and reliability gates failed), not a rejection of gates as a mechanism. The intervention stage was
not attempted. Lesson for the other plans: a baseline built from depth-profile ratios of the same trajectories is nearly an
accounting identity with occupation; use baselines from quantities that precede the window.

- Square-loop mechanism for the lead effect (H-loop) tested and rejected at this scale: see `open-questions-hypotheses.md` ("Kappa square-loop test", 2026-10-16) and `experiments/2026-10-16-kappa-square-loop/README.md`.
