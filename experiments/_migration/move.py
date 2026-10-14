"""Move legacy files to the locations in map.tsv.  usage: python3 experiments/_migration/move.py [--root DIR]
Resumable: entries whose source is gone and destination exists are skipped. Refuses to overwrite."""
import os, sys
from pathlib import Path

args = sys.argv[1:]
R = Path(args[args.index("--root") + 1]) if "--root" in args else Path(__file__).resolve().parents[2]
moved = skipped = missing = 0
for line in (R / "experiments/_migration/map.tsv").read_text().splitlines():
    old, new, _ = line.split("\t")
    s, d = R / old, R / new
    if not s.exists():
        if d.exists(): skipped += 1
        else: missing += 1; print("MISSING", old)
        continue
    if d.exists(): sys.exit(f"refusing to overwrite {new}")
    d.parent.mkdir(parents=True, exist_ok=True)
    os.rename(s, d); moved += 1
print(f"moved {moved}, already done {skipped}, missing {missing}")
