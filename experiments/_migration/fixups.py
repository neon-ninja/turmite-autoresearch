"""Non-slash path forms that rewrite.py cannot see (ROOT / "dumps", D / "name", Path("knock_e1"), parent.parent, sys.path).
Run after rewrite.py --apply and before move.py.  usage: python3 experiments/_migration/fixups.py [--root DIR]"""
import re, sys
from pathlib import Path

args = sys.argv[1:]
R = Path(args[args.index("--root") + 1]) if "--root" in args else Path(__file__).resolve().parents[2]
MAP = {}
for line in (R / "experiments/_migration/map.tsv").read_text().splitlines():
    o, n, f = line.split("\t"); MAP[o] = n
KNOCKOUT = MAP["knock"].rsplit("/knock", 1)[0]          # experiments/<date>-knockout-maps
PATTERNS = MAP["data/patterns.py"].rsplit("/scripts/", 1)[0]
done = []

def edit(rel, fn):
    p = R / rel
    if not p.exists(): print("MISSING", rel); return
    s = p.read_text(encoding="utf-8"); t = fn(s)
    if t != s:
        p.write_text(t, encoding="utf-8", newline=""); done.append(rel)

def datanames(var, root_expr):
    """VAR / "name"  ->  VAR / "<repo-relative new path>"   (VAR is a directory that used to be data/)"""
    def fn(s):
        def r(m):
            name = m["name"]; key = f"data/{name}"
            return f'{m["pre"]}"{MAP[key]}"' if key in MAP else m.group(0)
        return re.sub(rf'(?P<pre>\b{var} / )"(?P<name>[^"/]+)"', r, s)
    return fn

# A/B: every migrated script
for src in sorted((R / "data").glob("*.py")):
    rel = f"data/{src.name}"
    def fn(s, lib=(src.name in {"analyse_probe.py", "calib_kd.py", "mixing.py", "tether.py"})):
        s = s.replace('sys.path.insert(0, "data")', 'sys.path.insert(0, "experiments/_lib")')
        s = s.replace("Path(__file__).resolve().parent.parent", "Path(__file__).resolve().parents[2 if lib else 3]")
        s = re.sub(r'ROOT / "dumps"', f'ROOT / "{KNOCKOUT}/dumps"', s)
        s = re.sub(r'ROOT / "knock"', f'ROOT / "{KNOCKOUT}/knock"', s)
        s = re.sub(r'ROOT / "synth"', f'ROOT / "{PATTERNS}/synth"', s)
        s = re.sub(r'Path\("(knock_e[15]|dumps_e[156]|dumps_f)"\)', lambda m: f'Path("{MAP[m[1]]}")', s)
        return s
    edit(rel, fn)

edit("data/mixing_calib.py", lambda s: s.replace('sys.path.insert(0, str(Path(__file__).parent))', 'sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "_lib"))'))
edit("data/cube_stats.py", lambda s: datanames("D", "")(s.replace("D = Path(__file__).resolve().parent", "D = Path(__file__).resolve().parents[3]")))
edit("data/followup_stats.py", lambda s: datanames("D", "")(s.replace('D = ROOT / "data"', "D = ROOT")))
def kf(s):
    s = s.replace('d = ROOT / "data"', "d = ROOT")
    s = s.replace('d.parent / "knock"', f'ROOT / "{KNOCKOUT}/knock"')
    return datanames("d", "")(s)
edit("data/knockout_followup.py", kf)

P = "research-outputs/cube-root-growth-paper"
def paper_compute(s):
    s = s.replace('D = R / "data"', "D = R  # inputs are addressed by repo-relative path (they live in experiments/*/results)")
    s = s.replace("missing or empty inputs in data/:", "missing or empty inputs:")
    s = datanames("D", "")(s)
    s = re.sub(r'jl\("(?P<n>[^"/]+)"\)', lambda m: f'jl("{MAP["data/" + m["n"]]}")' if "data/" + m["n"] in MAP else m.group(0), s)
    return s
edit(f"{P}/compute_numbers.py", paper_compute)
def paper_fig(s):
    s = s.replace('D = R / "data"\nsys.path.insert(0, str(D))', 'D = R\nsys.path.insert(0, str(R / "experiments" / "_lib"))')
    return datanames("D", "")(s)
edit(f"{P}/figures.py", paper_fig)
def rdata(s):
    return re.sub(r'R / "data" / "(?P<n>[^"/]+)"', lambda m: f'R / "{MAP["data/" + m["n"]]}"' if "data/" + m["n"] in MAP else m.group(0), s)
for f in ("gallery.py", "textbook_figs.py", "seed_uncertainty.py", "mixing_random.py"):
    edit(f"{P}/{f}", lambda s: rdata(s).replace('sys.path.insert(0, str(R / "data"))', 'sys.path.insert(0, str(R / "experiments" / "_lib"))'))

def web(s):
    s = s.replace('DATA = ROOT / "data"', "DATA = ROOT")
    s = datanames("DATA", "")(s)
    s = s.replace('ROOT / "occ" / "page_occ.json"', f'ROOT / "{MAP["occ/page_occ.json"]}"')
    s = s.replace('ROOT / "occ" / f"page_{r}.png"', f'ROOT / "{MAP["occ/colours.png"].rsplit("/", 1)[0]}" / f"page_{{r}}.png"')
    s = s.replace('(ROOT / "snaps").glob', f'(ROOT / "{MAP["snaps/RL.png"].rsplit("/", 1)[0]}").glob')
    return s
edit("web/build.py", web)

# main.typ reads the transport forecast relative to its own directory ("../../data/..." is skipped by the generic rewrite)
edit(f"{P}/main.typ", lambda s: s.replace('json("../../data/transport_forecast.json")', f'json("../../{MAP["data/transport_forecast.json"]}")'))
print("fixups applied to:", *done, sep="\n  ")

# folder names must not leak into substring tests on the path (experiment folder "...exponent-rotor-calibrated...")
edit("data/item4_stats.py", lambda s: s.replace('"rotor" in PATH', '"rotor" in PATH.rsplit("/", 1)[-1]'))
