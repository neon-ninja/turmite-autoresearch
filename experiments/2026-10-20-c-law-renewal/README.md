# Can a local renewal model explain the cost of a new cell?

Scaffold created 2026-10-20 (+13:00), base revision
`adf66774570469da30b10ffc1ceab911670a9d09`.
**Status: prospective design, not a frozen preregistration. No simulations or
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
