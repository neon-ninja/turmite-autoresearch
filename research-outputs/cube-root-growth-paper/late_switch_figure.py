"""Figure for the RLRLLL blob -> column-sweeper switch (main.typ, section "A blob becomes a column sweeper").

Run from repo root: uv run research-outputs/cube-root-growth-paper/late_switch_figure.py
Reads only committed results of experiments/2026-10-13-g1-ribbon-breakout/results/ (g1_ribbon.csv from
examples/ribbon_trace.rs; g1_wake_*.csv, g1_sweep_trace_*.csv, g1_wake_extract.json from scripts/g1_wake_extract.py, which
alone reads the gitignored builder_dump output). Deterministic, no randomness. Writes fig/late_switch.{pdf,png} and
fig/late_switch_numbers.json (the predicted and measured d|x|^2/dt it plots)."""
import json, sys
from pathlib import Path
import numpy as np
import polars as pl
import matplotlib.pyplot as plt

R = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(R / "research-outputs" / "cube-root-growth-paper"))
import style as st  # noqa: E402

st.apply()
G1 = R / "experiments/2026-10-13-g1-ribbon-breakout/results"
OUT = R / "research-outputs/cube-root-growth-paper/fig"
T0S = (12800000000, 12900000000)
WIN = 2e7          # +-2e7 steps of 1e6-step windows around each dump, as in the audit

d = pl.read_csv(G1 / "g1_ribbon.csv").with_columns(dt=pl.col("t_end") - pl.col("t_end").shift(1).fill_null(0))
coarse, fine = d.filter(pl.col("dt") > 5_000_000), d.filter(pl.col("dt") <= 5_000_000)
ext = json.loads((G1 / "g1_wake_extract.json").read_text())["dumps"]

# onset exactly as g1_stats.py: first fine window after which steps/new stays < 8 to the end of the trace
f = fine.sort("t_end"); s = f["steps_per_new"].to_numpy(); ok = s < 8
onset = int(f["t_end"][max(i for i in range(len(ok)) if not ok[i]) + 1])

# predicted vs measured d|x|^2/dt
pm = {}
for t0 in T0S:
    e = ext[str(t0)]
    absx = 0.5 * (abs(e["x_start"]) + abs(e["x_end"]))
    L = e["L_traj"]
    pred = absx / (2 * L)        # dx/dt = -3 columns / (12 L steps) = -1/(4L), so d|x|^2/dt = 2|x|/(4L)
    w = f.filter((pl.col("t_end") > t0 - WIN) & (pl.col("t_end") <= t0 + WIN))
    t, x = w["t_end"].to_numpy().astype(float), w["x"].to_numpy().astype(float)
    slope, icpt = np.polyfit(t, x ** 2, 1)
    # between-window scatter of the local slope, to show how well "measured" is pinned down
    pm[t0] = {"L": L, "abs_x": absx, "predicted": pred, "measured": float(slope), "n_windows": int(w.height),
              "x_at_t0_from_windows": float(np.interp(t0, t, x))}
(OUT / "late_switch_numbers.json").write_text(json.dumps(
    {"onset_t": onset, "windows_pm_2e7": WIN, "d_abs_x_sq_dt": {str(k): v for k, v in pm.items()}}, indent=1))

fig = plt.figure(figsize=(st.W, 4.5))
gs = fig.add_gridspec(2, 3, height_ratios=[1.05, 1], width_ratios=[1.0, 1.0, 0.95], hspace=.6, wspace=.45)

# (a) steps per newly visited cell
ax = fig.add_subplot(gs[0, :])
c_ok, f_ok = coarse.filter(pl.col("steps_per_new").is_finite()), fine.filter(pl.col("steps_per_new").is_finite())
ymax_ax = 3e4
ax.plot(c_ok["t_end"] / 1e10, c_ok["steps_per_new"], ".", ms=2.5, color=st.ANT, label=r"$10^8$-step windows")
ax.plot(f_ok["t_end"] / 1e10, f_ok["steps_per_new"], ".", ms=1.6, color=st.BUILDER, label=r"$10^6$-step windows")
empty = fine.filter(~pl.col("steps_per_new").is_finite())["t_end"].to_numpy() / 1e10
ax.plot(empty, np.full(len(empty), ymax_ax * .7), "|", ms=4, mew=.6, color=st.INK, label="no new cell in window")
ax.axhline(4, **st.GUIDE); ax.text(0.01, 4.4, "4 steps per cell", fontsize=6.5, color="#6b6b64")
ax.axvline(onset / 1e10, **st.GUIDE)
ax.set_yscale("log"); ax.set_ylim(2.5, ymax_ax); ax.set_xlim(0, 1.31)
ax.set_xlabel(r"step $t$ ($\times 10^{10}$)"); ax.set_ylabel("steps per new cell")
ax.annotate(f"switch at $t = {onset / 1e10:.4f}\\times10^{{10}}$", xy=(onset / 1e10, 8), xytext=(onset / 1e10 - .02, 70),
            ha="right", fontsize=6.5, arrowprops=dict(arrowstyle="-|>", lw=.5, color=st.INK, shrinkA=0, shrinkB=1))
