# Output schema (proposed; not implemented)

`dumps/loops.csv` (gitignored, bulk): one row per (rule spec `RULE@SEED`, window k in 20..26). Columns, in order:

`rule, k, t, steps, escaped, area, perimeter, new_cells, boundary_steps,`
`loop_lead_b, loop_lead_i, loop_other_b, loop_other_i,`   loop steps by kind, on boundary (b) / interior (i) cells
`entry_lead_b, entry_lead_i, entry_other_b, entry_other_i,`   loop entries, same split
`laps_1, ..., laps_8, laps_gt8`   completed-run counts by lap length, lead-colour loops

The first nine columns must reproduce the corresponding `probe` CSV columns for the same spec (identity test). Derived:
loop_frac = (sum of the four loop_*) / steps.

`results/loops_per_rule.csv` (committed, small): per-rule aggregates the notebook reads, with the command, commit and seed in a header line.
