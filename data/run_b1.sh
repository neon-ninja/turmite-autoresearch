#!/bin/bash
# B1/A2: 20 ants + 8 rotors to 2^34. usage (WSL): RAYON_NUM_THREADS=3 bash data/run_b1.sh
cd /mnt/f/Code/turmite
(cat data/b1_rules.txt; for s in $(seq 1 8); do echo rotor:$s; done) > data/b1_walkers.txt
split -l 7 -d -a 2 data/b1_walkers.txt data/b1_batch_
for f in data/b1_batch_??; do
  n=${f##*_}
  [ -f data/b1_probe_$n.done ] && continue
  ~/turmite-target-frontier2/release/probe 34 32768 < $f > data/b1_probe_$n.csv 2> data/b1_probe_$n.log && touch data/b1_probe_$n.done
done
touch data/b1.done
