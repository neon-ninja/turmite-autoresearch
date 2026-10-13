# Research frontier (2026-10-13 PM)

Open questions, mechanistic hypotheses and experimental designs for the next phase of the investigation.
This note builds directly on:
- `notes/research-frontier.md` (items 1–32),
- `notes/research-frontier-2026-10-13.md` (items A1–A7, B1–B5, C1–C3, D1–D5, E1–E6),
- `notes/frontier-2026-10-13-results.md` (results ledger documenting the resolution of all Phase 2 items),
- `notes/kappa-experiment-design.md` (Arms A, B, C testing and ruling out five mechanisms for $\kappa$),
- `notes/open-questions-hypotheses.md` (questions Q8–Q16),
- and `paper/main.typ` / `notes/summary-2026-10-13.md`.

**Pre-registration declaration:**
Nothing here has been measured. Every prediction below is an explicit proposal written on 2026-10-13 PM *before* any test; per `AGENTS.md`, do not edit a prediction after seeing data, append what happened and any revision underneath. Predictions are quantitative guesses designed to discriminate between competing physical mechanisms, not summaries of results.

---

## F. What sets kappa? Global, topological and excursion mechanisms

### Context
In the growth identity $dA/dt = c \cdot \kappa \cdot P/A$, $\kappa = (B/S) / (P/A)$ measures the over-representation of the walker on the perimeter relative to a uniform spread.
In Phase 2, five candidate explanations for $\kappa$ were tested and decisively falsified:
1. Rule letters, $q = \#R/n$, prefixes, and substrings: no prediction (held-out $R^2 \le 0.04$, A1c).
2. Memoryless persistent walker: gives $\kappa \approx 1.35$ with an incorrect area exponent 0.95 (A1d).
3. Local boundary dwell time $v_e$: edge dwell is pinned at $v_e = 1.5$ for 10 of 12 extreme ants regardless of whether $\kappa = 0.12$ or $\kappa = 15$ (Spearman $\rho = -0.26$, A3).
4. Depleted crust depth $D$ (layers $d = 2..4$): failed to separate matched pairs of rules with equal length, equal $q$, and equal $c$ (Arm C sign test 13/26, $p = 0.58$).
5. Noisy rotor: adding memory noise does not smoothly interpolate $\kappa$ down to 1.4; instead, $\kappa$ spikes up to $6.65$ during the crossover to diffusion (Arm B).

Yet $\kappa$ has high between-rule repeatability ($\text{ICC} = 0.58$ across patched starts; blank-board runs agree on pair ordering in 25 of 26 pairs). If $\kappa$ is not set by the first cell layer (dwell) nor the near-surface crust, it must be determined by the **global interior excursion dynamics** connecting boundary visits.

---

### F1. Interior excursion length distribution $P(L)$ and the mean return time
By renewal / ergodic theory on a finite territory, the boundary occupancy fraction $B/S$ is given by:
$$
\frac{B}{S} = \frac{\langle \tau_{\text{edge}} \rangle}{\langle \tau_{\text{edge}} \rangle + \langle L_{\text{int}} \rangle}
$$
where $\langle \tau_{\text{edge}} \rangle$ is the mean dwell time during a boundary contact ($v_e \approx 1.5$), and $\langle L_{\text{int}} \rangle$ is the mean number of steps taken into the interior (layer $d \ge 1$) before returning to layer $d = 0$.
Since $\kappa = (B/S) \cdot (A/P)$ and $A/P \sim R \sim t^{1/3}$, $\kappa$ is governed by whether interior excursions are shallow loops that return rapidly to the perimeter or deep diffusive journeys that explore the core.

- **Hypothesis:** Rule-to-rule variation in $\kappa$ is inversely proportional to the median interior excursion length: $\kappa \propto 1 / L_{\text{med}}$. Low-$\kappa$ rules plunge deeply into the bulk (large $\langle L_{\text{int}} \rangle$), whereas high-$\kappa$ rules perform shallow, localized loops near the perimeter.
- **Test:** Instrument `src/probe.rs` to track excursion lengths $L$ (steps between leaving layer 0 and next landing on layer 0) across the 44 rules of the matched-pairs pool (`data/armc_specs.txt`) in octave windows $k = 22..25$. Measure the median excursion length $L_{\text{med}}$ and fit the power-law tail exponent $\gamma$ of $P(L) \sim L^{-\gamma}$ for $L \in [10, 10^4]$.
- **Prediction:**
  1. Median interior excursion length $L_{\text{med}}$ strongly anti-correlates with $\kappa$: Spearman $\rho(\kappa, L_{\text{med}}) \le -0.70$ ($p < 10^{-4}$).
  2. The distribution $P(L)$ has a heavy tail with $\gamma \in [1.2, 1.8]$. The tail exponent $\gamma$ correlates positively with $\kappa$ ($\rho \ge 0.60$): high-$\kappa$ rules have steeper cutoffs (fewer deep excursions).
