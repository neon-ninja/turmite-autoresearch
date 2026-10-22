//! Attack a registered claim instead of re-running it for longer.
//!
//! usage: falsify CLAIMS.txt OUT_DIR [--allow-dirty]
//!        falsify --replay OUT_DIR/ID.counterexamples.txt
//!
//! One claim per line (see `turmite::falsify::Claim::parse` and the experiment README). The claims
//! file must be committed unchanged, so the prediction predates the attack (AGENTS rule 8);
//! `--allow-dirty` is for harness development and is written into the provenance header.
//! Writes OUT_DIR/ID.csv (every trial), ID.verdict.txt and ID.counterexamples.txt (shrunk,
//! replayable). Output is identical for any RAYON_NUM_THREADS: trials are keyed by
//! (claim seed, rule, class, index) and sorted before writing.

use rayon::prelude::*;
use std::fmt::Write as _;
use std::process::Command;
use turmite::Rule;
use turmite::falsify::*;

fn git(args: &[&str]) -> String {
    let output = Command::new("git").args(args).output().expect("git is required for provenance");
    assert!(output.status.success(), "git {:?} failed: {}", args, String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn outcome_str(o: Outcome) -> String {
    match o {
        Outcome::Highway(hw) => format!("highway,{},{},{}", hw.period, hw.dx, hw.dy),
        Outcome::EdgeEscape => "edge,,,".into(),
        Outcome::Unresolved => "unresolved,,,".into(),
    }
}

fn trial_line(rule: &Rule, t: &Trial, horizon: u64) -> String {
    let p = t.perturb.as_ref().map_or("-".to_string(), |p| format!("{}:{}:{}:{}", p.at, p.ahead, p.radius, p.seed));
    format!(
        "rule={rule} class={} heading={} start={},{} size={} horizon={horizon} perturb={p} cells={}",
        t.class.name(),
        t.init.heading,
        t.sim.start.0,
        t.sim.start.1,
        t.sim.size,
        if t.init.cells.is_empty() { "-".into() } else { cells_to_string(&t.init.cells) }
    )
}

fn replay(path: &str) {
    for line in std::fs::read_to_string(path).unwrap().lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let kv: std::collections::BTreeMap<&str, &str> = line.split_whitespace().filter_map(|t| t.split_once('=')).collect();
        let rule: Rule = kv["rule"].parse().unwrap();
        let (sx, sy) = kv["start"].split_once(',').unwrap();
        let horizon: u64 = kv["horizon"].parse().unwrap();
        let mut sim = Sim::centred(kv["size"].parse().unwrap(), horizon);
        sim.start = (sx.parse().unwrap(), sy.parse().unwrap());
        let init = Init { cells: cells_from_string(kv["cells"]).unwrap(), heading: kv["heading"].parse().unwrap() };
        let perturb = (kv["perturb"] != "-").then(|| {
            let v: Vec<&str> = kv["perturb"].split(':').collect();
            Perturb { at: v[0].parse().unwrap(), ahead: v[1].parse().unwrap(), radius: v[2].parse().unwrap(), seed: v[3].parse().unwrap() }
        });
        let run = simulate(&rule, &init, &sim, perturb.as_ref());
        println!("{rule} {} -> {} steps={}", kv["class"], outcome_str(run.outcome), run.steps);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: falsify CLAIMS.txt OUT_DIR [--allow-dirty] | --replay FILE");
        std::process::exit(2);
    }
    if args.first().map(String::as_str) == Some("--replay") {
        return replay(&args[1]);
    }
    let (claims_path, out_dir) = (&args[0], &args[1]);
    let allow_dirty = args.iter().any(|a| a == "--allow-dirty");
    let tracked = Command::new("git").args(["ls-files", "--error-unmatch", "--", claims_path]).output()
        .is_ok_and(|o| o.status.success());
    let dirty = !tracked || !git(&["status", "--porcelain", "--", claims_path]).is_empty();
    if dirty && !allow_dirty {
        eprintln!("refusing: {claims_path} has uncommitted changes or is untracked. Commit the registered claims first (or --allow-dirty).");
        std::process::exit(2);
    }
    let provenance = format!(
        "# cmd: falsify {}\n# commit: {}\n# claims file blob: {}{}\n",
        args.join(" "),
        git(&["rev-parse", "HEAD"]),
        git(&["hash-object", claims_path]),
        if dirty { " (DIRTY: not a registered run)" } else { "" }
    );
    std::fs::create_dir_all(out_dir).unwrap();
    let text = std::fs::read_to_string(claims_path).unwrap();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let claim = Claim::parse(line).unwrap_or_else(|e| panic!("{line}: {e}"));
        run_claim(&claim, line, &provenance, out_dir);
    }
}

