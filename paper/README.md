# Manuscript build

From the repo root (WSL; see AGENTS.md for the env):

    export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
    uv run paper/compute_numbers.py   # data/* -> paper/numbers.json (bootstrap seed 20261012)
    uv run paper/figures.py           # -> paper/fig/*.pdf
    typst compile paper/main.typ paper/main.pdf

Typst is installed via `cargo install --locked typst-cli`. No number in main.typ is typed by hand;
inputs are the data files listed in the scripts, produced by the commands in notes/.
Data files were produced from an uncommitted working tree (no commit hash available yet).
