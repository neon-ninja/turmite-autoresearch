# macOS GPU support lab notebook

## 2026-10-23T11:55:01Z — intent — b942a82854b4d61222301959b9895060d2543d3f

Add an optional native wgpu sweep backend. Keep bounded-board, seeded-background and highway semantics. Start with public oracle tests before the implementation. The local device reports Apple M4 Max, 40 GPU cores, Metal support and 64 GiB system memory. Do not infer a speedup from hardware specifications.

## 2026-10-23T11:55:01Z — decision — b942a82854b4d61222301959b9895060d2543d3f

Use one invocation per ant and reuse the CUDA tile-pool design. Retain the current CUDA and CPU code paths. Add a separate `wgpu_sweep` binary so optional backend dependencies and errors remain explicit. The pruning-index match concerns conservation laws and does not exclude this correctness work.

## 2026-10-23T12:15:49.551Z — observation — c0c93c981811ef96074791c0a96d439029a05ecc

The public oracle test first failed because portable_gpu did not exist. After implementation, the Metal oracle passed. The original 16-tile pressure fixture did not force exhaustion on this GPU; its retry-count assertion failed. Replaced that fixture with four lanes sharing one tile on a one-tile board, so retries are forced while every solo job can fit. The corrected pressure test and reuse after a solo allocation error passed.

## 2026-10-23T12:15:49.551Z — observation — c0c93c981811ef96074791c0a96d439029a05ecc

All seven Metal integration tests, two CLI parser tests, shader validation, invalid-configuration tests and the explicitly enabled counter-carry hardware test passed. The full CPU release suite passed 205 tests. Clippy succeeded with both GPU features; old warnings remain, none in the added files. The 57-rule census CSV equals CPU sweep byte-for-byte. A mixed blank/seeded CLI example also ran on Metal. See results/validation.json for commands, source and binary hashes, seed and output provenance.

## 2026-10-23T12:15:49.551Z — decision — c0c93c981811ef96074791c0a96d439029a05ecc

Keep this as native census support. Do not claim a Mac speedup from correctness evidence. Future work should first measure lane count, slice length, readback and host detection costs; compare an optional native wgpu 64-bit shader before committing to a separate Metal implementation. CUDA hardware, hosted CI and other GPU platforms remain untested here.

## 2026-10-23 — intent / deviation (f3c8aae)

Measure end-to-end Metal speed relative to both CPU runners. Registered cases and predictions in README before running. CodSpeed exec harness is unavailable on Apple ARM; user explicitly authorized local wall-clock timing. Use monotonic Node timing, rotating backend order, output equality and five measured repetitions. No runtime implementation changes. Desktop applications are active; record this confounder.

## 2026-10-23T12:56:39.801Z — observation (f3c8aae860e2c8f3db8715b697132aba14e03a74)

Completed small: five samples per backend, one warmup each, rotating order. Every CSV matched SHA-256 186ba51b60017135b6695af3f20f37a63fd0ad0ed20ca68725aaa58e269fa2cd. Median seconds: fast=0.00515525, lean=0.006230708, Metal=0.040484834. See results/benchmark-small.json for all samples and provenance.

## 2026-10-23T12:57:19.060Z — observation (f3c8aae860e2c8f3db8715b697132aba14e03a74)

Completed batch: five samples per backend, one warmup each, rotating order. Every CSV matched SHA-256 7034cff93e7238bbdf7f551b5506b6ba05db3e365967c17487e64d1dbe4a145e. Median seconds: fast=1.313243167, lean=2.573710375, Metal=0.712973875. See results/benchmark-batch.json for all samples and provenance.

## 2026-10-23T12:58:25.291Z — observation (f3c8aae860e2c8f3db8715b697132aba14e03a74)

Completed long: five samples per backend, one warmup each, rotating order. Every CSV matched SHA-256 c747b55d4d8cabbc641c30939e858a5b46807681a7e85b3570a2f5ac484641d3. Median seconds: fast=2.8472575, lean=2.572413709, Metal=4.099581417. See results/benchmark-long.json for all samples and provenance.
