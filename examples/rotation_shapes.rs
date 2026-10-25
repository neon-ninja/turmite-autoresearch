//! rotation-boxes (Q25): do two rules leave the same footprint after the same number of steps, up to translation and the
//! symmetries of the square?
//! usage: rotation_shapes STEPS BOARD < pairs.txt      (one "RULE_A RULE_B" per line)
//! CSV: rule_a,rule_b,steps,shape_a,shape_b,equal,self_shift_a,self_shift_b,mirror_a   (shape = WxH:cells:hash)
//!   self_shift_*: the rule's own shape at STEPS vs STEPS+1000 differs (control C1: the test can say "different")
//!   mirror_a: shape of rule_a equals shape of its mirror image (control C2: must be true)
use turmite::prefix_horizon::touched_shape;

fn fmt(s: Option<turmite::prefix_horizon::Shape>) -> String {
    s.map_or("left_board".to_string(), |s| format!("{}x{}:{}:{:016x}", s.width, s.height, s.cells, s.hash))
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let steps: u64 = a[0].parse().unwrap();
    let board: usize = a[1].parse().unwrap();
    println!("rule_a,rule_b,steps,shape_a,shape_b,equal,self_shift_a,self_shift_b,mirror_a");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut it = line.split_whitespace();
        let (sa, sb) = (it.next().unwrap(), it.next().unwrap());
        let (ra, rb): (turmite::Rule, turmite::Rule) = (sa.parse().unwrap(), sb.parse().unwrap());
        let (ta, tb) = (touched_shape(&ra, steps, board), touched_shape(&rb, steps, board));
        let later = |r: &turmite::Rule| touched_shape(r, steps + 1000, board);
        let mirror: turmite::Rule = sa.chars().map(|c| if c == 'L' { 'R' } else { 'L' }).collect::<String>().parse().unwrap();
        println!(
            "{sa},{sb},{steps},{},{},{},{},{},{}",
            fmt(ta),
            fmt(tb),
            ta.is_some() && ta == tb,
            ta != later(&ra),
            tb != later(&rb),
            ta == touched_shape(&mirror, steps, board)
        );
    }
}
