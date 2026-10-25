# Lab notebook: rotation-boxes (Q25)

Append-only. Newest entry last. Tags: intent, assumption, observation, interpretation, surprise, decision, dead-end, question, deviation.
Entries were written at the end of the session from the README and git history, not as the work went on (a deviation from the conventions).

### 2026-10-25 · decae71 · intent
Test whether cyclic rotations of a rule share structure, or only a bounding box. The earlier "19 of 42" statistic conditioned on equal boxes, so it cannot measure enrichment.

### 2026-10-25 · 4723749 · observation
Unconditional comparison over the 702 survivors (rotation_boxes.py): rotation pairs share a box 52-129 times more often than composition-matched pairs at every horizon (p <= 1e-16); the 21 pairs form 12 groups in 114 classes; persistence across horizons is partial (8-13 of 21).

### 2026-10-25 · 4723749 · decision
Registered the shape test (P1: at most 2 of 21 identical at 1e8) before running. Preflight matched NR-20261022-additive-laws-order-blind on the word "cyclic"; different objective, so run. My line predicting "no candidate" was wrong and was corrected in the README.

### 2026-10-25 · b79b485 · deviation
First touched_shape oracle test failed (bounding box 8 against 7): the tracker counted the unwritten cell under the ant. Instrument fixed before any result was used; 150-case proptest then passed.

### 2026-10-25 · b79b485 · observation
At 1e8: 4 of 21 pairs identical (two classes). P1 failed narrowly, A1 also failed: mixed. Control C2 (mirror) held 21/21. Control C1 as registered (+1000 steps) was ill-posed: patterns grow too slowly.

### 2026-10-25 · 4f87172 · decision
Registered F1 (do the equal pairs stay equal at 1e9 and 1e10?) with C1 repaired to decade-spaced comparison. This extended the plan; logged as a deviation.

### 2026-10-25 · 9525e73 · observation
At 1e9 all four pairs differ (F1a failed). At 1e10 three of four differ (F1b failed as stated), but RRRRRRLLR and RRRRRRRLL have identical canonical footprints (28x28, 684 cells) again after differing at 1e9. Repaired C1 held: every rule's footprint changes each decade.

### 2026-10-25 · 9525e73 · surprise
A re-coincidence: two distinct trajectories (separated by step 45) have the same footprint at 1e8, different at 1e9, the same at 1e10. n = 1 pair; no mechanism.

### 2026-10-25 · 9525e73 · interpretation
The association is real but the "same object" reading is not supported. Possibly different rules fill toward a common envelope at different paces; untested. The background-independence test (original rule on uniform non-zero backgrounds) was not run.

### 2026-10-25 · 9525e73 · question
Is the 28x28:684 shape an envelope that rotation-class partners converge to? Test: footprints of these rules at 3e10 and 1e11, and the original rule on a uniform background k.
