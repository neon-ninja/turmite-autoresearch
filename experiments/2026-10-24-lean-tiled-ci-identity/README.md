# CI identity check for `sweep_lean` and the tiled backend

Created 2026-10-24T10:00Z, base commit `b942a82854b4d61222301959b9895060d2543d3f`.

## Question

`.github/scripts/check_determinism.py` compares `sweep` and `probe` CSVs at 1 and
3 Rayon threads. Most census data, though, comes from `sweep_lean` (documented as
"identical results" to `sweep`), and since 2026-10-23 it has a second storage
backend (`TURMITE_GRID=tiled`) whose CSV is documented as byte-identical to dense.
Neither claim is exercised by the hosted reproducibility job; they are covered only
by `cargo test` property tests and a one-off benchmark. Can the small CI batch
(every canonical nontrivial rule of lengths 2-6, 32,768 steps, grid 256) also pin
them down, end to end through the CLI?

## Pruning-index preflight

Index revision `b942a82854b4d61222301959b9895060d2543d3f`. Searched
`sweep_lean|tiled|determinis` in `notes/pruning-index/entries/`; sole entry is
`NR-20261022-additive-laws-order-blind` (additive conservation laws), which is
unrelated to implementation identity. No match. Decision: run. This is an
engineering identity check, not a sweep over a rule family, and excludes nothing.

## Registered predictions (before any run of the new checks)

P1. `sweep_lean` (dense) stdout is byte-identical to `sweep` stdout on the CI batch.
P2. `sweep_lean` is byte-identical at 1 and 3 Rayon threads, dense and tiled.
P3. Tiled with default settings, and with stress settings (tile side 2^2, cache of 2
    tiles, packed and raw; tile side 2^6 with 16 cache tiles and lz4), is
    byte-identical to dense. The stress settings force constant eviction, so a
    passing result is not vacuous.
P4. The new check can fail: a deliberately corrupted comparison input is
    reported as a failure (a planted-fault control).

A failure of P1-P3 would be a real discrepancy and must be reported as such, not
patched around. Passing says only that outputs agree on this batch, toolchain and
step budget; it is not evidence about asymptotic behaviour or other inputs.

## Results

See below (appended after the run).
