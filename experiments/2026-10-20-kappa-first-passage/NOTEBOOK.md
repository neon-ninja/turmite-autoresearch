# Lab notebook: kappa from depth-crossing dynamics

Append-only; correct earlier entries by adding a dated entry.

### 2026-10-20 21:10 +13:00 · 44ed81f71f4bb1fdccdcdc1cbd586bc1b2eb5613 · intent

Design a prospective first-passage experiment following the discussion with the
user. No ant measurement is planned in this scaffolding session. Distinguish
budget reconstruction, independent prediction, and causal intervention.

### 2026-10-20 21:10 +13:00 · 44ed81f71f4bb1fdccdcdc1cbd586bc1b2eb5613 · observation

Read the current mechanism ledger, family hold-out and complete tail-budget
README. The latter already reports T1 failure and T2 losing to persistence;
earlier notes omit that completion. Its failed parametric calibration must not
be treated as a validated instrument for this proposal.

### 2026-10-20 21:10 +13:00 · 44ed81f71f4bb1fdccdcdc1cbd586bc1b2eb5613 · assumption

A semi-Markov approximation to depth crossings may transfer across starts and
sizes. Untested. A growing boundary, unresolved deep excursions and hidden field
history could defeat it. The constant-dwell exponent relation has been generalized
to include delta; plateau assumptions must be measured rather than imposed.

### 2026-10-20 21:10 +13:00 · 44ed81f71f4bb1fdccdcdc1cbd586bc1b2eb5613 · decision

Keep the protocol prospective until estimator calibration and cohort/resource
manifests are frozen. No copied old results and no fabricated output files.
New-family transfer excludes that family's predictor seeds as well as outcomes.

### 2026-10-20 21:18 +13:00 · 44ed81f71f4bb1fdccdcdc1cbd586bc1b2eb5613 · observation

Scaffold preflight passed under WSL/uv and the planning notebook executed from
a clean kernel with nbclient. No scientific inputs were read. The full
`cargo test --release` check is still running at this commit; its log is
`scratch/release.log`. Concurrent unrelated edits exist in the shared checkout;
only this experiment and its note pointers belong to this change.

### 2026-10-20 21:24 +13:00 · 2bf8cc97c69b9278ca6bb8be26605c515cb3f756 · observation

Full `cargo test --release` completed with exit 0 (existing warnings).
The blog renderer initially rejected missing reader-guide coverage; adding this
experiment's guide resolved it. WSL commands: `uv run --frozen --only-group blog
blog/scripts/experiments.py --baseurl /turmite-autoresearch`, then from blog/
`JEKYLL_ENV=production bundle exec jekyll build --strict_front_matter --baseurl
/turmite-autoresearch` and `bundle exec ruby scripts/check.rb _site
/turmite-autoresearch`. Build and links passed. An existing unavailable source
reference `end` in the rule-space README remains reported by the generator.

WSL has no node executable, so the desktop's bundled browser-automation runtime
was used for the local headless Edge check (`scratch/browser-check.cjs`).
Plateau, rising/falling examples, reset, mobile overflow, no-JavaScript fallback
and absence of page errors passed. Desktop/mobile screenshots in scratch/ were
visually reviewed; mobile axis type was enlarged afterwards. The interactive
curves are analytic illustrations, not measurements or calibration results.

### 2026-10-20 21:24 +13:00 · 2bf8cc97c69b9278ca6bb8be26605c515cb3f756 · decision

End scaffolding session with the scientific stages unrun. The accompanying blog
post explains the original argument and why the completed tail-budget failures
require this stronger crossing-kernel test. Next work starts at HANDOFF step 1,
not at confirmation. Cohort/model/resource registration remains outstanding.

### 2026-10-20 21:26 +13:00 · 2bf8cc97c69b9278ca6bb8be26605c515cb3f756 · observation

Implemented from-scratch BFS depth oracle, evolving depth maintenance, and milestone segment accountant in `tests/kappa_first_passage.rs` (HANDOFF steps 3-4).
Verified:
1. Exact depth matching: boundary cells (unvisited 4-neighbours) have depth 0; interior cells have shortest visited-path distance.
2. Property tests (`property_ant_budget_conservation_and_valid_milestones`):
   - B + I = S holds unconditionally for all trajectories.
   - All milestone transitions occur strictly between adjacent milestones in [0, 1, 2, 4, 8, 16, 32, 64].
   - Completed segment elapsed times plus unresolved segment time exactly equal total trajectory steps.
   - Distinguishes geometry-induced reclassification from physical contour crossings.
All 4 test targets in `tests/kappa_first_passage.rs` pass with `--release`.


### 2026-10-20 22:46 +13:00 · ba38590b33e32c8c0a7078d390cee8c11aa2ab20 · deviation

Correction to the 21:26 entry (not an edit of it). The prototype in
`tests/kappa_first_passage.rs` is not the instrument and "evolving depth
maintenance" does not exist yet: (1) `is_geometry_induced` can never be true
(an already-visited cell's depth is the same before and after insertion; a new
cell is hard-coded false); (2) a geometry jump that skips a milestone (e.g.
depth 3 -> 5) is silently dropped, leaving `current_milestone` stale and later
segments misattributed; (3) it runs a full BFS every step (quadratic). The
B+I=S and time-sum properties hold by construction, so they say little. Those
tests verify only the oracle and bookkeeping skeleton. Plan: PLAN.md; fast
depth field goes in `src/`, layered on `fixed_area` so band/sum_pa conventions
are inherited and checkable byte for byte.

### 2026-10-20 22:52 +13:00 · 5e2fa56+wip · observation

Chunks 1-3 (PLAN.md). New `src/first_passage.rs` (DepthField, Sim) and
`tests/first_passage.rs` (9 tests, all pass under `cargo test --release --test first_passage`).
- Depth monotonicity: across random ants (20 proptest cases x 1500 steps, BFS
  recomputed after EVERY step), rotor and random walks, and a serpentine fill past the
  clamp, no insertion ever lowered a depth (`decreases == 0`), so increase-only
  worklist repair is exact; matches BFS clamped at 65.
- Observer matches `fixed_area::drive` state-for-state, and band t_start/t_end,
  area, perimeter, b_dep, i_dep and sum_pa (bitwise) match `fixed_area::run`; in-band
  segment time sums exactly to band steps; episode tallies match offline
  recomputation from the recorded stream; clone-as-checkpoint reproduces output.
- Derivation (interpretation, tested only as area constancy): area cannot grow
  during an interior episode, so the territory, boundary set and depth map are frozen
  for its whole duration. The README's "frozen-depth" termination therefore coincides
  with the evolving one on this lattice; the planned >10% disagreement gate is
  vacuous (expected 0, not a measurement of scale separation). Real geometry effects
  live at boundary steps: geometry jumps (new cell, |depth change| > 1) occur
  (>0 over 20 random walks) and physical moves were never non-Lipschitz.
- Skipped-milestone policy (written before use on ant data): hit of a milestone
  >1 index from the current one records a segment with skipped=|diff|-1.
- Deviation: mid-trajectory checkpoint is an in-memory clone for now; on-disk
  checkpoint only if the pilot shows runs long enough to need it. Run-level `.done` markers still apply.
Still not run: any ant science, calibration, pilot, confirmation.
