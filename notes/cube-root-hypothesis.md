# Why do chaotic ants grow as t^(1/3)?

Written 2026-10-10 07:45 NZDT, before any measurement beyond the census
(median width exponent 0.332 across 2,637 chaotic rules, stable from 2^16 to
2^27 steps and across rule lengths 3 to 12).

## Mechanism (the "well-mixed walker in a reflecting cup")

1. Blank cells repel. Every canonical rule turns R on colour 0, so an ant that
   steps onto fresh ground curls back on itself within a few steps. The
   unvisited plane acts as a wall the ant can only chip at.
2. Inside the visited region the ant's motion is irregular enough that, over
   times short compared with t, it is spread roughly evenly over the A visited
   cells.
3. So the fraction of time it spends on a boundary cell (a visited cell with an
   unvisited neighbour) is about P/A, where P is the number of boundary cells.
4. Each step taken from a boundary cell enters fresh ground with some roughly
   constant probability c.
5. The region is compact with a reasonably smooth edge, so P ~ A^(1/2).

Then dA/dt = c * kappa * P/A ~ A^(-1/2), so A ~ t^(2/3) and width ~ t^(1/3).

Compare: if the edge were fractal (P ~ A), growth would be t^(1/2). If the ant
had to return to the origin between growth events, as in internal DLA, it
would be t^(1/4).

## Predictions

- P1. Area A ~ t^(2/3), and the region stays compact: A / width^2 settles to
  a constant.
- P2. c = (new cells) / (steps starting on a boundary cell) is roughly constant
  in time for each rule.
- P3. kappa = (fraction of steps on the boundary) / (P/A) is roughly constant in
  time, and of order 1.
- P4. P ~ A^beta with beta close to 1/2.
- P5. More generally, with time exponents gamma_c and gamma_kappa for c and
  kappa, A ~ t^a with a = (1 + gamma_c + gamma_kappa) / (2 - beta). This should
  predict the measured exponent rule by rule, including the outliers
  (alpha from 0.19 to 0.6) whose growth is not a cube root. If the outliers'
  exponents are NOT explained by beta and gamma, the mechanism is wrong or
  incomplete.

## What would kill it

- kappa drifting strongly with t (the walker is not well mixed; e.g. it stays
  near the edge, or deep inside).
- c drifting with t while beta = 1/2 (a cube root by coincidence).
- The formula in P5 failing to predict alpha for the outliers.

## Interpretation correction — 2026-10-16 00:18 NZDT

Base commit `997c9b2da6efa526fc6c5a57104d103e1345e916`, dirty workspace.
The original predictions above are retained as registered. P5 is conditional
scaling algebra applied to quantities sharing the same growth data. Agreement
is a fit-consistency check, not independent validation of a mechanism; failure
could reflect finite-window fits or failure of the assumed power laws. It
should not have been offered as a stand-alone mechanism falsification test.
P2–P4 describe the proposed stationary regime but do not explain why it occurs.
Interior colour uniformity does not imply uniform spatial occupation or stable
boundary access. The revised paper states that the physical closure remains
missing; calibration concerns estimator bias. See the
[response audit](accounting-identity-response-2026-10-16.md) for evidence limits
and what independent prediction and intervention would need to establish.
