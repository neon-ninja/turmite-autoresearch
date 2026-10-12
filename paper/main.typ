// Every number below comes from numbers.json (built by compute_numbers.py).
#let N = json("numbers.json")
#let f(x, d: 3) = str(calc.round(x, digits: d))
#let pct(x) = str(calc.round(100 * x, digits: 0)) + "%"
#let iqr(v, d: 2) = "[" + f(v.at(0), d: d) + ", " + f(v.at(2), d: d) + "]"

#set document(title: "Cube-root growth of chaotic generalised Langton's ants")
#set page(paper: "a4", margin: (x: 2.2cm, y: 2.2cm), numbering: "1")
#set text(font: "New Computer Modern", size: 10.5pt)
#set par(justify: true, leading: 0.62em)
#set heading(numbering: "1.1")
#show heading.where(level: 1): set block(above: 1.4em, below: 0.8em)
#show figure.caption: set text(size: 9pt)
#set math.equation(numbering: "(1)")

#align(center)[
  #text(16pt, weight: "bold")[Cube-root growth of chaotic generalised Langton's ants:\ a calibrated mean-field decomposition]
  #v(0.6em)
  Tim McNamara \
  #text(9pt)[\[Affiliation to be confirmed\] · tim\@accelerant.dev]
]

#v(0.8em)
#block(inset: (x: 1.2cm))[
  *Abstract.* A generalised Langton's ant follows a string over {L, R}: on a cell of colour $c$ it turns by `rule[c]`, advances the cell to colour $(c+1) mod n$, and steps forward. Most rules never build a highway and instead grow an irregular, roughly circular territory. We measure how that territory grows over #N.census.rules canonical rules of length #N.census.n_min–#N.census.n_max, simulating #N.probe.analysed non-highway rules to $2^27$ steps (six to $2^32$). The bounding-box width grows as $t^alpha$ with median $alpha = #f(N.alpha_w_blob.at(0))$ over the #N.probe.blob non-logarithmic rules (rule-bootstrap 95% interval #f(N.alpha_w_blob.at(1)) to #f(N.alpha_w_blob.at(2))), with no dependence on rule length, and the visited area as $t^(2/3)$ to within the calibrated bias. To interpret this we run the same instruments on the rotor (Eulerian) walker, for which the exponent $1/3$ is proved @bourabee2026, and show that the instruments reproduce it with a known finite-window bias. We then split the area growth rate into three measured factors: the fraction of time spent on the territory's edge, the probability that an edge step enters new ground, and the edge-to-area ratio. Over time the edge-entry probability is constant ($|gamma| < 0.05$) for #f(100 * N.identity.frac_blob_gc_small, d: 1)% of these rules and the edge-occupation ratio for #pct(N.identity.frac_blob_gk_small), and the edge scales as $A^(#f(N.mf_blob.beta.at(1), d: 2))$, close to the $A^(1/2)$ that gives the cube root. Rules that deviate (#N.probe.log rules growing logarithmically, #N.probe.fast growing faster than a cube root) do so through drift of the edge-occupation ratio. Over longer ensembles ($2^28$–$2^30$ steps) the time-averaged occupation density of the ants is the same cone, $3(1 - r slash R)$, as the rotor's to within #pct(N.shape.max_rel_diff_ants_rotor_shells0_9), while rules differ in effective area exponent by up to 0.02 and a minority of outlying rules are genuinely different rather than noisy. A rule-letter model of the edge-entry probability explains #pct(N.followup.c_model.r2) of its variance on held-out rules. We report which parts of this picture are tautological, which are tested, and what remains unexplained.
]

= Introduction

Langton's ant @langton1986 is a two-colour automaton whose behaviour is striking: after a chaotic transient of about 10 000 steps it builds a periodic "highway" and escapes. Generalising the turn rule to a string over {L, R} of length $n$ @gale1995 @gajardo2001 gives a family whose long-time behaviour is, for most rules, unknown. For any rule containing both letters the track is known to be unbounded @gale1995 @bunimovich1992, and the dynamics is computationally hard in general @gajardo2002, yet we found no published measurement of how the territory of a rule that does _not_ build a highway grows with time.

Here we make that measurement and ask why the answer is what it is. The data suggest that most non-highway rules grow as $t^(1/3)$ in linear size. That is the exponent of the rotor walker, in which every site holds an arrow that turns by 90° each visit and the walker follows it. For the rotor it was conjectured by Priezzhev et al. @priezzhev1996, supported numerically by Kapri and Dhar @kapri2009, and proved by Bou-Rabee and Peres @bourabee2026 (radius of order $t^(1/3)$, range $t^(2/3)$, convex limit shape). The ant is not an Eulerian walker, so the match is not implied by their result; we offer an empirical decomposition of the growth rate that makes the match natural and tests each factor.

