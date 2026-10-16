# Lab notebook: loop surgery (causal test of H-loop)

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-16 21:00 (+13:00) · 9dce376 · intent
Scaffolded as the interventional follow-up to `../2026-10-16-kappa-square-loop/`, at the user's request for directories that let later researchers evaluate the predictions. Draft only: nothing is frozen or run. Purpose: observational P3 cannot rule out reverse causation (high boundary occupancy producing more loops), so suppress loops directly and compare to matched-rate noise and a non-loop shape control.

### 2026-10-16 21:01 (+13:00) · 9dce376 · assumption
(a) A loop-completing turn can be flipped without changing the ant's local environment in other ways that matter to kappa; this is false in general (the flipped step writes a different next heading and the cell colour still increments), so arm N and M controls matter more than arm S alone. (b) Matching arms on flip rate is enough to match "amount of perturbation"; it is not: flips at loop closures happen in a correlated, state-dependent way. Compare realised dynamics (new-cell rate, area growth exponent) across arms as well as kappa. Both untested.

### 2026-10-16 21:02 (+13:00) · 9dce376 · question
Is there a cleaner intervention than flipping turns, for example a rule-level edit that removes loops while holding the lead and area fixed? None found; a lead-k rule always loops on fresh cells by construction (P1), so lead and loop duration cannot be separated by rule edits alone. This is why the intervention acts on trajectories, not rules.

### 2026-10-16 20:56 (+13:00) · 9dce376 · deviation (corrects: timestamps of entries above)
The entries above carry times 20:55-21:02, written while the wall clock was at about 20:50-20:54, so some are stamped slightly in the future. Order is correct; treat the stamps as approximate. Also: the parent mutant probe run finished 20:53 (`../2026-10-16-kappa-leadrun-fresh/dumps/mutants.done` exists); `analyse_mutants.py` had not been run when this was written.