ax.legend(loc="upper left", ncol=3, bbox_to_anchor=(0, 1.0), columnspacing=1.2, handletextpad=.3)
ax.set_title("(a) blob phase, then a steady 4 steps per new cell", fontsize=8)

# (b1) y(t) of the sweep: slow build traverse, fast return
ax = fig.add_subplot(gs[1, 0])
t0 = T0S[0]
tr = pl.read_csv(G1 / f"g1_sweep_trace_{t0}.csv")
ax.plot(tr["t_rel"] / 1e5, tr["y"] / 1e3, lw=.7, color=st.BUILDER)
ax.set_xlabel(r"steps after $1.28\times10^{10}$ ($\times10^5$)"); ax.set_ylabel(r"$y$ ($\times10^3$ cells)")
ax.set_title("(b) sweep height over time", fontsize=8)
ax.set_ylim(-4.6, 5.6)
ax.text(0.5, 0.98, r"slow rise $\approx 10$ steps/row, fast fall $\approx 2$", transform=ax.transAxes, ha="center", va="top",
        fontsize=6.5)

# (b2) wake crop: columns repeat with period 3
ax = fig.add_subplot(gs[1, 1])
cr = pl.read_csv(G1 / f"g1_wake_crop_{t0}.csv")
nr, nc = ext[str(t0)]["crop"]["rows"], ext[str(t0)]["crop"]["cols"]
col = cr["colour"].to_numpy().reshape(nr, nc)
ax.imshow(st.cells_img(col, col != 255, 6), interpolation="nearest", aspect="auto")   # stretched horizontally so the 3-column period is legible
st.clean(ax)
ax.set_title("(c) wake colours", fontsize=8)
ax.plot([-.4, 2.4], [-2.5, -2.5], color=st.INK, lw=.8, clip_on=False)
ax.text(1.0, -3.5, "one period (3 columns)", fontsize=6, ha="left", va="bottom")
st.note(ax, f"{nc} cols $\\times$ {nr} rows, low-$y$ end;\nlag-3 column agreement "
            f"{ext[str(t0)]['col_lag_agreement']['3']:.3f}", y=-.02)

# (c) predicted vs measured d|x|^2/dt
ax = fig.add_subplot(gs[1, 2])
xs = np.arange(2)
ax.plot(xs, [pm[t]["predicted"] for t in T0S], "o", mfc="none", mec=st.INK, ms=6, label=r"predicted $|x|/2L$")
ax.plot(xs, [pm[t]["measured"] for t in T0S], "o", color=st.ANT, ms=3.2, label="measured")
for i, t in enumerate(T0S):
    ax.text(i, .5, f"pred. {pm[t]['predicted']:.2f}\nmeas. {pm[t]['measured']:.2f}", ha="center", va="top", fontsize=6)
ax.set_xticks(xs, [f"${t / 1e10:.2f}$\n$L={pm[t]['L']:,}$\n$|x|={pm[t]['abs_x']:,.0f}$" for t in T0S], fontsize=6)
ax.set_xlim(-.6, 1.6); ax.set_ylim(0, 1.45)
ax.set_xlabel(r"dump time ($\times10^{10}$)", labelpad=2); ax.set_ylabel(r"$d|x|^2/dt$")
ax.legend(loc="upper center", bbox_to_anchor=(.5, 1.0), handletextpad=.3)
ax.set_title("(d) sweep model", fontsize=8)

st.save(fig, OUT / "late_switch.pdf")
print(json.dumps(pm, indent=1), onset)
