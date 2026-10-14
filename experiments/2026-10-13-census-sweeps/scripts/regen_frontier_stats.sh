#!/bin/bash
# Regenerates every analysis text output of the frontier work from the raw data files (deterministic; run twice and diff to check).
# usage (WSL): bash experiments/2026-10-13-census-sweeps/scripts/regen_frontier_stats.sh OUTSUFFIX   (writes results/NAME.OUTSUFFIX instead of results/NAME.txt when OUTSUFFIX is given)
cd /mnt/f/Code/turmite
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
ext=${1:-txt}
run() { out=$1; shift; d=$(dirname "$(dirname "$1")")/results; uv run "$@" 2>&1 | grep -v "Reading inline" > $d/$out.$ext; }  # output lands in the results/ of the script's experiment
run highway_stats experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/scripts/highway_stats.py
run edge_exponent experiments/2026-10-13-edge-statistics/scripts/edge_exponent.py
run dose_stats experiments/2026-10-13-dose-corruption-dose/scripts/dose_stats.py
run classko_stats experiments/2026-10-12-knockout-maps/scripts/classko_stats.py
run recolour_stats experiments/2026-10-13-recolour/scripts/recolour_stats.py
run flip_stats experiments/2026-10-13-flip-damage/scripts/flip_stats.py
run formage_stats experiments/2026-10-13-formage-age/scripts/formage_stats.py
run tether_screen experiments/2026-10-13-tether-rules/scripts/tether_screen_stats.py
run spread_dups experiments/2026-10-13-spread-home/scripts/spread_dups.py
run spread_clusters experiments/2026-10-13-spread-home/scripts/spread_clusters.py
run spread_groups experiments/2026-10-13-spread-home/scripts/spread_groups.py
run hot_rare experiments/2026-10-13-hot-tiles/scripts/hot_rare.py
run hot_recolour_stats experiments/2026-10-13-hot-tiles/scripts/hot_recolour_stats.py
run item21_stats experiments/2026-10-13-item21-slow-outliers/scripts/item21_stats.py
run item4_stats experiments/2026-10-13-item4-exponent-rotor-calibrated/scripts/item4_stats.py
run item4_rotor_stats experiments/2026-10-13-item4-exponent-rotor-calibrated/scripts/item4_stats.py experiments/2026-10-13-item4-exponent-rotor-calibrated/results/item4_rotor_probe.csv
run ext_stats experiments/2026-10-13-ext-other-lattices-turmites-noise/scripts/ext_stats.py
run ext_hex experiments/2026-10-13-ext-other-lattices-turmites-noise/scripts/ext_lattice_stats.py experiments/2026-10-13-ext-other-lattices-turmites-noise/results/ext_hex.csv
run ext_cube experiments/2026-10-13-ext-other-lattices-turmites-noise/scripts/ext_lattice_stats.py experiments/2026-10-13-ext-other-lattices-turmites-noise/results/ext_cube.csv
run ext_tri experiments/2026-10-13-ext-other-lattices-turmites-noise/scripts/ext_lattice_stats.py experiments/2026-10-13-ext-other-lattices-turmites-noise/results/ext_tri.csv 16 24
run cube_fill experiments/2026-10-13-probe-growth-census/scripts/cube_fill.py
run shape_stats experiments/2026-10-13-shape-limit/scripts/shape_stats.py experiments/2026-10-13-shape-limit/results/shape_rotor.csv experiments/2026-10-13-shape-limit/results/shape_ants.csv
run c_letters experiments/2026-10-13-probe-growth-census/scripts/c_letters.py
run mixing_calib experiments/2026-10-13-mixing/scripts/mixing_calib.py
uv run research-outputs/cube-root-growth-paper/seed_uncertainty.py 2>&1 | grep -v "Reading inline" > experiments/2026-10-13-probe-growth-census/results/seed_uncertainty.$ext
run ext_cube_long experiments/2026-10-13-ext-other-lattices-turmites-noise/scripts/ext_long_stats.py cube
run ext_tm_long experiments/2026-10-13-ext-other-lattices-turmites-noise/scripts/ext_long_stats.py tm
run h1e10_stats experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/scripts/h1e10_stats.py
uv run experiments/2026-10-13-cell1/scripts/cell1_stats.py 2>&1 | grep -v "Reading inline" > experiments/2026-10-13-cell1/results/cell1_stats.$ext
