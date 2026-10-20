# Is RLRLLL's column sweeper a provable macro-transition?

Registered 2026-10-20 23:40 (+13:00), base commit `ba38590`. **Nothing in this folder has been run** when this file is
first committed. This is the first experiment of the proof track (`notes/proof-track-2026-10-20.md`): it asks whether a
behaviour seen in simulation can be turned into a finite proof obligation, rather than measured more precisely.

## Question

After about 1.2677e10 steps from a blank board, `RLRLLL` switches abruptly from blob growth to a boustrophedon column
builder (`notes/g1-builder-structure-2026-10-14.md`): slow traverses at ~10 steps per row, fast returns at ~2 steps per
row, three new columns per cycle, a wake that is exactly period-3 in x with colours 3, 4, 5, and a sweep height
`L ~ 0.67|x| - 960` that grows as the builder moves west. All of that is post hoc, from two 4e5-step windows of one run.

If the sweeper is a genuine macro-transition there is a family of configurations `C(L, q)` (`L` the column height, `q` a
finite phase) and integers such that

    F^{T(L,q)} (C(L, q)) = shift_3 C(L + g(q), q'),     T(L, q) = alpha L + beta(q),

for every admissible `L`. Proving that for all `L` turns an infinite trajectory into a finite check, and would give the
exact asymptotics of the builder instead of a fitted `|x|^2 = 0.8634 t - 1.093e10`.

This experiment has two stages. Stage 1 (measurement) tests whether the macrostate exists in the simulated run at all, with
predictions below. Stage 2 (proof) is attempted only if Stage 1 passes, and is judged by an independent checker, not by
the agent that proposes the invariant.

## Definitions (frozen before measurement)

- **Run**: `RLRLLL`, blank board 32768 x 32768, ant at the centre heading north, `DIRS` and update order of
  `src/fast.rs` / `examples/g1_ribbon_breakout_builder_dump.rs`. Deterministic; no seed.
- **Front**: `x_f(t)` = the minimum x of all cells visited by time t (the builder moves west, towards negative x).
- **Cycle boundary**: the step at which `x_f` first reaches `x_f(t_on) - 3k`, k = 1, 2, ..., where `t_on` is the first
  front advance after 1.2677e10. Cycle k runs between boundaries k and k+1; `T_k` is its length in steps.
- **Height** `L_k`: the number of visited cells in column `x_f(t_on) - 3k` (the column opened at boundary k), counted
  at boundary k+1, after one full cycle. `g_k = L_{k+1} - L_k`.
- **Reach** `W_k`: max over steps of cycle k of `(x_ant - x_f)`, i.e. how far behind the front the ant goes.
- **Front window**: columns `[x_f, x_f + W)` with `W = max_k W_k + 1`, rows spanning the visited extent, at a cycle
  boundary, with y re-centred on the topmost visited row of column `x_f + 3`, plus ant offset and heading.
- **Steady phase**: all cycles that end before the ant leaves the board (exit was at step 12,974,495,927).

## Predictions (registered before any measurement in this folder)

- **P1 (exact affine clock).** There are integers `alpha`, `beta` (allowing `beta` to depend on a phase with at most 4
  values) such that `T_k = alpha L_k + beta` holds **exactly** (zero residual) for at least 95% of steady-phase cycles,
  with `alpha = 12` (10 + 2 steps per row from the G1 note). Fails if the best affine fit leaves nonzero residuals on more
  than 5% of cycles, or `alpha != 12`.
- **P2 (constant growth).** The sequence `g_k` is eventually periodic with period <= 4 and mean in [1.9, 2.1]
  (the G1 slope 0.67 per column predicts 2 rows per 3-column cycle).
- **P3 (finite reach).** `W = max_k W_k <= 24` over the steady phase: the ant never reads columns more than 24 behind the
  front, so the older wake (and the blob) is causally irrelevant once the sweeper runs.
- **P4 (self-similar front).** Every front-window snapshot at a cycle boundary decomposes as
  `top_q . B^m . bottom_q`: fixed end blocks of at most 64 rows (at most 4 distinct phases `q`), and a bulk band of `m`
  repetitions of one block `B` whose period in y is <= 12 rows, with `m` the only thing that changes between cycles.
  Fails if no such decomposition covers >= 95% of boundaries.
- **P5 (derived asymptote; consequence of P1, P2).** With `alpha = 12` and `g = 2`,
  `d|x|^2/dt -> 18 / (alpha g) = 0.75` as `L -> infinity`. The fitted 0.8634 is then a finite-size effect of the
  offset `L ~ (2/3)|x| - 960`: the local slope `6|x| / (alpha L)` should fall monotonically towards 0.75 across the
  steady phase. Fails if the per-window local slope does not decrease with |x|.

What would change the plan: if P3 fails (unbounded reach), the sweeper reads its old wake and a finite macrostate cannot
exist without carrying the wake as state; the experiment records that as the negative result the proposal anticipated.
If P1 holds but P4 fails, the clock is regular while the configuration is not, which would point to an invariant coarser
than equality of configurations (for example a regular language of column words).

## Out of scope (stated in advance)

- **Entry.** A proof of the sweeper says nothing about why a blank-board run reaches it at 1.2677e10. Whether the first
  cycles after onset already lie in the family `C(L, q)` is recorded as an observation, not predicted.
- **Other rules.** Generalising to other no-highway escapes (`RLLLLRRL`, `RLLLLRLRRL`, `RLLLRLRLLL`) is a later step.

## Stage 2 sketch (if Stage 1 passes)

A proof obligation is finite if it is stated over the decomposition of P4:
1. *Traverse lemma*: with the ant entering the bulk band `B^m` in a fixed local state, it leaves the band after exactly
   `m s` steps having rewritten each `B` into a fixed `B'`, for all `m` (an induction on `m` whose step is one finite
   simulation of the ant across one `B`, with the neighbouring blocks fixed).
2. *End lemmas*: finite simulations of the turnarounds from each `top_q` / `bottom_q` with bounded context.
3. *Closure*: the composition of 1 and 2 maps `C(m, q)` to `shift_3 C(m + g/p, q')`, closing the induction.

The checker must be a small, separately written program (or a proof assistant) that consumes the proposed blocks and
replays only the finite lemmas against `src/reference.rs` semantics. Proposals may come from anywhere; acceptance comes
only from the checker.

## Commands

Run in a cloud container (Linux, cargo 1.97, 4 cores), not the WSL machine; all single-threaded and deterministic.

```bash
E=experiments/2026-10-20-rlrlll-macro-transition
cargo build --release --example rlrlll_macro_transition_trace --example rlrlll_macro_transition_check
# Stage 1 trace (2 min 53 s): blank 32768 board, arm at the first front advance after 12,677,000,000
./target/release/examples/rlrlll_macro_transition_trace RLRLLL 12677000000 $E/scratch/run1 32768 48 25 2> $E/scratch/run1.log
cp $E/scratch/run1/cycles.csv $E/results/cycles.csv
uv run $E/scripts/stage1_cycles.py $E/results/cycles.csv > $E/results/stage1_cycles.txt
uv run $E/scripts/stage1_snapshots.py $E/scratch/run1 > $E/results/stage1_snapshots.txt
# Stage 2: propose the family from snapshots 0, 1, 2, then check it
uv run $E/scripts/extract_canonical.py $E/scratch/run1 0 $E/results/canonical_k0.txt
./target/release/examples/rlrlll_macro_transition_check RLRLLL $E/results/canonical_k0.txt   # -> results/check_k0.txt
uv run $E/scripts/stage2_compare.py > $E/results/stage2_compare.txt
```

Tracer binary sha256 in `results/trace_binary.sha256`; checker sha256 in the header of `results/check_k0.txt`.
Snapshots (`scratch/`, gitignored, ~74 MB) are regenerated by the trace command. Negative controls:
`results/check_negative_controls.txt`.

## Results (appended 2026-10-20 23:55 +13:00; predictions above not edited)

Stage 1 trace: armed at step 12,677,001,095 (front x = -2194); the ant left the board at step 12,974,495,927 at
(x, y) = (-16384, -5895), the same step and cell as the independent G1 `ribbon_trace` run. 4,729 cycles recorded.

| Prediction | Outcome | Evidence |
|---|---|---|
| P1 exact affine clock, alpha = 12 | **Held.** `T_k - 12 L_k` takes exactly three values (358, 394, 518), cycling with period 3, on 100% of 4,729 cycles; least squares slope 12.000006 | `results/stage1_cycles.txt` |
| P2 constant growth, mean in [1.9, 2.1] | **Held, sharper than predicted.** `g_k = 2` on every one of 4,728 differences (period 1) | same |
| P3 reach <= 24 | **Held.** Max reach 20 columns behind the front; by phase exactly 12, 20, 19 | same |
| P4 self-similar front | **Held on the 373 snapshotted boundaries** (cycles 0-63, 1000-1063, 3000-3063 and every 25th), not on all 4,729 (deviation: snapshots were sampled). Bulk period 1 row (every bulk row identical), 3 top blocks (one per phase), 1 bottom block, ant at dx 0, dy 6 or 7, heading 3 | `results/stage1_snapshots.txt` |
| P5 slope falls towards 0.75 | **Held.** Per-decile d\|x\|^2/dt falls monotonically 1.538 -> 0.825; the checked law below gives the limit 9/12 = 0.75 exactly | `results/stage1_cycles.txt`, `results/stage2_compare.txt` |

**Stage 2: the family is closed (checked).** `results/canonical_k0.txt` proposes C(m, q), q = 0, 1, 2, read off
snapshots 0-2. The checker (`examples/rlrlll_macro_transition_check.rs`) shows, for every even m >= 2:

    C(m, 0) -> C(m + 2, 1) in 12 m +  934 steps
    C(m, 1) -> C(m + 2, 2) in 12 m + 1058 steps
    C(m, 2) -> C(m + 2, 0) in 12 m +  898 steps

each shifted 3 columns west, with the ant never going east of the 48-column window or west of the new front. Two
shift lemmas carry the induction over the bulk: a descent `A.r^2 -> r'^2.A` (A = 2 rows, 20 steps per 2 rows) and a
return `r'^2.A -> A.r''^2` (A = 1 row, 4 steps per 2 rows), so `12 = (20 + 4) / 2` steps per bulk row. The step
counts and end states agree with brute-force `reference::Ant` runs at m = 2, 4, 6, 8, 10, 20, 50, 100, 434 for all
three phases (27 runs, all identical). Negative controls fail as they should: one bulk cell changed (col 46, 3 -> 4)
gives "segments differ" in all three phases, and the wrong rule `RLRLLR` leaves the strip.

**Consequence (base case + closure).** Snapshot 0 of the blank-board run (step 12,677,007,177) *is* C(434, 0). So,
given the correctness of that 1.27e10-step simulation, `RLRLLL` from a blank board never forms a highway: from that
step on it is the sweeper forever, cycle k + 1 takes exactly `12 (434 + 2k) + (934, 1058, 898)[k mod 3]` steps, and
`|x_f|^2 / t -> 3/4`. The law reproduces all 4,728 measured cycle lengths exactly (`results/stage2_compare.txt`).
The fitted G1 slope 0.8634 was a finite-size value of a quantity whose limit is 0.75.

What this does **not** establish, and the trust it rests on:
- *Entry* (why the blob produces the sweeper) is untouched. A post hoc probe (`results/cycles_early.csv`, armed at
  1.26e10) shows the exact law already holds from the first front advance past the old blob edge (step
  12,676,421,767, x = -1820, column height 226); extrapolating the wedge to zero height puts its apex inside the blob.
  The base case is one simulation by the plain
  loop shared with `g1_ribbon_breakout_builder_dump` (not itself proptested against `reference`; its exit step agrees
  with `ribbon_trace`, which is a consistency check, not a proof).
- The checker is ~600 lines of uncertified Rust. Every concrete step is a `reference::Ant` step, and the brute-force
  runs and negative controls test it, but a proof-assistant formalisation (or an independent second checker) is the
  next level of assurance.
- Why the wake has colour period 3 for a rule of length 6, and why the bulk row is `14435435...`, are not explained by
  the check: it certifies, it does not explain.

## Entry probe (registered 2026-10-21, before any measurement)

Question: the exact law holds from the first front advance past the old blob edge (x = -1820, L = 226). Was the
sweeper already running inside the blob (the wedge apex extrapolates to |x| ~ 1480), or does it appear at the edge?

Instrument (`examples/rlrlll_macro_transition_entry.rs`, same plain loop as the tracer): from blank, run to step
12,676,421,767 (the first front advance after 1.26e10 in `results/cycles_early.csv`, cycle 0 ends at ...424,881). Log every
*turnaround* of the ant for the last 5e8 steps, defined as a y-extremum after a swing of >= 40 rows (zigzag filter), as
(t, x, y, kind). At the end dump per-column visited extent (count, top, bottom) for every column of the board.

Predictions (E1 = sweeper already present, E2 = born at the edge):
- **E1a** Column extents are linear in x: top and bottom each with slope +-1/3 row/column (wedge edges, as measured
  at x <= -1820: L grows 2 per 3 columns) down to x >= -1600 (L ~ 140), residual <= 3 rows.
- **E1b** Ant turnarounds with x in [-1800, -1600] show spacing T = 12 L + c_q with c_q in the early-run set
  {374, 402, 530}-like (period 3), and swing amplitude equal to the column extent L(x) within 3 rows.
- **E2 (alternative)** the swing amplitude jumps (is not continuous in x) at x ~ -1820 and the turnaround log earlier
  than that is not a monotone westward sweep.
E1 is rejected if E1b fails for x in [-1800, -1700]. If E1 holds, report the westmost x where the period-3 law still fits,
and what lies there (the apex, or something that seeded it). Not predicted: what seeds the wedge.

### Entry probe results (appended 2026-10-21; the registration above is not edited)

Commands (one core, 2 min 18 s for the main run; binary sha256 in `results/entry_binary.sha256`, built in a fresh
`CARGO_TARGET_DIR`; code at the commit that adds `examples/rlrlll_macro_transition_entry.rs`):

```bash
B=~/turmite-target-rlrlll/release/examples/rlrlll_macro_transition_entry
$B RLRLLL 12676421767 12176421767 $E/scratch/entry 32768 40        # turns.csv, columns.csv
uv run $E/scripts/entry_analysis.py $E/scratch/entry   > $E/results/entry_analysis.txt
uv run $E/scripts/entry_backtrack.py $E/scratch/entry  > $E/results/entry_backtrack.txt
$B RLRLLL 12676233468 12676233000 $E/scratch/entry_onset 32768 40  # column extents at the onset (not committed)
```

| Prediction | Outcome |
|---|---|
| E1a linear column extents, slope 1/3 | **Failed as registered.** Visited-cell counts per column west of the edge grow 3 to 12 per column, because they include the old blob. (Post hoc: the observable was wrong; the wedge is visible in the ant's turnarounds, not in the column counts.) |
| E1b period-3 spacing and swing = L(x) within 3 rows | **Failed as registered (second clause), so by the registered rule E1 is rejected.** Clock clause: period 3 holds on all 94 cycles, but the constants are `T - 12 swing` = 358, 374, 514 (31 each), not the registered "{374, 402, 530}-like" and not the first run's {358, 394, 518}; the likely cause is cutting cycles at the maximum turnaround instead of at a front advance (not tested). Geometry clause: at x in [-1800, -1700] the counts are about 700 to 925 while the swings are about 170 to 215, so swing = L(x) fails. |
| E2 born at the edge | **Not supported**: the swing is continuous in x across x = -1820 (167 at x = -1720, 226 at -1815). |

Post hoc reading (not a registered outcome): E1's geometry test used the wrong observable, but the clock and the
swing-vs-x line (slope 2/3 per column, 3-column and 1-row steps per cycle) say the ant is on the *sweeper clock* well
before the edge. Whether the configuration behind it equals the closed family C(m, q) is **not** shown: no cell-level
comparison was made, and in columns -1540 to -1500 the blob reaches y = -952, overlapping the sweeper's rows
(-953 to -912), so for roughly the first 7 cycles the window behind the sweeper still holds old blob cells.
"Blank ground" is established only for the cells ahead of it.

What the run shows (observations; 1 run, deterministic, no seed):
- The ant is on the sweeper clock (not shown: the closed family itself) from t = 12,676,233,468 (swing 41, x = -1536, the swing filter's floor) to the exit.
  The first front advance (12,676,421,767, x = -1820) is just the sweeper crossing the old blob's west edge; nothing
  happens to the law there. Before t = 12,676,233,468 the turnaround log is irregular (swings 5 to 20 around
  x = -1470 .. -1500, y = -930 +- 20) for at least 1e4 steps.
- Backward extrapolation (swing 0 at x ~ -1474) agrees with the last irregular turnaround, at x = -1473 (step
  12,676,223,434). So the wedge appears to grow from a seed of height near zero at the NW corner of the old blob.
- Ahead of the sweeper: its rows lie north of the blob top edge in every column it later enters (checked on the
  onset dump: blob top row at x = -1541 is -839, at x <= -1700 is >= -399, sweeper rows <= -819), i.e. it runs on
  blank ground, consistent with the closure's hypothesis.

Caveats and a dead end: the seed itself is not resolved. Finer swing filters (3 and 10 rows) split each cycle into
several maxima (two alternating maxima per cycle, e.g. y = -912 / -939), so the one-maximum-per-cycle law check does not
apply there (`scripts/entry_seed.py`, `results/entry_seed.txt`, a failed method, kept for the record). A cycle-by-cycle
trace from the apex needs cycle boundaries defined without a front, which I have not done. So "born at zero
height" is an extrapolation, not an observation, and m below ~20 is not covered by a direct check of the family
for cycles since the apex (the proved family is C(m, q) for even m >= 2 on blank ground ahead). The wedge having started
inside a chaotic region at the blob's corner explains why the census sees a long transient but does not explain *why that
corner seeds a sweeper*.

## Task A: is the configuration behind the sweeper already C(m, q)? (registered 2026-10-21, before any code for it is written or run)

Question: the sweeper clock holds for 94 cycles before the front crosses the old blob edge. Does the *cell-level*
configuration behind the sweeper, at those cycles, equal the closed family `results/canonical_k0.txt`?

**What the family claims (read from the checker's header).** `C(m, q)`: on columns `[x_f, x_f+48)` the rows `top_q`, the bulk
row x m, `bottom_q`; **every other cell in columns west of `x_f+48` has colour 0** (above, below, and ahead of the front);
the ant stands at `(x_f + dx, top_row + dy)` heading `h` on the still-unvisited front column. My membership test is
exactly that, evaluated on a dump apron (columns `[x_f-320, x_f+64)`, rows `ant_y +- 450`), because a rectangle-only
match with a stray blob cell one row outside would not license the closure.

**Scoring conventions (fixed now).** Primary score: *colour equality with unvisited = colour 0* (the dynamics see colour only)
over the apron, plus the ant offset and heading. Secondary: equality of the visited flag as well. The one hypothesis scored is
the single `(m(x), q(x))` below; any search over other `m`, `q` is post hoc diagnostics.

**Boundaries and the family's range (arithmetic, not a measurement).** Use the tracer's event: the ant's `x` first goes below
its minimum since `T_track` = 12,676,233,000, at a column x = 2 (mod 3) (the residue of `canonical_k0.txt`; snapshot 0 is at
x = -2197). The canonical file gives, for x = 2 (mod 3):

    j = (-2197 - x)/3,   m(x) = 434 + 2 j   (m(-2197) = 434, m(-2200) = 436, m(-2203) = 438),   q(x) = j mod 3.

The family needs even `m >= 2`, so it **cannot** apply east of x = -1549 (m = 2); "matches earlier than swing 41" is impossible by
construction (swing 41 at x = -1536 is m ~ -7). The question is the first x <= -1549 where it holds. Two contamination sources,
each with its own threshold, both about columns of the 48-wide window:
(i) old blob cells in columns about -1540 .. -1500 (blob reaches y = -952, overlapping the sweeper's rows);
(ii) the wake of the off-family cycles with m < 2, in columns about -1546 .. -1474.
The full window clears both only if `x + 47 < -1546`, i.e. x <= -1594 (x = 2 mod 3).

**Controls (must behave as stated before the scan is looked at).**
- C1. Events at x = -2197, -2200, -2203 occur at `t` equal to rows 0, 1, 2 of `results/cycles.csv` (12677007177, 12677013319,
  12677019609), and their dumps equal `C(434, 0)`, `C(436, 1)`, `C(438, 2)` on the full membership test.
- C2. The same dumps compared with the wrong q (two alternatives) and with m +- 2 all fail.
- C3. A one-cell perturbation (one in each of top, bulk, bottom) fails, with first differing cell exactly the perturbed one.
- C4. Hot loop, DIRS and update order byte-identical to the tracer's; the tool adds no state bits to `cells`.

**Predictions for the scan** (targets: every x = 2 (mod 3) from -1699 to -1480; whichever fire):
- **A1 (clock).** For consecutive events (x, x-3): `dt = 12 m(x) + c_q(x)`, c = (934, 1058, 898), holds for every pair with
  x <= -1552 (both ends m >= 2). Not predicted east of that; report the first failure going east.
- **A2 (family).** Every target x <= -1594 (about 37 events) passes the primary test. Confidence ~55%: the sweeper clock held
  but the cell-level claim was never checked, and an off-family transient could leave a different wake. Fails if any of them fails.
- **A3 (contamination).** For targets -1591 <= x <= -1552 the primary test fails, and every differing cell is either
  visited before `T_track` (old blob) or in a column >= -1546. Fails if a differing cell lies elsewhere (that would be a third
  source I did not name) or if some of these targets pass (then the contamination is smaller than I thought; report how far).
- **A4 (exploratory, no pass/fail).** Report the first x going east at which a 21-column window (the checker's reach is
  <= 20) still passes the primary test.
Per-event output: first differing cell as canonical (row, col), absolute (x, y), expected vs actual value, and whether the
cell was visited before `T_track`.

Instrument: `examples/rlrlll_macro_transition_window.rs` (to be written next), built into a fresh `CARGO_TARGET_DIR`,
sha256 logged. Comparison: `scripts/window_compare.py`. Task B (the seed) is registered separately after A is scored.

### Task A results (appended 2026-10-21 07:55 +13:00; the registration above is not edited)

Commands (one core; window run 2 min 36 s; binaries in `results/window_binary.sha256` (scan, 901-row apron) and
`results/window_binary_apron.sha256` (control run, 1401-row apron, otherwise the same loop); fresh target dirs; code at
`79dd2b5` and the APRON-argument commit):

```bash
B=~/turmite-target-rlrlll-w/release/examples/rlrlll_macro_transition_window
$B RLRLLL 12676233000 $E/scratch/win 32768 $(seq -1699 3 -1480 | tr '\n' ','),-2197,-2200,-2203      # scan; 55 events
$B2 RLRLLL 12676233000 $E/scratch/win_ctrl 32768 -2197,-2200,-2203 700                                 # controls
uv run $E/scripts/window_compare.py $E/scratch/win_ctrl controls > $E/results/window_controls.txt
uv run $E/scripts/window_compare.py $E/scratch/win scan > $E/results/window_scan.txt
uv run $E/scripts/window_cone.py $E/scratch/win > $E/results/window_cone.txt                           # POST HOC
```

The 19 targets east of -1537 were skipped (the ant was already at x = -1534 at `T_track`; the first event is -1537).

**Controls (scored before the scan was read): all pass.** C1: events at -2197, -2200, -2203 have `t` equal to rows 0-2 of
`results/cycles.csv` and equal C(434,0), C(436,1), C(438,2) on colour, visited flag, heading and the whole apron
(n_in = n_out = 0). C2: 8 wrong (q, m) per event all fail. C3: a one-cell change in top, bulk and bottom fails at exactly that
cell (9 of 9). C3b: a stray cell outside the rectangle fails the blank-outside clause. (First attempt of the control script
failed with an IndexError because the 901-row apron is shorter than the m = 434 rectangle; fixed by rerunning the three control
targets with apron 700, a tool parameter, not by changing the comparison. Logged as a deviation.)

| Prediction | Outcome as registered |
|---|---|
| A1 clock `dt = 12 m + c_q` for pairs x <= -1552 | **Held**: 51 of 51 pairs. East of that the formula still holds for the pairs at x = -1540, -1543, -1546, -1549 (m = -4 .. 2) and fails first (going east) at x = -1537 (m = -6): dt 678 vs 826. |
| A2 all 36 targets x <= -1594 pass the primary test | **Failed as registered: 0 of 36 pass.** Every failure is the blank-outside clause: old blob cells (visited before `T_track`) lie inside the 384 x 901 apron outside the rectangle (all 100% of the differing cells at every target are "old"). Inside the rectangle there are no differences at any target x <= -1552. |
| A3 for -1591 <= x <= -1552 the primary test fails and every differing cell is old blob or column >= -1546 | **Held as registered** (14 of 14 fail; all differing cells old), but for a different reason than I named: those are blob cells outside the rectangle, not contamination of the window. Registered thresholds (i), (ii) were wrong about where the window is clean. |
| A4 21-column window, exploratory | Uninformative under the registered (whole-apron) test: 2 of 51 pass. |

**Post hoc reading (not a registered outcome; `scripts/window_cone.py`, `results/window_cone.txt`).** Restricting the test to the
object the family is actually about, the rectangle `top_q . bulk^m . bottom_q` on the 48 columns, plus the ant offset and
heading, plus no nonzero cell in a light cone (rectangle widened by 3 rows behind the front and by 3 + s//3 rows s columns
ahead of it, out to 320 columns west): **50 of 51 targets from x = -1699 to -1552 match exactly (colour and visited flag,
zero cone violations), and the single failure is x = -1549 (m = 2)**: 17 cells inside the rectangle differ from C(2, 0), first
at canonical (row 24, col 46), absolute (-1503, -932), expected 5, got 0, a cell visited before `T_track`. The nearest nonzero
cell outside the rectangle behind the front is 31 to 278 rows from it. So:

- The sweeper is on C(m, q) at the cell level from **x = -1552, m = 4, q = 1, step 12,676,237,591**, 769,586 steps before
  the original base case (C(434, 0) at 12,677,007,177) and 184,176 steps before the front crosses the old blob edge. That
  is 89 cycles before the edge crossing.
- Neither the 48-column window nor the off-family wake of the m < 2 cycles contaminates it, contrary to my registered
  expectation (ii): the window at m = 4 spans columns -1552 .. -1505, which contain the wake of cycles with m = 2, 0, -2, ...
  and still equals the family. At m = 2 the failure is old blob cells inside the rectangle, i.e. source (i).
- "Earlier than swing 41" is not meaningful (the family needs m >= 2, x <= -1549); the cell-level match begins one cycle after
  that bound.

What this does and does not earn: with C(4, 1) as the base case, the closure gives the law from there to the exit
(the induction is unchanged), and the base case now rests on a 1.26768e10-step simulation that is the same plain loop
(not a longer one). The ant never reading the old blob cells outside the cone is a consequence of closure plus the cone
margin; the margin (3 rows) is chosen from the observed growth, **not derived from the checker**, so "blank ground" is
verified for a 3-row margin and 320 columns, and beyond column -1872 by the fact that the global front was at -1820.

## Task B: the seed, from the apex to the family (registered 2026-10-21, before the run)

Task A fixed where the family starts at the cell level (m = 4, x = -1552) and that the clock formula with the extended
`m(x)` still holds at x = -1540 .. -1549 (m = -4 .. 2) but fails at -1537 (m = -6). Cycle boundaries stay front-free in the
sense of Task A (the ant's own minimum x since a tracking time), now with `T_track2` = 12,676,223,500, just after the last
irregular turnaround logged at x = -1473 (step 12,676,223,434), with targets x = 2 (mod 3) from -1471 to -1537.
Same tool, same binary as the scan (`results/window_binary.sha256`), default apron.

Predictions:
- **B1.** Events fire for most targets in [-1537, -1471] in strictly decreasing x with increasing t (the ant's own minimum is
  monotone by construction; the prediction is that few targets are skipped: <= 5 of 23). Fails if > 5 skipped, which would
  mean the ant retreats and re-advances in this zone (not a sweep).
- **B2.** The formula `dt = 12 m(x) + c_q(x)` fails for every pair with m <= -6 (x >= -1537): the law is not a continuation
  of the family below the point where the end blocks of the configuration overlap. Fails if it holds for any such pair.
- **B3 (what fits instead).** For those pairs, `dt - 12 * swing` (swing = y-extent of the ant's excursions in the cycle, taken
  from the same dumps as the extent of nonzero cells in the front column's rows) is NOT constant: its sd over the pairs with
  x >= -1537 is more than 10 steps. Exploratory; no pass/fail beyond this.
- **B4.** At no target x in [-1537, -1471] does the window equal a rectangle `top . bulk^m . bottom` of the family (m is
  undefined there); only reported as the first differing cell vs the m = 0 construction `top_q . bottom_q`, no score.
Not predicted: the seed itself. If B1 or B2 fails the interpretation "chaotic corner, then a clean family from m ~ -4" must be
revised and I will say so.

### Task B results (appended 2026-10-21 07:57 +1300; registration above not edited)

Command: `$B RLRLLL 12676223500 $E/scratch/winB 32768 $(cat scratch/targetsB.txt)` (targets -1537 .. -1471 step 3; same binary as the
scan), then `uv run $E/scripts/window_seed.py $E/scratch/winB $E/scratch/targetsB.txt > $E/results/window_seed.txt`. One run, 21 events.

| Prediction | Outcome as registered |
|---|---|
| B1 <= 5 of 23 targets skipped, monotone | **Held**: 2 skipped (-1471, -1474; the ant was already at -1475), 21 fired, t strictly increasing. |
| B2 formula fails for every pair with m <= -6 | **Failed**: the formula `12 m + c_q`, evaluated at negative m, holds exactly on 9 of 20 pairs with m <= -6, from x = -1534 down to x = -1510 (m = -24; step 12,676,226,047), and fails for every pair east of that (x = -1507 .. -1495 irregular: dt 842, 82, 10, 22, 30, 162, ...). |
| B3 sd of dt - 12 L > 10 | Numerically true (sd 264) but uninformative: the pooled pairs include the nine exact ones, whose dt - 12 L takes the three values 382, 418, 542. Not a test of anything. |
| B4 | Not run (no comparison construction was written). |

Heights of cells first visited after `T_track2` in the column 3 east of each event: 3 (col -1504), 19 (-1507), 21 (-1510), 23 (-1513),
..., +2 per 3 columns thereafter. **The wedge does not grow from zero height**: a column of height ~19 appears right after a
column of height 3, and from there every cycle obeys the family's clock exactly, 14 cycles (m = -24 .. 2) before the cell-level
family applies. The backward extrapolation to an apex at x ~ -1474 (README Entry probe) was wrong, and the "swing 41" floor
was my zigzag filter, not the sweeper. The clock law's validity at negative m (columns shorter than the family's own end
blocks) is unexplained: C(m, q) is undefined there, so the exact formula is an observation about the trajectory, not covered by the
closure. What at the NW blob corner makes a height-19 first column: open.

Commit of this section: after `dbc3827`. Notebook `notebook.ipynb` rebuilt (Task A/B cells, executed clean from the built script).

## Task C: assurance. An independent second checker, and a derived cone margin (registered 2026-10-21 08:15 +1300, before any Task C code is written or run)

Aim: reduce the trust placed in `examples/rlrlll_macro_transition_check.rs` and close the weak joint of Task A (the
3-row cone margin). Nothing below is marked "proved" on my say-so: I report agreement and disagreement. If the two checkers
disagree anywhere, the smallest disagreeing case is settled by brute force before anything else is said.

**Independence rules for the second checker** (`scripts/c1_*.py`, Python 3 stdlib only, run under `uv`): no import from
`src/` (no `turmite` crate, `reference.rs`, `fast.rs`) and none of the first checker's code or data structures (`Sym`, `Seg`,
`Count`, `normalise`, `run_patch`). It reads only `results/canonical_k0.txt` (the proposal, as text) and the rule string. The
only place `reference::Ant` is used is the *comparison driver* for C1a (a small Rust example
`examples/rlrlll_c1_reference_hash.rs` that prints reference-ant hashes and traces), kept separate from the second checker.
Its proof architecture is my own and differs from the first checker's peel-and-regroup engine: a fixed pipeline
`S1 (top block -> descent frame) . descent lemma^n . S3 (bottom turnaround) . return lemma^n . S5 (top block -> new front)`,
each stage a finite run on an explicit patch, composed by explicit row-count bookkeeping. Stage boundaries are found by a
separate, untrusted `c1_derive.py` from a brute-force exemplar trajectory; the verifier consumes its JSON output as data.

**Conventions to re-derive, not copy.** Heading/turn conventions come from `canonical_k0.txt` (ant heading 3 = west, the front
column is the unvisited one on the west) and a brute-force closure test: of the four candidates (heading table clockwise or
counter-clockwise from heading 0 = north, R = +1 or -1), those that make C(m, q) -> C(m+2, q+1) are accepted.

### Predictions

- **C1a-1 (conventions).** Exactly 2 of the 4 candidates close the cycle for RLRLLL on the canonical file, and they are the
  same dynamics up to heading labels; I then use the clockwise table, heading 0 = north, R = +1.
  Fails if 0, 1, 3 or 4 of the candidates close, or if the two closing ones are not label-equivalent.
- **C1a-2 (semantics, blank board).** Over the first 1e6 steps from a blank board (heading 0 = north), my ant and
  `reference::Ant` agree at all 1,000 checkpoints (hash of (x, y, heading, colour read) accumulated every 1,000 steps) and in
  the full final state (position, heading, step count, number and hash of non-zero cells). Fails on any single disagreement.
- **C1a-3 (semantics, windows).** For m in {2, 4, 10, 50} and q = 0, 1, 2 (12 cases), one full cycle from `canonical_k0.txt`
  (until the minimum x first reaches x_f - 3): the position-sequence hash, the step count and the final window agree with
  `reference::Ant` in all 12. Fails on any disagreement.
- **C1b (finite lemmas). Held means all of the following.**
  (a) a descent lemma exists for each q with p = 2 rows per application and s = 20 steps; (b) a return lemma exists for each q
  with p = 2 and s = 4 (the frame height H is a property of where I cut the frame and **may differ** from the first checker's
  2 and 1; I do not predict it); (c) the three end-case finite runs (S1, S3, S5) exist with no sentinel read, no cell east of
  the 52-column strip touched and no premature front advance, for all three q; (d) the composed cycle length from brute force
  at m = 2, 4, ..., 100 (50 values x 3 phases = 150 runs) is exactly `12 m + (934, 1058, 898)[q]` and the end state is exactly
  C(m + 2, q + 1) (window and ant); (e) the symbolic lower bound my pipeline needs, `m0`, is <= 4 (probability ~55%; where
  `m0 > 2`, the cases `2 <= m < m0` are covered only by the brute-force runs up to 100 and I say so). A single mismatch in (d),
  or any of (a)-(c) not existing, is a fail.
- **C1c (induction, in my own words).** Written as a theorem in this README after the code runs, with an explicit list of the
  hypotheses checked by code and those argued on paper. Anything on paper only is listed as an unchecked step. No numerical
  prediction; the deliverable is the list.
- **C1 controls (each must behave as stated).**
  (1) an ant with one wrong turn (colour 3 turns L instead of R) disagrees with `reference::Ant` at C1a-2 (first checkpoint
  reported);
  (2) rule RLRLLR fails C1b where the first checker fails it (the ant leaves the strip: "left the strip");
  (3) one cell of the bulk row changed (column 46, 3 -> 4, as in `results/check_negative_controls.txt`) fails C1b in all three
  phases at the composition or stage-existence level, not by a crash;
  (4) both files the first checker rejected (mutated bulk cell; wrong rule) are rejected by mine too with the same verdict class
  ("result is not C(m', q+1)" in each phase; "left the strip" for RLRLLR).
  A control that does not behave as stated is scored as a fail of the control, and the verifier is not trusted until it is
  explained.
- **C2 (derive the cone margin).** Quantity: for each cycle of C(m, q), m = 2..100 step 2, q = 0, 1, 2 (150 cycles), the max
  number of rows **visited** (ant position at some step; this counts a cell that returns to colour 0 after 6 visits) above the
  initial top row and below the initial bottom row of the rectangle, and the min column visited.
  **C2a** rows above <= 1 and rows below <= 1 in all 150 cycles (probability ~45%: the end blocks are diagonal and I have not
  looked at them from this angle).
  **C2b** column x_f - 3 is reached only on the cycle's final step and nothing further west is visited (largely definitional;
  a failure would mean a bug in my cycle cut).
  **C2c (symbolic, all m)** the reach beyond the rectangle is attained inside S1, S3 and S5, which are finite runs independent of
  m, while the lemma runs only touch rows of their own patch; so the maximum over all m equals the maximum over those three runs
  per phase, and I predict it equals the brute-force maximum of C2a.
  **C2d (recheck of Task A)** simulate the ideal family forward 110 cycles from C(m, q) at each of Task A's 50 matching windows
  (x = -1552 .. -1699) with my ant, take every cell the ant ever visits, and require the real base configuration
  (`scratch/win` dumps, regenerated if absent) to have colour 0 at every such cell outside the base rectangle, and to equal the
  family inside it: all 50 windows pass (probability ~75%). If the derived margin is > 3 and some window has a non-zero cell
  inside it, Task A's base case is invalid and I say so first.
- **C3 (optional)**: only if C1 and C2 are done. First check whether Lean 4 / Coq is installed in WSL (`lean`, `lake`, `coqc`
  on PATH); if not, the result is "toolchain absent" and stop. No install.

### Task C registration corrections (appended 2026-10-21 08:16 +1300, commit after `c6a5b47`; the registered text above is not edited, and still nothing of Task C has been run; note the registration header says 08:15 but `date` read 08:14 at that point, a clock-reading slip, not a reordering)

1. **Control (1) was vacuous as written.** In RLRLLL colour 3 already turns L (0:R 1:L 2:R 3:L 4:L 5:L), so "colour 3 turns L instead of R"
   is the correct ant. Re-registered: the broken ant flips colour 3 from L to R (rule `RLRRLL`) and, as a second broken ant, colour 0 from R to L
   (`LLRLLL`). Each must disagree with `reference::Ant` (on `RLRLLL`) at C1a-2; the first-disagreement checkpoint is reported.
2. **Controls (2)/(4): the first checker's RLRLLR verdict differs by phase** (`results/check_negative_controls.txt`): phases 0 and 1 give
   "left the strip at column 52"; phase 2 gives "result is not C(m', 0): segment counts differ: 50 vs 47". Expected from my checker: for RLRLLR
   the *class* of verdict per phase is {leaves the strip / does not reach C(m+2, q+1)}, and I do not require the same text, because the
   stage structure differs. "Same place" is defined concretely: the first step at which a brute-force run of the mutated input deviates from
   the unmutated run, and the stage of my pipeline that rejects it; I report both next to the first checker's verdict.
   The mutated-bulk input file was never committed: I regenerate it (bulk row string index 46 is '3', so "3 -> 4"; the WEST-padded index would be
   '4'), run the first checker on it, confirm it reproduces the committed output lines, commit it as `results/canonical_k0_mut46.txt`, and feed
   identical bytes to both checkers.
3. **C1a-1 will probably fail as registered, and I will score it as written.** The canonical file fixes heading 3 = west, which excludes the
   counter-clockwise tables; of the clockwise pair, R = -1 is the mirror ant. My expectation (written now, before the run) is that 1 of the 4
   candidates closes, not 2. That would be a missed registered prediction, not a disagreement between checkers, and I will not relabel it.
4. **C1a driver is thin.** Python builds every initial state (blank and each C(m, q)) as plain `x y colour` lines plus position and heading; the Rust
   driver only loads them, steps `reference::Ant` N times (N from Python's cycle length), folds *every* step into a 64-bit hash of
   (x, y, heading, colour read) with explicit two's-complement encoding and prints the accumulator at each 1,000-step checkpoint. It does not parse
   `canonical_k0.txt` and does not decide when a cycle stops. A mismatch is first diffed step by step before it is called a semantic disagreement.
   Build in a fresh `CARGO_TARGET_DIR`, exit status checked, sha256 logged.
5. **Verifier obligations I add to the pipeline** (checked by code, each with a failing-input test where cheap): a sentinel on every open side of
   every finite run (above A in the descent as well), reading a sentinel fails, arriving on the far sentinel at the final step is allowed; at each
   handoff the end state equals the next stage's precondition exactly, and rows a stage does not own are verified untouched; counts are integers
   >= 0 for all even m >= m0 and the total equals 12 m + c_q as a polynomial identity, not only at sampled m; the post-condition includes
   "colour 0 outside the rectangle in every strip column west of the new x_f + 48", enforced on purpose.
6. **C2 emphasis.** The base case that matters is C(4, 1) at x = -1552; the other 49 windows are extra. 110 cycles suffice because reach is at most
   20 columns behind the front (first checker's P3, to be re-derived by mine): once x_f(k) + 20 < -1820 every later read is in a column never visited
   by base time, which happens for k >= 97. The dump apron reaches only 320 columns west (to x = -1872) but 110 cycles reach about 333, so I verify
   from the dump that columns -1820 .. -1872 are unvisited, and for anything further west rely on the global-front fact, stated as such. I report the
   envelope of the union of visited cells against Task A's `3 + s//3` cone; if the union pokes outside that cone anywhere, Task A's margin was not a
   sufficient certificate even if C2d passes, and the headline says so.
7. **C3**: `which lean lake` finds `~/.elan/bin/lean` and `lake` in WSL, so C3 is not blocked by the toolchain. Version recorded when used.
