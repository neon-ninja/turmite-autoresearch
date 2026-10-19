"""Small byte-for-byte CLI checks; writes only to the explicit scratch directory."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=True)
    target = Path(os.environ.get("CARGO_TARGET_DIR", root / "target"))
    if not target.is_absolute():
        target = root / target
    cases = [
        ("sweep", ["2", "6", "32768", "256"], None),
        ("probe", ["14", "256"], "RL\nRLR\nLLRR\nRLR@1\nRLR@2\nrotor:1\nrotor:2\nrw:1\nrw:2\n"),
    ]
    recipe = {
        "commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
        "build": "cargo build --release --locked --bin sweep --bin probe",
        "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
        "cargo_lock_sha256": sha256(root / "Cargo.lock"),
        "script_sha256": sha256(Path(__file__)),
        "runs": [],
    }
    failed = []
    for name, flags, stdin in cases:
        binary = target / "release" / name
        outputs = []
        for threads in (1, 3):
            command = [str(binary), *flags]
            result = subprocess.run(command, input=stdin, text=True, capture_output=True,
                                    cwd=root, env={**os.environ, "RAYON_NUM_THREADS": str(threads)}, check=True)
            path = out / f"{name}-{threads}.csv"
            path.write_bytes(result.stdout.encode())
            (out / f"{name}-{threads}.log").write_text(result.stderr)
            # Prevent vacuous agreement if the CLI emits only a header.
            if len(result.stdout.splitlines()) < 2:
                failed.append(f"{name}: no data rows")
            outputs.append(path.read_bytes())
            recipe["runs"].append({"command": command, "stdin": stdin,
                                   "RAYON_NUM_THREADS": threads, "binary_sha256": sha256(binary),
                                   "output": path.name, "output_sha256": sha256(path)})
        if outputs[0] != outputs[1]:
            failed.append(f"{name}: output differs between 1 and 3 threads")
    recipe["failures"] = failed
    (out / "recipe.json").write_text(json.dumps(recipe, indent=2) + "\n")
    if failed:
        raise SystemExit("; ".join(failed))
    print("PASS: sweep and seeded probe CSVs match at 1 and 3 Rayon threads")


if __name__ == "__main__":
    main()
