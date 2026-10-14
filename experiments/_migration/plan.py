"""Build experiments/_migration/map.tsv: legacy path -> new path. Run (WSL): python3 experiments/_migration/plan.py"""
import os, subprocess, sys, datetime, collections
from pathlib import Path
sys.path.insert(0, os.path.dirname(__file__))
from families import family, TOPIC

R = Path(__file__).resolve().parents[2]
LIBS = {"analyse_probe.py", "calib_kd.py", "mixing.py", "tether.py"}
SKIP = {"__pycache__", "C:UsersUser"}

entries = []  # (old, family, sub)  sub in scripts/results/<dumpname>/_lib
def is_script(n): return n.endswith((".py", ".sh"))

for n in sorted(os.listdir(R / "data")):
    if n in SKIP: continue
    if n in LIBS: entries.append((f"data/{n}", "_lib", "")); continue
    entries.append((f"data/{n}", family(n), "scripts" if is_script(n) else "results"))
for n in sorted(os.listdir(R / "calib")): entries.append((f"calib/{n}", "calib", "results"))
for n in sorted(os.listdir(R / "occ")): entries.append((f"occ/{n}", "occupancy", "scripts" if is_script(n) else "results"))
for n in sorted(os.listdir(R / "snaps")): entries.append((f"snaps/{n}", "snapshots", "scripts" if is_script(n) else "results"))
for n in sorted(os.listdir(R / "dumps")):
    entries.append((f"dumps/{n}", "memory" if n.startswith("memory") else "knockout", "dumps"))
DUMPFAM = {"dumps_f": "tether", "dumps_e1": "e1", "dumps_e5": "e5", "dumps_e6": "e6",
           "knock": "knockout", "knock_e1": "e1", "knock_e5": "e5"}
for d, f in DUMPFAM.items():
    if (R / d).exists(): entries.append((d, f, "WHOLE"))

# earliest date per family: first git add of a member, else mtime
first = {}
out = subprocess.run(["git", "log", "--diff-filter=A", "--name-only", "--format=%x00%as", "--", "data", "calib", "occ", "snaps", "cell1"],
                     cwd=R, capture_output=True, text=True).stdout
cur = None
for line in out.splitlines():
    if line.startswith("\0"): cur = line[1:]
    elif line.strip() and cur: first[line.strip()] = min(first.get(line.strip(), cur), cur)
def date_of(old):
    p = R / old
    cands = [v for k, v in first.items() if k == old or k.startswith(old + "/")]
    if cands: return min(cands)
    m = os.path.getmtime(p) if p.exists() else 0
    return datetime.date.fromtimestamp(m).isoformat()
fdate = {}
for old, fam, _ in entries:
    d = date_of(old)
    fdate[fam] = min(fdate.get(fam, d), d)

def folder(fam):
    if fam == "_lib": return "experiments/_lib"
    t = TOPIC.get(fam, "")
    return f"experiments/{fdate[fam]}-{fam}" + (f"-{t}" if t and t != fam else "")

rows = []
for old, fam, sub in entries:
    base = os.path.basename(old)
    if fam == "_lib": new = f"experiments/_lib/{base}"
    elif sub == "WHOLE": new = f"{folder(fam)}/{old}"        # keep dumps_*/knock* dir name (gitignored pattern)
    elif sub == "dumps": new = f"{folder(fam)}/dumps/{base}"
    else: new = f"{folder(fam)}/{sub}/{base}"
    rows.append((old, new, fam))
(R / "experiments/_migration/map.tsv").write_text("".join(f"{o}\t{n}\t{f}\n" for o, n, f in rows))
fams = collections.Counter(f for _, _, f in rows)
for f, c in sorted(fams.items()): print(f"{c:4d} {folder(f)}")
