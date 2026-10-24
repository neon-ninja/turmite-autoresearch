# Lab notebook: Square-ring fillers

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-25 10:05 (+13:00) · b942a82 · intent
Session start (Corvid, AI research agent). Asked to pick a task that moves the research forward. Read `notes/ACTIVE.md`, the
evidence ledger and the open questions. Chose Q20 (why `R^k L R R...` fills at 2(k+1) steps per cell) with Q15 (are the census
builders one class?) because both are open, cheap to test exactly, and bear on which census rules are outside the cube-root
regime (ACTIVE priority 2: board artefacts). Decision informed: whether the board fillers and builders have one exact mechanism.

### 2026-10-25 10:20 (+13:00) · b942a82 · observation
Exploration with a scratchpad Python ant (not committed; listed in README "What I looked at before registering"). `RLRRRL` from a
blank board: at 4e5 steps 97,382 of 100,898 visited cells have exactly 4 visits; reads after 1e6 steps are counts 0..3 (250,674,
250,153, 249,537, 249,015) and count 4 (621), nothing higher; xmax front advances once per ~11e3 steps, dt alternating by +72 / -8.
Edge profile, north side (outer to inner): `11..11..`, `2332 / 3223`, `4444`. Same profile with 1->k, 2->k+1, 3->2k+1, 4->2k+2 for
k = 2 (`RRLRRRR`) and k = 3 (`RRRLRRRRR`).

### 2026-10-25 10:25 (+13:00) · b942a82 · interpretation
The period-8(k+1) read sequences (k = 1, 2, 3) split into 4-step blocks; most are four R turns, i.e. closed 2x2 loops. A fresh
plaquette is looped while its new cells read below k; the L at count k ejects the ant. This would force `T = R^k L R^(k+1)` on counts
0..2k+1 and give each cell exactly 2k+2 visits. Confidence moderate: derived from three k values and one phase of the orbit. The
corner (count 2k+2) is not understood.

### 2026-10-25 10:28 (+13:00) · b942a82 · surprise
`RLRR`, `RLRRR`, `RRLRRR` read the same turns as fillers on counts 0..2k+2 yet are not fillers from a blank board by 1e6 steps
(spc 34-57). Filling from blank needs more than the ring's turn table: the seed must reach the ring. Like a highway, the ring is an
attractor whose basin need not contain the blank board.

### 2026-10-25 10:31 (+13:00) · b942a82 · decision
Pruning preflight: index `b942a82854b4d61222301959b9895060d2543d3f`; `rg --files notes/pruning-index/entries` lists one entry;
`rg -n -i -e filler -e builder -e crawler -e square -e RLRRR notes/pruning-index/entries` gives no output. The sole entry
`NR-20261022-additive-laws-order-blind` concerns additive conservation laws (rules of length 2..8), not growth mechanisms; does not
apply. Run. Budget as in README (CPU, < 1 h of 4 cores in total).

### 2026-10-25 10:31 (+13:00) · b942a82 · deviation
`notes/ACTIVE.md` (2026-10-23) prioritises boundary access and defers new proof programmes. This experiment is a bounded
regime-validation item (Priority 2) with finite computations, not a Lean programme; recorded as a deliberate departure from the
Priority-1 sequence. Registration (README P1-P6) written before any registered measurement.

### 2026-10-25 10:52 (+13:00) · b0ba3a1 · observation
P1 run: `uv run --frozen experiments/2026-10-25-square-ring-fillers/scripts/p1_crawler.py` (example
`square_ring_fillers_crawler`, built `cargo build --release --locked`). `results/p1_crawler.txt`: k = 1..64, 320 main rules
(minimal plus four random-suffix rules per k, seed 20261025) all translate by (-4, 0) every 8(k+1) steps for 16 periods, each
count 0..2k+1 read exactly 64 times; all 4,288 flips of letters 0..2k+1 fail in the first period; all 293 flips of unread suffix
letters change nothing; the independent Python ant agrees with Rust on 140 runs (k <= 10). **P1 held.** 29 s wall time.

### 2026-10-25 10:58 (+13:00) · b0ba3a1 · interpretation
Hand trace of one period for general k (to be written into the README as a proof): steps 1-2 and 5 read k (L), 6-8 open a fresh
plaquette, k-1 loops on it read (k+j, j, j, j), two steps close it (2k, k), four steps open the second plaquette (k+1, k+1, 0, k+1),
k-1 loops read (k+1+i, k+1+i, i, k+1+i), two final steps read 2k+1 twice; 8 + 4(k-1) + 2 + 4 + 4(k-1) + 2 = 8(k+1). Required turns:
T(k) = L, T(0..k-1) = R, T(k+1..2k+1) = R, nothing else. This is a proof for all k >= 1 (the loop counts are the only k-dependence);
P1's 64 values are a check of it, not the evidence.

### 2026-10-25 11:05 (+13:00) · b0ba3a1 · observation
Footprint instrument `examples/square_ring_fillers_footprint.rs` oracle (`scripts/oracle.sh` -> `results/oracle.txt`): final colour
fingerprint equal to `fast::run` (via `rlrlll_macro_transition_resim`) for RLRRRL, RRLRRRR, RLLR, RRLLLRLLLRR, RLRR at 1e6 and 1e7
steps (4096 board); independent Python ant reproduces histogram, visited, window new cells, first_gt, fronts_after for six 2e5-step
specs including two rule switches. Instrument accepted.
