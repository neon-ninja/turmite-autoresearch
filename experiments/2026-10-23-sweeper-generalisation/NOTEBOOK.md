# Lab notebook: Are the other three no-highway escapes macro-transitions?

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-23 18:05 (+13:00) · d8c0892 · intent
Generalise the RLRLLL checker (user request, session start). First establish whether `RLLLLRRL`, `RLLLLRLRRL`, `RLLLRLRLLL`
run sweepers at all. Informs whether Stage 2 (checker generalisation) has any target.

### 2026-10-23 18:05 (+13:00) · d8c0892 · assumption
The user flagged mid-session that "escaped" only means the ant reached the board edge, and the three final boxes are nearly
square, so a filler is as likely as a sweeper. I agree; the plan does not assume sweepers. Status: untested, see G4/G1-G3.

### 2026-10-23 18:05 (+13:00) · d8c0892 · decision
Pruning preflight: index `d8c0892db5e97d4d4f6f8de0933814a51b5a5b7c`; `rg --files notes/pruning-index/entries` lists one entry;
`rg -n -i -e RLLLLRRL -e RLLLLRLRRL -e RLLLRLRLLL -e sweeper -e escape -e column notes/pruning-index/entries` exit 1, no
output. Sole entry `NR-20261022-additive-laws-order-blind` has a different objective (conservation laws); does not apply. Run.
Budget: see README. Note `notes/ACTIVE.md` (committed 17:49 today) defers extending proofs to more patterns unless bounded;
the user's explicit request overrides, so the work is bounded as written in the README and stops at the stated criteria.

### 2026-10-23 18:05 (+13:00) · d8c0892 · deviation
Task D of the parent (`scripts/d_resim.sh`, registered 2026-10-21, never run) was launched first in WSL at about 17:50 as the
cheap item-1 result; its output is `parent/results/d_resim.txt`. It is not part of this experiment.
