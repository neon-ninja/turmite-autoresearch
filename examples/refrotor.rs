//! refrotor SEED N : distinct sites (including the origin) and squared distance
//! after N steps of the reference rotor.
use std::collections::HashSet;
use turmite::reference::Rotor;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut r = Rotor::new(a[1].parse().unwrap());
    let n: u64 = a[2].parse().unwrap();
    let mut seen = HashSet::from([(0, 0)]);
    for _ in 0..n { r.step(); seen.insert(r.pos); }
    println!("{},{},{}", n, r.pos.0 * r.pos.0 + r.pos.1 * r.pos.1, seen.len());
}
