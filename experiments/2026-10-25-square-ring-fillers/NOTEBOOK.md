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

### 2026-10-25 10:36 (+13:00) · c2db106 · deviation
corrects: 10:52, 10:58, 11:05. Those three timestamps were estimated, not read from the clock, and are later than the real time.
Real times: the P1 run and its entry were made at about 10:33-10:35, the oracle at about 10:35, and the commit `c2db106` at
10:36 (+13:00) (git commit time is authoritative). Order of events is as written. From here on every entry's time is read from
`date` when it is written.

### 2026-10-25 10:38 (+13:00) · 8080b28 · observation
P2/P3 run (`scripts/p23_run.sh`, 48 s, binary hashes in `results/p23_run.log`) and analysis (`scripts/p23_analyse.py` ->
`results/p23_analysis.txt`). **P2 held 25/25**: every census board filler has an exact clock (100.00% zero residual, 489-1,450
cycles per direction in [2^26, 2^27]) with slope a = 8(k+1) in all four directions, phase period p in {1, 2} everywhere.
**P3 held 25/25**: max read 2k+2; shares of 0..2k+1 within 0.08% (relative) of 1/(2k+2); count 2k+2 below 0.007% of reads;
new-cell rate within 0.06% of 1/(2k+2). Steps per visited cell at 2^27: 3.998 (k=1), 5.996, 7.994, 9.992, 11.989 (k=5).
The offsets b differ between rules with the same k (seed-dependent ring aspect, as registered: not predicted).
Note on earlier entries: the 10:05-10:28 entries were written at 10:31 from memory; exploration preceded the first clock read
(10:29), so those four times are approximate.

### 2026-10-25 10:38 (+13:00) · c281996 · observation
P4 step 1 (`scripts/p4_predict.py` -> `results/p4_predictions.txt`): the exact quadratics through the committed exit steps
on boards 1024/2048/4096 are E = 4G^2 - 49G + 1420 (RLRRRRL), 6G^2 - 49G + 773 (RRLRR), 8G^2 - 65G + 2363 (RRRLRRRRR). **P4 (i)
held**: alpha = 2(k+1) exactly, and all coefficients are integers. Predictions for G = 8192, 16384, 32768 are in the file,
committed before the 8192/16384 runs and before looking at the census 32768 rows. B1's shortfall is 49/(4G), 49/(6G), 65/(8G):
1.20%, 0.80%, 0.79% at G = 1024 (the old README quoted 1.3%, 0.6%, 0.3% relative to 2(k+1)G^2(1 - 0.0003)).

### 2026-10-25 10:44 (+13:00) · 9614c69 · observation
P4 step 2/3 (`scripts/p4_run.sh`, sweep_lean sha256 in `results/p4_run.log`, 4.4 min; `scripts/p4_compare.py` ->
`results/p4_analysis.txt`): **P4 held, 12/12 exact**. The committed quadratics predict the exit step to the step on 8192, 16384
and 32768 boards for all three rules, and the census 32768 records (h1e10 chunks 57, 50, 33) agree, e.g. RRRLRRRRR exits at
8,587,807,035 = 8G^2 - 65G + 2363. Correction to my 11:3x entry's parenthesis: the old README's 1.3%, 0.6%, 0.3% are one rule
(RLRRRRL) across G = 1024, 2048, 4096; the quadratic gives 49/(4G) = 1.20%, 0.60%, 0.30% (+ gamma), consistent.

### 2026-10-25 10:44 (+13:00) · 9614c69 · interpretation
The 25 board-filler "late highways" are now fully predicted: their census escape steps follow from an exact two-phase revolution
clock, and the registered B1 failure was the -beta G term (beta = -49, -49, -65). The census period 8(k+1) and dx/dy = 4 they were
reported with are the crawler of P1 running along the board edge.

### 2026-10-25 10:45 (+13:00) · 2182bbf · observation
P5 (`scripts/p5_run.sh`, 10 s; `scripts/p5_analyse.py` -> `results/p5_analysis.txt`). **P5 held.** (a) Switching at t0 to
RLRR, RRLRRR, RRRLRRRR (same turns on counts 0..2k+2) gives the identical visit field, position and heading after 1e8 more steps:
the first two never fill from a blank board, yet run the ring forever once given it. (b) Flipping only T(2k+2) to L breaks the
ring at the first corner: first read above 2k+2 at t0 + 2,956 / 1,962 / 3,477 steps against revolutions of about 8,000 / 13,900 /
19,700; visited cells at the end 0.76M / 0.87M / 0.79M against 25.3M / 17.0M / 12.9M. Also: the originals first read above 2k+2
at steps 60, 96, 132 (the seed), never later in the window. Correction to the 10:44 entry: "my 11:3x entry" means the 10:38
P4-step-1 entry.

### 2026-10-25 10:45 (+13:00) · 2182bbf · interpretation
The ring is an attractor whose existence depends exactly on T(0..2k+2) = R^k L R^(k+2) (P1 for the edge, P5 for the corner),
while reaching it from a blank board depends on the seed, i.e. on letters the ring never reads. Same logical structure as a
highway: the structure is certified by finitely many letters, its onset is not.
