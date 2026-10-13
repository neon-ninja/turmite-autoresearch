#!/bin/bash
cd /mnt/f/Code/turmite
S=~/turmite-target-frontier2/release/examples/shape
for s in $(seq 211 216); do echo rotor:$s; done > data/a4c_rotor_specs.txt
for r in RLR RLRLRRRLLR RLLLRLLRRLR RLRLLRRLL RLRLRRRRLR RLLRLLRLRRRL; do for s in 211 212; do echo $r@$s; done; done > data/a4c_ants_specs.txt
$S 34 16384 < data/a4c_rotor_specs.txt > data/a4c_shape_rotor.csv 2> data/a4c_rotor.log && touch data/a4c_rotor.done
$S 34 16384 < data/a4c_ants_specs.txt > data/a4c_shape_ants.csv 2> data/a4c_ants.log && touch data/a4c_ants.done
touch data/a4c.done
