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

### 2026-10-16 09:05 (+13:00) · 269288e · intent
Decode the prefix-8 signal in kappa. Reframed target as the residual after the ln A(22) spline, stationary rules (the x2 "0.63" is area alone). Registered D1-D5 in README before running.

### 2026-10-16 09:35 (+13:00) · 269288e · observation
`scripts/decode.py`: lookup gain +0.201; additive R-indicators +0.301; summaries +0.330; own-prefix census residual 0.045-0.138; shuffle controls ~0. Post hoc ablations: leading-run length alone +0.330, letter 2 alone +0.227, letters 4-8 +0.065, letters 9+ ~0, length and R fraction ~0.

### 2026-10-16 09:40 (+13:00) · 269288e · surprise
D1 and D5 failed: the lookup is beaten by a one-number feature. Residual is near-linear in leading run (+0.1 per step), same slope in census and pool (`results/leadrun_table.txt`).

### 2026-10-16 09:45 (+13:00) · 269288e · interpretation
Prefix info is essentially "how many identical letters open the rule". Confound not excluded: lead run vs ln A(22) corr -0.56, one area snapshot. A-series post hoc on scored data, so +0.33 is optimistic. Next: registered fresh-rule test plus fixed-area leading-run mutation dose-response.

### 2026-10-16 09:50 (+13:00) · 269288e · deviation
No executed notebook.ipynb written (template left as is); README holds the results. To do before this experiment counts as finished.
