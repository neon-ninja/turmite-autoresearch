// Census numbers come from numbers.json; forecasts load their reproducible analysis output.
#let N = json("numbers.json")
#let TF = json("../../experiments/2026-10-13-transport-forecast/results/transport_forecast.json").cohorts
#let f(x, d: 3) = str(calc.round(x, digits: d))
#let pct(x) = str(calc.round(100 * x, digits: 0)) + "%"
#let iqr(v, d: 2) = "[" + f(v.at(0), d: d) + ", " + f(v.at(2), d: d) + "]"

#set document(title: "Cube-root growth of most non-highway generalised Langton's ants of length up to 12")
#let ink = rgb("#20201e")
#let accent = ink
#let muted = rgb("#696963")
#let sans = "DejaVu Sans"
#set page(
  paper: "a4",
  margin: (left: 24mm, right: 24mm, top: 23mm, bottom: 23mm),
  header: context if counter(page).get().first() > 1 [
    #set text(font: sans, size: 8pt, fill: muted)
    #grid(columns: (1fr, auto), [GENERALISED LANGTON’S ANTS], [T. McNamara])
  ],
  footer: context [
    #set text(font: sans, size: 8pt, fill: muted)
    #align(right, counter(page).display("1"))
  ],
)
#set text(font: "Libertinus Serif", size: 11pt, fill: ink)
#set par(justify: true, leading: 0.65em, spacing: 0.8em)
#set heading(numbering: "1.1")
#show heading.where(level: 1): it => block(above: 1.7em, below: 0.8em)[
  #set text(font: "Libertinus Serif", size: 18pt, weight: "regular", fill: ink)
  #it
]
#show heading.where(level: 2): it => block(above: 1.2em, below: 0.6em)[
  #set text(font: "Libertinus Serif", size: 12pt, weight: "bold", fill: ink)
  #it
]
#show heading.where(level: 3): set text(font: sans, size: 10.5pt, fill: accent)
#set figure(gap: 0.75em)
#show figure.caption: it => block[
  #set text(font: sans, size: 8.8pt, fill: muted)
  #set par(justify: false, leading: 0.6em)
  #it
]
#set table(stroke: 0.4pt + rgb("#d4dfe3"), inset: 7pt)
#show table: set text(font: sans, size: 9pt)
#show raw: set text(font: "DejaVu Sans Mono", size: 0.85em)
#set math.equation(numbering: "(1)")

#block[
  #set par(justify: false)
  #text(font: sans, size: 8pt, fill: muted, tracking: 1.5pt)[DISCRETE DYNAMICS]
  #v(23pt)
  #text(font: "Libertinus Serif", size: 38pt, weight: "regular", fill: ink)[Cube-root growth\ of most non-highway\ generalised Langton’s ants]
  #v(9pt)
  #text(font: "Libertinus Serif", size: 14pt, style: "italic", fill: muted)[Rules of length $<= 12$: a calibrated mean-field decomposition]
  #v(17pt)
  #text(font: sans, size: 10pt, weight: "bold")[Tim McNamara]
  #v(3pt)
  #text(font: sans, size: 9pt, fill: muted)[Accelerant Ltd, Wellington, New Zealand · tim\@accelerant.dev]
  #v(14pt)
  #line(length: 24mm, stroke: 0.6pt + ink)
]
#v(0.8em)
#block(inset: (top: 8pt, bottom: 8pt))[
  *Abstract.* A generalised Langton's ant follows a string over {L, R}: on a cell of colour $c$ it turns by `rule[c]`, advances the cell to colour $(c+1) mod n$, and steps forward. Most rules never build a highway; they grow an irregular, roughly round territory. Does it grow like the rotor walker, whose radius grows as $t^(1/3)$ @bourabee2026? For most of the #N.probe.analysed non-highway rules of length #N.census.n_min–#N.census.n_max that we simulated (to $2^27$ steps, six to $2^32$), yes: the bounding-box width grows as $t^alpha$ with median $alpha = #f(N.alpha_w_blob.at(0))$, good to about $plus.minus 0.01$ after calibrating the finite-window bias on the rotor, and the time-averaged visit density is the rotor's cone. Some rules do not follow it (#N.probe.log grow logarithmically, #N.probe.fast faster), and the edge does not match: the ants' surface width grows about twice as fast as the rotor's. Splitting the growth rate into an edge-occupation ratio $kappa$ and an edge-entry probability $c$ (an identity, so not independent evidence for the cube root), we find that deviating rules deviate through $kappa$, that $c$ is predictable from the rule's letters (held-out $R^2 = #f(N.followup.c_model.r2, d: 2)$) and $kappa$ is not. What sets $kappa$, and the ant's wider edge, is unexplained.

  #v(0.4em)
 ]
#block(breakable: false, above: 8pt)[
  #align(center, image("fig/hero.pdf", width: 80%, height: 40mm, fit: "contain"))
  #align(center)[
  #set text(font: sans, size: 8pt, fill: muted)
  #set par(leading: 0.45em, spacing: 0pt)
  Same bulk growth. Different measured edge growth.\
  Occupation fields at $2^22$ steps; darkness shows visits per cell in units of $t/A$, on a shared scale from 0 to 4 (values above 4 clipped). Each territory is fitted to its panel.
  ]
]
#pagebreak()
#block[
  #text(font: "Libertinus Serif", size: 18pt, weight: "regular", fill: ink)[Key results]
  #v(0.5em)
  - *Growth.* Over the #N.probe.blob blob rules the median width exponent is #f(N.alpha_w_blob.at(0)), with no dependence on rule length, and the area grows as $t^(2/3)$ to within the same bias (@sec:census-results). The rule-bootstrap interval (#f(N.alpha_w_blob.at(1)) to #f(N.alpha_w_blob.at(2))) resamples rules only and ignores a larger finite-window bias; individual rules differ by up to 0.02 at $2^30$ steps.
  - *The bulk matches the rotor.* The time-averaged visit density is the cone $3(1 - r slash R)$ to within #pct(N.shape.max_rel_diff_ants_rotor_shells0_9) of the rotor's. The cone follows from uniform mixing plus $t^(2/3)$ growth, so this is a consistency check, not extra evidence (@sec:ens).
  - *The edge does not.* The ants' surface width grows with an exponent of about 0.22–0.25 against the rotor's 0.10–0.13 (local exponents at matched sizes $2^22$–$2^32$; a single fit from $2^18$ gives 0.26 against 0.16, and Kapri and Dhar report about 0.13). The gap does not close to $2^34$ steps (@sec:ens).
  - *Not every rule obeys.* #N.probe.log rules grow logarithmically, #N.probe.fast blob rules have $alpha > 0.37$, and #N.census.builders more were excluded as builders or slow space-fillers. Highway classification is by finite simulation.
  - *The decomposition.* Drift exponents below 0.05 hold for #f(100 * N.identity.frac_blob_gc_small, d: 1)% of rules for $c$ and #pct(N.identity.frac_blob_gk_small) for $kappa$, with an edge exponent of #f(N.mf_blob.beta.at(1), d: 2) in the census window. A drift of 0.05 shifts the area exponent by about 0.03, as much as the differences between rules (@sec:factors).
  - *What is new.* Rules that violate the cube root do so through drift of $kappa$, not of $c$. A chirality model explains #pct(N.followup.c_model.r2) of the variance of $c$ on held-out rules; nothing we tried predicts $kappa$, which differs between rules moderately reproducibly (intraclass correlation 0.58) but is not carried by the letters, and a memoryless walker does not reproduce it, which motivates board-memory comparisons without proving memory sufficient (@sec:factors).
  - *Unexplained.* The value of $kappa$ (which sets the prefactor), why it drifts in some rules, and the ant's wider edge. We say which parts are tautological, which are tested, and what remains open.
]

= Introduction

Langton's ant @langton1986 is a two-colour automaton whose behaviour is striking: after a chaotic transient of about 10 000 steps it builds a periodic "highway" and escapes. Generalising the turn rule to a string over {L, R} of length $n$ @gale1995 @gajardo2001 gives a family whose long-time behaviour is, for most rules, unknown. For any rule containing both letters the track is known to be unbounded @gale1995 @bunimovich1992, and the dynamics is computationally hard in general @gajardo2002, yet we found no published measurement of how the territory of a rule that does _not_ build a highway grows with time.

#figure(
  image("fig/timeline.pdf", width: 100%),
  caption: [Langton's ant (`RL`) at six moments, all drawn at the same scale. For about 10 000 steps it wanders and builds an apparently random patch; then it falls into a repeating 104-step cycle that moves it two cells diagonally each time @langton1986 (our sweep reproduces period 104 and onset 9976 for `RL`: `experiments/2026-10-13-census-sweeps/results/sweep_v2.csv`), laying a straight "highway" that continues indefinitely. The rules studied in this paper are those that never do this.],
) <fig:timeline>

#figure(
  image("fig/gallery.pdf", width: 100%),
  caption: [What the ants draw. Each colour is one of the $n$ states a cell can be in; blank is never visited. Top left: classic Langton's ant (`RL`) after 17 000 steps, a tangle followed by a straight "highway" that runs off forever. The other panels are rules that never build a highway, after $2^25$ (33 million) steps: most grow into a roughly round blob whose texture looks random up close (bottom right), but the shapes differ (notched, ragged, or, for `RLLR`, a square frame) and one (`RRLRLR`#sym.dots.h`LLL`) barely grows at all, staying near its starting point. Rule letters give the turn on each colour.],
) <fig:gallery>

