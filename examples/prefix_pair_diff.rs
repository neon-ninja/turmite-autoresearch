//! prefix-causal-horizon: when do the positions of two plain ants first differ?
//! usage: prefix_pair_diff RULE_A RULE_B MAX_STEPS [BOARD]    (prints one CSV line: rule_a,rule_b,result,steps)
use turmite::prefix_horizon::{PairRun, first_position_difference};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (ra, rb) = (a[0].parse().unwrap(), a[1].parse().unwrap());
    let max_steps: u64 = a[2].parse().unwrap();
    let board: usize = a.get(3).map_or(4096, |b| b.parse().unwrap());
    let (kind, t) = match first_position_difference(&ra, &rb, max_steps, board) {
        PairRun::Differ(t) => ("differ", t),
        PairRun::Same(t) => ("same", t),
        PairRun::Left(t) => ("left_board", t),
    };
    println!("{},{},{kind},{t}", a[0], a[1]);
}
