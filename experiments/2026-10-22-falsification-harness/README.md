# Falsification harness (2026-10-22)

## Question
Claims such as "rule R is chaotic" or "rule R forms a stable highway" have so far been checked by running the
blank-board ant for longer, which cannot discriminate a true property from a horizon artefact. Can an automated
loop that *attacks* a registered claim (perturbed starts, asymmetric colourings, obstacles in front of a highway,
off-centre starts on odd boards, an adaptive hill-climb) find counterexamples a longer run would miss, and report
honestly when it cannot?

## What was built
- `src/falsify.rs`, `src/bin/falsify.rs`: seeded runner (`simulate`), ten attack classes, claim parser, verdicts.
- `tests/falsify.rs`: the runner equals `reference::Ant` step for step on arbitrary finite starts and on
  mid-run obstacles; two exact symmetries (heading = rotation; uniform colour-k background = rule rotated by k);
  known-answer claims (RL, RRL, RLL, from the census); Clopper-Pearson against tabulated values; the mid-run
  obstacle and heading tests were mutation-checked (a deliberately wrong direction window and a dropped heading
  are both caught).
- Usage: `falsify CLAIMS.txt OUT_DIR` (refuses a claims file that is not committed unchanged), and
  `falsify --replay OUT_DIR/ID.counterexamples.txt`.

### Design decisions that carry the science
1. **Asymmetry of evidence.** A confirmed highway is a finite witness, so it refutes "no highway" claims (after
   reproduction on a second board geometry and delta-debugging shrinking). A start that has not escaped by the
   horizon refutes nothing about "highway from every start"; it is reported `Unsupported`, never `Falsified`.
2. **Horizon attack.** `horizon` is what the author looked at; trials run to `horizon * 4^ladder` and the report lists
   confirmed highways by `horizon * 4^k`. A refutation whose every counterexample appears after the stated horizon
   is flagged `horizon-sensitive`.
3. **Instrument controls always run.** Four headings of the blank start (rotations of one another) and blank starts
   near an edge of odd-sized boards (same dynamics as the centred start). If either disagrees with the baseline
   the verdict is `Invalid`, because the detector, not the rule, is then at fault.
4. **Valid perturbations only.** Finite non-blank cells in a blank far field (the highway detector assumes the
   ant walks into blank cells). Dense random backgrounds are out of scope.
5. **Highway types compare up to the lattice symmetries**: `(period, |long drift|, |short drift|)`.
6. **Determinism.** Every trial is a function of (claim seed, rule, class, index); the adaptive chains are
   sequential inside a chain and parallel across chains; rows are sorted before writing.

### Limits (stated, not hidden)
- "Survived" means "no counterexample in N attacks at this horizon", with the Clopper-Pearson bound for probabilistic
  claims; it is never a proof.
- Chaotic claims are only refuted by highways. A rule that is "chaotic" at horizon h but bounded/periodic in some
  other way is not tested for that. Growth-exponent claims (the t^(1/3) question) are not covered yet.
- The adaptive objective for chaotic claims (smallest bounding box) is a heuristic, not derived.
- The board must be large enough for the claimed highway to exit within the horizon (drift/period times size/2).

## Registered predictions (written 2026-10-22 21:35 +13:00, before any attack was run)
Setup: `claims.txt`, 64 trials per class, all ten classes, adaptive 4 chains x 40 evaluations.

| # | prediction | confidence |
|---|---|---|
| P1 | No claim ends `Invalid`: heading and boundary controls agree with the baseline on all five rules. | 85% |
| P2 | C1 (RL, highway, type (104;2,2)): no counterexample of another highway type (`hw_other = 0` in every class). Verdict `Unsupported` rather than `Survived`: at least one dense start (half or sparse) is still unresolved at 2e6 steps. | 65% / 60% |
| P3 | C2 (RRL, type (18;1,1)): `Falsified` by at least one patch/mono/sparse start ending in a highway of another type. | 55% |
| P4 | C3 (RRRL, type (52;2,2)): `Falsified` likewise. | 50% |
| P5 | C4 (RLL chaotic, horizon 1e5, attacked to 1.6e6): `Survived`, zero highways in all attack classes. | 75% |
| P6 | C5 (RRLR chaotic, horizon 1e5): `Falsified`, flagged horizon-sensitive. Census onset is 256240, so the blank start itself escapes only after the stated horizon; rung 0 (by 1e5) shows 0 highways, the top rung shows at least 1. | 90% |
| P7 | The adaptive hill-climb finds a longer delay before escape than the best of the 64 random `patch` starts in each of C1-C3 (the adversary beats sampling). Computed by `scripts/adaptive_vs_sampling.sh`. | 60% |

What would count as the harness failing: P1 false (a control disagrees) means the detector has a bug to fix before
any claim is believed; P6 false means the horizon attack does not do what it was built to do.

## Reproduce
```
cargo build --release --locked --bin falsify
target/release/falsify experiments/2026-10-22-falsification-harness/claims.txt experiments/2026-10-22-falsification-harness/results
```
Provenance (command, commit, claims blob hash) is the header of every `results/*.csv`.

## Results
(appended below after the run; predictions above are not edited.)
