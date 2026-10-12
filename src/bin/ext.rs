//! Growth curves for the extension simulators (src/ext.rs).
//!
//! usage: ext MIN_LOG2 MAX_LOG2 SIZE2D SIZE3D < specs.txt > growth.csv
//! Spec lines (all deterministic; randomness only via explicit seeds):
//!   sq:RULE[:m=ANTS][:eps=NOISE][:bg][:seed=S]   L/R ant; several ants, turn noise, random background
//!   tm:STATES:COLOURS:ID                         multi-state turmite, table number ID
//!   hex:RULE                                     hexagonal lattice, letters r l R L (60/120 degrees)
//!   cube:RULE                                    cubic lattice, letters L R U D
//! Output rows: spec,k,area,extent[,escaped marker as k=-1].
use rayon::prelude::*;
use turmite::Rule;
use turmite::ext::*;

fn run(spec: &str, lo: u32, hi: u32, s2: usize, s3: usize) -> String {
    let parts: Vec<&str> = spec.split(':').collect();
    let (curve, esc) = match parts[0] {
        "sq" => {
            let rule: Rule = parts[1].parse().unwrap();
            let mut o = SqOpts::default();
            for p in &parts[2..] {
                if let Some(v) = p.strip_prefix("m=") {
                    o.ants = v.parse().unwrap();
                } else if let Some(v) = p.strip_prefix("eps=") {
                    o.noise = v.parse().unwrap();
                } else if let Some(v) = p.strip_prefix("seed=") {
                    let s: u64 = v.parse().unwrap();
                    o.noise_seed = s;
                    if o.bg_seed.is_some() || parts.contains(&"bg") {
                        o.bg_seed = Some(s ^ 0x5555);
                    }
                }
            }
            if parts.contains(&"bg") && o.bg_seed.is_none() {
                o.bg_seed = Some(0x5555);
            }
            growth(SqSim::new(Turmite::from_rule(&rule), s2, &o), lo, hi)
        }
        "tm" => {
            let tm = Turmite::from_id(parts[1].parse().unwrap(), parts[2].parse().unwrap(), parts[3].parse().unwrap());
            growth(SqSim::new(tm, s2, &SqOpts::default()), lo, hi)
        }
        "hex" => growth(HexSim::new(hex_turns(parts[1]).unwrap(), s2), lo, hi),
        "cube" => growth(CubeSim::new(parts[1], s3), lo, hi),
        "rotor3" => growth(Rotor3::new(parts[1].parse().unwrap(), s3), lo, hi),
        "tri" => growth(TriSim::new(&parts[1].parse::<Rule>().unwrap(), s2), lo, hi),
        other => panic!("unknown kind {other}"),
    };
    let mut out: String = curve.iter().map(|(k, a, e)| format!("{spec},{k},{a},{e}\n")).collect();
    if esc {
        out += &format!("{spec},-1,0,0\n");
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (lo, hi): (u32, u32) = (a[1].parse().unwrap(), a[2].parse().unwrap());
    let s2: usize = a[3].parse().unwrap();
    let s3: usize = a[4].parse().unwrap();
    let specs: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap().trim().to_string()).filter(|l| !l.is_empty()).collect();
    let blocks: Vec<String> = specs.par_iter().map(|s| run(s, lo, hi, s2, s3)).collect();
    println!("spec,k,area,extent");
    for b in blocks {
        print!("{b}");
    }
}
