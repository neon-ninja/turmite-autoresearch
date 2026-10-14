"""One-off: convert the kappa-rule-space scripts (ROOT / "data" / name built at run time) to a runtime lookup."""
import re
base = "experiments/2026-10-kappa-rule-space/scripts/"
def edit(fn, f):
    s = open(base + fn).read(); t = f(s); open(base + fn, "w").write(t); print(fn, "changed" if s != t else "UNCHANGED")
def kc(s):
    s = s.replace("(read-only use of data/)", "(read-only use of the pre-restructure data, now in experiments/*/results/)")
    s = s.replace('sys.path.insert(0, str(ROOT / "data"))\nfrom analyse_probe import per_rule  # noqa: E402',
'''sys.path.insert(0, str(ROOT / "experiments" / "_lib"))
from analyse_probe import per_rule  # noqa: E402


def legacy(name):
    """Path of a pre-restructure data/ file, which now lives in some experiments/<exp>/results/ (not in this experiment)."""
    hits = [p for p in sorted(ROOT.glob(f"experiments/*/results/{name}")) if EXP not in p.parents]
    if len(hits) != 1:
        raise FileNotFoundError(f"{name}: expected exactly one match under experiments/*/results, got {hits}")
    return hits[0]''')
    return s
edit("kcommon.py", kc)
for fn in ("kcommon.py", "w0_build.py", "w2_stationarity.py", "w7_emergence.py"):
    edit(fn, lambda s: re.sub(r'ROOT / "data" / ("[^"]+"|f)', r'legacy(\1)', s))
