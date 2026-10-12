//! A repeated rule u^k is the same ant as u (frontier item 23).
use proptest::prelude::*;
use turmite::Rule;
use turmite::reference::Ant;

proptest! {
    #[test]
    fn repeat_equals_root(bits in 1u32..64, n in 2usize..7, k in 2usize..4, steps in 1usize..3000) {
        let u = Rule::from_bits(bits, n);
        let rep = Rule(u.0.repeat(k));
        let (mut a, mut b) = (Ant::new(u.clone()), Ant::new(rep.clone()));
        for _ in 0..steps {
            a.step();
            b.step();
            prop_assert_eq!(a.pos, b.pos);
            prop_assert_eq!(a.heading, b.heading);
        }
        prop_assert_eq!(rep.root().0.len() <= n, true);
    }
}

#[test]
fn root_examples() {
    assert_eq!("RLRL".parse::<Rule>().unwrap().root().to_string(), "RL");
    assert_eq!("RRLRRL".parse::<Rule>().unwrap().root().to_string(), "RRL");
    assert!("RLL".parse::<Rule>().unwrap().is_primitive());
    assert!(!"RLLRLL".parse::<Rule>().unwrap().is_primitive());
}
