# Does the c law (and kappa's deficit) survive a change of lattice? Hexagonal and triangular ants

Registered 2026-10-20 00:25 (+13:00), base commit `0de4d38`. **Nothing in this folder has been run**; no lattice
c, kappa or lead slope has been computed on hex or triangular ants before this file was committed. (Disclosure: the old
`experiments/2026-10-13-ext-other-lattices-turmites-noise` screens measured area and extent exponents only, never c or kappa.)

## Question

`experiments/2026-10-19-c-law-mechanism` found that c (births per boundary step) obeys `1/c = a + b * lead` on the square lattice
(a ~ 1.36-1.40, b ~ 0.68-0.70), that 78% of circulating boundary reads return after exactly 4 steps (2x2 loops), and that the
constants a, b are not derived. The kappa deficit (ants at 0.81x the uniform reference) is also unexplained. The square
lattice has one loop length (4) and one coordination number (4), so it cannot say which lattice property sets b.
Hexagonal (z = 6) and triangular-cell (z = 3) ants change both, and hex offers two loop lengths on one lattice.

## Definitions (frozen)

- **Generic probe**: `src/probe.rs`'s octave windows `(2^(k-1), 2^k]`, with the neighbour set per lattice: square 4, hex 6
  (`HEX_DIRS` axial offsets), triangular 3 (`tri_step`, parity dependent). `boundary step` = step starting on a visited cell with
  >= 1 unvisited neighbour; `birth` = step onto a never-visited cell; `c = births / boundary steps`; `P` = visited cells with
  >= 1 unvisited neighbour; `kappa = (B/S) / (P_end/A_end)`.
- **Lead** = length of the opening run of identical letters. On hex the alphabet is `{r, l, R, L}` (60/60/120/120 degrees,
  `hex_turns`); a run of `r` (or `l`) closes a loop of 6 cells around a centre cell, a run of `R` (or `L`) closes a loop of 3
  cells around a vertex (checked in gate G3 by stepping, not assumed). Triangular `L`/`R` is +-2 edges: loop of 6 around a vertex.
  Square: loop of 4. Mirror-canonical: `s_0 = R` (square, tri), `s_0 in {r, R}` (hex). Hex strata are **r-lead** and **R-lead**.
- **kappa_unif(a, beta)** = `2 * integral_{1/2}^{1} x^(-a(1-beta)) dx` (`experiments/2026-10-16-kappa-uniform-baseline`), with a and
  beta fitted per run over k = 20..26 exactly as `experiments/_lib/analyse_probe.py`. It is an analytic formula in the run's own
  (a, beta), so it ports to any lattice; what is untested is that its premise (P/A tracks the window) holds there: gate G4.
  **kappa ratio** = kappa(k=26) / kappa_unif.
- **Window and board**: census convention, kappa and c from the last window k = 26 (2^26 steps), board 8192 x 8192, blank background.
- **Filter (stationary blob, frozen)**: not escaped (never within 2 cells of the board edge), births > 0 in every window
  k = 20..26, `c > 0`, `|g_k| < 0.05` (slope of kappa on t over k = 20..26, as `c_law.py`), and width exponent `alpha_w` in
  [0.28, 0.40] as a proxy for `cls == blob` (the census `cls` is not reproduced by committed analysis code; this proxy is new
  and is a documented deviation from the census definition).
- **Retention** (kept / run) is reported per lattice, stratum and lead. Cells with < 8 kept rules are reported as underpowered and
  are not topped up silently (a top-up is a logged deviation).

## Rule lists (frozen, `scripts/make_lists.py`, seed 20261020)

Primitive rules, length uniform in 10..14, first letter canonical, opening run exactly `m` (letter `m` differs from `s_0`, uniform
among the other letters on hex), remaining letters uniform. 48 candidates per (lattice, stratum, lead m = 1..5): square 240,
triangular 240, hex 480 (240 r-lead + 240 R-lead). Every rule runs once (blank background is deterministic, so there is no seed).
Controls: random walk, 32 seeds per lattice (uniform neighbour step, seeds 1..32); mean-field walker (resample uniformly over
visited cells, then uniform neighbour), 8 seeds per lattice. Both use the generic probe windows.

