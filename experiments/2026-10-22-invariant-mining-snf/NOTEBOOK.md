# Lab notebook: invariant mining, stage 2

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-22 21:36 (+13:00) · b0c9e8a · intent
User request: add automated invariant mining to the analysis pipeline, to find conservation laws, parity
invariants and symmetries that restrict trajectories, instead of only measuring step counts and growth.

### 2026-10-22 21:38 (+13:00) · b0c9e8a · decision
Another agent had just (21:33) created `src/invariants.rs`, `src/bin/invariants.rs`, `tests/invariants.rs`
and then `experiments/2026-10-22-invariant-mining/` (enumerative miner over `a x + b y + d h + e t +
sum w(colour)` at m = 2, 4). I do not edit or commit any of that. This experiment is a separate folder with
a stage-2 scope: all moduli at once (Smith normal form), sublattice families, the universal quotient, and the
question of what the rule can change. My Rust is an `examples/` oracle checker only, so my build does not
depend on their uncommitted `pub mod`. I build into a private `CARGO_TARGET_DIR` and stage explicit paths.

### 2026-10-22 21:43 (+13:00) · db7eb40 · intent
Registered ansatz families, derivation of the order-blindness argument, and predictions P1-P8 in the README
and committed it before writing any solver code (`db7eb40`, commit time 21:43:05).

### 2026-10-22 21:51 (+13:00) · eaa092c · deviation
(1) The README's registration header says "21:50"; the commit that froze it is 21:43:05. The text is a wrong
guess, the freeze is the commit. (2) The README says "all 510 rules of length 2..8"; the correct count is
2^2 + ... + 2^8 = 508. (3) These notebook entries for 21:36-21:43 were written together at 21:51, not as they
happened. None of these changes a prediction.

### 2026-10-22 21:44 (+13:00) · db7eb40 · observation
`uv run --frozen` fails here (`failed to remove .venv\lib64: Access is denied`; the shared `.venv` is
unwritable). The solver is standard-library Python, so every script is run as `uv run --no-project`. The
`--frozen` rule of AGENTS.md is for scripts with dependencies; these have none.

### 2026-10-22 21:46 (+13:00) · db7eb40 · observation
`lawmod.py --selftest` passes: SNF kernel order and generators equal brute-force enumeration for 100 random
integer matrices at m in {2, 3, 4, 6, 8, 12}; the families A0/F1/F2/F3 equal brute force on RL, RR, RLR at
small m; Langton's heading law (`heading = #colour-1 cells mod 4`) is in A0 for RL.

### 2026-10-22 21:47 (+13:00) · db7eb40 · surprise
First census run on n <= 5: P1 holds (0 exceptions), but my P4 (pure-tape heading law iff D = 0 mod 4) fails
for 10 rules, all with D = 2 mod 4 and n even (RR, LL, RRRL, ...). Cause (post hoc): the universal colour clock
gives t = sum c N_c (mod 2) for even n, and heading parity equals time parity, so heading parity is a
function of the tape whenever n is even. The law found is Phi(h) = 2h, E_R = E_L = 2, and the cycle condition
2n = 0 mod 4 holds. My registered claim was too strong; it conflated "a heading law" with "a law that pins h
mod 4". P4 as registered is falsified. A post hoc refinement (a law with Phi(1) odd) is computed separately
and labelled post hoc.

### 2026-10-22 21:49 (+13:00) · db7eb40 · observation
Oracle smoke test (n <= 4, 2000 steps, 16 starts per rule, seed 20261022): 0 law violations, 6560 control
vectors all detected. Then full n <= 6 run: 0 law violations; all 6560 controls (n <= 4) detected. See
`results/oracle_summary.json`.

### 2026-10-22 21:52 (+13:00) · eaa092c · observation
Cross-check against the parallel miner's binary (built from its uncommitted source, hashes recorded): for all
508 rules, m in {2, 4}, (lines printed) + 1 = A0 SNF order: 0 mismatches in 1016 comparisons. Two derivations
with different search strategies agree.
Highways: 582 distinct confirmed rules in `confirmed_v2.csv` + `confirmed_1B.csv` (not 589; duplicates and
the 555-primitive count in frontier-results.md are different filters). No violation of any tile-free law in
F1, F2, F3; the tile-free group has the same order for every one of the 582 rules (F1/F2: 4, 8, 8 at m = 2,
4, 8; F3: 8, 16, 16), i.e. it is the universal parity group and adds nothing to H19.
