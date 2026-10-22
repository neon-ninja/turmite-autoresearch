# Invariant mining, stage 2: all moduli at once, sublattice families, and what the rule can and cannot change

Companion to `experiments/2026-10-22-invariant-mining/` (a parallel agent's enumerative miner for
`I = a*x + b*y + d*h + e*t + sum w(colour)` at moduli 2 and 4). That folder is not edited here. This
one asks what the enumerative miner cannot: which laws exist at *every* modulus, which laws are
merely universal (true of every unit-step walker that turns by 90 degrees, with an n-colour counter,
whatever the rule), and how much of the rest depends on the rule at all.

## Question and registered prediction

Registered 2026-10-22 21:50 +13:00 (NZDT), starting revision `b0c9e8a3be66d75176fb27d0b54d758b17558272`,
before any solver code exists. Nothing below has been computed yet.

**Question.** For an n-colour ant, what is the full group of additive local conservation laws, and does
it carry any information about the rule beyond what is already visible in `(n, #R)`?

### Ansatz families (fixed before mining; no weights are searched and a "best" one reported)

A law is `V = a*x + b*y + e*t + Phi(h, s(pos)) + sum_cells w(s(cell), colour)  (mod m)`, with the
normalisation `w(s, 0) = 0` (the blank field contributes nothing) and one `Phi` value fixed to 0. It is a
*proved* invariant when `V` is unchanged by every local transition the family admits (cell colour `c`,
heading `h`, class `s` of the ant's cell), colour wraps `n-1 -> 0` and heading wraps included. Each
condition is linear over `Z/m`, so all moduli follow from one Smith normal form over `Z` of the
integer constraint matrix.

| family | site class `s` | transitions required | `Phi` | meaning |
|---|---|---|---|---|
| A0 | none | all `(c, h)` | `d*h` (integer heading, as in the parallel miner) | cross-check ansatz |
| F1 | none | all `(c, h)` | arbitrary function of `h` | superset of A0 |
| F2 | `(x+y) mod 2` | `(c, h, s)` with `h = s (mod 2)` | arbitrary on reachable `(h, s)` | sublattice potentials |
| F3 | `(x mod 2, y mod 2)` | `(c, h, s)` with `h = x+y (mod 2)` | arbitrary on reachable `(h, s)` | finer lattice classes |

The restriction "`h + x + y` even" in F2/F3 is itself a proved invariant (every step flips the parity of
`x+y` and of `h`), and the blank start `(h, x, y) = (0, 0, 0)` lies on it, so laws proved on that
manifold constrain every blank-start trajectory. `A0 <= F1 <= F2 <= F3` as sets of functionals.

The *universal* group `U_n` of a family is the solution set when every colour is allowed either turn
(equivalently the intersection over all `2^n` rules of length `n`, and equivalently the "random-turn
walker with an n-colour counter" negative control). The *rule-specific content* of a rule is the
quotient `G(rule) / U_n`, summarised through the Smith invariant factors ("fingerprint": free rank and
the sorted nontrivial factors `d_i`; the law group at modulus `m` is `Z_m^free x prod Z_{gcd(d_i, m)}`).

### Derivation behind P1 (written before computing)

For a fixed rule, a law forces, per colour `c`, the quantity
`E_c(h, s) = a*dx + b*dy + e + Phi(h', s') - Phi(h, s)` (with `h' = h + turn_c`, `(dx, dy)` the step in
direction `h'`) to be independent of `h` (the cell potential cannot see the heading), and then
`w(s, c+1) - w(s, c) = -E_c(s)`. Going once round the colour cycle leaves one condition per class:
`#R * E_R(s) + #L * E_L(s) = 0 (mod m)`. The order of the colours never enters.

### Predictions

| # | prediction | confidence | failure means |
|---|---|---|---|
| P1 | For each of F1, F2, F3 the fingerprint of a rule is a function of `(n, #R)` alone: zero exceptions among all 510 rules of length 2..8 (non-primitive and non-canonical included). Corollaries: complement, reversal and every cyclic shift preserve the fingerprint. | 90% | derivation error or solver bug |
| P2 | A0 agrees with the parallel miner: for every rule of length 2..8 and `m` in {2, 4}, (number of nonzero tuples listed by `invariants RULE`) + 1 = size of the A0 solution set from the SNF solver. | 85% | one of the two solvers is wrong; resolve against the oracle |
| P3 | The universal F1 group at `m = 2` has order `2^3` for even `n` and `2^2` for odd `n` (checkerboard parity, heading parity, and for even `n` the colour clock). At odd prime `p` the universal F1 group is trivial unless `p` divides `n`, when it has order `p` (colour clock only). | 80% | a universal law I did not enumerate |
| P4 | In F1 at `m = 4`, a rule has a law with `a = b = e = 0` and non-constant `Phi` (heading determined by the tape; for Langton's ant, heading = number of colour-1 cells mod 4) **iff** `D = #R - #L = 0 (mod 4)`. Checked on all 510 rules. | 90% | derivation error |
| P5 | Oracle: every SNF generator of A0, F1, F2, F3 for every rule of length 2..6 at `m` in {2, 3, 4, 6, 8, n, 2n, 4n} is conserved along 2000 reference-ant steps from 16 seeded random starts on the manifold: 0 violations. Control: random vectors outside the solution set are violated within the same runs in at least 99% of draws (so the checker has power). | 95% | solver, sign convention or checker bug |
| P6 | Sublattice accounting is new: at modulus `n` the universal order satisfies `|U_n^{F2}| > |U_n^{F1}|` for every `n` in 2..8, because the ant alternates sublattices (departures from even cells minus departures from odd cells = `t mod 2`). | 75% | no sublattice accounting law (my argument is wrong) |
| P7 | The universal F3 order exceeds F2 at modulus `n` for at least one `n` in 2..8. | 45% | finer classes add nothing |
| P8 | Tile-free consequences for highways: the laws with every `w = 0` (no dependence on the laid tile) constrain `(period, dx, dy)` only through "period even" and "`dx + dy` even". All confirmed highways in `experiments/2026-10-13-census-sweeps/results/confirmed_*.csv` obey every such law (`a*dx + b*dy + e*period = 0 mod m`). I expect no constraint beyond the known H19. | 85% | detector bug (a violation) or a genuinely new constraint (a surprise) |

If P1 holds, every law in these families is a function of `(n, #R)`, so the instrument cannot separate
`RRLL` from `RLRL`. That would be a finding (an order-blind class of invariants), not a failure of the
instrument, and it is what I expect.

### Explicitly not tested here (open)
- Laws that couple the ant to the colours of neighbouring cells, pair potentials, edge flux, any
  history dependence. These are the only places left where rule *order* could enter. A pair-potential
  ansatz needs a quotient by functionals that vanish on all finite fields (telescoping
  `kappa(c) - kappa(c')` terms and duplicates of site potentials), otherwise group orders are inflated;
  not built.
- Any claim about growth exponents or highway formation beyond P8.

## Pruning-index preflight

- Index revision `b0c9e8a3be66d75176fb27d0b54d758b17558272`. `rg --files notes/pruning-index/entries`
  returned nothing (no entries). `rg -n -i -e invariant -e conservation -e parity -e symmetr -e modular
  notes/pruning-index/entries` returned nothing (rc=1).
- Candidate signature IDs: explicit no matches (a no-match is a recorded outcome, not evidence of novelty).
- Scope: exact algebraic computation on all rules of length 2..8; the solver has no randomness; the
  oracle check uses seeded starts.
- Decision: run. No exclusions. No family pruned.
- Compute budget: seconds to minutes on one core; stop after the registered tables.

## Reproduction

Filled in as runs complete. Library: `experiments/_lib/lawmod.py`. Scripts: `scripts/`.

## Results, uncertainty and limitations

Appended below this line only, after each run; the predictions above are not edited.

## Pruning-index outcome and next question

To be appended.
