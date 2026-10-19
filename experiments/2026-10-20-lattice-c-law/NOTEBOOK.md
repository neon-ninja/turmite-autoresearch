# Lab notebook: lattice c law (hex and triangular ants)

Append-only. Newest entry last. Never edit an old entry; correct it with a new one
(`corrects: <timestamp>`). Tags: intent, assumption, observation, interpretation,
surprise, decision, dead-end, question, deviation. See `experiments/README.md`.

### 2026-10-20 00:25 (+13:00) · 0de4d38 · intent
Start `lattice-c-law`: test whether the square-lattice c law (1/c = 1.36 + 0.70 lead) and the kappa deficit hold on hex and triangular
ants. Registered the three loop-length hypotheses (A: b ~ loop length, B: lattice-independent, C: ~ 4/z) with numeric signature vectors
before any lattice c exists. Decision it informs: whether the 2x2-loop picture of c is specific to square geometry.

### 2026-10-20 00:25 (+13:00) · 0de4d38 · assumption
kappa_unif(a, beta) ports to hex and triangular because it is analytic in the run's own (a, beta); its premise (a uniform walker has P/A
tracking the window) is untested there. Falsified by gate G4 (mean-field walker ratio within 0.05 of 1). Status: untested.

### 2026-10-20 00:25 (+13:00) · 0de4d38 · question
My derivation says loop length cancels per loop cell (b = l_b / N_b) and the data must decide how N_b scales with the loop; I do not know
which limit is right, so the H-A vs H-B odds (25/40) are a guess. The census `cls == blob` is not reproducible from committed code, so I
use an alpha_w proxy (deviation from the census definition, stated in the README).
