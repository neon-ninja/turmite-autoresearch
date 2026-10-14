"""Rewrite legacy path references (data/..., dumps*/..., knock*/..., calib/, occ/, snaps/) using map.tsv.
usage: python3 experiments/_migration/rewrite.py [--apply]   (dry run by default; run BEFORE the files are moved)"""
import os, re, sys, glob, collections
from pathlib import Path
sys.path.insert(0, os.path.dirname(__file__))
from families import family, TOPIC  # noqa

R = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path(__file__).resolve().parents[2]
APPLY = "--apply" in sys.argv
MAP = {}
for line in (R / "experiments/_migration/map.tsv").read_text().splitlines():
    o, n, f = line.split("\t"); MAP[o] = (n, f)
FOLDER = {}  # family -> folder
for o, (n, f) in MAP.items():
    FOLDER[f] = n.split("/")[0] + "/" + n.split("/")[1]

PAT = re.compile(r"(?<![\w.\-/])(?P<abs>(?:/mnt/f/Code/turmite/)?)(?P<root>data|calib|occ|snaps|dumps(?:_[a-z0-9]+)?|knock(?:_[a-z0-9]+)?)/(?P<rest>[^\s\"'`),;:\]]*)")
SPECIAL = re.compile(r"[*{%$]")
report = collections.defaultdict(list)

def sub_for(comp):
    return "scripts" if comp.endswith((".py", ".sh")) else "results"

def resolve(root, rest, path):
    """return replacement string for root/rest, or None to leave unchanged"""
    # strip trailing sentence punctuation from rest
    tail = ""
    while rest and rest[-1] in ".,;:":
        tail = rest[-1] + tail; rest = rest[:-1]
    if root.startswith(("dumps", "knock")):
        key = root
        if root == "dumps":
            comp = rest.split("/")[0]
            fam = "memory" if comp.startswith("memory") else "knockout"
            return f"{FOLDER[fam]}/dumps/{rest}{tail}"
        for o, (n, f) in MAP.items():
            if o == root: return f"{n}/{rest}{tail}" if rest else f"{n}/{tail}"
        return None
    if not rest:
        return None
    comp, _, remain = rest.partition("/")
    remain = ("/" + remain) if _ else ""
    old = f"{root}/{comp}"
    if old in MAP:
        new = MAP[old][0]
        return new + remain + tail
    # not an existing top-level entry: infer from the name
    if root == "data":
        pre = SPECIAL.split(comp)[0]
        if not pre:
            report["UNRESOLVED"].append((path, f"{root}/{rest}")); return None
        fam = "_lib" if comp in ("analyse_probe.py", "calib_kd.py", "mixing.py", "tether.py") else family(pre)
        if fam == "_lib": new = f"experiments/_lib/{comp}"
        elif fam not in FOLDER:
            report["UNRESOLVED"].append((path, f"{root}/{rest}")); return None
        else: new = f"{FOLDER[fam]}/{sub_for(comp)}/{comp}"
        report["INFERRED"].append((path, f"{root}/{rest} -> {new}"))
        return new + remain + tail
    if root in ("occ", "snaps", "calib"):
        fam = {"occ": "occupancy", "snaps": "snapshots", "calib": "calib"}[root]
        new = f"{FOLDER[fam]}/{sub_for(comp) if root != 'calib' else 'results'}/{comp}"
        report["INFERRED"].append((path, f"{root}/{rest} -> {new}"))
        return new + remain + tail
    report["UNRESOLVED"].append((path, f"{root}/{rest}"))
    return None

def rewrite_text(text, path):
    def repl(m):
        out = resolve(m["root"], m["rest"], path)
        if out is None: return m.group(0)
        return out
    return PAT.sub(repl, text)

TARGETS = []
for pat in ["data/*.py", "data/*.sh", "occ/*.py", "snaps/*.py", "examples/*.rs", "src/*.rs", "src/bin/*.rs", "tests/*.rs",
            "web/*.py", "web/*.js", "web/*.html", "notes/*.md", "AGENTS.md", "research-outputs/*/*.py", "research-outputs/*/*.typ",
            "research-outputs/*/*.md", "experiments/20*/README.md", "experiments/20*/scripts/*", "experiments/*.md", "calib/**/*.sh", "calib/*.sh", "calib/*.py"]:
    TARGETS += glob.glob(str(R / pat), recursive=True)
changed = 0
for t in sorted(set(TARGETS)):
    p = Path(t); rel = p.relative_to(R).as_posix()
    try: s = p.read_text(encoding="utf-8")
    except UnicodeDecodeError: continue
    new = rewrite_text(s, rel)
    if new != s:
        changed += 1
        if APPLY: p.write_text(new, encoding="utf-8", newline="")
print(f"{'APPLIED' if APPLY else 'dry run'}: {changed} files changed of {len(set(TARGETS))}")
for k, v in report.items():
    print(f"== {k} ({len(v)})")
    for path, what in v[:200]: print(f"  {path}: {what}")
