# macOS GPU support lab notebook

## 2026-10-23T11:55:01Z — intent — b942a82854b4d61222301959b9895060d2543d3f

Add an optional native wgpu sweep backend. Keep bounded-board, seeded-background and highway semantics. Start with public oracle tests before the implementation. The local device reports Apple M4 Max, 40 GPU cores, Metal support and 64 GiB system memory. Do not infer a speedup from hardware specifications.

## 2026-10-23T11:55:01Z — decision — b942a82854b4d61222301959b9895060d2543d3f

Use one invocation per ant and reuse the CUDA tile-pool design. Retain the current CUDA and CPU code paths. Add a separate `wgpu_sweep` binary so optional backend dependencies and errors remain explicit. The pruning-index match concerns conservation laws and does not exclude this correctness work.
