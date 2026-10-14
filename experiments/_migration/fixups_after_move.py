"""Run AFTER move.py: the two scripts whose input/output paths are computed at run time.
usage: python3 experiments/_migration/fixups_after_move.py [--root DIR]"""
import re, sys
from pathlib import Path
args = sys.argv[1:]
R = Path(args[args.index("--root") + 1]) if "--root" in args else Path(__file__).resolve().parents[2]
MAP = {}
for line in (R / "experiments/_migration/map.tsv").read_text().splitlines():
    o, n, f = line.split("	"); MAP[o] = n
done = []
def edit(rel, fn):
    p = R / rel
    s = p.read_text(encoding="utf-8"); t = fn(s)
    if t != s:
        p.write_text(t, encoding="utf-8", newline=""); done.append(rel)

# --- the two scripts whose output/input paths are computed at run time ---
def regen(s):
    s = s.replace('run() { out=$1; shift; uv run "$@" 2>&1 | grep -v "Reading inline" > data/$out.$ext; }',
                  'run() { out=$1; shift; d=$(dirname "$(dirname "$1")")/results; uv run "$@" 2>&1 | grep -v "Reading inline" > $d/$out.$ext; }  # output lands in the results/ of the script\'s experiment')
    probe_results = MAP["data/seed_uncertainty.txt"].rsplit("/", 1)[0]
    s = re.sub(r"^run seed_uncertainty (\S+)$", rf'uv run \1 2>&1 | grep -v "Reading inline" > {probe_results}/seed_uncertainty.$ext', s, flags=re.M)
    s = s.replace("writes data/NAME.OUTSUFFIX instead of data/NAME.txt", "writes results/NAME.OUTSUFFIX instead of results/NAME.txt")
    return s
edit(MAP["data/regen_frontier_stats.sh"], regen)
a4 = MAP["data/a4_shape_ants.csv"].rsplit("/", 1)[0]
edit(MAP["data/width_radius_refit.py"], lambda s: s.replace('f"data/{cohort}_shape_{family}.csv"', f'f"{a4}/{{cohort}}_shape_{{family}}.csv"'))
print("manual fixups done")
