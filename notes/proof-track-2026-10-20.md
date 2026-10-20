# Proof track: from simulated regularities to checked macro-transitions (2026-10-20)

Started 2026-10-20 from a proposal by the project owner. This runs **in parallel with** the empirical programme. It does
not replace it, and it reuses the same oracle, records and registration rules. Experiments live in their own folders as
usual; this note holds the cross-cutting plan and what counts as success.

## Why run it alongside the empirical work

The empirical track asks *how much* (exponents, prefactors, kappa, the c law) and answers with calibrated estimates. The
proof track asks *must it continue, and why*, and answers with a finite obligation that a checker accepts or rejects.
The two tracks feed each other:

- Simulation supplies the conjectures: a period-3 wake, a cycle time proportional to height, a bounded reach behind the
  front. Without those observations there is nothing to prove.
- A proof turns a fitted law into an exact one, and can say which part of a fit is finite-size. For example, if the
  RLRLLL sweeper is the macro-transition proposed in `experiments/2026-10-20-rlrlll-macro-transition/`, the fitted
  `|x|^2 = 0.8634 t - 1.09e10` should have true asymptotic slope 0.75, not 0.86.
- A *failed* abstraction is informative too: if no bounded-reach macrostate exists, the behaviour needs its wake as
  state, which is a precise statement of why it resists simple description.

Compute does not conflict: the proof track needs single long runs (minutes on one core) and small finite checks, not
sweeps. It should not start heavy jobs while the 10^11 census holds the cores (`notes/ACTIVE.md`).

## Levels, and what counts as each

1. **Simulation**: a rule does X over a finite interval (the existing record).
2. **Experimental mathematics**: a regularity stated exactly enough to fail (zero-residual clock, exact block decomposition),
   registered before measurement.
3. **Formal analysis**: a finite proof obligation (base case plus induction step over a parameter such as column height),
   discharged by a checker independent of the code that found the conjecture.
4. **Theory**: the same theorem, or the same proof template, holding for a family of rules for a structural reason.

## Vocabulary of mechanisms (candidate formal objects)

| Structure | Formal object | Repo source |
|---|---|---|
| Highway | translation-periodic orbit (finite check) | `src/detect.rs`, census |
| Column sweeper | parameterised inductive invariant `C(L, q) -> C(L+g, q')` | `notes/g1-builder-structure-2026-10-14.md` |
| Crystal ray | finite-state transducer / regular language of ray words | `experiments/2026-10-crystal-rules/` |
| Tethered ant | trapping certificate: invariant region the ant cannot leave | `experiments/2026-10-13-tether-rules/` |
| Boundary excursion | return map on boundary states (must be shown well defined) | `experiments/2026-10-14-t1-excursion-tomography/`, kappa work |
| Long transient | transition between metastable macrostates (entry problem) | `experiments/2026-10-13-highways-unresolved-rules-1e10-1e11/` |

## Architecture rule

Conjecture generators (agents, scripts, search) may be wrong; they propose blocks, phases and invariants. **Acceptance
comes only from a checker**: a small, separately written program (or proof assistant) that replays the finite lemmas
against `src/reference.rs` semantics. An agent may not mark a statement proved on its own say-so. Every checked theorem
records the checker's version and input hashes, like any other result.

## Known limits (stated up front)

- A proof that a macrostate persists is not a proof that a blank-board run enters it. For RLRLLL, persistence and entry
  (at ~1.2677e10 steps) are separate problems; entry may be out of reach.
- Undecidability rules out a general procedure for all rules, not strong theorems about subclasses.
- The cube-root growth of chaotic rules is a statistical claim; the proof track does not expect to prove it soon. The
  diffusive heuristic `dL/dt ~ 1/L^2` is already close to the existing transport decomposition (`c kappa`, boundary access);
  a useful contribution would be a *model class* in which that heuristic is a theorem, not a proof for real ants.

## Prior art to check (from memory, not yet verified against sources)

