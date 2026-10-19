# Lab notebook: lattice c law (hex and triangular ants)

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-20 00:25 (+13:00) · 0de4d38 · intent
Start `lattice-c-law`: test whether the square-lattice c law (1/c = 1.36 + 0.70 lead) and the kappa deficit hold on hex and triangular
ants. Registered the three loop-length hypotheses (A: b ~ loop length, B: lattice-independent, C: ~ 4/z) with numeric signature vectors
before any lattice c exists. Decision it informs: whether the 2x2-loop picture of c is specific to square geometry.

### 2026-10-20 00:25 (+13:00) · 0de4d38 · assumption
kappa_unif(a, beta) ports to hex and triangular because it is analytic in the run's own (a, beta); its premise (a uniform walker has P/A
tracking the window) is untested there. Falsified by gate G4 (mean-field walker ratio within 0.05 of 1). Status: untested.

### 2026-10-20 00:25 (+13:00) · 0de4d38 · question
My derivation says loop length cancels per loop cell (b = l_b / N_b) and the data must decide how N_b scales with the loop; I do not know
which limit is right, so the H-A vs H-B odds (25/40) are a guess. The census `cls == blob` is not reproducible from committed code, so I
use an alpha_w proxy (deviation from the census definition, stated in the README).

### 2026-10-20 01:10 (+13:00) · adf6677 · observation
Gates G1-G3 pass (`cargo test --release --test lattice_probe`, 7 tests): square windows equal `probe.rs` on the fields the lattice probe
produces; hex/triangular/square ants and both random movers equal a HashMap re-implementation; ant areas equal `HexSim`/`TriSim` at every
octave; one-letter loop cells are square 4, hex r 6, hex R 3, triangular 6 (the periods predicted from geometry). A deliberate mutation
(`fresh_nbrs += unv + (t & 1)`) failed 5 of 7 tests, then reverted. No c was aggregated on any lattice yet. A timing pilot ran 3 rules not
in the frozen lists (output discarded).

### 2026-10-20 01:10 (+13:00) · adf6677 · deviation
(1) The README said "all fields" for G1 and "2^26 steps": the lattice probe omits the radius-of-gyration and ant-spread fields (not needed
here), so G1 compares the seven fields it produces; and the census kappa window k = 26 is the octave (2^26, 2^27], so runs go to 2^27
steps (windows k = 12..26), as the census. Neither changes any prediction. (2) The kappa-ratio and width-exponent analyses use the
window set k = 20..26 as `analyse_probe.py`.

### 2026-10-20 01:45 (+13:00) · adf6677 · observation
Ran controls (106 s) and 960 ants (488 s). Gates G4 (mean-field ratio 0.994 / 1.005 / 1.004) and G5 (square b 0.696, a 1.286) pass. Registered
fits: b = 0.696 square, 0.355 triangular [0.340, 0.371], 0.876 hex R-lead; hex r-lead OLS is garbage (b 1.98, R^2 0.01) because one rule has
1/c = 390. Decision rule: undecided. L1, L2, K4 failed; K1, K2, K3 held. Files: `results/analysis.txt`.

### 2026-10-20 01:50 (+13:00) · adf6677 · surprise
b falls with loop length; none of A, B, C (all predicted b flat or rising with loop length / coordination) is close. Triangular is half the square.
Per-lead medians (post hoc, `results/explore.txt`) give hex r-lead b ~ 0.27, so the within-hex ratio is ~0.32, not 2.

### 2026-10-20 01:55 (+13:00) · adf6677 · interpretation
All three registered hypotheses are disproven as stated (triangular alone excludes each by >0.3). I do not have a replacement: b(l) = 0.86, 0.70,
0.34, 0.28 for l = 3, 4, 6, 6 is a post hoc description on four points. Alternatives not excluded: the hex fits mix letter types after the lead; the
"loop around a vertex" lengths I used may not be the loops that matter on triangles (they enclose a vertex, not a cell); survivor selection (retention
0.4-0.98 by cell). Confidence in "b depends on lattice": high (CI-level). In "b falls with l": moderate (4 strata, 3 distinct l).

### 2026-10-20 01:55 (+13:00) · adf6677 · decision
Not extending the registered analysis (no outlier trimming added after the fact). Next experiment, if any, must register a b(l) prediction before
using a new loop length or a letter-type-separated hex lead.
