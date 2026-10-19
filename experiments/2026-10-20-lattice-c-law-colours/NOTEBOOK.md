# Lab notebook: lattice c law, colours and circulations

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-20 00:55 (+13:00) · 6049d6a (+uncommitted instrument) · intent
Take the top open item of `notes/ACTIVE.md` (why b falls with loop length) and measure the two factors of the parent's
identity b = l_b / N_b rather than fit b(l). Built `src/lattice_trace.rs` (edge_trace's per-colour, lag, N_b tallies on three
lattices, plus runs of consecutive lag-`ell` returns and hole births). Gates T1 passed (square equals `edge_trace`, all lattices
equal a HashMap re-implementation, two mutations caught; the mutation runs left a `proptest-regressions` file that I deleted
because its cases fail only under the mutants). Predictions P1-P6 are in the README, written before running the 960 ants.

### 2026-10-20 00:56 (+13:00) · 6049d6a · assumption
A circulation of the lead letter shows up as a run of consecutive reads with lag exactly `ell`. This can be false where the
ant crosses its own ring or a second loop shares cells; it is checked only through P1 (does Cb carry the slope). Untested.

### 2026-10-20 00:56 (+13:00) · 6049d6a · assumption
Alternative I cannot exclude: the b(l) drop is survivor selection (the kept set differs by lattice; square lead 2 and hex R lead 2 lose >50%).
The tracer is run on all 960 rules and quantities are reported for kept rules only, with retention printed; a check on all non-escaped rules is a planned robustness line.
