# AGENTS.md

Guidance for AI agents (and humans) working in this repository.

## What this project is

An empirical study of generalised Langton's ants: rules are strings over
`{L, R}` (colour `c` turns by `rule[c]`, advances to `(c+1) % n`, steps
forward; classic Langton's ant is `RL`). The headline question is why the
"chaotic" rules grow as t^(1/3) in width (area ~ t^(2/3)), and what sets the
pattern-field, edge and mixing statistics around that. See `notes/` for the
live hypotheses, results and open questions.

## Environment: use WSL for toolchains

This is a Windows machine. **Rust, Python and friends live in WSL**, not on the
Windows PATH. Run anything toolchain-related through WSL:

```bash
wsl -e bash -lc "cd /mnt/f/Code/turmite && cargo test --release"
wsl -e bash -lc "cd /mnt/f/Code/turmite && uv run experiments/2026-10-13-census-sweeps/scripts/census.py"
```

- The repo is visible in WSL at `/mnt/f/Code/turmite`.
- Currently installed in WSL: cargo/rustc 1.98 (edition 2024), Python 3.12,
  `uv`, numpy, scipy, PIL. **matplotlib and pandas are not installed.**
- **Use `uv` for all Python work**, never bare `pip` or system-wide installs:
  `uv run script.py` for scripts, `uv add`/`uv pip install` into a project
  venv, `uv run --with matplotlib script.py` for one-offs. Dependencies are
  declared in `pyproject.toml` (numpy, scipy, polars, matplotlib, pillow;
  non-package project) with `uv.lock` committed. Add new ones with `uv add`
  and mention them in the commit/notes that need them. The venv lives in the
  WSL home, not on `/mnt/f`; set this in every WSL invocation:
  `export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy`
  (e.g. `wsl -e bash -lc "export UV_PROJECT_ENVIRONMENT=\$HOME/.venvs/turmite UV_LINK_MODE=copy && cd /mnt/f/Code/turmite && uv run experiments/2026-10-13-census-sweeps/scripts/census.py"`).
  Scripts use polars, not pandas.
- Use `-lc` (login shell) so `~/.cargo/bin` is on PATH.
- Machine has 6 logical cores; sweeps use `rayon`. Long jobs: run in the
  background and write to a log under the experiment's `results/`, don't block on them.
- Build artifacts go in `/target` (gitignored). Building from `/mnt/f` is
  slow-ish; if it matters, set `CARGO_TARGET_DIR=~/turmite-target`.
- Git, file editing and Windows-side tools can be used natively from
  PowerShell/Bash; just don't expect `cargo`/`python3` there.

## Reproducibility is the first requirement

A result that cannot be regenerated is not a result. Every number that lands
in `notes/` or `experiments/*/results/` must be traceable to code, inputs and a seed.

1. **Determinism.** All randomness is seeded explicitly; no `thread_rng`, no
   time-based seeds, no hash-order dependence. Parallel code (`rayon`) must give
   identical output regardless of thread count; reduce in a fixed order or
   key results by (rule, seed).
2. **Record the recipe.** Each data file has a producing script/binary and
   the exact command line. Put the command, git commit, and parameters
   (seeds, step counts `N`, windows) in a header line, a sibling `.log`/`.json`,
   or the relevant note. Never hand-edit generated data.
3. **Pin the environment.** Commit `Cargo.lock`. Do not bump dependencies
   casually; if you do, rerun the tests and say so. Python scripts declare
   their dependencies (PEP 723 header or `pyproject.toml` + `uv.lock`) and run
   under `uv`.
4. **Two implementations, one oracle.** `reference` (obvious, slow) is the
   oracle; `fast` is what sweeps use and must agree with it step-for-step.
   Any new optimisation or walker gets a property test against a reference
   version (see `tests/ant.rs`, `tests/probe.rs`). Commit `proptest-regressions/`
   files: they are failing cases worth keeping.
5. **Calibrate instruments.** Before trusting a statistic on ants, run it on a
   walker whose answer is known (rotor walker: t^(1/3) radius; random walk as
   negative control). See `notes/calibration.md`. If you change a fit, window
   or estimator, redo the calibration and report the bias.
6. **Resumable and checkpointed.** Long sweeps write `*.done` markers and
   checkpoints so a rerun skips finished work and yields the same bytes.
7. **Report faithfully.** State sample sizes, seeds, windows and
   uncertainty (between-seed sd, not just a fit's standard error). Say when a
   test failed or a step was skipped. Negative and null results go in the
   notes too.
8. **Dated notes.** Hypotheses are written down *with a timestamp before* the
   measurement that tests them (see `notes/cube-root-hypothesis.md`), with
   explicit predictions. Don't rewrite a prediction after seeing the data;
   append what happened and any revision.

## Intellectual ambition

The aim is understanding, not a pile of plots. Be bold:

- Ask *why*, and propose a mechanism that makes falsifiable, quantitative
  predictions (an exponent, a ratio, a scaling collapse), then try hard to
  break it. Prefer a sharp wrong hypothesis to a vague correct one.
- Chase the strange: rules that don't fit the census, outliers in the fits,
  unresolved cases (`experiments/2026-10-13-census-sweeps/results/unresolved_le10.txt`), anything where the story in
  `notes/open-questions-hypotheses.md` is thin. Connect to the literature
  (rotor walkers, internal DLA, Kapri & Dhar, Bou-Rabee & Peres) and try to
  sharpen it, up to a conjecture or proof sketch where one is within reach.
- Scale up when the question needs it (more rules, longer runs, bigger `N`),
  but design experiments so each one can discriminate between competing
  hypotheses. Controls and ablations beat volume.
- Be skeptical of your own results: seek confounds (finite-window bias, seed
  variance, rule-length effects, detector errors) before announcing a finding.
- When something is unexplained, say so plainly and add it to the open
  questions rather than papering over it.

## Tooling and layout

| Path | What |
|---|---|
| `src/rule.rs` | `Rule`, `Turn`: rule parsing, canonical forms |
| `src/reference.rs` | Oracle ant (and reference rotor); clarity over speed |
| `src/fast.rs` | Fast ant used by sweeps; tested against `reference` |
| `src/walkers.rs` | Calibration walkers: `Rotor`, `RandomWalk` |
| `src/detect.rs` | Highway / periodicity detection |
| `src/probe.rs` | Growth/shape probe: width, area, edge, gyration statistics |
| `src/synth.rs` | Synthetic data generation for tests |
| `src/bin/sweep.rs` | Census sweep over rules |
| `src/bin/confirm.rs` | Re-verify claimed highways / classifications |
| `src/bin/growth.rs` | Growth-exponent measurement |
| `src/bin/probe.rs` | Run the probe on a rule/seed |
| `examples/*.rs` | One-off analyses (`edgestats`, `rotorstats`, `occupancy`, `longperiod`, ...); run with `cargo run --release --example NAME -- ARGS` |
| `tests/` | Property tests (`proptest`) comparing fast vs reference |
| `experiments/YYYY-MM-slug/` | **New work goes here.** One folder per investigation: `README.md` (question, timestamped hypothesis, commands, results), `NOTEBOOK.md` (lab notebook), `notebook.ipynb` (shareable Jupyter write-up), `scripts/`, `results/` (small, committed), `experiments/2026-10-12-knockout-maps/dumps/` + `scratch/` (bulk, gitignored). Copy `experiments/_template/` to start. See `experiments/README.md` |
| `research-outputs/<name>/` | Publishable artefacts (currently `cube-root-growth-paper/`: Typst manuscript, figure/number scripts, `fig/`). Reads committed results, not `dumps/` (one known exception, see conventions) |
| `experiments/_lib/` | Shared Python helpers imported by several experiments (`analyse_probe.py`, `mixing.py`, `calib_kd.py`, `tether.py`); scripts add it with `sys.path.insert(0, "experiments/_lib")` |
| `experiments/_migration/` | One-off tooling and `map.tsv` (old path -> new path) from the 2026-10-14 restructure of the flat `data/` layout |
| `notes/` | Hypotheses, results, open questions: the scientific record |
| `web/` | In-browser simulator; `build.py` assembles it from `template.html` + `sim.js` + `page.js` |

Typical commands (all via WSL, from `/mnt/f/Code/turmite`):

```bash
cargo test --release                        # property tests vs oracle
cargo run --release --bin sweep -- ...       # census sweep
cargo run --release --example rotorstats -- ...
uv run experiments/2026-10-13-census-sweeps/scripts/census.py                        # analysis over the census CSVs
uv run web/build.py                          # rebuild the web page
node web/smoke.js                            # web smoke test (if node installed)
```

Always build and run sweeps with `--release`; debug builds are far too slow for
real step counts.

## Working conventions

- Read the relevant note in `notes/` before starting an investigation, and
  update it when you finish. Link data files and the command that made them.
- Keep Rust deterministic and dependency-light (currently only `rayon`, plus
  `proptest` for dev). Justify new crates.
- Match surrounding style; comments explain *why*, not what.
- **Isolate working material.** Each investigation lives in its own
  `experiments/YYYY-MM-slug/` folder (scripts, investigation-specific notes,
  results). Bulk output (dumps, knockout maps, scratch) goes in that folder's
  `dumps*/`, `knock*/` or `scratch/`, never in the repo root or
  `research-outputs/`. Scripts write only inside their own folder and find the
  repo root via `Path(__file__).resolve().parents[3]` (script in `experiments/<slug>/scripts/`). Rust one-offs stay in
  `examples/` (Cargo only discovers them there), prefixed with the experiment
  slug. The former flat `data/`, `calib/`, `occ/`, `snaps/`, `dumps*`, `knock*` no longer
  exist: their contents were migrated into experiment folders (old path -> new path in
  `experiments/_migration/map.tsv`; helpers in `experiments/_lib/`). Cross-cutting hypotheses and open questions stay
  in `notes/`; paper numbers are computed from `experiments/*/results/` by scripts in
  `research-outputs/`, which should not read `dumps/` (known exception: `memory_figures.py`
  reads `experiments/2026-10-13-memory-walkers/dumps/memory_paths.csv`).
- **Keep a lab notebook, and write a shareable notebook.** Every experiment has an
  append-only `NOTEBOOK.md` (dated entries tagged `intent`, `assumption`,
  `observation`, `interpretation`, `surprise`, `decision`, `dead-end`, `question`,
  `deviation`; each with timestamp and git hash; corrections are new entries, never
  edits) and a `notebook.ipynb` written for a colleague who hasn't seen the work
  (question, assumptions, provenance cell with commit and input hashes, results with
  what they do and do not show, caveats, next steps). Write entries as you go, not
  afterwards: at least one when a session starts, after every run or analysis, on
  every departure from the plan, and at the end. Notebooks read only committed
  `results/`, run top to bottom from a clean kernel under `uv` with a fixed
  seed, are committed with small outputs, and must agree with the README. Predictions
  are still registered in the README before the measurement. Full rules and the
  Jupyter setup (not yet in `pyproject.toml`): `experiments/README.md`.
- Don't commit large raw dumps: `dumps*/`, `knock*/` (at the root
  and under `experiments/*/`), `experiments/*/scratch/`, `/synth` and `/target` are gitignored for a
  reason. Summaries and the scripts that regenerate them are what belong in git.
- Commit only when asked. Never rewrite or delete existing data/results
  without checking what's there and saying so.
- Before declaring work done: tests pass in `--release`, regenerated outputs
  match what the notes claim, and the notes say how to reproduce them.
