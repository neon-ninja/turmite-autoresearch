use turmite::{Rule, reference::Ant, invariants::{Law, mine, verifies}};

fn value(ant: &Ant, law: &Law) -> i64 {
    let [a,b,d,e] = law.coefficients.map(i64::from);
    (a * i64::from(ant.pos.0) + b * i64::from(ant.pos.1)
        + d * i64::from(ant.heading) + e * ant.steps as i64
        + ant.cells.values().map(|&c| i64::from(law.weights[c as usize])).sum::<i64>())
        .rem_euclid(i64::from(law.modulus))
}

#[test]
fn all_small_rules_against_oracle() {
    for n in 2..=5 { for bits in 0..(1 << n) {
        let rule = Rule::from_bits(bits, n);
        for m in [2, 3, 4] {
            let laws = mine(&rule, m);
            for heading in 0..4 {
                let mut ant = Ant::new(rule.clone());
                ant.heading = heading;
                ant.pos = (-2, 1);
                for x in -3..=3 { for y in -3..=3 {
                    ant.cells.insert((x,y), (x + 2_i32 * y).rem_euclid(n as i32) as u8);
                }}
                let initial: Vec<_> = laws.iter().map(|law| value(&ant, law)).collect();
                for _ in 0..64 {
                    ant.step();
                    for (law, &expected) in laws.iter().zip(&initial) { assert_eq!(value(&ant, law), expected); }
                }
            }
        }
    }}
}

#[test]
fn controls_and_completeness() {
    let rule: Rule = "RL".parse().unwrap();
    let laws = mine(&rule, 2);
    assert!(laws.contains(&Law { modulus: 2, coefficients: [1,1,0,1], weights: vec![0,0] }));
    assert!(laws.contains(&Law { modulus: 2, coefficients: [0,0,0,1], weights: vec![0,1] }));
    assert!(!verifies(&rule, &Law { modulus: 2, coefficients: [1,0,0,0], weights: vec![0,0] }));
    for code in 1..16 {
        for weight in 0..2 {
            let law = Law { modulus: 2, coefficients: std::array::from_fn(|i| (code >> i) & 1), weights: vec![0, weight] };
            assert_eq!(verifies(&rule, &law), laws.contains(&law));
        }
    }
}
