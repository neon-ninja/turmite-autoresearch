# Negative-result pruning index

This directory is the shared, versioned database of dead-end signatures. Each
`entries/<id>.md` is a structured record following
[`DEAD_END.md`](../../experiments/_template/DEAD_END.md). Git supplies history and
review; no external service or generated database is required. The index starts
empty: existing negative results must be audited before being registered.

## Query before spending compute

From the repository root:

```text
git rev-parse HEAD
rg --files notes/pruning-index/entries
rg -n -i -e "RULE_OR_FAMILY" -e "OBJECTIVE_OR_OBSERVABLE" notes/pruning-index/entries
```

Replace the search terms with literal rule strings, family names, objective and
observable names; use `rg -F` for literal-only searches. Also inspect the titles
and membership predicates of all records: a family predicate may cover a rule
without spelling out its string. Text search retrieves candidates, not a sound
membership test. Read each candidate's full evidence, conditions and amendments.
There is no automatic filtering of sweep inputs.

Log the revision, queries, candidate IDs, membership reasoning and decision in the
experiment README and NOTEBOOK. A no-match search is a recorded outcome, not
evidence that a rule is novel. Before launching, compare the actual run spec to
the recorded conditions and save the included and excluded specs with reasons.

## Signature semantics

A signature identifies a claim about a rule subspace **for a stated objective**.
Specify vertices (valid rules and any additional model parameters), edges (for
example one L/R substitution at fixed length), and an exact membership predicate
or committed enumeration. A neighborhood radius alone is not evidence. Describe
any transition-graph or orbit invariant precisely, with its extractor and version;
an informal shape label is not a topological invariant.

Default matching uses exact rule strings and conditions. If quotienting by mirror
symmetry or primitive roots from `src/rule.rs`, justify preservation of the
observable and transform initial states/backgrounds accordingly. Primitive-root
reduction need not preserve full color statistics. Do not assume cyclic shifts,
reversal, common prefixes, or equal turn counts imply equivalent dynamics.

Required dispositions:

| Evidence class | Permitted decision |
|---|---|
| `proved` | Skip only when a checked proof covers the membership predicate, objective and all proposed conditions. |
| `empirical` | Defer repeating the same tested rules, backgrounds, seeds, budgets, windows and instrument for the same objective; no extrapolation to neighbors or longer times. |
| `inconclusive` | Keep searchable as a warning about method or cost; never prune on this basis. |

Specify quantitative failure/triviality criteria before measuring. Report sample
sizes, uncertainty, controls and possible delayed transitions. “Unstructured
chaos”, “dissipation” and “uninteresting” require model-specific definitions and
an objective; they are not standalone pruning criteria for conservative ants.
Do not populate records with fabricated examples presented as observations.

## Commit and reopen

First commit reproducible evidence in the originating experiment. Then add a
fully completed signature referencing that full commit hash, result paths and
hashes, exact commands and any proof/checker. Use an ID such as
`NR-YYYYMMDD-experiment-claim`; check for collisions and existing equivalent claims
before adding it. Link the ID from the experiment and relevant scientific note.
Unfilled templates live only in `_template/`, never in `entries/`.

Records are append-only after publication. Append dated amendments with an
evidence revision, reason and replacement status (`active`, `reopened`,
`superseded`, `withdrawn`); link replacement IDs where applicable. The latest
unambiguous amendment determines status. Never erase the original claim.
Check all matching records: contradictory evidence, conflicting amendments,
missing evidence or uncertain membership mean **run/review, not skip**.

Reopen on a counterexample, changed objective, instrument correction, newly
relevant background/seed, increased horizon beyond empirical coverage, or a
failed proof assumption. Register the discriminating test and compute budget
before running it, and append the outcome even if negative. Explicitly justify
deliberate replication and retain calibration controls. No signature prohibits a
well-motivated challenge; it prevents unexamined repetition.
