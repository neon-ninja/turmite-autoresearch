"""Static check: every repo-relative path literal under experiments/ in scripts/notes resolves (placeholders -> wildcards).
Outputs that do not exist yet are accepted if their parent directory does.   usage: python3 check_paths.py [--root DIR]"""
import re, sys, glob, os
from pathlib import Path
a = sys.argv[1:]
R = Path(a[a.index("--root") + 1]) if "--root" in a else Path(__file__).resolve().parents[2]
PAT = re.compile(r"experiments/[A-Za-z0-9_\-./{}*?%$:<>\[\]NR]+")
bad = []; n = 0
files = [p for pat in ("experiments/20*/scripts/*", "experiments/_lib/*.py", "research-outputs/*/*.py", "web/build.py", "examples/*.rs", "tests/*.rs")
         for p in glob.glob(str(R / pat))]
for f in sorted(files):
    try: text = Path(f).read_text(encoding="utf-8")
    except Exception: continue
    for m in PAT.finditer(text):
        lit = m.group(0).rstrip(".:,")
        if "<" in lit or lit.endswith("/"): lit = lit.rstrip("/")
        g = re.sub(r"\{[^}]*\}|\$\{?\w+\}?|%\d*\w|NN|\?\?", "*", lit)
        g = g.replace("[", "*").replace("]", "*")
        n += 1
        if glob.glob(str(R / g)): continue
        if (R / g).parent.exists() or glob.glob(str(R / os.path.dirname(g))): continue
        bad.append((os.path.relpath(f, R), lit))
print(f"checked {n} literals, {len(bad)} unresolved")
for f, l in bad: print(f"  {f}: {l}")
