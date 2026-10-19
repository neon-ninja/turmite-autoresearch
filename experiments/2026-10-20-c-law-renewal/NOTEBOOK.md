# Lab notebook: local renewal model of the c law

Append-only. Corrections are new dated entries. No measurements in this scaffold.

### 2026-10-20 00:30 (+13:00) · adf66774570469da30b10ffc1ceab911670a9d09 · intent

Scaffold the selected enquiry: explain boundary reads per birth through a
local reward process, and challenge it with lead- and composition-preserving
early/late swaps. Read the parent's README, `src/edge_trace.rs`, its driver,
`notes/open-questions-hypotheses.md` and `notes/calibration.md` before design.
User requested a scaffold, not a simulation campaign.

### 2026-10-20 00:30 (+13:00) · adf66774570469da30b10ffc1ceab911670a9d09 · assumption

Local boundary states may close approximately under transition. Untested.
Births are not known regeneration times; stationary event weights are not
automatically transferable between rules. Matched interior seeds provide
paired replicates, not independent boundary reads.

### 2026-10-20 00:30 (+13:00) · adf66774570469da30b10ffc1ceab911670a9d09 · decision

README predictions are draft until instrument calibration, pilot support and
model choices are frozen. Existing aggregate tracer cannot train ordered
transition models; do not simulate the main cohort before that gap is filled.
Keep this work isolated from the lattice and blog edits already in progress.

### 2026-10-20 00:34 (+13:00) · adf66774570469da30b10ffc1ceab911670a9d09 · observation

First design-generation attempt caught an invalid swap at the end of the
opening run: index 4 can be the first opposite letter when lead=4. Added an
explicit lead-preservation rejection before accepting any family; no design
files had been written by the failed attempt. Re-ran `uv run --frozen
experiments/2026-10-20-c-law-renewal/scripts/prepare.py` and `--check` in WSL:
52 families generated and byte-verified (4 excluded pilot, 32 training, 16
validation; three rules each). These are design counts, not measurements.
Provenance and exact source hashes are in `results/design/provenance.json`.
WSL initially required sandbox escalation; the authorized retry succeeded.

### 2026-10-20 00:36 (+13:00) · 3e7b307 · observation

