#!/bin/bash
cd /mnt/f/Code/turmite
for s in $(seq 11 16); do echo rotor:$s; done > data/a2b_walkers.txt
~/turmite-target-frontier2/release/probe 36 32768 < data/a2b_walkers.txt > data/a2b_probe.csv 2> data/a2b_probe.log && touch data/a2b.done
