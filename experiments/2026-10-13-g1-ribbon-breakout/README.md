# g1-ribbon-breakout

Migrated legacy investigation (2026-10-14). Its files used to sit in the flat `data/` directory (and `calib/`, `occ/`, `snaps/`, `dumps*`, `knock*` where listed) with the label prefix below; `experiments/_migration/map.tsv` gives the exact old -> new path of every file. This stub was generated from file names and script docstrings. It is **not** the original write-up: the question, timestamped hypotheses, commands and results are in `notes/` (mainly `notes/frontier-results.md`, `notes/frontier-status.md`, `notes/research-frontier*.md`; search for the label), and the producing command is usually in the script docstring or a `run_*.sh` here.

Old labels / prefixes: g1

## Scripts

- `scripts/g1_breakout_stats.py`: dependencies = ["polars", "numpy"]
- `scripts/g1_stats.py`: G1 analysis of experiments/2026-10-13-g1-ribbon-breakout/results/g1_ribbon.csv (made by examples/ribbon_trace.rs). Predictions: notes/research-frontier-2026-10-

## Results

3 entries in `results/` (1 are `.log`/`.done` markers); csv/json/txt outputs, `*_stats.txt` hold the summaries quoted in the notes.

## 2026-10-15 21:22 NZDT: survivor comparison registration
Base commit da88e61b690507526ba4e2a2c017d9c7941beb91; existing dirty work is unrelated.
Before computing the comparison: use every chunk CSV with a sibling .done in
h1e10 and h1e11, separately (not the historical 240-rule subset). Escape means
stop=escaped and empty period; no claim of infinite-time nonperiodicity.
All runs are deterministic blank-board starts, no random seed.

Structural signature S: in complete 1M-step G1 windows, steps/new cell <8 AND
axis-aligned new-cell box aspect >=20. Predict >=90% of the previously defined
breakout windows (steps/new cell <10) satisfy S; <=5% of preceding complete fine
windows satisfy S. Calibration: a synthetic 40-by-10000 slab at 4 steps/cell
passes; a square at the same efficiency and a thin slow slab fail.
This is a window-box signature, not a measurement of individual stroke width.

Transfer prediction T: >=75% of no-highway escapes in EACH completed census
have whole-history box aspect >=2. This deliberately weaker census proxy tests
whether elongated territory is shared; it cannot validate S in another rule.
Compare against escaped recognised highways and budget survivors. Report all
rules, dimensions and prior-decade expansion ratios where available. RLRLLL
is a budget survivor at 1e10, so its later escape belongs to the 1e11 cohort.
No new multi-billion-step simulations are needed for this archival comparison.

Recipe: uv run experiments/2026-10-13-g1-ribbon-breakout/scripts/compare_survivors.py
under WSL with UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy.
Outputs: results/survivor_comparison.json and results/survivor_geometry.csv.
Input SHA-256 hashes and script hash are stored in the JSON. Re-run must give
identical bytes. Historical generator: examples/ribbon_trace.rs (command in
notes/research-frontier-2026-10-13-pm.md G1); census: src/bin/sweep_lean.rs,
blank board, grid 32768, budgets 1e10/1e11, original logs beside chunks.

## 2026-10-15 archival outcome
S held: 281/298 complete breakout windows (94.3%), 0/676 preceding windows.
Median efficiency 4.022 steps/new cell; median transverse window box 54 cells;
the old <=12 prediction still fails (0/298). Partial final window excluded.
T failed: 0/37 h1e10 and 0/3 h1e11 no-highway escapes have global aspect >=2.
The completed h1e11 cohort is 324, not the historical 240 or planned 732.
Its three no-highway escapes are RLLLLRRL, RLLLLRLRRL, RLLLRLRLLL;
max-extent expansion relative to 1e10 is 1.1691, 2.1892, 8.5065.
RLRLLL is in h1e10/chunk_29.csv (3377 x 3250), and in h1e11/chunk_27.txt
without a completed CSV row. Its later evidence is the G1 trace, not a completed
h1e11 census record. Full per-rule comparisons and input hashes are in outputs.

Interpretation: the efficient narrow-window signature is descriptive evidence
for RLRLLL's phase switch, not an independently discovered mechanism. Global
aspect does not identify local ribbons: even recognised highways usually have
nearly square whole-history boxes at board escape. No shared local mechanism
has been established for the other survivors. Need local-window traces and
moving-frame recurrence tests, on larger boards, to distinguish parallel strokes
from periodic translation. No new simulations or causal claims.
Timestamp correction: registration's 21:22 was an estimated time; actual clock
was 21:19 after the first analysis. Tool order establishes registration before run.

## 2026-10-17 column-sweeper figure extract
- scripts/g1_wake_extract.py <dumps dir>: reads the gitignored builder_dump outputs (g1_<T0>.traj.csv, g1_<T0>.wake.bin, T0 = 12800000000, 12900000000; produced by examples/builder_dump.rs, see 
otes/g1-builder-structure-2026-10-14.md) and writes esults/g1_wake_extract.json (L, x, colour shares, lag agreement, dump sha256, crop coordinates) plus esults/g1_wake_{shape,crop}_<T0>.csv and esults/g1_sweep_trace_<T0>.csv.
- Consumer: esearch-outputs/cube-root-growth-paper/late_switch_figure.py (paper Figure 16). Regeneration needs the dumps, which exist only in the g1-builder-structure worktree (commit 18b49fc); the committed extract is the durable record.