Here we make that measurement. The natural comparison is the rotor walker, in which every site holds an arrow that turns by 90° each visit and the walker follows it. For the rotor, radius $tilde t^(1/3)$ was conjectured by Priezzhev et al. @priezzhev1996, supported numerically by Kapri and Dhar @kapri2009, and proved by Bou-Rabee and Peres @bourabee2026, in an arXiv preprint that we understand has not yet been refereed, for uniformly random initial arrows (range of order $t^(2/3)$, convex limit shape). The ant is not an Eulerian walker, so nothing implies it should match. We find that in the bulk it does: the exponents and the whole time-averaged occupation profile agree. At the edge it does not: the ants' surface width grows with roughly twice the temporal exponent, and we cannot tell whether that is a different universality class or a slow transient. We also find that the quantity that sets the growth prefactor ($kappa$ below) is a moderately reproducible property of each rule that no feature of the rule string predicts. These are the open problems; the calibrated census and the decomposition of the growth rate below are the tools that make them well posed.

*Contributions.*
+ A census of #N.census.rules rules with growth exponents for #N.probe.analysed of them, reporting the whole distribution, its dependence on rule length, and the rules that fall outside it.
+ A calibration: the same pipeline applied to the rotor walker (known exponent) and a random walk (negative control), giving the estimator's bias and between-seed noise.
+ A three-factor decomposition of the area growth rate, with the factors measured per rule, and a classification of deviating rules by which factor drifts.
+ A future-window forecast from early transport measurements, tested across replicated starts and against rotor and random-walk controls.
+ A held-out predictive model for the edge-entry probability from the rule's letters, and a test of the "well-mixed" assumption behind the second factor.

@sec:factors and @sec:tested separate what is an identity from what is a test, and list what we could not explain. A hypothesis about "homes" that tether some rules to their origin was investigated and is mentioned only in the limitations; we make no claim about it.

= Model and methods

== Rules, census and classes

#figure(
  image("fig/mechanics.pdf", width: 100%),
  caption: [The first steps of two ants. The ant (black triangle) looks at the colour of the cell it stands on, turns left or right as the rule's letter for that colour says, changes the cell to the next colour (cyclically), and steps forward. `RL` has two colours, so a cell flips between blank and green; `RLL` has three (blank, green, orange).],
) <fig:mech>

A rule is a string $r in {L,R}^n$. The ant has a position, a heading and a grid of colours in $ZZ_n$, all initially 0. At each step it reads the colour $c$ under it, turns left or right by `r[c]`, increments the colour to $(c+1) mod n$, and moves one cell. Swapping every L and R gives the mirror-image ant, so we keep only rules starting with R, and drop the trivial rules in which every colour turns the same way.

A sweep of all such rules of length 2–12 (4083 rules) ran each on a blank $8192^2$ grid for up to $10^8$ steps with a highway detector (`experiments/2026-10-13-census-sweeps/results/sweep_v2.csv`); 578 escaped or were detected as highway builders (including classic `RL`). The census studied here is the remaining #N.census.rules rules of length #N.census.n_min–#N.census.n_max (3505 that ran the full budget plus 7 that escaped late).

