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

### 2026-10-21 08:40 (+13:00) · 67f7d03 · observation
Pilot (25/27 specs; RLRLLLL ineligible, step budget). Controls C1b 2851/2851, rw identical, null ICC 0.011+-0.019. Population G reaches 0.8 at r*~1-7; time-weighted at ~L. Details in README "Pilot results".

### 2026-10-21 09:00 (+13:00) · 67f7d03 · decision
Registered P5 (time-weighted reach) after seeing the pilot (so pilot-informed), kept P1-P4 unchanged, added radius 256, committed as 1998e7d before launching confirmation.

### 2026-10-21 09:40 (+13:00) · 1998e7d · observation
Confirmation 72/72. P1 falsified (median r*/L 0.096/0.053, gamma 0.12), P1-alt met on the population statistic, P2 met, P3 met (C2 only from the pilot), P4 met with the outcome-conditioning caveat, P5 met (median r*_t/L 1.16, gamma_t 0.99). `results/confirm_analysis.txt`, `confirm_p4.txt`.

### 2026-10-21 09:45 (+13:00) · 1998e7d · surprise
I expected (and registered) P1: global reach even in the population statistic. It failed: the typical excursion is local. The global dependence lives only in the time-weighted view. My prior was wrong about the typical case and the pilot-informed P5 was right only because the pilot had shown it.

### 2026-10-21 09:50 (+13:00) · 1998e7d · interpretation
Two-population picture: short loops with a local return map, and rare long excursions whose duration depends on structure out to O(L). Alternatives not excluded: (i) the time-weighted reach reflects a handful of structural gates, not diffuse structure; (ii) shuffle-induced highway-like or trapped configurations lengthen T in the replicas, inflating the reach; (iii) censoring at the 2^20 cap for deep excursions (not tabulated here: check cens fraction before citing).

### 2026-10-21 10:00 (+13:00) · 4a34642 · observation
Cap check for alternative (iii): 317 of 660160 replicas (0.05%) censored at 2^20; 0 of 8252 baselines. Censoring does not drive the time-weighted reach. Alternative (ii) (shuffle creating long trapped replicas) is not excluded; its signature would be replica T far above baseline T at small r.
