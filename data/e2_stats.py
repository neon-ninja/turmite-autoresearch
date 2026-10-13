# /// script
# dependencies = ["polars", "numpy"]
# ///
"""E2: sensitive-disc radius vs near_home measuring radius. Inputs knock/RULE/{base_r6,base_r15}.txt, coarse_r6/r15.csv, plus the stored r=10 map
(base.csv, coarse.csv). Metric and decision rule: notes/frontier-2026-10-13-results.md. usage: uv run data/e2_stats.py"""
import numpy as np, polars as pl
def load(rule, tag):
    sfx = "" if tag == 10 else f"_r{tag}"
    d = pl.read_csv(f"knock/{rule}/coarse{sfx}.csv")
    if tag == 10:
        b = pl.read_csv(f"knock/{rule}/base.csv")
    else:
        b = pl.read_csv(f"knock/{rule}/base_r{tag}.txt")
    return d, float(b["near_home"][0]), float(b["mx"][0]), float(b["my"][0])
for rule in ("RRRRRLLLLLL", "RLLLLRRRRLR"):
    print("\n" + rule)
    out = {}
    for R in (6, 10, 15):
        d, B, mx, my = load(rule, R)
        d = d.filter(pl.col("ok") == 1)
        sw = d.filter(pl.col("kind") == "swap"); pa = d.filter((pl.col("kind") == "patch") & (pl.col("nvis") >= 5))
        null = (B - sw["near_home"].to_numpy()) / B
        thr = float(np.quantile(null, 0.95))
        t = pa.group_by("x", "y").agg(loss=(B - pl.col("near_home")).mean() / B).with_columns(
            r=(((pl.col("x") + 2.5 - mx) ** 2 + (pl.col("y") + 2.5 - my) ** 2) ** 0.5))
        hot = t.filter(pl.col("loss") > thr)
        w = hot["loss"].to_numpy(); r = hot["r"].to_numpy()
        rad = float(np.sqrt((w * r ** 2).sum() / w.sum())) if len(w) else float("nan")
        rng = np.random.default_rng(20261013); bs = []
        for _ in range(1000):
            i = rng.integers(0, len(w), len(w)); bs.append(np.sqrt((w[i] * r[i] ** 2).sum() / w[i].sum()))
        out[R] = (rad, float(np.std(bs)))
        print(f"  R={R:2d}: base near_home {B:.4f}; null95 rel loss {thr:.4f}; hot tiles {len(w)} of {t.height}; loss-weighted RMS radius {rad:.1f} +- {np.std(bs):.1f} (bootstrap over hot tiles)")
    rho = out[15][0] / out[6][0]
    se = rho * np.hypot(out[15][1] / out[15][0], out[6][1] / out[6][0])
    print(f"  rho = radius(15)/radius(6) = {rho:.2f} +- {se:.2f}  (>=1.8 tracks measuring radius; <=1.3 tracks excursion scale)")