## Gates (instrument; fail = no claim on that lattice)

- **G1** the generic probe reproduces `src/probe.rs` windows exactly (all fields) on the square lattice, >= 20 rules.
- **G2** hex and triangular generic probes equal a naive HashMap re-implementation (all window fields) on >= 20 rules each, plus
  `HexSim`/`TriSim` area sequences, and a one-line mutation is caught.
- **G3** single-letter loop periods by stepping: square R 4; hex r 6, R 3; triangular R 6. A different value revises Definitions
  before any data is aggregated (logged deviation, not a failure of the science).
- **G4** mean-field walker, each lattice: kappa ratio within 0.05 of 1.00 (square was 1.118 against 1.107 measured). If it fails on a
  lattice, kappa is reported raw there and no ratio claim is made.
- **G5** square ants from this pipeline: median `1/c` lead slope in [0.55, 0.85] and intercept in [1.20, 1.60] (census 0.70/1.36).

## Derivation of what the loop length can and cannot do

Little's law (confirmed to 1.6% on the square lattice): `1/c` = mean boundary reads per born cell. Take lead `m`. A cell on the
circulating loop has been read once per lap, its colour advances by one per read, so the loop is "spent" after `m` reads of every
loop cell: reads per episode = `m * l_b` where `l_b` is the number of loop cells that are boundary cells. Hence
`b = d(1/c)/dm = l_b / N_b`, `N_b` = births per episode. The loop length `l` itself cancels *per cell*. What the derivation does
**not** fix is how `N_b` scales with `l`: if births per episode are set by a chain after the kill (about fixed), `b ~ l_b ~ l`;
if a larger loop wraps proportionally more fresh ground, `N_b ~ l_b` and `b` is lattice-independent. So the mechanism allows both
limits and the data must choose. A third, weaker idea ties `b` to coordination through the fresh fraction of a loop's neighbours.
The square b = 0.70 is the anchor for all three.

## Predictions

Fitted `1/c = a + b * lead`, leads 1..5, OLS over kept rules; b per stratum with a 2000-draw rule bootstrap, seed 20261020.

Signature vector of b = (triangular, hex R-lead, hex r-lead):
- **H-A (b scales with loop length)**: b = 0.70 * l/4 -> (1.05, 0.52, 1.05). Within hex, b_r / b_R = 2.
- **H-B (b lattice-independent)**: (0.70, 0.70, 0.70). b_r / b_R = 1.
- **H-C (b ~ 4/z)**: (0.93, 0.47, 0.47). b_r / b_R = 1.

Decision rule: a hypothesis is *supported* if all three measured strata are within 0.15 of its vector **and** every other
hypothesis misses at least one stratum by > 0.20; otherwise *undecided* (reported with the measured vector, no hypothesis forced).
My odds: H-A 25%, H-B 40%, H-C 10%, none/undecided 25%. Independently of the vector:
- **L1** within hex, the `b_r / b_R` bootstrap CI excludes 1 (H-A's signature alone): 30%.
- **L2** the intercept `a` is positive and within 0.5 of 1.36 on every lattice: 50% (no mechanism predicts it).

Kappa:
- **K1** kappa ratio median < 1 on hex and triangular (as on square, 0.81): 55%. Each median in [0.65, 0.97]: 40%.
- **K2** random-walk kappa ratio >= 1.10 on each lattice (square rw 1.24-1.32): 70%. A ratio < 1 for the memoryless control
  means the formula or probe fails on that lattice and K1 is void for it.
- **K3** `|slope of ln kappa on lead|` < 0.08 per step in every stratum (square marginal -0.022): 60%.
- **K4** rw `c` on each lattice is below the ant median c (square rw 0.448, ants higher): 60%.

## Order of work and commands (filled in as built)

This file and `NOTEBOOK.md` are committed first. Then `src/lattice_probe.rs` and `tests/lattice_probe.rs` (G1-G3; these use hex and
triangular rules but aggregate no c; any peek at c before the instrument is final is disclosed in the notebook), then
`scripts/make_lists.py`, the run, and `scripts/analyse.py` writing to `results/`.

## Results

None yet.
