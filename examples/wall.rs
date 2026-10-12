use turmite::fast::{self, Config, Grid};
fn main() {
    let r: turmite::Rule = "RRLRRRRRRLRL".parse().unwrap();
    for size in [512usize, 1024, 2048, 3072, 4096, 6144] {
        let mut g = Grid::new(size);
        let res = fast::run(&r, &mut g, &Config { max_steps: 300_000_000, ..Config::default() });
        println!("{size}: {:?} steps={} hw={:?} tail={:?} heading={} extent={:?}", res.stop, res.steps, res.highway, res.periodic_tail, res.heading, res.extent);
    }
}
