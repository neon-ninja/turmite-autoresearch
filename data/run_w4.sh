#!/bin/bash
cd /mnt/f/Code/turmite
~/turmite-target-frontier2/release/probe 36 32768 < data/w4_specs.txt > data/w4_probe.csv 2> data/w4_probe.log && touch data/w4.done
