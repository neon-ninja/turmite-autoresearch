use turmite::fast::{self, Config, Grid};
fn main() {
    let r = std::env::args().nth(1).unwrap();
    let rule = r.parse().unwrap();
    let mut g = Grid::new(8192);
    let cfg = Config { max_steps: 300_000_000, checkpoint: 300_000_000, max_period: 1_000_000, history: 5_000_000 };
    let res = fast::run(&rule, &mut g, &cfg);
    println!("{r} {:?} steps={} hw={:?} tail={:?} pos={:?}", res.stop, res.steps, res.highway, res.periodic_tail, res.pos);
    if let Some(h) = res.highway { println!("onset {}", fast::highway_onset(&rule, h, res.steps)); }
}
