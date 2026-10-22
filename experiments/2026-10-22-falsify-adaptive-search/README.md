# Making the falsification harness's adaptive search beat random sampling (2026-10-22)

## Question
In `experiments/2026-10-22-falsification-harness` the adaptive attacker lost to 64 random patches in 3 of 3 highway claims
(longest escape delay: RL 76386 vs 94565, RRL 19466 vs 20118, RRRL 27726 vs 79915). Is that because hill-climbing is
the wrong tool here, or because this particular climber was badly built?

## Diagnosis (observed 2026-10-22, from the committed CSVs, before any new code)
- Escape delay is heavy-tailed and plateau-dominated. RL: typical 53-62k, rare 94k-123k. The rare values come from
  `sparse` (max 114116) and `obstacle` (max 122842), not from dense patches (patch max 94565, class mean 57756).
  RRRL: typical 26.5-27.5k, rare 79-80k from `mono` and `patch`.
- The old chain (a) started from a dense radius-3 random patch, which is typical rather than tail; (b) mutated one
  cell at a time on a plateau, so almost every step is a tie or noise; (c) never touched the obstacle (a drop in front
  of the highway), the one lever that gave the largest delay on RL.

## Hypothesis
The landscape is rugged but has structure the old search ignored. An evolutionary search that (1) seeds its population
from the same random draws as the control, (2) keeps an elite set, (3) mutates at several scales (recolour, add, delete,
shift, heading) and (4) includes the obstacle's parameters as genes, will find longer delays than random search at the
same evaluation budget.

## Design (fixed before measuring)
- Claims: C1 RL, C2 RRL, C3 RRRL, as registered in `../2026-10-22-falsification-harness/claims.txt` (horizon 2e6, board 2048).
- Three arms, each given exactly B = 200 evaluations per replicate, 30 independent replicates per claim:
  - `random`: 200 draws, each a uniformly chosen class of {patch, mono, half, stripe, sparse, obstacle}, via the harness's `make_trial`.
  - `old`: the harness's existing `adaptive_chain` (one-cell hill-climb), 200 evaluations.
  - `evolve`: 50 random draws as in `random`, then (mu + lambda) search, mu = 8 elites, 150 children, each a 1-3 stacked
    mutations of a uniformly chosen elite.
- Score (higher is the attacker's win): escape step for a highway of the baseline type; horizon*2 for an unresolved or edge
  escape; 4*horizon for a highway of a *different* type (a counterexample). Metric per replicate: the maximum score over its
  200 evaluations. Comparison: one-sided Mann-Whitney between arms over the 30 replicates (normal approximation, tie-corrected).
- Output is keyed by (claim, arm, replicate); thread count must not change it (checked).

## Registered predictions (2026-10-22, before any run of the new code)
| # | prediction | confidence |
|---|---|---|
| E1 | `evolve` beats `random` (median max-score higher, one-sided Mann-Whitney p < 0.01) in **both** C1 (RL) and C3 (RRRL). | 55% |
| E2 | In C2 (RRL, thin tail) `evolve` vs `random` is not significant at p < 0.05. | 60% |
| E3 | `old` does not beat `random` (p > 0.05) in any claim, reproducing the earlier loss under this paired design. | 75% |
| E4 | In C3 `evolve` finds a different-type highway (a counterexample) in more replicates than `random` does. | 50% |

If E1 fails the harness's adaptive class stays labelled "extra random samples" and no claim should lean on it. If E1
holds, it shows only that this search beats this sampler on these three rules; it is not a general adversary.

## Reproduce
`cargo run --release --locked --bin falsify_search -- experiments/2026-10-22-falsify-adaptive-search/results` (provenance
header: command, commit, and a hash of any uncommitted `src/`+`tests/` diff, which is empty for a clean run).

## Results
(appended after the run; predictions above are not edited.)
