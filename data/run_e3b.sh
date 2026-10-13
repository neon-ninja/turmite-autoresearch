#!/bin/bash
cd /mnt/f/Code/turmite
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
uv run data/e3b_jobs.py
K=$HOME/turmite-target-frontier4/release/examples/knock
for rule in RRRRRLLLLLL RLLLLRRRRLR; do d=knock/$rule
  STRAT_ROUTE=$d/e3/route.u32 STRAT_RET=$d/e3/ret.u32 $K dumps/$rule.grid $rule 67108864 jobs $d/e3b.jobs $d/e3b.csv 2> $d/e3b.log && touch $d/e3b.done
done
touch data/e3b.done