*Contributions.*
+ A census of #N.census.rules rules with growth exponents for #N.probe.analysed of them, reporting the whole distribution, its dependence on rule length, and the rules that fall outside it.
+ A calibration: the same pipeline applied to the rotor walker (known exponent) and a random walk (negative control), giving the estimator's bias and between-seed noise.
+ A three-factor decomposition of the area growth rate, with the factors measured per rule, and a classification of deviating rules by which factor drifts.
+ A held-out predictive model for the edge-entry probability from the rule's letters, and a test of the "well-mixed" assumption behind the second factor.

Sections 5 and 8 separate what is an identity from what is a test, and list what we could not explain. A hypothesis about "homes" that tether some rules to their origin was investigated and is mentioned only in the limitations; we make no claim about it.

= Model and methods

== Rules, census and classes

A rule is a string $r in {L,R}^n$. The ant has a position, a heading and a grid of colours in $ZZ_n$, all initially 0. At each step it reads the colour $c$ under it, turns left or right by `r[c]`, increments the colour to $(c+1) mod n$, and moves one cell. Swapping every L and R gives the mirror-image ant, so we keep only rules starting with R, and drop the trivial rules in which every colour turns the same way. A sweep of all such rules of length 2–12 (4083 rules) ran each on a blank $8192^2$ grid for up to $10^8$ steps with a highway detector (`data/sweep_v2.csv`); 578 escaped or were detected as highway builders (including classic `RL`). The census studied here is the remaining #N.census.rules rules of length #N.census.n_min–#N.census.n_max (3505 that ran the full budget plus 7 that escaped late).

