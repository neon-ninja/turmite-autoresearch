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