- **What breaks it:** If $L_{\text{med}}$ is uncorrelated with $\kappa$ ($|\rho| < 0.25$), bulk excursion length is not the carrier of $\kappa$, implying that the difference arises from anisotropic boundary geometry or multi-scale trapping.

---

### F2. Tangential edge currents and boundary velocity polarization
Does the walker reflect isotropically from the boundary, or does it set up a persistent directed current along the perimeter?

- **Hypothesis:** High-$\kappa$ rules establish a boundary-gliding state: on layer 0, the ant's velocity vector $\vec{v}$ is strongly polarized along the local perimeter tangent $\vec{t}$ ($\langle |\vec{v} \cdot \vec{t}| \rangle \gg \langle |\vec{v} \cdot \vec{n}| \rangle$). Low-$\kappa$ rules experience immediate normal reflection into the interior ($\langle \vec{v} \cdot \vec{n}_{\text{in}} \rangle \approx 1$).
- **Test:** On each step visiting layer 0 in octave windows $k = 22..25$, compute the local boundary normal $\vec{n}$ (pointing into the interior) and tangent $\vec{t}$. Compute the polarization ratio:
  $$
  \mathcal{R}_{\text{edge}} = \frac{\langle |\vec{v} \cdot \vec{t}| \rangle}{\langle \max(0, \vec{v} \cdot \vec{n}_{\text{in}}) \rangle}
  $$
  across the 24 rules of Arm A (12 extreme-kappa and 12 typical-kappa rules).
- **Prediction:** $\mathcal{R}_{\text{edge}}$ correlates strongly with $\kappa$: Spearman $\rho(\mathcal{R}_{\text{edge}}, \kappa) \ge 0.65$. Rules with $\kappa > 2.0$ have $\mathcal{R}_{\text{edge}} > 1.4$, while rules with $\kappa < 0.5$ have $\mathcal{R}_{\text{edge}} < 0.8$.
- **What breaks it:** If $\mathcal{R}_{\text{edge}}$ is invariant across rules ($1.0 \pm 0.1$), boundary motion is locally isotropic and does not carry the $\kappa$ variation.

---

### F3. Critical singularity of boundary occupancy in the noisy rotor
Arm B showed that adding memory noise $\epsilon$ to the rotor caused $\kappa$ to spike up to $6.65$ at $\epsilon = 0.01$ during crossover, rather than decreasing from 2.0 to 1.4.

- **Hypothesis:** The spike in $\kappa$ is a dynamic crossover singularity. In the rotor, arrow updates enforce exact Eulerian flux balance. Weak noise $\epsilon$ destroys Eulerian cycle closure in the bulk first, destroying interior recurrence, while the boundary reflection remains trapped by unvisited outer cells. The peak boundary enhancement $\kappa_{\max}(\epsilon)$ diverges as $\epsilon \to 0$ with an anomalous scaling:
  $$
  \kappa_{\max}(\epsilon) \sim \epsilon^{-\mu}, \quad t^*(\epsilon) \sim \epsilon^{-\nu}
  $$
- **Test:** Run the noisy rotor (`NoisyRotor` in `src/walkers.rs`) at finer geometric noise levels $\epsilon \in \{10^{-4}, 3\times 10^{-4}, 10^{-3}, 3\times 10^{-3}, 10^{-2}\}$ to $2^{29}$ steps on a $32768^2$ board, with 8 seeds each. Locate the peak $\kappa$ and its step $t^*$.
- **Prediction:**
  1. $t^*(\epsilon) \sim \epsilon^{-1.0 \pm 0.15}$.
  2. $\kappa_{\max}(\epsilon) \sim \epsilon^{-0.28 \pm 0.06}$.
  3. For $\epsilon = 10^{-4}$, the $t^{2/3}$ area scaling survives up to $2^{26}$ with $\kappa = 2.05 \pm 0.10$.
- **What breaks it:** If $\kappa_{\max}$ plateaus for $\epsilon \le 10^{-3}$ rather than growing, the peak is a finite-board artifact rather than a true scaling singularity.

---

### F4. Macroscopic boundary impedance and Robin boundary condition
In continuum diffusion problems, a reflecting boundary with partial penetration is modeled by a Robin boundary condition: $-D \nabla \rho \cdot \vec{n} = \lambda \rho$, where $b = D/\lambda$ is the extrapolation length.