Each census rule was then run to $2^27$ steps (`data/growth.csv`, `data/census.py`). A rule is called a _highway builder_ (#N.census.builders rules) if it did not complete $2^27$ steps inside the grid or its steps per bounding-box cell at $10^8$ steps were below 20; builders grow linearly and are excluded. Of the rest, a rule is _log-growing_ (#N.census.log rules) if the fitted width exponent from $t=2^21$ to $2^27$ is below 0.15, and otherwise _blob_ (#N.census.blob rules). The cut at 0.15 is chosen by eye from the gap in the exponent histogram and has not been tuned against any outcome.

== Instrumented runs

The probe (`src/probe.rs`) tracks the _visited_ set (every cell the ant has ever stood on), its area $A$, its _boundary_ $P$ (visited cells with at least one unvisited 4-neighbour), and, in octave windows $[2^k, 2^(k+1))$, the number of steps that start on a boundary cell and the number of steps that enter a never-visited cell. It also gives the bounding box, the radius of gyration $R_g$ of the visited set, and the ant's own spread. For each rule we fit log–log slopes over windows $k = 20 ... 26$:

- width exponent $alpha$ (bounding-box side against $t$) and $R_g$ exponent $alpha_g$;
- area exponent $a$ ($A tilde t^a$);
- boundary exponent $beta$ ($P tilde A^beta$).

#N.probe.analysed rules were analysed; #N.probe.unanalysed of the #(N.census.blob + N.census.log) growing rules had no usable window set (the ant left the probe grid before $2^26$ steps, or a window contained no new cells) and were not fitted, and #N.probe.bursty were flagged as bursty and removed from the statistics below (the flag is a column of `data/probe_summary.csv` computed from the largest single-window jump, `max_jump`, and its share of growth, `max_share`; the script that set it was not preserved in the repository, but the flag is reproduced exactly by a threshold on `max_jump`: every flagged rule has `max_jump` $>=$ #f(N.bursty_rule.flagged_min_jump, d: 2) and every unflagged rule $<=$ #f(N.bursty_rule.unflagged_max_jump, d: 2), and including the flagged rules moves the blob median width exponent from #f(N.alpha_w_blob.at(0)) to #f(N.with_bursty.alpha_w.at(0))). The unfitted rules are #pct(N.probe.unanalysed / (N.census.blob + N.census.log)) of growing rules, and we do not know whether they differ systematically; in particular rules that outgrow the grid are the fastest growers, so the fast tail is likely under-represented.

The census contains #N.repeats.n rules that are exact repeats of a shorter rule ($u^k$, which behave identically to $u$); we keep them, since removing them moves the blob median width exponent only from #f(N.repeats.blob_median_alpha_w_all, d: 4) to #f(N.repeats.blob_median_alpha_w_primitive, d: 4).

== The three-factor decomposition

Per window of $S$ steps let $B$ be the number of steps that start on a boundary cell and $Delta A$ the number that enter new ground. Define

$ c = (Delta A) / B, quad kappa = (B slash S) / (P slash A), quad (d A) / (d t) = c dot kappa dot P / A. $ <eq:rate>

The last equality is an _identity_ given these definitions. Hence if $c$ and $kappa$ drift as $t^(gamma_c)$ and $t^(gamma_kappa)$ and $P tilde A^beta$, then

$ a = (1 + gamma_c + gamma_kappa) / (2 - beta), quad alpha approx a / 2. $ <eq:a>

@eq:a is also algebra, so agreement between the measured $a$ and @eq:a (median error #f(N.identity.median_abs_err_blob) over blob rules) checks the fits for consistency only. The empirical content is in the _values_ of the three factors: $kappa$ is the fraction of time on the boundary relative to a uniform spread over the territory, $c$ the chance a boundary step breaks new ground. If $gamma_c = gamma_kappa = 0$ and $beta = 1/2$, @eq:a gives $a = 2/3$.

== Reproducibility

All simulation is deterministic and seeded (rules start from a blank board, so a rule has one trajectory; seeds enter only in the walkers and the patched-start control). The Rust fast ant used for sweeps is tested step for step against an oracle (`tests/ant.rs`, `tests/probe.rs`). Every number in this paper is computed by `paper/compute_numbers.py` from the files in `data/` and written to `paper/numbers.json`, which this document reads; figures are made by `paper/figures.py`. Bootstrap intervals use a fixed seed. Commands are in `paper/README.md`.

= Calibration against the rotor walker

An exponent fitted over seven octaves from $2^20$ to $2^26$ is not the asymptotic exponent. We therefore ran the identical pipeline on #N.calib_A.rotor.alpha_w.n rotor-walker seeds (random initial arrows, $2^27$ steps), whose width exponent is $1/3$ and area exponent $2/3$ @bourabee2026, and on 16 random walks as a negative control.

#figure(
  table(
    columns: (auto, auto, auto, auto, auto),
    align: (left, right, right, right, right),
    stroke: (x: none), inset: 4pt,
    [], [width $alpha$], [$R_g$ exp.], [area $a$], [$beta$],
    [truth (rotor)], [1/3], [1/3], [2/3], [--],
    [rotor, median], [#f(N.calib_A.rotor.alpha_w.median)], [#f(N.calib_A.rotor.alpha_rg.median)], [#f(N.calib_A.rotor.a.median)], [#f(N.calib_A.rotor.beta.median)],
    [rotor, between-seed sd], [#f(N.calib_A.rotor.alpha_w.sd)], [#f(N.calib_A.rotor.alpha_rg.sd)], [#f(N.calib_A.rotor.a.sd)], [#f(N.calib_A.rotor.beta.sd)],
    [ants (core), median], [#f(N.calib_A.ants_core.alpha_w.median)], [#f(N.calib_A.ants_core.alpha_rg.median)], [#f(N.calib_A.ants_core.a.median)], [#f(N.calib_A.ants_core.beta.median)],
    [ants (core), between-rule sd], [#f(N.calib_A.ants_core.alpha_w.sd)], [#f(N.calib_A.ants_core.alpha_rg.sd)], [#f(N.calib_A.ants_core.a.sd)], [#f(N.calib_A.ants_core.beta.sd)],
    [random walk, median], [#f(N.calib_A.random_walk.alpha_w.median)], [#f(N.calib_A.random_walk.alpha_rg.median)], [#f(N.calib_A.random_walk.a.median)], [#f(N.calib_A.random_walk.beta.median)],
  ),
  caption: [Same pipeline on three systems. The rotor has known exponents; the random walk is a negative control (area exponent near 1, boundary exponent near 1).],
) <tab:calib>

Findings. (i) The pipeline recovers the rotor: the radius-of-gyration exponent is #f(N.calib_A.rotor.alpha_rg.median) (sd #f(N.calib_A.rotor.alpha_rg.sd)) against the true 0.333, the width exponent is slightly low (#f(N.calib_A.rotor.alpha_w.median)) and noisier, and the area exponent overshoots by #f(N.calib_A.rotor.a.median - 2/3, d: 2). We therefore treat the finite-window bias of $a$ as $+0.02$ and prefer the $R_g$ estimator, which is the least noisy. (ii) The random walk is clearly separated (area exponent #f(N.calib_A.random_walk.a.median, d: 2)). (iii) The ants' $R_g$ exponent #f(N.calib_A.ants_core.alpha_rg.median) and area exponent #f(N.calib_A.ants_core.a.median, d: 2) sit within the rotor's bias and sd. (iv) The boundary exponent of both is about #f(N.calib_A.rotor.beta.median, d: 2), not 1/2; it is a finite-window effect shared with the rotor and not evidence of a rough edge. To check the pipeline further we reproduced the published rotor measurements of Kapri and Dhar @kapri2009 from #N.calib_B.seeds_long seeds (radius exponent local slopes #N.calib_B.nu_local.map(x => str(x)).join(", "), tending to $1/3$; boundary-width exponent local slopes falling from #N.calib_B.delta_local.first() to #N.calib_B.delta_local.last(), against their reported value $alpha slash 3 approx 0.13$).

A reservation: for the rotor, the true limit exponent is known, but only by proof for the radius; our statements about its $beta$ and $c$ are measurements, not theorems.

= Results: the census

#figure(
  image("fig/exponents.pdf", width: 100%),
  caption: [Fitted exponents of #N.probe.blob blob-class ant rules and #N.calib_A.rotor.alpha_w.n rotor seeds. Dashed line: $1/3$. The ants' distribution is wider than the rotor's: sd ratio #f(N.calib_A.sd_ratio_alpha_w, d: 1) (width), #f(N.calib_A.sd_ratio_alpha_rg, d: 1) ($R_g$).],
) <fig:exp>

*Exponent.* Over the #N.probe.blob blob rules the median width exponent is #f(N.alpha_w_blob.at(0)) (rule bootstrap 95% interval #f(N.alpha_w_blob.at(1)) to #f(N.alpha_w_blob.at(2))), the $R_g$ exponent #f(N.alpha_rg_core.at(0)), and the area exponent #f(N.a_core.at(0)) in the core. The interquartile range of the width exponent is #iqr(N.iqr_alpha_w_blob). The bootstrap resamples _rules_; it does not include the finite-window bias, which @tab:calib puts at about $+0.02$ for $a$ and negative for $alpha$. Against the rotor in the same window the differences (ants minus rotor, medians over all blob rules) are #f(N.bias.alpha_w.ant_minus_rotor, d: 3) for the width exponent, #f(N.bias.alpha_rg.ant_minus_rotor, d: 3) for the $R_g$ exponent and #f(N.bias.a.ant_minus_rotor, d: 3) for the area exponent. Subtracting the rotor's own bias from the ants' medians gives #f(N.bias.alpha_w.ant_corrected, d: 3) (width), #f(N.bias.alpha_rg.ant_corrected, d: 3) ($R_g$) and #f(N.bias.a.ant_corrected, d: 3) (area), against $1/3$, $1/3$ and $2/3$. The corrections point in different directions, so the data are consistent with $1/3$ at roughly the $plus.minus 0.01$ level, not "equal to $1/3$ to three digits"; the correction assumes the ants share the rotor's finite-window bias, which we cannot check.

*Rule length.* The median is flat in $n$: Spearman $rho$ between $alpha$ and $n$ is #f(N.spearman_alpha_n) over blob rules, and per-length medians for $n = 7 ... 12$ lie within about 0.01 of one another (table in `paper/numbers.json`, `by_n`). The spread between rules does not shrink with $n$ in our sample, so we do not claim a rule-length trend or its absence at larger $n$.

*Long runs.* Six typical rules were run to $2^32$ steps; their width exponents fitted from $2^17$ are between #f(N.cube.long_alpha.at(0)) and #f(N.cube.long_alpha.at(1)), and neither factor in the decomposition drifts (@fig:ck).

*Noise.* The between-rule scatter is mostly run-to-run: re-running #N.calib_C.rules rules from #N.calib_C.runs random $5 times 5$ starting patches gives within-rule sd #f(N.calib_C.alpha_w.sd_within) for $alpha$ and a between-rule true sd of only #f(N.calib_C.alpha_w.sd_between_true) (intraclass correlation #f(N.calib_C.alpha_w.icc, d: 2)). Genuine rule-to-rule differences in $alpha$ are small; for $R_g$ and $a$ they are detectable (ICC #f(N.calib_C.alpha_rg.icc, d: 2), #f(N.calib_C.a.icc, d: 2)). We cannot say whether they reflect different exponents or different rates of convergence.

= Results: the three factors

For the #N.probe.core core rules (those with $0.30 < alpha < 0.37$), the medians (and quartile ranges) of the factors are $c = #f(N.mf_core.c.at(1), d: 2)$ #iqr(N.mf_core.c), $kappa = #f(N.mf_core.kappa.at(1), d: 2)$ #iqr(N.mf_core.kappa) and $beta = #f(N.mf_core.beta.at(1), d: 3)$ #iqr(N.mf_core.beta, d: 3), with territory fill (area over bounding box) #f(N.mf_core.fill.at(1), d: 2) #iqr(N.mf_core.fill). The drift exponents are tight around zero: $|gamma_c| < 0.05$ for #pct(N.identity.frac_blob_gc_small) and $|gamma_kappa| < 0.05$ for #pct(N.identity.frac_blob_gk_small) of all #N.probe.blob blob rules (the fractions are computed on the whole class, because "core" is itself defined by a width exponent near 1/3 and would make small drifts nearly automatic). @fig:ck shows the long runs: both factors plateau from $2^17$ to $2^32$ for six rules. These are the observations that the cube root rests on; they were predicted in advance (`notes/cube-root-hypothesis.md`, 2026-10-10, predictions P1–P4) and could have failed; P4 predicted $beta$ close to $1/2$, and the measured $beta$ is about 0.52, equal to the rotor value in the same window (@tab:calib). The medians quoted here are for the core class, which is selected on the outcome, and are descriptive only.

#figure(
  image("fig/ck_time.pdf", width: 100%),
  caption: [Edge-entry probability $c$ and edge-occupancy ratio $kappa$ against $log_2 t$ for six rules run to $2^32$. Both are flat after $2^20$; $kappa$ fluctuates by roughly ten percent from octave to octave.],
) <fig:ck>

*Comparison with the rotor walker.* For the rotor the same decomposition gives $c = #f(N.rotor_ck.c.at(0), d: 2)$ and $kappa = #f(N.rotor_ck.kappa.at(0), d: 1)$; a random walk gives $c = #f(N.rw_ck.c.at(0), d: 2)$. So $c$ alone does not discriminate (the random walk's $c$ is close to the ants'), and the ants' $kappa$ (about #f(N.mf_core.kappa.at(1), d: 1)) is about half the rotor's. The cube root does not require the same $c$ and $kappa$ as the rotor; it requires that they be constant and that the edge scale as $A^(1/2)$. The ant's edge occupation is _lower_ than uniform ($kappa < 1$ for most rules); we have no explanation of the value of $kappa$.

*Deviating rules.* #N.probe.log rules grow logarithmically ($alpha$ median #f(N.cube.log.alpha.at(1))), with $kappa$ median #f(N.cube.log.kappa.at(1), d: 4) and $gamma_kappa = #f(N.cube.log.g_k.at(1), d: 2)$: the ant rarely visits the edge and the occupancy of the edge decays, consistent with an ant confined near its origin. #N.probe.fast blob rules have $alpha > 0.37$ with $gamma_kappa$ median #f(N.cube.fast.g_k.at(1), d: 2) (edge occupation rises). Across blob rules the correlation of $alpha$ with $gamma_kappa$ is #f(N.identity.corr_alpha_gk, d: 2), with $gamma_c$ #f(N.identity.corr_alpha_gc, d: 2) and with $beta$ #f(N.identity.corr_alpha_beta, d: 2) (@fig:dev). So the outliers are outliers because the ant's occupation of the edge drifts, which is the failure condition we wrote down in advance for the mechanism ("$kappa$ drifting strongly with $t$"). This is a restatement through @eq:a, not an explanation: _why_ $kappa$ drifts for these rules is unexplained here.

#figure(
  image("fig/deviations.pdf", width: 100%),
  caption: [Left: width exponent against the drift exponent of $kappa$. Right: area exponent against the value predicted by @eq:a; the agreement is by construction up to fit error (see text).],
) <fig:dev>

= Results: what sets $c$, and is the ant well mixed?

*Edge-entry probability.* Across rules $c$ ranges from about 0.1 to 0.67. A simple model predicts it from the rule's letters: the edge is chiral (for edge steps on colours 1 to 3, the share with fresh ground on the left exceeds that on the right in #N.followup.chiral.left_more of #N.followup.chiral.rules calibration rules; medians #f(N.followup.chiral.left_med, d: 2) against #f(N.followup.chiral.right_med, d: 2)), and so a rule that turns L on a young colour enters new ground more often. Constants (the share of edge steps by colour and the left/right fresh-ground rates) are fitted on a 300-rule sample and applied to the others without refitting. On #N.followup.c_model.n held-out rules the model attains $R^2 = #f(N.followup.c_model.r2, d: 2)$ (correlation #f(N.followup.c_model.corr, d: 2); median absolute error #f(N.followup.c_model.mae_med, d: 2)); grouping rules by their first letter after the leading R explains #f(N.followup.r2_letters.r1, d: 2) of the variance of $c$, by the next three letters jointly #f(N.followup.r2_letters.r123, d: 2), and by the sixth letter alone (rules of length at least 8) #f(N.followup.r2_letters.r6, d: 3). The model was a post-hoc fit to these data, though evaluated on rules outside its calibration sample; we have not tested it on rules of length above 12.

#figure(
  image("fig/cmodel.pdf", width: 55%),
  caption: [Held-out edge-entry probability: model against measurement. The model takes few distinct values; the $R^2$ is mostly between-cluster, and within the clusters near 0.33 and 0.53 it discriminates poorly (rules with predicted $c approx 0.53$ have measured $c$ from about 0.25 to 0.67).],
) <fig:cm>

*Mixing.* The mean-field step assumes the ant is spread roughly evenly over its territory, behaving like a persistent random walk with turning probability $q = \#R slash n$. The sample is the #N.mix_sample.listed rules in `data/mix_rules.txt`, of which #N.mixing.typ_n are blob rules with $0.30 < alpha < 0.37$; the procedure used to choose the list was not recorded, so it should be read as a convenience sample. We therefore repeated the test on #N.mixing_random.n rules drawn uniformly at random (fixed seed) from the core class, with predictions written beforehand (`notes/mixing-random-sample.md`): #N.mixing_random.uni_n of #N.mixing_random.n have near-uniform interior colours (our prediction of 60–85% was exceeded), their MSD-slope ratio has median #f(N.mixing_random.uni_ratio.at(1), d: 2) (range #f(N.mixing_random.uni_ratio.at(0), d: 2) to #f(N.mixing_random.uni_ratio.at(2), d: 2)), and the remaining #N.mixing_random.non_n have median ratio #f(N.mixing_random.non_ratio_med, d: 2). The convenience-sample numbers follow. For the #N.mixing.typ_n typical rules we tested (a) colour uniformity (total-variation distance of interior colours from uniform), (b) turn correlations, and (c) the slope of mean-square displacement against lag, which for independent turns is $(1 - a')/(1 + a')$ with $a' = (2q-1)^2$. For #N.mixing.uni_n rules with near-uniform interior colours (TV $<$ 0.05) the measured slope is within [#f(N.mixing.uni_ratio.at(0), d: 2), #f(N.mixing.uni_ratio.at(2), d: 2)] of the prediction (median #f(N.mixing.uni_ratio.at(1), d: 2)), the displacement exponent is #f(N.mixing.uni_expo.at(1), d: 2), and the lag-1 turn correlation is #f(N.mixing.uni_corr1.at(1), d: 3) at the median. For the other #N.mixing.non_n rules, whose interior colours are far from uniform, the MSD slope is off by a factor of #f(N.mixing.non_ratio.at(1), d: 2) at the median (correlation between the ratio and the colour deviation across the sample: #f(N.mixing.corr_ratio_tv, d: 2)). Time to mix across the territory is a fraction #f(N.mixing.mix.at(1), d: 3) (median) of the growth time. The ant is therefore well described as a random walker in a random environment for #pct(N.mixing.uni_n / N.mixing.typ_n) of the sampled rules and not for the remainder. That the ant is spread out in time does not by itself imply that its edge occupation is $kappa$; it only makes $kappa$ a stationary quantity, and @fig:ck shows that.

= Long ensembles, the occupation profile and the outliers

Three further measurements, produced by separate scripts (`data/item4_stats.py`, `data/shape_stats.py`, `data/item21_stats.py`; hypotheses and predictions are in `notes/frontier-results.md`, written before each run), bear on the weakest points above.

*Is the exponent universal?* Eleven rules (stratified by length 3–12) were run from #(N.item4.runs_kept) random $5 times 5$ starting patches, 24 seeds each, to $2^30$ steps. The local area exponent, averaged over seeds and over the last six three-octave windows, has mean #f(N.item4.ant_last6_mean, d: 3) across rules (sd #f(N.item4.ant_last6_sd, d: 3), range #f(N.item4.ant_range.at(0), d: 3) to #f(N.item4.ant_range.at(1), d: 3)). The same estimator applied to 24 rotor seeds, whose exponent is proved to be $2/3$, gives #f(N.item4.rotor_last6, d: 3): even the rotor sits $0.01$ above $2/3$ at $2^30$ by this estimator. #N.item4.within_0006_of_rotor of the #N.item4.rules rules are within 0.006 of the rotor; the others are up to 0.02 above it or 0.017 below it by amounts many standard errors wide. One rule (`RRLLLLLLR`) does not self-average: its seed-to-seed spread rises with time. So the data are compatible with a common exponent $2/3$ with slow, rule-dependent convergence, but they cannot distinguish that from rule-dependent exponents within about 0.02.

*The occupation profile.* The time-averaged visit density, as a function of distance from the centroid in units of the territory radius $R_("eq") = sqrt(A slash pi)$ and of $t slash A$, is a cone, $n(r) = 3 (1 - r slash R_("eq")) t slash A$, for ants and rotor alike. Over the first ten of fifteen radial shells the ants' profile (#N.shape.ants.runs runs of 11 rules, $2^#N.shape.ants.kmax$ steps) and the rotor's (#N.shape.rotor.runs seeds, $2^#N.shape.rotor.kmax$ steps) differ by at most #pct(N.shape.max_rel_diff_ants_rotor_shells0_9), and the ant profile differs from the cone by at most #pct(N.shape.max_rel_diff_ants_cone_shells0_9); the across-rule relative sd at mid-radius is #pct(N.shape.ants.profile_rel_sd_shell6). This is a prediction we had expected to fail (we had written that the ant's profile would differ from the rotor's). It is the sharpest quantitative support for the claim that the ant behaves like the rotor on large scales: the density profile, not just the exponent, is shared. The territories are round (median anisotropy #f(N.shape.ants.aniso, d: 3) at $2^28$, the rotor's #f(N.shape.rotor.aniso, d: 3) at $2^26$; fill #f(N.shape.ants.fill, d: 2)). The surface width does _not_ match: its local exponent fitted over $k >= 18$ is #f(N.shape.ants.dW, d: 2) for ants and #f(N.shape.rotor.dW, d: 2) for the rotor (Kapri and Dhar: about 0.13), so the ants' edge is broader at equal size, though both exponents are falling.

*Are the outliers noise?* We re-ran the #N.item21.high_n fastest and #N.item21.low_n slowest blob rules from 12 random patches each. If their exponents had been run-to-run noise they would regress to the middle. They mostly do not: only #pct(N.item21.high_regress) of the fast rules fall below 0.37 and #pct(N.item21.low_regress) of the slow rules rise above 0.30; the Spearman correlation between census and re-measured exponent is #f(N.item21.spearman_census, d: 2), and the re-measured exponent correlates with $gamma_kappa$ at #f(N.item21.spearman_gk, d: 2). So the outliers are real properties of the rules over $2^20$–$2^27$ steps, with edge occupation drifting; whether they are different asymptotes or very slow transients is untestable at this length (the previous paragraph shows even typical rules differ by about 0.02 at $2^30$). This revises our earlier statement that most between-rule scatter is noise: for typical rules it is (within-rule sd 0.016 against a between-rule sd of 0.004), but the tails are not.

= What is tested and what is not

#table(
  columns: (auto, 1fr, auto),
  align: (left, left, left),
  stroke: (x: none), inset: 4pt,
  [*Claim*], [*Status*], [*Evidence*],
  [Width $tilde t^(1/3)$, area $tilde t^(2/3)$ for non-highway rules], [Measured; consistent after the rotor-calibrated bias], [@tab:calib, @fig:exp],
  [$c$ constant in time], [Tested, not refuted], [@fig:ck, $gamma_c$],
  [$kappa$ constant for typical rules], [Tested, not refuted for #pct(N.identity.frac_blob_gk_small) of blob rules; refuted for log and fast classes], [@fig:ck, @fig:dev],
  [$P tilde A^(1/2)$], [Not quite: $beta = #f(N.mf_core.beta.at(1), d: 3)$, equal to the rotor's under the same window], [@tab:calib],
  [@eq:a predicts $a$ for outliers], [Algebraic identity; not a test of mechanism], [@fig:dev (right)],
  [Rate of $c$ from rule letters], [Held-out $R^2 = #f(N.followup.c_model.r2, d: 2)$; length $<= 12$ only], [@fig:cm],
  [Well-mixed ant], [True for #N.mixing.uni_n of #N.mixing.typ_n convenience-sampled rules and #N.mixing_random.uni_n of #N.mixing_random.n random core rules], [Mixing paragraph],
  [Occupation profile is the rotor's cone], [Measured; predicted to fail, did not (within #pct(N.shape.max_rel_diff_ants_rotor_shells0_9) of the rotor)], [Section 7],
  [One exponent for all rules], [Neither confirmed nor refuted: rules differ by up to 0.02 at $2^30$, as does the rotor from its proved value], [Section 7],
  [Outliers are noise], [Refuted for most: #pct(N.item21.high_regress) / #pct(N.item21.low_regress) regress], [Section 7],
  [Surface width matches the rotor], [Not supported: ant exponent #f(N.shape.ants.dW, d: 2) against #f(N.shape.rotor.dW, d: 2)], [Section 7],
  [Why $kappa$ takes its value, why it drifts in some rules], [*Unexplained*], [],
  [Why the edge exponent is exactly 1/2 asymptotically], [*Unexplained*; the finite-window $beta$ is #f(N.mf_core.beta.at(1), d: 2) for ants and rotor alike], [],
)

= Limitations

- *One trajectory per rule.* The ant is deterministic from a blank board; the only noise estimate is the patched-start control (#N.calib_C.rules rules, #N.calib_C.runs runs). Rules are not independent samples of anything; the bootstrap interval is a statement about this set of rules.
- *Finite windows.* All exponents come from seven octaves; the rotor calibration quantifies but does not remove the bias, and the rotor's bias need not equal the ants'. Long runs to $2^32$ cover only six rules.
- *Rule length $<= 12$.* Both the census and the $c$ model stop there. Rules with large $n$ are the majority of all rules; if the cube-root regime is a property of short rules, we cannot tell.
- *Edge exponent at finite time.* A separate check on the six long runs (`data/edge_exponent.py`, output `data/edge_exponent.txt`) gives local boundary exponents of 0.51 to 0.53 at $2^26$–$2^28$, drifting toward but not reaching 1/2, and the rotor's local slope is 0.52 at $2^22$; this is consistent with a finite-time effect shared by both walkers but does not establish the asymptotic value.
- *Exclusions.* #N.probe.unanalysed rules were not fitted and #N.probe.bursty flagged bursty; the highway-builder and log classes use thresholds chosen by eye.
- *Highway builders.* Classification is by finite simulation. A rule counted as chaotic may build a highway after $2^27$ steps (Langton's ant itself takes about $10^4$). Several hundred rules of length $<= 10$ were left unresolved in an earlier highway census (`data/unresolved_le10.txt`).
- *Mechanism is a decomposition, not a derivation.* We do not derive the cube root from the rule; we show it follows from three measured quantities each of which is constant in time for most rules.
- *Related but untested ideas.* Experiments in which the ant's territory was replaced by statistically matched fields (pair, $2 times 2$, $3 times 3$) suggest that the log-growing "tethered" rules depend on the exact configuration of the territory rather than on local statistics. That work was run on two rules at one age with one trajectory each, and a follow-up knockout map has not been run. We draw no conclusion from it in this paper.

= Conclusion

Non-highway generalised Langton's ants of length 3–12 grow, to within the bias our calibration measures, as $t^(1/3)$ in linear size and $t^(2/3)$ in area. The growth rate splits into an edge-entry probability and an edge-occupation ratio that stay constant for the large majority of rules, together with an edge that scales close to $A^(1/2)$; rules that violate the cube root are exactly those whose edge occupation drifts. The result is empirical and stops short of a proof, and the quantitative values of the constants are explained only for $c$ (by a rule-letter model) and not for $kappa$ or $beta$. A natural next step is to measure these factors for the rotor with arrows initialised non-uniformly, where $kappa$ should change in a controlled way, and to extend the census beyond $n = 12$ with a stated-in-advance prediction.

= Availability and statements

*Data and code availability.* [PLACEHOLDER: repository URL and archived DOI (e.g. Zenodo) to be added once the working tree is committed.] The Rust simulator, the Python analysis scripts, the generated data files, `paper/compute_numbers.py` (every number in this text) and `paper/figures.py` are in the repository; the exact commands are in `paper/README.md` and `notes/`. At the time of writing the tree is uncommitted, so no commit hash is quoted.

*Use of AI tools.* [PLACEHOLDER: disclosure of AI assistance in analysis and drafting, wording to follow the target venue's policy.]

*Competing interests and funding.* [PLACEHOLDER]

#bibliography("refs.bib", style: "ieee")
