#!/bin/bash
cd /mnt/f/Code/turmite
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
uv run data/d1b_specs.py
RAYON_NUM_THREADS=1 ~/turmite-target-frontier2/release/ext 12 20 2048 64 < data/d1b_specs.txt > data/d1b_screen.csv 2> data/d1b.log && touch data/d1b.done
