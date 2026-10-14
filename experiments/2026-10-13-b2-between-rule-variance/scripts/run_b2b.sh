#!/bin/bash
cd /mnt/f/Code/turmite
cut -d, -f1 experiments/2026-10-13-patched-starts/results/patched.csv | tail -n +2 | sed 's/@.*//' | sort -u > experiments/2026-10-13-b2-between-rule-variance/results/b2b_rules.txt
(for r in $(cat experiments/2026-10-13-b2-between-rule-variance/results/b2b_rules.txt); do for s in 1 2 3 4 5 6 7 8; do echo sbg:$r:$s:0.02; done; done) > experiments/2026-10-13-b2-between-rule-variance/results/b2b_walkers.txt
~/turmite-target-frontier5/release/probe 27 16384 < experiments/2026-10-13-b2-between-rule-variance/results/b2b_walkers.txt > experiments/2026-10-13-b2-between-rule-variance/results/b2b_probe.csv 2> experiments/2026-10-13-b2-between-rule-variance/results/b2b_probe.log && touch experiments/2026-10-13-b2-between-rule-variance/results/b2b.done
