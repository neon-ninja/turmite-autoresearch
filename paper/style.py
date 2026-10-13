"""Shared visual language for every figure in main.typ. Import, call apply(), use the names below.

Colours carry a fixed meaning across the paper:
  ANT / blob rules   blue          ROTOR walker   red         RANDOM walk (control)  grey
  LOG-growing rules  teal          builders       brown       HIGHWAY builders       purple
  EDGE cells         black outline (never a fill: cells keep their own colour underneath)
  theory / guides    grey dashed   (slope guides, the cone, y = x)
Cell states (colour 0 .. n-1 of a rule) use CELL, a separate qualitative set with no red, so a red mark always
means "rotor" and black always means "edge / ant". Colour 0 is light grey when visited; never-visited ground is white.
Conventions: rule names in monospace; full-width figures 6.5 in wide; 8 pt text, 6.5 pt for notes/legends; no top/right
spines; legends unframed; log axes in base 2 for t, base 10 for sizes."""
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from cycler import cycler

W = 6.5            # full text width, inches
W_HALF = 3.2

ANT = "#3b4cc0"
ROTOR = "#d1495b"
RANDOM = "#8a8a8a"
LOG = "#1fa187"
BUILDER = "#8c6d31"
HIGHWAY = "#7a3b8f"
EDGE = "#111111"
GUIDE = dict(color="#777777", ls="--", lw=.8)
INK = "#222222"

# Cell-state palette (index = colour). 0 is the visited-blank grey.
CELL = ["#e3e3e3", "#1fa187", "#e8a33d", "#7a3b8f", "#5fb3d9", "#8c6d31", "#b8d85a", "#444444", "#e377c2", "#3b4cc0", "#17becf", "#bcbd22"]

# Several ant rules in one plot: a blue-violet family (the rotor is red, log growth teal).
RULES = ["#3b4cc0", "#5fb3d9", "#7a3b8f", "#8c6d31", "#444444", "#9aa5ea"]
RULE_COLOUR = {"RLL": RULES[0], "RLR": RULES[1], "RRLL": RULES[2], "RRLRLRLLLLLL": LOG}


def apply():
    plt.rcParams.update({
        "font.size": 8, "axes.titlesize": 8, "axes.labelsize": 8, "legend.fontsize": 6.5, "legend.frameon": False,
        "xtick.labelsize": 7, "ytick.labelsize": 7, "axes.spines.top": False, "axes.spines.right": False,
        "axes.edgecolor": INK, "axes.labelcolor": INK, "text.color": INK, "xtick.color": INK, "ytick.color": INK,
        "axes.prop_cycle": cycler(color=RULES), "lines.linewidth": 1.0, "figure.dpi": 150, "savefig.dpi": 300,
    })


def short(rule, keep=6):
    """Rule name for titles: long rules are abbreviated with an ellipsis."""
    return rule if len(rule) <= keep + 3 else rule[:keep] + "…" + rule[-3:]


def rule_title(ax, rule, extra="", pad=2):
    ax.set_title(short(rule) + extra, fontsize=8, family="monospace", pad=pad)


def cell_rgb(n):
    """(n, 3) float array: the colour of each cell state, 0 = visited blank."""
    return np.array([[int(CELL[c % len(CELL)][i:i + 2], 16) for i in (1, 3, 5)] for c in range(n)], float) / 255


def cells_img(col, vis, n):
    """RGB image of a colour field; never-visited cells are white."""
    return np.where(vis[..., None], cell_rgb(n)[col % n], 1.0)


def clean(ax):
    """Image panel: no ticks, no frame."""
    ax.set_xticks([]); ax.set_yticks([])
    for s in ax.spines.values():
        s.set_visible(False)


def note(ax, text, y=-0.03):
    ax.text(0.5, y, text, transform=ax.transAxes, ha="center", va="top", fontsize=6.5)


def guide_line(ax, x, y, **kw):
    ax.plot(x, y, **{**GUIDE, **kw})
