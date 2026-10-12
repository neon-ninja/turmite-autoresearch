# Pattern-matched fields: hypotheses before measuring

Written 2026-10-11 13:20 NZDT. Data in hand: the transplant and shuffle
results in followup_stats.json. No pattern-matched fields generated yet.

## Question

The two "hard" tethered rules (RRRRRLLLLLL, RLLLLRRRRLR) re-tether in their
own final field but never in a shuffled one (same colour counts, random
places). The three "easy" ones (RRRLLRLLR, RLLRRRRRLRLR, RRRLLLLRRRLL) tether
in shuffled fields too. How much of the field's arrangement does a hard rule
need before it can form a home?

## Ladder of fields (each keeps the exact colour counts of the visited cells)

0. shuffle: colours permuted at random (the published control).
0r. radial shuffle: permuted only within rings of width 2 around the
    territory centre. Keeps any large-scale gradient, destroys local order.
1. pairs: horizontal and vertical nearest-neighbour pair histograms matched.
2. 2x2: the histogram of oriented 2x2 blocks matched.
3. 3x3: the histogram of oriented 3x3 blocks matched.

Fields are made by simulated annealing over swaps of visited cells, starting
from a shuffle, minimising the L1 distance between histograms. Unvisited cells
stay blank and count as an extra symbol in windows that touch them.

## Predictions

- H1 (local arrangement). A hard rule needs some local pattern, not a
  gradient. Radial shuffle fails; some level k tethers. My guess: pairs fail,
  2x2 sometimes, 3x3 mostly.
- H2 (gradient). The hard rules need the large-scale radial structure. Radial
  shuffle tethers, and the local levels don't help unless they happen to
  rebuild the gradient.
- Easy rules tether at every level (they already tether when shuffled).

## The trap

The territories are small: about 7,000 and 11,000 visited cells for the two
hard rules, against 11^4 = 14,641 possible 2x2 patterns and 11^9 (2.4
billion) 3x3 patterns. If most 3x3 patterns occur once, matching their
histogram exactly forces the field to be a jigsaw of the original's patches,
and the annealer may simply reassemble the original. A positive 3x3 result
then means nothing. So for every field I also measure:

- the share of cells equal to the original at the same place (shuffle
  baseline: sum of squared colour shares), and the best share over small
  shifts;
- whether the ant tethers where the original home was, or somewhere else;
- how many distinct patterns there are per window at each level.
