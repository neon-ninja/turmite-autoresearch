# Manuscript build

## Chirality baseline comparison (2026-10-15)

Added a training-only comparison on the original 300 calibration and 2,747
held-out rules. Three-letter group means achieve R²=0.807648, compared with
0.585589 for one letter and 0.660023 for chirality. Chirality reduces SSE
by 17.96% against one-letter grouping but has 76.75% more SSE than the
three-letter lookup. Within broad L/R groups its R² is 0.189960/0.163770;
after centering within three-letter groups, pooled residual R² is about zero.
Paired rule-bootstrap intervals condition on the fitted models and do not
measure initial-seed or training uncertainty. The critique's quoted R²=0.88
does not match the existing files (R²=0.660, correlation 0.869).

The abstract, key results, comparison table, figure caption, evidence table
and a provenance footnote now state this limitation. New paper numbers are extracted by
`chirality_baseline_numbers.py` from
`experiments/2026-10-15-chirality-baselines/results/`, without editing legacy
results. Full registration, commands and caveats are in that experiment's
README and `notes/chirality-baselines-2026-10-15.md`.

Rebuild from WSL with the standard environment exports:

```bash
uv run experiments/2026-10-15-chirality-baselines/scripts/compare.py
uv run research-outputs/cube-root-growth-paper/chirality_baseline_numbers.py
typst compile --root . research-outputs/cube-root-growth-paper/main.typ research-outputs/cube-root-growth-paper/main.pdf
uv run --with pymupdf experiments/2026-10-15-chirality-baselines/scripts/verify_artifacts.py
```

Release tests and three synthetic statistical controls pass; the shareable
notebook executed cleanly; analysis outputs reproduce byte-for-byte. The
38-page PDF compiled without diagnostics and pages 1, 2, 20 and 21 were
rendered and reviewed. Hashes are in the experiment's `results/verification.json`.
No project dependencies changed and no commit was made. Existing working-tree
changes were retained.

Commit scope: only this comparison is staged from the shared manuscript.
The committed source uses a local provenance footnote because the expanded
audit appendix belongs to other uncommitted work. A matching PDF is compiled
from the staged source; the shared working PDF and unrelated edits remain.

From the repo root (WSL; see AGENTS.md for the env):

    export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
    uv run research-outputs/cube-root-growth-paper/compute_numbers.py   # data/* -> research-outputs/cube-root-growth-paper/numbers.json (bootstrap seed 20261012)
    uv run research-outputs/cube-root-growth-paper/figures.py           # -> research-outputs/cube-root-growth-paper/fig/*.pdf
    typst compile --root . research-outputs/cube-root-growth-paper/main.typ research-outputs/cube-root-growth-paper/main.pdf

