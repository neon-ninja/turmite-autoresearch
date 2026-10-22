# <NR-YYYYMMDD-experiment-claim>: <precise claim>

Template only. Publish a completed copy to `notes/pruning-index/entries/<id>.md`
after committing its evidence. See `notes/pruning-index/README.md`.

- Schema: 1
- ID: <unique ID matching filename>
- Registered: <ISO timestamp with timezone>
- Author: <name; identify AI agents>
- Initial status: <active | reopened | superseded | withdrawn>
- Evidence class: <proved | empirical | inconclusive>
- Objective and search keywords: <question, observables, rule/family names>
- Claim and permitted pruning decision: <what exactly may be skipped/deferred>

## Topological dead-end signature

- Model and update semantics: <lattice, state/color transitions, walkers>
- Rule-space vertices and adjacency: <domain, lengths, edge definition>
- Membership predicate: <exact, decidable predicate or pinned enumeration>
- Canonicalization/equivalences: <none by default; mapping and justification>
- Invariants: <definition, extractor path/version, or not used>
- Scope: <initial states, backgrounds, seeds, boundaries/grid, noise>
- Observable and instrument: <definition, code revision, detector/fit settings>
- Horizon and windows: <steps, sampling, fitting windows; proof domain if universal>
- Exclusions: <conditions and observables not covered>

## Evidence and limits

- Preregistered criterion: <README path, pinned revision and threshold>
- Evidence revision: <full git commit of already committed evidence>
- Inputs/results: <committed paths and SHA-256 hashes>
- Recipe: <exact commands, seeds, parameters, environment and binary hash if required>
- Proof or measured result: <argument/checker, or sample sizes and uncertainty>
- Controls and oracle checks: <outcomes, calibration bias, skipped checks>
- Confounds: <finite-time effects, seed coverage, detector errors>
- Why the evidence covers the predicate: <proof; empirical claims list tested members only>

## Reopening and cross-references

- Reopen if: <falsifiable counterexample or changed condition>
- Discriminating next test: <what new evidence would matter>
- Experiment / lab notebook / scientific note: <links>
- Related signature IDs: <IDs or none; explain overlap/conflict>

## Amendments (append only)

<Timestamp, author, evidence commit, reason, resulting status, replacement ID if any.>