- **Hypothesis:** The ant's interior density satisfies a quasi-stationary Helmholtz/diffusion equation in the comoving frame with an effective boundary impedance $\lambda_r$. $\kappa$ is directly determined by the macroscopic impedance $\lambda_r$: $\kappa = \kappa_0 / (1 + b_r / R)$.
- **Test:** For the 24 rules of Arm A, measure the radial density gradient $\partial \rho / \partial r$ at $r = 0.8 R_{\text{eq}} .. 0.95 R_{\text{eq}}$. Compute the effective impedance $\lambda_{\text{eff}} = (D \partial \rho / \partial r) / \rho_{\text{edge}}$.
- **Prediction:** $\lambda_{\text{eff}}$ explains $> 60\%$ of the variance of $\kappa$ across the 24 rules ($R^2 \ge 0.60$).

---

## G. Breakout instabilities, ribbon dynamics, and late escapes

### Context
In Phase 2, three major anomalies in asymptotic confinement were discovered:
1. `RLRLLL` (one of the 14 core rules tested to $2^{34}$) escaped a $32768^2$ board at $\approx 1.3 \times 10^{10}$ steps with **no highway** (Q8). It built an anomalous $\sim 10^4$-cell thin ribbon structure along the boundary (bounding box jumped from 3,377 at $10^{10}$ to 18,175 at $1.3 \times 10^{10}$).
2. In `RRLLLLLLR` (A7c), 1 of 62 runs at $2^{32}$ suffered a massive 14x area blowout. About 5% of random rules exhibit rising seed-to-seed CV, heavily concentrated in `RLLL...` prefixes.
3. In the 1e10–1e11 decade, the highway onset rate fell from 4.7% to 1.25% (3/240), but 3 additional rules escaped the board as unclassified non-highway frame/ribbon builders (B5).

---

### G1. Anatomy of the `RLRLLL` non-highway boundary breakout
What is the ant doing when a blob rule suddenly leaves the grid without forming a highway?

- **Hypothesis:** The breakout is an **edge unravelling mode** (a "boundary highway" or surface wave). The ant does not build a periodic highway into open space; instead, it encounters a specific boundary defect that locks it into a 1D perimeter-following crawl. The crawl advances along the envelope of the existing blob, carving a thin ribbon until it reaches open boundary corners and shoots outward.
- **Test:** Run `examples/escape_trace.rs` on `RLRLLL` across the escape window $t \in [1.25 \times 10^{10}, 1.31 \times 10^{10}]$. Measure:
  1. Manhattan ribbon thickness $\Delta_\perp$ (transverse width).
  2. Local step efficiency $\eta_{\text{step}} = \Delta t / \Delta A$.
  3. Periodicity of displacement modulo perimeter length.
- **Prediction:**
  1. The transverse width remains tightly bounded: $\Delta_\perp \le 12$ cells across $> 95\%$ of the escape trajectory.
  2. The step efficiency is quasi-ballistic: $\eta_{\text{step}} < 8$ steps per new cell (contrasting with $> 100$ steps/cell in the blob phase).
  3. The motion has a repeating quasi-period $p \in [24, 256]$ that shifts adiabatically as the local boundary radius of curvature changes.
- **What breaks it:** If the ant's visited set expands isotropically in 2D with $\Delta_\perp > 100$ cells, the escape is an accelerated diffusive blob, not a boundary wave-guide.

---

### G2. Asymptotic survival analysis of blob rules: hazard rate decay
Is every blob rule strictly metastable (destined to escape eventually), or is there a non-zero measure of truly confined chaotic rules?

- **Hypothesis:** The hazard rate of escaping into any ballistic or ribbon state decays as a power law:
  $$
  h(t) = -\frac{1}{S(t)} \frac{dS}{dt} \sim t^{-\theta}, \quad \theta \approx 0.65 \pm 0.15
  $$
  Because $\theta < 1$, the cumulative hazard $\int h(t) dt$ diverges as $t^{1-\theta}$, implying that asymptotically as $t \to \infty$, almost all rules eventually break out, but the median lifetime is $> 10^{16}$ steps.
- **Test:** Merge the full 732 survivor census at $10^{11}$ steps (`data/h1e11/run3.log`). Fit the survivor function $S(t)$ over decades $10^8, 10^9, 10^{10}, 10^{11}$.
- **Prediction:**
  1. The total number of new escapes in $(10^{10}, 10^{11}]$ across all 732 rules will be $\le 10$ (escape rate $\le 1.4\%$).
  2. The fitted hazard exponent $\theta \in [0.5, 0.8]$ (falsifying both a flat rate $\theta = 0$ and an exponential cutoff $\theta > 1$).
- **What breaks it:** If new escapes in $(10^{10}, 10^{11}]$ exceed 25 rules (hazard rate $\ge 3.4\%$), the thinning was a small-sample fluctuation and the rate is approximately constant.

