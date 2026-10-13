#!/bin/bash
# C2: biased-arrow rotor vs unbiased control. usage (WSL): RAYON_NUM_THREADS=1 bash data/run_c2.sh
cd /mnt/f/Code/turmite
for s in $(seq 1 12); do echo rotor:$s; for b in 0.25 0.5 0.75 0.9; do echo brotor:$s:$b; done; done > data/c2_walkers.txt
~/turmite-target-frontier2/release/probe 28 8192 < data/c2_walkers.txt > data/c2_probe.csv 2> data/c2_probe.log && touch data/c2.done
