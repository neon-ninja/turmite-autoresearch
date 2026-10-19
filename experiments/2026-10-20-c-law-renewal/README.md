# Can a local renewal model explain the cost of a new cell?

**Current status (2026-10-20):** ordered-event recorder passes oracle tests;
walker controls and narrow stationary-solver calibration pass. The excluded
pilot failed its common-horizon gate. Main outcomes remain unopened; see
the dated addenda below. No local closure or c-law coefficients are derived.

Scaffold created 2026-10-20 (+13:00), base revision
`adf66774570469da30b10ffc1ceab911670a9d09`.
**Scaffold status at creation: prospective design, not a frozen preregistration. No simulations or
scientific fits have been run for this experiment.** Preparing rules is design
generation, not evidence. Freeze the instrument, model and pilot decisions in a
dated addendum before any main-cohort measurement. Preserve this draft.

## Question and motivation

Why does the number of boundary reads per birth approximately follow
`1/c = a + b m`, where `m` is the opening same-letter run? Can a small local
state predict both this cost and corrections from letters 5–8?

The [parent experiment](../2026-10-19-c-law-mechanism/README.md) establishes a
controlled flat-band setting and rejects the simple flat colour-profile and
transition-kernel explanations. Its Stage 2/3 results motivate locality, but
do not derive the intercept or slope. Its Stage 4 reconciliation rules out
treating the conditional lead–kappa association as evidence of a mechanism.
See [the open questions](../../notes/open-questions-hypotheses.md) and
[calibration](../../notes/calibration.md). This experiment targets **c**, not
kappa or a derivation of the growth exponent.

## Hypothesis and competing explanations

**H-local (untested):** the sequence of boundary events can be approximated
by a finite-state Markov reward process. Short loops, birth chains and
excursions away from a local patch determine the boundary-read cost per birth.
Increasing the lead changes their transition probabilities; later letters
change survival after a short loop. Transition rules learned on training
families should transfer to other rule families and interior seeds.

**H-history:** the same local state has different futures depending on
long-lived field structure or previous returns. A model can reproduce pooled
c while missing the return budget, paired-rule effects or history dependence.
Births are candidate renewal points, not assumed independent regenerations.

## Definitions and accounting

- A boundary read starts on a visited cell with at least one unvisited
  four-neighbour, including inner-hole rims, exactly as `probe`/`edge_trace`.
- A birth is a step onto a never-visited cell. The first read there occurs
  on the following step. Attribute births to the source read.
- Measure `c = births / boundary_reads` over `(T/2, T]`. Report zero denominators.
- Index all boundary reads in time order, including reads that do not birth.
  For each event record source state, whether it births, and the next event
  state. Interior steps between events have zero boundary reward but their
  elapsed duration and return category are retained.
- For an ergodic closed embedded chain with stationary weights `pi` and
  expected birth reward `p(s)`, predict `c_hat = sum_s pi(s) p(s)`. This is a
  *prediction* only if transitions/rewards are learned without target-run
  counts. Computing pi from the target's empirical event frequencies is an
  accounting check and must never be scored as predictive success.
- Independently partition each boundary read into previous source-cell read
  lag `first`, `4`, `other <=16`, or `>16`. Lag is since any previous read,
  not merely the previous boundary read. Report category counts per birth;
  they sum exactly to `1/c`. Chains are maximal consecutive births.
- Keep initial/final partial chains and transitions explicit. Unseen next
  events at the end are right-censored, not long-return observations.
  Cell-lifetime `N_b` is secondary: window entrants and censored cells can
  invalidate an uncorrected Little's-law equality.

## Proposed state hierarchy (must be frozen before main measurement)

Work in the incoming-heading frame, reflecting L-first rules to R-first.
Candidate S0: source colour (0..13), its next eight rule turns relative to
the initial turn (cyclic), and the four-neighbour visited mask. Candidate S1:
S0 plus the visited mask and current turn signs at visited sites in the 3x3
patch. Unvisited cells are explicitly marked, not treated as visited colour 0.
Candidate history challenge S2: S1 plus the previous-read lag category and
previous birth-chain length (0,1,2,3,>=4).

S0/S1 are competing local models; S2 tests hidden history, not an excuse to
silently enlarge a failed S1. States, orientation, transition pooling,
smoothing/backoff, treatment of unseen states, stationary-solver choice and
non-ergodic classes remain **implementation gates**. A raw rule identifier
must not enter a state. No fit may use target-rule c, target event frequencies
or target area. Publish unseen-state mass and solver failures.

Sparse states may make this hierarchy infeasible. Pilot support decides
whether a prospective revised state is necessary; preserve the rejected
candidate and register the revision before main measurements.

## Cohort and paired challenge

