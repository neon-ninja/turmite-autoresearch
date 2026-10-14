"""Rebuild presentation assets only; existing measurements are never rerun.

Run from the repository root: uv run research-outputs/cube-root-growth-paper/rebuild_figures.py
Logs and input/source hashes are written beside the figures.
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "research-outputs/cube-root-growth-paper/fig"
SCRIPTS = ["figures.py", "gallery.py", "textbook_figs.py", "memory_figures.py"]
INPUTS = [
    "experiments/2026-10-13-probe-growth-census/results/probe_summary.csv", "experiments/2026-10-13-calib-instrument-calibration/results/calib_rotor.csv", "experiments/2026-10-13-probe-growth-census/results/cube_stats.json",
    "experiments/2026-10-13-probe-growth-census/results/c_model.csv", "experiments/2026-10-13-probe-growth-census/results/probe_all.csv", "experiments/2026-10-13-calib-instrument-calibration/results/calib_rw.csv",
    "experiments/2026-10-13-census-sweeps/results/sweep_v2.csv", "experiments/2026-10-13-census-sweeps/results/census.csv", "experiments/2026-10-13-memory-walkers/dumps/memory_paths.csv",
    "experiments/2026-10-13-memory-walkers/results/memory_walkers_pilot/combined.csv", "experiments/_lib/analyse_probe.py",
    "research-outputs/cube-root-growth-paper/style.py", "research-outputs/cube-root-growth-paper/main.typ", "research-outputs/cube-root-growth-paper/numbers.json", "research-outputs/cube-root-growth-paper/refs.bib",
    "experiments/2026-10-13-transport-forecast/results/transport_forecast.json",
] + [f"research-outputs/cube-root-growth-paper/{name}" for name in SCRIPTS]


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    os.chdir(ROOT)
    gal = Path(os.environ.get("GAL", Path.home() / "gal")).resolve()
    snapshots = [gal / f"{r}.bin" for r in
                 ("RL", "RLL", "RLR", "RRLL", "RLLR", "RRLRLLRLRRLR", "RRLRLRLLLLLL")]
    snapshots += [gal / f"RLL_k{k}.bin" for k in (14, 17, 20, 23)]
    paths = [ROOT / p for p in INPUTS] + snapshots
    recipe = {
        "command": "uv run research-outputs/cube-root-growth-paper/rebuild_figures.py",
        "git_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
        "working_tree": "Source hashes identify uncommitted edits.",
        "python": sys.version,
        "snapshot_directory": str(gal),
        "seeds": {"rotor_illustration": 1, "memory_paths": 301,
                  "gallery": "deterministic blank board"},
        "sha256": {str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p): digest(p)
                   for p in paths},
    }
    with (OUT / "rebuild.log").open("w") as log:
        for script in SCRIPTS:
            log.write(f"uv run research-outputs/cube-root-growth-paper/{script}\n")
            log.flush()
            subprocess.run([sys.executable, f"research-outputs/cube-root-growth-paper/{script}"], check=True,
                           stdout=log, stderr=subprocess.STDOUT)
        command = ["typst", "compile", "--root", ".", "research-outputs/cube-root-growth-paper/main.typ", "research-outputs/cube-root-growth-paper/main.pdf"]
        log.write(" ".join(command) + "\n")
        log.flush()
        subprocess.run(command, check=True, stdout=log, stderr=subprocess.STDOUT)
    recipe["outputs_sha256"] = {p.name: digest(p) for p in sorted(OUT.glob("*.pdf"))}
    recipe["outputs_sha256"]["main.pdf"] = digest(ROOT / "research-outputs/cube-root-growth-paper/main.pdf")
    (OUT / "rebuild_recipe.json").write_text(json.dumps(recipe, indent=2) + "\n")


if __name__ == "__main__":
    main()
