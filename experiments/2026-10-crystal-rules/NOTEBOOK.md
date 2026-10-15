# Lab notebook: crystal-rules

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-14 19:03 (+1300) · 6ce3793 · intent
User shared two renders (RLLLLRRL, LRRRRRLLR, 30M steps) and named a third
(RLLLLLLLRRLRR) as "crystal-like", and asked for an experiment design to find out why.
This session designs only (README has hypotheses, stages, predictions); no simulation has
been run. The design separates four mechanisms (glider in a medium, spinner + launcher,
reversibility, frame-builder) and tests whether this is the K3 frame-builder class.

### 2026-10-14 19:03 (+1300) · 6ce3793 · assumption
Renders are unrotated, so the 45-degree rays are lattice diagonals. Check in
`occ/render.py`. Status: untested.

### 2026-10-14 19:03 (+1300) · 6ce3793 · assumption
The three rules are one phenomenon. I only saw two, as images. S0 tests this.
Status: untested.

### 2026-10-14 19:03 (+1300) · 6ce3793 · observation
From the screenshots only (no data file): both renders have 45-degree rays, nested rotated
rectangles, dense blobs at some corners, busiest cell 164 and 139 visits in 30M steps,
bounding boxes 2624x2630 and 2824x2824. The repo notes already record RLLLLRRL as a B5 late
grid escape with no recognised highway at 1.26e10 steps
(`notes/frontier-2026-10-13-results.md`, B5).

### 2026-10-14 19:29 (+1300) · 6ce3793 · observation
S0. `examples/crystal_s0.rs` (WSL: `crystal_s0 RULE 25 4096 experiments/2026-10-crystal-rules/dumps`), blank board, 2^25 steps, deterministic.
RLLLLRRL bbox 2716x2721, visited/bbox 0.9971, steps/visited 4.55, max visits 164.
LRRRRRLLR 2956x2956, 1.0000, 3.84, max 139. RLLLLLLLRRLRR 2442x2442, 1.0000, 5.63, max 291 (cap 255 in the .visits dump).
Controls: RLRRRL 2899x2899, 0.9992, 4.00, max 30; RL leaves the 4096 board (highway). max_visits 164 and 139 equal the
user's screenshots (30M steps), so the renders are reproduced. Renders: `results/*_visits.png` (`scripts/render_visits.py`):
RLLLLLLLRRLRR shows the same rotated-rectangle lattice; RLRRRL is a flat square with only a diagonal X (no crystal).

### 2026-10-14 19:29 (+1300) · 6ce3793 · surprise
The screenshots are visit-count heat maps of a box that is ~100% visited. "Red" is not empty space: it is the
uniform ground state. `scripts/s1_patch.py`: in all three rules the ground is "visited exactly 2x, final colour 2"
and covers 80.2% / 83.3% / 81.1% of visited cells; the control RLRRRL has ground 4x / colour 4. The crystal lives in
the other ~20% of cells. This invalidates the way README prediction 1 is phrased (">= 90% of visited cells on 45-degree runs"
is not a meaningful test when the box is full); see the deviation entry.

### 2026-10-14 19:29 (+1300) · 6ce3793 · observation
`scripts/s1_patch.py` raw patches (RLLLLRRL, LRRRRRLLR): a thin ray is a one-cell-wide 8-connected diagonal line of colour 3
(= ground + 1, visited 3x) in a sea of colour-2 cells (visited 2x). Not a 2-cell staircase of defect. Ray ends in RLLLLRRL show
a short multi-colour motif (colours 4,7,6,0,4 ...) where lines meet.

### 2026-10-14 19:29 (+1300) · 6ce3793 · deviation
Prediction 1 (README) is mis-specified. Replacement for S1-S4, registered here BEFORE measuring it (a different statistic, not
an edit of the old one): define ground state G = (modal visit count, modal colour) of visited cells; "crystal score" =
fraction of non-ground visited cells that lie on a diagonal 8-connected run of non-ground cells of length >= 20, divided by
the same fraction on axial 4-connected runs. Prediction: crystal rules > 3 (diagonal dominates), controls RLRRRL, random-colour
fields ~ 1 or undefined. Also new prediction (from the ground-state result): all crystal rules have G = (2x, colour 2); the
census (S4) will show crystal rules to be those with G visited-count 2, and RLRRRL-type frame builders (G = 4x or 6x)
are not crystal. H-spin's "L^k run spins the ant" is now doubtful: ground colour 2 sits INSIDE the run (RLLLLRRL colours 1-4 are L),
which would make the ant spin on ground cells, not cross them in straight lines; S2 spinner isolation is kept but reinterpreted.

