#!/bin/bash
# D2: 3D ants to 2^32 in a 768^3 box. usage (WSL): CARGO_TARGET_DIR=~/turmite-target-frontier2 RAYON_NUM_THREADS=2 bash data/run_d2.sh
cd /mnt/f/Code/turmite
cargo build --release --bin ext 2>&1 | tail -1
$CARGO_TARGET_DIR/release/ext 24 32 64 768 < data/cube_long_specs.txt > data/d2_cube.csv 2> data/d2_cube.log && touch data/d2.done
