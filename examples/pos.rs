use turmite::reference::Ant;
fn main() { for (r,k) in [("RL",11000u64),("RLR",100000),("LLRR",100000),("RRLLRRLRRR",200000)] { let mut a=Ant::new(r.parse().unwrap()); a.run(k); println!("{r} {k} {} {} {}",a.pos.0,a.pos.1,a.heading);} }
