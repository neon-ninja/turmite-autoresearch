# Q25: do cyclic rotations of a rule share structure, or only a bounding box?

Registered 2026-10-25 (part 2 below before it is run), base `decae71`. Follow-up to `experiments/2026-10-25-prefix-causal-horizon/` (Q25 there).

## Background and the exact identity

`rot_k(w)[c] = w[(c + k) mod n]`. Colour `c'` of the rotated rule is colour `(c' + k) mod n` of `w`, so a rotated rule started on a blank board is the
original rule started on a uniform background of colour `k`. (Not a theorem about attractors; just a relabelling, checked by the oracle test below.)

## Part 1 (observational, run before this registration; `scripts/rotation_boxes.py` -> `results/rotation_boxes.txt`)

Committed census CSVs only. Among the 702 survivors at 1e11, over all 2,012 equal-length rotation (or mirror-rotation) pairs versus all other equal-length pairs,
the chance of sharing (width, height) is 52-129 times higher at every horizon 1e8..1e11 even against a composition-matched control (4 of 24,411 control pairs
at 1e11, against 21 of 2,012 rotation pairs; Fisher p <= 1e-16, but pairs inside one class are not independent). The 21 pairs sharing a box at 1e11 form 12 groups
(sizes 2-4) among 114 rotation classes with at least two survivors: about 10% of classes. The shared boxes are small (28-118 cells). Persistence is partial: of the 21 pairs, 9 share a box at 1e10, 13 at 1e9 and 8 at 1e8.
So: a real association, concentrated in a minority of classes, not a persistent identity of boxes across horizons. The earlier "19 of 42" statistic conditioned on equal boxes and is superseded by this unconditional comparison.

## Part 2 registration (before running)

**Question.** In the 12 groups, do the rotation partners have the same *shape* (the set of touched cells, up to translation and the eight symmetries of the square) after the same number of steps, or only a bounding box of the same size?

**Prediction P1 (68% confident):** at 1e8 steps at most 2 of the 21 pairs have identical touched shapes. The trajectories of rotation partners separate within 45 steps (prefix lemma), so equal shapes at a late time would require
an exact symmetry between two different dynamical systems; I expect equal boxes to be a coincidence of scale (slowly growing patterns of similar size) with only the rotation-class enrichment to explain.
**Alternative A1:** 11 or more of the 21 pairs have identical canonical shapes: the rotated rule then builds the same object on a different background (background-independence), and the box coincidence is structural.
Between 3 and 10 is reported as mixed, with the pair list. **Control C1:** each rule's shape is compared with itself under a different horizon (1e8 vs 1e8 + 1000 steps): must differ for a growing pattern, so the test can say "different".
**Control C2:** the shape hash of a rule against its own mirror image must be equal (the hash is symmetry-canonical): must always hold.
**Instrument.** `prefix_horizon::touched_shape` (canonical hash of the touched set over the 8 symmetries and translation), oracle-tested against the reference ant (count, bounding box and hash invariance under transforms of the reference set) in `tests/prefix_horizon.rs`.
**Pruning index.** Index revision `decae71`; `rg -n -i -e rotation -e "cyclic" -e background notes/pruning-index/entries`: expected no candidate (the only entry concerns additive laws); decision run.
