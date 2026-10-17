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

## Using the GPU sweeper (`src/gpu/`, `gpu_sweep`)

A CUDA batch runner for thousands of independent (rule, background-seed) ants. Built and validated in
`2026-10-16-gpu-ant-sweep/` (its README has the design, benchmarks and limits). Use it when a
question needs the **end state of many `fast::run`-style ants**; do not use it for the other things below.

**When it pays**
- A census-style sweep: many rules, one step budget, output = the `sweep`/`sweep_lean` CSV (steps, stop,
  highway period/drift/onset, extent, tail period). Measured on lengths 2..=12 (4083 rules, grid 8192):
  4.9x faster than `sweep_lean` on 6 cores at 1e8 steps, 11.8x at 1e6 (the 1e6 figure is mostly lean's
  per-rule overhead). Fill the device: use at least ~4000 jobs per call and `--lanes 4096` or more; with fewer
  jobs than lanes the extra lanes sit idle.
- Seeded random backgrounds: `--bg-seeds 1..8` crosses every rule with background seeds 1..=8 (`ext::bg_colour`).

**When it does not apply (yet)**
- `growth`, `edgestats` and other windowed statistics, turn noise, several ants, turmites with states, and the
  lattices in `ext`: not ported. `probe` is ported separately, see "`gpu_probe`" below; its other walkers
  (`rotor:`, `rw:`, `sbg:`, `pw:`, `cookie:`, `errw:`, ...) are CPU-only.
- Rules with more than 128 colours; single very long runs (an ant is a serial chain: more lanes do not make one run faster).

**Setup (WSL, once)**: no sudo and no CUDA toolkit; NVRTC comes from a pip wheel and the driver is Windows'.
```bash
uv pip install --target ~/cuda-libs nvidia-cuda-nvrtc-cu12==12.9.86
source experiments/2026-10-16-gpu-ant-sweep/scripts/env.sh     # LD_LIBRARY_PATH; CARGO_TARGET_DIR=~/turmite-target
```
`env.sh` must be sourced in every WSL invocation that builds or runs with `--features cuda`
(e.g. `wsl -e bash -lc "cd /mnt/f/Code/turmite && source experiments/2026-10-16-gpu-ant-sweep/scripts/env.sh && cargo run --release --features cuda --bin gpu_sweep -- ..."`).
Device 0 is the RTX 4000 Ada here (the P1000 is device 1; the name is printed on stderr).

**From a script or shell** (same arguments and CSV as `sweep_lean`; identical bytes when no seeds are used):
```bash
cargo run --release --features cuda --bin gpu_sweep -- 2 12 100000000 8192 --lanes 4096 --stats results/gpu.json > results/census.csv
printf 'RRLLLRLLLRRR\nRL 7\n' | gpu_sweep - - 1000000     # stdin: RULE or "RULE SEED" per line
```
`--stats FILE` writes a one-line JSON (jobs, steps, wall seconds, device, lanes, grid, slice) to cite in the
experiment's README alongside the command and commit.

**From Rust** (an `examples/<slug>_*.rs` or a new binary, built with `--features cuda`):
```rust
use turmite::gpu::{GpuConfig, GpuSweeper, Job};
let mut sw = GpuSweeper::new(GpuConfig { lanes: 4096, grid: 8192, ..GpuConfig::default() })?;
let results = sw.run(&jobs, |done| eprintln!("{done}"))?;   // results[i] belongs to jobs[i], a fast::RunResult
```
Results depend only on the job, never on lanes, slice length, scheduling or the other jobs in the batch.

**Rules for experiments that use it**
- Treat it as the `fast` implementation: any claim that rests on it names the commit, the exact command, `--lanes`
  and grid, and states that output equals `sweep_lean`. For a new instrument, spot-check a slice against
  `sweep_lean` (`cmp` the CSVs) before trusting the full run.
- Sweeps are resumable at the batch level only: split a long census into chunks (one `*.done` marker per chunk
  of a few thousand rules) and concatenate in a fixed order.
- Do not run it (or a CPU benchmark) while another experiment's long job is using the machine: it takes all 6
  cores for detection and onset replays and the whole GPU, and it will distort both runs' timings (this
  happened once; see `2026-10-16-gpu-ant-sweep/NOTEBOOK.md`). Check `pgrep -af release/` first.
- It grabs all free VRAM for its tile pool by default. A second process or a second `GpuSweeper` in one
  process needs an explicit `--pool-gb` / `pool_tiles`.
- Pool exhaustion requeues jobs automatically; a job that cannot fit alone returns an error naming the rule.
  Never "fix" that by lowering the grid size: that changes escape/budget classification.
- After changing `src/gpu/` run `cargo test --release --features cuda --test gpu --test onset` (it compares with
  `fast::run` and a dense seeded-background reference). Plain `cargo test --release` does not exercise the GPU path.

### `gpu_probe`: the probe instrument on the GPU (`src/gpu/probe.*`, `2026-10-16-gpu-probe/`)

Same stdin and CSV as `probe MAX_LOG2 [GRID]`, byte-identical (`tests/gpu_probe.rs`, plus a full-scale cmp recorded in
`2026-10-16-gpu-probe/results/`), for lines `RULE` and `RULE@SEED` only; any other walker spec is rejected before
anything runs. Use it instead of `probe` when a batch is hundreds or more ants at `probe 27 8192`-sized budgets; for a
handful of runs the CPU `probe` is simpler and has no setup cost (~1-2 s NVRTC + pool allocation).
```bash
source experiments/2026-10-16-gpu-ant-sweep/scripts/env.sh
cargo run --release --features cuda --bin gpu_probe -- 27 8192 --lanes 2400 --stats results/gpu.json < specs.txt > probe.csv
```
From Rust: `turmite::gpu::{GpuProber, ProbeConfig, ProbeJob}` (`ProbeJob { rule, patch_seed: Option<u64>, max_log2 }`;
results are `probe::ProbeResult`, index-aligned with the jobs). Things to know:
- Cells are 16-bit (8 KiB tiles), so the tile pool holds half as many cells as the sweeper's. For the 2400-run kappa
  batch the bounding-box estimate is about 13 GB; check `oom_requeues` in `--stats` (a requeue restarts that ant from step 0).
- Refuses jobs whose window sums could pass 2^53 (`probe::window_sums_exact`), where the CPU's f64 sums stop being exact.
- Same etiquette as the sweeper: it takes all free VRAM by default (`--pool-gb`) and a whole GPU; do not time it
  while another job runs (`pgrep -af release/`, `nvidia-smi`).
- After changing `src/gpu/probe.*` or `src/probe.rs`: `cargo test --release --features cuda --test gpu_probe`.

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
