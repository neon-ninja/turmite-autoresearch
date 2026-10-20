# Lab notebook: RLRLLL column sweeper as a macro-transition

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-20 23:40 (+13:00) · ba38590 · intent
Start a proof-oriented track in parallel with the empirical programme, prompted by a proposal (from the project
owner) to turn simulated regularities into machine-checkable macro-transitions. First target: the RLRLLL column
sweeper, because the G1 note already shows a period-3 wake and a cycle time proportional to sweep height. Predictions
P1-P5 registered in README before any run here. This runs in a cloud container (4 cores), not on the WSL machine
where the 1e11 census holds all cores, so it does not compete with that job.

### 2026-10-20 23:40 (+13:00) · ba38590 · assumption
The steady phase reads only a bounded strip behind the front (P3). If false, no finite macrostate exists without
carrying the wake. Checked directly by the reach statistic W_k. Status: untested.

### 2026-10-20 23:44 (+13:00) · 48c47ad · observation
Trace done in 2 min 53 s: armed at 12,677,001,095, left the board at 12,974,495,927 (same step as G1's ribbon_trace),
4,729 cycles. `results/stage1_cycles.txt`: T - 12 L takes exactly 358/394/518 in a period-3 cycle on every cycle;
g = 2 on every cycle; reach max 20. `results/stage1_snapshots.txt`: 373 snapshots, bulk period 1, 3 top blocks,
1 bottom block. P1-P5 held (P4 on the sampled boundaries only).

### 2026-10-20 23:45 (+13:00) · 48c47ad · surprise
The family already holds at the very first cycle after the G1 onset time, with m = 434 (column height 477). The sweeper
is not born small at 1.2677e10; either it was running earlier with smaller m, or it starts at full size. Worth a direct
look (entry question).

### 2026-10-20 23:47 (+13:00) · 48c47ad · dead-end
First checker run: no shift lemma ever applied. Diagnosis by replay: the descent moves 2 rows per 20 steps while
alternating columns 3 and 4, so the moving frame has period 2 in a period-1 bulk; and my lemma required the ant inside
A, while a peel is only triggered after the ant has already stepped onto the band. Fixed both (k-fold block regrouping,
with m = 2j since m is even in every phase; a sentinel row the ant may reach but not read at the lemma's last step).

### 2026-10-20 23:50 (+13:00) · 48c47ad · observation
Checker: CLOSED. C(m,q) -> C(m+2, q+1) in 12m + (934, 1058, 898) for all even m >= 2, via a descent lemma (H=2,
p=2, s=20) and a return lemma (H=1, p=2, s=4). 27 brute-force reference runs (m = 2..434, three phases) agree; the
law matches all 4,728 measured cycle lengths. Negative controls rejected (mutated bulk cell; rule RLRLLR).
`results/check_k0.txt`, `results/check_negative_controls.txt`, `results/stage2_compare.txt`.

### 2026-10-20 23:53 (+13:00) · 48c47ad · interpretation
With snapshot 0 as base case, RLRLLL from blank never builds a highway and |x|^2/t -> 3/4 exactly; G1's 0.8634 was
finite-size. Confidence: high in the closure (two independent routes, symbolic and brute force, plus controls); the
base case rests on one uncertified 1.27e10-step simulation. Alternatives not excluded: a shared bug between the
tracer and the extraction would only affect the base case, which the 4,728-cycle agreement makes implausible.
Not explained: the colour period 3, the bulk row, the end blocks. The checker certifies; it does not explain.

### 2026-10-20 23:58 (+13:00) · 48c47ad · observation
Entry probe: `rlrlll_macro_transition_trace RLRLLL 12600000000 $E/scratch/run_early 32768 48 1000000` →
`results/cycles_early.csv`. The first front advance after 1.26e10 is at step 12,676,421,767 (x_f = -1820, the old blob
edge, box about ±1770 at 1.2e10). From that very cycle the law already holds exactly: g = 2 on all 4,853
differences, T - 12 L in {374, 402, 530} in period 3 on all 4,854 cycles (cut at a different column residue from run1,
hence different constants), reach max 20. Column height at the edge: L = 226.

### 2026-10-20 23:59 (+13:00) · 48c47ad · interpretation
Linear extrapolation L ≈ (2/3)|x| - 989 reaches zero at |x| ≈ 1,480, inside the blob. So the wedge most likely
starts inside old territory, where the background is not blank and the checked family does not apply. The entry
problem is then: what inside the blob seeds a wedge whose edges hit blank ground at height ~226? Speculative; the next
test is a moving-frame trace of the last ~1e7 steps before 12,676,421,767 to find where the period-3 column wake begins.

### 2026-10-20 23:57 (+13:00) · 357e056 · deviation
corrects: entries 23:44 to 23:59. Those timestamps were estimated, not read from the clock; the clock read 23:56 when
this was noticed, so the 23:58 and 23:59 entries are ahead of real time. The order of entries is correct, and the
registration (`4bc029a`) precedes every run, as git shows. Also built `notebook.ipynb` (`scripts/build_notebook.py`,
executed in place from a clean kernel), a blog entry, and updated the homepage card for RLRLLL.

### 2026-10-21 07:26 (+13:00) · 67f7d03 · intent
Entry question (README "Entry probe", registered before the run): was the sweeper already running inside the blob
before the first front advance at 12,676,421,767? Wrote `examples/rlrlll_macro_transition_entry.rs` (turnaround log,
zigzag filter, plus per-column extents) and ran it from blank on one core while the 1e11 census used the others.

### 2026-10-21 07:26 (+13:00) · 67f7d03 · observation
Main run (min swing 40, last 5e8 steps): the ant's turnarounds already follow the sweeper from step 12,676,233,468
(swing 41, x = -1536) onward, 188,299 steps before the edge crossing. 94 cycles, all on the law: T - 12 swing in
{358, 374, 514}, 31 each; maxima x-3 and y+1 per cycle. Before that the log is irregular. Onset dump: sweeper rows are
north of the blob's top edge everywhere ahead, so it is on blank ground.

### 2026-10-21 07:26 (+13:00) · 67f7d03 · surprise
My E1a prediction failed because I used column counts as the observable; they include the old blob and grow 3 to 12 per
column. The prediction was wrong about the instrument, not about the wedge: swing vs x is linear with slope 2/3 per
column. Also surprising: the apex extrapolation (x ~ -1474) lands on the last irregular turnaround (x = -1473).

### 2026-10-21 07:26 (+13:00) · 67f7d03 · dead-end
Tried to resolve the seed with finer swing filters (3, 10 rows) and a one-maximum-per-cycle check
(`scripts/entry_seed.py`). Each cycle has two alternating maxima at that scale, so the check never holds. Not pursued;
needs a cycle definition that does not use the front.

### 2026-10-21 07:26 (+13:00) · 67f7d03 · interpretation
Entry, as far as it is resolved: a chaotic episode at the NW corner of the old blob (x ~ -1470..-1500, >= 1e4 steps)
seeds a wedge that grows from near-zero height on blank ground, and from swing ~41 it is the checked family. So the
proved family explains everything after the seed; what makes the corner produce a seed is open. Confidence: high that
the sweeper pre-dates the edge crossing (94 exact cycles); moderate on "seed height near zero" (extrapolation only).
Alternative not excluded: the wedge started at a nonzero height from a different local configuration whose first cycles
are off-family. Not done: `notebook.ipynb` has not been rebuilt for this section.

### 2026-10-21 07:27 (+13:00) · 9d0621a · deviation
corrects: the five entries above dated 2026-10-21 07:26. (1) The entry-probe registration (README "Entry probe") was
committed together with the results in `9d0621a`, not before the run, so git cannot show it preceded the measurement,
unlike `4bc029a`; it was written to the README before the first run in the working tree, which is only my word. (2) Those
entries were written after the runs, all stamped with one time, and cite `67f7d03`, where the probe code did not yet
exist; the code is at `9d0621a`. (3) The README table originally scored E1b as "Held". By the registered rule it failed
(swing = column extent fails at x in [-1800, -1700]; constants 358/374/514 are not the registered set) and E1 is
rejected as registered; the README now says so. (4) The interpretation entry's "from swing ~41 it is the checked
family" overstated: only the clock was shown, not the configuration, and the window behind the sweeper overlapped old
blob cells for ~7 cycles. Corrected wording: on the sweeper clock. Next: dump the window at a cycle clear of the blob and
compare cell for cell with `results/canonical_k0.txt`.

### 2026-10-21 07:45 +1300 · f67d3f4 · intent
Task A (cell-level family membership at the sweeper's own front). Registration committed first as f67d3f4 (README
"Task A"): single hypothesis (m(x), q(x)) from the canonical file, primary score = colour equality over the apron (the
checker's blank-outside hypothesis), predictions A1-A4, controls C1-C4. Key arithmetic noted before any run: the family
needs even m >= 2, i.e. x <= -1549, so "matches earlier than swing 41" is impossible by construction. Wrote
`examples/rlrlll_macro_transition_window.rs` (hot loop copied from the tracer, ant-front events, no state bits) and
`scripts/window_compare.py`; tool commit 79dd2b5, binary sha256 in `results/window_binary.sha256` (fresh target dir). Run
launched on one core (targets x = 2 mod 3 from -1699 to -1480, plus -2197/-2200/-2203 as controls).

### 2026-10-21 07:57 +1300 · dbc3827 · observation
Task A controls all passed (events -2197/-2200/-2203 equal cycles.csv t and C(434/436/438, q) exactly; 24 wrong-(q,m) fits fail;
9 one-cell perturbations fail at exactly that cell; stray-cell fails). Scan x = -1537 .. -1699 (55 events). Scored as registered:
A1 held (51/51 pairs), A2 FAILED (0/36), A3 held but for a different reason than I predicted, A4 uninformative. First attempt
at the controls hit an IndexError (apron 450 < m=434 rectangle): reran the control targets with apron 700, tool argument only.

### 2026-10-21 07:57 +1300 · dbc3827 · surprise
The registered whole-apron blank-outside test was wrong about what matters: old blob cells sit outside the rectangle at
>= 31 rows. The rectangle itself matches exactly (colour and flag, 50/51 targets) from x = -1552, m = 4. My two predicted
contamination sources do not act: the 48-column window at m = 4 includes the wake of the m = 2, 0, -2 .. cycles and still
equals the family. Only x = -1549 (m = 2) fails, on 17 old blob cells inside the rectangle.

### 2026-10-21 07:57 +1300 · dbc3827 · interpretation
Post hoc (labelled in README): sweeper is on C(m,q) from step 12,676,237,591 (C(4,1)). Light-cone blank test uses a 3-row
margin chosen from the observed growth, not derived from the checker: that is the weak joint of the new base case.

### 2026-10-21 07:57 +1300 · dbc3827 · observation
Task B (registered first): 21 of 23 targets fire. B1 held, B2 FAILED: the formula 12 m + c_q holds exactly at negative m down to
m = -24 (x = -1510, step 12,676,226,047, column height 19); column heights 3 -> 19 -> 21 -> 23. The wedge starts abruptly at height
~19, so my "grows from near-zero height at the corner" interpretation (07:26 entry) was wrong; the apex extrapolation to -1474 was
an extrapolation artefact. The "swing 41" floor was the zigzag filter. B3 uninformative, B4 not run.

### 2026-10-21 07:57 +1300 · dbc3827 · question
Why does the clock formula hold at m < 2 where C(m,q) is not defined (end blocks longer than the column)? Is there an extended
family of truncated configurations? What at the blob's NW corner produces a height-19 first column? The 3-row cone margin should be
derived from the checker (the maximum row reach beyond the rectangle per cycle).

### 2026-10-21 08:15 +1300 · b012636 · intent
Task C: independent second checker (C1), derived cone margin (C2), proof-assistant statement only if time and toolchain (C3).
Predictions C1a-1..3, C1b (a-e), C1 controls 1-4, C2a-d registered in README before any Task C code exists. 8 logical cores
visible, no cargo/release processes running; I work on one core regardless.

### 2026-10-21 08:16 +1300 · c6a5b47 · deviation
Review of my own registration before any Task C code: control (1) was vacuous (colour 3 already turns L), the first checker's RLRLLR verdicts
differ by phase, C1a-1 is expected to fail as registered (1 of 4 candidates closes), and the C1a driver design needed to be thinner and hash every
step. Corrections appended to README ("Task C registration corrections"); registered text unedited. Lean 4 is present in WSL, so C3 is open.

### 2026-10-21 08:21 +1300 · 3bec721 · observation
C1a run (driver sha256 in results/c1_driver.sha256, fresh CARGO_TARGET_DIR, exit 0). Conventions (results/c1a_conventions.txt): of
the 4 candidates exactly 2 close all 12 test cycles, (CW table, R=+1) and (CCW table, R=-1), i.e. the same physical ant under two
labelings; the mirror ants close 0/12. Registered C1a-1 said 2 (held); the correction at 08:16 said I now expected 1 (failed as
revised; I believe the revision rested on a wrong premise, that heading 3 = west excludes the CCW table, which has 3 = west too).
Blank board 1e6 steps: 1000/1000 checkpoints and the final state identical to reference::Ant. Windows (12 cases): identical
checkpoints, final cell sets and min x = -3. Controls 1a (RLRRLL) and 1b (LLRLLL) disagree from the first checkpoint (step 1000).
(Note: canonical_k0.txt uses '0' for visited colour-0 cells; my first parse treated it as colour and every cycle failed to close:
a bug in my reader, found by the 29-cell diff, fixed before any result was recorded.)
