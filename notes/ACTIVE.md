# Active work and open items

Living index, last updated 2026-10-19 (after merging `worktree-paper-figure-proposals`, `8f4b978`, and the lead-effect
reconciliation, `5d866ce`). One line per item: what is open, where it lives, what would close it. Remove an item when it is
done or dropped; record the outcome in the experiment's NOTEBOOK and the relevant note, not here. Scientific claims belong in
`notes/open-questions-hypotheses.md`; this file only tracks what is unfinished.

## Science: open questions

- **Origin of the c law constants a = 1.4, b = 0.68** (1/c = 1.36 + 0.70 lead). Candidate: loop-formation probability x 4 steps
  per revolution / births per chain. Also why letters 5-8 matter. `experiments/2026-10-19-c-law-mechanism/README.md` (Stage 3,
  "Still open"), `experiments/2026-10-19-c-law-mechanism/NOTEBOOK.md` (19:22 closing entry).
- **S-arm kappa drop (d = -0.198) is unexplained.** A total effect, not area-conditioned; confounded with flip direction
  (L>R +0.15, R>L -0.21, post hoc). Needs direction-balanced flips. `experiments/2026-10-19-c-law-mechanism/README.md` (Stage 4
  results), `experiments/2026-10-16-kappa-leadrun-fresh/README.md` (Part B addendum, Proposal 1).
- **Does lead have any direct effect on kappa?** Current bound |b| < ~0.06 per step given ln c, one cohort, one window,
  survivor-selected. A tighter test needs a mediator-free kappa (late windows only) or a second cohort.
  `experiments/2026-10-19-c-law-mechanism/results/stage4_analysis.txt`, `notes/open-questions-hypotheses.md` (Reconciliation).
- **Arm-E dropout bias (retention 0.53)** in the single-flip mutants: bound it from existing data, no new runs
  (Proposal 2, unregistered). `experiments/2026-10-16-kappa-leadrun-fresh/README.md`, data
  `experiments/2026-10-16-kappa-leadrun-fresh/results/mutant_pairs.csv`.
- **Mixed-turn loops (period 6/8) and opening-transient loops** were not tested; the square-loop failure only covers same-turn
  2x2 loops. Also no CI on the "lead-colour laps are ~8% of loop steps" figure.
  `experiments/2026-10-16-kappa-square-loop/NOTEBOOK.md` (22:58 end-of-session entry), `experiments/2026-10-16-kappa-square-loop/README.md` (Results).
- **Blob-regime boundary** of the c law between sparse-background density 0.2 and 0.7 (kept fraction 0-2% at density 1).
  `experiments/2026-10-19-c-law-mechanism/README.md` (Stage 3 results).
- **kappa/kappa_unif distribution** (median 0.81, sd ln 0.35): location and spread are two separate questions.
  `experiments/2026-10-16-kappa-uniform-baseline/results/census_spread.txt`, `notes/open-questions-hypotheses.md`.
- **Late-highway tail / unresolved rules** and why `RLRLLL` switches to a period-3 column sweeper (trigger unknown).
  `experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/`, `experiments/2026-10-13-g1-ribbon-breakout/README.md`,
  `experiments/2026-10-13-census-sweeps/results/unresolved_le10.txt`.
- **Paper-figure proposals 1-3 and 5 not built.** `notes/paper-figure-proposals-2026-10-16.md`,
  `notes/handover-paper-figures-2026-10-17.md`.

## Uncommitted work in progress (do not lose)

- **n-rotor reference, `experiments/2026-10-19-kappa-walker-reference/`**: modified `README.md`, `NOTEBOOK.md`, untracked
  `notebook.ipynb`, plus `results/specs_nrotor.txt`, `scripts/analyse_nrotor.py`, `scripts/run_nrotor.sh` (status at session
  start; another session may have committed since; check `git status`).
- **`experiments/2026-10-16-kappa-uniform-baseline/NOTEBOOK.md`**: uncommitted edit.
- **Blog:** `blog/index.html` modified, `blog/_posts/2026-10-19-the-ant-below-every-walker.md` untracked.
- **rulepat, worktree `.claude/worktrees/rule-pattern-experiment-aa95e3`**: 23 *staged, uncommitted* files for
  `experiments/2026-10-14-rulepat/` (README, results, `examples/rulepat_dump.rs`, `examples/rulepat_scan.rs`). Exists only in
  that worktree; commit it on its branch `claude/rule-pattern-experiment-aa95e3` before removing the worktree, then decide
  whether to merge. `main` has no rulepat.

## Repository housekeeping

- **Re-execute `experiments/2026-10-16-kappa-leadrun-fresh/notebook.ipynb`** (taken from the branch at merge, not re-run here;
  build with `scripts/build_notebook.py`, execute with the command in `experiments/README.md`).
- **Stale worktrees and branches** (nothing deleted yet):
  - `.claude/worktrees/agent-a810708e829d8af34` and branch `backup/pre-census-split`: same commits; the unique `39df9f2` content is in
    `main`. Its uncommitted GPU probe port (`src/gpu/probe.rs`, `experiments/2026-10-16-gpu-probe-port/`) is an earlier attempt at what
    `main` has as `gpu_probe`; compare `experiments/2026-10-16-gpu-probe-port/results/` with `experiments/2026-10-16-gpu-probe/results/` before discarding.
  - `.claude/worktrees/g1-builder-structure`: old `data/` layout; content appears superseded by `main`. Confirm the dropped
    `TODO(G1)` paper change is already in `research-outputs/cube-root-growth-paper/main.typ`, then remove.
  - `.claude/worktrees/paper-figure-proposals` (locked) is now merged; remove after confirming nothing else is unsaved there.
- **c-law `stage4_analysis` needs its dumps.** `experiments/2026-10-19-c-law-mechanism/scripts/stage4_analyse.py` overwrites
  `results/stage4_analysis.{txt,json}` without the 4c section when `dumps/p1_probe.done` is absent (it did when re-run at merge;
  restored from git). Guard the script or note it in the README.

## Closed (for reference, do not reopen without new grounds)

- **Paper follow-up revision (2026-10-20):** c law, corrected uniform reference,
  conditioning check and negative-test statuses incorporated; PDF rebuilt and
  visually checked, release tests passed. Recipe and output hash in the paper README.

- `experiments/2026-10-16-kappa-square-loop/`: H-loop dead at this scale (P2 failed).
- `experiments/2026-10-16-kappa-loop-surgery/`: closed unrun on 2026-10-19 (gate failed; premise further weakened by the collider result).
