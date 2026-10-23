# Do the census highways survive a configuration-level check? (a translated-cycler decider)

Registered 2026-10-23 18:12 (+13:00), base commit `6e0840b`. **Nothing in this folder has been run** when this file is first committed.
Proof track, `notes/proof-track-2026-10-20.md` Next steps item 2 ("a highway is the m-free special case"). Sibling:
`experiments/2026-10-23-sweeper-generalisation/`.

## Question

The census calls a rule a highway when `detect.rs` finds a position-periodic stretch of the ant's trace (`MIN_SPAN` repetitions) and
`confirm` finds the same period, drift and onset on two board sizes. That is evidence about the *trace*, not a statement about the
*configuration*: nothing there shows that the walls the ant lays down are reproduced each period, or that the cells it reads stay inside a
region that is reproduced. This experiment applies the parent's proof architecture at its simplest (no end blocks, no band lemma, m-free):
a **rectangle translated-cycler certificate**, checked by a program that shares no code with `detect.rs`.

**Theorem (to be checked, not assumed).** Let C0 be the true configuration at time t1 of a run from the infinite blank plane, with the ant
at `a` heading `h`. Let `P >= 1`, `v = (dx, dy)`, and let `W` be an axis-parallel rectangle. If

- (a) running P steps from C0 the ant only ever stands on cells of W (so it only reads W) and ends at `a + v` with heading `h`;
- (b) the configuration C1 after those P steps satisfies `C1(z) = C0(z - v)` for every z in `W ∩ (W + v)`;
- (c) `C0(w) = 0` for every w in `W \ (W - v)` (the leading edge is blank);
- (d) `C0(z) = 0` for every z outside W that lies in `W + kv` for some `k >= 1` (the strip ahead is blank);

then for every `k >= 0`, at time `t1 + kP` the ant is at `a + kv` with heading `h` and the configuration restricted to `W + kv` is C0 shifted by `kv`.

Proof sketch (to be rechecked by the program's logic and, if cheap, in Lean on the existing plane model): induction on k. A rectangle satisfies
`(W + jv) ∩ (W + kv) ⊆ W + (k-1)v` for `j < k` (per coordinate, intervals), so the cells of `W + (k+1)v` outside `W + kv` were never touched, hence
equal C0, hence blank by (d), and equal the shifted C0 there by (c); the cells inside the overlap are given by (b). The ant reads only `W + kv`
in cycle k by translation invariance. This is a *persistence* theorem from t1; it says nothing about entry (a run from blank reaches t1 by plain
simulation, which is the base case).

## Pruning-index preflight

- Index revision `6e0840bb05a1aa261c13ee1a0019550c0e92aac0`; `rg --files notes/pruning-index/entries` (one entry);
  `rg -n -i -e highway -e decider -e certificate -e translated notes/pruning-index/entries` matches only incidental words in
  `NR-20261022-additive-laws-order-blind` (its exclusions explicitly leave growth and highway questions open). No applicable signature.
- Decision: run. The objective differs (configuration-level persistence, not additive laws or growth).
- Budget: dumps of the 571 `confirmed_v2` highways (onset <= 1e8), each a single run to `onset + K*period` steps for K in {3, 10, 30, 100, 300};
  minutes on 6 cores. No census expansion, no new highway search. The 13 `confirmed_1B` rules (onset up to 6e8) only if the first pass succeeds.

## Inputs, instruments, controls

- Rules, periods, drifts, onsets: `experiments/2026-10-13-census-sweeps/results/confirmed_v2.csv` (`period, dx, dy, onset`). Sign convention as
  `fast::run` (`DIRS = [(0,-1),(1,0),(0,1),(-1,0)]`, heading 0 = north, y down); the program checks the sign by simulation and reports mismatches.
- Dumper: new example `examples/highway_decider_dump.rs` (plain loop, same DIRS and update order as the other tracers) writes, for a rule and a step
  count, the ant state and every non-zero cell, plus the state after P further steps computed by `reference::Ant` (an independent run from step 0 on
  the HashMap ant, so it also cross-checks the dump).
- Decider: `scripts/decide.py`, Python stdlib only, its own ant, no import from `src/`; reads the dump and `(P, dx, dy)`, takes W = bounding box of
  the cells the ant stands on in P steps, checks (a)-(d). Snapshot times `t1 = onset + K*P`; every K is recorded and the first that passes is reported.
- Oracle (AGENTS.md rule 4): the Python ant's state after P steps equals the `reference::Ant` state in the dump for every dump examined
  (position, heading, hash of all cells).
- Controls (must behave as stated before the full run is read):
  (C1) one cell of C0 inside `W ∩ (W + v)` changed -> rejected at (b);
  (C2) a non-zero cell placed ahead in the strip (d) -> rejected;
  (C3) a wrong drift (dx changed by 1) -> rejected at (a);
  (C4) the same rules at a time before the onset -> no certificate;
  (C5) non-highway rules (first 50 of `non_highway.txt`) at `t1 = 1e7`, candidate `(P, v)` taken from every `P <= 3000` at which the ant's
  displacement over P steps is non-zero -> 0 certificates.

## Predictions (registered before any run)

- **D1.** At least 90% of the 571 rules receive a certificate for some K in {3, 10, 30, 100, 300} with `(P, v)` equal to the census values
  (probability ~70%). Fails if < 90%. I expect failures among rules with large period whose one-period bounding box contains cells that are not
  shift-periodic (the active region is wider than the bounding box of the path).
- **D2.** Of the certified rules, 100% pass controls C1-C3 (each applied to a sample of 50 certified rules; a failure to reject is a bug in the decider).
- **D3.** C4 and C5: zero certificates. A certificate for a non-highway or pre-onset snapshot falsifies either the decider or the census and must be
  examined cell by cell.
- **D4 (oracle).** The Python ant and `reference::Ant` agree on all dumps examined (100%).
- **D5 (impostor check).** Every census rule that fails to certify at all five K is listed with the first failing clause. If that clause is (a) (the
  ant is not back at heading `h` on `a + v` after P steps) for any rule, the census period or drift is wrong for it and the census must be corrected.

Not predicted: which rules fail, or the mean window size.

## What each outcome changes

D1 held: the census `highway` label for those rules is upgraded from trace-periodicity to a persistence theorem from t1 (modulo the fidelity of
the Python and Rust ants, which D4 checks on the examined dumps), and proof-track item 2 gets its first decider. D1 failed: the rectangle is too
crude; report why (a non-rectangular window or a longer cycle is the next design, not a retreat). A rule failing at clause (a) or (b) at *every* K
is evidence that its census label needs another look, not that the highway is false.

## Commands

Filled in as run. Builds into a fresh `CARGO_TARGET_DIR`, exit status checked, binary sha256 logged (WSL, `/mnt/f/Code/turmite`).

## Results, uncertainty and limitations

(none yet)
