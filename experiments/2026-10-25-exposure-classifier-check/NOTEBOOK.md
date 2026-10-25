# Lab notebook: formal check of the exposure classifier

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-25 09:12 (+00:00) · 8a073b7 · intent
Close off the "instrument bug" explanation for the failed P3/P4 of 2026-10-20-kappa-exposure-classes
with a checker written separately from the Rust (proof-track architecture rule). Plan: a Rust
example that calls the instrument's own `Region` / `run` code on every 3x3 pattern and every visit
order and on scripted walks, dumps the tables to `results/`; a PEP 723 z3 script that proves the
documentation-level spec and checks the dumped tables against it.

### 2026-10-25 09:13 (+00:00) · 8a073b7 · decision
Pruning preflight at `8a073b75aab339e492863eaa6303a7d035906162`: `rg --files notes/pruning-index/entries`
(one entry); `rg -n -i -e exposure -e classifier -e "exposure class" -e z3 -e formal notes/pruning-index/entries`:
no matches. Decision RUN (finite check, seconds of compute).

### 2026-10-25 09:14 (+00:00) · 66b8ddb · deviation
The registration text says "09:14 UTC"; the registration commit `66b8ddb` is stamped 09:13:37 UTC.
Both precede any harness or checker code. No other change.

### 2026-10-25 09:15 (+00:00) · 66b8ddb · assumption
The instrument's private `Region` and `run` cannot be reached from another example: `include!` rejects
the file's `//!` header and a `#[path]` module hides private items from the parent. The harness
therefore carries a byte-for-byte copy of lines 20-248 of `examples/kappa_exposure_classes.rs`
(everything except the doc header and `main`) and asserts at runtime that the instrument source
(`include_str!`) contains that copy verbatim, so the code exercised is the code that ran.
Status: enforced by the harness; would fail loudly if the instrument changed.