`scripts/prepare.py` deterministically prepares the design in `results/design/`.
Seed 2026102001; 14-letter primitive R-first bases; m=1,2,3,4, each with
8 training and 4 validation families (48 bases). Four additional pilot
families, one per lead, are excluded from scoring. Every family contains:

1. Base rule.
2. An opposite-letter swap within zero-based indices 4..7 (letters 5–8).
3. An opposite-letter swap within indices 10..13 (letters 11–14), a late-letter
   comparator, **not assumed inert**.

Swaps preserve length, R-fraction and lead. Accept only families with three
primitive rules and no mirror/rotation-equivalent rules across or within
families. Exclude necklaces in the parent's sample, band and validation lists;
this does not guarantee absence from every historical dataset. Splits are by
family: no sibling crosses training, validation or pilot boundaries. No
stationarity or blob screening is used to select the design.

Band layout proposed: torus 256x4096, rows 0..2047 initially fresh, remainder
visited with uniform i.i.d. colours using the parent's `mix64` coordinate hash.
Start at (128,2048), heading north. Main interior seeds 2026102011..14, paired
across siblings; pilot seeds 2026102021..22. Later implementation must document
the exact hash, coordinate convention and binary digest.

Pilot horizon ladder: T=2^22 through 2^26. Choose one common T, the smallest
for which >=3 of 4 pilot families have all arms/seeds consume 10–80% of the
initial fresh band and yield >=5000 window births. If none qualifies, stop
and register a geometry/budget revision. Do not pick a different horizon for
each main rule. Main failures are retained in status tables, never replaced.

## Draft predictions and decision rules

Thresholds below are prospective design choices, not results or pilot-tuned
claims. Freeze or explicitly revise them after instrument calibration and
pilot support checks, before opening main outcomes.

| ID | Proposed test | Proposed criterion |
|---|---|---|
| G0 | Instrument equality vs `edge_trace` and independent sparse path reference | Exact births, boundary reads, chain counts and full lag categories on small seeded cases, including censoring and band prefill; same bytes with 1 and 6 threads |
| G1 | Synthetic finite-state reward chains with known stationary answers | Absolute c error <=0.01 at 10^6 events, 16 fixed seeds; report signed bias and between-seed sd; controlled nonlocal process exposes inadequate local state |
| G2 | Walker controls | Rotor and random-walk accounting agrees with unchanged tracer; larger rotor blank run c in [0.47,0.52], random walk in [0.40,0.48], 8 seeds each, configurations from parent calibration; these calibrate instruments, not ant models |
| G3 | Cohort/support | >=12 of 16 validation families have all arms and >=3/4 seeds usable; unseen state mass <=5% on each scored run and solver well-defined; otherwise primary verdict UNINFORMATIVE |
| P1 | S1 predicts validation c with training-only parameters | R² >=0.90 and median relative error <=5%, using per-rule means; beat training-fitted lead law and K3, with family-bootstrap interval for each MSE improvement above zero |
| P2 | Predict the accounting decomposition | Per-category median absolute error <=0.10 boundary reads/birth for first, lag-4, other-short and long reads; test all four, not just their sum |
| P3 | Predict paired early-swap effects | On validation family deltas of ln c, R² >=0.50 and sign accuracy >=75% for informative pairs; informative = paired |mean delta| >2 paired-seed SE, >=8 informative families required |
| P4 | Test locality against history | Adding S2 improves held-out MSE by <10%; >=20% improvement with bootstrap lower bound >0 rejects the proposed S1 closure; intermediate outcomes unresolved |

Compute validation R² with validation-mean denominator; constant/near-zero
delta variance makes P3 uninformative, not a pass. Fit lead baseline `1/c=a+bm`
and the parent's K3 on training data only. Fit all smoothing/hyperparameters
by family-wise inner CV inside training. Validation is opened once after
freezing the pipeline. No target-run transition estimation.

Uncertainty: 2000 family bootstrap draws, seed 2026102002, all siblings kept
together. Report between-interior-seed sd separately; reads are not independent
replicates. Publish unfiltered status and retention by arm and lead. Report
each prediction's verdict independently. Mechanistic support requires P1–P3
and no P4 rejection; it would support this closure on this geometry, not prove
an exact universal law. A good c fit with failed decomposition is a failure.

## Implementation handoff and gates

1. Commit this scaffold and design. Append the start entry to `NOTEBOOK.md`.
2. Implement an experiment-prefixed Rust example and a clear independent
   reference. Existing `src/edge_trace.rs` is an aggregate oracle, **not an
   ordered-event instrument**. Its driver truncates lag output: do not infer
   a complete long-return budget from that JSON. Add path/oracle property
   tests, deterministic reduction and explicit censoring.
