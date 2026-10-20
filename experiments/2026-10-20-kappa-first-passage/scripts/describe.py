# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy", "polars", "matplotlib"]
# ///
"""POST-HOC, EXPLORATORY descriptives of the confirmation (not registered predictions).
Reads only committed results/ (confirm/*.bands.csv, confirm/*.status.csv, confirm_predictions.csv, confirm_roles.csv).
    uv run experiments/2026-10-20-kappa-first-passage/scripts/describe.py
Writes results/describe.json and results/fig_pred_vs_obs.png.
"""
import json
import sys
from pathlib import Path

import matplotlib
import numpy as np
import polars as pl

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import load_runs  # noqa: E402

EXP = HERE.parent
R = EXP / "results"
roles = pl.read_csv(R / "confirm_roles.csv")
runs = load_runs.runs(R / "confirm").join(roles.select("spec", "role", "family", "stratum"), on="spec", how="left")
pred = pl.read_csv(R / "confirm_predictions.csv")
out: dict = {"note": "post hoc, exploratory; thresholds and verdicts are those frozen in README"}

# stop reasons
st = runs.unique("spec")
out["stop_by_role"] = {f"{r}/{s}": int(n) for r, s, n in st.group_by("role", "stop").len().iter_rows()}
bad = st.filter(pl.col("stop") != "complete").group_by("rule", "stratum", "stop").len().sort("len", descending=True)
out["non_complete_runs_by_rule"] = [dict(rule=r, stratum=s, stop=k, runs=int(n)) for r, s, k, n in bad.iter_rows()]
out["rules_with_any_non_complete"] = bad["rule"].n_unique()
out["non_complete_by_stratum"] = {s: int(n) for s, n in st.filter(pl.col("stop") != "complete").group_by("stratum").len().iter_rows()}

# E[T], E[D] vs scale, per eligible rule, outcome seeds
elig = pred["rule"].to_list()
slopes_T, slopes_D, slopes_k, ratio = [], [], [], []
for rule in elig:
    o = runs.filter((pl.col("rule") == rule) & (pl.col("role") == "outcome") & (pl.col("reached_hi") == 1))
    g = o.group_by("band").agg(
        (pl.col("I_entered_complete_steps").sum() / pl.col("I_entered_complete").sum()).alias("ET"),
        (pl.col("B_entered_complete_steps").sum() / pl.col("B_entered_complete").sum()).alias("ED"),
        (pl.col("area_end") / pl.col("perim_end")).mean().alias("L"),
        pl.col("kappa").mean().alias("k"),
    ).sort("band")
    if g.height == 3:
        lL = np.log(g["L"].to_numpy())
        slopes_T.append(np.polyfit(lL, np.log(g["ET"].to_numpy()), 1)[0])
        slopes_D.append(np.polyfit(lL, np.log(g["ED"].to_numpy()), 1)[0])
        slopes_k.append(np.polyfit(lL, np.log(g["k"].to_numpy()), 1)[0])
q = lambda v: [float(x) for x in np.quantile(v, [0.1, 0.25, 0.5, 0.75, 0.9])]
out["slope_lnET_on_lnL"] = dict(n=len(slopes_T), quantiles_10_25_50_75_90=q(slopes_T))
out["slope_lnED_on_lnL"] = dict(n=len(slopes_D), quantiles_10_25_50_75_90=q(slopes_D))
out["slope_lnkappa_on_lnL"] = dict(n=len(slopes_k), quantiles_10_25_50_75_90=q(slopes_k))
out["frac_ET_slope_within_0p5_1p5"] = float(np.mean([0.5 <= s <= 1.5 for s in slopes_T]))
out["identity_check"] = "1 + delta - slope_ET = d ln kappa_inst / d ln L (renewal-reward, README); compare medians"
out["identity_median_lhs"] = float(1 + np.median(slopes_D) - np.median(slopes_T))
out["identity_median_rhs"] = float(np.median(slopes_k))

# who is badly predicted
pred = pred.with_columns(ratio=pl.col("obs_BS") / pl.col("pred_M2"))
out["obs_over_pred_ratio_quantiles_10_25_50_75_90"] = q(pred["ratio"].to_numpy())
out["frac_within_factor_2"] = float(((pred["ratio"] > 0.5) & (pred["ratio"] < 2)).mean())
out["frac_obs_gt_2x_pred"] = float((pred["ratio"] > 2).mean())
out["frac_obs_below_0p6_pred"] = float((pred["ratio"] < 0.6).mean())
(R / "describe.json").write_text(json.dumps(out, indent=2))

fig, ax = plt.subplots(1, 2, figsize=(9, 4), constrained_layout=True)
c = np.where(pred["stratum"].to_numpy() == "stationary", "#1f77b4", "#d62728")
ax[0].scatter(pred["pred_M2"], pred["obs_BS"], c=c, s=22)
lim = [pred["pred_M2"].min() * 0.5, pred["obs_BS"].max() * 2]
ax[0].plot(lim, lim, "k--", lw=1, label="y = x")
ax[0].plot(lim, [0.5 * l for l in lim], color="gray", lw=1, label="y = x/2")
ax[0].set(xscale="log", yscale="log", xlabel="M2 prediction of band-2 B/S (kernel bands 0,1)", ylabel="observed band-2 B/S (outcome seeds)")
ax[0].legend(frameon=False)
ax[1].hist(slopes_T, bins=15, color="#444")
ax[1].axvline(1, color="k", ls="--")
ax[1].set(xlabel="d ln E[T] / d ln (A/P), three bands", ylabel="rules")
ax[0].set_title("blue stationary, red drifting", fontsize=9)
fig.savefig(R / "fig_pred_vs_obs.png", dpi=130)
print(json.dumps(out, indent=2))
