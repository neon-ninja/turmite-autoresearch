# <Experiment title>

## Question and registered prediction

<Timestamp with timezone, starting git revision, question, mechanism, quantitative
prediction, failure criterion and competing explanation. Register before running.>

## Pruning-index preflight

Consult `notes/pruning-index/README.md` before each new sweep or variation.

- Index revision and search commands: <full hash and queries>
- Candidate signature IDs: <IDs or explicit no matches>
- Scope comparison: <membership, objective, initial conditions, seeds, horizon, instrument>
- Decision: <run / skip / reopen; reasons and any replication/control exception>
- Run spec and exclusions: <committed paths; per-exclusion signature and reason>
- Compute/stopping budget: <bound and what result warrants the next run>

## Reproduction

<Scripts in scripts/, exact commands, locked environment, seeds, step counts,
windows, inputs and hashes, code revision, checkpoints and output locations.>

## Results, uncertainty and limitations

<Link committed results/, distinguish observations from interpretations, report
failed/skipped tests, controls and sample sizes. Do not hand-edit generated data.>

## Pruning-index outcome and next question

<New signature ID, amendment to an existing ID, or explicit reason no signature
is warranted. Copy DEAD_END.md only when there is evidence; commit evidence first.
Record what would reopen a dead end. Update NOTEBOOK.md, notebook.ipynb and the
relevant notes; link blog reflections when appropriate.>