fn run_claim(claim: &Claim, line: &str, provenance: &str, out_dir: &str) {
    let rule = &claim.rule;
    // `horizon` is what the claim's author looked at; attack up to 4^ladder times further. Run once at
    // the top: a highway "by h" is one whose escape step is <= h.
    let top = claim.horizon * 4u64.pow(claim.ladder);
    let base = baseline(rule, claim.size, top);
    let plan = plan(claim, &base);
    let mut results: Vec<TrialResult> = plan
        .par_iter()
        .map(|&(c, i)| run_trial(rule, &make_trial(rule, c, i, claim.seed, claim.size, top, &base)))
        .collect();
    if claim.classes.contains(&Class::Adaptive) {
        let chains: Vec<Vec<TrialResult>> = (0..claim.chains)
            .into_par_iter()
            .map(|c| adaptive_chain(rule, claim.kind, c, claim.evals, claim.seed, claim.size, top, &base))
            .collect();
        results.extend(chains.into_iter().flatten());
    }
    results.sort_by_key(|r| (r.trial.class, r.trial.index));
    let mut report = evaluate(claim, &base, &results);

    // A counterexample must survive a change of board geometry before it counts, then is shrunk.
    let falsifies = |r: &Run| match claim.kind {
        Kind::Chaotic => matches!(r.outcome, Outcome::Highway(_)),
        Kind::Highway => matches!(r.outcome, Outcome::Highway(hw) if Some(canon(hw)) != base.canon) && claim.same_canon,
    };
    let mut ce_text = String::new();
    let mut reproduced = 0;
    if claim.universal || claim.kind == Kind::Chaotic {
        for &i in report.counterexamples.iter().take(5) {
            let t = &results[i].trial;
            let again = run_trial(rule, &on_other_geometry(t));
            if !falsifies(&again.run) {
                continue;
            }
            reproduced += 1;
            let small = shrink(&t.init, 300, |init| {
                let mut u = t.clone();
                u.init = init.clone();
                falsifies(&run_trial(rule, &u).run)
                    && falsifies(&run_trial(rule, &on_other_geometry(&u)).run)
            });
            let mut u = t.clone();
            u.init = small;
            let _ = writeln!(ce_text, "{}", trial_line(rule, &u, top));
        }
        if !report.counterexamples.is_empty() && reproduced == 0 {
            report.verdict = Verdict::Invalid;
            report.reasons.push("counterexample did not reproduce on a second board geometry: detector/board artefact".into());
        }
    }

    let mut csv = format!("{provenance}# claim: {line}\n");
    let _ = writeln!(csv, "# baseline: {:?} canon={:?}", base.run.outcome, base.canon);
    csv.push_str("class,index,seed,outcome,period,dx,dy,steps,ext_w,ext_h,category,heading,size,start,perturb,cells\n");
    for r in &results {
        let t = &r.trial;
        let _ = writeln!(
            csv,
            "{},{},{},{},{},{},{},{:?},{},{},{}:{},{},{}",
            t.class.name(),
            t.index,
            t.seed,
            outcome_str(r.run.outcome),
            r.run.steps,
            r.run.extent.0,
            r.run.extent.1,
            categorise(&r.run, &base),
            t.init.heading,
            t.sim.size,
            t.sim.start.0,
            t.sim.start.1,
            t.perturb.as_ref().map_or("-".to_string(), |p| format!("{}:{}:{}", p.at, p.ahead, p.radius)),
            if t.init.cells.is_empty() { "-".into() } else { cells_to_string(&t.init.cells) },
        );
    }
    std::fs::write(format!("{out_dir}/{}.csv", claim.id), csv).unwrap();

    let mut v = format!("{provenance}# claim: {line}\n");
    let _ = writeln!(v, "claim {} ({:?}, rule {}, registered {})", claim.id, claim.kind, claim.rule, claim.registered);
    let _ = writeln!(v, "baseline (blank start): {:?}, canon {:?}", base.run.outcome, base.canon);
    let _ = writeln!(v, "\n{:<9} {:>5} {:>8} {:>8} {:>6} {:>10}", "class", "n", "hw_same", "hw_other", "edge", "unresolved");
    for row in &report.rows {
        let _ = writeln!(v, "{:<9} {:>5} {:>8} {:>8} {:>6} {:>10}", row.class.unwrap().name(), row.n, row.hw_same, row.hw_other, row.edge, row.unresolved);
    }
    if let Some(o) = base.onset {
        let _ = writeln!(v, "baseline highway onset (exact, blank board): step {o}");
    }
    let _ = writeln!(v, "\nconfirmed escapes by horizon (attack trials only; an escape lags its onset by the board-crossing time; survival is not proof):");
    for (h, k, n) in &report.rungs {
        let _ = writeln!(v, "  h = {h:>12}: {k}/{n}");
    }
    if let Some((lo, hi)) = report.success_ci {
        let _ = writeln!(v, "highway success rate, Clopper-Pearson 95%: [{lo:.4}, {hi:.4}]");
    }
    for r in &report.reasons {
        let _ = writeln!(v, "note: {r}");
    }
    let _ = writeln!(v, "counterexamples: {} found, {} reproduced on a second geometry", report.counterexamples.len(), reproduced);
    let _ = writeln!(v, "VERDICT {}: {:?}", claim.id, report.verdict);
    std::fs::write(format!("{out_dir}/{}.verdict.txt", claim.id), &v).unwrap();
    if !ce_text.is_empty() {
        std::fs::write(format!("{out_dir}/{}.counterexamples.txt", claim.id), format!("{provenance}# claim: {line}\n{ce_text}")).unwrap();
    }
    print!("{v}");
}
