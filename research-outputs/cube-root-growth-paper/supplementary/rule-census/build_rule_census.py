# /// script
# dependencies = ["polars"]
# ///
"""Build supplementary/rule-census/rule_census.csv: one row per swept rule (length 2-12,
4083 rules), joining every per-rule measurement the paper's census uses.

Run from the repo root:
  uv run research-outputs/cube-root-growth-paper/supplementary/rule-census/build_rule_census.py

Deterministic: pure joins of committed result files, no randomness; rows are
sorted by (n, rule). Nothing here is re-measured; see README.md for provenance.
"""
import json
import subprocess
from pathlib import Path

import polars as pl

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
CS = ROOT / "experiments/2026-10-13-census-sweeps/results"
PG = ROOT / "experiments/2026-10-13-probe-growth-census/results"


def root_of(s: str) -> str:
    """Shortest u with u^k == s (identical dynamics); s itself if primitive."""
    n = len(s)
    for p in range(1, n):
        if n % p == 0 and s[:p] * (n // p) == s:
            return s[:p]
    return s


sweep = pl.read_csv(CS / "sweep_v2.csv").with_columns(
    steps_per_cell=pl.col("steps") / (pl.col("width") * pl.col("height")))
sweep = sweep.rename({c: f"sweep_{c}" for c in sweep.columns
                      if c not in ("rule", "n", "steps_per_cell")})
sweep = sweep.rename({"steps_per_cell": "sweep_steps_per_cell"})

conf = pl.read_csv(CS / "confirmed_v2.csv").drop("n")
conf = conf.rename({c: f"conf_{c}" for c in conf.columns if c != "rule"})

# 1e9-step re-run of the rules still unresolved at 1e8 (a subset of rules).
b1 = pl.concat([pl.read_csv(CS / "sweep_1B.csv"),
                pl.read_csv(CS / "sweep_1B_extra.csv")], how="diagonal_relaxed").drop("n").unique(maintain_order=True)
assert b1["rule"].is_unique().all(), "1e9 files disagree on a rule"
b1 = b1.rename({c: f"run1e9_{c}" for c in b1.columns if c != "rule"})
b1_hw = set((CS / "1B_highways.txt").read_text().split())

cen = pl.read_csv(CS / "census.csv").drop("n", "spc")
cen = cen.rename({"full": "reached_2e27", "alpha": "census_alpha_w",
                  "per_doubling": "census_ext_per_doubling", "ext": "census_ext_2e27",
                  "cls": "phase_census"})

probe = pl.read_csv(PG / "probe_summary.csv").drop("n", "cls", "alpha")
probe = probe.rename({"alpha_w": "probe_alpha_w", "alpha_rg": "probe_alpha_rg"})
probe = probe.rename({c: f"probe_{c}" for c in probe.columns
                      if c not in ("rule",) and not c.startswith("probe_")})

cmod = pl.read_csv(PG / "c_model.csv").drop("c")
cmod = cmod.rename({"c_pred": "cmodel_c_pred", "held_out": "cmodel_held_out"})

d = (sweep.join(conf, on="rule", how="left")
     .join(b1, on="rule", how="left")
     .join(cen, on="rule", how="left")
     .join(probe, on="rule", how="left")
     .join(cmod, on="rule", how="left"))

d = d.with_columns(
    in_census=pl.col("phase_census").is_not_null(),
    phase=pl.coalesce("phase_census", pl.lit("highway")),
    analysed=pl.col("probe_alpha_w").is_not_null(),
    n_L=pl.col("rule").str.count_matches("L"),
    n_R=pl.col("rule").str.count_matches("R"),
    primitive_root=pl.col("rule").map_elements(root_of, return_dtype=pl.String),
    highway_1e9=pl.col("rule").is_in(list(b1_hw)),
).with_columns(
    is_repeat=pl.col("primitive_root") != pl.col("rule"),
    # the 7 census rules that escaped the 8192 grid late are kept in the census
    census_late_escape=pl.col("in_census") & (pl.col("sweep_stop") == "escaped"),
).drop("phase_census")

front = ["rule", "n", "n_L", "n_R", "primitive_root", "is_repeat", "phase", "in_census",
         "census_late_escape", "analysed"]
d = d.select(front + [c for c in d.columns if c not in front]).sort("n", "rule")
d.write_csv(OUT / "rule_census.csv", float_precision=10)

commit = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"],
                        capture_output=True, text=True).stdout.strip()
summary = {
    "rows": d.height, "columns": d.width, "git_head_at_build": commit,
    "phase_counts": {r["phase"]: r["len"] for r in d.group_by("phase").len().iter_rows(named=True)},
    "in_census": int(d["in_census"].sum()), "analysed": int(d["analysed"].sum()),
}
(OUT / "rule_census.build.json").write_text(json.dumps(summary, indent=1, sort_keys=True) + "\n")
print(json.dumps(summary, indent=1, sort_keys=True))
assert d["rule"].is_unique().all() and d.height == 4083
