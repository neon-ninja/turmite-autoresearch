# Manuscript build

From the repo root (WSL; see AGENTS.md for the env):

    export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
    uv run paper/compute_numbers.py   # data/* -> paper/numbers.json (bootstrap seed 20261012)
    uv run paper/figures.py           # -> paper/fig/*.pdf
    typst compile paper/main.typ paper/main.pdf

Typst is installed via `cargo install --locked typst-cli`. Numbers in main.typ are generated, with the exceptions below;
inputs are the data files listed in the scripts, produced by the commands in notes/. Most numbers come from numbers.json; the sweep tallies (4083, 578, 3505+7, 304 builders), the 24-seed item4 and 12-patch item21 designs, the a4c edge exponents (0.22-0.25, 0.10-0.13) and the 300-rule c-model sample are typed by hand from the cited data/*.txt files and should be moved into compute_numbers.py.
Exception: numbers quoted from the text outputs `data/a1b_icc.txt`, `a1_kappa_proxy.txt`, `a1c_predict.txt`, `a1_edge_q.txt`,
`edge_exponent.txt`, `shape_stats.txt`, `b4_stats.txt`, `b5_stats.txt` are typed in main.typ with the file named beside them.
Data files were produced from an uncommitted working tree (no commit hash available yet).

Illustrative figures (`fig/gallery.pdf`, `fig/growth_curves.pdf`): `uv run paper/gallery.py`. The ant grids come from
`examples/snapshot.rs` (commands in the script's docstring; 64 MB each, not committed). Growth curves use
`data/probe_all.csv` and `data/calib_rotor.csv` (rotor seed `rotor:0`).

Explanatory figures (`fig/{mechanics,timeline,edge,timelapse,census,rotor}.pdf`): `uv run paper/textbook_figs.py` (~40 s).
Needs `$GAL/RLL_k{14,17,20,23}.bin` from `examples/snapshot.rs` (commands in the script docstring); the rest is simulated
in the script (reference ant, rotor with numpy seed 1) or read from `data/sweep_v2.csv` and `data/census.csv`.

All figure scripts (`figures.py`, `gallery.py`, `textbook_figs.py`) import `paper/style.py` for the palette and conventions (colour meanings are documented at its top); change colours there, not in the scripts.
