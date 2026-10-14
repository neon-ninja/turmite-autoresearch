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
wsl -e bash -lc "cd /mnt/f/Code/turmite && uv run data/census.py"
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
  (e.g. `wsl -e bash -lc "export UV_PROJECT_ENVIRONMENT=\$HOME/.venvs/turmite UV_LINK_MODE=copy && cd /mnt/f/Code/turmite && uv run data/census.py"`).
  Scripts use polars, not pandas.
- Use `-lc` (login shell) so `~/.cargo/bin` is on PATH.
- Machine has 6 logical cores; sweeps use `rayon`. Long jobs: run in the
  background and write to a log under `data/`, don't block on them.
- Build artifacts go in `/target` (gitignored). Building from `/mnt/f` is
  slow-ish; if it matters, set `CARGO_TARGET_DIR=~/turmite-target`.
- Git, file editing and Windows-side tools can be used natively from
  PowerShell/Bash; just don't expect `cargo`/`python3` there.

## Reproducibility is the first requirement

A result that cannot be regenerated is not a result. Every number that lands
in `notes/` or `data/` must be traceable to code, inputs and a seed.

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
  unresolved cases (`data/unresolved_le10.txt`), anything where the story in
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
| `experiments/YYYY-MM-slug/` | **New work goes here.** One folder per investigation: `README.md` (question, timestamped hypothesis, commands, results), `scripts/`, `results/` (small, committed), `dumps/` + `scratch/` (bulk, gitignored). See `experiments/README.md` |
| `research-outputs/<name>/` | Publishable artefacts (currently `cube-root-growth-paper/`: Typst manuscript, figure/number scripts, `fig/`). Reads committed results, not `dumps/` (one known exception, see conventions) |
| `data/` | **Legacy** flat results (csv/json/txt) plus the Python scripts that make them (`census.py`, `calibrate.py`, `patterns*.py`, `mixing*.py`, ...). Do not add new investigations here |
| `calib/` | Calibration run outputs and checkpoints |
| `notes/` | Hypotheses, results, open questions: the scientific record |
| `occ/`, `snaps/` | Rendered occupancy pages and snapshots (`render.py`, `page_images.py`, `portrait.py`) |
| `web/` | In-browser simulator; `build.py` assembles it from `template.html` + `sim.js` + `page.js` |

Typical commands (all via WSL, from `/mnt/f/Code/turmite`):

```bash
cargo test --release                        # property tests vs oracle
cargo run --release --bin sweep -- ...       # census sweep
cargo run --release --example rotorstats -- ...
uv run data/census.py                        # analysis over data/*.csv
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
  `dumps*/`, `knock*/` or `scratch/`, never in the repo root, `data/` or
  `research-outputs/`. Scripts write only inside their own folder and find the
  repo root via `Path(__file__).resolve().parents[2]`. Rust one-offs stay in
  `examples/` (Cargo only discovers them there), prefixed with the experiment
  slug. Legacy root-level `dumps*`, `knock*`, `data/`, `calib/` are frozen: read
  from them, don't add to them. Cross-cutting hypotheses and open questions stay
  in `notes/`; paper numbers are computed from `experiments/*/results/` (or
  legacy `data/`) by scripts in `research-outputs/`, which should not read `dumps/`
  (known exception: `memory_figures.py` reads `dumps/memory_paths.csv`).
- Don't commit large raw dumps: `dumps*/`, `knock*/` (at the root
  and under `experiments/*/`), `experiments/*/scratch/`, `/synth` and `/target` are gitignored for a
  reason. Summaries and the scripts that regenerate them are what belong in git.
- Commit only when asked. Never rewrite or delete existing data/results
  without checking what's there and saying so.
- Before declaring work done: tests pass in `--release`, regenerated outputs
  match what the notes claim, and the notes say how to reproduce them.
