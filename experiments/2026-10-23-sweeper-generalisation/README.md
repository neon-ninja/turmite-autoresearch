# Are the other three no-highway escapes macro-transitions? (proof track, generalisation)

Registered 2026-10-23 18:05 (+13:00), base commit `d8c0892`. **Nothing in this folder has been run** when this file is
first committed. Parent: `experiments/2026-10-20-rlrlll-macro-transition/` (RLRLLL column sweeper, closed family
C(m, q)); plan: `notes/proof-track-2026-10-20.md`, Next steps item 2.

## Question

`RLLLLRRL`, `RLLLLRLRRL`, `RLLLRLRLLL` are the only no-highway "escapes" of the 1e11 cohort
(`experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/results/h1e11/`: stop = escaped at steps 12,601,085,717 /
39,619,909,558 / 28,427,043,780; final boxes 32750x32735, 32404x32601, 32683x32750 on the 32768 board).
**"Escaped" only means the ant touched the board edge.** Nothing in the record says any of them runs a sweeper.
The three boxes are nearly square, so a board-filling growth (area ~ t, many revisits, no repeating cycle) fits the
record as well as a sweeper does. This experiment first decides which it is, and only then (Stage 2) tries to
generalise the RLRLLL checker. A sweeper is **not assumed**.

Competing readings, all registered as outcomes:

- **SW (sweeper):** after some onset a front advances in one axis direction by a repeating cycle whose length is an exact affine
  function of the cycle's perpendicular swing, `dt = a * swing + b(q)`, `q` a phase of period <= 6. Same signature as P1 of the parent.
- **HW (missed highway):** swing constant and `dt` periodic (translation-periodic). A census correction if found.
- **FI (filler / no finite-state cycle):** the front advances irregularly; no exact clock. Includes growth with square-ish
  boxes at ~4 steps per new cell. The escape is then a board effect on an open-ended process.
- **IN (inconclusive):** instrument or budget failure. Not a dead end.

## Pruning-index preflight

- Index revision: `d8c0892db5e97d4d4f6f8de0933814a51b5a5b7c`.
- Commands: `rg --files notes/pruning-index/entries`; `rg -n -i -e RLLLLRRL -e RLLLLRLRRL -e RLLLRLRLLL -e sweeper -e escape -e column notes/pruning-index/entries` (exit 1, no output).
- Candidates: sole entry `NR-20261022-additive-laws-order-blind`. Membership (rules of length 2..8, additive-law mining) covers
  `RLLLLRRL` (length 8) for a different objective (conservation laws); nothing about growth, sweepers or escapes. It does not apply.
- Decision: **run.** Same rules, new objective (macro-transition structure). The parent experiment's RLRLLL result is a
  positive control, not a prune.
- Budget (bounded closure, per `notes/ACTIVE.md` 2026-10-23): Stage 1 = three single-core runs, each <= 1 h wall time on the
  32768 board, plus one 65536-board repeat for the first rule that survives. Stage 2 only if Stage 1 gives SW for >= 1 rule, and
  then for **one** rule at a time, capped at one working session each; stop on the first rule where the proposal cannot be
  closed by finite lemmas and record why. No sweeps, no census expansion.

## Amendment (2026-10-23 18:15 +13:00, before any run; the text above is kept as committed in `7aed7de`)

After a side note from the user that a filler fits the census equally well, Stage 1 runs all three rules on a **65536** board
(4 GiB each, so the ant passes the census escape step instead of stopping there), budget <= 1.5 h per run. G5 becomes: a
32768-board repeat of the first rule that shows SW reproduces the 65536 run's cycles up to the 32768 exit. "Window" for the
exact-clock test = the last 2e8 steps before the rule's census escape step, and separately the last 2e8 steps of the whole run.
Nothing else changes.

## Instrument

