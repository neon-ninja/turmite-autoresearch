# Lab notebook: translated-cycler decider for census highways

Append-only. Newest entry last. Never edit an old entry; correct it with a new one (`corrects: <timestamp>`).

### 2026-10-23 18:12 (+13:00) · 6e0840b · intent
Give the census highways a configuration-level persistence certificate (README). Informs whether proof-track item 2 has a census-wide decider and
whether any census label is an impostor.

### 2026-10-23 18:12 (+13:00) · 6e0840b · decision
Pruning preflight at `6e0840bb05a1aa261c13ee1a0019550c0e92aac0`: queries and result in the README; no applicable signature; run, within the stated budget.
Timestamps in this notebook are read from the clock (`date`), unlike some first entries of the sibling experiment (corrected there).

### 2026-10-23 18:27 +1300 · 354eab7 · observation
Full run (registered predictions D1-D5; controls and C5 run before the summary was read): 571 of 571 census highways certified (568 at K = 3); C4 0 pre-onset certificates; C1-C3 all rejected (50/50, 38/38, 50/50); C5 0 of 72,864 candidates in 100 non-highway rules; 0 oracle disagreements in 3,423 dumps. Details and caveats in the README; `results/summary.txt`.

### 2026-10-23 18:27 +1300 · 354eab7 · deviation
(1) The post-registration extras: 50 additional non-highway rules in C5 and the from-zero verification (408 rules, new binary). (2) I hand-replaced the driver's wrong TOTAL line in `results/verify_from_zero.txt` (case-sensitivity bug in my one-off driver); the 408 per-rule lines are unchanged. (3) 3 dumps at K = 300 missing (board edge), not failures.
