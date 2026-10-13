#!/bin/bash
# E5: knockout maps on 6 more tethered rules (data/e5_rules.txt, 2^24 dumps) + RRRRRLLLLLL with the tile origin shifted by (2,2). usage (WSL): RAYON_NUM_THREADS=1 bash data/run_e5.sh
cd /mnt/f/Code/turmite
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
D=$HOME/turmite-target-frontier4/release/examples/dump
mkdir -p dumps_e5 knock_e5
for R in $(cat data/e5_rules.txt); do
  [ -f dumps_e5/$R.grid ] || $D $R 24 dumps_e5
  [ -f knock_e5/${R}_off0/done ] || OFFX=0 OFFY=0 uv run data/e5_map.py ${R}_off0 dumps_e5/$R.grid $R
  [ -f knock_e5/${R}_off2/done ] || OFFX=2 OFFY=2 uv run data/e5_map.py ${R}_off2 dumps_e5/$R.grid $R
done
R=RRRRRLLLLLL
[ -f knock_e5/${R}_off2/done ] || OFFX=2 OFFY=2 uv run data/e5_map.py ${R}_off2 dumps/$R.grid $R
touch data/e5.done
