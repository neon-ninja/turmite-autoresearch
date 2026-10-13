#!/bin/bash
# E1: knockout map at other ages and with the field rotated 90 degrees (RRRRRLLLLLL). usage (WSL): RAYON_NUM_THREADS=1 bash data/run_e1.sh
cd /mnt/f/Code/turmite
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
D=$HOME/turmite-target-frontier4/release/examples/dump
R=RRRRRLLLLLL
mkdir -p dumps_e1 knock_e1
# rotated field: relative coordinates (x,y) -> (-y,x), i.e. 90 degrees clockwise on screen; roll by one to keep home at (C,C)
uv run --with numpy python -c "
import numpy as np
S=4096
g=np.fromfile('dumps/$R.grid',dtype=np.uint8).reshape(S,S)
np.roll(np.rot90(g,-1),1,axis=1).tofile('dumps_e1/${R}_rot90.grid')"
[ -f dumps_e1/${R}_k22.grid ] || { $D $R 22 dumps_e1 && mv dumps_e1/$R.grid dumps_e1/${R}_k22.grid && rm -f dumps_e1/$R.traj; }
[ -f dumps_e1/${R}_k25.grid ] || { $D $R 25 dumps_e1 && mv dumps_e1/$R.grid dumps_e1/${R}_k25.grid && rm -f dumps_e1/$R.traj; }
for spec in "rot90 dumps_e1/${R}_rot90.grid" "k22 dumps_e1/${R}_k22.grid" "k25 dumps_e1/${R}_k25.grid" "k26 dumps_f/$R.grid"; do
  set -- $spec
  [ -f knock_e1/${R}_$1/done ] || if [ "$1" = rot90 ]; then export START_H=1; else unset START_H; fi; uv run data/e1_map.py ${R}_$1 $2 $R
done
touch data/e1.done
