# Handover: GPU probe (rewritten 2026-10-19; replaces the 2026-10-16 handover)

## State in one paragraph
`gpu_probe` (GPU port of `probe` for plain rules and `RULE@SEED`) is implemented, committed and byte-identical to the CPU `probe` on every
full-scale run so far. P1, P3 and P4 are measured and held. **P2's 3.84x speedup was measured on a stale binary** and must not be cited
as a HEAD result; a clean paired re-measure is the main open item. Nothing of mine is running.

## Commits (all on master)
`53ccccc` code (WIP) · `8266186` first benchmark, notebook · `8b0f1a6` P4 (subagent) · `0c48a66` P4 dedupe, fresh-build proptests.
Files: `src/gpu/probe.{rs,cu}`, `src/bin/gpu_probe.rs`, `tests/gpu_probe.rs`, refactor in `src/probe.rs`/`src/bin/probe.rs`,
`experiments/2026-10-16-gpu-probe/` (README, NOTEBOOK, notebook.ipynb, scripts, results).

## Results (details in README Results and NOTEBOOK)
| Prediction | Status | Evidence |
|---|---|---|
| P1 correctness | held | 24-spec spot check; all 2400 specs identical to the kappa-leadrun CPU dump, including 6/6 runs of the fresh HEAD build (sha 91c072f7) |
| P2 speed 2-6x | **measured on a stale binary** | 3.84x = CPU 681.6 s vs GPU 177 s, binary 4d3e395d built 2026-10-16 20:46, not 53ccccc |
| P3 memory | held | 0 requeues, pool about 2.41e6 tiles (my 1.6e6 estimate was about 50% low) |
| P4 lanes | held | device time 278.1 / 217.2 / 154.3 s at 1024 / 1536 / 2400 lanes, n = 2, spread <= 0.4 s. Mostly queueing; per-lane speed falls 23% |

Fresh build at 2400 lanes takes 156.4 s. Against 681.6 s that would be 4.4x, but the CPU `probe` binary (sha f1a309cf) came from the same
possibly stale target dir, so no HEAD speedup is claimed.

## The stale-build problem (root cause, so you do not repeat it)
Source files on `/mnt/f` kept mtimes of 2026-10-16 20:41-20:43 although their content changed later; the binary in `~/turmite-target` is dated
20:46, so cargo thought it was current and rebuilt nothing. `scripts/bench.sh` pipes its build through `| tail -1`, which hid this. Telltale: the
stale `--stats` JSON has no `total_steps`. Anything built in `~/turmite-target` since 10-16 is suspect (earlier proptest passes, the post-refactor
`cargo test` passes, the CPU `probe`). Verified fresh build: `CARGO_TARGET_DIR=~/turmite-target-p4` (gpu_probe 91c072f7, probe 7d341949, built 10-18).
Source mtimes were not touched.

## To do, in order (each needs an otherwise idle machine; check `ps -eo pid,args | grep -E '/release/|cargo'` and `nvidia-smi` first)
1. **Paired P2 re-measure.** Build `probe` and `gpu_probe` at HEAD into a clean dir (or `touch` the sources; check exit status, log binary sha256).
   Time `probe 27 8192` (about 11 min) and `gpu_probe 27 8192 --lanes 2400` on `2026-10-16-kappa-leadrun-fresh/results/specs.txt`, compare to the dump.
   Record it as a new README/NOTEBOOK entry; do not edit the registered P2 text or the old result.
2. **Fix `scripts/bench.sh`** (and check `spotcheck.sh`, other scripts that `| tail -1` a build): test the build exit status, log the binary sha256, abort if stale.
   `scripts/bench_lanes.sh` already does this via `EXPECT_SHA`.
3. **Rerun on a verified build:** `cargo test --release` and `cargo test --release --features cuda --test gpu --test onset`
   (last run in the suspect dir). `--test gpu_probe` already passes 5/5 on the fresh build.
4. Update README, NOTEBOOK (append-only), `notebook.ipynb` (execute per `experiments/README.md`), commit explicit paths only.
5. Optional, unmeasured: 16-bit cell vs two byte planes; one-byte packing for n <= 32. Only if VRAM binds (about 13-19 GB of 20 used at 2400 lanes).

## Things to know
- Env: `source experiments/2026-10-16-gpu-ant-sweep/scripts/env.sh` in every WSL build/run. Use `wsl -e bash -lc "..."`; detached `nohup ... &` inside a one-shot `wsl -e` gets killed, run long jobs via the Bash tool's background mode.
- Another session's CPU batch (`run_mutants.sh`, worktree `.claude/worktrees/paper-figure-proposals`) was running during P4 prep; it was finished before the P4 phases. Timing is contaminated if anything else is live.
- The patch is initialised at tile allocation (it straddles four tiles at 8192); do not "simplify" it to the first tile.
- `gpu_probe` rejects non-ant specs; rotor/rw/sbg/pw/cookie/errw stay CPU-only. `total_steps` counts the budget for non-escaped runs only.
- Untracked and not mine to commit: a stray `notebook.ipynb` at the repo root (unknown origin, probably another session), `numbers_check.py`, `run_check.sh`, `.claude/`, `.codex/`, `.githooks/`; plus modified paper/notes files from other work. Stage explicit paths only.
- `scratch/` (gitignored) holds the benchmark CSVs, the stale P2 binary at `scratch/p2_binary/gpu_probe`, an aborted-run log, and my throwaway helper scripts. `~/quiet_649.sh` can be deleted.
