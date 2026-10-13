# /// script
# dependencies = ["polars", "numpy"]
# ///
"""B1/A2/A4 rule set: the 6 rules already run to 2^32 (data/long.csv) + the 11 item-4 rules (13 distinct after overlap... union),
topped up to 20 with random blob rules (numpy seed 20261013, alpha_w in 0.30-0.37, non-bursty) from probe_summary.csv.
Writes data/b1_rules.txt. usage: uv run data/b1_select.py"""
import numpy as np, polars as pl
long_ = pl.read_csv("data/long.csv")["rule"].unique().sort().to_list()
i4 = [l.strip() for l in open("data/item4_rules.txt") if l.strip()]
base = list(dict.fromkeys(long_ + i4))
s = pl.read_csv("data/probe_summary.csv").filter((~pl.col("bursty")) & (pl.col("cls") == "blob") & pl.col("alpha_w").is_between(0.30, 0.37))
pool = sorted(set(s["rule"].to_list()) - set(base))
rng = np.random.default_rng(20261013)
extra = [pool[i] for i in rng.choice(len(pool), 20 - len(base), replace=False)]
rules = base + extra
open("data/b1_rules.txt", "w").write("\n".join(rules) + "\n")
print(len(base), "base +", len(extra), "extra =", len(rules)); print(rules)