Each census rule was then run to $2^27$ steps (`experiments/2026-10-13-probe-growth-census/results/growth.csv`, `experiments/2026-10-13-census-sweeps/scripts/census.py`). A rule is called a _builder_ (#N.census.builders rules) if it did not complete $2^27$ steps inside the grid or its steps per bounding-box cell at $10^8$ steps were below 20; builders are excluded.

Most are not highway builders: the recognised highways were already removed above, only 5 of the 304 have a highway period in `experiments/2026-10-13-census-sweeps/results/sweep_v2.csv` (none confirmed), 289 stay in the grid and fill it slowly (about 2--6 steps per box cell, frame-like structures) and 10 left the grid (`experiments/2026-10-13-b4-unfitted-rules/results/b4_builders.txt`). We did not check that the 289 form one class.

Of the rest, a rule is _log-growing_ (#N.census.log rules) if the fitted width exponent from $t=2^21$ to $2^27$ is below 0.15, and otherwise _blob_ (#N.census.blob rules). The cut at 0.15 is chosen by eye from the gap in the exponent histogram and has not been tuned against any outcome.

#figure(
  image("fig/census.pdf", width: 80%),
  caption: [Outcome of every rule in the sweep (4083 rules of length 2–12; the single length-2 rule is `RL` and is not shown), by rule length. Highway builders are a third to two fifths of the length-3 and length-4 rules and about one in seven from length 6 on. "Frame" builders (mostly rules that fill a frame slowly, a few that leave the grid) are a small slice. The two top classes, blob and log-growing, are the subject of this paper.],
) <fig:census>

== Instrumented runs

The probe (`src/probe.rs`) tracks the _visited_ set (every cell the ant has ever stood on), its area $A$, its _boundary_ $P$ (visited cells with at least one unvisited 4-neighbour), and, in octave windows $[2^k, 2^(k+1))$, the number of steps that start on a boundary cell and the number of steps that enter a never-visited cell. It also gives the bounding box, the radius of gyration $R_g$ of the visited set, and the ant's own spread. For each rule we fit log–log slopes over windows $k = 20 ... 26$:

- width exponent $alpha$ (bounding-box side against $t$) and $R_g$ exponent $alpha_g$;
- area exponent $a$ ($A tilde t^a$);
- boundary exponent $beta$ ($P tilde A^beta$).

#N.probe.analysed rules were analysed; #N.probe.unanalysed of the #(N.census.blob + N.census.log) growing rules had no usable window set (a window contained no new cells) and were not fitted, and #N.probe.bursty were flagged as bursty and removed from the statistics below (the flag is a column of `experiments/2026-10-13-probe-growth-census/results/probe_summary.csv` computed from the largest single-window jump, `max_jump`, and its share of growth, `max_share`; the script that set it was not preserved in the repository, but the flag is reproduced exactly by a threshold on `max_jump`: every flagged rule has `max_jump` $>=$ #f(N.bursty_rule.flagged_min_jump, d: 2) and every unflagged rule $<=$ #f(N.bursty_rule.unflagged_max_jump, d: 2), and including the flagged rules moves the blob median width exponent from #f(N.alpha_w_blob.at(0)) to #f(N.with_bursty.alpha_w.at(0))).

The unfitted rules are #pct(N.probe.unanalysed / (N.census.blob + N.census.log)) of growing rules, and they are not fast growers: rerun on a $32768^2$ grid, none left the grid, all reached $2^26$ steps, and their median area there is 1936 against 316162 for the fitted blob rules, so they are stalled or tethered (`experiments/2026-10-13-b4-unfitted-rules/results/b4_stats.txt`). If fast growers are missing from the census it is for another reason; the only candidates are the 10 grid-escapers among the builders (0.3% of the census).

The census contains #N.repeats.n rules that are exact repeats of a shorter rule ($u^k$, which behave identically to $u$); we keep them, since removing them moves the blob median width exponent only from #f(N.repeats.blob_median_alpha_w_all, d: 4) to #f(N.repeats.blob_median_alpha_w_primitive, d: 4).

== The three-factor decomposition

Per window of $S$ steps let $B$ be the number of steps that start on a boundary cell and $Delta A$ the number that enter new ground. Define

$ c = (Delta A) / B, quad kappa = (B slash S) / (P slash A), quad (d A) / (d t) = c dot kappa dot P / A. $ <eq:rate>

The last equality is an _identity_ given these definitions. Hence if $c$ and $kappa$ drift as $t^(gamma_c)$ and $t^(gamma_kappa)$ and $P tilde A^beta$, then

$ a = (1 + gamma_c + gamma_kappa) / (2 - beta), quad alpha approx a / 2. $ <eq:a>

@eq:a is also algebra, so agreement between the measured $a$ and @eq:a (median error #f(N.identity.median_abs_err_blob) over blob rules) checks the fits for consistency only. The empirical content is in the _values_ of the three factors: $kappa$ is the fraction of time on the boundary relative to a uniform spread over the territory, $c$ the chance a boundary step breaks new ground. If $gamma_c = gamma_kappa = 0$ and $beta = 1/2$, @eq:a gives $a = 2/3$.

#figure(
  image("fig/edge.pdf", width: 100%),
  caption: [The quantities in @eq:rate on a real territory (`RLL`, $2^16$ steps). Grey: cells visited so far, area $A$. Black: the _edge_, visited cells with at least one never-visited neighbour, size $P$. Only a step taken from an edge cell can enter new ground; $c$ is the fraction of edge steps that do, and $kappa$ compares the share of time the ant spends on the edge with the share of the territory that is edge. The right panel zooms on the boxed stretch of coastline.],
) <fig:edge>

== Reproducibility

All simulation is deterministic and seeded (rules start from a blank board, so a rule has one trajectory; seeds enter only in the walkers and the patched-start control). The Rust fast ant used for sweeps is tested step for step against an oracle (`tests/ant.rs`, `tests/probe.rs`). Every number in this paper is computed by `research-outputs/cube-root-growth-paper/compute_numbers.py` from the files in `data/` and written to `research-outputs/cube-root-growth-paper/numbers.json`, which this document reads; figures are made by `research-outputs/cube-root-growth-paper/figures.py`. Bootstrap intervals use a fixed seed. Commands are in `research-outputs/cube-root-growth-paper/README.md`.

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
  caption: [Same pipeline on three systems. The rotor has known exponents; the random walk is a negative control (area exponent near 1, boundary exponent near 1). The "ants (core)" rows are selected on $0.30 < alpha < 0.37$, which truncates their sd: it understates the between-rule spread and flatters the comparison with the rotor's sd. @fig:exp, over all blob rules, is the fair comparison.],
) <tab:calib>

Findings.

- (i) The pipeline recovers the rotor: the radius-of-gyration exponent is #f(N.calib_A.rotor.alpha_rg.median) (sd #f(N.calib_A.rotor.alpha_rg.sd)) against the true 0.333, the width exponent is slightly low (#f(N.calib_A.rotor.alpha_w.median)) and noisier, and the area exponent overshoots by #f(N.calib_A.rotor.a.median - 2/3, d: 2). We therefore treat the finite-window bias of $a$ as $+0.02$ and prefer the $R_g$ estimator, which is the least noisy.
- (ii) The random walk is clearly separated (area exponent #f(N.calib_A.random_walk.a.median, d: 2)).
- (iii) The ants' $R_g$ exponent #f(N.calib_A.ants_core.alpha_rg.median) and area exponent #f(N.calib_A.ants_core.a.median, d: 2) sit within the rotor's bias and sd.
- (iv) The boundary exponent of both is about #f(N.calib_A.rotor.beta.median, d: 3) (rotor) and #f(N.calib_A.ants_core.beta.median, d: 3) (ants), not 1/2; it is a finite-window effect shared with the rotor and not evidence of a rough edge.

To check the pipeline further we reproduced the published rotor measurements of Kapri and Dhar @kapri2009 from #N.calib_B.seeds_long seeds (radius exponent local slopes #N.calib_B.nu_local.map(x => str(x)).join(", "), tending to $1/3$; boundary-width exponent local slopes falling from #N.calib_B.delta_local.first() to #N.calib_B.delta_local.last(), against their reported value $alpha slash 3 approx 0.13$).

A reservation: for the rotor, the true limit exponent is known, but only by proof for the radius; our statements about its $beta$ and $c$ are measurements, not theorems.

= Results: the census <sec:census-results>

#figure(
  image("fig/growth_curves.pdf", width: 100%),
  caption: [How the territory grows with the number of steps $t$ (log scales, so a power law is a straight line). Three ant rules (blue and purple) and the rotor walker (red) climb parallel to the dashed guide lines, slope $1/3$ for the width and $2/3$ for the visited area; doubling the width takes about eight times as many steps. The log-growing rule `RRLRLR`#sym.dots.h`LLL` (teal) falls away, and the random walk (grey), the negative control, grows much faster, with width $tilde t^(1/2)$.],
) <fig:growth>

#figure(
  image("fig/timelapse.pdf", width: 100%),
  caption: [One rule (`RLL`) at four times, each panel rescaled to the same size. Eight times as many steps buys a territory only twice as wide ($8^(1/3) = 2$), and the shape stays the same round blob: it looks self-similar, and the roundness statistics of @sec:ens (anisotropy 0.05--0.07) agree, but we did not test scaling beyond that.],
) <fig:timelapse>

#figure(
  image("fig/exponents.pdf", width: 100%),
  caption: [Fitted exponents of #N.probe.blob blob-class ant rules and #N.calib_A.rotor.alpha_w.n rotor seeds. Dashed line: $1/3$. The ants' distribution is wider than the rotor's: sd ratio #f(N.calib_A.sd_ratio_alpha_w, d: 1) (width), #f(N.calib_A.sd_ratio_alpha_rg, d: 1) ($R_g$).],
) <fig:exp>

*Exponent.* Over the #N.probe.blob blob rules the median width exponent is #f(N.alpha_w_blob.at(0)) (rule bootstrap 95% interval #f(N.alpha_w_blob.at(1)) to #f(N.alpha_w_blob.at(2))), the $R_g$ exponent #f(N.alpha_rg_core.at(0)), and the area exponent #f(N.a_core.at(0)) in the core.

The interquartile range of the width exponent is #iqr(N.iqr_alpha_w_blob). The bootstrap resamples _rules_; it does not include the finite-window bias, which @tab:calib puts at about $+0.02$ for $a$ and negative for $alpha$.

Against the rotor in the same window the differences (ants minus rotor, medians over all blob rules) are #f(N.bias.alpha_w.ant_minus_rotor, d: 3) for the width exponent, #f(N.bias.alpha_rg.ant_minus_rotor, d: 3) for the $R_g$ exponent and #f(N.bias.a.ant_minus_rotor, d: 3) for the area exponent.

Subtracting the rotor's own bias from the ants' medians gives #f(N.bias.alpha_w.ant_corrected, d: 3) (width), #f(N.bias.alpha_rg.ant_corrected, d: 3) ($R_g$) and #f(N.bias.a.ant_corrected, d: 3) (area), against $1/3$, $1/3$ and $2/3$. The corrections point in different directions, so the data are consistent with $1/3$ at roughly the $plus.minus 0.01$ level, not "equal to $1/3$ to three digits"; the correction assumes the ants share the rotor's finite-window bias, which we cannot check.

*Rule length.* The median is flat in $n$: Spearman $rho$ between $alpha$ and $n$ is #f(N.spearman_alpha_n) over blob rules, and per-length medians for $n = 7 ... 12$ lie within about 0.01 of one another (table in `research-outputs/cube-root-growth-paper/numbers.json`, `by_n`). The spread between rules does not shrink with $n$ in our sample, so we do not claim a rule-length trend or its absence at larger $n$.

*Long runs.* Six typical rules were run to $2^32$ steps; their width exponents fitted from $2^17$ are between #f(N.cube.long_alpha.at(0)) and #f(N.cube.long_alpha.at(1)), and neither factor in the decomposition drifts (@fig:ck).

*Noise.* The between-rule scatter is mostly run-to-run: re-running #N.calib_C.rules rules from #N.calib_C.runs random $5 times 5$ starting patches gives within-rule sd #f(N.calib_C.alpha_w.sd_within) for $alpha$ and a between-rule true sd of only #f(N.calib_C.alpha_w.sd_between_true) (intraclass correlation #f(N.calib_C.alpha_w.icc, d: 2)).

Genuine rule-to-rule differences in $alpha$ are small; for $R_g$ and $a$ they are detectable (ICC #f(N.calib_C.alpha_rg.icc, d: 2), #f(N.calib_C.a.icc, d: 2)). We cannot say whether they reflect different exponents or different rates of convergence.

Where the rule-specific part of $a$ sits can be partly located (`experiments/2026-10-13-h1-lacunarity-decomposition/results/h1_lacunarity.txt`, `experiments/2026-10-13-h3-scaling-collapse/results/h3_collapse.txt`; the 99 random rules of the ICC above, window $k = 20$–$26$). The area is the product of the two box sides and a filling fraction, so $a = alpha_x + alpha_y + gamma_eta$ exactly, with $gamma_eta$ the log-slope of $A slash (W_x W_y)$. The rule-to-rule variance of $a$ is carried #pct(N.h1.share_ax) by $alpha_x$, #pct(N.h1.share_ay) by $alpha_y$ and #pct(N.h1.share_geta) by $gamma_eta$ (covariances of rule means, which include run noise); the two sides rise or fall together (correlation #f(N.h1.corr_axay, d: 1)), while $gamma_eta$ has ICC #f(N.h1.icc_geta, d: 2), like the sides taken singly (#f(N.h1.icc_ay, d: 2) and #f(N.h1.icc_ax, d: 2); $a$ itself #f(N.h1.icc_a, d: 2)). The territories consolidate slowly ($gamma_eta = +$#f(N.h1.mean_geta, d: 3), positive for #pct(N.h1.frac_geta_pos) of rules), but rules do not differ in $a$ mainly because they consolidate at different rates. We have not explained why the sides co-vary by rule.

= Results: the three factors <sec:factors>

For the #N.probe.core core rules (those with $0.30 < alpha < 0.37$), the medians (and quartile ranges) of the factors are $c = #f(N.mf_core.c.at(1), d: 2)$ #iqr(N.mf_core.c), $kappa = #f(N.mf_core.kappa.at(1), d: 2)$ #iqr(N.mf_core.kappa) and $beta = #f(N.mf_core.beta.at(1), d: 3)$ #iqr(N.mf_core.beta, d: 3), with territory fill (area over bounding box) #f(N.mf_core.fill.at(1), d: 2) #iqr(N.mf_core.fill).

The drift exponents are small, but the 0.05 threshold is loose: through @eq:a a drift of 0.05 shifts $a$ by about 0.033 and $alpha$ by about 0.017, as much as the rule-to-rule differences we are trying to explain. These fractions therefore say that $c$ and $kappa$ drift by less than the effect size of interest, not that they are exactly constant, and only the six $2^32$ runs probe much smaller drifts. $|gamma_c| < 0.05$ for #pct(N.identity.frac_blob_gc_small) and $|gamma_kappa| < 0.05$ for #pct(N.identity.frac_blob_gk_small) of all #N.probe.blob blob rules (the fractions are computed on the whole class, because "core" is itself defined by a width exponent near 1/3 and would make small drifts nearly automatic).

@fig:ck shows the long runs: both factors plateau from $2^17$ to $2^32$ for six rules. These are the observations that the cube root rests on; the predictions P1–P4 were written down before the analysis (`notes/cube-root-hypothesis.md`, dated 2026-10-10, three days before this draft) and could have failed; P4 predicted $beta$ close to $1/2$, and the measured $beta$ is #f(N.mf_core.beta.at(1), d: 3) (core median), equal to the rotor's #f(N.calib_A.rotor.beta.median, d: 3) in the same window (@tab:calib).

The medians quoted here are for the core class, which is selected on the outcome, and are descriptive only.

#figure(
  image("fig/ck_time.pdf", width: 100%),
  caption: [Edge-entry probability $c$ and edge-occupancy ratio $kappa$ against $log_2 t$ for six rules run to $2^32$. Both are flat after $2^20$; $kappa$ fluctuates by roughly ten percent from octave to octave.],
) <fig:ck>

*Comparison with the rotor walker.* For the rotor the same decomposition gives $c = #f(N.rotor_ck.c.at(0), d: 2)$ and $kappa = #f(N.rotor_ck.kappa.at(0), d: 1)$; a random walk gives $c = #f(N.rw_ck.c.at(0), d: 2)$. So $c$ alone does not discriminate (the random walk's $c$ is close to the ants'), and the ants' $kappa$ (about #f(N.mf_core.kappa.at(1), d: 1)) is about half the rotor's.

The cube root does not require the same $c$ and $kappa$ as the rotor; it requires that they be constant and that the edge scale as $A^(1/2)$. The ant's edge occupation is _lower_ than uniform ($kappa < 1$ for most rules); we have no explanation of the value of $kappa$.

*Is $kappa$ a property of the rule?* The claim that $kappa$ differs between rules moderately reproducibly, yet is not carried by the letters, rests on three measurements, drawn from four data files; the between-rule fractions are intraclass correlations as in the Noise paragraph.

- (i) _Moderately reproducible_ (`experiments/2026-10-13-a1-kappa/results/a1b_icc.txt`; the within-rule sd is nearly as large as the between-rule sd): 99 random rules, each run from 12 random $5 times 5$ patches, have a between-rule sd of $kappa$ of 0.44 against a within-rule sd of 0.37 in the core band (ICC 0.58; 0.63 for $log kappa$; 58 further core rules give 0.50 and 0.63). The width exponent on this design (`experiments/2026-10-13-b2-between-rule-variance/results/b2_probe.csv`) has ICC 0.12 (0.065 in the Noise paragraph, a different set of rules and starts), so $kappa$ is much more rule-specific than the exponent it controls.
- (ii) _Not carried by the letters_ (`experiments/2026-10-13-a1-kappa/results/a1_kappa_proxy.txt`, `experiments/2026-10-13-a1-kappa/results/a1c_predict.txt`): a linear regression on string features (turn probability $q$, $n$, run count, letters 1--6), fitted on a random half of the #N.probe.blob non-bursty blob rules and scored on the other half, has held-out $R^2$ = 0.003 for $kappa$ and 0.002 for its drift $gamma_kappa$, against 0.78 for $c$ with the same features and split (a different, simpler model than the chirality model of the next section, which reaches #f(N.followup.c_model.r2, d: 2) on a larger held-out set); Spearman($kappa$, $q$) = 0.08. On 192 rules with at least six core-band runs, leave-one-rule-out $R^2$ for the rule-mean $log kappa$ is $-0.06$ to $-0.04$ for ridge on letter-by-position, about 0 for ridge on 2- and 3-letter substring counts, and 0.04 for a prefix-5 neighbour mean.
- (iii) _Not carried by the edge colour mix_ (`experiments/2026-10-13-a1-kappa/results/a1_edge_q.txt`): on 40 random core rules the edge-weighted turn probability correlates with $c$ at Spearman $-0.84$ (a positive control that the instrument can see rule dependence) but with $kappa$ at only $-0.27$, and leave-one-out $R^2$ of $kappa$ on it and the edge-colour distance from uniform is $-0.05$.

So $kappa$ behaves as an emergent property of each rule's dynamics.

*Do interior excursions explain boundary occupation?* Excursion tomography did not identify a size-independent explanation of $kappa$ (`notes/excursion-tomography-2026-10-13.md`; analysis and provenance in `experiments/2026-10-14-t1-excursion-tomography/`). Apparent associations involving return displacement divided by territory radius and return angle were largely attributable to territory size. Early excursion features did not improve held-out prediction of later $log kappa$ beyond an area baseline. Extreme completed-excursion duration relative to territory radius remains associated with $kappa$, but the ratio retains a size confound, its binned quantile has coarse resolution, and window-crossing excursions are censored. The rotor control missed its registered tolerance and final-window random-walk controls were unavailable. These measurements narrow the candidate explanations without establishing a mechanism; matched-area comparisons or interventions are needed.

*Does early transport predict later growth?* A same-window identity cannot test persistence. We therefore specified a forecast on existing replicated runs before computing its errors (`notes/transport-forecast-2026-10-13.md`). Write $g=P/sqrt(A)$; formally @eq:rate gives $(d (A^(3/2)))/(d t)=(3/2)c kappa g$. For an octave ending at $t$, with duration $S=t/2$, the exact discrete budget is

$ Q = (A^(3/2) - (A-Delta A)^(3/2)) / S. $ <eq:budget>

The endpoint product $(3/2)c kappa g=(3/2)(Delta A/S)sqrt(A)$ is a right-end approximation, typically #f(TF.ants.rule_medians.endpoint_over_exact, d: 3) times the exact budget in this cohort. We forecast with @eq:budget. Each trajectory's mean budget over windows ending at $2^20$, $2^21$ and $2^22$ predicts its increments in $A^(3/2)$ over the next four octaves, ending at $2^23$–$2^26$, without updating from those outcomes. A competing model extrapolates a log–log drift fitted to the same three early budgets. The rule-independent baseline is the median early rule-mean budget after excluding the target rule and all its seeds.

There are #TF.ants.runs complete trajectories across #TF.ants.rules rules, with approximately 12 patched starts per rule; #TF.ants.excluded.len() trajectories fail the completeness, nonescape or positive-growth criteria. We apply no outcome exponent filter. For each trajectory, error is the median absolute relative increment error over four forecast windows; the summary is the median of equal-weight rule means. Constant budgets give #f(100 * TF.ants.rule_medians.constant, d: 1)% error against #f(100 * TF.ants.rule_medians.baseline, d: 1)% for the baseline and #f(100 * TF.ants.rule_medians.drift, d: 1)% for the drift fit. The reduction is #f(100 * TF.ants.reduction, d: 1)% (rule-bootstrap 95% interval #f(100 * TF.ants.reduction_ci.at(0), d: 1)–#f(100 * TF.ants.reduction_ci.at(1), d: 1)%; 2000 resamples, seed 20261013), passing the stated 30% prediction. Integrating all four forecast increments from the observed area at $2^22$ gives final-area error #f(100 * TF.ants.rule_medians.constant_final, d: 1)% against #f(100 * TF.ants.rule_medians.baseline_final, d: 1)% for the baseline.

Calibration separates persistence from a universal forecasting advantage. The same estimator on #TF.rotor.runs rotor seeds, with matched windows, gives #f(100 * TF.rotor.rule_medians.constant, d: 1)% error against #f(100 * TF.rotor.rule_medians.baseline, d: 1)% for the other-seed baseline. On #TF.random_walk.runs random-walk seeds (training $k=17$–$19$, forecast $k=20$–$23$), errors are #f(100 * TF.random_walk.rule_medians.constant, d: 1)% and #f(100 * TF.random_walk.rule_medians.baseline, d: 1)%; drift extrapolation still misses by #f(100 * TF.random_walk.rule_medians.drift, d: 1)%. Synthetic $A=C t^(2/3)$ and $A=C t$ recover their known constant and square-root budgets respectively.

This typical accuracy hides severe failures. RMS within-rule seed sd of relative increment errors is #f(TF.ants.rms_seed_sd.constant, d: 2) for constant budgets, #f(TF.ants.rms_seed_sd.drift, d: 2) for drift and #f(TF.ants.rms_seed_sd.baseline, d: 2) for the baseline; these are ratios, not percentages. One retained trajectory (`RLLRRRLLRLRR@108`) has constant-budget error about 6506 times the observed increment. Near-stalls make relative errors enormous, so the robust improvement does not imply uniformly reliable predictions. Escaped or incomplete trajectories are excluded, and this test cannot forecast their breakouts. An exploratory check, specified after the primary result, uses the median early budget of the other seeds of the same rule and gives #f(100 * TF.ants.rule_medians.other_seeds, d: 1)% error, slightly better than each trajectory's own early budget. This supports persistent rule-level transport in the measured windows; it neither identifies the spatial-memory mechanism nor proves asymptotic scaling. Exact recipes, input hashes and all trajectory errors are in `experiments/2026-10-13-transport-forecast/scripts/transport_forecast.py` and `experiments/2026-10-13-transport-forecast/results/transport_forecast.json`.

*A memoryless control.* We then tested the mechanistic candidate directly: a memoryless walker that turns right with probability $Q$ and left otherwise, independent of the board, driven through the same probe (`experiments/2026-10-13-a1-kappa/results/a1d_stats.txt`; $Q in {0.25, 0.4, 0.5, 0.6, 0.75}$, six seeds each, $2^26$ steps on a $32768^2$ board; predictions written beforehand in `notes/frontier-2026-10-13-results.md`, A1d). Its $kappa$ is roughly constant at 1.3 to 1.45 (within $plus.minus 5%$ across $Q$ from 0.25 to 0.6), but its area exponent is 0.94 to 0.96, not $2/3$.

So independent persistent turns reproduce roughly uniform occupancy but neither the cube root nor the ants' $kappa$ (about 0.9) nor the rotor's (about 2.0). This rejects the independent-turn mechanism; it motivates testing board memory without establishing that memory alone is sufficient, or that every memoryless mechanism must fail.

This rules out one explanation of $kappa$; it does not supply another.

*What does stochastic board memory change?* @fig:memory-paths makes three comparisons concrete. A once-excited walk uses a directional cookie on its first departure from each site, then moves uniformly on revisits @benjamini2003. Randomly orienting those cookies removes the ensemble drift. A linearly edge-reinforced walk instead favours edges it has crossed: each undirected edge starts at weight 1 and gains $I$ on each crossing. Neighbour choices are proportional to those weights @merkl2006. These models separate one-use site memory from persistent edge memory; the random-arrow cookie is our specified variant of the directional model.

#figure(
  image("fig/memory_paths.pdf", width: 100%),
  caption: [Nine walkers after the same 65,536 steps. Stochastic walkers use the preselected seed 301; the ant starts blank. Colour records position samples per site (including the origin); orange traces the final 1,024 steps and + marks the origin. Panels have equal x/y scaling except the directional-cookie panels, whose y-axis is expanded to show transverse motion. Spatial scales differ: read the cell coordinates. Directional cookies spread east, random cookies resemble the random walk, and strong reinforcement repeatedly uses a small territory. These single trajectories illustrate behaviour, not the ensemble exponents. $B$ is the first-departure probability of following the cookie instead of choosing a uniform neighbour; $I$ is the edge-weight increment.],
) <fig:memory-paths>

