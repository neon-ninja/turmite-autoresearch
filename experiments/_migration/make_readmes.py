"""Write a README.md stub into each migrated experiment folder that lacks one (derived from file names and script docstrings)."""
import ast, os, re, collections
from pathlib import Path
R = Path(__file__).resolve().parents[2]
rows = [l.split("\t") for l in (R / "experiments/_migration/map.tsv").read_text().splitlines()]
oldof = collections.defaultdict(list)
for old, new, fam in rows:
    parts = new.split("/")
    if parts[1] != "_lib": oldof[parts[1]].append(old)
def first_doc(p):
    try:
        if p.suffix == ".py":
            d = ast.get_docstring(ast.parse(p.read_text(encoding="utf-8")))
            if d: return d.strip().splitlines()[0][:160]
        for line in p.read_text(encoding="utf-8").splitlines()[:6]:
            if line.startswith("#") and not line.startswith(("#!", "# ///")) and len(line) > 4: return line.lstrip("# ").strip()[:160]
    except Exception: pass
    return ""
n = 0
for d in sorted((R / "experiments").glob("20*/")):
    if (d / "README.md").exists() or d.name not in oldof: continue
    scripts = sorted((d / "scripts").glob("*")) if (d / "scripts").exists() else []
    results = sorted(os.listdir(d / "results")) if (d / "results").exists() else []
    dumps = [x for x in os.listdir(d) if x.startswith(("dumps", "knock"))]
    lines = [f"# {d.name[11:]}", "",
             "Migrated legacy investigation (2026-10-14). Its files used to sit in the flat `data/` directory (and `calib/`, `occ/`, `snaps/`, `dumps*`, `knock*` where listed) with the label prefix below; "
             "`experiments/_migration/map.tsv` gives the exact old -> new path of every file. This stub was generated from file names and script docstrings. "
             "It is **not** the original write-up: the question, timestamped hypotheses, commands and results are in `notes/` (mainly `notes/frontier-results.md`, `notes/frontier-status.md`, `notes/research-frontier*.md`; search for the label), "
             "and the producing command is usually in the script docstring or a `run_*.sh` here.", "",
             f"Old labels / prefixes: {', '.join(sorted({re.sub(r'^[a-z]+/', '', o).split('_')[0].split('.')[0] for o in oldof[d.name]})[:12])}", ""]
    if scripts:
        lines += ["## Scripts", ""] + [f"- `scripts/{s.name}`" + (f": {first_doc(s)}" if first_doc(s) else "") for s in scripts] + [""]
    if results:
        sh = [r for r in results if not r.endswith((".log", ".done"))]
        lines += ["## Results", "", f"{len(results)} entries in `results/` ({len(results) - len(sh)} are `.log`/`.done` markers); csv/json/txt outputs, `*_stats.txt` hold the summaries quoted in the notes.", ""]
    if dumps: lines += ["## Bulk output (gitignored)", ""] + [f"- `{x}/`" for x in dumps] + [""]
    (d / "README.md").write_text("\n".join(lines))
    n += 1
print("wrote", n, "READMEs")
