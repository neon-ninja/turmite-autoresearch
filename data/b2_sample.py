# /// script
# dependencies = ["polars", "numpy"]
# ///
"""B2: 100 random non-bursty blob rules (numpy seed 20261013), disjoint from the 40 rules of data/patched.csv, 12 patched seeds each.
Writes data/b2_walkers.txt (RULE@SEED, seeds 101..112 to keep them disjoint from earlier patched seeds). usage: uv run data/b2_sample.py"""
import numpy as np, polars as pl
s = pl.read_csv("data/probe_summary.csv").filter((~pl.col("bursty")) & (pl.col("cls") == "blob"))
old = set(l.split(",")[0].split("@")[0] for l in open("data/patched.csv").read().splitlines()[1:])
rules = sorted(r for r in s["rule"].to_list() if r not in old)
rng = np.random.default_rng(20261013); pick = sorted(rng.choice(len(rules), 100, replace=False))
open("data/b2_walkers.txt", "w").write("\n".join(f"{rules[i]}@{sd}" for i in pick for sd in range(101, 113)) + "\n")
print(len(old), "old rules excluded;", len(pick), "rules ->", 12 * len(pick), "runs")
