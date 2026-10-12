use turmite::reference::Ant;
fn main() { let a: Vec<String> = std::env::args().collect(); let mut ant = Ant::new(a[1].parse().unwrap()); ant.run(a[2].parse().unwrap()); println!("{} {}", ant.pos.0, ant.pos.1); }