### 2026-10-14 19:38 (+1300) · 6ce3793 · observation
Crystal score as registered (`scripts/crystal_score.py`, minrun 20; dumps 2^25): RLLLLRRL 1.96, LRRRRRLLR 2.32, RLLLLLLLRRLRR 1.95, control RLRRRL 1.25.
Diagonal-run cover of non-ground cells: 0.977 / 0.976 / 0.958 vs 0.454 (control). Registered prediction "score > 3": FAILED as stated (the
axial cover is inflated to 0.4-0.5 by dense blobs and parallel neighbouring rays). The diag-cover separation (0.96-0.98 vs 0.45) is clear but the
threshold is a post-hoc choice.

### 2026-10-14 19:38 (+1300) · 6ce3793 · observation
S2, ant alone on a uniform board of colour G (`examples/crystal_s2.rs`, `scripts/s2_ground_sweep.sh` -> `results/s2_ground_sweep.csv`, 4096 board, 2^22 steps, 4 headings):
RLLLLRRL escapes only from G=2 (4/4 headings, 50 steps per diagonal cell, i.e. ~ (1,1) per 50 steps); LRRRRRLLR from G=2 (58) and G=3 (62.2);
RLLLLLLLRRLRR from G=2 (86) and G=3 (203); control RLRRRL: no G escapes. G=0 (blank board) never escapes for any rule. Every other G confined
(sub-diffusive blobs).
### 2026-10-14 19:38 (+1300) · 6ce3793 · observation
Ground square of side L inside a blank board (crystal_s2 ... 8192 L), ant at centre, 2^26 steps, G=2: RLLLLRRL L=600 -> bbox +-1728 by 2^26, L=1200 -> +-2109;
LRRRRRLLR L=600 -> +-1875, L=1200 -> +-2180. The ant does not leave (position stays inside the box); the box grows far beyond L.
### 2026-10-14 19:38 (+1300) · 6ce3793 · interpretation
Mechanism (confidence moderate, 4 observations agree): the ant fills the box with a 2x-visited "ground" of colour 2; on that ground a lone ant is a true
highway (period 50/58/86 per diagonal cell); a ray is such a highway; at the ground boundary the ant is reflected back into the ground, which sets up
the nested rectangles and makes the box grow. Alternative not excluded: the ray end is not a "reflection" but a new highway started by a blob (the
ray starts/ends are not yet analysed: no trajectory-level ray statistics have been done).

### 2026-10-14 19:38 (+1300) · 6ce3793 · observation
S1 width exponent from bbox at 2^20..2^28 (`crystal_s0 RULE 28 16384`, `results/s1_bbox_octaves.csv`, 9 octaves, least squares): 0.495 (RLLLLRRL), 0.503 (LRRRRRLLR), 0.502 (RLLLLLLLRRLRR), 0.499 (control RLRRRL).
steps/visited at 2^28: 4.38 / 4.03 / 5.31 / 4.00. Fit uncertainty not computed (single deterministic trajectory).
### 2026-10-14 19:38 (+1300) · 6ce3793 · observation
S4 census (`examples/crystal_s4.rs`; 3,512 census rules; `results/s4_census.csv`, `results/s4_tables.txt`): 246 rules fill their bbox at < 10 steps per cell (2^22). Ground state of those: Gv=4 156 rules
(21 with a lone highway on their own ground, but ground fraction 0.99, flat), Gv=6 52, Gv=8 12, Gv=2 23. Grid-level crystal score (`results/s4_crystal_scores.txt`, 2^23, 50 rules) picks out 8 crystals
(non-ground fraction >= 0.13 and diag cover >= 0.95): RLLLLRRL, RLLLLLRRL (= complement of LRRRRRLLR), RLLLLLLRRL, RLLLLLLLRRL, RLLLLLLLLRRL, RLLLLRRRLLL, RLLLLLLRRLRR, RLLLRRRLRLRR. All have Gv=2 and colour 2.
No Gv=4/6 rule is crystal (non-ground fraction 0.007 = the diagonal X only).
### 2026-10-14 19:38 (+1300) · 6ce3793 · observation
S3 mutation map (`scripts/s3_mutants.py`, `results/s3_*`): none of the Hamming-1 mutants of RLLLLRRL, RLLLLLRRL, RLLLLLLLRRLRR is crystal (0/8, 0/9, 0/13; one filled-flat) and 0/142 Hamming-2 mutants;
controls: 1/125 crystal-proxy mutants (all at Hamming-2, RLLLLRRRRLR family). Run-length family (`results/s3_family.csv`): R L^k RRL is crystal for k = 4..15 (ground 0.78-0.83), partial at k=3 (26.8 steps/visited), not at k <= 2;
SEE CORRECTION ENTRY.
### 2026-10-14 19:38 (+1300) · 6ce3793 · deviation
README paths: the census list is at `experiments/2026-10-13-census-sweeps/results/non_highway.txt` (repo was reorganised; `data/non_highway.txt` no longer exists). The census only contains rules of length <= 12, so the 13-letter
RLLLLLLLRRLRR is outside it. The grid-level crystal class used in S3/S4 (box filled, Gv=2, ground fraction < 0.95, or diag cover >= 0.95 with non-ground >= 0.13) is a post-hoc threshold chosen after looking at S0.
### 2026-10-14 19:38 (+1300) · 6ce3793 · surprise
Crystals are isolated points in rule space under point mutation but come in an indel family (the L-run length k). The run does not scale the crystal; it acts as a threshold k >= 4 (ground 2 sits inside the run, so k decides whether
colours 2..k+1 all turn L and hence whether the lone-ant motion on ground 2 is a highway).

