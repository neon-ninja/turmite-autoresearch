#!/bin/bash
cd /mnt/f/Code/turmite
grep -v -F -f <(awk -F, 'NR>1 && $2==-1{print $1}' data/d2_cube.csv | sort -u | sed 's/$/,/') <(sed 's/$/,/' data/cube_long_specs.txt) | sed 's/,$//' > data/d2c_specs.txt
~/turmite-target-frontier2/release/ext 28 34 64 1024 < data/d2c_specs.txt > data/d2c_cube.csv 2> data/d2c_cube.log && touch data/d2c.done