New example `examples/sweeper_generalisation_frontlog.rs` (hot loop = the plain loop of the parent's tracer). It logs, for each of
the four directions d, every step at which the global extreme in d advances: `(t, extreme, perpendicular min, perpendicular max
of the ant since the previous advance in d)`. Cycles for any period P are derived afterwards in Python by grouping consecutive
advances (cycle boundary k = first time the front reaches f0 + P k); the swing of a cycle is the union of the perpendicular
ranges of its advances. Direction-agnostic (no assumption that the front moves west) and no assumption about P or the
clock constants.

Oracle (AGENTS.md rule 4): the example's final `(t, x, y, heading, FNV-1a of every cell colour)` is compared with `fast::run`
at 1e7, 1e8 steps for each rule (`scripts/oracle.sh`), and a Python reference ant reproduces the event log for the first 2e5 steps
(`scripts/oracle_log.py`). Positive control: RLRLLL with the same tool must recover `dt - 12 swing in {358, 394, 518}`-type
three-phase exactness (parent P1) at P = 3, direction west. Negative control: the unrelated rule `RRLLLLRRL`
(log growth in the census) must give no exact clock.

## Predictions (registered before any run)

Definitions. For direction d and period P in 1..8, cycles are as above. **Exact clock on a window** = there are an integer `a`
and values `b_0..b_{p-1}` (p <= 6) with `dt_k = a swing_k + b_{k mod p}` for >= 95% of the cycles in the window and every
residual zero (not small). Window = last 2e8 steps before the ant leaves the board (the census escape point), unless noted.
The best (d, P, p) is reported; with 4 x 8 x 6 candidates the multiplicity is irrelevant because the criterion is zero residual on
hundreds of cycles, but the number of candidates tried is reported.

- **G1 (RLLLLRRL).** Exact clock on the last-2e8 window: P(true) ~ 45%. Reason: census class "builder", 3.96 steps per new cell
  and width exponent 0.49, the same signature RLRLLL has after its switch, but the box is square at the end.
  Falsified if no (d, P, p) gives an exact clock.
- **G2 (RLLLRLRLLL).** Exact clock on the last-2e8 window: ~40%; onset (first cycle after which the clock is exact to the end)
  between 1e10 and 2.5e10: ~60% given SW. The expansion of 8.5x between 1e10 and 1e11 looks like a late switch.
- **G3 (RLLLLRLRRL).** Exact clock: ~30%. Expansion 2.2x only; most likely FI or an unfinished transient.
- **G4 (filler discrimination).** For each rule, in 1e6-step windows over the last 2e8 steps, report steps per new cell and the
  aspect of the box of newly visited cells (the parent's signature S: spc < 8 and aspect >= 20). Prediction: S holds in
  >= 90% of windows for a rule iff that rule is SW. Falsified if a rule has S holding and no exact clock, or the reverse
  (then S is not a sweeper signature and the parent's G1 note must say so).
- **G5 (board independence).** For any rule with SW, repeating on the 65536 board reproduces the identical cycle sequence
  (same `dt`, `swing` for every cycle that ended before the first run's board exit). Falsified by any difference.
  (Dynamics are board-independent until the edge, so this tests the tool, not the physics.)
- **G6 (controls).** Positive control passes and negative control fails as stated above. If either does not, no verdict on the
  three rules is recorded: outcome IN.

What each outcome changes: SW for >= 1 rule -> Stage 2 (propose block family from front snapshots, generalised checker, one
rule at a time). All FI -> record as a negative result: the "no-highway escape" label hides at least these growth types, and
the proof track's census-wide decider has no sweeper to find among them; the unresolved list's remaining survivors are the next place.

## Stage 2 sketch (only if SW)

Reuse the parent's architecture, parametrised by data: stage list `S_0, L_1^n, S_1, L_2^n, ..., S_k` of finite runs and iteration
lemmas (patch `G . B^p -> r' . G` with sentinel rows), composition by list identity, 150+ brute-force cycle cross-checks,
mutation and wrong-rule controls, and a Lean certificate generator only for the lemma stages. The proposer (a script) may be
wrong; acceptance only by the checker. Direction is normalised by rotating headings, never by mirroring (a mirror changes the rule).

## Commands

Filled in as they are run (see `NOTEBOOK.md` for timestamps and hashes). Builds go to a fresh `CARGO_TARGET_DIR`,
exit status checked, binary sha256 logged. Runs are in WSL (`/mnt/f/Code/turmite`).

## Results, uncertainty and limitations

(none yet)