---

### G3. Heavy-tailed area distributions in the `RLLL...` family
Why does `RRLLLLLLR` exhibit rare 14x blowouts (A7c), and why do 5% of rules show rising seed-to-seed CV?

- **Hypothesis:** Rules prefixed with `RLLL` possess an absorbing boundary defect that can trigger temporary unravelling. The territory area distribution $P(A)$ at fixed $t$ has a heavy Fréchet / Pareto tail:
  $$
  P(A > x) \sim x^{-\alpha_{\text{tail}}}
  $$
  with $\alpha_{\text{tail}} \in (1, 2)$, producing an infinite variance (hence rising empirical CV) even while the bulk median converges.
- **Test:** Run 100 patched starts to $2^{30}$ steps for 4 `RLLL`-prefixed rules (`RRLLLLLLR`, `RLLLRRRRLR`, `RLLLLRRRRL`, `RLLLRLLLRR`) and 4 control core rules (`RLR`, `RLLR`, `RLRL`, `RRLR`). Compute the Hill estimator of the tail exponent $\alpha_{\text{tail}}$ for $A / \text{median}(A)$.
- **Prediction:**
  1. For `RLLL` rules, $\alpha_{\text{tail}} = 1.6 \pm 0.3$ ($\alpha_{\text{tail}} < 2$ explains the rising CV).
  2. For control core rules, $\alpha_{\text{tail}} > 4.5$ (thin sub-Gaussian tails, falling CV).
- **What breaks it:** If `RLLL` rules have $\alpha_{\text{tail}} > 3.0$, the 14x outlier was a unique fluke rather than a heavy-tailed phase.

---

## H. The area vs width discrepancy and territory geometry

### Context
Phase 2 established a major geometric puzzle (Q10, Q11, B2):
- The ICC of the **width exponent** across rules is only $0.12$ (rule identity explains almost nothing; run noise dominates).
- The ICC of the **area exponent** across rules is $0.34 - 0.43$ (rule identity explains 3x–4x more variance!).
- Furthermore, under initial arrow bias (C2b/c), width at $2^{28}$ expanded by 2.6x at bias 0.25 while the width exponent stayed $0.33$, but the area exponent increased to $0.79$.

How can rule identity control the area exponent while the width exponent is universally pinned near $1/3$?

---

### H1. Dynamic lacunarity: explaining the area ICC via filling fraction drift
Area and bounding box dimensions are geometrically related by:
$$
A(t) = \eta(t) \cdot W_x(t) W_y(t)
$$
where $\eta(t) \in (0, 1]$ is the territory compactness (filling fraction).
Taking logarithmic derivatives:
$$
a(t) = 2 \alpha_w(t) + \frac{d \ln \eta}{d \ln t} \equiv 2 \alpha_w(t) + \gamma_\eta(t)
$$

- **Hypothesis:** The excess rule-specificity in the area exponent ($ICC = 0.38$) is entirely carried by the logarithmic drift of the filling fraction, $\gamma_\eta$. Rules with $a > 2/3$ are steadily consolidating internal bays ($\gamma_\eta > 0$), while rules with $a < 2/3$ are steadily developing internal dendritic fissures or lacunarity ($\gamma_\eta < 0$).
- **Test:** Compute $\eta(t) = A(t) / (W_x(t) W_y(t))$ across all 152 rules with $\ge 6$ runs in `data/b2_probe.csv` in octave windows $k = 20..26$. Fit $\gamma_\eta = d \ln \eta / d \ln t$.
- **Prediction:**
  1. $\gamma_\eta$ accounts for $> 80\%$ of the variance in $a - 2\alpha_w$ ($R^2 \ge 0.80$).
  2. The between-rule ICC of $\gamma_\eta$ is $\ge 0.35$ (matching the excess ICC of area).
  3. The mean compactness $\bar{\eta}$ is negatively correlated with $\gamma_\eta$: rules with low initial compactness consolidate fastest.
- **What breaks it:** If $\gamma_\eta \approx 0$ for all rules, or if $\gamma_\eta$ fails to correlate with $a - 2\alpha_w$, the area-width discrepancy is an artifact of bounding-box orientation rather than internal compactness.

---

### H2. Anisotropic elongation under arrow bias
In the biased rotor (C2b/c), why does bias $0.25$ jump the area exponent from $0.68$ to $0.79$ while the width exponent remains $0.33$?

