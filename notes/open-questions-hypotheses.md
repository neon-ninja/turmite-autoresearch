# Open questions: hypotheses before measuring

Written 2026-10-10 10:30 NZDT. Data in hand: probe_summary.csv (c, kappa, ...
per rule). No new measurements yet.

## Q1. Why is the ant well mixed inside its territory?

H1. Inside, the ant is a random walker in a random environment. Each interior
cell has been visited many times, so its colour (visits mod n) is close to
uniformly distributed and nearly independent of its neighbours'. The ant then
turns R with probability q = (number of R in the rule) / n on each step,
independently of earlier turns. That is a persistent random walk, and a
reflecting wall plus a symmetric walk gives a uniform stationary density.

Predictions:
- P1.1 Interior colours are close to uniform over 0..n-1, with weak
  neighbour correlation.
- P1.2 The fraction of R turns the ant actually makes is close to #R/n.
- P1.3 Turns are nearly uncorrelated, and the mean squared displacement grows
  linearly with lag, with slope (1 - a) / (1 + a), where a = (2q - 1)^2.
  (Derivation: each turn multiplies the velocity by +i or -i; the average
  factor is i(2q - 1), so velocities k steps apart correlate as
  Re[(i(2q-1))^k], which vanishes for odd k and is (-a)^(k/2) for even k.
  Summing gives MSD / lag = 1 + 2 * sum_m (-a)^m = (1 - a) / (1 + a).)
- P1.4 Mixing time across the blob ~ R^2 / D, which is much less than the
  time to grow, ~ R^3. So the walker has time to spread out.

H1' (for tethered, log-growing rules). Something breaks the random environment
near home, for example a colour field that is not uniform, giving an inward
drift. If the drift v is constant and diffusion is D, occupancy falls off as
exp(-r v / D), which matches the exponential profile already seen.
- P1.5 Tethered rules have a measurable inward radial drift, and D / v matches
  the decay length of their occupancy profile.

## Q2. Why is c about 0.44, and why does it vary?

H2. Edge cells are young: most have been visited once or twice, so their
colour is 1 or 2. On an edge cell the ant's turn is therefore set mostly by
rule[1] (and rule[2]), not by a random colour. If the low colours turn the same
way as colour 0 (R), the ant tends to curl back inside, so c is low. If they
turn the opposite way (L), the ant zig-zags outward, so c is high.

Predictions:
- P2.1 Among all rules, c depends strongly on rule[1] and rule[2], much more
  than on later letters.
- P2.2 Edge cells' colours are concentrated on 1 and 2.
- P2.3 A local model (an ant on a straight edge, cells beyond it blank, edge
  cells with the measured colour mix, interior colours uniform) predicts c per
  rule better than the random-heading estimate (fresh neighbours / 4).

## Added 2026-10-13 from the frontier work (see notes/frontier-results.md)

- Q3. Why do 556 tethered rules share only ~28 discrete home types (spread values equal to 0.3% across different rules and territories)? What invariant of the rule labels them?
- Q4. Why are the rare colours (6, 8, 10 / 0, 6, 8) the critical cells of the hard rules' homes, and are they rare in the other 112 rules of the same class?
- Q5. Is RRLLLLLLR (CV of A/t^(2/3) rising with t, alpha_w 0.38 +- 0.07) truly non-self-averaging, and with how many 'phases'?
- Q6. Why does the ant's visit density equal the rotor's cone 3 t/A (1 - r/R_eq) to a few per cent? A derivation would also fix the 2/3 exponent.
- Q7. Why is the ants' boundary width exponent (0.25-0.27 at 2^28) above the rotor's (0.13-0.16 under the same estimator)?
