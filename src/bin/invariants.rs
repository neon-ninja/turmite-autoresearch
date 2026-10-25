//! Machine-readable exact constraints for downstream census analysis.
use turmite::{Rule, invariants::mine};

fn main() -> Result<(), String> {
    let rules = std::env::args().skip(1).map(|s| s.parse::<Rule>()).collect::<Result<Vec<_>, _>>()?;
    if rules.is_empty() { return Err("usage: invariants RULE [RULE ...] (moduli 2, 4)".into()); }
    println!("rule,modulus,x,y,heading,time,colour_weights");
    for rule in rules {
        for m in [2, 4] {
            for law in mine(&rule, m) {
                let [a,b,d,e] = law.coefficients;
                let weights = law.weights.iter().map(ToString::to_string).collect::<Vec<_>>().join(";");
                println!("{rule},{m},{a},{b},{d},{e},{weights}");
            }
        }
    }
    Ok(())
}
