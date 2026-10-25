//! prefix-causal-horizon: print the first-read time of every colour of each rule on stdin.
//! usage: prefix_horizon MAX_STEPS [BOARD] < rules.txt
//! CSV: rule,n,steps,escaped,wrapped_read,first_reads  (first_reads = first_0;first_1;..., empty = colour not read).
//! Deterministic; no seeds.
use turmite::prefix_horizon::horizon;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let max_steps: u64 = a[0].parse().unwrap();
    let board: usize = a.get(1).map_or(4096, |b| b.parse().unwrap());
    println!("rule,n,steps,escaped,wrapped_read,first_reads");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let rule = line.trim().parse().unwrap();
        let h = horizon(&rule, max_steps, board);
        let firsts: Vec<String> = h.first.iter().map(|f| f.map_or(String::new(), |v| v.to_string())).collect();
        println!("{},{},{},{},{},{}", line.trim(), h.first.len(), h.steps, h.escaped, h.wrapped_read.map_or(String::new(), |v| v.to_string()), firsts.join(";"));
    }
}
