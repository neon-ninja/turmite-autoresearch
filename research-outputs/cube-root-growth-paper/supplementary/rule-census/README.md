# Supplementary dataset: per-rule census table

`rule_census.csv` has one row per rule swept (all 4083 canonical, non-trivial
L/R rules of length 2-12) and 60 columns. It collects every per-rule quantity
behind the paper's census tables and figures. Nothing is re-measured: the file is
a join of committed result files, rebuilt by

```bash
uv run research-outputs/cube-root-growth-paper/supplementary/rule-census/build_rule_census.py
```

(deterministic, no randomness, rows sorted by `(n, rule)`; `rule_census.build.json`
records row counts and the git HEAD at build time). Every rule starts from a blank
board, so a rule has exactly one trajectory and there are no seeds.

Rule notation: character `c` of the string is the turn taken on colour `c`
(`L`/`R`); the cell then advances to colour `(c+1) % n`. `RL` is Langton's ant.

## Which rows are "the census"

`in_census` is true for 3512 rows. The other 571 rows are rules the
10^8-step sweep removed as highway builders (`phase = highway`), kept so the
table partitions the whole sweep. Counts by `phase`: blob 2637, log 571,
builder 304, highway 571. (The paper's "578 highway/ballistic" counts 571 plus
the 7 rules that escaped the 8192^2 grid late without a detected highway but
are kept in the census; these are flagged by `census_late_escape`.)

## Columns

Identity and class
- `rule`, `n` (length), `n_L`, `n_R` (letter counts).
- `primitive_root`, `is_repeat`: shortest `u` with `u^k = rule`; repeats behave
  identically to their root (52 census rules are repeats).
- `phase`: `highway` (removed by sweep), else the census class `builder` /
  `log` / `blob`. Builder: did not complete 2^27 steps inside the grid, or
  fewer than 20 steps per bounding-box cell at 10^8 steps. Log: fitted width
  exponent over t = 2^21..2^27 below 0.15. Blob: otherwise.
  Source: `census.py`.
- `in_census`, `census_late_escape`, `analysed` (has probe statistics: 3055
  growing rules; the other growing rules had a window with no new cells).

`sweep_*`: highway sweep, 10^8 steps, 8192^2 grid, with detector
(`sweep_v2.csv`). `stop` is `budget` or `escaped`; `steps` taken;
`period`, `dx`, `dy`, `onset`: detected highway period, displacement per period
and onset step (empty if none; 2 escaped rules have no detected highway);
`width`, `height`: bounding box at stop; `tail_period`; `steps_per_cell` =
`steps / (width*height)` (the builder criterion uses it).

`conf_*`: independent re-verification of the detected highways
(`confirmed_v2.csv`, `src/bin/confirm.rs`); `*16` columns use the longer
confirmation window. Only highway rows are present.

`run1e9_*`: the 10^9-step re-run of the rules still unresolved at 10^8 (857
rows have it; same column meaning as `sweep_*`). `highway_1e9` is true for the
12 rules that file lists as highways at 10^9. **These 12 keep their
original census `phase`** (the paper's census is defined by the 10^8 detector);
filter them out if you want a stricter blob set.

`reached_2e27`, `census_alpha_w`, `census_ext_per_doubling`, `census_ext_2e27`:
growth-census fit (`census.csv`, from `growth.csv`): did the run complete 2^27
steps; log2-log2 slope of the extent over t = 2^21..2^27 (the class cut uses
this); mean extent gain per doubling; extent at the last point.

`probe_*`: probe statistics from `probe_summary.csv` (3055 rows), fits over
octave windows k = 20..26 (`experiments/_lib/analyse_probe.py`):
- `alpha_w`, `alpha_rg`: width and radius-of-gyration exponents (the paper's
  headline blob median `alpha_w` is 0.3326, reproduced from this column).
- `a`: area exponent; `beta`: boundary-vs-area exponent (P ~ A^beta).
- `c`: fresh-cell entries per boundary step (last window); `kappa`: relative
  boundary occupation (boundary-step fraction / (P/A)), last window.
- `g_c`, `g_k`: log-log drift slopes of `c` and `kappa` against t.
- `fill`: area / bounding box (last window); `g_fill` its drift; `spread`: ant
  r^2 / visited-set r_g^2.
- `a_pred = (1 + g_c + g_k) / (2 - beta)`: the growth-rate identity's
  prediction of `a` (bookkeeping, not a cause).
- `max_jump`, `max_share`: largest single-window area jump and its share of
  growth; `bursty` flags rules with a large `max_jump` (kept, not removed).

`cmodel_c_pred`, `cmodel_held_out`: out-of-sample prediction of `c` from rule
letters (`c_model.csv`) and whether the rule was held out of that fit.

Empty cells mean "not applicable / not measured" (e.g. no probe statistics for
highway rules, builders or the unfitted growers), not zero.

## Caveats

- Classification is by finite simulation (10^8 steps for highways, 2^27 for
  growth); a blob may still build a highway later (see the `highway_1e9` rows
  and the paper's discussion of late highways).
- Probe statistics are finite-window estimates from one trajectory; they carry
  fit and window bias (see the paper's calibration section). Between-seed
  uncertainty does not exist here (blank start) but the window choice matters.
- Floats are written to 10 decimal places.