- **Hypothesis:** Arrow bias induces an elliptic deformation of the territory. If the major axis grows as $W_\parallel \sim t^{\alpha_1}$ and the minor axis as $W_\perp \sim t^{\alpha_2}$, bounding-box width is dominated by the major axis ($\alpha_w = \alpha_1 \approx 1/3$), while area grows as $A \sim W_\parallel W_\perp \sim t^{\alpha_1 + \alpha_2}$. At bias 0.25, the minor axis grows faster than the major axis ($\alpha_2 \approx 0.46$), filling in transverse space.
- **Test:** Measure the principal eigenvalues $\lambda_1(t), \lambda_2(t)$ of the spatial gyration tensor for the biased rotor at bias $\beta \in \{0, 0.05, 0.10, 0.15, 0.20, 0.25\}$ up to $2^{28}$ steps. Fit the individual exponents $\alpha_1, \alpha_2$ of $\sqrt{\lambda_1}$ and $\sqrt{\lambda_2}$.
- **Prediction:**
  1. At bias $0$, $\alpha_1 = \alpha_2 = 0.335 \pm 0.005$.
  2. At bias $0.25$, $\alpha_1 = 0.333 \pm 0.008$, while $\alpha_2 = 0.455 \pm 0.015$.
  3. The sum $\alpha_1 + \alpha_2 = 0.788 \pm 0.015$, exactly matching the measured area exponent $0.787$.
- **What breaks it:** If $\alpha_1$ and $\alpha_2$ remain equal at bias $0.25$, the area exponent increase is not driven by anisotropic axis scaling.

---

### H3. Universal scaling collapse of the blob class
Can the diverse growth curves of all core blob rules be collapsed onto a single universal master curve?

- **Hypothesis:** All core blob rules belong to a single scale-invariant universality class governed by the dimensionless time:
  $$
  \tau = (c_r \cdot \kappa_r)^{3/2} \cdot t
  $$
  Under this rescaling, the area growth curve $\tilde{A}(\tau) = A(t) / (c_r \kappa_r)$ and the radial density profile $\tilde{\rho}(\tilde{r})$ collapse onto a universal curve across all blob rules with no free parameters.
- **Test:** Take 30 diverse blob rules from `data/b2_probe.csv` with $\kappa$ spanning $0.4$ to $2.5$. Rescale $A(t)$ by $\tau^{2/3}$ and compute the variance across rules before and after rescaling.
- **Prediction:** Rescaling by $(c_r \kappa_r)$ reduces the between-rule variance of normalized area $A(t) / t^{2/3}$ by $> 75\%$.
- **What breaks it:** If the variance reduction is $< 30\%$, the prefactor $K_r$ depends on additional invariants beyond $c_r$ and $\kappa_r$.

---

## I. Interface universality and surface roughness

### Context
Phase 2 settled the asymptotic behavior of the surface width exponent (A4c, Q7):
- Ants: $W_{\text{surf}} \sim t^{0.235 \pm 0.015}$ out to $2^{34}$.
- Rotor: $W_{\text{surf}} \sim t^{0.125 \pm 0.015}$ out to $2^{34}$.
The factor-of-two gap does not close over 6 decades. Yet both have identical bulk growth exponent $\approx 1/3$.
Kapri & Dhar conjectured $W_{\text{surf}} \sim t^{1/8} = t^{0.125}$ for the rotor. For ants, $0.25 = 1/4$ matches the classic Edwards-Wilkinson (EW) interface.

---

### I1. Edwards-Wilkinson vs Eulerian hyperuniform interface class
In standard interface growth in $1+1$ dimensions:
- **Edwards-Wilkinson (EW):** temporal exponent $\beta = 1/4 = 0.25$, spatial roughness exponent $\chi = 1/2 = 0.50$.
- **Kardar-Parisi-Zhang (KPZ):** temporal exponent $\beta = 1/3 \approx 0.33$, spatial roughness exponent $\chi = 1/2 = 0.50$.
- **Eulerian / Rotor interface:** Kapri & Dhar propose $\beta = 1/8 = 0.125$, $\chi = 1/4 = 0.25$.

- **Hypothesis:** The ant's perimeter belongs to the Edwards-Wilkinson class ($\beta = 1/4, \chi = 1/2$), where turn-sequence stochasticity acts as uncorrelated white noise on interface height. The rotor belongs to an Eulerian hyperuniform interface class ($\beta = 1/8, \chi = 1/4$), where the divergence-free rotor field suppresses interface fluctuations.
- **Test:** Unfold the perimeter $s \in [0, P(t)]$ into a 1D interface $h(s, t) = r(s, t) - \bar{r}(t)$. Compute the spatial height-height correlation function:
  $$
  C(l, t) = \langle (h(s+l, t) - h(s, t))^2 \rangle_s
  $$
  across spatial lags $l \in [2, P/4]$ for 10 core ants and 10 rotors at $t = 2^{30}$. Fit $C(l) \sim l^{2\chi}$.
