//! lattice_c_law_run MIN_LOG2 MAX_LOG2 SIZE < specs > windows.csv
//! One spec per line: `LATTICE MOVER` with LATTICE in square|hex|tri and MOVER `ant:RULE`, `rw:SEED` or `mf:SEED`.
//! Output rows (see `lattice_probe::CSV_HEADER`) in spec order, independent of the thread count.

use rayon::prelude::*;
use std::io::Read;
use turmite::lattice_probe::*;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (lo, hi, size): (u32, u32, usize) = (a[1].parse().unwrap(), a[2].parse().unwrap(), a[3].parse().unwrap());
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let specs: Vec<(Lattice, String)> = s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (lat, m) = l.split_once(' ').expect("LATTICE MOVER");
            let lat = match lat {
                "square" => Lattice::Square,
                "hex" => Lattice::Hex,
                "tri" => Lattice::Tri,
                o => panic!("bad lattice {o}"),
            };
            (lat, m.trim().to_string())
        })
        .collect();
    let rows: Vec<String> = specs
        .par_iter()
        .map(|(lat, m)| {
            let (kind, arg) = m.split_once(':').expect("kind:arg");
            let mover = match kind {
                "ant" => Mover::Ant(arg.to_string()),
                "rw" => Mover::RandomWalk(arg.parse().unwrap()),
                "mf" => Mover::MeanField(arg.parse().unwrap()),
                o => panic!("bad mover {o}"),
            };
            csv_rows(*lat, m, &lattice_probe(*lat, &mover, size, lo, hi))
        })
        .collect();
    println!("{CSV_HEADER}");
    for r in rows {
        print!("{r}");
    }
}
