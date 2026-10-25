//! rotation-boxes part 3: footprint of a rule on each uniform background k = 0..n-1.
//! usage: rotation_backgrounds STEPS BOARD < rules.txt     CSV: rule,k,steps,shape (WxH:cells:hash, or left_board)
use turmite::prefix_horizon::touched_shape_bg;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let steps: u64 = a[0].parse().unwrap();
    let board: usize = a[1].parse().unwrap();
    println!("rule,k,steps,shape");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let name = line.trim();
        let rule: turmite::Rule = name.parse().unwrap();
        for k in 0..rule.len() {
            let s = touched_shape_bg(&rule, steps, board, k)
                .map_or("left_board".to_string(), |s| format!("{}x{}:{}:{:016x}", s.width, s.height, s.cells, s.hash));
            println!("{name},{k},{steps},{s}");
        }
    }
}