The registered pilot used eight seeds (301–308) per setting, up to $2^20$ steps on an $8192^2$ board. Table @tab:memory reports per-seed fits over $k=14$–$18$ and $kappa$ at $2^18$; uncertainties are between-seed sd. All 16 directional-cookie runs escaped before completing that window, so we report no fitted exponent for them. Random-arrow cookies stayed close to the random-walk control. Weak edge reinforcement had a mean area exponent near $2/3$, but its large scatter and falling $kappa$ do not establish the ant regime. Strong reinforcement greatly reduced exploration.

#figure(
  table(columns: (2.5fr, 1fr, 1fr), align: (left, center, center),
    table.header([Walker], [Area slope $a$], [$kappa$ at $2^18$]),
    [Random walk], [0.924 ± 0.043], [1.350 ± 0.082],
    [Rotor], [0.740 ± 0.027], [1.845 ± 0.118],
    [Random cookie, $B=0.25$], [0.942 ± 0.047], [1.327 ± 0.075],
    [Random cookie, $B=0.75$], [0.920 ± 0.044], [1.293 ± 0.094],
    [Edge reinforcement, $I=1$], [0.698 ± 0.303], [0.312 ± 0.132],
    [Edge reinforcement, $I=4$], [0.187 ± 0.092], [0.328 ± 0.447],
  ),
  caption: [Stochastic memory pilot, eight complete seeds per row. The early rotor area slope overshoots $2/3$ by 0.074, so this window cannot identify an asymptotic exponent. In the later $k=16$–$20$ window, the rotor gives 0.715 ± 0.019, weak edge reinforcement 0.701 ± 0.277, and strong reinforcement 0.216 ± 0.096. Weak reinforcement's mean $kappa$ falls to 0.155 ± 0.084 at $2^20$.],
) <tab:memory>

