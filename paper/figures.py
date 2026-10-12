"""Figures for main.typ. Run from repo root: uv run paper/figures.py"""
import json, sys
from pathlib import Path
import numpy as np
import polars as pl
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

R = Path(__file__).resolve().parent.parent
D = R / "data"
sys.path.insert(0, str(D))
import analyse_probe as ap  # noqa: E402

plt.rcParams.update({"font.size": 8, "axes.spines.top": False, "axes.spines.right": False, "figure.dpi": 150})
OUT = R / "paper" / "fig"
OUT.mkdir(exist_ok=True)

summ = pl.read_csv(D / "probe_summary.csv")
blob = summ.filter((pl.col("cls") == "blob") & ~pl.col("bursty"))
logs = summ.filter((pl.col("cls") == "log") & ~pl.col("bursty"))
ap.KLO, ap.KHI = 20, 26
rot = ap.per_rule(pl.read_csv(D / "calib_rotor.csv"))

# Fig 1: exponent distributions, ants vs rotor.
fig, ax = plt.subplots(1, 2, figsize=(6.4, 2.4), sharey=False)
bins = np.linspace(0.15, 0.55, 81)
for a, key, title in [(ax[0], "alpha_w", "bounding-box width"), (ax[1], "alpha_rg", "radius of gyration")]:
    a.hist(blob[key], bins=bins, density=True, color="#888", label=f"ants ($n={blob.height}$)")
    a.hist(rot[key], bins=bins, density=True, color="#d55", alpha=.8, label=f"rotor ($n={rot.height}$)")
    a.axvline(1 / 3, color="k", lw=.8, ls="--")
    a.set_xlabel(f"fitted exponent, {title}"); a.set_xlim(0.2, 0.5)
ax[0].set_ylabel("density"); ax[0].legend(frameon=False)
fig.tight_layout(); fig.savefig(OUT / "exponents.pdf"); plt.close(fig)

# Fig 2: c and kappa against time on six long runs.
cs = json.loads((D / "cube_stats.json").read_text())["long"]
fig, ax = plt.subplots(1, 2, figsize=(6.4, 2.3))
for r, v in cs.items():
    ax[0].plot(v["k"], v["c"], lw=.9, label=r); ax[1].plot(v["k"], v["kappa"], lw=.9)
ax[0].set_ylabel("$c$"); ax[1].set_ylabel(r"$\kappa$")
for a in ax: a.set_xlabel(r"$\log_2 t$")
ax[0].set_ylim(0, 0.7); ax[1].set_ylim(0, 1.6)
ax[0].legend(frameon=False, fontsize=5.5, ncol=2)
fig.tight_layout(); fig.savefig(OUT / "ck_time.pdf"); plt.close(fig)

# Fig 3: where the deviations come from.
fig, ax = plt.subplots(1, 2, figsize=(6.4, 2.6))
ax[0].scatter(blob["g_k"], blob["alpha_w"], s=2, c="#555", alpha=.5, label="blob")
ax[0].scatter(logs["g_k"], logs["alpha_w"], s=2, c="#d55", alpha=.5, label="log-growth")
ax[0].set_xlabel(r"$\gamma_\kappa$ (drift of $\kappa$)"); ax[0].set_ylabel("width exponent"); ax[0].legend(frameon=False, markerscale=3)
ax[1].scatter(blob["a_pred"], blob["a"], s=2, c="#555", alpha=.5)
ax[1].plot([0.2, 1.2], [0.2, 1.2], "k--", lw=.7)
ax[1].set_xlabel(r"$(1+\gamma_c+\gamma_\kappa)/(2-\beta)$"); ax[1].set_ylabel("area exponent")
ax[1].set_xlim(0.45, 1.0); ax[1].set_ylim(0.45, 1.0)
fig.tight_layout(); fig.savefig(OUT / "deviations.pdf"); plt.close(fig)

# Fig 4: held-out c model.
m = pl.read_csv(D / "c_model.csv").filter(pl.col("held_out"))
fig, ax = plt.subplots(figsize=(3.1, 2.8))
ax.scatter(m["c_pred"], m["c"], s=2, c="#555", alpha=.4)
ax.plot([0.1, 0.7], [0.1, 0.7], "k--", lw=.7)
ax.set_xlabel("$c$ predicted from rule letters"); ax.set_ylabel("$c$ measured")
fig.tight_layout(); fig.savefig(OUT / "cmodel.pdf"); plt.close(fig)
print("figures ok")