### 2026-10-14 19:38 (+1300) · 6ce3793 · observation
corrects: the S3 family sentence in the entry above (I had misread the table; the sed placeholder "SEE CORRECTION ENTRY" refers here). From `results/s3_family.csv` (steps/visited, ground fraction):
R L^k RRL: crystal-like (filled, <6.5 steps/visited, ground 0.78-0.83) for k = 4..15; k=3 filled at 26.8 steps, ground 0.56 (partial); k <= 2 not filled.
R L^k RRLRR: k=4 123.6 steps (not filled fast), k=5 23.8 (ground 0.60), k >= 6 filled at 6.5-8.1 steps with ground 0.75-0.83 (crystal-like); k = 5 is borderline.
R L^k RRRLLL: k=4 is crystal (census rule RLLLLRRRLLL, 2.76 steps, ground 0.86); k=5 filled with ground 0.99 (flat, 2.07 steps); k >= 6 does not fill (steps/visited 76-3600, ground < 0.5 or no ground).
Only the grid-level crystal score was run on the census rules, not on this family (proxy: filled + Gv=2 + ground fraction < 0.95).

### 2026-10-14 22:13 (+1300) · 797704a · intent
Continue S1/S2 with a trajectory instrument and oracle comparison before interpreting interventions. Earlier grid escapes are finite-time escape observations, not proofs of periodic highways; the reflection claim is also provisional until trajectories at ray ends are measured. Existing results will be preserved.

### 2026-10-14 22:14 (+1300) · 797704a · observation
New examples/crystal_rules_trace.rs passed cargo test --release --example crystal_rules_trace (32 property cases, random rules of length 2..12, up to 1,999 steps, stepwise position/heading/read colour/updated cell against reference::Ant). Log: results/trace_oracle.log. This verifies the NEW loop only; it does not retroactively verify the earlier S0/S2/S4 loops. Windowed traces generated for RLLLLRRL, LRRRRRLLR, RLLLLLLLRRLRR and RLRRRL, blank board, 33,554,432 steps, size 8192, window 4096; results/trace_RULE.csv. The final partial window is retained. No automatic ray classification or intervention result is claimed.

### 2026-10-14 22:16 (+1300) · 7eff085 · observation
Paired patch experiment: all 12/12 samples per rule had greater min(abs(dx),abs(dy)) after 4096 steps on colour-2 exterior than colour-0 exterior. Command: cargo run --release --example crystal_rules_trace -- intervene RULE; output results/local_patch_RULE.csv. Summary: uv run experiments/2026-10-crystal-rules/scripts/local_patch_summary.py -> results/local_patch_summary.json (input SHA256 recorded). These are selected windows of one deterministic blank-start trajectory, not independent seeds. Original state preserved only inside radius 16 in each coordinate; this is a local-patch test, not a strip transplant.

### 2026-10-14 22:16 (+1300) · 7eff085 · interpretation
The local environment causally affects continued diagonal travel in these sampled states. This supports medium-dependent propagation, but proves neither exact periodicity nor reflection at ray ends. The new stepwise oracle test passed again after intervention support was added (results/trace_oracle_v2.log).

### 2026-10-14 22:17 (+1300) · 7eff085 · decision
Release suite completed successfully (results/release_suite.log), and the example oracle tests passed separately. Recipes and source hashes are in results/trajectory_recipe.txt. Current S1/S2 advance: verified windowed trace loop and completed paired local-patch intervention. Remaining: actual ray segmentation and calibration, exact periodicity proof/check, ray-end knockouts and spinner isolation; earlier instrument loops remain unverified. No commit made. The paired intervention supports a medium effect, not the previous stronger reflection claim.
