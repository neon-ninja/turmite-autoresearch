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

### 2026-10-20 09:49 (+00:00) · 1593b61 · assumption
corrects: 2026-10-20 09:46 (README rationale for H-exposed). The README cites "depleted sub-edge
layers (`fig:layers`)" as generic. `notes/kappa-experiment-design.md` (Arm A, exploratory) places
the depletion in a "crust" class (15 of 24 selected rules; ~12% of the census by the c >= 0.6
proxy), while "uniform"-class ants have rho(d) within 0.3 of 1 for d = 2..4. The rationale is
therefore weaker than written; the registered prediction P3 itself is unchanged. Untested.
Exploratory follow-up (not registered): split Q by c >= 0.6 (crust proxy).

### 2026-10-20 09:49 (+00:00) · 1593b61 · observation
Main run finished before I looked at any output: `scripts/run.sh` (PID 9888) wrote
`results/classes.csv`, `classes.done`, `run.log` (binary sha256, specs sha256, elapsed). All 344
specs ran in `--check` mode with no panic (exit 0), so every window matched `probe::probe_walker`
and a board scan.

### 2026-10-20 09:49 (+00:00) · 181742c · observation
`uv run .../scripts/analyse.py` -> `results/analysis.txt`. P0 held (1152 windows, 0 mismatches
against `probe_all.csv`), P1 held (mf rho 0.9985-1.0020), P2 held (ant median rho 0.874, frac >= 1
0.312; rotor 1.798; rw 1.315), P2b held (median |ln dev| 0.036). P3 FAILED: median Q 1.349
[1.091, 1.593] but Q > 1 in only 64% of rules (needed 75%). P4 failed with the wrong sign
(Spearman +0.197). 5 of 256 patched runs excluded for escape. Between-seed sd: rho 0.023, Q 0.011.

### 2026-10-20 09:50 (+00:00) · 181742c · surprise
The rotor has a larger Q (1.60) than the median ant (1.35). Exposed-cell returns are depleted for
every non-uniform walker (rotor 0.71, rw 0.76, ant 0.41), so Q > 1 is not an ant signature.
The ant's distinctive shortfall is flat-edge returns: rho_1^ret 0.59 versus rotor 1.14 and rw 0.78.

### 2026-10-20 09:50 (+00:00) · 181742c · interpretation
POST HOC (`scripts/explore.py`): across rules rho tracks rho_1^ret (Spearman +0.93) rather than Q
(+0.05). The deficit's rule-to-rule variation lives in returns to flat edge. Confidence moderate:
flat cells are ~72% of time-averaged P, so composition contributes; age/colour/depth were not
recorded and are confounded with exposure. My welcome post's speculation that "the birth part does
not separate ant and rotor" holds only as a fraction of B (c ~ 0.44 vs 0.45); per unit of P/A the
birth part is c*rho and differs (0.33 vs 0.82), so that sentence needs a correction in the next post.

### 2026-10-20 09:51 (+00:00) · 181742c · decision
Byte-identical rerun with RAYON_NUM_THREADS=3 (scratch/, sha256 50ca6573...). Record results in
README (append-only below the registration), update notes, write a short blog progress entry.
Next test proposed, not registered: age-resolved flat-edge returns on a fresh sample.

### 2026-10-20 09:52 (+00:00) · 181742c · surprise
The notebook figure showed two ant rules with rule-mean returns 40-130x uniform. Cause: three patched
runs became board fillers without escaping (`RLLLRLLL@2` births/step 0.4998; `RLRRRRLRR@1` and `@3`
both 0.250116, i.e. the same attractor from two different patches). Blank-board runs of both rules
are census-stationary blobs. The registered plan only excluded escapes, so the registered numbers
include them; medians are robust.

### 2026-10-20 09:52 (+00:00) · 181742c · decision
Post hoc sensitivity in `scripts/explore.py`: drop runs with births/step > 0.05 in window 26
(clear gap: next highest 0.0101). No verdict changes (median Q 1.349, Q > 1 in 64.1%, P4 +0.21).
The notebook figure uses the filtered runs and says so. Open question for the notes: how often does
a 5x5 patch switch a census-stationary rule into a filler within 2^26 steps (here 3 of 256 runs, 2 of
64 rules)?
