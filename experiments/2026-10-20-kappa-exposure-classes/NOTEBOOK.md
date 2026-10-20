# Lab notebook: boundary occupation by exposure class

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. Times are UTC.

### 2026-10-20 09:40 (+00:00) · ba38590 · deviation
Environment differs from AGENTS.md (Windows + WSL, 6 cores, rustc 1.98, Python 3.12). This box:
Linux x86_64 (kernel 6.12.94+), 8 logical cores, 15 GB RAM, rustc 1.85.1 / cargo 1.85.1 (edition
2024 is stable since 1.85, so no upgrade was needed), uv 0.13.3, Python 3.13.5, no GPU
(`nvidia-smi` not found; no CUDA work). Commands run natively, not via `wsl -e`, with
`UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy`. Baseline `cargo test --release` on
ba38590: 144 passed, 0 failed, 0 ignored (log kept outside the repo). No push access: work is
committed locally on branch `turmite-research-agent/onboarding`.

### 2026-10-20 09:42 (+00:00) · ba38590 · intent
Locate the ants' below-uniform boundary occupation (census median kappa/kappa_unif 0.81) within
the boundary: flat edge (u = 1 unvisited neighbour) versus exposed cells (u >= 2). Informs whether
a mechanism for the deficit should be sought in edge roughness/tips or in returns to flat edge.
Also, as a by-product, test the 1.11 uniform reference rule by rule with an estimator that does not
need it.

### 2026-10-20 09:43 (+00:00) · ba38590 · observation
Reading `src/probe.rs`: per window c = new_cells/boundary_steps and kappa = (boundary_steps/S) /
(P_end/A_end), so c * kappa * P_end/A_end = new_cells/S exactly. The identity dA/dt = c kappa P/A
is an accounting identity of the probe's definitions at window level; its only non-trivial content
is the end-of-window convention, which is what makes the uniform value ~1.11. The time-averaged
share sum P(t)/A(t) removes that convention.

### 2026-10-20 09:45 (+00:00) · ba38590 · observation
Instrument validation (before registration; no class statistics examined except two RL windows
printed by `head`): `kappa_exposure_classes 20 512 --check` on 18 specs (ants plain and patched,
rotor, rw, mf) passes. A mutant clipping the class at 2 panics in `--check` (exit 101). mf seeds
2026101601-02 at grid 4096, 2^20 steps reproduce `examples/kappa_uniform_baseline.rs` on all 24
windows (area, perimeter, new_cells, boundary_steps): 0 mismatches.

### 2026-10-20 09:46 (+00:00) · ba38590 · decision
Registered P0-P4 in README.md before running the sample. Sample fixed by `scripts/select_rules.py`
(seed 20261020). Preferred hypothesis H-exposed (median Q >= 1.18).
