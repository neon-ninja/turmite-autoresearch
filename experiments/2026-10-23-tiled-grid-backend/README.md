# Tiled CPU grid backend: memory and throughput of sparse and packed tiles

Bounded engineering experiment (no new science, no classification changes). Architecture, state
transitions, memory accounting, checkpoint format and limits: [DESIGN.md](DESIGN.md).

## Question and registered prediction

Registered 2026-10-23 18:58 +13:00, before any timing run. Starting revision `2ee0086e075eb2b1fca04781870dff37aaaea48b`;
implementation commit `c0b1d5d` (tests pass, CSV byte-identity checked, **no timing or memory measurement yet**). The working tree
is dirty with other contributors' files only (`experiments/2026-10-23-sweeper-generalisation/`, `highway-decider`); benchmark
binaries are built from a `git archive` of the benchmark commit, not from the working tree.

**Question.** `sweep_lean` allocates `(size+2)^2` bytes per worker and writes all of them at the start of every rule
(`LeanGrid::clear`), so resident memory is `threads x size^2` whatever the ant does. Does a tiled grid (sparse 64x64 tiles; a
bounded cache of uncompressed tiles; bit-packed inactive tiles) give a useful reduction in peak resident memory at an
acceptable throughput cost, on workloads taken from the existing census?

**Backends compared** (same rule, board, budget, `RAYON_NUM_THREADS=1`; the CSV must be byte-identical, which is a hard gate):

| id | backend | settings |
|---|---|---|
| A | dense `LeanGrid` | |
| B | tiled, sparse allocation only | `TURMITE_GRID=tiled TURMITE_CACHE_TILES=unlimited` (tile 2^6; also 2^5, 2^7 on W1, W3) |
| R | tiled, bounded cache, raw inactive | cache 64, `TURMITE_INACTIVE=raw` (isolates caching/copy cost from packing) |
| C | tiled, bounded cache, packed inactive | cache 64, 1024 (and 16 as a thrash probe), `TURMITE_INACTIVE=packed` |
| D | block compression | **only if** criterion A5 below fires; otherwise not implemented |

**Workloads** (all from `experiments/2026-10-13-census-sweeps/results/sweep_2_12_100M.csv`, board 8192, 1e8 steps unless noted;
the labels are finite-time descriptions of that run, nothing more):

| id | rule | role | evidence in the census run |
|---|---|---|---|
| W1 | `RLR` | compact irregular trajectory | budget, extent 998 x 1035; census class `blob` |
| W2 | `RRRRRRRLLRRR` | repeated local revisits | budget, extent 17 x 18 |
| W3 | `RLLRLLLRRRLL` | growing structured trajectory | budget, extent 7067 x 7068; census class `builder` (alpha 0.50) |
| W4a | `RL` | thin travelling trajectory | escapes the 8192 board at 222061 steps (highway, period 104) |
| W4b | `RL` | same, board 65536, 2e7 steps (does not escape) | memory is proportional to the path, not the board |
| W5 | `RRRRRRRLLRRR` | tile-boundary stress | W2 with tile 2^2 or 2^3 and cache 2: start cell is a tile corner, footprint spans many tiles |

**Predictions** (peak RSS = `VmHWM` of the process; ratios are medians of 5 interleaved repetitions; throughput is steps/s from
the wall time of the run, which includes grid construction):

- P1. Dense RSS is 68-80 MiB on every 8192 workload (the 64 MiB grid is fully written) and about 4.1-4.3 GiB on the 65536 board.
- P2. Empty-process baseline RSS is 2-8 MiB.
- P3. B: RSS <= 12 MiB on W1, <= 8 MiB on W2, <= 10 MiB on W4a, <= 20 MiB on W4b; on W3 55-70 MiB, i.e. 0.75-0.95 of dense
  (no real saving where the footprint nearly fills the board).
- P4. C (cache 64) on W3: payload 2560 B per tile (n = 12 gives w = 4: 2048 + 512 bytes) so RSS 38-46 MiB, 0.55-0.75 of B.
- P5. Throughput relative to dense (A = 1): B 0.6-0.95 on W1, 0.25-0.7 on W2 (the start is a tile corner, so a compact pattern
  crosses tile edges constantly), 0.6-0.95 on W3, 0.6-1.0 on W4a. C with cache 1024 on W1 (about 290 tiles touched, so nothing is
  ever evicted) is >= 0.95 of B. C with cache 64 on W3 is < 0.3 of B (miss rate x about 30 microseconds per miss dominates).
  C with cache 16 is a thrash and slower still.
