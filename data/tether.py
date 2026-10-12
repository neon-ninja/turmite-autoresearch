"""Radial drift and local diffusion, binned by distance from home.

For an unbiased walk the mean Cartesian displacement projected on the outward
unit vector at the start point is zero. A confining pull makes it negative.
With drift v(r) and diffusion D(r) (MSD = 4 D tau in 2D), zero net flux gives
    d ln(density) / dr = v / D,
so the occupancy profile can be predicted from local motion alone.
"""
import sys
from pathlib import Path

import numpy as np

S = 4096


def radial(rule, dumps, tau, nbins=24):
    p = np.fromfile(Path(dumps) / f"{rule}.traj", dtype="<i2").reshape(-1, 2).astype(float)
    a, b = p[:-tau], p[tau:]
    r = np.hypot(a[:, 0], a[:, 1])
    ok = r > 0.5
    u = a[ok] / r[ok, None]
    d = b[ok] - a[ok]
    v = (d * u).sum(axis=1)                      # outward displacement
    m2 = (d ** 2).sum(axis=1)
    rr = r[ok]
    edges = np.quantile(rr, np.linspace(0, 0.995, nbins + 1))
    idx = np.digitize(rr, edges) - 1
    out = []
    for i in range(nbins):
        s = idx == i
        if s.sum() < 2000:
            continue
        out.append((rr[s].mean(), v[s].mean() / tau, m2[s].mean() / (4 * tau), s.sum()))
    return np.array(out)


def density_profile(rule, dumps, nbins=24):
    """Time per cell against r, from the same trajectory."""
    p = np.fromfile(Path(dumps) / f"{rule}.traj", dtype="<i2").reshape(-1, 2).astype(int)
    r = np.hypot(p[:, 0], p[:, 1])
    cells, counts = np.unique(p, axis=0, return_counts=True)
    rc = np.hypot(cells[:, 0], cells[:, 1])
    edges = np.quantile(r, np.linspace(0, 0.995, nbins + 1))
    out = []
    for lo, hi in zip(edges[:-1], edges[1:]):
        s = (rc >= lo) & (rc < hi)
        if s.sum() < 5:
            continue
        out.append(((lo + hi) / 2, counts[s].mean()))
    return np.array(out)


if __name__ == "__main__":
    dumps = sys.argv[1]
    for rule in sys.argv[2:]:
        prof = density_profile(rule, dumps)
        slope_obs = np.polyfit(prof[:, 0], np.log(prof[:, 1]), 1)[0]
        print(f"\n{rule}: observed d ln(time per cell)/dr = {slope_obs:+.3f} per cell (fit over all r)")
        for tau in (2, 4, 8):
            rad = radial(rule, dumps, tau)
            ratio = rad[:, 1] / rad[:, 2]
            w = rad[:, 3]
            print(f"  tau={tau}: v/D weighted mean {np.average(ratio, weights=w):+.3f};  "
                  f"by r: " + " ".join(f"{x:.0f}:{y:+.2f}" for x, y in zip(rad[::4, 0], ratio[::4])))
