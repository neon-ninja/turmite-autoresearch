# Native macOS GPU support

## Question and registered predictions

Registered 2026-10-23T11:55:01Z, from `b942a82854b4d61222301959b9895060d2543d3f`.
Can the existing CUDA sweep contract run through wgpu and Metal without changing the ant or its detector?

- Correctness: every `RunResult` field must equal the CPU oracle for blank and seeded backgrounds, including grid escape, partial heading words, tile crossings and pool-pressure retries.
- Scheduling: changing job order, lane count or slice size must not change a job's output.
- Portability: CPU builds must not need wgpu or a GPU. macOS selects Metal; other native platforms can use Vulkan or DX12.
- Performance hypothesis, not measured: batches of independent ants can hide memory latency on Apple GPUs. A single ant remains serial. No speedup factor is predicted or claimed in this implementation task.

The implementation uses the existing bounded board and sparse 64-by-64 tiles. It keeps byte-sized cells by packing four cells into each WGSL `u32`. Each tile belongs to one invocation, so cell writes need no atomics. Step counters and background seeds use pairs of `u32` values to preserve `u64` semantics without optional shader extensions.

## Pruning-index preflight

At the starting revision, queried `notes/pruning-index/entries` with `rg -n -i -e 'metal|wgpu|gpu|oracle'` and read the only signature, `NR-20261022-additive-laws-order-blind`. That record concerns additive conservation laws; it does not cover GPU implementation or oracle validation. Decision: run bounded correctness checks, without a scientific sweep or exclusions. No dead-end signature is warranted for a platform port.

## Dependencies and backend choice

The optional `wgpu` feature adds wgpu for validated compute and native Metal/Vulkan/DX12 access, pollster for its synchronous CLI, and bytemuck for checked host/shader data layouts. CPU users do not compile these dependencies.

A separate native Metal shader would add a second platform-specific implementation. wgpu translates the same WGSL kernel to Metal and leaves room for other native GPUs. A direct Metal backend is a later option only if measurement identifies a limitation that wgpu cannot address.

Primary documentation: [wgpu](https://docs.rs/wgpu/30.0.1/wgpu/), [WGSL types](https://www.w3.org/TR/WGSL/#types), [Metal resource storage modes](https://developer.apple.com/documentation/metal/resource-storage-modes).

## Reproduction

No justfile or Nix flake exists in this revision. Use release builds.

```sh
PROPTEST_RNG_SEED=20261023 cargo test --release --locked --features wgpu --test portable_gpu
cargo clippy --release --locked --features wgpu --all-targets
cargo test --release --locked
cargo run --release --locked --features wgpu --bin wgpu_sweep -- 2 6 32768 256 --lanes 64
```

GPU tests require a real compatible adapter and fail if it is absent. They do not silently fall back to CPU execution.

## Results and limitations

Implementation and validation are in progress. No local performance measurements have been made.
The first port targets census sweeps. The CUDA `gpu_probe` statistics and the browser simulator are separate instruments.

## CLI and Rust API

```sh
# Explicit rule list, with an optional full-width background seed.
printf 'RL\nRLR 18446744073709551615\n' | cargo run --release --locked --features wgpu --bin wgpu_sweep -- - - 1000000 8192 --lanes 128 --slice 65536 --pool-tiles 65536 --stats run.json
```

The Rust API is `turmite::portable_gpu::{GpuConfig, GpuSweeper, Job}`. Construct a sweeper with `GpuSweeper::new`, then call `run(&jobs, progress)`. Returned rows follow input order. Reuse the sweeper for later batches. `config()` exposes the fixed allocation settings; `device_name`, `backend` and `stats` expose the selected device and completed-job counters.

Default memory is bounded: a 256 MiB tile pool, about 64 MiB each for tile directories and heading rings, another ring-sized staging buffer, and small state/free-list buffers. Exact sizes depend on grid, history and lanes. The constructor checks the adapter's binding and buffer limits before allocation. A shared-memory Mac still needs room for these buffers and CPU onset replay; the pool does not consume all reported system memory.

`--pool-tiles` sets the pool explicitly. If concurrent ants exhaust it, failed ants restart at a lower concurrency. If one ant cannot fit alone, the call returns an error. It never emits a truncated successful row. The next call clears device state after either success or a pool-exhaustion error.

Supported: one-state square-grid ants, 1..=128 colours in the library, blank or `bg_colour` seeded backgrounds, `u64` lifetime budgets, and independent jobs. The CLI parser accepts the repository's standard 2..=128-colour rules on stdin; enumerated rule lengths are 2..=31. Backend limits can impose smaller grid or lane counts. Checkpoints, the CUDA probe statistics, other walkers, other lattices, multi-ant runs and browser WebGPU are not part of this port.

Escape follows `fast::run`: the step out of the board counts, and the outside cell does not extend the bounding box. The known `lean` edge case when the budget ends on exactly that step is not copied; see the tiled-grid design note. This can affect CSV equality with `sweep_lean` for that exact boundary condition.

## Performance work still needed

The implementation establishes correctness on Metal, not a speedup. Use many independent, long-running jobs to evaluate throughput. Compare the same rules, seeds, grid, detection history, horizon and output hashes against the CPU runner. Separate setup, stepping, readback, detection and onset replay; compare full process wall time as well as dispatch time. Use repeated runs on an idle Mac and report spread and thermal conditions. Keep timestamp queries optional and validate their values before using them.

Start by varying lane count and slice length; the tradeoff is latency hiding versus state readback and dispatch overhead. The current host waits for state readback once per slice and processes detection before the next slice. Overlapping that work and specializing away seeded-background code are later candidates, subject to profiling. Native MSL/objc2-metal is an alternative if validated bounds checks, packed loads, or 64-bit emulation prove material. Its additional implementation and synchronization obligations require the same oracle checks.

wgpu also exposes optional native `SHADER_INT64` arithmetic ([feature documentation](https://docs.rs/wgpu/30.0.1/wgpu/struct.Features.html)). A capability-checked shader variant using it is a smaller next experiment than a separate Metal backend. The initial shader uses paired words so the same exact simulation does not depend on that optional feature.
