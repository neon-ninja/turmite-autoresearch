#!/bin/bash
cd /mnt/f/Code/turmite
~/turmite-target-frontier2/release/probe 36 32768 < experiments/2026-10-13-w4-edge-exponent-2e36/results/w4_specs.txt > experiments/2026-10-13-w4-edge-exponent-2e36/results/w4_probe.csv 2> experiments/2026-10-13-w4-edge-exponent-2e36/results/w4_probe.log && touch experiments/2026-10-13-w4-edge-exponent-2e36/results/w4.done
