"""One-off: repair AGENTS.md / experiments/README.md after the path rewrite (generic `dumps/` prose) and describe the finished migration."""
import re
KO = "experiments/2026-10-12-knockout-maps/dumps/"
MEM = "experiments/2026-10-13-memory-walkers/dumps/memory_paths.csv"
def sub(path, pairs, regex=()):
    s = open(path).read()
    for a, b in pairs:
        if a not in s: print("NOT FOUND in", path, ":", a[:70])
        s = s.replace(a, b)
    for a, b in regex: s = re.sub(a, b, s, flags=re.S)
    open(path, "w").write(s)

sub("AGENTS.md", [
 ("write to a log under `data/`, don't block", "write to a log under the experiment's `results/`, don't block"),
 ("in `notes/` or `data/` must be traceable", "in `notes/` or `experiments/*/results/` must be traceable"),
 (f"Reads committed results, not `{KO}`", "Reads committed results, not `dumps/`"),
 ("# analysis over data/*.csv", "# analysis over the census CSVs"),
 ("`dumps*/`, `knock*/` or `scratch/`, never in the repo root, `data/` or\n  `research-outputs/`.", "`dumps*/`, `knock*/` or `scratch/`, never in the repo root or\n  `research-outputs/`."),
 ("`Path(__file__).resolve().parents[2]`", "`Path(__file__).resolve().parents[3]` (script in `experiments/<slug>/scripts/`)"),
 ("Legacy root-level `dumps*`, `knock*`, `data/`, `calib/` are frozen: read\n  from them, don't add to them.", "The former flat `data/`, `calib/`, `occ/`, `snaps/`, `dumps*`, `knock*` no longer\n  exist: their contents were migrated into experiment folders (old path -> new path in\n  `experiments/_migration/map.tsv`; helpers in `experiments/_lib/`)."),
 ("computed from `experiments/*/results/` (or\n  legacy `data/`) by scripts in `research-outputs/`, which should not read `" + KO + "`\n  (known exception: `memory_figures.py` reads `" + MEM + "`).",
  "computed from `experiments/*/results/` by scripts in\n  `research-outputs/`, which should not read `dumps/` (known exception: `memory_figures.py`\n  reads `" + MEM + "`)."),
 ("`results/`/`data/`, run top to bottom", "`results/`, run top to bottom"),
])
# layout table: drop the rows of directories that no longer exist, add _lib/_migration
s = open("AGENTS.md").read()
s = re.sub(r"\| `data/` \| \*\*Legacy\*\*[^\n]*\n\| `calib/` \|[^\n]*\n", "| `experiments/_lib/` | Shared Python helpers imported by several experiments (`analyse_probe.py`, `mixing.py`, `calib_kd.py`, `tether.py`); scripts add it with `sys.path.insert(0, \"experiments/_lib\")` |\n| `experiments/_migration/` | One-off tooling and `map.tsv` (old path -> new path) from the 2026-10-14 restructure of the flat `data/` layout |\n", s)
s = re.sub(r"\| `occ/`, `snaps/` \|[^\n]*\n", "", s)
open("AGENTS.md", "w").write(s)

sub("experiments/README.md", [
 (f"  {KO}        bulk raw output (ignored by git; see below)", "  dumps/        bulk raw output (ignored by git; see below)"),
 (f"Bulk output goes in `{KO}` (or", "Bulk output goes in `dumps/` (or"),
 (f"Never `{KO}` or", "Never `dumps/` or"),
 (f"paper code should not read `{KO}`.", "paper code should not read `dumps/`."),
 ("committed `data/`. Never", "committed results from other experiments. Never"),
 ("`Path(__file__).resolve().parents[2]`", "`Path(__file__).resolve().parents[3]` (the script sits in `experiments/<slug>/scripts/`)"),
])
s = open("experiments/README.md").read()
i = s.index("## Legacy material")
s = s[:i] + """## Migrated legacy material

Work done before this layout used flat directories (`data/`, `calib/`, `occ/`,
`snaps/`, plus gitignored `dumps*/` and `knock*/`), with experiment labels
(`a1`..`w4`, `h1e11`, `t1b`, ...) as file-name prefixes. On 2026-10-14 it was
split by label into `experiments/<date>-<label>[-topic]/{scripts,results}/`
(bulk dumps and knockout maps into the matching `dumps*/`, `knock*/`). Shared
helper modules are in `experiments/_lib/`.

- `experiments/_migration/map.tsv` maps every old top-level path to its new one
  (use it to resolve paths in historical logs, `.log` files and old notes, which
  still quote the old locations); `README.md` of each folder has the recipe.
- The tooling that did it (`plan.py`, `rewrite.py`, `fixups*.py`, `move.py`,
  `check_paths.py`) is kept there for audit; do not rerun it.
- Dates in folder names are the day the label's first file was committed, which for
  most legacy labels is the day of the initial commit, not of the experiment.
- Scripts still read some other experiment's `results/` (e.g. the census CSVs in
  `2026-10-13-census-sweeps/results/`); such reads are explicit literal paths.
- Cross-experiment dumps are shared: `2026-10-12-knockout-maps/dumps/` holds the
  RULE.grid/.traj fields that the knockout, patterns, cell1, e* and hot-tile
  experiments read.
"""
open("experiments/README.md", "w").write(s)
