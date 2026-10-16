# kappa and the square-loop trap (H-loop)

**Status: scaffold only (2026-10-16 ~20:55 +13:00). Only P1 has been run (a derivation check). The instrument,
sampler and analysis (P2-P4) are NOT written. Nothing here is evidence for H-loop.**
Parent: `../2026-10-16-kappa-leadrun-fresh/` (leading-run effect on ln kappa; read its README first).
The hypothesis was first written in that folder's `NOTEBOOK.md`, entry "2026-10-16 20:45 ... question, intent",
before any loop measurement. The statements below copy it. Do not reword predictions after seeing data; append instead.
Causal follow-up (draft): `../2026-10-16-kappa-loop-surgery/`.

## Question

Why does a longer opening same-letter run ("lead" k: `R^k L...` or `L^k R...`) raise kappa = (B/S)/(P_end/A_end)
at fixed area? kappa is defined in `notes/kappa-mechanisms-2026-10-15.md`. The effect, on fresh stationary rules:
+0.152 ln-kappa per lead step [0.130, 0.174], concave (leads 1-3: +0.235), mean residual -0.15 (lead 1) to +0.42 (lead 6).

## Mechanism under test (H-loop)

On fresh (colour 0) cells an all-R lead traces the 2x2 loop O -> E -> SE -> S -> O (4 steps per lap), and each lap
raises all four cells by one colour. The loop persists while the four cells' colours are < k: k laps = 4k steps, until
the first L-turn breaks it. Young cells are the frontier. So a longer lead means more of the ant's frontier time is
spent circling in short loops, hence more boundary occupation (higher B/S) at the same area.
Derivation check (P1): `scripts/p1_sanity.py`, result `results/p1_sanity.txt`.

## Operational definitions (fixed before any code; do not tune after seeing data)

- **Loop step** at time t: pos[t] == pos[t-4] AND the four turns made at steps t-3..t are the same letter. A same-turn
  closed 4-path is exactly a 2x2 lap; pos-repeat alone also catches back-and-forth artefacts. Chirality-agnostic.
- **Lead-colour loop step**: a loop step in which all four cells entered in the lap had colour < lead (before the update).
  **Other loop step**: any other loop step (for example a lap inside a later same-letter run of the rule). This split is the specificity control.
- **Loop entry**: first loop step of a maximal run of consecutive loop steps. Report loop-step fraction and loop-entry rate.
- **Boundary step**: the probe's definition: the step starts on a cell with an unvisited 4-neighbour (`Region::on_boundary`, `src/probe.rs`).
- **Windows**: probe octave windows (2^k, 2^(k+1)]. Reported kappa is the **k = 26** window value
  (`experiments/_lib/analyse_probe.py`: KLO=20, KHI=26, `kappa=float(g["kappa"][-1])`); ln A uses k = 22.
  The instrument reports loop statistics for k = 20..26. Primary window for P2-P3 is k = 26; k = 22 and 24 are secondary.
  (The 20:45 notebook entry said only "the probe window"; this fixes it. See NOTEBOOK 20:58 `decision`.)

## Registered predictions (copied from the 20:45 notebook entry; thresholds are my design choices, not findings)

- **P1** (rule-only): on a blank board a lead-k rule's opening 4k steps repeat with period 4 exactly (100%).
- **P2**: loop-step fraction in the window rises monotonically with lead 1..6 on the fresh stationary rules (176 in
  `../2026-10-16-kappa-leadrun-fresh/results/fresh_rules.csv`), and within each ln A22 tertile Spearman(fraction, lead) > 0.5.
- **P3** (mediation): adding per-rule loop fraction to spline(ln A22) + lead removes >= half of the lead coefficient
  (same stationary rules; coefficient before is 0.152). An unchanged coefficient means loops are a symptom, not the carrier.
- **P4**: P(loop step | step on boundary cell) >= 2 x P(loop step | interior step).
- Kill/downgrade logic (registered): P2 fails => H-loop dead. P2 holds, P3 fails => loops are a symptom of lead.
  P2 and P3 hold, P4 fails => loops matter but not through boundary occupation: unexplained, say so.

**Added now, before any loop data (written after P1-P4, so flagged as later additions):**
- **C1 (specificity)**: P3 repeated for the *other* loop kind only. H-loop predicts the mediation fraction for other-loops is
  less than half that of lead-colour loops. If both mediate equally, "loop" is a generic recurrence proxy, not the lead mechanism.
- **C2 (identity caveat)**: loop steps on boundary cells are boundary steps by construction, so P3 partly restates B/S. Also run
  P3 with *interior-only* loop steps and with loop-entry rate. If mediation vanishes when boundary loop steps are excluded, report it as accounting.
- **C3 (mutation link)**: on the usable base/mutant pairs of `../2026-10-16-kappa-leadrun-fresh/results/mutant_pairs.csv` (exists only after
  that folder's `analyse_mutants.py` has run), the paired change in loop fraction should have the sign of d = ln kappa(mut) - ln kappa(base),
  Spearman > 0.3 over arms E and S pooled. Not powered; report whatever it is.

## Competing explanations the data should also be able to support

1. **Young-cell exposure / colour phase** (`notes/kappa-mechanisms-2026-10-15.md` section 2, `../2026-10-15-kappa-prefix-exposure/`):
   lead matters only because early colours are read most. Would predict loop fraction adds nothing beyond exposure of colours 0..k-1.
2. **Lead as proxy for composition**: R-count alone gained 0.29 vs lead 0.65 (parent L6), so composition is not the whole story; other prefix letters are untested.
3. **Survivor selection** (parent L5 failed): high-lead rules drop out more. Loop statistics on survivors inherit this.
4. **Generic short-period recurrence** (period 2/4/6 trapping of any kind, not tied to lead). C1 addresses this.

## How to evaluate (decision table)

| P2 | P3 | C1 | P4 | Reading |
|---|---|---|---|---|
| fail | any | any | any | H-loop dead at this scale. Write it up; add to `notes/open-questions-hypotheses.md` |
| hold | fail | any | any | loops correlate with lead but do not carry the kappa effect |
| hold | hold | fail | any | generic recurrence, not lead-specific |
| hold | hold | hold | fail | loops carry the effect, but not via boundary occupation as assumed: unexplained |
| hold | hold | hold | hold | consistent with H-loop, still observational. Proceed to `../2026-10-16-kappa-loop-surgery/` |

## Files

`PLAN.md` instrument and analysis spec. `SCHEMA.md` output columns. `HANDOFF.md` first steps. `NOTEBOOK.md` lab notebook.
`notebook.ipynb` skeleton. `scripts/p1_sanity.py` (run). `scripts/analyse_loops.py` (stub that exits until data exist).

## Commands (WSL, repo root)

```bash
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
uv run experiments/2026-10-16-kappa-square-loop/scripts/p1_sanity.py   # done: results/p1_sanity.txt
# NOT YET WRITTEN: examples/kappa_square_loop_sampler.rs, scripts/run.sh, scripts/analyse_loops.py (see PLAN.md)
```

## Results

P1 only: 240/240 rules (k = 1..6, both chiralities) repeat with exactly k laps (`results/p1_sanity.txt`, seed 20261016).
This confirms the hand derivation for the blank-board opening. It says nothing about the 2^26-2^27 window, where the
cells around the frontier are not blank and a pure lead-colour loop needs four cells all below the lead; how often that happens is what P2 measures.
