# t1-excursion-tomography

Migrated legacy investigation (2026-10-14). Its files used to sit in the flat `data/` directory (and `calib/`, `occ/`, `snaps/`, `dumps*`, `knock*` where listed) with the label prefix below; `experiments/_migration/map.tsv` gives the exact old -> new path of every file. This stub was generated from file names and script docstrings. It is **not** the original write-up: the question, timestamped hypotheses, commands and results are in `notes/` (mainly `notes/frontier-results.md`, `notes/frontier-status.md`, `notes/research-frontier*.md`; search for the label), and the producing command is usually in the script docstring or a `run_*.sh` here.

Old labels / prefixes: t1, t1b

## Scripts

- `scripts/t1_audit.py`: T1 audit: is D_med/R vs kappa a normalisation artefact (D in cells ~ 1)?
- `scripts/t1_depth_analysis.py`
- `scripts/t1_excursions.py`: Analysis of T1 excursion tomography data (experiments/2026-10-14-t1-excursion-tomography/results/t1_excursions.csv).
- `scripts/t1_pair_disp.py`
- `scripts/t1_specs.py`: Patched starts seeds 0..7
- `scripts/t1b_analysis.py`: T1b analysis (notes/excursion-tomography-2026-10-13.md section 7).

## Results

12 entries in `results/` (2 are `.log`/`.done` markers); csv/json/txt outputs, `*_stats.txt` hold the summaries quoted in the notes.
