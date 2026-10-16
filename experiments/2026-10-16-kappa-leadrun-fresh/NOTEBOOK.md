# Lab notebook: kappa leading-run test on fresh, lead-stratified rules

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-16 19:40 (+13:00) · d4fd349 · intent
Previous loop iteration left the tail-budget experiment finished and committed (its HANDOVER "still undone" list was stale: decomposition and onset sensitivities are in baac481). Picked up the open item from `2026-10-16-kappa-prefix-decode`: the leading-run finding is post hoc on scored data and confounded with ln A(22) (corr -0.56). Test it on fresh rules stratified by lead run. Predictions L1-L6 registered in README before any probe run.

### 2026-10-16 19:46 (+13:00) · d4fd349 · observation
`scripts/sample.py` wrote 300 rules (6 lead strata x 2 lengths x 25) and 2400 specs (seeds 601-608); no overlap with W3b/W3c by construction. `run.sh` launched in WSL (same `probe 27 8192` as W3c, ~22 min expected). `scripts/analyse.py` written while it runs, before any output exists; it implements L1-L6 as registered, with one choice the README left open: L2's fresh refit uses a spline basis with knots from the fresh sample's own ln A quantiles.

### 2026-10-16 20:10 (+13:00) · d4fd349 · observation
Probe run finished 20:07 (exit 0, 2400 runs, results/probe.log; binary sha256 recorded there). A user "pause" request arrived after completion; nothing was running. `analyse.py` NOT yet run, so no outcome has been read. Work-in-progress commit requested by the user.

### 2026-10-16 20:15 (+13:00) · 58b4f38 · observation (analyse.py, results/analysis.txt)
L1 HELD (gain +0.651 [0.498, 0.828]); L3 HELD (tertile slopes 0.151/0.155/0.149); L2 FAILED (census slope 0.195 outside my 0.07-0.13; fresh refit 0.152 [0.129, 0.174]); L4 FAILED (shuffled median -1.43); L5 FAILED (stationary share 0.68 at lead 1 vs 0.38 at lead 6). Per-lead residual means -0.148..+0.418, monotone. Single run of the script, no variants.
### 2026-10-16 20:16 (+13:00) · 58b4f38 · interpretation, surprise
Leading run is real out of sample and at fixed area, not a ln A proxy. Surprise: L5, high-lead rules drop out of the stationary core band, which can bias the slope. L2 and L4 failures are my specification errors (L2 range taken from the unadjusted table; L4 shuffle without refitting the coefficient can never be near 0). Not repaired after seeing data. Next: mutation dose-response at fixed area (causal), and a refit-on-shuffle null; handle escapes by a larger board / longer-run censoring analysis.

### 2026-10-16 21:05 (+13:00) · 78bcafd · intent
Resumed from HANDOVER.md (read AGENTS.md and experiments/README.md first). Plan: (A) refit-on-shuffle null and selection bounds from existing data, `scripts/followup.py`; (B) single-flip mutation dose-response, new runs, registered in README before running; then notebook.ipynb and notes. Predictions A1/A2/M1-M4 written to the README before any of this ran (A is on already-seen data, flagged as such).

### 2026-10-16 21:15 (+13:00) · 78bcafd · observation (scripts/followup.py, results/followup.txt)
A1: shuffled-label gain median -0.001 (unrestricted), +0.000 (within ln A quintile); max +0.049/+0.058; observed +0.651. A2: slope all leads +0.152, leads 1-3 +0.235, leads 1-4 +0.197, len13 +0.144, len14 +0.157, IPW-by-lead +0.144. Dropouts: escaped-run counts not monotone in lead (3,12,7,4,9,16 of 50); kept share falls mainly at lead 6.
### 2026-10-16 21:16 (+13:00) · 78bcafd · interpretation, deviation
A1 replaces the failed L4: the instrument gives ~0 on a noninformative label, so the +0.65 is not an artefact of spline-plus-label fitting. A2 sub-prediction (leads 1-3 slope within 0.05 of all-lead) FAILED: the dependence on lead is concave, so "slope per step" depends on the range. Also corrects my 20:16 interpretation: escapes are not the main dropout route; most dropouts are rules without >= 6 core-band runs (checked only via counts, not by inspecting the individual dropped rules' alpha_w; could be checked). Confidence moderate.
### 2026-10-16 21:25 (+13:00) · 78bcafd · observation
`scripts/mutants.py` produced 417 mutants (E 152, S 113, C 152; 30 coincide with rules in the base sample), 3336 runs, `results/mutants.csv`, `mutant_specs.txt`. Launched `run_mutants.sh` (same `probe 27 8192` as the base run) in WSL, nothing else using the machine at launch. Mutant analysis script to be written while it runs and run only after it finishes.