3. Implement synthetic estimator calibration, then excluded pilot. Freeze
   model details, horizon, resource budget and runnable commands in README.
   No scientific inference before G0–G2. Main G3 is checked before scoring.
4. Run resumable per-(family,arm,seed) chunks. Keep bulk ordered events under
   this experiment's ignored `dumps/`; commit small transition/reward summaries
   sufficient to refit without dumps. `.done` only after output validation;
   verify input and binary hashes before reuse. Interrupted chunks restart.
5. Analyse once, execute the shareable notebook using only committed results,
   append verdicts here and in the lab notebook; update open questions and
   blog homepage when there is evidence or a changed research priority.

Use CPU: the new instrument is not GPU-ported. No concurrency with heavy jobs
or GPU timing. Estimate runtime in the pilot before launching main work; long
runs go in the background with a log under this experiment's `results/`.
For published runs build into a fresh target directory, check build exit
status and record binary SHA-256 to avoid the `/mnt/f` stale-build trap.

## Available commands (WSL, repository root)

Every invocation must set the WSL environment:

```bash
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
uv run --frozen experiments/2026-10-20-c-law-renewal/scripts/prepare.py
uv run --frozen experiments/2026-10-20-c-law-renewal/scripts/prepare.py --check
cargo test --release
```

Only the design preparer exists. There is deliberately no simulation or
scientific analysis command yet; steps 2–3 above are required first.
`prepare.py` refuses to overwrite different design files. Its provenance
records exact command, base revision, source/config/script hashes and seeds.
No dependencies are added. Notebook execution once Jupyter is available:

```bash
uv run --frozen --with jupyter --with nbconvert --with ipykernel jupyter nbconvert --to notebook --execute --inplace experiments/2026-10-20-c-law-renewal/notebook.ipynb
```

## Results

None. The generated manifest contains planned rules and assignments only.

### Scaffold verification — 2026-10-20 00:36 (+13:00)

Design generated and byte-checked: 4 pilot + 32 training + 16 validation
families, three rules each. The shareable notebook executed in a fresh kernel
against committed design inputs; `cargo test --release` passed (existing
warnings). These checks do not evaluate G0–G3 or P1–P4. No Rust instrumentation
or scientific estimator was implemented. See `NOTEBOOK.md` for commands,
the mount-mode correction and concurrent commit provenance.

## Ordered-event instrument registration — 2026-10-20 00:40 (+13:00)

Before new ant measurements: `src/edge_renewal.rs` implements source states
exactly as the draft hierarchy, emitting every boundary read in `(T/2,T]`.
Patch order is forward, forward-right, right, back-right, back, back-left,
left, forward-left; each two-bit digit is fresh=0, visited with rule[0]'s
turn=1, visited with the opposite turn=2. Four-neighbour bits are forward,
right, back, left. Frame is incoming heading, reflected for L-first rules.
`next8` bit j marks equality of rule[(colour+j) mod n] to rule[0]. The full
state also contains the source colour, previous-read lag category and count
of consecutive preceding births (capped at 4). Lag tracking starts at step 1,
including the warmup, and tracks all reads. Window chains are clipped as
in `edge_trace`, with left/right censoring flags kept separately.

The callback is after the move but all state features are captured before
the move. No target outcome or area is encoded. Consecutive emitted events
are linked only within the measurement window; the last is right-censored,
never linked back to the first. Counts output retains state reads/births and
ordered transition counts plus event-gap category/duration sum. Events mode
retains every emitted timestamp. Summary mode avoids count-map allocation.
Tests compare against an independent sparse position oracle and the full
unchanged aggregate tracer, including rotor and random walk.

`examples/c_law_renewal_trace.rs` is a single-run driver. Interface:

```text
band RULE SEED LOG2 W H FRESH_ROWS [counts|events|summary]
blank RULE|rotor:SEED|rw:SEED LOG2 SIZE [counts|events|summary]
```

CSV goes to stdout; a summary JSON goes to stderr. In summary mode the JSON
also goes to stdout. Band initialization matches the parent's hash exactly:
`mix64(mix64(seed) XOR ((x as u64)<<32 | y as u64)) mod rule_length`.
This registration covers the instrument; main models remain unfrozen.

### Calibration and excluded-pilot freeze — 2026-10-20 00:42:53 (+13:00)

Frozen before running these scripts. `scripts/calibrate.py` tests the actual
stationary solver on a two-state chain P=[[.9,.1],[.2,.8]], rewards (.1,.7),
known c=.3, 10^6 events per seed 2026102031..46. G1's <=.01 absolute-error
criterion applies. A periodic chain must converge under lazy iteration;
a reducible chain is rejected pending an explicitly registered class rule.
Negative control: a hidden alternating 256-event regime, birth probabilities
.05/.95, seed 2026102047, 10^6 events. Its pooled c is near .5 even when
P(next birth | birth) and P(next birth | no birth) differ by >.5. This exposes
why a correct total cannot validate history-free closure. G1 calibration
here covers the stationary/reward solver, **not yet the ant state/backoff
model**, which still needs its own synthetic validation after selection.