Typst is installed via `cargo install --locked typst-cli`. Numbers in main.typ are generated, with the exceptions below;
inputs are the data files listed in the scripts, produced by the commands in notes/. Most numbers come from numbers.json; the sweep tallies (4083, 578, 3505+7, 304 builders), the 24-seed item4 and 12-patch item21 designs, the a4c edge exponents (0.22-0.25, 0.10-0.13) and the 300-rule c-model sample are typed by hand from the cited data/*.txt files and should be moved into compute_numbers.py.
Exception: numbers quoted from the text outputs `experiments/2026-10-13-a1-kappa/results/a1b_icc.txt`, `a1_kappa_proxy.txt`, `a1c_predict.txt`, `a1_edge_q.txt`,
`edge_exponent.txt`, `shape_stats.txt`, `b4_stats.txt`, `b5_stats.txt` are typed in main.typ with the file named beside them (the 2026-10-13 PM numbers h1, h3, g1 are instead parsed from `experiments/2026-10-13-h1-lacunarity-decomposition/results/h1_lacunarity.txt`, `h3_collapse.txt`, `g1_stats.txt`, `g1_ribbon.log` by compute_numbers.py).
Data files were produced from an uncommitted working tree (no commit hash available yet).

Transport forecasts: `uv run experiments/2026-10-13-transport-forecast/scripts/transport_forecast.py > experiments/2026-10-13-transport-forecast/results/transport_forecast.log`.
The manuscript loads `experiments/2026-10-13-transport-forecast/results/transport_forecast.json` directly; `--root .` permits
that repository-relative input. Registration, controls, exclusions and verification
are in `notes/transport-forecast-2026-10-13.md`; input hashes and seed are in the JSON.

Illustrative figures (`fig/gallery.pdf`, `fig/growth_curves.pdf`): `uv run research-outputs/cube-root-growth-paper/gallery.py`. The ant grids come from
`examples/snapshot.rs` (commands in the script's docstring; 64 MB each, not committed). Growth curves use
`experiments/2026-10-13-probe-growth-census/results/probe_all.csv` and `experiments/2026-10-13-calib-instrument-calibration/results/calib_rotor.csv` (rotor seed `rotor:0`).

Explanatory figures (`fig/{mechanics,timeline,edge,timelapse,census,rotor}.pdf`): `uv run research-outputs/cube-root-growth-paper/textbook_figs.py` (~40 s).
Needs `$GAL/RLL_k{14,17,20,23}.bin` from `examples/snapshot.rs` (commands in the script docstring); the rest is simulated
in the script (reference ant, rotor with numpy seed 1) or read from `experiments/2026-10-13-census-sweeps/results/sweep_v2.csv` and `experiments/2026-10-13-census-sweeps/results/census.csv`.

All figure scripts (`figures.py`, `gallery.py`, `textbook_figs.py`) import `research-outputs/cube-root-growth-paper/style.py` for the palette and conventions (colour meanings are documented at its top); change colours there, not in the scripts.

Memory-walker comparison figures: after building the probe and completing
`uv run experiments/2026-10-13-memory-walkers/scripts/memory_walkers_pilot.py $HOME/turmite-target-memory/release/probe`,
run `cargo run --release --example memory_paths -- 301 16 > experiments/2026-10-13-memory-walkers/dumps/memory_paths.csv`
and `uv run research-outputs/cube-root-growth-paper/memory_figures.py`. The dump is gitignored; source and input
hashes are in `research-outputs/cube-root-growth-paper/fig/memory_recipe.json`. The new typed pilot table and
width-radius prose use `experiments/2026-10-13-memory-walkers/results/memory_walkers_pilot/summary.json` and
`experiments/2026-10-13-w-width-radius-refit/results/width_radius_refit.json`; regenerate the latter with
`uv run experiments/2026-10-13-w-width-radius-refit/scripts/width_radius_refit.py`. Predictions and results are preserved in
`notes/walker-comparisons-2026-10-13.md`.

## Typesetting design (2026-10-13)

The manuscript uses Libertinus Serif for reading text, the title and headings,
DejaVu Sans for captions and page furniture, and DejaVu Sans Mono for rule strings.
Typst bundles the mathematical font; the other families must be available to
`typst fonts` (all are installed in the current WSL environment). The ink,
near-black accent and warm-grey caption colours are defined at the top of `main.typ`.
The title and abstract form an opening page; key results begin the next page.
References start on a fresh page to avoid a trailing isolated entry.

Rebuild: `typst compile --root . research-outputs/cube-root-growth-paper/main.typ research-outputs/cube-root-growth-paper/main.pdf`.
Visual QA: `typst compile --root . --format png --ppi 100 research-outputs/cube-root-growth-paper/main.typ 'tmp/paper-design/page-{p}.png'`.
This changes presentation only; generated numbers and figure inputs are unchanged.

## Figure design and title-page image (2026-10-14)

`uv run research-outputs/cube-root-growth-paper/rebuild_figures.py` regenerates all figure assets and the manuscript
from existing inputs. It writes `fig/rebuild.log` and `fig/rebuild_recipe.json`
with the git commit, source/input hashes, seeds and output hashes. Use the WSL
environment exports above. Snapshot inputs still come from `$GAL` (default
`~/gal`); no measurements or sweeps are rerun by this command.

Figures use thin neutral axes, sparse outward ticks, no grid or enclosing box,
outline histograms rather than overlapping fills, and direct endpoint labels
for the six-rule transport and growth comparisons. Short leaders connect labels
to their true endpoints when label spacing changes. Dense ensemble comparisons
retain a shared legend. Scientific colour meanings are unchanged. Vector PDFs
embed TrueType fonts; matching PNGs are generated for inspection, and PDF dates
are omitted for reproducible output.

`textbook_figs.py` also creates `fig/hero.pdf`: ant RLL and rotor occupation fields
at `2^22` steps (blank-board ant; rotor initial-arrow seed 1), normalized by `t/A`,
with a shared greyscale from 0 to 4, clipping higher values. Each territory is
fitted to its panel, so physical sizes should not be compared. This illustration
introduces the bulk/edge comparison; the measured edge exponents remain in the
body of the paper. It is not a new measurement or an estimator change.
