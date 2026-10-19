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
