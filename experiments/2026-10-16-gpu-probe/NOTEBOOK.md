# Lab notebook: GPU probe

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-16 20:47 (+13) · 5b97961 (dirty tree) · intent
Task: port `probe` (plain rules and `RULE@SEED`) to the GPU with byte-identical CSV, so `probe 27 8192` batches stop
using the CPU. User message mid-session: "can you use cuda to avoid the cpu contention?" - taken as: the point of the
port, and do the work on the GPU where possible, no CPU-heavy runs while their job is live. Plan: refactor the CPU
window/CSV code into shared functions, new kernel + host module + binary + tests, register predictions, then time.

### 2026-10-16 20:48 (+13) · 5b97961 · observation
The user's own `probe 27 8192` batch (3336 mutant specs, pid 2027659, ~590% CPU, started 20:36) is live. Per the task I do
not benchmark while it runs and I do not touch it. My builds and tests run under `nice`, CPU oracles with `RAYON_NUM_THREADS=2`.
Footprint estimate from the existing 2400-run dump (kappa-leadrun-fresh/dumps/probe.csv, last row per run): sum of
bbox tiles (w/64+2)(h/64+2) = 1.61e6 (mean 670, max 10100 per run); 365 of 2400 runs escaped the 8192 board.
At 8 KiB per tile that is about 13 GB, so it fits in 20 GB VRAM and a 16-bit cell is affordable.
Also: the earlier kappa-leadrun probe wall time (19:46:58 -> 20:07:29 = 1231 s) overlapped the GPU-sweep benchmarks (see
`2026-10-16-gpu-ant-sweep/NOTEBOOK.md`, deviation), so it is not a clean CPU baseline; the CPU baseline is re-measured here.

### 2026-10-16 20:4x (+13) · 5b97961 · decision
Cell layout: one u16 per cell (colour | unv+1), 8 KiB tiles, instead of two byte planes (the task suggested measuring
both). Reason: a single load per step gives everything. NOT measured against two planes; recorded as untested.
One-byte packing (n <= 32) is the fallback if VRAM binds. Scheduler duplicated from `GpuSweeper::run` rather than
generalised, so `tests/gpu.rs` keeps guarding an untouched sweep.

### 2026-10-16 20:4x (+13) · 5b97961 · decision
Patch init at tile allocation for the patch cells inside that tile (not "when the first tile is allocated" as the task
suggested): at size 8192, c = 4096 is a multiple of 64, so the 5x5 patch spans four tiles. Tests include grids with
c = 63, 64, 65 for this.

### 2026-10-16 20:4x (+13) · 5b97961 · observation
CPU refactor check: old binary saved as `~/probe_old` before editing; new `probe` vs `probe_old` on 18 mixed specs
(plain, `@`, rotor:, rw:, sbg:, brotor:, nrotor:, pw:, cookie:, rcookie:, errw:), at `17 1024` and `15 8192`: `cmp`
identical both times (149 and 125 lines). First `gpu_probe` vs `probe_old` on the ant specs (`17 1024`: 59 lines,
`15 8192`: 50 lines; 16 lanes, odd slice 100003): byte-identical on the first try.

### 2026-10-16 20:4x (+13) · 5b97961 · observation
`cargo test --release --features cuda --test gpu_probe`: 5/5 pass in 109 s (48 proptest cases x 7 rigs: one-tile 64,
ragged 100, c=63/64/65 grids, tight pool of 16 tiles, 64 lanes at min_log2 8; scheduling independence by reversing job
order; starved pool with `oom_requeues > 0` asserted; mid-window escape; error paths).

### 2026-10-16 20:55 (+13) · 5b97961 · observation
Full-scale spot check (`scripts/spotcheck.sh`, every 100th spec of kappa-leadrun-fresh, 24 specs, `27 8192`): GPU CSV
byte-identical to the new CPU `probe` (439 rows, sha256 84fa1fa8..., 2 of 24 runs escaped the board). CPU side ran 1 thread
niced (291 s) because the user's batch was live; GPU 54 s for 24 ants on 32 lanes. Output: `results/spotcheck.txt`.

### 2026-10-16 20:58 (+13) · 5b97961 · observation
All 2400 specs on the GPU (`gpu_probe 27 8192 --lanes 2400`), compared with the existing kappa-leadrun CPU dump
(`2026-10-16-kappa-leadrun-fresh/dumps/probe.csv`): `cmp` IDENTICAL; `oom_requeues` 0; wall 177 s (device path 161 s).
This run is NOT a benchmark: the user's 3336-spec probe was still running on 6 cores until 20:53:47 (their log), and the
CPU baseline needed for a ratio had not been measured. Kept in scratch/ only. Prediction P1 (second half) and P3 hold.

### 2026-10-16 20:59 (+13) · 5b97961 · deviation
The user's batch finished at 20:53:47, but at ~20:59 a different session started building my tree into
`~/turmite-target-probeport` and running `gpu_probe 27 8192 --lanes 2400` (pid 2051469, not mine). I did not touch it. A timing run
now would overlap it on the GPU (and its CPU part on 1 core), so I wait until no `release/` or cargo process has been seen for
3 minutes (`~/quiet_649.sh`) and then run `scripts/bench.sh` (gpu1, cpu, gpu2, never overlapping, with before/after process checks).

### 2026-10-16 21:02 (+13) · 5b97961 · decision
User said "pause" and then "prepare to wrap up for the day.. write a handover doc". Stopped before the clean benchmark; killed only my own
wait helper. Wrote `HANDOVER.md`. State: code and correctness done (tests 5/5, 24-spec and 2400-spec byte-identity); timing (P2), the
rest of the test suite after the `src/probe.rs` refactor, `notebook.ipynb` and the commit remain. The one 177 s GPU figure is contaminated
(user's CPU batch live) and must not be cited as a benchmark. If the clean ratio lands near 7x, P2 (2-6x) is falsified on the fast side.

### 2026-10-17 17:56 (+13) · 53ccccc · observation
Clean benchmark (`scripts/bench.sh`, idle machine, phases sequential): gpu1 177.4 s, cpu 681.6 s, gpu2 176.9 s, all three CSVs byte-identical to the
kappa-leadrun CPU dump. Speedup 3.84x. P2 (2x-6x, central 3.5x) HELD; P3 held (0 requeues); P1 held; P4 untested.

### 2026-10-17 17:57 (+13) · 53ccccc · interpretation
The handover's "about 7x, P2 falsified" was wrong: it divided by a CPU time (1231 s) contaminated by the GPU-sweep benchmarks. The clean CPU time is 681.6 s.
The GPU figure was the same with and without the user's batch running (177 s both times), so the GPU path was never the contaminated side. Pool use (2.41e6 tiles)
exceeded my 1.6e6 bounding-box estimate by about 50% without requeues. Single spec set, so 3.84x is a measurement for this workload, not a general speed claim.

### 2026-10-17 17:58 (+13) · 53ccccc · observation
`cargo test --release` (plain) and `--features cuda --test gpu --test onset` pass after the `src/probe.rs` refactor (plain log was tail-only).

