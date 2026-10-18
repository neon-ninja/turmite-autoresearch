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

### 2026-10-18 19:36 (+13:00) · fa73570 · deviation
The 2026-10-16 mutant run (launched 21:25 entry above) was interrupted: `results/mutants_probe.log` stops at the "600" progress line of 3336 runs, has no `finished` line, `dumps/mutants.done` is absent and no probe process is running. Cause not established (the session ended; the partial `dumps/mutants.csv` is not in this worktree, `dumps/` being gitignored). Decision (instructed by the user via the launching agent): rerun from scratch with the same command, `bash experiments/2026-10-16-kappa-leadrun-fresh/run_mutants.sh`, in worktree `paper-figure-proposals` at fa73570, no change to `mutant_specs.txt`, `mutants.py`, `analyse_mutants.py` or predictions M1-M4. The partial output was never analysed. `dumps/` created empty first because the script redirects into it. The binary is rebuilt from the current tree (fa73570, not 78bcafd); intervening commits are paper/notes work, and the probe sha256 is logged at the end for comparison with `results/probe.log`. `mutants_probe.log` will be overwritten by the rerun (the old content is in git history).

### 2026-10-18 19:52 (+13:00) · fa73570 · observation
Rerun finished (19:34 -> 19:51, exit 0, 3336 runs, `dumps/mutants.done` written, `results/mutants_probe.log`). The log's `git` field is empty because WSL git cannot resolve this worktree's gitdir (`fatal: not a git repository`); the actual commit is fa73570. The binary sha256 is 6234da50..., which differs from the base run's 38f8bba8... . A different build path changes the hash, so this alone is not evidence of a code change. `git diff d4fd349 HEAD -- src Cargo.*` shows only additive `gpu/` and `onset.rs` changes. Empirical check (`scripts/check_overlap.py`, written before the analysis was run, `results/overlap_check.txt`): the 30 mutants that are also base-sample rules reproduce `fresh_rules.csv` exactly (all diffs 0.0, 18/30 kept in both). The paired d therefore compares like with like.

### 2026-10-18 19:53 (+13:00) · fa73570 · observation (scripts/analyse_mutants.py, results/mutants_analysis.txt)
Single run, unmodified, no crash. 257 kept+stationary pairs. E n=81: d -0.062 [-0.152, +0.025], dA -0.292. S n=73: d -0.198 [-0.290, -0.110], dA +0.013. C n=103: d -0.006 [-0.049, +0.037], dA 0.000. Pooled regression: b +0.158 [+0.139, +0.179], c (dA) +1.037 [+0.957, +1.119]. M1 FAILED (E mean d CI includes 0; the b part holds). M2 FAILED (E - C -0.056 [-0.159, +0.040]). M3 HELD. M4 retention E 0.53, S 0.65, C 0.68. Area-matched (|dA|<0.15): E +0.193 (n=26), S -0.237 (n=37).

### 2026-10-18 19:55 (+13:00) · fa73570 · observation, deviation (post hoc)
After reading the above I wrote `scripts/describe_mutants.py` (`results/mutants_describe.txt`), which is post hoc and descriptive only. The area-adjusted residual d - 1.037 dA is E +0.241, S -0.211, C -0.005. E's dA grows more negative with dlead (dlead 1: -0.13, n=41; dlead 2: -0.40, n=31; dlead 3-4: n=9). Checked `experiments/_lib/analyse_probe.py`: kappa = B·A/P at k=26, so ln kappa contains ln A explicitly.

### 2026-10-18 19:56 (+13:00) · fa73570 · interpretation, surprise
The lead effect looks causal at matched area in both directions: S lowers ln kappa by about 0.2 with area unchanged, and E net of area raises it by about 0.24. Both are close to the cross-sectional 0.15-0.2 per step. Its total effect is sign-mixed. Surprise: extending the lead also cuts A22 by about 0.3 (more for bigger jumps), but shortening it does not raise A22. This asymmetry cancels E's total effect and is why M1/M2 failed. The asymmetry is unexplained. Confidence: moderate for S (registered, clean, area unchanged), lower for the E decomposition (post hoc; c ≈ 1 is partly definitional and shares noise with d; E has the worst retention at 0.53, so selection could move it). Alternative for S: an early-position effect rather than a lead effect, since no arm flips an early letter without changing the lead (W6 found early-letter effects of both signs).
### 2026-10-18 19:57 (+13:00) · fa73570 · question
Why does growing the leading R-run shrink the pattern (A22) while shrinking the run leaves it unchanged? A registered test would add an arm that flips index k+1 or k+2 with the lead unchanged (early position, no lead change), plus a larger board or censoring analysis for E dropouts.
### 2026-10-18 20:05 (+13:00) · fa73570 · decision, observation
notebook.ipynb has been filled and executed (`uv run --with nbconvert --with ipykernel --with nbformat jupyter nbconvert --to notebook --execute --inplace ...`, no errors). Its verdicts and numbers match the README. It reads only committed `results/`. It was generated by a builder in the gitignored `scratch/`, so the committed notebook is the artefact. Its provenance cell prints "git unknown" when executed from WSL in this worktree (same gitdir issue as above). Closing status: Part B is complete. notes/open-questions-hypotheses.md has not been updated (left for a later session; see HANDOVER).