- Busy Beaver Challenge (bbchallenge): deciders for translated cyclers and "bouncers" (machines sweeping a growing tape),
  and a Coq-checked BB(5) proof (2024). Bouncers are the 1D analogue of the column sweeper; their shift-rule induction is the
  template for Stage 2 here. A turmite is a 2D Turing machine, so the analogy is direct.
- Gale, Propp, Sutherland and Troubetzkoy, "Further travels with my ant" (Math. Intelligencer, 1995): proofs of symmetric
  growth for some generalised ants. Bunimovich and Troubetzkoy (1992): the unbounded-trajectory theorem for Langton's ant.
- Gajardo, Moreira and Goles (2002): Langton's ant is computationally universal (limits on what can be decided).

These must go through the literature-review process (`experiments/2026-10-15-literature-review/`) before being cited
in the paper.

## Experiments in this track

- `experiments/2026-10-20-rlrlll-macro-transition/`: is the RLRLLL sweeper a macro-transition (Stage 1 measurement,
  predictions P1-P5 registered at `4bc029a`), and if so, can it be checked for all heights (Stage 2)?
  **Result (2026-10-20):** P1-P5 held; the three-phase family is closed for all even m >= 2 (checker plus brute-force
  cross-checks and negative controls). First level-3 result of the track; see that README for what it rests on.
  **Entry (2026-10-21):** the cell-level family holds from step 12,676,237,591 (C(4,1) at x = -1552; base case moved 769,586 steps earlier, 184,176 before the edge crossing); the clock formula extends exactly to m = -24 (x = -1510, height-19 first column), so the wedge does not start at zero height; registered E1, A2, B2 failed. The 3-row light-cone margin is observed, not derived from the checker.
  **Task C (2026-10-21):** (C1) an independent second checker (stdlib Python, own ant, own pipeline S1 . descent^n . S3 . return^n . S5; `c1b.py`) reproduces 12m + 934/1058/898, lemma periods 2 (20 and 4 steps), m0 = 2, and 150 brute-force cycles; its ant agrees with `reference::Ant` on 1e6 steps and 12 windows; controls: broken ants and a mutated bulk cell rejected as registered, `RLRLLR` rejected at a different place than the first checker (scored split). (C2) cycles visit at most 2 rows above and 1 below their rectangle and up to 18 columns east of the front; the exact criterion (real configuration = ideal initial configuration on every cell the ideal ant visits, 97 cycles) passes in 50 of 50 windows including the base case C(4,1), but Task A's cone (3 + s//3) was too small (excess up to 7 rows); registered C2a failed. (C3) Lean 4: frame lemma and the two shift lemmas as iteration theorems for all n, six kernel-decided certificates (`experiments/2026-10-20-rlrlll-macro-transition/lean/`); the end runs, composition and cycle-to-cycle padding are still paper steps.

## Next steps (proposed, none started)

1. *Assurance* (partly done 2026-10-21, Task C): second checker and Lean shift lemmas exist. Left: Lean statements of S1, S3, S5 and the composition identity for all n (the generator and the iteration theorems are in place; needs a bounded-run semantics for "first reaches column -3" and a padding story across cycles), and a re-run of the 1.27e10-step base case by a second simulator.
2. *Generalise the checker*, not the rule: run the same propose-then-check loop on the other no-highway escapes
   (`RLLLLRRL`, `RLLLLRLRRL`, `RLLLRLRLLL`) and on every census highway (a highway is the m-free special case). A
   "sweeper decider" over the census would be the proof-track analogue of the bbchallenge deciders.
3. *Entry* (mostly done 2026-10-21): family at the cell level from m = 4, clock to m = -24; cone margin derived (exact criterion, Task C; the observed 3-row margin was too small); open: explain the clock at m < 2, and what at the blob NW corner makes a height-19 first column.
4. *Explain, not just certify*: why the bulk row is `14435435...` and why the colour period is 3 for a length-6 rule.
