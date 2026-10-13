#!/bin/bash
# C2c: bias 0.2, 0.25, 0.3 on a 32768 board. usage (WSL): RAYON_NUM_THREADS=3 bash data/run_c2c.sh
cd /mnt/f/Code/turmite
for s in $(seq 1 8); do for b in 0.2 0.25 0.3; do echo brotor:$s:$b; done; done > data/c2c_walkers.txt
~/turmite-target-frontier2/release/probe 28 32768 < data/c2c_walkers.txt > data/c2c_probe.csv 2> data/c2c_probe.log && touch data/c2c.done
