#!/bin/bash
cd /mnt/f/Code/turmite
cut -d, -f1 data/patched.csv | tail -n +2 | sed 's/@.*//' | sort -u > data/b2b_rules.txt
(for r in $(cat data/b2b_rules.txt); do for s in 1 2 3 4 5 6 7 8; do echo sbg:$r:$s:0.02; done; done) > data/b2b_walkers.txt
~/turmite-target-frontier5/release/probe 27 16384 < data/b2b_walkers.txt > data/b2b_probe.csv 2> data/b2b_probe.log && touch data/b2b.done
