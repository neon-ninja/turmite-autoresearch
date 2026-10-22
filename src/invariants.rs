//! Exhaustive additive modular laws, proved by checking every local transition.
use crate::{DIRS, Rule};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Law {
    pub modulus: i32,
    /// Coefficients of x, y, heading (0..3), and elapsed steps.
    pub coefficients: [i32; 4],
    /// Site potential, with colour zero fixed to zero.
    pub weights: Vec<i32>,
}

/// Independently check the induction step, including heading and colour wraps.
pub fn verifies(rule: &Rule, law: &Law) -> bool {
    let m = law.modulus;
    if m < 2 || law.weights.len() != rule.len() || law.weights[0] != 0 {
        return false;
    }
    let [a, b, d, e] = law.coefficients;
    (0..rule.len()).all(|c| (0..4).all(|h| {
        let next = (h + i32::from(rule.0[c].delta())) % 4;
        let (dx, dy) = DIRS[next as usize];
        (a * dx + b * dy + d * (next - h) + e
            + law.weights[(c + 1) % rule.len()] - law.weights[c]).rem_euclid(m) == 0
    }))
}

/// All nonzero coefficient tuples in this ansatz; includes dependent multiples.
/// Composite moduli are supported without assuming a field or dividing pivots.
pub fn mine(rule: &Rule, modulus: i32) -> Vec<Law> {
    assert!((2..=16).contains(&modulus));
    let mut laws = Vec::new();
    for a in 0..modulus { for b in 0..modulus { for d in 0..modulus { for e in 0..modulus {
        if [a, b, d, e] == [0; 4] { continue; }
        let mut weights = vec![0];
        for turn in &rule.0[..rule.len() - 1] {
            let h = i32::from(turn.delta());
            let (dx, dy) = DIRS[h as usize];
            weights.push((weights.last().unwrap() - a * dx - b * dy - d * h - e).rem_euclid(modulus));
        }
        let law = Law { modulus, coefficients: [a, b, d, e], weights };
        if verifies(rule, &law) { laws.push(law); }
    }}}}
    laws
}
