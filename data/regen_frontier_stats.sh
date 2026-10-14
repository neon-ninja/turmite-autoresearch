#!/bin/bash
# Regenerates every analysis text output of the frontier work from the raw data files (deterministic; run twice and diff to check).
# usage (WSL): bash data/regen_frontier_stats.sh OUTSUFFIX   (writes data/NAME.OUTSUFFIX instead of data/NAME.txt when OUTSUFFIX is given)
cd /mnt/f/Code/turmite
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
ext=${1:-txt}
run() { out=$1; shift; uv run "$@" 2>&1 | grep -v "Reading inline" > data/$out.$ext; }
run highway_stats data/highway_stats.py
run edge_exponent data/edge_exponent.py
run dose_stats data/dose_stats.py
run classko_stats data/classko_stats.py
run recolour_stats data/recolour_stats.py
run flip_stats data/flip_stats.py
run formage_stats data/formage_stats.py
run tether_screen data/tether_screen_stats.py
run spread_dups data/spread_dups.py
run spread_clusters data/spread_clusters.py
run spread_groups data/spread_groups.py
run hot_rare data/hot_rare.py
run hot_recolour_stats data/hot_recolour_stats.py
run item21_stats data/item21_stats.py
run item4_stats data/item4_stats.py
run item4_rotor_stats data/item4_stats.py data/item4_rotor_probe.csv
run ext_stats data/ext_stats.py
run ext_hex data/ext_lattice_stats.py data/ext_hex.csv
run ext_cube data/ext_lattice_stats.py data/ext_cube.csv
run ext_tri data/ext_lattice_stats.py data/ext_tri.csv 16 24
run cube_fill data/cube_fill.py
run shape_stats data/shape_stats.py data/shape_rotor.csv data/shape_ants.csv
run c_letters data/c_letters.py
run mixing_calib data/mixing_calib.py
run seed_uncertainty research-outputs/cube-root-growth-paper/seed_uncertainty.py
run ext_cube_long data/ext_long_stats.py cube
run ext_tm_long data/ext_long_stats.py tm
run h1e10_stats data/h1e10_stats.py
uv run experiments/2026-10-13-cell1/scripts/cell1_stats.py 2>&1 | grep -v "Reading inline" > experiments/2026-10-13-cell1/results/cell1_stats.$ext
