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
- **P4 not run.** (Superseded 2026-10-18: P4 run, see below.)
- Other tests after the `src/probe.rs` refactor: plain `cargo test --release` and `cargo test --release --features cuda --test gpu --test onset`
  all pass (exit 0; the plain run was logged tail-only, 5 result lines, all ok).

### P4: throughput vs lanes (2026-10-18)

Recipe: `EXPECT_SHA=91c072f7... bash scripts/bench_lanes.sh` (HEAD 14b9a24, `gpu_probe` built fresh in `~/turmite-target-p4`, sha256
91c072f715cf...; same 2400 specs as P2). Order 2400, 1024, 1536, 1536, 1024, 2400; no foreign process before or after any phase.
Log `results/bench_lanes.log`, stats `results/bench_lanes_<L>_r<k>.json`. Every CSV byte-identical to the CPU dump, 0 requeues.
Steps/s uses one numerator for all L, `total_steps` = 2.731e11 (budget of non-escaped runs; identical across L because outputs are).

| Lanes | Whole-process wall, r1 / r2 (mean) | Device path `wall_secs`, r1 / r2 (mean) | Steps/s (device) | Per lane | Launches |
|---|---|---|---|---|---|
| 1024 | 280.34 / 280.25 (280.30) s | 278.14 / 278.12 (278.13) s | 0.982e9 | 9.59e5 | 319 |
| 1536 | 219.30 / 219.41 (219.35) s | 217.16 / 217.26 (217.21) s | 1.257e9 | 8.19e5 | 256 |
| 2400 | 156.64 / 156.24 (156.44) s | 154.35 / 154.16 (154.26) s | 1.771e9 | 7.38e5 | 128 |

- **P4 held** under the criterion fixed in NOTEBOOK before running (mean time strictly falls 1024 > 1536 > 2400, each gap larger than the
  larger repeat spread, same order on the device path). Gaps 60.9 s and 62.9 s against repeat spreads of at most 0.40 s. Batch
  throughput rises 1.28x (1024 to 1536) and 1.41x (1536 to 2400), 1.80x overall on the device path.
- What it does and does not show: n = 2 per L, one spec set, one GPU. Only L = 2400 has every ant resident. Below that, ants queue
  (about 2.3 and 1.6 rounds of jobs), so the rise in batch steps/s is largely what queueing predicts and would fail only if per-lane
  throughput collapsed with occupancy. It did not collapse but did fall: per-lane steps/s is 23% lower at 2400 than at 1024 (memory
  pressure from more resident ants is the obvious candidate, not tested). P4 measures batch scheduling on this workload, not lane-level
  parallel efficiency at fixed occupancy.

**Provenance caveat on P2 (found 2026-10-18).** The P2 benchmark did not time a 53ccccc build. The binary it ran (sha256 4d3e395d...)
was built 2026-10-16 20:46; the source on `/mnt/f` kept mtimes of 20:41-20:43 although its content changed later, so cargo never
rebuilt it and `bench.sh`'s `| tail -1` hid that. P2's 3.84x and its byte-identity checks describe that build. The HEAD build is faster
on the same specs (156 s vs 177 s at 2400 lanes) and also byte-identical, but P2 was not rerun against a fresh CPU baseline, so no
HEAD speedup is claimed. Other artefacts built in `~/turmite-target` since 10-16 may be stale the same way (see NOTEBOOK 2026-10-18).

### P4 and a provenance correction (added 2026-10-18; the registered text above is unchanged)

**Correction to P2/P3/tests.** `bench.sh` builds into `~/turmite-target`. The source files on `/mnt/f` carry mtimes of 2026-10-16 20:41-20:43,
older than the binary built there at 20:46, so cargo never rebuilt after later content changes. The `gpu_probe` timed for P2 (sha256 4d3e395d...,
logged in `results/bench.log` under "git 53ccccc") emits no `total_steps` in its stats JSON, so it is an older build than 53ccccc. P2's 3.84x and its
byte-identity result are therefore a measurement of that 10-16 20:46 build, not provably of HEAD. The earlier "5/5 gpu_probe proptests" and post-refactor
`cargo test` passes were also run in `~/turmite-target` and carry the same doubt. They are not retracted (the build was correct for what it contained) but are
superseded below by runs on a verified fresh build. Fresh build: `CARGO_TARGET_DIR=~/turmite-target-p4`, `gpu_probe` sha256 91c072f7..., built 2026-10-18 19:57.

**Re-checks on the fresh build:** `cargo test --release --features cuda --test gpu_probe` 5/5 pass (17 s). All six P4 runs (2400 specs, `probe 27 8192`) are
byte-identical to the CPU dump, so P1 holds for the fresh build at full scale (6 of 6).

**P4 (registered criterion, fixed before running, NOTEBOOK 2026-10-18): HELD.** Recipe: `scripts/bench_lanes.sh`, log `results/bench_lanes.log`,
stats `results/bench_lanes_<L>_r<k>.json`; order 2400,1024,1536,1536,1024,2400, no foreign processes in any phase, n = 2 per L.

| Lanes | Wall r1 / r2 (s) | Device `wall_secs` r1 / r2 | Steps/s (device, mean) | Launches |
|---|---|---|---|---|
| 1024 | 280.34 / 280.25 | 278.14 / 278.12 | 9.82e8 | 319 |
| 1536 | 219.30 / 219.41 | 217.16 / 217.26 | 1.257e9 | 256 |
| 2400 | 156.64 / 156.24 | 154.35 / 154.16 | 1.771e9 | 128 |

Both orderings are strictly decreasing, gaps (61 s and 63 s wall) exceed the repeat spreads (<= 0.4 s) by over 100x. Steps/s uses the identical `total_steps`
(2.731e11) for every L, so it is the inverse of device time: 1.28x from 1024 to 1536, 1.41x from 1536 to 2400, 1.80x overall.
What it does not show: below 2400 lanes ants queue (about 3 and 2 rounds against 1), so a rise is expected from scheduling alone; the observed 1.8x is less
than 2.3x (2400/1024), so per-lane throughput is lower with more lanes active or tails are hidden by refill. This is batch throughput, not lane efficiency.
One spec set, one GPU, n = 2.

**P2 on the fresh build (indicative only):** 156.4 s mean wall against the CPU 681.6 s gives 4.36x, inside the registered 2-6x and below the 4.9x
falsification line. The CPU baseline was timed with a `probe` from `~/turmite-target` (sha256 f1a309cf...), whose provenance is subject to the same mtime doubt,
so the ratio is not a clean paired measurement; a clean one needs `probe` rebuilt at HEAD and re-timed (about 11 min).

