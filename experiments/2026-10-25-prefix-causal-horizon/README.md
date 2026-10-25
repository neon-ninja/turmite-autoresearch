# Prefix causal horizon: what a rule's later letters cannot yet have done

Registered 2026-10-25, base `a25ab35`, branch `ccr-9a56d193-javkym`. Status: **REGISTERED; nothing run.**
Prompted by the user's remark that the prefix analysis may be amenable to formal methods, and by the Q19 work
(`experiments/2026-10-25-late-highway-fourth-decade/`), where 680 of 702 survivors extend another survivor and
prefix-sharing highway rules had identical onsets.

## Question

Several notes treat "rules sharing a prefix" as if they shared dynamics for a while (identical late-highway onsets, the kappa prefix
lookups). Is there an exact statement of *how long*, can it be proved for all rules rather than tested on samples, and does it say
anything about the data we have?

## Statements (to be proved, and tested against the oracle)

Rules `w` (length `n`) and `w'` (length `n'`), blank board, start at the origin, one step = read colour `c`, turn by `rule[c]`,
write `(c+1) mod n`, move. Steps are numbered from 1. `lcp` = length `p` of the longest common prefix. `F_c(w)` = the number of the
first step that reads colour `c` (a cell's `(c+1)`-th visit, for `c < n`; colour `c` is written on visit `c`).

* **Clock lemma (K).** For `c < n` colour `c` is read for the first time on the first `(c+1)`-th visit to any cell. Hence `F_c` is the
  record time of the maximum per-cell visit count, and is the same number for every rule whose prefix of length `c` is the same
  (it needs only the first `c` letters; see A).
* **Lemma A (diverge exactly at the first read of the first differing colour).** If `p < min(n, n')` then positions agree after steps
  `0 .. F_p - 1` and differ after step `F_p`, where `F_p` is computed in either run (they coincide until then).
* **Lemma B (prefix case).** If `w` is a proper prefix of `w'` (`p = n < n'`): positions agree after steps `0 .. F_n(w') - 1`, and
  differ after step `F_n(w')` iff `w[0] != w'[n]`.
* **Lemma L (lift / periodic extension).** Let `q` = least `c < n'` with `w'[c] != w[c mod n]` (infinity if none), and `W` = first step
  that reads a cell previously written 0 by a wrap of `w'` (a cell visited `n'` times). Then positions agree after steps
  `0 .. min(F_q(w'), W) - 1` (and, when `n | n'`, the wrap term `W` is dropped).
* **Lemma R (roots).** If `n | n'` and `w'` is `w` repeated, the two trajectories are identical forever (already used in `src/rule.rs::root`).

Pen proof for A: by induction on steps, the pair of states is equal (position, heading, every cell) as long as the colour read is `< p`,
because the letters and the successor `c + 1` coincide for `c < p < min(n, n')`; the first read of colour `p` turns by different
letters, and the four headings give four different moves. The induction is over an unbounded grid, which is why the step obligation is
checked by SMT over an array-valued grid rather than over finite boards.

## Registered predictions (before any run)

* **S1.** z3 proves the inductive step (negated goal `unsat`) for: colour-range invariant, A, B-step, L (concrete lengths 1..12, every
  letter pattern symbolic), R. For A with symbolic `n, n', p` too.
* **S2 (instrument calibration, must come out `sat`).** Negative controls: A with the guard widened to `c <= p`; L without the
  turn-agreement premise; L without the wrap flag when `n` does not divide `n'`; R with `n` not dividing `n'`.
* **R1.** Exhaustively for all rule pairs with `n, n' <= 6` (cap 4000 steps, reference ant on both): A holds with equality, B holds,
  L's bound holds, R holds, and the number of pairs not reaching their `F_p` within the cap is reported (not silently dropped).
  A proptest at lengths up to 12 agrees. A failure would be a bug in the statement or in `first_reads`.
* **R2 (scaling).** For the 702 survivors of `experiments/2026-10-25-late-highway-fourth-decade/results/survivors_1e11.txt`, `F_c(w)`
  grows like `c^gamma` with gamma in [2.5, 3.5] (a cube root of time for the maximum visit count, as for the mean). 55% confident;
  outside [2, 4] I would call it wrong. Every survivor reads its last colour (`F_{n-1}`) before 1e7 steps: 85%.
* **R3 (consequence for Q19).** Because `F_p` is small (R2), prefix sharing makes rules identical only for the first ~1e4-1e6 steps:
  of the 21 groups of survivors with identical box at 1e11, at most 3 contain a pair that Lemmas A/B/L cannot separate before 1e11.
  So equal boxes are coincidences of boundedness/size, not shared trajectories, and "dedupe by prefix" saves nothing. 75% confident.
* **R4 (filler pairs).** For the 54 late-highway onsets, every pair with identical onsets (within 1e-4 relative) is explained by the
  lemmas: a Lift or prefix pair whose bound exceeds the onset. The pairs with 6+ shared letters that the earlier note counted
  as "prefix" evidence but that are Case A with `F_p` far below the onset would contradict the lemma or the note. 70% confident that none do.

## Pruning-index preflight

Index revision `a25ab35`; `rg -n -i -e prefix -e lcp -e "shared trajectory" notes/pruning-index/entries`: no candidates (the one entry
concerns additive conservation laws). Decision: run. A proof is not a dead-end claim; no signature planned.

## Reproduction (filled in as run)

```bash
uv run --frozen experiments/2026-10-25-prefix-causal-horizon/scripts/prove_prefix.py        # z3; PEP 723 header, no change to uv.lock
cargo test --release --locked --test prefix_horizon
cargo run --release --example prefix_horizon -- ...
```

## Addendum 2026-10-25 (before R1-R4 or the Rust test were run): the sharper unified form

Working through the Lift proof I noticed that `W` (first read of a wrapped cell, visit `n'+1`) always exceeds `F_c(w')` for every
`c < n'` (a cell cannot have `n'+1` visits before it has `c+1`), and no wrap happens before `F_{n'-1} >= F_q`. So Lemmas A, B and L
collapse into one statement. For `n <= n'` and `q` = least `c < n'` with `w'[c] != w[c mod n]` (infinity if none):

* **(i)** if `q < n'`: the first step after which the positions differ is **exactly** `F_q(w')`;
* **(ii)** if `q = infinity` and `n | n'`: the trajectories are identical forever;
* **(iii)** if `q = infinity` and `n` does not divide `n'`: positions agree at least until `W(w')`, the first step that reads a cell
  visited `n'` times before.

(A is (i) with `q = p`; B is (i) with `q = n` when `w[0] != w'[n]`, otherwise (i) with a later `q` or (iii).) This is the form R1 tests;
the registered lemmas are its special cases, so a failure of either is a failure of the registration. The z3 lift obligation is still
needed for (iii) (the wrap flag), and for (i) up to `F_q`.
**z3 note (first run, before this addendum):** the negative control NC1 returned `unknown` rather than `sat` because the quantified
range premises stop z3 from building a model, so S2 as written is not yet a valid calibration. Controls will be restated with
quantifier-free premises (the grid is the zero grid plus the cell under the ant) and rerun; this is a deviation from S2, not a result.
