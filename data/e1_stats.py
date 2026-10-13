# /// script
# dependencies = ["polars", "numpy"]
# ///
"""E1 (the first rot90 map was invalid and is replaced by rot90b, see the notes): hot-tile lobe direction at different ages / rotated field. Inputs knock_e1/RRRRRLLLLLL_*/{base.csv,coarse.csv} and the original
knock/RRRRRLLLLLL/{base.csv,coarse.csv}. Hot rule as knockout_analyse.py. Decision rules: notes/frontier-2026-10-13-results.md. usage: uv run data/e1_stats.py"""
import numpy as np, polars as pl
from pathlib import Path
rng = np.random.default_rng(0)
def hot_tiles(d):
    b = pl.read_csv(d / "base.csv"); B = float(b["near_home"][0]); mx, my = float(b["mx"][0]), float(b["my"][0])
    c = pl.read_csv(d / "coarse.csv").filter(pl.col("ok") == 1)
    c = c.with_columns(loss=B - pl.col("near_home"))
    null = c.filter(pl.col("kind") == "swap")["loss"].to_numpy()
    p95 = float(np.quantile(null, .95)); p95m3 = float(np.quantile(rng.choice(null, size=(20000, 3)).mean(axis=1), .95))
    pt = c.filter((pl.col("kind") == "patch") & (pl.col("nvis") > 0))
    t = pt.group_by("x", "y").agg(n=pl.len(), loss=pl.col("loss").mean(), nex=(pl.col("loss") > p95).sum()).filter(pl.col("n") >= 3)
    t = t.with_columns(hot=(pl.col("loss") > p95m3) & (pl.col("nex") >= 2))
    return B, mx, my, t
def angle(h, mx, my):
    # screen coordinates: +y is south. compass bearing clockwise from north: atan2(dx, -dy)
    dx = h["x"].to_numpy() + 2.5 - mx; dy = h["y"].to_numpy() + 2.5 - my
    th = np.arctan2(dx, -dy); R = np.hypot(np.cos(th).mean(), np.sin(th).mean()); mean = np.degrees(np.arctan2(np.sin(th).mean(), np.cos(th).mean())) % 360
    return mean, R, np.hypot(dx, dy).mean()
dirs = {"k24 (original)": Path("knock/RRRRRLLLLLL")}
for tag in ("k22", "k25", "k26", "rot90b"):
    p = Path(f"knock_e1/RRRRRLLLLLL_{tag}")
    if (p / "done").exists(): dirs[tag] = p
print("map | base near_home | tiles | hot tiles | lobe bearing (deg clockwise from north) | resultant length | mean hot distance from tether centre")
res = {}
for name, p in dirs.items():
    B, mx, my, t = hot_tiles(p); h = t.filter(pl.col("hot"))
    if h.height >= 3:
        m, R, dist = angle(h, mx, my); res[name] = (m, R, h.height)
        print(f"{name:15s} | {B:.4f} | {t.height} | {h.height} | {m:6.1f} | {R:.2f} | {dist:.1f}   (tether centre {mx:.2f},{my:.2f})")
    else:
        print(f"{name:15s} | {B:.4f} | {t.height} | {h.height} | too few hot tiles for a direction")
if "rot90b" in res and "k24 (original)" in res:
    d = (res["rot90b"][0] - res["k24 (original)"][0]) % 360
    print(f"rotated-minus-original lobe bearing: {d:.0f} degrees (exact rotation covariance predicts 90)")
base = res.get("k24 (original)")
for n in ("k22", "k25", "k26"):
    if n in res and base:
        d = (res[n][0] - base[0] + 180) % 360 - 180
        print(f"{n}: bearing difference from the 2^24 lobe: {d:+.0f} degrees; fixed-direction criterion (|d| <= 30 and >= 5 hot tiles): {'yes' if abs(d) <= 30 and res[n][2] >= 5 else 'no/NA'}")
