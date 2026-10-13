#!/bin/bash
cd /mnt/f/Code/turmite
for s in $(seq 401 464); do echo RRLLLLLLR@$s; done > data/a7c_walkers.txt
~/turmite-target-frontier2/release/probe 32 16384 < data/a7c_walkers.txt > data/a7c_probe.csv 2> data/a7c_probe.log && touch data/a7c.done
