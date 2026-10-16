# Handover: kappa leading-run fresh test (2026-10-16, updated 21:20)
State: all registered parts run and written up (README Results, Part A, Part B; NOTEBOOK; notebook.ipynb executed from committed results; note added to notes/open-questions-hypotheses.md). Nothing from this last stretch is committed yet.
Results: lead effect replicates on 176 fresh stationary rules (gain +0.65, slope +0.152, refit-on-shuffle null ~0). Mutants (257 pairs): shortening lead -0.20 at fixed area, control flips null, extending lead net -0.06 because ln A22 drops 0.29 and ln kappa tracks ln A22 with coefficient ~1. Registered failures: L2, L4, L5, A2 (within-0.05 clause), M1, M2.
Open: ln A22 is post-treatment (adjusted estimates not clean causal); run length vs letter at index k; kappa from late windows only; kappa ~ redundant with early area?
A subagent is porting probe stats to the GPU in a separate worktree (experiments/2026-10-16-gpu-probe-port/); result pending, not merged.
Reproduce: sample.py -> run.sh -> analyse.py -> followup.py -> mutants.py -> run_mutants.sh -> analyse_mutants.py -> build_notebook.py + nbconvert (README Commands).
