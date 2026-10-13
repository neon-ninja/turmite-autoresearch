#!/bin/bash
# E2: knockout coarse map with the near_home radius at 6 and 15 (stored job lists). usage (WSL): RAYON_NUM_THREADS=1 bash data/run_e2.sh
cd /mnt/f/Code/turmite
K=$HOME/turmite-target-frontier2/release/examples/knock
for rule in RRRRRLLLLLL RLLLLRRRRLR; do
  for R in 6 15; do
    d=knock/$rule
    [ -f $d/coarse_r$R.done ] && continue
    NEAR_R=$R $K dumps/$rule.grid $rule 67108864 base /tmp/e2_$rule > $d/base_r$R.txt
    NEAR_R=$R $K dumps/$rule.grid $rule 67108864 jobs $d/coarse.jobs $d/coarse_r$R.csv && touch $d/coarse_r$R.done
  done
done
touch data/e2.done
