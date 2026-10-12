//! arrowstats SEEDS : frequency of each initial arrow and of equal-neighbour
//! pairs (horizontal and vertical) over a 2048 x 2048 region, for seeds 0..SEEDS.
use turmite::walkers::initial_arrow;
fn main() {
    let n: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let (mut freq, mut eqh, mut eqv, mut pairs) = ([0u64; 4], 0u64, 0u64, 0u64);
    let mut joint = [[0u64; 4]; 4];
    for seed in 0..n {
        for y in -1024i64..1024 {
            for x in -1024i64..1024 {
                let a = initial_arrow(seed, x, y);
                freq[a as usize] += 1;
                let r = initial_arrow(seed, x + 1, y);
                let d = initial_arrow(seed, x, y + 1);
                eqh += (a == r) as u64;
                eqv += (a == d) as u64;
                joint[a as usize][r as usize] += 1;
                pairs += 1;
            }
        }
    }
    let tot: u64 = freq.iter().sum();
    println!("freq: {:?}", freq.map(|f| f as f64 / tot as f64));
    println!("P(equal) horizontal {:.6} vertical {:.6} (expect 0.25, sd {:.6})", eqh as f64 / pairs as f64, eqv as f64 / pairs as f64, (0.25f64 * 0.75 / pairs as f64).sqrt());
    let chi: f64 = joint.iter().flatten().map(|&o| { let e = pairs as f64 / 16.0; (o as f64 - e).powi(2) / e }).sum();
    println!("chi^2 of horizontal pair table (15 dof): {chi:.1}");
}