- **Prediction:**
  1. For ants, $\chi = 0.50 \pm 0.04$ (Edwards-Wilkinson).
  2. For the rotor, $\chi = 0.25 \pm 0.04$ (Kapri-Dhar Eulerian class).
  3. This proves that ants and rotors belong to **distinct interface universality classes** despite sharing the bulk $t^{1/3}$ exponent.
- **What breaks it:** If ants have $\chi \approx 0.25$, they share the rotor's surface class and $\beta \approx 0.24$ is a very slow transient.

---

### I2. Hyperuniformity of boundary step fluctuations
Why would the rotor suppress surface fluctuations compared to the ant?

- **Hypothesis:** Rotor boundary visits are hyperuniform: in equal perimeter angular sectors of angle $\Delta \theta$, the variance of visit counts $\text{Var}(N)$ scales sub-linearly with the mean: $\text{Var}(N) \sim \langle N \rangle^{\gamma}$ with $\gamma < 1$. For ants, boundary visits are diffusive/Poissonian: $\text{Var}(N) \sim \langle N \rangle^1$.
- **Test:** Partition the boundary into 64 equal angular sectors. Count visits $N_i$ to each sector in octave windows $k = 22..26$ for 8 ants and 8 rotors. Fit $\text{Var}(N)$ vs $\langle N \rangle$.
- **Prediction:**
  1. For ants, $\gamma = 1.00 \pm 0.05$ (Poissonian).
  2. For rotors, $\gamma = 0.52 \pm 0.08$ (hyperuniform suppression of fluctuations).
- **What breaks it:** If both have $\gamma \approx 1$, hyperuniformity does not explain the rotor's smoother surface.

---

## J. Tethering physics: phase-space resonance, hubs, and formation

### Context
In Phase 2, three striking discoveries re-framed the nature of tethered ("hard") rules:
1. `RRRRRLLLLLL` tether is intermittent in age: taking the frozen field at $2^{25}$ and dropping a fresh ant at the origin heading North yields an **untethered** walk (`near_home` 0.061), whereas at $2^{22}, 2^{24}, 2^{26}$ it is tethered (0.36–0.47) (Q9).
2. The self-healing hypothesis was **reversed**: corrupting the *most-visited* cells hurts most, and rare colours double the damage at equal visit quartile (A6, Q13).
3. The hot windows become critical *late* in development ($2^{22}..2^{24}$), not early (E6).
4. Contiguous 5x5 patches cause $\approx 35\%$ less damage per rare cell changed than scattered swaps (E4, Q14).

---

### J1. The phase-space lock: mapping the attraction basin of RRRRRLLLLLL (Q9)
Why does the $2^{25}$ frozen field fail to tether a fresh ant starting North at the origin?

- **Hypothesis:** Tethering is a phase-locked limit cycle on the joint space $(x, y, \text{heading}, \text{colour})$. The basin of attraction $\mathcal{B}_{\text{home}}$ occupies only a tiny volume fraction of the core state space ($< 5\%$). The native ant remains tethered because its internal heading and arrival coordinates are synchronized with the field's cyclic rewrites; dropping an ant with an arbitrary heading (North) lands outside the basin.
- **Test:** On the frozen $2^{25}$ field of `RRRRRLLLLLL`, test all 4 headings $\{N, E, S, W\}$ and all entry coordinates $(x, y)$ in the core $r \le 10$ (441 sites $\times$ 4 headings = 1,764 initial conditions). Run each for $10^7$ steps and record `near_home`. Also extract the *exact* native state $(x^*, y^*, h^*)$ of the continuous run at step $2^{25}$.
- **Prediction:**
  1. Fewer than $5\%$ of test states achieve `near_home` $> 0.30$.
  2. The native state $(x^*, y^*, h^*)$ is strictly inside the tethered basin (`near_home` $\ge 0.40$).
  3. The tethered initial conditions form connected 1D orbit filaments in phase space, not a solid 2D spatial disc.
- **What breaks it:** If $> 50\%$ of initial conditions are tethered, the $2^{25}$ North start was an unlucky rare exception rather than evidence of phase locking.

---

### J2. The home as a reversible permutation network: centrality of critical cells
Why do the most-visited cells hurt most when corrupted, and why do rare colours double the damage?

