# Kappa matched-area lab notebook

Append-only. Tags: intent, assumption, observation, interpretation, surprise, decision, dead-end, question, deviation.

### 2026-10-14 22:25 (+13:00) · 797704a · intent
Follow the T1b continuation's "next work": decide whether rule-specific kappa survives matching on territory size, using existing T1b windows, before spending simulation on interventions (T2). Predictions in README.

### 2026-10-14 22:25 (+13:00) · 797704a · assumption
Seeds are replicates of a rule, and windows from the same seed at k=22 and 25 are not independent; the pooled ICC treats them as exchangeable and overstates precision. Check: ICC by k separately (the script prints it). Untested.

### 2026-10-14 22:25 (+13:00) · 797704a · assumption
At fixed t, ln A and ln kappa are linked by the identity of Eq. 1, so "controlling for area" removes part of the signal by construction; the M1 threshold of 0.40 was chosen with that in mind, not derived.

### 2026-10-14 22:35 (+13:00) · 797704a · observation
matched_area.py (README command) -> results/matched_area.txt. Pooled ICC raw 0.674, area-adjusted 0.286; by k adjusted 0.561 / 0.467. Slopes of ln kappa on ln A: between rules +1.271, within rules +0.420. Partial Spearman cutoff/R vs kappa given A: -0.560 (k=22), -0.695 (k=25), intervals exclude 0.

### 2026-10-14 22:35 (+13:00) · 797704a · surprise
Both slopes are positive. The registered M2 assumed the across-rule slope would be clearly negative. I mis-signed the confound; M2 therefore failed for a reason unrelated to the hypothesis it was meant to test. T1's "inverse territory size" wording needs re-reading.

### 2026-10-14 22:35 (+13:00) · 797704a · interpretation
M1 failed (0.286 < 0.40) while M3 held, so the registered decision branches conflict; call it unresolved. The per-k adjusted ICCs (0.47-0.56) are post hoc and not a substitute. The pooled ICC is probably depressed because the pooled slope blends between (1.27) and within (0.42) slopes. Confidence low. Alternatives: seed replicates are not exchangeable across k; the linear area adjustment is wrong in form.

### 2026-10-14 22:35 (+13:00) · 797704a · decision
Do not run an intervention yet. Register matched-area pairs first (README "Next").

### 2026-10-15 19:45 (+13:00) · 7eff085 · intent
Register and run matched-area pairs (README second section) before any intervention.

### 2026-10-15 19:45 (+13:00) · 7eff085 · observation
matched_pairs.py -> results/matched_pairs.txt. Tol 0.05: 62/67 pairs; median |dlnk| ratio matched/all 0.24/0.19 (P-a failed, <0.25 band); higher-kappa rule has smaller cutoff in 82%/76% of pairs, intervals exclude 0.5 (P-b held).

### 2026-10-15 19:45 (+13:00) · 7eff085 · interpretation
Area carries most of kappa's rule variation at fixed t; a small residual still orders the cutoff. Confidence low-moderate: seed-noise floor for the residual not yet computed. Notebook.ipynb not updated (still template stub for this section) and should be before closing the experiment.

### 2026-10-15 19:45 (+13:00) · 7eff085 · decision
No intervention yet. Next: seed-noise floor and noise-restricted P-b.
