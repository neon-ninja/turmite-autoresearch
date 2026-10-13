#!/bin/bash
# A4: boundary-width series to 2^32. usage (WSL): CARGO_TARGET_DIR=~/turmite-target-frontier2 RAYON_NUM_THREADS=1 bash data/run_a4.sh  (waits for data/b1.done)
cd /mnt/f/Code/turmite
while [ ! -f data/b1.done ]; do sleep 60; done
cargo build --release --example shape 2>&1 | tail -1
for r in RLR RLRLRRRLLR RLLLRLLRRLR RLRLLRRLL RLRLRRRRLR RLLRLLRLRRRL; do for s in 201 202 203 204; do echo $r@$s; done; done > data/a4_ants_specs.txt
for s in $(seq 201 208); do echo rotor:$s; done > data/a4_rotor_specs.txt
$CARGO_TARGET_DIR/release/examples/shape 32 16384 < data/a4_rotor_specs.txt > data/a4_shape_rotor.csv 2> data/a4_rotor.log && touch data/a4_rotor.done
$CARGO_TARGET_DIR/release/examples/shape 32 16384 < data/a4_ants_specs.txt > data/a4_shape_ants.csv 2> data/a4_ants.log && touch data/a4_ants.done
touch data/a4.done
