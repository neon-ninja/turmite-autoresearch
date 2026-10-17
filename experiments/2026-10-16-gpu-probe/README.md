# GPU probe: `probe` on a CUDA device

**Status:** in progress (code and tests done 2026-10-16; timing waits for an idle machine). **Goal:** run
`probe MAX_LOG2 [GRID]` batches (plain rules `RULE` and patched ants `RULE@SEED`) on the RTX 4000 Ada with CSV
rows **byte-identical** to `src/bin/probe.rs`, so a batch like `2026-10-16-kappa-leadrun-fresh`
(2400 runs, `probe 27 8192`, about 3e11 steps, ~20 min on 6 CPU cores) stops occupying the CPU.

## Question

Can the probe instrument (visited region, perimeter, fresh-neighbour counts, octave windows, gyration and ant
moments) be ported to the tile-pool GPU runner with exactly the CPU's output, and how much faster is it than the
6-thread CPU probe on the real 2400-run batch?

## Design (decided before any timing)

- New files only: `src/gpu/probe.cu` (kernels), `src/gpu/probe.rs` (`GpuProber`, `ProbeJob`, `ProbeConfig`),
  `src/bin/gpu_probe.rs`, `tests/gpu_probe.rs`. `ant.cu`, `GpuSweeper`, `fast`, `lean` are untouched. The scheduler
  (lane refill between slices, results by job index, pool exhaustion -> requeue + halve the active-lane cap, solo
  failure -> `GpuError`) is copied from `GpuSweeper::run` rather than generalised, so the sweep's tests keep guarding it.
- **Cell = 16 bits**: low byte colour, high byte `unv + 1` for visited cells, 0 for never-visited. An all-zero tile is
  blank and unvisited, so the zeroed pool and the retire kernel's zeroing need no change, and a neighbour in an
  unallocated tile is unvisited without allocating. One load per step gives colour, visited flag and the
  boundary/fresh-neighbour counts (the current cell is always visited). 8 KiB tiles (vs 4 KiB in the sweep).
  A one-byte layout (5-bit colour + 3-bit `unv + 1`, n <= 32) is possible; not built unless VRAM binds.
- **Windows in the kernel**: a lane's slice is cut into chunks that end at window boundaries (`steps == 2^(k+1)`);
  the hot loop has no window test. At a boundary the kernel writes one 104-byte record (area, perimeter, counters,
  region sums, ant sums, bounding box) to `recs[lane][k]` and resets the window counters. An escaping step ends the
  run without closing its window, so no partial window is ever emitted and `escaped` is set on all emitted rows.
- **Exact floats**: the GPU accumulates `ax, ay, axx` as integers; the host converts them with `as f64` and calls
  `probe::finish_window`, the same function the CPU probe now uses (same expression order). Exact while a window's
  `axx < 2^53` (`probe::window_sums_exact`); `GpuProber::run` refuses jobs past that bound (2^27 steps on 8192 is fine).
- **Patch**: `PatchedAnt::initial_cell` (hash of seed and relative position inside the 5x5 patch) is written when a
  tile is allocated, for the patch cells that fall in that tile. The patch straddles up to four tiles (at 8192 the start
  is a tile corner), so writing it with the first tile would be wrong.
- Escape test `|x| >= c-2 || |y| >= c-2` as one unsigned compare per axis; the bounding box is updated only on
  first visits (as the CPU does).
- Refactor of the CPU side (`src/probe.rs`): `WindowSums`, `finish_window`, `csv_rows`, `CSV_HEADER`, `window_sums_exact`
  shared by `probe`, `gpu_probe` and the GPU library. Checked by comparing the old and new `probe` CSV on 18 mixed specs
  (all walker kinds), see NOTEBOOK.

## Predictions (registered 2026-10-16 20:49 NZDT, before any timing)

