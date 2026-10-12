use turmite::fast::{self, Config, Grid};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rule: turmite::Rule = a[1].parse().unwrap();
    for size in [8192usize, 16384] {
        let mut g = Grid::new(size);
        let res = fast::run(&rule, &mut g, &Config { max_steps: 1_000_000_000, ..Config::default() });
        let onset = res.highway.map(|h| fast::highway_onset(&rule, h, res.steps));
        println!("{} grid {size}: {:?} steps={} hw={:?} onset={onset:?} extent={:?}", a[1], res.stop, res.steps, res.highway, res.extent);
    }
}
