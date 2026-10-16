# Handover: kappa leading-run fresh test (2026-10-16)
State: probe run + analyse.py done; README Results and NOTEBOOK updated. notebook.ipynb is STILL the unfilled template (todo, must run from committed results/).
Result: lead-run effect replicates on 176 fresh stationary rules and at fixed ln A (L1, L3 held; slope 0.15/step). L2, L4 (my mis-specified criteria) and L5 (survivor selection: stationary share 0.68 -> 0.38 from lead 1 to 6) failed.
Next: (1) mutation dose-response at matched area (causality; W6 earlier found early-letter effects at fixed area go both directions); (2) shuffled-lead null with refit coefficient; (3) check what the dropouts do (escapes: 365/2400 runs) and bound the selection bias; (4) fill notebook.ipynb; (5) add to notes/open-questions-hypotheses.md.
Reproduce: sample.py -> run.sh (~22 min) -> analyse.py (see README Commands).
