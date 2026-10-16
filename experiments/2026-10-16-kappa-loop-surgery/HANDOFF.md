# Handoff (written 2026-10-16 ~21:00 +13:00)

1. Read `AGENTS.md`, `experiments/README.md`, then `../2026-10-16-kappa-square-loop/{README,PLAN,NOTEBOOK}.md`, then this README and PLAN.md.
2. Gate: do not start unless the square-loop experiment has run and P2 (loop fraction rises with lead) holds. If P2 failed, append a `decision` entry
   here closing this folder, mark the README status, and stop.
3. `git status` first; the workspace is dirty with other sessions' edits (notably `src/bin/probe.rs`). Prefer a new binary over editing it.
4. Write the walker and its tests (PLAN.md items 1-2) before any scientific run. Freeze S1-S4, the cohort and the q grid as `decision` entries; they are drafts now.
5. Run the rate pilot, set matched controls, run once, analyse once. Report retention and nulls.
6. Fill `notebook.ipynb` from committed results only.

State: planning only. No code, no data, no result.
