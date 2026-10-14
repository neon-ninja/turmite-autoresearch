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
  NOTEBOOK.md   lab notebook: dated, append-only log of intent, assumptions,
                observations, surprises, decisions and dead ends (see below)
  notebook.ipynb  shareable Jupyter write-up for colleagues (see below)
  scripts/      analysis / job-launch scripts (Python under uv, shell)
  results/      small, committed outputs (csv/json/txt/png) + .log/.done markers
  dumps/        bulk raw output (ignored by git; see below)
  scratch/      anything throwaway (ignored by git)
```

Start a new investigation by copying `experiments/_template/` (it has a
`NOTEBOOK.md` skeleton and a starter `notebook.ipynb`).

## Lab notebook (required for every experiment)

The README is the formal record: question, pre-registered predictions, commands,
results. The lab notebook is the *thinking*: what you believed, why you tried
something, what surprised you. Without it a colleague (or you in a month) can see
what was run but not why, and cannot tell a planned test from a post-hoc story.

**`NOTEBOOK.md`** — plain text, append-only, newest entry last. Keep it as you
work, not reconstructed afterwards. Each entry has a heading
`### YYYY-MM-DD HH:MM (+TZ) · <git short hash> · <tag>` and one or more of these
tags, so entries can be grepped:

| Tag | Use for |
|---|---|
| `intent` | what I am about to do and why; the decision it will inform |
| `assumption` | something I am relying on and have **not** verified; give how it could be checked. Later entries mark it `tested: held/refuted` with a link |
| `observation` | what the data show, with the file and command. Facts only |
| `interpretation` | what I think it means. Kept apart from observations, with confidence and the alternative explanations |
| `surprise` | anything that contradicted an expectation (these are the leads) |
| `decision` | a fork taken: options, choice, reason (including choices made for convenience) |
| `dead-end` | what failed or was null, so nobody repeats it; include why it failed |
| `question` | open question for a colleague or a later session |
| `deviation` | any departure from the README plan or from a registered prediction |

Rules:
- **Append-only.** Never edit or delete an old entry. A wrong entry is corrected by a
  new entry that cites it (`corrects: <timestamp>`), so the history of belief stays visible.
- **Predictions are registered in the README before the measurement**, not in the
  notebook. The notebook may contain the reasoning; the README holds the fixed
  statement. Link them.
- **Observation vs interpretation** are separate entries. Do not write "this shows"
  in an `observation`.
- **Record assumptions explicitly**, including mundane ones (cohort definitions,
  which pool, which statistic is "r_obs"). If you cannot say what would show an
  assumption false, say so.
- **Negative and null results get entries**, with the same care as positive ones.
- Every entry that reports a number names the command and file that produced it.
- Agents: write an `intent` entry at the start of a session, an entry after each
  run or analysis, a `deviation` entry whenever the plan changes, and a closing entry
  with status and next steps. Record instructions the user gave that shaped a choice.
- No secrets, no personal data.

**`notebook.ipynb`** — a Jupyter notebook written for a colleague who has not seen
the work, so the investigation can be shared and re-read without opening a terminal.
For a long investigation use `notebooks/NN-topic.ipynb` instead of one file. Required
structure, as headed markdown cells:

1. *Question and why it matters* (two or three sentences, links to the README and `notes/`).
2. *Assumptions and definitions* (copy of the live `assumption` entries, status marked).
3. *Provenance cell*: prints git commit, date, the paths and SHA-256 of every input file,
   and the seed. This is what ties the notebook to a reproducible state.
4. *Data and method*: what was run, where the heavy computation lives (scripts, binaries).
5. *Results*: tables and figures, each followed by one plain sentence on what it shows
   and one on what it does **not** show.
6. *Caveats and controls* (positive/negative controls, size and drift controls,
   noise ceiling; failed predictions stated as failed).
7. *What this changes / next steps*, plus *open questions*.

