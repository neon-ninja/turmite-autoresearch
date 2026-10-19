# Lab notebook: kappa of the calibration walkers against their own uniform reference

Append-only. Newest entry last. Corrections are new entries. Tags: intent, assumption,
observation, interpretation, surprise, decision, dead-end, question, deviation.

### 2026-10-19 (+13) · 6de71f2 · intent
Recompute rotor and memoryless-walker kappa/kappa_unif with each walker's own measured (a, beta); decide whether the ant location (median 0.81) is ant-specific. README registered with predictions before any new run. Informs the open item in the 2026-10-16 01:00 deviation entry.

### 2026-10-19 (+13) · 6de71f2 · observation
`scripts/prior_ab.py` on existing CSVs (a1d_probe.csv, calib_rotor.csv; k=12..22): pw a 0.89-0.93, beta 0.88-0.93, kappa_unif 1.02-1.03, ratio 1.21-1.32; rotor a 0.724, beta 0.564, kappa_unif 1.104, ratio 1.68.

### 2026-10-19 (+13) · 6de71f2 · surprise
The task premise "memoryless reference near 1.16" does not hold: the 1.16 value assumes beta = 1/2, but porous memoryless territories have beta ~0.9, giving kappa_unif ~1.02-1.03. Registered in README before the run. The ratios therefore stay ~1.2-1.3 rather than dropping.
