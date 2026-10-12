use turmite::fast::{self, Config, Grid};
fn main() {
    let mut g = Grid::new(4096);
    for r in ["RL", "RLR", "LLRR", "RRLL", "LRRRRRLLR", "RRLLLRLLLRRR"] {
        let rule = r.parse().unwrap();
        let t = std::time::Instant::now();
        let res = fast::run(&rule, &mut g, &Config::default());
        let onset = res.highway.map(|h| fast::highway_onset(&rule, h, res.steps));
        println!("{r:>14} {:?} steps={} hw={:?} onset={:?} extent={:?} {:?}", res.stop, res.steps, res.highway, onset, res.extent, t.elapsed());
    }
}
