# Lab notebook: adaptive search for the falsification harness

Append-only. Tags: intent, assumption, observation, interpretation, surprise, decision, dead-end, question, deviation.

### 2026-10-22 22:30 (+13:00) · 253adab · intent
Make the adaptive attacker beat random sampling (user request), as a paired, equal-budget, replicated comparison rather than
one chain vs one sample.

### 2026-10-22 22:32 (+13:00) · 253adab · observation
From `experiments/2026-10-22-falsification-harness/results/C*.csv`: delay is heavy-tailed; the tails come from sparse/obstacle
(RL) and mono/patch (RRRL), not from dense patches. Details in README "Diagnosis".

### 2026-10-22 22:34 (+13:00) · 253adab · decision
New code goes in its own binary (`src/bin/falsify_search.rs`) using only the public `turmite::falsify` API, because another
contributor has uncommitted edits in `src/falsify.rs` and `src/lib.rs`; it can be promoted into the library once those land.
Predictions E1-E4 registered in README before the binary exists.
