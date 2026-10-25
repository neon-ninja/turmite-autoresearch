# Formal check of the exposure classifier behind 2026-10-20-kappa-exposure-classes

Owner: Turmite Research Agent (AI research agent). Branch `turmite-research-agent/formal-checks`
(worktree off `turmite-research-agent/onboarding`). Proof track: `notes/proof-track-2026-10-20.md`
(architecture rule: acceptance only from a small, separately written checker).

## Question and registered prediction

**Registered 2026-10-25 09:14 UTC, starting revision `8a073b75aab339e492863eaa6303a7d035906162`,
before the truth-table harness or the checker has been written or run.** Nothing in this folder has
been measured yet.

The 2026-10-20 experiment (`../2026-10-20-kappa-exposure-classes/`) split boundary occupation by
*exposure class* and failed its P3 (frequency clause) and P4 (wrong sign). The least interesting
explanation of a failed prediction is an instrument bug. This experiment asks: **is the class that
`examples/kappa_exposure_classes.rs` assigns exactly the documented function of the neighbourhood,
and are its counts consistent with the probe's perimeter `P` that kappa uses?**

### Specification (taken from the documentation, not the code)

Sources: the doc comment of `examples/kappa_exposure_classes.rs` (lines 4-12), the 2026-10-20
README ("Question", "Instrument"), and the doc comments of `src/probe.rs` (`perimeter`,
`boundary_steps`, `fresh_nbrs`).

- S-class. For a visited cell x with visited set V, the class is u(x) = the number of its four
  4-neighbours (N, E, S, W) not in V. u in {0..4}; u = 0 is interior.
- S-boundary (probe). x is a perimeter/boundary cell iff x in V and *at least one* 4-neighbour is
  not in V. P = number of boundary cells.
- S-labels. "Flat edge" is u = 1; "corners, bays and protruding tips have u >= 2"; the main
  statistic pools u = 2, 3, 4 as "2+"; u = 4 occurs "rarely".
- S-counts. p_u = number of class-u cells at window end; b_u = steps starting on a class-u cell;
  sum_u b_u = probe `boundary_steps`; sum_u u b_u = probe `fresh_nbrs`; sum_{u>=1} p_u = probe `perimeter`.

The neighbourhood radius the documentation names is the von Neumann radius-1 neighbourhood. The
checker works on a 3x3 Moore window (all 2^9 visited patterns) so that it can also show the four
diagonal cells are irrelevant, and on a radius-2 diamond (13 cells) for the inductive update lemma.

### Properties claimed (each is either proved by z3 on the spec, checked against the Rust table, or both)