`cargo test --release` completed with exit status 0 in WSL (existing compiler
warnings; no Rust changed by this scaffold). Local full log:
`scratch/release-tests.log` (ignored). `prepare.py --check` verified the
committed design. Executed `notebook.ipynb` top to bottom in a fresh kernel
with the README's `uv run --frozen --with jupyter --with nbconvert --with
ipykernel jupyter nbconvert ...` command; saved small design-audit outputs.
The first notebook attempt rejected WSL mount-only executable-bit changes;
the read-only content check now passes `git -c core.filemode=false diff`.
Dependencies were not added to the project or lockfile.

### 2026-10-20 00:36 (+13:00) · 3e7b307 · deviation

Another active repository session committed the initially staged scaffold
alongside its lattice results in 3e7b307 before this session's commit ran.
Kept that existing commit intact. The scaffold's recorded base revision is
still adf66774570469da30b10ffc1ceab911670a9d09; generated-input hashes are
unchanged. Final notebook and verification entries get a separate scoped
commit. No unrelated edits were reverted.

### 2026-10-20 00:36 (+13:00) · 3e7b307 · decision

Scaffolding complete. Ordered-event instrument, estimator implementation,
synthetic calibration, walker controls, pilot and main simulations remain
unrun. The draft blog entry and open-question pointer label that status.
No homepage result highlight is warranted by design preparation alone.

### 2026-10-20 00:37 (+13:00) · a976af5 · intent

The active goal now authorizes continuing beyond scaffolding: learn the local
process governing reads per birth. Previous turn made progress by committing
the reproducible scaffold. Authoritative inspection finds no ordered-event
instrument or live run for this experiment. Implement a callback instrument
and an independent sparse-path oracle first, retaining complete read-lag
categories and both window-edge censoring flags. Do not open main-cohort
outcomes before the pilot and model freeze.

### 2026-10-20 00:37 (+13:00) · a976af5 · assumption

New lattice results in `notes/open-questions-hypotheses.md` reject a slope
proportional to loop length. This does not invalidate testing square-lattice
local closure, but rules out interpreting a loop-cost decomposition as a
universal slope derivation without a separate model for how many cells share
each loop. No new measurement made in this session yet.

### 2026-10-20 00:52:43 (+13:00) · 02f5d13 · observation

Implemented `src/edge_renewal.rs`, the single-run driver, sparse path oracle
and aggregate-oracle comparisons. Initial test failed on the invalid "R"
fixture (rules require >=2 colours); corrected to "RR", then all four test
groups passed. The recorder also satisfies the exact immediate-birth
predicate for every oracle-tested ant event. Entire `cargo test --release`
passed with exit 0 after the pilot (log `scratch/full-release.log`). No GPU
files or probe estimator changed; no CUDA tests required for this addition.

### 2026-10-20 00:52:43 (+13:00) · 02f5d13 · observation

Frozen calibration/pilot scripts ran via the detached
`scripts/launch_pilot.py` -> `run_pilot_all.sh` job. PID 1642950 was observed
live during the run; terminal `session.exit` is 0. An earlier shell launch
never created a PID/log or live child, verified before the successful retry.
Fresh-build recipe, binary/source hashes, per-run argv and resumable markers
are under `results/pilot_v1/`; commands are in README.

Synthetic stationary reward calibration: 16 seeds x 10^6 events, maximum
absolute c error 0.0018502, signed bias +0.0002108, between-seed sd 0.0008653.
Hidden history: pooled c 0.499742, conditional contrast 0.802996. This passes
the solver/negative-control criteria; the ant backoff estimator is still
unimplemented. `results/calibration.json` records all seeds and code hash.
Rotor (8 seeds): c mean 0.497504, between-seed sd 0.000467; random walk (8):
0.446602, sd 0.008122; no escapes. Count CSV identical at thread settings
1 and 6. G2 passes. See `results/pilot_report.json`.

### 2026-10-20 00:52:43 (+13:00) · 02f5d13 · surprise

The original pilot horizon gate failed for every family at every k=22..26.
At k=26 only 2/24 individual runs meet both requirements: 6 fail >=5000
window births; 22 fail 10–80% band consumption. Boundary-time fractions
range from 0.0000459 to 0.01148. No rules were screened or replaced.
`scripts/pilot_diagnose.py` -> `results/pilot_diagnostics.{csv,json}`.
The low event supply may come from geometry, starting-field trapping or
genuine rule dynamics; these summaries do not distinguish the causes.

### 2026-10-20 00:52:43 (+13:00) · 02f5d13 · decision

Do not launch the main cohort or claim closure failed. A registered support
diagnostic repeats only excluded pilots at the same k=26/binary, retaining
the horizon failure. It will test whether the source-colour/eight-turn word
isolates families before any model is fitted. A separate lattice sweep
(observed live PID 1655913) currently occupies the machine; wait for it to
finish before this diagnostic. Main prediction tests P1–P4 remain unrun.

### 2026-10-20 00:57:10 (+13:00) · e81f238 · observation

Updated shareable notebook from committed calibration/pilot summaries using
`scripts/build_notebook.py`, then executed it top to bottom in a new kernel
with the README's nbconvert command. Outputs reproduce control means,
between-seed variation and failed horizon gate. Support results are explicitly
pending. `uv run --frozen --only-group blog blog/scripts/test_experiments.py`
passes (2 tests); homepage and reader guide now point to the pinned pilot
failure, not a claimed mechanism. The full release suite already passed.

### 2026-10-20 00:57:10 (+13:00) · e81f238 · decision

`uv run --frozen experiments/2026-10-20-c-law-renewal/scripts/launch_support.py`
queued the registered support diagnostic. Worker PID 1662130 was confirmed
live with its exact command; it checks actual `/proc` command lines before
starting, waiting for all observed release/cargo jobs. The earlier lattice
PID 1655913 finished; a new live colour-analysis job PID 1661904 still runs.
Worker log: `results/pilot_support/session.log`; runtime PID/exit files are
local and ignored. Do not restart this job based on a stale log or observation
timeout: inspect that handle and terminal status first. The research goal is
not achieved: no ant transition model, held-out prediction, or coefficient
derivation exists yet. Sampling and physically compatible transitions remain
the next scientific decisions after the excluded support audit.

### 2026-10-20 00:59:52 (+13:00) · e7052b7 · observation

Queued worker 1662130 waited for actual heavy jobs, then completed the
registered support repeat with terminal status 0. All 24 counts-mode
summaries exactly match the original k=26 summaries; deterministic reward
and incoming/outgoing/censoring flow checks pass. `scripts/support_probe.py`
produced `results/support_report.json`; bulk count tables and their hashes
are under `dumps/pilot_support/` and `results/pilot_support/` respectively.

Every S0/S1/S2 read is unseen under leave-one-pilot-family-out support:
100% unseen mass in all 24 runs. The registered feasibility prediction held.
Without source colour, eight turns + patch still has median 0.94443 unseen
mass. Current turn + patch has median 0.02264, maximum 0.07459 (7/24 >5%).
No c model, main outcomes or original P1–P4 test was fitted or scored.

### 2026-10-20 00:59:52 (+13:00) · e7052b7 · interpretation

The proposed rich lookup has no transfer support in this pilot. It preserves
so much rule context that different families never share its observed states.
This is a representation/feasibility failure, not evidence against all local
mechanisms, and not a proof about the larger planned training cohort. Coarse
states share support but omit the explicit future phase sequence. A candidate
revision is compositional: apply the target rule's colour clock exactly and
learn only unknown patch/return conditions. That proposal still requires a
sampling revision, synthetic calibration and new frozen predictions before
any main measurement. The original horizon failure stands.

### 2026-10-20 01:02:24 (+13:00) · 0fa4db2 · observation

Rebuilt and re-executed `notebook.ipynb` in a clean kernel after committing
the support summaries; it reproduces the complete support table and retains
all untested predictions as untested. Experiment-page publication tests pass
again (2 tests). Updated the open-question record, active-work index, blog
draft, reader guide and homepage with the support failure and pinned evidence.
The support worker is terminal with exit 0; no experiment job remains live.
Release tests passed earlier; only data, notebooks and documentation changed
after that check. Goal remains active: a supported local process determining
state frequencies, a tested replacement closure and coefficient derivation
are still missing. Next session must redesign and register those tests,
not relaunch the completed diagnostic or score the original rich lookup.

### 2026-10-20 01:06:34 (+13:00) · a95ad13 · intent

Previous goal turn was progress: calibrated recorder, failed sampling gate
and unsupported rich representation were measured and committed. Inspected
current files and goal; no own experiment job is live. A separate lattice
colour job (PID 1663389) is live, so do not begin new ant measurements yet.
Register the finite colour-memory intervention in README before implementation:
apply target-rule phase updates exactly while resident; erase only forgotten
phases, retaining the model's own visited geometry. M0 oracle tests come first.
Four excluded base rules, two seeds, fixed radii/policies; main cohort unopened.

### 2026-10-20 01:06:34 (+13:00) · a95ad13 · decision

The earlier 10% consumed-band requirement is not retained for this new pilot:
it mixes a size requirement into a boundary-cost measurement and failed
despite some substantial boundary samples. Replace it prospectively by minimum
boundary reads/births and an upper depletion bound, reporting every exclusion.
This does not change the original failed verdict. The finite-memory model is
a counterfactual colour-field closure, not an unmodified ant and not a proof
that local geometry alone suffices. Its source phase must be reset before
instrumentation; a Walker.step-only reset would label the wrong phase.

### 2026-10-20 01:23:41 (+13:00) · a95ad13 · deviation

Before any new model run, amended the proposed fixed-time pilot to a boundary
clock: warm 2048 reads, measure 8192, global cap 2^28 steps. The original
fixed-time proposal remains in README. At most 10240 births can occur before
completion, analytically preventing band depletion (<2%) in every completed
run. Complete model windows with zero births remain outcomes, not exclusions.
This is a new prospective window; rotor/rw calibration and independent
offline-window equivalence are mandatory before interpreting comparisons.

### 2026-10-20 01:23:41 (+13:00) · a95ad13 · observation

Implemented finite colour cache, source-preparation hook and boundary clock.
Release checks: 5 local-memory tests, 4 existing renewal tests and 5 original
aggregate-tracer tests pass; driver builds and Python runner/analysis compile.
Logs: `scratch/memory-instrument-tests.log`, `scratch/memory-driver-build.log`.
The new walker matches independent HashMap cache eviction step for step with
seeded wrapping paths; full mode equals AntWalker. Known loops and an entire
path retained in-cache have zero resets. No ant-pilot outcomes measured yet.

### 2026-10-20 01:23:41 (+13:00) · a95ad13 · interpretation

Exact geometric deduction: alternating axes imply a closed k-step path has
k/4 positive and k/4 negative moves along each axis, so no coordinate can
stray by more than k/4. Cache eviction at radius r therefore requires a
return lag >=4(r+1). Radius 4 preserves every <=16-step source phase. Tests
exhaust all 16-step turn words/closed prefixes and check the bound on every
oracle-observed reset. This clarifies the intervention: it directly erases
only long-return phases while preserving immediate short-loop clocks, though
the changed path may alter the frequency of those short loops indirectly.
It does not yet explain the measured c-law coefficients.

### 2026-10-20 01:23:41 (+13:00) · a95ad13 · decision

Freeze `memory_design.json`, implementation and M1/M2/M3 analysis before
launch. Cases are four excluded bases x two seeds x seven arms, no fitting or
main-outcome access. A fresh build re-runs oracle tests; boundary-clock rotor
and random-walk controls gate the pilot. Source hashes and deterministic
per-case markers make the run resumable. Most recent process inspection
showed no cargo/lattice/memory runner live, but the launcher rechecks actual
heavy jobs before each stage. Do not claim locality from a low-exposure match.

### 2026-10-20 01:37:33 (+13:00) · c826721 · observation

Worker PID 1686706 ran build, controls, 56-case pilot and registered analysis;
terminal `results/memory_v1/session.exit` is 0. Fresh binary SHA-256
393aafa68177b7c51f94a967583290a742b7c6912da65e3ef9f15b4033eb29e7,
full recipe/source hashes and logs in `results/memory_v1/build.json`.
Boundary-clock controls, 8 seeds each: rotor c 0.500748, between-seed sd
0.003133; rw 0.462433, sd 0.006132. Relative to the old fixed-time means,
differences are +0.003244/+0.015830; record that window bias rather than
treating the early clock as asymptotically calibrated. Source/clock oracles
and the entire `cargo test --release` pass; full log
`scratch/memory-full-release.log`. No new dependencies or GPU code changed.

### 2026-10-20 01:37:33 (+13:00) · c826721 · surprise

All 8 full references and uniform r=4/r=8 arms complete their windows.
Registered M1 and M2 FAIL for both radii. Median absolute ln c ratio is small
(0.012054/0.012623) but maxima 0.298926/0.276185 exceed 0.25. Median largest
budget errors 0.248662/0.200881 exceed 0.20 (r=8 narrowly). Do not round the
last value into a pass. Lead-4 RRRRLLRRLLLLRR has c 31–35% too high at r=4
in both seeds, not an isolated noisy point. This corresponds to about
24–26% fewer reads per birth, not a 35% increase in read cost.

Six zero-reset arms cap at 2^28 steps before the measurement window opens;
only the lead-3 rule supplies two defined comparisons. M3 is UNINFORMATIVE.
That rule has zero boundary reset exposure at r>=4, so its close match is
weak evidence. Every case, cap and paired comparison is retained in
`results/memory_report.json` and per-case files/markers.

### 2026-10-20 01:37:33 (+13:00) · c826721 · interpretation

Post-failure exploratory `scripts/diagnose_memory.py` ->
`results/memory_diagnostics.json`: r=4 on the lead-4 rule removes 1.63/1.59
long-return reads per birth while adding 0.51/0.47 lag-4 reads; net cost falls
0.95/1.05. Lead-2 short/long changes nearly cancel in the total. The exact
radius bound ensures short source phases were not directly reset; the altered
long-return history indirectly changes short-loop frequencies. This is the
confound the registered budget challenge was intended to detect.
Full-ant event-clock versus old k=26 maximum absolute ln c ratio is 0.0406
on these same eight seeds, an exploratory age audit, not a universal bias bound.
Neither a close total nor this audit establishes the local closure.

### 2026-10-20 01:37:33 (+13:00) · c826721 · decision

Stop the uniform-forgetting closure as registered. Do not enlarge the radius
or open main outcomes. Next candidate: retain only the old turn bit at
forgotten cells, sampling a phase within that turn class; compare preserving
two future turns. This can distinguish a directional exterior field from
the distant full phase clock. It is still a proposal requiring a new oracle,
registration and held-out test; the mechanism and c-law constants remain open.
Rendered `results/memory_pilot.png` from the report with `scripts/plot_memory.py`.
Inspected the image, corrected only its generated y-axis bounds/label using
`--replace`, then inspected again; numerical inputs were not changed.

### 2026-10-20 01:41:53 (+13:00) · a878d5e · observation

Rebuilt the shareable notebook with `scripts/build_notebook.py` and executed
it top to bottom in a clean kernel using the README nbconvert command. It
reads committed JSON/image summaries only, recomputes the primary medians and
maximum errors, and reproduces both registered failures and exploratory
decomposition. Saved small outputs include the inspected figure. Publication
tests (`uv run --frozen --only-group blog blog/scripts/test_experiments.py`)
pass (2 tests). Updated the notes, active index, experiment guide and homepage;
added a coherent blog entry at `_posts/2026-10-20-the-short-loops-kept-their-clocks.md`
with pinned evidence. No own simulation or build job remains live.

### 2026-10-20 01:41:53 (+13:00) · a878d5e · question

What information must a forgotten cell keep? A prospective one-bit exterior
model could preserve its old current turn and randomize the phase within that
equivalence class; a two-turn version preserves some future timing. These
are interventions on the model's own generated history, not reads from a
target ant. A match requires substantial actual phase scrambling and correct
return budgets, not just close c. No variant is implemented yet; register
predictions and a new sparse oracle before running. Main outcomes and original
P1–P4 remain unopened/untested. The full scientific goal is not achieved.
