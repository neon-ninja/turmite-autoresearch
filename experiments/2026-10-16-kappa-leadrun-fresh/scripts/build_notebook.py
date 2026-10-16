# /// script
# dependencies = ["nbformat"]
# ///
"""Writes notebook.ipynb (unexecuted); execute with the nbconvert command in experiments/README.md.
usage (repo root): uv run experiments/2026-10-16-kappa-leadrun-fresh/scripts/build_notebook.py"""
from pathlib import Path
import nbformat as nbf

EXP = Path(__file__).resolve().parents[1]
md, code = nbf.v4.new_markdown_cell, nbf.v4.new_code_cell
cells = [
md("""# Does the leading run of identical letters set kappa? A fresh-rule and mutation test

## Question and why it matters
`kappa` is the pattern-field coefficient of the t^(1/3) growth law. A post hoc finding (`../2026-10-16-kappa-prefix-decode`) said the number of identical opening letters of a rule (`lead`) predicts ln kappa beyond early area ln A(22). Here it is tested on 300 fresh, lead-stratified rules (lengths 13/14) and by single-letter mutants. See `README.md`, `NOTEBOOK.md`, `notes/open-questions-hypotheses.md`.

## Assumptions and definitions
- `lead`: length of the opening run of one letter, capped at 6 in models. Stationary: |g_k| < 0.05; kept: >= 6 core-band runs (alpha_w in [0.25, 0.42], kappa > 0) of 8 seeds. *Assumed, unverified*: dropout from the kept set does not bias the slope (README Part A bounds it only by reweighting by lead).
- `ln A22`: log area at probe window k=22, a post-treatment quantity in the mutant experiment (it changes when a letter is flipped), so adjusting for it is not a clean causal estimate.
- Predictions L1-L6, A1-A2, M1-M4 were registered in the README before the measurement; those that failed are reported as failed."""),
md("## Provenance"),
code("""import hashlib, json, subprocess, datetime
from pathlib import Path
import numpy as np, polars as pl
SEED = 20261016; np.random.seed(SEED)
root = Path.cwd()
while not (root / "AGENTS.md").exists():
    assert root != root.parent, "repo root not found"; root = root.parent
E = root / "experiments/2026-10-16-kappa-leadrun-fresh/results"
files = ["analysis.json", "analysis.txt", "followup.json", "followup.txt", "fresh_rules.csv", "mutants_analysis.json", "mutants_analysis.txt", "mutant_pairs.csv", "mutants.csv", "rules.txt"]
for f in files:
    assert (E / f).exists(), f
    print(f"{f:24s} {hashlib.sha256((E / f).read_bytes()).hexdigest()[:16]}")
print("git", subprocess.run(["git", "rev-parse", "--short", "HEAD"], cwd=root, capture_output=True, text=True).stdout.strip(), "seed", SEED, datetime.date.today())"""),
md("""## Data and method
Heavy computation lives in `scripts/` (`sample.py`, `analyse.py`, `followup.py`, `mutants.py`, `analyse_mutants.py`) and the Rust `probe 27 8192` binary; this notebook only loads committed outputs. The census training set (2183 stationary rules) is from `2026-10-kappa-rule-space`."""),
md("## Results 1: the lead effect on fresh rules"),
code("""print((E / "analysis.txt").read_text())"""),
md("""**Shows:** the held-out gain of the lead term over a ln A22 spline is large and its slope is positive within area tertiles (L1, L3 held). **Does not show:** that the census slope (+0.195) transfers unchanged (L2 failed: fresh +0.152), nor that survivors represent all rules (L5 failed: retention falls with lead). L4 failed as a mis-specified control; see the next table."""),
md("## Results 2: nulls and selection bounds"),
code("""print((E / "followup.txt").read_text())
t = pl.read_csv(E / "fresh_rules.csv")
print(t.group_by("lead").agg(rules=pl.len(), kept=pl.col("kept").sum(), stationary=pl.col("stationary").sum(), mean_lnk=pl.col("lnk").mean()).sort("lead"))"""),
md("""**Shows:** a coefficient refit on a shuffled label gives gain ~0 (so +0.65 is not a fitting artefact) and the slope is positive in every cut. **Does not show:** that the slope is linear: it is steeper at leads 1-3 (+0.235) than overall, and high-lead strata are selection-exposed."""),
md("## Results 3: single-letter mutants (257 pairs)"),
code("""print((E / "mutants_analysis.txt").read_text())
p = pl.read_csv(E / "mutant_pairs.csv")
print(p.group_by("arm").agg(n=pl.len(), mean_d=pl.col("d").mean(), mean_dA=pl.col("dA").mean(), mean_dlead=pl.col("dlead").mean()).sort("arm"))"""),
md("""**Shows:** shortening the lead lowers ln kappa (-0.20) at unchanged area, control flips do nothing, and the pooled regression slope on dlead (+0.158) reproduces the cross-sectional slope. **Does not show:** a net raw effect of extending the lead (-0.06, CI includes 0, because area drops 0.29 and ln kappa tracks ln A22 with coefficient ~1), so registered M1 and M2 failed. It also does not separate "run length" from "the letter at index k"."""),
md("""## Caveats and controls
- Negative controls: refit-on-shuffle (A1) and the late-letter control arm C (d = -0.006).
- Failed predictions: L2 (range), L4 (control mis-specified), L5, the A2 "within 0.05" clause, M1, M2. None repaired.
- ln A22 is post-treatment; adjusted and area-matched mutant estimates are not unbiased causal effects. Sample sizes: 176 fresh stationary rules, 257 mutant pairs; one seed set (601-608) throughout.

## What this changes / next steps
The lead association is real and survives fresh rules, shuffling nulls and within-rule pairing, but it is entangled with early area. Next: flips that leave early area unchanged, kappa from late windows only, and larger mutant sweeps (a GPU port of the probe statistics is in progress separately).

## Open questions
Is ln kappa nearly redundant with ln A22 (coefficient ~1)? Does the letter at index k, rather than run length, carry the effect? Why is the dependence concave in lead?"""),
]
nb = nbf.v4.new_notebook(cells=cells, metadata={"kernelspec": {"display_name": "Python 3", "language": "python", "name": "python3"}})
nbf.write(nb, EXP / "notebook.ipynb")
