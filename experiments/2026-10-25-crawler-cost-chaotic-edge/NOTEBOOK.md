# Lab notebook: is a chaotic edge a broken crawler?

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-25 22:55 (+13:00) · 56aac50 · intent
Tim asked me to take the square-ring crawler's "cost per filled cell" over to the chaotic edge. I read the c-law
record first (chiral-edge, c-law-mechanism, renewal, lattice) so I would not repeat H-loop or the K3/K4 kernels. The
fact that matters most is Stage 2c: c does not depend on the interior's colours, so it is a property of the
self-built edge layer. The crawler is a self-built edge layer that has been solved exactly.

### 2026-10-25 23:00 (+13:00) · 56aac50 · observation
Exploration, before registering: `c_law_mechanism_trace blank 22 2048` on `R^k L R^(k+1)`, k = 1..8 (scratchpad,
not kept). For k = 5 and 7, r_j = 1.00 for j <= k, 0.25 at k+1, 0 beyond; 1/c = 6.241 and 8.231 (k + 5/4 = 6.25 and
8.25); chains {1, 3} half each. For the other k, 1/c = 2.320, 3.175, 4.089, 5.149, 7.061, 8.972, with chain-2 shares of
0.10-0.30. Three of those rules later turned out to be in the P1 cohort (disclosed in the README).

### 2026-10-25 23:05 (+13:00) · 56aac50 · interpretation
The crawler line `1/c = m + 5/4` and the chaotic lead law `1.363 + 0.696 m` share an intercept to within 0.11. Read
through the crawler, the chaotic edge loses about 0.3 boundary reads per lead step. My conjecture (H-crawl) is that
those are circulations broken by shared cells reading the wrong letter. The crawler names the counts at which shared
cells must read `s_0`: m+1..2m+1. Confidence is moderate. H-young (support from young shared cells, no window) is
just as natural, and C4's weak prefix-4 gain is mild evidence against a window at m = 1.

### 2026-10-25 23:10 (+13:00) · 56aac50 · decision
Pruning preflight at 56aac50: two entries, no match (queries in README). Run. Budget under 1e11 steps.
Cohort sizes from rule strings only: 40 Pi-blobs with m >= 2 (17, 9, 4, 2, 3, 2, 2, 1 for m = 2..9), all "other" in
square-ring P6. W parents are drawn from 1045 / 417 / 276 stationary blobs (m = 1 / 2 / 3, n in 10..12).

### 2026-10-25 23:30 (+13:00) · 56aac50 · intent
Registering H-crawl, P0-P3 and the decision rule in the README, then committing before any list or run.

### 2026-10-25 23:22 (+13:00) · a387d27 · deviation
corrects: 23:30 registration time. The README header says "Registered 2026-10-25 23:30". That was the time I
planned, not a clock reading. The registration commit `a387d27` is timestamped 10:14:17 UTC (23:14 +13:00), and the
commit time governs. From here on, times are read from `date`.

### 2026-10-25 23:22 (+13:00) · a387d27 · observation
P0a (`scripts/p0_strip.py` -> `results/p0_strip.txt`, written after the registration): HELD exactly for k = 1..8,
`1/c = k + 5/4` in exact rationals, with r and chains as derived. Lists (`scripts/make_lists.py` -> `results/lists/`):
25 fillers, 40 Pi rules, 73 control draws (56 distinct), 90 parents, 748 mutants (664 distinct). Two Pi rules
(`RRRRRRRRLRRR`, m = 8; `RRRRRRRRRLRR`, m = 9) have no composition-matched control at any relaxation, because no
non-Pi stationary blob has lead 8 or 9 at those lengths and R-counts. P1b is therefore scored over the 38 Pi rules that
have controls. This is a consequence of the frozen matching rule, not a choice I made after seeing data.
