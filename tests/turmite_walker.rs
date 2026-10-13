//! `TurmiteWalker` (used by `examples/shape.rs` for `tm:` specs) must agree step-for-step with the
//! `ext::SqSim` oracle: same cell colours and positions.
use proptest::prelude::*;
use turmite::DIRS;
use turmite::ext::{Sim, SqOpts, SqSim, Turmite};
use turmite::walkers::{TurmiteWalker, Walker};

proptest! {
    #[test]
    fn turmite_walker_matches_sqsim(id in 0u64..(64u64.pow(4)), steps in 1usize..3000) {
        let size = 512usize;
        let tm = Turmite::from_id(2, 2, id % (16u64.pow(4)));
        let mut sim = SqSim::new(Turmite::from_id(2, 2, id % (16u64.pow(4))), size, &SqOpts::default());
        let mut w = TurmiteWalker::new(tm);
        let c = (size / 2) as i64;
        let mut cells = vec![0u8; size * size];
        let (mut x, mut y, mut h) = (0i64, 0i64, 0u8);
        for _ in 0..steps {
            sim.step();
            if sim.escaped() { break; }
            let idx = ((c + y) as usize) * size + (c + x) as usize;
            w.step(&mut cells[idx], &mut h);
            x += DIRS[h as usize].0 as i64;
            y += DIRS[h as usize].1 as i64;
            prop_assert_eq!(sim.positions()[0], (x, y));
            prop_assert_eq!(sim.cell(x, y), cells[((c + y) as usize) * size + (c + x) as usize]);
        }
    }
}
