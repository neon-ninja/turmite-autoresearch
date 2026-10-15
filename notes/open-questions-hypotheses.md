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

## Added 2026-10-13 (later) from `notes/frontier-2026-10-13-results.md`

Answered or sharpened there: Q3 (no single rule feature labels the home types: A5), Q4 (rare colours are rare in 99.8% of tethered rules and 5% of blob rules, but the self-healing explanation is reversed: the most-visited cells hurt most: A6), Q5 (RRLLLLLLR: CV rises through 2^29, unimodal; rising CV occurs in ~5% of random rules, mostly RLLL-prefixed: A7), Q6 (cone follows from uniform occupancy + the exponent, and correctly fails on a random background: A3), Q7 (ants 0.24 vs rotor 0.13 still falling, ants trend inconclusive: A4).
New, unexplained:
- Q8. RLRLLL (an item-4 "core" rule) leaves a 32768 board at ~1.3e10 steps with no highway: an ~1e4-cell thin structure along the edge (bounding box 3,377 at 1e10, 18,175 at 1.3e10). What is it, and are the other "escaped without highway" cases (4.6% of the 10^10 survivors, 1.2% in (1e10, 1e11]) the same thing?
- Q9. The tether of RRRRRLLLLLL is intermittent in age: a fresh ant started from the 2^25 field at home heading north is not tethered (near_home 0.061) while 2^22, 2^24, 2^26 are (0.43, 0.47, 0.36). What state does the original ant carry that the field plus "home, north" does not?
- Q10. In the rotor with north-biased arrows (C2b/C2c) the territory width at 2^28 grows +25% at bias 0.15 and 2.6x at 0.25, while kappa and c change by < 10% and the width exponent not at all. Which factor of the identity dA/dt = c kappa P/A carries the amplitude, and why does the area exponent rise (0.715 to 0.787) while the width exponent does not (elongation untested)?
- Q11. Why is the area-exponent ICC across rules (0.34) so much larger than the width-exponent ICC (0.12) (B2)? Rule identity matters for area but little for width.
- Q12. The 3D ants' extent exponent is 0.234 and volume exponent 0.771 at 2^28-2^32, so volume/extent^3 is still rising as ~t^0.07: what is the 3D limit shape (D2)?
- Q13. Why does the corruption of the most-visited cells hurt most, with rare colours doubling the damage at equal visit quartile (A6)? How much of "visit rate" is just distance from home?
- Q14. Does contiguity reduce the damage per critical cell (patch ~0.65x the loss per rare cell of a swap, E4), or is that a location effect?
- Q15. The census "builders" (304) are mostly non-highway frame builders with steps per bounding-box cell ~2-6; are they one class, and does it match the sqrt(t) turmite cluster (D1)?
- Q16. The late-highway onset rate fell from 4.7% to 1.25% per decade at 1e10-1e11 (B5): is the set of late highways finite, and what is the distribution of onset times within a prefix family?

See `notes/research-frontier-2026-10-13-pm.md` for the Phase 3 hypotheses, tests, and pre-registered predictions addressing Q8–Q16 and the open mechanisms of kappa.

### Kappa prefix robustness follow-up (2026-10-15 22:40 +13:00)
The [prefix audit](../experiments/2026-10-kappa-rule-space/README.md#prefix-audit-addendum) retains the W3c stationary lead: five-fold lookup encoding plus a five-knot nonlinear ln A(22) control gives +0.201 held-out R2 gain [0.103,0.311] on 162 rules, seed 20261015 and 2000 rule bootstraps. Removing five largest positive contributors leaves +0.150. All-rules gain +0.041 [-0.018,0.103] remains weak. This is an audit of inspected data, not fresh confirmation or a mechanism; one area snapshot may leave growth-history confounding. Reproduce with uv run experiments/2026-10-kappa-rule-space/scripts/prefix_audit.py; input/source hashes in results/prefix_audit.json. Next discrimination: matched-area mutation intervention with early-colour exposure measurements.