Notebook rules:
- **Reads committed inputs only**: `results/`, committed results from other experiments. Never `dumps/` or
  `scratch/`. Heavy computation stays in `scripts/` and the Rust binaries; the notebook
  loads their outputs, or imports light functions from `scripts/`. Long jobs are not
  run from a notebook.
- **Runs top to bottom from a clean kernel** under `uv`, deterministically (seed in
  the first cell), in a few minutes. The first code cell finds the repo root with
  `Path.cwd()` upward search (notebooks have no `__file__`) and fails loudly if inputs are missing.
- **Numbers must match the README and `results/`.** The notebook is not the source of
  truth for the paper; paper numbers still come from scripts under `research-outputs/`.
  If a notebook computes something new, put the producing code in `scripts/` and the
  output in `results/`.
- **Commit with outputs**, so colleagues can read it without running it. Keep outputs
  small (figures as PNG at modest dpi, no giant tables); clear stray outputs and
  hidden state before committing. Re-execute before every commit and check that
  nothing changed unexpectedly.
- **No network access, no widgets, no absolute paths, no credentials.**
- Dependencies (not in `pyproject.toml` yet): run one-off with
  `uv run --with jupyterlab --with ipykernel --with nbconvert --with nbformat jupyter lab`
  from WSL (`--no-browser`; open the printed URL on Windows). If notebooks become
  routine, add them with `uv add --group notebook jupyterlab ipykernel nbconvert nbformat`
  and say so in the commit, per the dependency rule in AGENTS.md.
- Execute and check, from the repo root:
  `uv run --with nbconvert --with ipykernel --with nbformat jupyter nbconvert --to notebook --execute --inplace experiments/YYYY-MM-slug/notebook.ipynb`
- To share outside the repo, export `jupyter nbconvert --to html` into `results/`
  (commit only if small) and mention the commit hash it was built from.

At the end of an investigation the README's results summary, the final notebook and
the closing `NOTEBOOK.md` entry must agree; if they differ, the README plus `results/`
win and the others are corrected by a new entry.

## Rules

- **Isolation.** A script reads shared code (`src/`, built binaries) and its own
  folder; it writes only into its own folder. Bulk output goes in `dumps/` (or
  `knock*/`, `scratch/`), never in the repo root or a shared directory. Those
  paths are gitignored for every experiment (`experiments/*/dumps*/`,
  `experiments/*/knock*/`, `experiments/*/scratch/`).
- **Repo root.** Scripts locate it with `Path(__file__).resolve().parents[3]` (the script sits in `experiments/<slug>/scripts/`)
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

## Migrated legacy material

Work done before this layout used flat directories (`data/`, `calib/`, `occ/`,
`snaps/`, plus gitignored `dumps*/` and `knock*/`), with experiment labels
(`a1`..`w4`, `h1e11`, `t1b`, ...) as file-name prefixes. On 2026-10-14 it was
split by label into `experiments/<date>-<label>[-topic]/{scripts,results}/`
(bulk dumps and knockout maps into the matching `dumps*/`, `knock*/`). Shared
helper modules are in `experiments/_lib/`.

- `experiments/_migration/map.tsv` maps every old top-level path to its new one
  (use it to resolve paths in historical logs, `.log` files and old notes, which
  still quote the old locations); `README.md` of each folder has the recipe.
- The tooling that did it (`plan.py`, `rewrite.py`, `fixups*.py`, `move.py`,
  `check_paths.py`) is kept there for audit; do not rerun it.
- Dates in folder names are the day the label's first file was committed, which for
  most legacy labels is the day of the initial commit, not of the experiment.
- Scripts still read some other experiment's `results/` (e.g. the census CSVs in
  `2026-10-13-census-sweeps/results/`); such reads are explicit literal paths.
- Cross-experiment dumps are shared: `2026-10-12-knockout-maps/dumps/` holds the
  RULE.grid/.traj fields that the knockout, patterns, cell1, e* and hot-tile
  experiments read.
