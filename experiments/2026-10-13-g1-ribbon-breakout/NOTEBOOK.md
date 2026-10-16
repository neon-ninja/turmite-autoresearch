# G1 lab notebook

### 2026-10-15 21:22 (+13:00) · da88e61 · intent
User requested RLRLLL anatomy and comparison with other escaped-without-highway
survivors. Read Q8, frontier G1 and its withdrawal before analysis. Register S
and T in README before computing; reuse existing traces and all completed chunks.

### 2026-10-15 21:22 (+13:00) · da88e61 · assumption
Census extents are whole-history bounding boxes (verified in sweep_lean source),
not local ribbon thickness. A failed transfer proxy cannot refute local ribbons.
Blank starts are deterministic; rules, not windows, are the sampling units.

### 2026-10-15 21:20 (+13:00) · da88e61 · deviation
Timestamp correction: the registration and initial entries say 21:22, entered
from an estimated clock; actual tool clock after analysis was 21:19. Registration
was written before running compare_survivors.py, as tool order records. Keep the
original text intact. No new trajectory simulations were run. WSL required a
sandbox escalation after E_ACCESSDENIED; the approved retry succeeded.

### 2026-10-15 21:20 (+13:00) · da88e61 · observation
Command: uv run experiments/2026-10-13-g1-ribbon-breakout/scripts/compare_survivors.py.
Outputs: results/survivor_comparison.json, results/survivor_geometry.csv.
Complete fine windows: 974; breakout 298, preceding 676. S passes in 94.2953%
of breakout and 0% preceding windows. Median steps/new cell 4.022; transverse
new-cell box median 54; none <=12. First breakout window ends 12677000000.
The historical 299-window count includes the partial final window excluded here.
Census: h1e10 has 807 rules (732 budget, 38 highway, 37 no-highway escapes);
h1e11 has 324 completed rules (317 budget, 4 highway, 3 no-highway escapes).
Zero of 37 and zero of 3 no-highway escapes have aspect >=2. Transfer T fails
in both cohorts. Median aspects are 1.000031 and 1.002050 respectively.
Later escapes: RLLLLRRL, RLLLLRLRRL, RLLLRLRLLL. Their maximum-extent expansion
from 1e10 is 1.1691, 2.1892, 8.5065 respectively (not a rate or local thickness).
RLRLLL is h1e10/chunk_29.csv: budget, 3377 x 3250 at 1e10. It is listed in
h1e11/chunk_27.txt but has no completed CSV row; its escape comes from G1.

### 2026-10-15 21:20 (+13:00) · da88e61 · dead-end
T is falsified. Whole-history box aspect is a poor transfer instrument: even
37 of 38 earlier recognised highways fail aspect>=2. Stopping on a square
board and pre-existing territory confound this proxy. It cannot classify local
ribbons or rule out repeated strokes that together fill a square box.
The original G1 <=12-cell prediction remains failed; efficiency is not periodicity.

### 2026-10-15 21:20 (+13:00) · da88e61 · interpretation
S separates the two already known RLRLLL phases descriptively; thresholds were
chosen with historical G1 summaries in view, so this is not independent discovery
or a causal test. Window counts are correlated, no window-based confidence interval.
Other survivors share board escape but do not share elongated whole-history boxes.
Their local mechanism is unmeasured. No evidence here establishes a surface wave,
a repeat period, or an asymptotic universality class.

### 2026-10-15 21:20 (+13:00) · da88e61 · decision
Close archival comparison with failed T reported. Next discriminating measurement:
record equal-duration local windows for all three later escapes on a larger board,
plus pre-onset and recognised-highway controls; apply S unchanged, add moving-frame
tape/field recurrence to distinguish parallel strokes from periodic translation.
Do not treat absent unfinished chunks as budget survivors or merge decades as iid.

### 2026-10-15 21:23 (+13:00) · da88e61 · observation
Verification: second analysis run produced identical SHA-256 for both outputs:
JSON f0fcadf227826de070f3c19d22b07c1cfe3b411d3e66dda433597a9607e300a4;
CSV 6cad1fdcb8e15e297572ec7777b05cd9fdbc7f4be7028f3b436be25113c7c507.
Shareable notebook executed top to bottom via uv run --with nbconvert --with
ipykernel --with nbformat jupyter nbconvert --to notebook --execute --inplace
experiments/2026-10-13-g1-ribbon-breakout/notebook.ipynb.
cargo test --release passed; log results/comparison_tests.log. No dependencies
changed. New results are pending commit because the user did not request one;
notebook reads only results paths, which should be committed together.

### 2026-10-16 21:09 NZDT · c9811f5 · intent
User requested a three-part paper figure: switch, period-3 wake, and predicted
versus measured d|x|²/dt. Recovered an existing draft and compact extracts from
worktree-paper-figure-proposals (b7c55f9 plus untracked files); original exporter,
raw dumps and hypothesis note from worktree-g1-builder-structure (c82030e).
Use archival inputs, verify hashes and extract reproduction before plotting.
No new physical hypothesis, estimator or simulation measurement is proposed.

### 2026-10-16 21:16 NZDT · c9811f5 · observation
Raw dump SHA-256 hashes matched the archived manifest. g1_wake_extract.py
reproduced every recovered compact CSV byte-for-byte. The original scaling
analysis uses strict +/-20M windows and the slope of |x| times 2|x(t0)|;
the draft used a different fit. Restored the original method, reproducing
0.862245 and 0.832701 (39 samples each) versus 0.864883 and 0.835453 predicted.
Commands and outputs: README and paper fig/late_switch_numbers.json.
Independent exporter check passed all 28 rules of lengths 2--4, including
all wake bytes; cargo test --release passed. See results/*check.log and
figure_release_tests.log. First check attempted length-1 rules, correctly
rejected by Rule parsing; test domain corrected to valid rule lengths.
The first plot build needed a Polars integer-index cast; initial Typst caption
needed spaced math identifiers. Both corrected. Notebook execution passed.

### 2026-10-16 21:16 NZDT · c9811f5 · decision
Recovered originals instead of starting the proposed new simulation. Cropped
the displayed wake to 36x48 square cells and added the sweep trace to panel b;
caption distinguishes snapshot end time from trajectory start. PDF page 28
visually checked: complete figure and caption, no clipping. Two local rates
remain post-hoc same-trajectory consistency checks, without seed error bars.
Intent timestamp 21:09 was approximate; actual first tool clock was 21:08:56.

### 2026-10-16 21:21 NZDT · c6b5053 · observation
Archive and reproduction tools committed in c6b5053. Verified raw hashes,
byte-identical re-extraction, and byte-identical PDF/PNG on repeat: results/
figure_verification.json (renderer pypdfium2 5.14.0). Figure is on manuscript
page 28. Final legend includes all six states because boundary defects include
states 0--2; amended prose from exact periodicity/only 3--5 to strong periodicity
and dominance by 3--5, matching the measured 0.994 agreement. Notebook rerun
against committed results. No full unrelated-figure rebuild was needed.

### 2026-10-16 21:19:04 NZDT · c6b5053 · decision
Closing status: figure, caption, audit recovery, rebuild hook and shareable
notebook complete; final rendered page inspected, verification passed. The
previous 21:21 heading was an estimate written ahead of the clock; actual clock
at this closing entry is 21:19:04. Next figure proposals remain unimplemented.
