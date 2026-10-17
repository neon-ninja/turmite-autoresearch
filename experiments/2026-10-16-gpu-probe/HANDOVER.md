# Handover: GPU probe (written 2026-10-16 ~21:02 NZDT, work paused at the user's request)

## State in one paragraph
`gpu_probe` works and is **byte-identical to the CPU `probe`** everywhere it was checked. The only unfinished
parts are the **clean timing** (P2), the final README/notebook write-up, running the rest of the test suite
after my `src/probe.rs` refactor, and committing. Nothing is committed. No job of mine is running.

## What exists (all uncommitted; stage explicit paths only, the tree has many unrelated changes)
| Path | What |
|---|---|
| `src/gpu/probe.cu` | kernels: 16-bit cell (colour byte, `unv+1` byte), windows closed in-kernel, per-lane `PRec` records |
| `src/gpu/probe.rs` | `GpuProber`, `ProbeJob { rule, patch_seed, max_log2 }`, `ProbeConfig`; scheduler copied from `GpuSweeper` |
| `src/gpu/mod.rs` | +3 lines: `mod probe; pub use probe::{...}` (sweep code untouched) |
| `src/bin/gpu_probe.rs`, `Cargo.toml` | CLI, same interface as `probe`; `[[bin]] gpu_probe`, `required-features = ["cuda"]` |
| `src/probe.rs`, `src/bin/probe.rs` | refactor: `WindowSums`, `finish_window`, `window_sums_exact`, `CSV_HEADER`, `csv_rows` shared by CPU and GPU; `ProbeResult` now `Clone, Debug` |
| `tests/gpu_probe.rs` | 5 tests (proptest vs CPU over 7 rigs, scheduling independence, starved pool, mid-window escape, error paths) |
| `experiments/2026-10-16-gpu-probe/` | README (design + predictions P1-P4 registered 20:49 before timing), NOTEBOOK.md, `scripts/spotcheck.sh`, `scripts/bench.sh`, `results/spotcheck*.txt` |
| `AGENTS.md`, `experiments/README.md` | tooling-table row and a `gpu_probe` section; the "probe not ported" line updated |

## Verified so far
- Old vs refactored CPU `probe` (saved binary `~/probe_old`): identical on 18 mixed specs of every walker kind, at `17 1024` and `15 8192`.
- `cargo test --release --features cuda --test gpu_probe`: 5/5 pass (109 s).
- Full scale: 24 specs (every 100th line of kappa-leadrun `specs.txt`) at `27 8192`: GPU == CPU byte for byte (`results/spotcheck.txt`).
- All 2400 specs on GPU == the existing kappa-leadrun CPU dump (`cmp` identical), `oom_requeues` 0. That output is in `scratch/` only.

## NOT done
1. **Clean timing.** The only GPU number so far (177 s wall, 161 s device path, 2400 lanes) is NOT a benchmark: the user's CPU batch was still
   running. The old CPU wall time (1231 s) is also contaminated by the GPU-sweep benchmarks, so it is not a baseline either.
   Run `bash experiments/2026-10-16-gpu-probe/scripts/bench.sh` (WSL, repo root; gpu1, cpu, gpu2 in sequence, about 25 min,
   writes `results/bench.log`) only on an idle machine, in the background. Check first: `ps -eo pid,args | grep -E '/release/|cargo'` and `nvidia-smi`.
   At about 20:58 a **different session** built this tree into `~/turmite-target-probeport` and ran `gpu_probe 27 8192 --lanes 2400`
   (results in `results/full_L2400*` of its own cwd, if it kept them). Ask the user whether that was them before benchmarking, and see if it
   holds numbers worth reusing. Everything it needs from me is in the tree as it stands.
2. Prediction check, to write into the README Results and a closing NOTEBOOK entry. If the clean ratio is about 177 s vs a ~20 min CPU run (about 7x),
   P2 (registered 2-6x, falsified at >= 4.9x) would be **falsified on the fast side**; say so plainly, do not edit the prediction. P3 held (no requeues).
   P4 (lane scaling) was not tested.
3. `cargo test --release` (plain) and `cargo test --release --features cuda --test gpu --test onset`: **not run since the `src/probe.rs` refactor**. Both are required before declaring done. They use all cores, so run them when no batch is live.
4. `notebook.ipynb` (from `experiments/_template/`, reads only committed `results/`: `spotcheck.txt`, `bench.log`, `bench_gpu*.json`), then execute it per `experiments/README.md`.
5. Commit (only if asked), explicit paths. `tests/*.proptest-regressions` if any appears (none so far).
6. Optional, unmeasured: two byte planes vs the 16-bit cell; one-byte packing for n <= 32 (halves pool use). Only worth doing if VRAM binds (estimate for the 2400 batch is 13 GB of 20).

## Things to know
- Reuse: `source experiments/2026-10-16-gpu-ant-sweep/scripts/env.sh` in every WSL build/run (sets `LD_LIBRARY_PATH`, `CARGO_TARGET_DIR=~/turmite-target`).
- The patch is initialised at tile allocation (it straddles four tiles at 8192, where the start is a tile corner); do not "simplify" this to the first tile.
- `gpu_probe` rejects non-ant specs up front; rotor/rw/sbg/pw/cookie/errw stay CPU-only (out of scope as asked).
- `ProbeStats.total_steps` counts the budget for non-escaped runs only (escaped runs add 0), so steps/s in `--stats` is a slight underestimate.
- Scratch files (`experiments/2026-10-16-gpu-probe/scratch/`, gitignored) hold the CPU/GPU CSVs; `~/quiet_649.sh` was my wait helper and can be deleted.
