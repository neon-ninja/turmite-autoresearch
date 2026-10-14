# experiments/

One folder per investigation. Everything that is *working material* for a
question (its scripts, its notes, its small results, its bulk dumps) lives in
that folder and nowhere else, so an investigation can be read, rerun or
deleted as a unit.

```
experiments/YYYY-MM-slug/
  README.md     question, timestamped hypothesis + predictions (written BEFORE
                the measurement), exact commands, git commit, seeds, status,
                results and what they mean. Append; do not rewrite predictions.
  scripts/      analysis / job-launch scripts (Python under uv, shell)
  results/      small, committed outputs (csv/json/txt/png) + .log/.done markers
  dumps/        bulk raw output (ignored by git; see below)
  scratch/      anything throwaway (ignored by git)
```

## Rules

- **Isolation.** A script reads shared code (`src/`, built binaries) and its own
  folder; it writes only into its own folder. Bulk output goes in `dumps/` (or
  `knock*/`, `scratch/`), never in the repo root or a shared directory. Those
  paths are gitignored for every experiment (`experiments/*/dumps*/`,
  `experiments/*/knock*/`, `experiments/*/scratch/`).
- **Repo root.** Scripts locate it with `Path(__file__).resolve().parents[2]`
  and are run from the repo root, e.g.
  `uv run experiments/2026-10-foo/scripts/stats.py` (see AGENTS.md for the
  WSL wrapper).
- **Rust one-offs** stay in `examples/` (Cargo only auto-discovers examples
  there). Prefix the file with the experiment slug and name the example in the
  experiment's README. Reusable code goes in `src/`, with a property test
  against `reference` in `tests/`.
- **Reproducibility** rules in AGENTS.md apply per experiment: seeds explicit,
  recipe recorded, `.done` markers for resumable sweeps, generated data never
  hand-edited.
- **Publishing.** Numbers that appear in a paper are computed by the paper's
  own scripts under `research-outputs/<name>/` from files in
  `experiments/*/results/`; paper code should not read `dumps/`.
- **Promoting findings.** When an experiment concludes, put the summary in its
  README and link it from `notes/` (cross-cutting hypotheses and the open
  questions list stay in `notes/`).

## Legacy material

Work done before this layout lives in the flat directories `data/`, `notes/`,
`calib/`, `cell1/`, `dumps*/`, `knock*/`, `occ/`, `snaps/` (labels such as
`a1`..`w4`, `h1e11`, `t1b` are file-name prefixes in `data/`). It is left in
place because the paper's inputs and ~80 hard-coded paths depend on it. Do not
add to it: new work starts a folder here. Migrating a legacy label means moving
its `data/<label>*` files, fixing the paths in its scripts, and rerunning to
check that the output is byte-identical.