`scripts/run_pilot.py` builds in a fresh WSL-home target directory, records
binary/source hashes, then runs sequential resumable controls and excluded
pilot only. No heavy job or GPU timing may overlap measurement. Control
seeds 2026102051..58: rotor blank 2^24, 4096 square; random walk blank 2^20,
4096 square (the parent's non-escaping configurations). Mean c must meet G2;
report between-seed sd and escapes, never silently exclude escaped runs.
Small band count output must match byte-for-byte at thread settings 1 and 6.

Pilot follows the original ladder 2^22..2^26 and exact family gate above.
No automatic extension is allowed after failure. Counts at a selected horizon
are recorded only for excluded pilot rules; bulk goes to ignored `dumps/`.
Before main freeze, measure support of S0, S1 and S2 by leave-one-pilot-family
out count summaries, comparing both exact features and rule-independent
coarsenings. This is a feasibility diagnostic, not a validation score. If the
8-turn word isolates rule families, reject this state design rather than
passing a transition model off as a learned lookup of rule identity.

Commands (WSL, usual UV exports; no main-run command exists):

```bash
uv run --frozen experiments/2026-10-20-c-law-renewal/scripts/calibrate.py
uv run --frozen experiments/2026-10-20-c-law-renewal/scripts/run_pilot.py --stage build
uv run --frozen experiments/2026-10-20-c-law-renewal/scripts/run_pilot.py --stage controls
uv run --frozen experiments/2026-10-20-c-law-renewal/scripts/run_pilot.py --stage pilot
```

Run long stages in the background, logging under `results/pilot_v1/`.
Every chunk has its exact argv, build/driver hashes, duration, complete
summary and SHA-256 `.done` marker. This is an instrument/pilot stage and
does not establish P1–P4.

### Exact immediate reward; remaining modelling task

The source state determines the immediate birth outcome exactly. In the
canonical frame, let d=1 (right) when `next8 & 1` is set, otherwise d=3
(left). Then `born = ((visited4 >> d) & 1) == 0`. Consequently a fitted
birth-probability regressor is unnecessary for ant states. The independent
path comparison also checks this identity for each emitted ant event.
The reward-estimator calibration remains a solver control, not evidence
for a learned ant birth mechanism.

What needs explaining is the occupancy of these local states, including
how loops and long returns generate their distribution. A cross-rule
transition model must additionally enforce target-rule-compatible successor
features; a pooled transition matrix can otherwise jump into an eight-turn
word that the target rule cannot produce. This is an explicit main-model
gate, not permission to estimate successor weights on the target trajectory.
Neither the immediate predicate nor the accounting identity derives a or b.

## Pilot outcome and diagnostic addendum — 2026-10-20 00:49 (+13:00)

The original ladder terminated normally with **no usable common horizon**:
0/4 complete families pass at any k=22..26. At k=26 only 2/24 runs pass;
6 miss the >=5000 window-birth requirement and 22 miss the 10–80% consumed
band requirement. The main cohort remains unrun. Source:
`results/pilot_diagnostics.{json,csv}`, made by `scripts/pilot_diagnose.py`.
G0 oracle checks and G2 walker controls pass; the synthetic stationary solver
passes its narrow calibration, but the full ant estimator is not calibrated.
This pilot failure gives no verdict on P1–P4 or local closure.

Before collecting any state counts from this cohort, register a **diagnostic
repeat only** of all 24 excluded pilot runs at k=26, same geometry, seeds and
hashed binary. This changes neither geometry nor horizon and cannot rescue
the failed gate. Budget <=24*2^26 extra steps; no other heavy jobs may run.
Require summaries to match original runs exactly. Save state/transition
counts under ignored `dumps/pilot_support/`, hashes and compact audits under
`results/`; every chunk is validated before its `.done` marker.

Prospective feasibility prediction: S1's full source-colour/eight-turn-word
state will have >5% unseen read mass in at least 12/24 runs when trained on
the other three pilot families. The same test on S0, S2 and C0/C1/C2/C4/C8
coarsenings (word prefix lengths 0,1,2,4,8 plus patch, omitting source colour)
is descriptive. No c model or target transition weights will be fitted.
Passing support would only permit further model design; it would not repair
the horizon failure, demonstrate transfer or establish a mechanism.

Command, usual WSL UV exports: `uv run --frozen
experiments/2026-10-20-c-law-renewal/scripts/support_probe.py`.