- **P1 (correctness)**. `gpu_probe 27 8192` on the 24 spot-check specs (every 100th line of
  `2026-10-16-kappa-leadrun-fresh/results/specs.txt`, lines 1, 101, ..., 2301) is byte-identical to `probe 27 8192`;
  all proptests pass (windows equal, floats included). Also predicted: the full 2400-spec GPU CSV equals the existing
  kappa-leadrun CPU dump byte for byte. Falsified by any differing byte.
- **P2 (speed)**. On the full 2400-spec file at `probe 27 8192`, GPU wall time (whole process) is **2x to 6x faster**
  than a clean `probe` on 6 CPU threads in the same session, central estimate 3.5x. That is below the sweep kernel's
  4.9x: each ant gets no cheaper per step (an extra 16-bit load stream plus window sums on every step), 2400 jobs fill
  only 2400 of the >= 4096 lanes at which the sweep was still gaining throughput, and tails are not hidden by
  more jobs than lanes. **Falsified if the speedup is >= 4.9x (then the probe is not costlier than the sweep) or < 2x.**
- **P3 (memory)**. All 2400 ants fit in the default pool at once (`oom_requeues == 0`, pool about 19 GB;
  bounding-box tile estimate from the existing dump: 1.6e6 tiles = 13 GB at 8 KiB). Falsified by any requeue.
- **P4 (throughput vs lanes)**. Steps/s rises with lanes from 1024 to 2400 (every ant resident). Reported only if run.

## Use it

```bash
source experiments/2026-10-16-gpu-ant-sweep/scripts/env.sh
cargo run --release --features cuda --bin gpu_probe -- 27 8192 --stats results/x.json < specs.txt > probe.csv
```
Same stdin lines and CSV as `probe`; only `RULE` and `RULE@SEED` lines (other walkers are rejected before running).
Flags: `--lanes N` (default 4096), `--slice N`, `--pool-gb G`, `--device K`, `--stats FILE`.
Library: `turmite::gpu::{GpuProber, ProbeConfig, ProbeJob}`; `GpuProber::run(&jobs, progress)` returns
`Vec<probe::ProbeResult>` index-aligned with the jobs.

## Results

Recipe: `bash scripts/bench.sh` (commit 53ccccc, machine otherwise idle: no `release/` processes, loadavg 1.0 before gpu1;
the cpu phase's loadavg of 5.9 is its own 6 threads). Log: `results/bench.log`, stats `results/bench_gpu{1,2}.json`.
Inputs: kappa-leadrun-fresh `specs.txt`, 2400 lines, sha256 1e9198bf...; RTX 4000 Ada, `--lanes 2400`.

| Phase | Whole-process wall | Output vs existing CPU dump |
|---|---|---|
| gpu1 | 177.4 s (device path 161.0 s) | byte-identical |
| cpu (`probe 27 8192`, 6 threads) | 681.6 s | byte-identical |
| gpu2 | 176.9 s | byte-identical |

- **P1 held**: 24-spec spot check (`results/spotcheck.txt`) and all 2400 specs are byte-identical; 5/5 `gpu_probe` proptests pass.
- **P2 held** (registered 2x-6x, falsified at >= 4.9x or < 2x): speedup 681.6/177.4 = **3.84x** (3.85x with gpu2), against a central estimate of 3.5x.
  n = 1 CPU run and 2 GPU runs on one spec set, so no between-run spread beyond gpu1 vs gpu2 (0.4 s); not a sweep over specs or rules.
  An earlier handover guess of ~7x came from a contaminated CPU time (1231 s) and is superseded.
- **P3 held**: `oom_requeues` 0, pool 2,409,257 tiles. (The registered 1.6e6-tile bounding-box estimate was low by about 50%; the pool still sufficed.)
- **P4 not run.**
- Other tests after the `src/probe.rs` refactor: plain `cargo test --release` and `cargo test --release --features cuda --test gpu --test onset`
  all pass (exit 0; the plain run was logged tail-only, 5 result lines, all ok).

