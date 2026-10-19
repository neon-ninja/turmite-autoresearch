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
- **Mixed-turn loops (period 6/8) and opening-transient loops** were not tested; the square-loop failure only covers same-turn
  2x2 loops. (CI on the 8% figure done 2026-10-20: 7.6% [3.2, 12.3] at lead 6.)
  `experiments/2026-10-16-kappa-square-loop/NOTEBOOK.md` (22:58 end-of-session entry), `experiments/2026-10-16-kappa-square-loop/README.md` (Results).
- **Blob-regime boundary** of the c law between sparse-background density 0.2 and 0.7 (kept fraction 0-2% at density 1).
  `experiments/2026-10-19-c-law-mechanism/README.md` (Stage 3 results).
- **kappa/kappa_unif distribution** (median 0.81, sd ln 0.35): location and spread are two separate questions.
  `experiments/2026-10-16-kappa-uniform-baseline/results/census_spread.txt`, `notes/open-questions-hypotheses.md`.
- **Late-highway tail / unresolved rules** and why `RLRLLL` switches to a period-3 column sweeper (trigger unknown).
  `experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/`, `experiments/2026-10-13-g1-ribbon-breakout/README.md`,
  `experiments/2026-10-13-census-sweeps/results/unresolved_le10.txt`.
- **Paper figures 1-5 are built** (2026-10-20: `fig:walker-map`, `fig:edge-gap`, `fig:layers`, `fig:prefix`; late-switch earlier). Open:
  run `rebuild_figures.py` once to confirm the recipe regenerates them (not run; it overwrites all figures), and proposals 6-8 are
  unattempted. `notes/paper-figure-proposals-2026-10-16.md`, `notes/handover-paper-figures-2026-10-17.md` (stale for item 4).

## Uncommitted work in progress (do not lose)

- **n-rotor reference, `experiments/2026-10-19-kappa-walker-reference/`**: modified `README.md`, `NOTEBOOK.md`, untracked
  `notebook.ipynb`, plus `results/specs_nrotor.txt`, `scripts/analyse_nrotor.py`, `scripts/run_nrotor.sh` (status at session
  start; another session may have committed since; check `git status`).
- **`experiments/2026-10-16-kappa-uniform-baseline/NOTEBOOK.md`**: uncommitted edit.
- **Blog:** `blog/index.html` modified, `blog/_posts/2026-10-19-the-ant-below-every-walker.md` untracked.
- **rulepat, branch `claude/rule-pattern-experiment-aa95e3`**: committed 2026-10-20 as `275d52b` (23 files for
  `experiments/2026-10-14-rulepat/`). Still to decide: merge into `main` (it has no rulepat) and then remove the worktree.

## Repository housekeeping

- **Push `main`.** Local `main` is ahead of `origin/main` (merge `8f4b978`, reconciliation `5d866ce`, loop-surgery closure
  `8b290d8`, and any later commits). Not pushed; needs the user's go-ahead.
- **Jupyter is not in `pyproject.toml`**: `experiments/2026-10-16-kappa-leadrun-fresh/notebook.ipynb` was re-executed 2026-10-20
  using `uv pip install ipykernel` into the WSL venv plus `ipykernel install --sys-prefix`; decide whether to `uv add` it.
  Its Proposal 2 bounds are not in the notebook (rebuild with `scripts/build_notebook.py` if wanted).
- **Refresh `blog/index.html`** with the c-law / collider result (convention in `AGENTS.md`). The experiment pages were
  re-rendered 2026-10-20 (from a worktree under WSL set `GIT_DIR=/mnt/f/Code/turmite/.git/worktrees/<name>` and `GIT_WORK_TREE=$PWD`).
- **Stale worktrees and branches** (nothing deleted). On 2026-10-20 tracked work was committed on their own branches
  (`agent-a810708e829d8af34` as `4cb0261`, `g1-builder-structure` as `b985636`, rulepat as `275d52b`), but **gitignored data
  exists only in the worktrees and is lost on removal** (checked with `git status --ignored`): `g1-builder-structure/dumps/`
  73 MB (the only raw input for `g1_wake_extract.py`; regenerating takes hours), `rule-pattern-experiment-aa95e3/experiments/2026-10-14-rulepat/dumps/`
  192 MB and `scratch/` 3 MB (plus `target/`, rebuildable), `agent-a810708e829d8af34/experiments/2026-10-16-gpu-probe-port/scratch/` 19 MB.
  Archive what matters before any removal. `g1-builder-structure` is also the only home of `examples/builder_dump.rs` and
  `notes/g1-builder-structure-2026-10-14.md` (handover item 3). Deletion needs the user's go-ahead:
  - `.claude/worktrees/agent-a810708e829d8af34` and branch `backup/pre-census-split`: same commits; the unique `39df9f2` content is in
    `main`. Its uncommitted GPU probe port (`src/gpu/probe.rs`, `experiments/2026-10-16-gpu-probe-port/`) is an earlier attempt at what
    `main` has as `gpu_probe`; compare `experiments/2026-10-16-gpu-probe-port/results/` with `experiments/2026-10-16-gpu-probe/results/` before discarding.
  - `.claude/worktrees/g1-builder-structure`: old `data/` layout; content appears superseded by `main`. Confirm the dropped
    `TODO(G1)` paper change is already in `research-outputs/cube-root-growth-paper/main.typ`, then remove.
  - `.claude/worktrees/paper-figure-proposals` (locked) is **not** fully merged: since the 2026-10-19 merge it carries commits
    `70c8050`..`0eb4822` (housekeeping, four new paper figures with `main.typ` / `rebuild_figures.py` edits, notebook re-run). `main`
    has moved on (`cd2f20c`..`ec63e68`, including large `research-outputs/` changes), so merging this branch will need a
    `main.typ` / `rebuild_figures.py` conflict check. Do not remove it before that merge.

## Closed (for reference, do not reopen without new grounds)

- `experiments/2026-10-16-kappa-square-loop/`: H-loop dead at this scale (P2 failed).
- `experiments/2026-10-16-kappa-loop-surgery/`: closed unrun on 2026-10-19 (gate failed; premise further weakened by the collider result).