#figure(
  image("fig/memory_growth.pdf", width: 100%),
  caption: [Growth and boundary occupancy across the same pilot. Thin lines are individual seeds; thick lines show the geometric mean area and arithmetic mean $kappa$. All solid curves have eight seeds. Dashed directional-cookie curves end when runs escape; terminal labels give the number still contributing, and later points describe survivors only. The grey guides show area proportional to $t^(2/3)$ and $kappa=1$. Persistent memory can suppress exploration without producing stable ant-like boundary occupancy.],
) <fig:memory-growth>

The prediction that random cookies at $B=0.75$ would give $a$ between 0.60 and 0.75 failed. Weak reinforcement passed that area band but failed the predicted $kappa$ band of 0.5–3. Thus none of these settings reproduced both features of the target regime. This is a finite-window negative result, not a general exclusion of stochastic memory. Definitions, predictions, recipes and all per-seed fits are in `notes/walker-comparisons-2026-10-13.md` and `experiments/2026-10-13-memory-walkers/results/memory_walkers_pilot/summary.json`.

We also measured where the walker stands relative to the edge (`examples/layers.rs`; design and predictions in `notes/kappa-experiment-design.md`, written beforehand). Let layer $d$ be the Manhattan distance to the nearest never-visited cell minus one, so layer 0 is the edge, and let $rho(d)$ be the visit rate per cell at layer $d$ relative to the territory mean ($rho(0) = kappa$ by construction, which we checked: the layer-0 count equals the probe's perimeter in every row). For the rotor (12 seeds, $k = 22 ... 25$) $rho$ is 1.8 at layer 0, 1.1 at layer 1 and 0.9--1.0 deeper; for the memoryless walker it falls smoothly from 1.4 to 0.8 over four layers. For the ants it does not have a single shape. In 24 ant rules (12 chosen as the lowest and highest rule-mean $kappa$ and 12 closest to 0.9, blank board, `experiments/2026-10-13-layers-colours/results/layers_stats.txt`, `experiments/2026-10-13-layers-colours/results/layers_stats_typical.txt`, `experiments/2026-10-13-layers-colours/results/layers_classes.txt`) we find two classes. In 15, a _crust_: layers 2 to 4 are almost never visited ($rho$ between 0.01 and 0.6), the walker concentrates in layers 0 and 1, and for 13 of them $c$ is 0.61--0.66 and the mean number of edge-phase visits per cell is 1.5 to 1.6 (the other two have $c$ 0.44 and 0.48), yet $kappa$ ranges from 0.13 to 18. In 9, the occupation is uniform to within 30% at layers 2 to 4, $c$ is 0.27--0.48, and $kappa$ lies between 0.82 and 0.96 (a range truncated by how those rules were chosen). A proxy for the crust class in the census, $c >= 0.6$, picks out 12% of blob rules (319 of 2622, including a spike of 263 with $c$ between 0.65 and 0.70) whose median $kappa$ (0.80) is ordinary. Two predictions made beforehand failed: that the $kappa$ difference between rules lives in the first two layers (it did not, for 15 of 24), and that $kappa$ follows the edge dwell time (Spearman $-0.26$ for the 12 extreme rules, $-0.59$ for the 12 typical, the wrong sign). The two-class split was found after looking and is exploratory. Its consequence is that $kappa$ is not fixed by the edge-entry statistics: within the crust class $c$ and the dwell are constant while $kappa$ spans more than an order of magnitude. This characterises the unexplained quantity; it does not explain it. A second intervention gave a result that we did not predict. A rotor whose arrow is replaced by a random one with probability $epsilon$ per visit (`experiments/2026-10-13-arm-kappa-depletion-arms/results/armb_stats.txt`; $epsilon$ from 0 to 0.1, eight seeds each, 38 of 48 runs fitted, $k = 20 ... 26$) reproduces the rotor at $epsilon = 0$ (area exponent 0.69, $kappa = 2.0$), but already at $epsilon = 0.001$ the area exponent is 0.76 and by $epsilon = 0.03$ it is 0.94, earlier than we predicted. $kappa$ did not fall towards the memoryless value 1.4 as predicted; it rose to 3.6, 5.8 and 6.7 for $epsilon = 0.001, 0.003, 0.01$ and fell back at larger $epsilon$, with a positive drift in time, so these values are transient readings during the crossover. Meanwhile $c$ stayed at 0.498 throughout. Exact deterministic memory is therefore what holds $kappa$ at 2 in the rotor, and $kappa$ and $c$ vary independently, as in the ants' crust class. A third test used 26 pairs of rules with equal length, equal number of R and edge-entry probability within 0.02 but rule-mean $kappa$ differing by at least 0.5 (`experiments/2026-10-13-arm-kappa-depletion-arms/results/armc_stats.txt`): the depth of the depleted crust does not separate them (the lower-$kappa$ member has the deeper crust in 13 of 26 pairs, against the 18 we required). The blank-board $kappa$ orders 25 of the 26 pairs as the patched-start rule means do, so the rule dependence is robust to the kind of run. Five candidate predictors and one intervention have now failed to account for it.

*Deviating rules.* #N.probe.log rules grow logarithmically ($alpha$ median #f(N.cube.log.alpha.at(1))), with $kappa$ median #f(N.cube.log.kappa.at(1), d: 4) and $gamma_kappa = #f(N.cube.log.g_k.at(1), d: 2)$: the ant rarely visits the edge and the occupancy of the edge decays, consistent with an ant confined near its origin.

#N.probe.fast blob rules have $alpha > 0.37$ with $gamma_kappa$ median #f(N.cube.fast.g_k.at(1), d: 2) (edge occupation rises). Across blob rules the correlation of $alpha$ with $gamma_kappa$ is #f(N.identity.corr_alpha_gk, d: 2), with $gamma_c$ #f(N.identity.corr_alpha_gc, d: 2) and with $beta$ #f(N.identity.corr_alpha_beta, d: 2) (@fig:dev).

So the outliers are outliers because the ant's occupation of the edge drifts, which is the failure condition we wrote down in advance for the mechanism ("$kappa$ drifting strongly with $t$"). This is a restatement through @eq:a, not an explanation: _why_ $kappa$ drifts for these rules is unexplained here.

#figure(
  image("fig/deviations.pdf", width: 100%),
  caption: [Left: width exponent against the drift exponent of $kappa$. Right: area exponent against the value predicted by @eq:a; the agreement is by construction up to fit error (see text).],
) <fig:dev>

