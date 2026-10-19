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