- P6. Codec cost: encode and decode of one 64 x 64 tile at w = 4 are each 8-25 microseconds (measured separately, not in the
  main timing).
- P7. W5 with cache 2 shows transitions/step >= 0.1 and a throughput < 0.15 of dense.

**Adoption criteria** (registered; chosen for these workloads: the point of the backend is a smaller process, so memory is
required to move by a factor, while throughput only has to stay practical):

- A1. Exactness (gate): byte-identical CSV for every backend and setting on every workload, and `cargo test --release --locked`
  green. Failure rejects the backend regardless of anything else.
- A2. Sparse tiles (B) are *adopted as the memory-saving option* if on W1, W2, W4a, W4b peak RSS <= 0.25 x dense AND median
  throughput >= 0.5 x dense on W1 and W4a. If only the memory half holds, B is kept as an opt-in for memory-bound batches and the
  throughput cost is reported as its price.
- A3. Packed inactive storage (C) is *worth keeping over B* only if, at some cache bound, on W3 peak RSS(C) <= 0.75 x peak
  RSS(B) AND throughput(C) >= 0.3 x throughput(B). Otherwise packing is retained as code and tests but documented as not
  recommended, and the negative result is reported.
- A4. Cache sensitivity is reported, not gated; the recommendation names a minimum cache for each workload class.
- A5. Block compression (D) is attempted only if both hold: the packed inactive payload is >= 40% of C's peak RSS on some
  workload, and an offline per-tile compression of the real canonical packed payloads (deflate and xz as proxies that bound what
  LZ4-class codecs can achieve, each tile compressed independently with a 4-byte header and a raw fallback) shrinks the total
  payload by >= 1.3x. Otherwise D is not built.
- Failure of the main prediction: if B's RSS on W1/W2/W4 is > 0.5 x dense, or any exactness gate fails, the design is rejected as
  a memory tool.

Competing explanation for any slowdown: tile-transition overhead (hash lookup, `settle`) versus codec cost versus cache misses.
`R` (raw inactive, same cache) and the unlimited-cache `B` separate them.

## Pruning-index preflight

- Index revision and search commands: `git rev-parse HEAD` = `2ee0086e075eb2b1fca04781870dff37aaaea48b`;
  `git ls-files notes/pruning-index/entries`; `git grep -n -i -l -e tiled -e storage -e memory -e backend -e grid -- notes/pruning-index/entries`.
- Candidate signature IDs: none matched. The sole entry, `NR-20261022-additive-laws-order-blind`, concerns additive conservation
  laws of rules of length 2..8; its membership predicate, objective and observable (law group of a rule) are unrelated to
  storage-backend performance.
- Scope comparison: this experiment varies the data structure under a fixed deterministic ant; it makes no claim about any rule
  family's dynamics and asks no scientific question the index could prune. The census rules are workloads, not subjects.
- Decision: run. No dead-end signature is warranted by a performance result.
- Run spec and exclusions: [scripts/workloads.tsv](scripts/workloads.tsv); nothing excluded.
- Compute/stopping budget: about 600 single-thread runs of at most a few seconds each (W4b at most about 1 minute for the dense
  baseline, which writes 4 GiB), plus the offline compression estimate. No further run is justified by this experiment's result
  unless a criterion above says so.

## Reproduction

See [scripts/](scripts/): `build_bench.sh` (fresh `CARGO_TARGET_DIR`, build from `git archive` of the stated commit, binary
sha256), `bench.sh` (interleaved repetitions, writes `results/*.json` and a CSV-identity check), `summarise.py` (uv, stdlib only),
`compress_estimate.py` (criterion A5). The exact commands, toolchain, CPU, load, binary hashes and benchmark commit are recorded
in [results/provenance.txt](results/provenance.txt) (written by `bench.sh`) and in `NOTEBOOK.md`.

## Results, uncertainty and limitations

Filled in after the runs (below the line; the registration above is not edited).

---
