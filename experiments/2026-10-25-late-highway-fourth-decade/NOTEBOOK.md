# Lab notebook: <experiment title>

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### YYYY-MM-DD HH:MM (+TZ) · <git short hash> · intent
What I am about to do, why, and which decision it informs.

### YYYY-MM-DD HH:MM (+TZ) · <git short hash> · assumption
Statement. How it could be checked / what would show it false. Status: untested.

### YYYY-MM-DD HH:MM (+TZ) · <git short hash> · observation
Fact, with the command and file that produced it.

### YYYY-MM-DD HH:MM (+TZ) · <git short hash> · interpretation
What I think it means; confidence; the alternative explanations I have not excluded.

### YYYY-MM-DD HH:MM (+TZ) · <git short hash> · decision
Pruning preflight: index revision, exact queries, candidate signature IDs (or no
matches), scope comparison, run/skip/reopen decision and compute budget. Repeat
when scope changes. See `notes/pruning-index/README.md`.

### YYYY-MM-DD HH:MM (+TZ) · <git short hash> · dead-end
Criterion tested, evidence commit, limits, shared signature ID or reason no
signature is warranted, and what would reopen the question. Append corrections.

### 2026-10-25 · df07dfe · intent
Start Q19: does the genuine late-highway yield per decade stay at ~1.7% in (1e11, 1e12]? User pointed at the new tiled grid backend as the enabler.

### 2026-10-25 · df07dfe · decision
Pruning preflight at df07dfe: queries `rg -n -i -e highway -e late -e yield -e unresolved -e decade notes/pruning-index/entries`; the only entry
(NR-20261022-additive-laws-order-blind) is unrelated. No match; run. Population = 702 budget rows of h1e11 chunks (results/survivors_1e11.txt).

### 2026-10-25 · df07dfe · observation
scripts/power.py: even all 702 at 1e12 has power 0.47 against a halving per decade (0.86 against x0.3); m=200 has 0.19. Cost ~1,000-2,000 core-hours.

### 2026-10-25 · df07dfe · interpretation
Q19 as a flat-versus-falling test is under-powered at any compute this container can offer; its useful output is an interval for p to pool in a
log-linear trend with the three earlier decades. I should have written that down when Q19 was posed (the notes called a third decade "would distinguish flat from slowly falling"); that was optimistic.

### 2026-10-25 · df07dfe · observation
Tiled backend, 65536 board, 4 parallel workers, rules 1-4 of sample_order at 3e9 steps: 179-188 Msteps/s, 11-16 MiB RSS each; dense == tiled bytes for rules 1-2.
A dense 65536 board would need 4.3 GB per worker. Per-rule cost at 1e12 ~1.5 h/core if throughput holds (T3 untested at that horizon).

### 2026-10-25 · df07dfe · decision
Not launching a 1e12 stage unasked: Stage A is ~18 h wall on all 4 cores in an ephemeral container and cannot test the hypothesis by itself. Asking the user to set the compute budget.
