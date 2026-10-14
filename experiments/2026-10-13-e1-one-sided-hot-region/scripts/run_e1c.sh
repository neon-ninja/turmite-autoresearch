#!/bin/bash
cd /mnt/f/Code/turmite
unset START_H
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
D=$HOME/turmite-target-frontier4/release/examples/dump
R=RRRRRLLLLLL
mkdir -p dumps_e1
for A in 20 21 23 27; do
  G=experiments/2026-10-13-e1-one-sided-hot-region/dumps_e1/${R}_k$A.grid
  if [ ! -f $G ]; then $D $R $A dumps_e1; mv experiments/2026-10-13-e1-one-sided-hot-region/dumps_e1/$R.grid $G; rm -f experiments/2026-10-13-e1-one-sided-hot-region/dumps_e1/$R.traj; fi
  if [ ! -f experiments/2026-10-13-e1-one-sided-hot-region/knock_e1/${R}_k$A/done ]; then uv run experiments/2026-10-13-e1-one-sided-hot-region/scripts/e1_map.py ${R}_k$A $G $R; fi
done
touch experiments/2026-10-13-e1-one-sided-hot-region/results/e1c.done
