# Hull-width lab notebook

Append-only. Tags: intent, assumption, observation, interpretation, surprise, decision, dead-end, question, deviation.

### 2026-10-14 22:15 (+13:00) · 797704a · intent
Referee-style feedback on the cube-root paper asked whether the ants' faster W_surf growth is outer-interface roughening or hole rims. The notes (frontier-results.md P7) already said holes were never separated. Build an instrument that splits them and register predictions before running it. Informs the paper's edge claims (Sec. 7, limitations).

### 2026-10-14 22:15 (+13:00) · 797704a · assumption
Hole rims are the only difference between "all surface" and "outer surface" under 4-connectivity of the exterior. Diagonal-only channels to the border count as closed, so some pockets that are 8-connected to the outside are classed as holes. Check: rerun the split with 8-connected exterior on a few runs and compare P_hole. Untested.

### 2026-10-14 22:15 (+13:00) · 797704a · assumption
The outermost-cell-per-angular-bin profile is a fair proxy for the hull; it ignores overhangs and is an extremal statistic. Same bias for both walkers, but not shown to cancel in the spectrum exponent. Check by using the rotor as reference and by comparing with the outer-cell radial sd.

### 2026-10-14 22:15 (+13:00) · 797704a · observation
Oracle test `cargo test --release --example hull_width` passes (400-cell square, 3x3 and 1-cell holes: A=390, outer 76, hole rim 16, 2 holes, hole area 10).

### 2026-10-14 22:55 (+13:00) · 797704a · observation
run_hull.sh then hull_stats.py. No escapes. Ants W_out exponent 0.239, W_all 0.239; rotor W_out 0.135, W_all 0.130. Hole-rim share of surface cells: ants 0.038, rotor 0.252 at k=28. zeta_loc ants 0.63/0.60 vs rotor 0.44/0.40 (k=28/30); D_h ants 1.07 vs rotor 1.16. W_all matches shape's Wsurf exactly on 160 rows.

### 2026-10-14 22:55 (+13:00) · 797704a · surprise
Holes: the rotor has six times the hole-rim share of the ants; I had expected the reverse (prediction 2 failed). Hull dimension also came out lower for ants (prediction 5 failed). The ants' hull is smoother at small scale but wider overall.

### 2026-10-14 22:55 (+13:00) · 797704a · deviation
Prediction 4 was met numerically but I had the sign of "rougher" wrong (larger zeta_loc = smoother). Not counted as support for H-rough beyond prediction 3. First hull_stats.py run was killed for slowness (polars filtering inside the bootstrap loop) and the loop vectorised; estimators unchanged.

### 2026-10-14 22:55 (+13:00) · 797704a · interpretation
Porosity does not explain the wider ant edge (H-hole rejected, consistent across W_all/W_out). Remaining candidate: long-wavelength lobes/notches. Confidence moderate; 6 rules, finite window, extremal profile. Alternatives: the outermost-per-bin profile hides overhang structure; D_h range too short.
