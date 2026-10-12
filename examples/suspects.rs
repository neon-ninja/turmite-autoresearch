use turmite::fast::{self, Config, Grid};
fn main() {
    let mut g = Grid::new(8192);
    for r in std::env::args().skip(1) {
        let rule = r.parse().unwrap();
        let res = fast::run(&rule, &mut g, &Config { max_steps: 200_000_000, ..Config::default() });
        let onset = res.highway.map(|h| fast::highway_onset(&rule, h, res.steps));
        println!("{r:>13} {:?} steps={} hw={:?} onset={onset:?} tail={:?} extent={:?}", res.stop, res.steps, res.highway.map(|h|h.period), res.periodic_tail.map(|h|h.period), res.extent);
    }
}