= Results: what sets $c$, and is the ant well mixed?

*Edge-entry probability.* Across rules $c$ ranges from about 0.1 to 0.67. A chirality model predicts it from the rule's letters (this is the model behind the abstract's figure; the 0.78 quoted in the previous section is a plain linear regression on string features, with different features, sample and split, so the two $R^2$ values are not comparable): the edge is chiral (for edge steps on colours 1 to 3, the share with fresh ground on the left exceeds that on the right in #N.followup.chiral.left_more of #N.followup.chiral.rules calibration rules; medians #f(N.followup.chiral.left_med, d: 2) against #f(N.followup.chiral.right_med, d: 2)), and so a rule that turns L on a young colour enters new ground more often.

Constants (the share of edge steps by colour and the left/right fresh-ground rates) are fitted on a 300-rule sample and applied to the others without refitting.

On #N.followup.c_model.n held-out rules the model attains $R^2 = #f(N.followup.c_model.r2, d: 2)$ (correlation #f(N.followup.c_model.corr, d: 2); median absolute error #f(N.followup.c_model.mae_med, d: 2)); grouping rules by their first letter after the leading R explains #f(N.followup.r2_letters.r1, d: 2) of the variance of $c$, by the next three letters jointly #f(N.followup.r2_letters.r123, d: 2), and by the sixth letter alone (rules of length at least 8) #f(N.followup.r2_letters.r6, d: 3).

The model was a post-hoc fit to these data, though evaluated on rules outside its calibration sample; we have not tested it on rules of length above 12.

#figure(
  image("fig/cmodel.pdf", width: 55%),
  caption: [Held-out edge-entry probability: model against measurement. The model takes few distinct values; the $R^2$ is mostly between-cluster, and within the clusters near 0.33 and 0.53 it discriminates poorly (rules with predicted $c approx 0.53$ have measured $c$ from about 0.25 to 0.67).],
) <fig:cm>

*Mixing.* The mean-field step assumes the ant is spread roughly evenly over its territory, behaving like a persistent random walk with turning probability $q = \#R slash n$. The sample is the #N.mix_sample.listed rules in `experiments/2026-10-13-mixing/results/mix_rules.txt`, of which #N.mixing.typ_n are blob rules with $0.30 < alpha < 0.37$; the procedure used to choose the list was not recorded, so it should be read as a convenience sample.

We therefore repeated the test on #N.mixing_random.n rules drawn uniformly at random (fixed seed) from the core class, with predictions written beforehand (`notes/mixing-random-sample.md`): #N.mixing_random.uni_n of #N.mixing_random.n have near-uniform interior colours (our prediction of 60–85% was exceeded), their MSD-slope ratio has median #f(N.mixing_random.uni_ratio.at(1), d: 2) (range #f(N.mixing_random.uni_ratio.at(0), d: 2) to #f(N.mixing_random.uni_ratio.at(2), d: 2)), and the remaining #N.mixing_random.non_n have median ratio #f(N.mixing_random.non_ratio_med, d: 2).

The convenience-sample numbers follow. For the #N.mixing.typ_n typical rules we tested (a) colour uniformity (total-variation distance of interior colours from uniform), (b) turn correlations, and (c) the slope of mean-square displacement against lag, which for independent turns is $(1 - a')/(1 + a')$ with $a' = (2q-1)^2$.

For #N.mixing.uni_n rules with near-uniform interior colours (TV $<$ 0.05) the measured slope is within [#f(N.mixing.uni_ratio.at(0), d: 2), #f(N.mixing.uni_ratio.at(2), d: 2)] of the prediction (median #f(N.mixing.uni_ratio.at(1), d: 2)), the displacement exponent is #f(N.mixing.uni_expo.at(1), d: 2), and the lag-1 turn correlation is #f(N.mixing.uni_corr1.at(1), d: 3) at the median.

For the other #N.mixing.non_n rules, whose interior colours are far from uniform, the MSD slope is off by a factor of #f(N.mixing.non_ratio.at(1), d: 2) at the median (correlation between the ratio and the colour deviation across the sample: #f(N.mixing.corr_ratio_tv, d: 2)).

Time to mix across the territory is a fraction #f(N.mixing.mix.at(1), d: 3) (median) of the growth time.

The ant is therefore well described as a random walker in a random environment for #pct(N.mixing.uni_n / N.mixing.typ_n) of the sampled rules and not for the remainder. That the ant is spread out in time does not by itself imply that its edge occupation is $kappa$; it only makes $kappa$ a stationary quantity, and @fig:ck shows that.

= Long ensembles, the occupation profile and the outliers <sec:ens>

Three further measurements, produced by separate scripts (`experiments/2026-10-13-item4-exponent-rotor-calibrated/scripts/item4_stats.py`, `experiments/2026-10-13-shape-limit/scripts/shape_stats.py`, `experiments/2026-10-13-item21-slow-outliers/scripts/item21_stats.py`; hypotheses and predictions are in `notes/frontier-results.md`, written before each run), bear on the weakest points above.

*Is the exponent universal?* Eleven rules (stratified by length 3–12) were run from #(N.item4.runs_kept) random $5 times 5$ starting patches, 24 seeds each, to $2^30$ steps. The local area exponent, averaged over seeds and over the last six three-octave windows, has mean #f(N.item4.ant_last6_mean, d: 3) across rules (sd #f(N.item4.ant_last6_sd, d: 3), range #f(N.item4.ant_range.at(0), d: 3) to #f(N.item4.ant_range.at(1), d: 3)).

The same estimator applied to 24 rotor seeds, whose exponent is proved to be $2/3$, gives #f(N.item4.rotor_last6, d: 3): even the rotor sits $0.01$ above $2/3$ at $2^30$ by this estimator. #N.item4.within_0006_of_rotor of the #N.item4.rules rules are within 0.006 of the rotor; the others are up to 0.02 above it or 0.017 below it by amounts many standard errors wide.

One rule (`RRLLLLLLR`) does not self-average: its seed-to-seed spread rises with time. So the data are compatible with a common exponent $2/3$ with slow, rule-dependent convergence, but they cannot distinguish that from rule-dependent exponents within about 0.02.

