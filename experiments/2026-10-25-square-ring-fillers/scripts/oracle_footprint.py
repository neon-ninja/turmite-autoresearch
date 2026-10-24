"""Independent stdlib check of square_ring_fillers_footprint: read histogram, visited count, window new cells, first_gt,
fronts_after and final position, for short runs including a rule switch. Exit 1 on any mismatch.

usage: python3 experiments/2026-10-25-square-ring-fillers/scripts/oracle_footprint.py BIN
"""
import subprocess
import sys

SPECS = ["RLRRRL", "RRLRRRR", "RLLR", "RLRRRL>RLRRLL@100000#4", "RRLRRRR>RRLRRRL@120000#6", "RL"]
STEPS, GRID, WIN = 200000, 512, 65536


def ref(spec):
    gt = None
    if "#" in spec:
        spec, g = spec.split("#")
        gt = int(g)
    r2, t0 = None, None
    if ">" in spec:
        r1, rest = spec.split(">")
        r2, t0 = rest.split("@")
        t0 = int(t0)
    else:
        r1 = spec
    dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)]
    vis = {}
    c = GRID // 2
    x = y = c
    h = 0
    rule = r1
    reads = []
    newc = []
    visited = 1
    first = None
    fronts = []
    xmax = x
    t = 0
    stop = "budget"
    while t < STEPS:
        if t0 is not None and t == t0:
            rule = r2
        v = vis.get((x, y), 0)
        h = (h + (1 if rule[v % len(rule)] == "R" else 3)) % 4
        vis[(x, y)] = v + 1
        reads.append(min(v, 64))
        if gt is not None and first is None and t >= (t0 or 0) and v > gt:
            first = t
        x += dirs[h][0]
        y += dirs[h][1]
        t += 1
        if not (0 <= x < GRID and 0 <= y < GRID):
            stop = "escaped"
            newc.append(0)
            break
        fresh = vis.get((x, y), 0) == 0
        newc.append(1 if fresh else 0)
        visited += fresh
        if x > xmax:
            xmax = x
            if t0 is not None and t > t0 and len(fronts) < 8:
                fronts.append(t)
    w = reads[-WIN:]
    hist = {}
    for v in w:
        hist[v] = hist.get(v, 0) + 1
    rs = ";".join(f"{v}:{hist[v]}" for v in sorted(k for k in hist if k < 64))
    if 64 in hist:
        rs += (";" if rs else "") + f"64+:{hist[64]}"
    return dict(stop=stop, steps=str(t), visited=str(visited), win_new=str(sum(newc[-WIN:])), reads=rs,
                first_gt="" if first is None else str(first), fronts_after=";".join(map(str, fronts)))


def main():
    out = subprocess.run([sys.argv[1], str(STEPS), str(GRID), str(WIN)], input="\n".join(SPECS) + "\n",
                         capture_output=True, text=True, check=True).stdout.splitlines()
    head = out[0].split(",")
    bad = 0
    for line, spec in zip(out[1:], SPECS):
        row = dict(zip(head, line.split(",")))
        exp = ref(spec)
        diffs = [k for k in exp if row[k] != exp[k]]
        print(f"{spec}: {'OK' if not diffs else 'MISMATCH ' + str({k: (row[k], exp[k]) for k in diffs})}")
        bad += bool(diffs)
    sys.exit(1 if bad else 0)


main()