- **Hypothesis:** The home functions as a cycle-preserving permutation network. The most-visited cells are high-centrality "hub switches" in the transition graph $G = (V, E)$; corrupting a hub breaks the cycle closure, converting closed recurrent loops into escaping open paths. Rare colours act as conditional control bits (branch selectors); corrupting a rare colour redirects entire loop families.
- **Test:** Construct the directed transition multigraph $G$ on cells within $r \le 12$ of the tether center for `RRRRRLLLLLL` over $10^6$ steps. Compute betweenness centrality $C_B(v)$ and page rank for every cell. Correlate $C_B$ with single-cell knockout damage measured via `examples/knock.rs`.
- **Prediction:**
  1. Knockout loss correlates strongly with betweenness centrality: Spearman $\rho(\text{loss}, C_B) \ge 0.72$.
  2. Controlling for $C_B$, cells holding rare colours still exhibit a $1.8\times \pm 0.3\times$ higher loss than common colours (confirming their role as branch selectors).
- **What breaks it:** If knockout loss does not correlate with betweenness centrality ($|\rho| < 0.25$), the network topology does not explain why visit frequency dictates vulnerability.

---

### J3. Contiguity penalty vs spatial dispersion: resolving E4 / Q14
Why does a contiguous 5x5 patch cost $\approx 35\%$ less per rare cell changed than scattered swaps?

- **Hypothesis:** High-centrality critical cells are clustered in an ultra-compact core ($r \le 4$). A contiguous 5x5 patch inevitably covers both critical hub cells and redundant peripheral cells (diminishing the marginal loss per corrupted cell), whereas scattered swaps select rare cells independently across multiple separate functional loops.
- **Test:** Compare contiguous $3 \times 3$ and $5 \times 5$ patches positioned directly over the highest-centrality hubs against scattered swaps whose radial distance distribution from the center is strictly matched to that of the patch.
- **Prediction:** When radial distance and hub centrality are matched, the damage ratio of patches to swaps shifts from $0.65$ to $1.00 \pm 0.12$.
- **What breaks it:** If radially matched patches still cause $< 75\%$ of the damage of swaps, spatial contiguity inherently promotes self-healing.

---

### J4. Late home formation: the condensation transition (E6)
Why do critical windows only become sensitive late in history ($2^{22}..2^{24}$)?

- **Hypothesis:** The home undergoes a sharp condensation transition. Prior to $t_c \approx 2^{21}$, the ant is in an extended chaotic transient with $R(t) \sim t^{1/3}$. At $t_c$, the boundary closes into a self-reinforcing reflecting cage; once closed, subsequent steps accrete frozen rings from inside out, locking the critical cells into place.
- **Test:** Trace the maximum displacement $R_{\max}(t)$ and the radius of gyration $R_g(t)$ continuously from $t = 10^5$ to $10^8$ for `RRRRRLLLLLL` and `RLLLLRRRRLR`.
- **Prediction:** $R_g(t)$ grows as $t^{0.33 \pm 0.03}$ up to a sharp transition time $t_c \in [10^6, 5\times 10^6]$, where $d R_g / dt$ drops abruptly by $> 90\%$ within a single octave.
- **What breaks it:** If $R_g(t)$ shows a smooth, continuous logarithmic deceleration across all octaves, there is no discrete condensation transition.

---

## K. Microscopic reversibility, dimensions, and skeleton classes

### Context
Phase 2 resolved the 3D exponents ($V \sim t^{0.769}$, $R \sim t^{0.237}$, D2c) and showed that the turmite $\sqrt{t}$ cluster is a 1D filamentary skeleton (D1).
Two fundamental theoretical questions remain open:
1. What role does microscopic invertibility play in enforcing the $t^{1/3}$ law?
2. What is the limit shape in 3D, and do the 289 ant frame builders belong to the turmite 1D skeleton class?

---

### K1. The No-Interior-Void theorem and microscopic reversibility
Why do Langton's ants grow as compact 2D discs with zero internal unvisited voids, whereas non-reversible growth models (Eden, DLA, percolation) produce porous fractals?

- **Hypothesis:** Microscopic invertibility prohibits isolated unvisited interior voids. An unvisited void requires a perimeter loop that self-intersects and traps empty sites; in an invertible deterministic automaton, any such enclosing orbit would create a conflict of pre-images upon time reversal.
- **Test:** Compute the Euler characteristic $\chi = N_{\text{components}} - N_{\text{holes}}$ and total void volume $V_{\text{void}}$ of the visited territory across 200 blob rules at $2^{26}$ steps. Compare against an explicitly non-invertible ant variant: rule updates increment $c \to (c+1) \bmod (n-1)$ when $c > 0$, merging colours 0 and 1 into a common absorbing sink.
- **Prediction:**
  1. For reversible ants, $V_{\text{void}} = 0$ for $100\%$ of rules at $2^{26}$ steps (no internal voids).
  2. For the non-reversible variant, $> 35\%$ of rules produce permanent internal voids with $V_{\text{void}} / A \in [0.02, 0.18]$, and the area growth exponent crosses over from $2/3$ to $0.85 \pm 0.08$.