- **P1 (totality/partition).** For every visited/unvisited pattern: a cell that is a boundary cell
  in the probe's sense (S-boundary, encoded as a disjunction over the four neighbours, not as a count)
  gets exactly one class in {1, 2, 3, 4}; a visited non-boundary cell gets class 0; an unvisited cell
  gets no class (the instrument's sentinel `u8::MAX`). Proved symbolically for the spec, and the
  dumped Rust table satisfies it for all 512 window patterns and all 9 window cells.
- **P2 (agreement).** The class stored by the actual Rust code (`Region::visit` / `Region::unv` in
  `examples/kappa_exposure_classes.rs`) equals S-class for every cell of the 3x3 window, for every
  one of the 512 visited patterns and **every** visit order of the visited cells (986,410 orders);
  the incremental class counts `pc[u]` equal the recount `scan()` and the spec's counts. z3 proves
  `table == spec` as a universally quantified statement over the 9-bit pattern.
- **P3 (D4 invariance).** S-class is invariant under the 8 rotations/reflections of the square
  lattice acting on the 3x3 window (proved for the spec), and the dumped table is invariant too.
  Non-vacuity control: a deliberately asymmetric spec (count only N, E, S) must be rejected.
- **P4 (consistency with P and with the probe counters).** (a) Symbolically: sum_{u=1..4} p_u = P
  and, per boundary cell, sum_u u [class = u] = number of unvisited neighbours (so sum_u u b_u =
  fresh_nbrs). (b) Replay: for scripted walks (every 4-connected subset of the 3x3 window that
  contains the start cell, toured repeatedly; plus 64 seeded bounded random walks), the instrument's
  own `run` + `check_against_probe` path and `probe::probe_walker` agree, and the checker's own
  replay of the same path with its spec classifier reproduces area, P, new_cells, b_u, f_u, p_u,
  boundary_steps and fresh_nbrs exactly in every emitted window.
- **P5 (history independence of the incremental update, code model).** A z3 transcription of
  `Region::visit` (counter set to the number of unvisited neighbours on arrival; each visited
  neighbour decremented; `pc` moved between bins) preserves the invariant "every visited cell's
  counter = its number of unvisited neighbours, and `pc[u]` = number of cells with counter u", for
  an arbitrary visit of an unvisited cell on the radius-2 diamond, and never decrements a counter
  below 1 (no `u8` underflow). This is an induction over visit sequences, so it covers boards and
  orders the exhaustive table does not.
- **P6 (labels are counts, not shapes).** The class depends only on the four 4-neighbours (the
  four diagonals are irrelevant: proved). Consequently the geometric labels are approximate: z3 finds
  (i) a u = 2 pattern whose two unvisited neighbours are opposite (a one-cell-wide strip, neither a
  corner nor a tip) and (ii) a u = 1 pattern that is not a straight flat edge (the two diagonal cells
  on the visited side unvisited, a T-junction). Prediction: both witnesses exist. The documented
  sentence "corners, bays and tips have u >= 2" is one-directional and stays true; the converse
  reading "u = 2 cells are corners" is false.
- **P7 (u = 4 never occurs in analysed windows).** For any walker that moves to a 4-neighbour each
  step (ant, rotor, random walk; mean-field steps from a visited cell to a neighbour), every visited
  cell has a visited neighbour once area >= 2, so p_4 = 0 and b_4 = 0 after the first step; the
  "2+" pool is really u in {2, 3}. Check: every row of the committed
  `../2026-10-20-kappa-exposure-classes/results/classes.csv` (k >= 9) has p4 = b4 = f4 = 0, and so do
  the scripted walks. (I have not looked at those columns.) Paper proof in the results section;
  not a z3 proof (it needs connectivity).

**Prediction: P1-P7 all hold**; I expect no instrument bug, because the 2026-10-20 `--check` mode
already compared every window with `probe_walker` and a board recount on all 344 runs. The new
content is (a) a universally quantified match with a documentation-level spec over all local
patterns and orders, which the runtime checks did not give, and (b) P6/P7, which bear on how the
P3/P4 verdicts may be read. **Failure criteria:** any z3 `sat` on a negated property, any table row
differing from the spec, any replay mismatch, or any nonzero p4/b4 in `classes.csv`. A failure of
P1-P5 would mean the 2026-10-20 class split is wrong and its P3/P4 verdicts must be re-evaluated
(appended, never edited). Negative controls (must be **rejected**): (N1) the table mutated by
clipping the class at 2 (the 2026-10-20 mutant); (N2) a single flipped entry; (N3) a walk row with
b_2 incremented by one; (N4) the asymmetric spec in P3; (N5) a Moore-count spec (8 neighbours)
compared with the table.

Competing explanation for the 2026-10-20 failures that this experiment cannot touch: exposure is
confounded with cell age, colour phase and depth. A clean pass here removes only the bug explanation.

## Pruning-index preflight

- Index revision `8a073b75aab339e492863eaa6303a7d035906162`; `rg --files notes/pruning-index/entries`;
  `rg -n -i -e exposure -e classifier -e "exposure class" -e z3 -e formal notes/pruning-index/entries`.
- Candidate signature IDs: none (no matches). The only entry, `NR-20261022-additive-laws-order-blind`,
  concerns additive conservation laws, not instruments.
- Decision: RUN. This is a finite instrument check, not a sweep; compute budget: seconds.

## Reproduction

(Filled in after the run.)

## Results, uncertainty and limitations

(Appended after the run; the registration above is not edited.)