*The occupation profile.* The time-averaged visit density, as a function of distance from the centroid in units of the territory radius $R_("eq") = sqrt(A slash pi)$ and of $t slash A$, is a cone, $n(r) = 3 (1 - r slash R_("eq")) t slash A$, for ants and rotor alike. Over the first ten of fifteen radial shells the ants' profile (#N.shape.ants.runs runs of 11 rules, $2^#N.shape.ants.kmax$ steps) and the rotor's (#N.shape.rotor.runs seeds, $2^#N.shape.rotor.kmax$ steps) differ by at most #pct(N.shape.max_rel_diff_ants_rotor_shells0_9), and the ant profile differs from the cone by at most #pct(N.shape.max_rel_diff_ants_cone_shells0_9)\; the across-rule relative sd at mid-radius is #pct(N.shape.ants.profile_rel_sd_shell6). This is a prediction we had expected to fail (we had written that the ant's profile would differ from the rotor's). The agreement is nonetheless not independent evidence for the cube root, because the cone is implied by it.

Suppose the walker is spread uniformly over the current territory, $n(r, t') = 1 slash A(t')$ for $r < R(t')$, and $A(t) prop t^a$. A cell at radius $r$ is inside from the time $t_r$ at which $R(t_r) = r$, so its accumulated visits are $rho(r) = integral_(t_r)^t d t' slash A(t') = (t slash ((1-a) A)) (1 - (r slash R)^(2(1-a) slash a))$. For $a = 2/3$ this is exactly $(3 t slash A)(1 - r slash R)$, and it integrates to $t$ over the disc. The cone is therefore the footprint of a uniformly mixing walker inside a disc growing as $t^(2/3)$; nothing specific to rotors or ants enters beyond those two assumptions (`notes/frontier-2026-10-13-results.md`, A3). It also explains why ants and rotor agree although their $kappa$ differ (about 0.9 against 2.0): $kappa$ changes only a thin edge layer, not the bulk integral. What the profile adds is a check of uniform occupancy in the bulk.

With the measured rotor exponent $a = 0.687$ the formula gives a centre prefactor $1 slash (1-a) = 3.19$ rather than 3, against a measured 2.87 (`experiments/2026-10-13-shape-limit/results/shape_stats.txt`) that the 0.075 $R_("eq")$ shell width lowers by roughly 3--4%, so the data do not separate $a = 2/3$ from 0.687 at better than about 5%. The formula's predictions for other exponents were tested on walkers with $a != 2/3$ (`notes/frontier-2026-10-13-results.md`, W2, W3 and the checks after them; `experiments/2026-10-13-w3/results/w3_stats.txt`, `experiments/2026-10-13-w-width-radius-refit/results/w_checks.txt`), with results that do not support the derivation beyond $a = 2/3$. (i) Rotors whose arrow is randomised with probability $epsilon = 0.001$ ($a = 0.75$–$0.78$, four seeds, territories round enough for a radial profile: anisotropy $<= 0.31$, fill 0.96) have a centre density of 1.9–2.1 against 3.5–3.8 from the formula, so the formula fails and the cone, at 3, is also off. For $epsilon >= 0.003$ and for a memoryless persistent walker the territories are elongated or holey (anisotropy up to 0.9, fill down to 0.3), so the premise fails first and the profile cannot test the formula. (ii) Two chaotic 2-state turmites with a stable $a approx 0.74$ over six octaves (`tm:2:2:46382`, `7982`; single trajectories) keep a cone-like centre (2.8 and 3.1) below the formula's 3.5 and 3.2. So the cone is not simply a consequence of "uniform occupancy and an exponent", and we cannot say that uniform occupancy needs $a = 2/3$; every walker we tried has $a >= 2/3$, and the exponents are finite-window values. We had also meant to test the formula on the $a approx 1/2$ turmite cluster, but those tables grow as one-dimensional lines (anisotropy 1.00), and eight of the ten sampled are the same trajectory, so the cluster does not bear on the question.

The territories are round (median anisotropy #f(N.shape.ants.aniso, d: 3) at $2^28$, the rotor's #f(N.shape.rotor.aniso, d: 3) at $2^26$; fill #f(N.shape.ants.fill, d: 2)).

The surface width does _not_ match: its local exponent fitted over $k >= 18$ is #f(N.shape.ants.dW, d: 2) for ants and #f(N.shape.rotor.dW, d: 2) for the rotor (Kapri and Dhar: about 0.13), so the ants' edge is broader at equal size. These single fits use a different window ($k >= 18$) from the matched-$k$ follow-up below, which is why the absolute values differ; the comparison that matters is the ratio.

A longer follow-up (`experiments/2026-10-13-a4-boundary-width/results/a4c_stats.txt`; 36 ant runs and 14 rotor seeds to $2^34$) finds both exponents flat rather than falling, about 0.22–0.25 for ants and 0.10–0.13 for the rotor at every matched $k$ from 22 to 32, so the gap of roughly a factor two does not close in the accessible range; whether it closes asymptotically is undecided. The gap is not an artefact of the extreme-$kappa$ rules with a "crust" (an almost unvisited shell between the active edge and the bulk, found in 15 of 24 rules examined; see @sec:factors): to $2^30$ the 9 ordinary-$kappa$ rules without a crust have local exponents 0.21–0.35 (median 0.27), all above 0.18, against 0.17–0.45 (mean 0.32) for the 15 crust rules (difference +0.04, Mann–Whitney $p = 0.054$; Spearman with $log kappa$ +0.40, $p = 0.056$; one trajectory per rule, W1). The wider ant edge is thus generic to ants, with at most a mild extra roughening in the high-$kappa$ tail.

#figure(
  image("fig/rotor.pdf", width: 100%),
  caption: [Left and middle: how often each cell was visited (log scale) in a single run of $2^22$ steps, for the rotor walker (one seed) and the ant `RLL`. Right: visits per cell, averaged in rings around the centre, against the cone $3(1 - r slash R)$. Both curves follow the cone; this figure is one run each and is an illustration, not the ensemble measurement quoted in the text.],
) <fig:rotor>

*Width against radius.* Refitting the same A4/A4c trajectories directly against $R=sqrt(A slash pi)$ gives $W_("surf") prop R^zeta$ with rotor $zeta=0.401$ (14 seeds, sd 0.073; bootstrap mean interval 0.366–0.437) and ant $zeta=0.714$ (36 runs across six rules, RMS within-rule seed sd 0.134; rule-bootstrap interval 0.664–0.761), over $k=22$–$32$. The rotor prediction $zeta in [0.30,0.37]$ failed: this estimate is closer to $3/8$, implied by $W prop t^(1/8)$ and $R prop t^(1/3)$, than to $1/3$. Restricting to $k=26$–$32$ gives 0.389 and 0.685; the later A4c cohort alone over $k=26$–$34$ gives 0.395 and 0.637. The ant/rotor slope ratio therefore changes from 1.78 to 1.61. Replacing $R$ with $R_g$ or the mean outer radius changes these means by less than 0.005. This confirms a finite-window gap but does not support an exact factor-two law. These are growth-trajectory slopes, not spatial roughness exponents; height correlations and removal of global shape modes are needed to test a KPZ or other interface universality class (`experiments/2026-10-13-w-width-radius-refit/results/width_radius_refit.json`).

*Are the outliers noise?* We re-ran the #N.item21.high_n fastest and #N.item21.low_n slowest blob rules from 12 random patches each. If their exponents had been run-to-run noise they would regress to the middle. They mostly do not: only #pct(N.item21.high_regress) of the fast rules fall below 0.37 and #pct(N.item21.low_regress) of the slow rules rise above 0.30; the Spearman correlation between census and re-measured exponent is #f(N.item21.spearman_census, d: 2), and the re-measured exponent correlates with $gamma_kappa$ at #f(N.item21.spearman_gk, d: 2). So the outliers are real properties of the rules over $2^20$–$2^27$ steps, with edge occupation drifting; whether they are different asymptotes or very slow transients is untestable at this length (the previous paragraph shows even typical rules differ by about 0.02 at $2^30$).

This revises our earlier statement that most between-rule scatter is noise: for typical rules it is (within-rule sd 0.016 against a between-rule sd of 0.004), but the tails are not.

= What is tested and what is not <sec:tested>

#[
#set par(justify: false)
#table(
  columns: (0.9fr, 1.5fr, auto),
  align: (left, left, left),
  stroke: (x: none), inset: 4pt,
  [*Claim*], [*Status*], [*Evidence*],
  [Width $tilde t^(1/3)$, area $tilde t^(2/3)$ for non-highway rules], [Measured; consistent after the rotor-calibrated bias], [@tab:calib, @fig:exp],
  [$c$ constant in time], [Tested, not refuted], [@fig:ck, $gamma_c$],
  [$kappa$ constant for typical rules], [Tested, not refuted (drift below 0.05) for #pct(N.identity.frac_blob_gk_small) of blob rules; refuted for log and fast classes], [@fig:ck, @fig:dev],
  [$P tilde A^(1/2)$], [Not confirmed: measured $beta =#f(N.mf_core.beta.at(1), d: 3)$, equal to the rotor's under the same window], [@tab:calib],
  [@eq:a predicts $a$ for outliers], [Algebraic identity; not a test of mechanism], [@fig:dev (right)],
  [Edge-entry probability $c$ from rule letters (chirality model)],[Held-out $R^2 = #f(N.followup.c_model.r2, d: 2)$; length $<= 12$ only], [@fig:cm],
  [Well-mixed ant], [True for #N.mixing.uni_n of #N.mixing.typ_n convenience-sampled rules and #N.mixing_random.uni_n of #N.mixing_random.n random core rules], [Mixing paragraph],
  [Occupation profile is the rotor's cone], [Measured; predicted to fail, did not (within #pct(N.shape.max_rel_diff_ants_rotor_shells0_9) of the rotor). Derived as a consequence of uniform occupancy and $A tilde t^(2/3)$, so not independent of the cube root. *Unexplained*: why the profile is the cone. The derivation from uniform occupancy plus the exponent fails for walkers with $a eq.not 2/3$ (noisy rotor, random-background ants), while two turmites with $a approx 0.74$ keep a cone-like centre], [@sec:ens],
  [One exponent for all rules], [Neither confirmed nor refuted: rules differ by up to 0.02 at $2^30$, as does the rotor from its proved value], [@sec:ens],
  [Outliers are noise], [Refuted for most: #pct(N.item21.high_regress) / #pct(N.item21.low_regress) regress], [@sec:ens],
  [Surface width matches the rotor], [Not supported: ants' exponent is about twice the rotor's (#f(N.shape.ants.dW, d: 2) against #f(N.shape.rotor.dW, d: 2) for a fit over $k >= 18$; 0.22–0.25 against 0.10–0.13 at matched $k$ = 22–32)], [@sec:ens],
  [$kappa$ is rule-specific but not predicted by the rule string], [Moderately reproducible (ICC 0.58); held-out $R^2$ 0.003 (letters) against 0.78 for $c$], [@sec:factors],
  [Why $kappa$ takes its value, why it drifts in some rules], [*Unexplained*; a memoryless persistent walker (A1d) gives $kappa approx 1.4$ and area exponent 0.95, rejecting that memoryless control; edge-layer anatomy (A1-typical) separates a crust class from a uniform class but does not predict $kappa$ (predictions failed); $kappa$ is not shared between cyclic rotations of a rule (ICC 0.20, K1); a stochastic true self-avoiding walk gets $kappa$ up to 1.97 with area exponent 0.98 (E1), so $kappa approx 2$ is not specific to the rotor], [@sec:factors],
  [The prefactor of $A tilde t^(2/3)$ is set by $c kappa$], [Partly: dividing by $c kappa$ cuts the between-rule variance of $ln(A slash t^(2/3))$ at $2^26$ by #pct(N.h3.reduction) (pre-registered level 75%); a free fit gives power #f(N.h3.fit_power, d: 2), $R^2 = $ #f(N.h3.fit_r2, d: 2). $c$ and $kappa$ come from the same runs as $A$, so part of this is the identity of @eq:a], [`experiments/2026-10-13-h3-scaling-collapse/results/h3_collapse.txt`],
  [Rule-specific area exponent is filling-fraction drift], [Refuted: $gamma_eta$ explains $R^2 = $ #f(N.h1.r2_geta_vs_excess, d: 2) of $a - 2 alpha_w$ and has ICC #f(N.h1.icc_geta, d: 2) (predictions $>= 0.80$, $>= 0.35$); the two box sides co-vary instead], [`experiments/2026-10-13-h1-lacunarity-decomposition/results/h1_lacunarity.txt`],
  [Whether the edge exponent is 1/2 asymptotically], [*Unexplained*; the finite-window $beta$ is #f(N.mf_core.beta.at(1), d: 3) for ants and #f(N.calib_A.rotor.beta.median, d: 3) for the rotor; to $2^36$ the ants (six rules, two starts each) read $0.509 plus.minus 0.003$ against the rotor's $0.501 plus.minus 0.003$, both close to 1/2, but the decision rules we set in advance are inconclusive (W4)], [],
)
]

= Limitations

- *One trajectory per rule.* The ant is deterministic from a blank board; the only noise estimate is the patched-start control (#N.calib_C.rules rules, #N.calib_C.runs runs). Rules are not independent samples of anything; the bootstrap interval is a statement about this set of rules.
- *Finite windows.* All exponents come from seven octaves; the rotor calibration quantifies but does not remove the bias, and the rotor's bias need not equal the ants'. Long runs to $2^32$ cover only six rules.
- *Rule length $<= 12$.* Both the census and the $c$ model stop there. Rules with large $n$ are the majority of all rules; if the cube-root regime is a property of short rules, we cannot tell.
- *Edge exponent at finite time.* A separate check on the six long runs (`experiments/2026-10-13-edge-statistics/scripts/edge_exponent.py`, output `experiments/2026-10-13-edge-statistics/results/edge_exponent.txt`) gives local boundary exponents of $P$ against $A$ (slope over five octaves from $k$; a different estimator from the census $beta$, hence 0.51–0.53 rather than #f(N.mf_core.beta.at(1), d: 3)), averaged over the six rules, of 0.53 at $k = 20$, 0.51 at $k = 26$ and 0.52 at $k = 28$, with single rules between 0.48 and 0.55 and no clear fall; the rotor's local slope is 0.52 at $k = 22$. A pure $A^(1/2)$ law fits worse than $P = c_1 sqrt(A) + c_0$ (rms log-residual 0.045--0.16 against 0.018--0.073 over the six rules), so the data are consistent with a square-root edge plus an offset shared by both walkers, but do not establish the asymptotic value. Extending the six ant rules (blank and one patched start each) to $2^36$ gives a mean local slope of $0.509 plus.minus 0.003$ (twelve runs; the rotor at $2^36$: $0.501 plus.minus 0.003$), with a fall of $0.009 plus.minus 0.004$ over the last two octaves against the rotor's $0.012 plus.minus 0.004$; our pre-set rules for "consistent with 1/2" and "plateau above 1/2" both return inconclusive, and the fall again rests on the last octave.
- *Exclusions.* #N.probe.unanalysed rules were not fitted and #N.probe.bursty flagged bursty; the builder and log classes use thresholds chosen by eye.
- *Highway builders.* Classification is by finite simulation. A rule counted as chaotic may build a highway after $2^27$ steps (Langton's ant itself takes about $10^4$). Several hundred rules of length $<= 10$ were left unresolved in an earlier highway census (`experiments/2026-10-13-census-sweeps/results/unresolved_le10.txt`). Of 807 such rules run to $10^10$ steps, 38 (4.7%) became highways in $(10^9, 10^10]$. In a 240-rule sample run to $10^11$ only 3 (1.25%) did in $(10^10, 10^11]$, so the tail is thinning, not flat, but one decade cannot show that the set of late highways is finite (`experiments/2026-10-13-b5-late-highways/results/b5_stats.txt`; the other 492 survivors were not run in that sample). A rule that stays on the board is not necessarily in a stable regime either: `RLRLLL`, one of the long-run rules, ran as a blob until step $#f(N.g1.onset_t / 1e10, d: 4) times 10^10$ and then switched within a few $10^6$-step windows to a steady builder at #f(N.g1.steady_steps_per_new, d: 2) steps per newly visited cell (all #N.g1.steady_windows later windows), and left a $32768^2$ board at $#f(N.g1.exit_step / 1e10, d: 4) times 10^10$ (`examples/ribbon_trace.rs`, `experiments/2026-10-13-g1-ribbon-breakout/results/g1_stats.txt`; one blank-board trajectory, trigger unknown). Two $4 times 10^5$-step trajectory dumps at $1.28 times 10^10$ and $1.29 times 10^10$ (`examples/builder_dump.rs`, `experiments/2026-10-13-g1-ribbon-breakout/scripts/g1_builder_*.py`; notes/g1-builder-structure-2026-10-14.md) show that this builder is a boustrophedon column sweeper rather than a perimeter crawl: it makes a slow traverse of the full height $L$ (6,400 rows at the first dump and 8,610 at the second) at about 10 steps per row, a fast return at about 2 steps per row, and adds three columns per round trip, and the wake is exactly periodic in $x$ with period 3 (column agreement 0.99, colours 3, 4, 5 only). Three columns of $L$ cells per $approx 12 L$ steps is 4 steps per new cell, and $d x slash d t = -1 slash (4 L)$ with the measured $L approx 0.67 |x| - 960$ predicts $d|x|^2 slash d t = 0.86$ and 0.84 at the two dumps against 0.86 and 0.83 measured, so $|x|$ grows as $t^(1 slash 2)$ in this phase. The two values of $L$ come from the same single trajectory, so this is a consistency check on the sweep picture, not a test of the $L tilde |x|$ law; why the pattern has period 3 and what triggers the switch are unexplained. Three further board escapes without a recognised highway in the $10^11$ sample may be of this kind but were not traced.
- *Mechanism is a decomposition, not a derivation.* We do not derive the cube root from the rule; we show it follows from three measured quantities each of which is constant in time for most rules.
- *Related but untested ideas.* Experiments in which the ant's territory was replaced by statistically matched fields (pair, $2 times 2$, $3 times 3$) suggest that the log-growing "tethered" rules depend on the exact configuration of the territory rather than on local statistics. That work was run on two rules at one age with one trajectory each, and a follow-up knockout map has not been run. We draw no conclusion from it in this paper.

= Conclusion

Non-highway generalised Langton's ants of length 3–12 grow, to within the bias our calibration measures, as $t^(1/3)$ in linear size and $t^(2/3)$ in area. The growth rate splits into an edge-entry probability and an edge-occupation ratio that stay constant for the large majority of rules, together with an edge that scales close to $A^(1/2)$; rules that violate the cube root are those whose edge occupation drifts (correlation of $alpha$ with $gamma_kappa$ #f(N.identity.corr_alpha_gk, d: 2) over blob rules); this is bookkeeping through @eq:a, not a cause. The result is empirical and stops short of a proof, and the quantitative values of the constants are explained only for $c$ (by a rule-letter model) and not for $kappa$ or $beta$. We did measure the factors for the rotor with arrows initialised with a northward bias (`notes/frontier-2026-10-13-results.md`, C2b): $kappa$ rose from 2.01 to 2.20 at bias 0.15 while the width exponent stayed at 0.32–0.33, until the cube root broke between bias 0.25 and 0.30; randomising the arrow with probability $epsilon$ raised $kappa$ up to about 3-fold and destroyed the cube root (Arm B). Neither intervention explains the ants' rule-specific $kappa$. The broader boundary-hitting and return-time hypothesis remains open, although the excursion summaries tested above did not explain rule dependence beyond territory size. A natural next step is to extend the census beyond $n = 12$ with a stated-in-advance prediction, and to look for a rule-level quantity that, unlike the letters, the necklace and the edge-layer shape (all tested and failed), orders $kappa$.

The future-window forecast adds evidence beyond the same-window identities: early transport budgets predict later increments substantially better than an other-rule baseline in the typical replicated run. Rare near-stalls defeat the forecast, and the physical origin of persistent rule-level transport remains open.

= Availability and statements

*Data and code availability.* [PLACEHOLDER: repository URL and archived DOI (e.g. Zenodo) to be added once the working tree is committed.] The Rust simulator, the Python analysis scripts, the generated data files, `research-outputs/cube-root-growth-paper/compute_numbers.py` (every number in this text) and `research-outputs/cube-root-growth-paper/figures.py` are in the repository; the exact commands are in `research-outputs/cube-root-growth-paper/README.md` and `notes/`. At the time of writing the tree is uncommitted, so no commit hash is quoted.

*Use of AI tools.* The code, analyses, and draft of this paper were produced with substantial assistance from Claude (Anthropic) via Claude Code and Google Gemini via Antigravity, under the author's direction. Codex (OpenAI) also assisted with research planning, hypothesis development, and critical review of proposed experimental designs. The author is responsible for the content and has reviewed the results. [Check wording against the target venue's policy.]

*Competing interests and funding.* [PLACEHOLDER]

#pagebreak(weak: true)
#bibliography("refs.bib", style: "ieee")