- **What breaks it:** If any standard reversible ant generates a verified internal unvisited void of area $\ge 5$ cells, reversibility does not prohibit voids.

---

### K2. 3D limit shape: spherical isotropy vs faceted polyhedra (Q12)
In 3D, volume grows as $t^{0.769}$ and extent as $t^{0.237}$ (approaching $3/4$ and $1/4$). But is the asymptotic shape a sphere, an octahedron, or a cube?

- **Hypothesis:** The 3D limit shape is an isotropic sphere. The anisotropy tensor $A_{ij} = I_{ij} / \text{Tr}(I) - \frac{1}{3} \delta_{ij}$ decays toward zero as a power law:
  $$
  \|A(t)\| \sim t^{-\mu}, \quad \mu \approx 0.07 \pm 0.02
  $$
  explaining why the filling fraction converges from below.
- **Test:** Compute the inertia tensor eigenvalues $\lambda_1 \le \lambda_2 \le \lambda_3$ for the 3D ant at octave windows $k = 24..34$. Compute the sphericity $\Psi = \lambda_1 / \lambda_3$.
- **Prediction:** $\Psi(t)$ increases monotonically from $\approx 0.72$ at $2^{24}$ to $\ge 0.88$ at $2^{34}$, extrapolating to $1.00 \pm 0.03$ as $t \to \infty$.
- **What breaks it:** If $\Psi(t)$ plateaus at $< 0.80$, the 3D limit shape is an anisotropic or faceted polyhedron (e.g. rhombic dodecahedron).

---

### K3. Unifying the 289 ant frame-builders with the 1,560 turmite skeletons (Q15, D1)
The census contains 289 rules that stay on the grid but fill bounding boxes at only 2–6 steps per cell ("frame builders", e.g. `RLLR`). Turmites have an exact $\sqrt{t}$ cluster of 1,560 tables that form 1D skeletons (area/extent$^2 \approx 0.001$).

- **Hypothesis:** The 289 ant frame builders belong to the exact same dynamical class as the $\sqrt{t}$ turmite cluster: they expand by executing periodic perimeter patrols of length $L(t) \sim t^{1/2}$, laying a 1-cell thick boundary frame every cycle.
- **Test:** Run all 289 frame builders to $2^{26}$ steps using `src/bin/probe.rs`. Measure bounding box side $W(t)$ and area $A(t)$.
- **Prediction:**
  1. For $\ge 85\%$ of the 289 frame builders, the width exponent is $\alpha_w = 0.50 \pm 0.03$ (matching $\sqrt{t}$).
  2. The area exponent is $a \le 0.55$, and compactness $A / W^2$ decays as $t^{-0.48 \pm 0.04}$.
  3. This unifies the 289 ant frame-builders with the 1,560 turmite $\sqrt{t}$ tables into a single 1D perimeter-patrolling universality class.
- **What breaks it:** If the frame builders have $\alpha_w \approx 0.33$, they are slow-filling 2D blobs rather than 1D frames.

---

## L. Suggested priority and resource budget

| Priority | Item | Target | Cost | What it decides |
|---|---|---|---|---|
| **1** | **F1** | Interior excursion distribution $P(L)$ | ~1 CPU-hour (`data/armc_specs.txt`) | Explains what sets rule-specific $\kappa$ |
| **2** | **I1** | Interface correlation $C(l)$ (EW vs Rotor) | ~2 CPU-hours (20 runs to $2^{30}$) | Decides whether ants and rotors are different surface universality classes |
| **3** | **G1** | `RLRLLL` escape trace | < 15 CPU-min (`escape_trace.rs`) | Reveals the mechanism of non-highway grid breakouts |
| **4** | **H1** | Dynamic lacunarity $\gamma_\eta$ | Analysis of existing `b2_probe.csv` | Resolves the area ICC (0.34) vs width ICC (0.12) discrepancy |
| **5** | **J1** | `RRRRRLLLLLL` phase-space basin | ~2 CPU-hours (`examples/tetherlab.rs`) | Explains why frozen fields fail to tether arbitrary fresh ants |
| **6** | **K3** | 289 ant frame-builders vs turmite $\sqrt{t}$ | ~3 CPU-hours (`sweep_lean`) | Unifies the census frame builders with the 1D turmite cluster |
| **7** | **K1** | No-Interior-Void test (reversibility) | ~2 CPU-hours (`src/colours.rs`) | Links compact disc growth to microscopic invertibility |
| **8** | **G2** | Complete $10^{11}$ survivor census | Ongoing (`data/run_1e11b.sh`) | Decides if late highway onset keeps thinning or reaches a floor |
