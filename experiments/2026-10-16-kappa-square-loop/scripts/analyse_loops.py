# /// script
# dependencies = ["numpy", "polars", "scipy"]
# ///
"""STUB. Analysis P2, P3, P4, C1-C3 of ../README.md per ../PLAN.md section 3. Not implemented: the loop sampler does not
exist yet, so there is no input. It fails loudly instead of producing numbers.
usage (repo root): uv run experiments/2026-10-16-kappa-square-loop/scripts/analyse_loops.py"""
from pathlib import Path
import sys

EXP = Path(__file__).resolve().parents[1]
need = EXP / "dumps/loops.csv"
if not need.exists():
    sys.exit(f"missing {need}: write and run examples/kappa_square_loop_sampler.rs first (PLAN.md section 1)")
raise NotImplementedError("implement per PLAN.md section 3; register the [choose] decisions in NOTEBOOK.md first")
