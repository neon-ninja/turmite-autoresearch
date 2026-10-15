//! Windowed trajectory observations; no window is classified as a ray automatically.
//! Usage: RULE STEPS SIZE WINDOW (CSV to stdout). Blank initial board.
use turmite::{Rule, DIRS};

#[derive(Clone)]
struct Board {
    size: usize,
    cells: Vec<u8>,
    pos: (i32, i32),
    heading: u8,
    rule: Rule,
}
impl Board {
    fn new(rule: Rule, size: usize) -> Self {
        Self { size, cells: vec![0; size * size], pos: (0, 0), heading: 0, rule }
    }
    fn index(&self, p: (i32, i32)) -> usize {
        let c = (self.size / 2) as i32;
        assert!(p.0 + c >= 0 && p.1 + c >= 0 && p.0 + c < self.size as i32 && p.1 + c < self.size as i32);
        (p.1 + c) as usize * self.size + (p.0 + c) as usize
    }
    fn step(&mut self) -> u8 {
        let i = self.index(self.pos);
        let colour = self.cells[i];
        self.heading = (self.heading + self.rule.0[colour as usize].delta()) & 3;
        self.cells[i] = (colour + 1) % self.rule.len() as u8;
        let d = DIRS[self.heading as usize];
        self.pos = (self.pos.0 + d.0, self.pos.1 + d.1);
        colour
    }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.get(1).map(String::as_str) == Some("intervene") {
        intervene(&a[2]);
        return;
    }
    assert_eq!(a.len(), 5, "RULE STEPS SIZE WINDOW");
    let mut b = Board::new(a[1].parse().unwrap(), a[3].parse().unwrap());
    let steps: u64 = a[2].parse().unwrap();
    let window: u64 = a[4].parse().unwrap();
    assert!(window > 0);
    println!("t_start,t_end,x_start,y_start,x_end,y_end,heading_end,reads_colour_0,reads_colour_2");
    let mut start = b.pos;
    let mut counts = [0u64; 2];
    let mut t0 = 0;
    for t in 1..=steps {
        let colour = b.step();
        counts[0] += u64::from(colour == 0);
        counts[1] += u64::from(colour == 2);
        if t % window == 0 || t == steps {
            println!("{t0},{t},{},{},{},{},{},{},{}", start.0, start.1, b.pos.0, b.pos.1, b.heading, counts[0], counts[1]);
            start = b.pos;
            counts = [0; 2];
            t0 = t;
        }
    }
}

// A paired experiment: preserve a local snapshot, change only the exterior.
// Select by displacement before looking at intervention outcomes.
fn intervene(rule: &str) {
    let mut b = Board::new(rule.parse().unwrap(), 4096);
    let window = 4096;
    println!("rule,t_start,background,dx,dy,exact_prefix_steps");
    let mut selected = 0;
    for block in 0..8192 {
        let original = b.clone();
        let start = b.pos;
        let mut path = Vec::new();
        for _ in 0..window { b.step(); path.push((b.pos, b.heading)); }
        let dx = b.pos.0 - start.0;
        let dy = b.pos.1 - start.1;
        if dx.abs() < 64 || (dx.abs() - dy.abs()).abs() > 4 { continue; }
        for background in [0u8, 2u8] {
            let mut transplant = Board::new(original.rule.clone(), original.size);
            transplant.cells.fill(background);
            transplant.pos = original.pos;
            transplant.heading = original.heading;
            // Preserve a 33x33 local patch; exterior is the intervention.
            for y in -16..=16 { for x in -16..=16 {
                let p = (start.0 + x, start.1 + y);
                let i = original.index(p);
                transplant.cells[i] = original.cells[i];
            }}
            let mut prefix = 0;
            let mut matching = true;
            for expected in &path {
                transplant.step();
                matching &= (transplant.pos, transplant.heading) == *expected;
                prefix += usize::from(matching);
            }
            println!("{rule},{},{background},{},{},{prefix}", block * window, transplant.pos.0-start.0, transplant.pos.1-start.1);
        }
        selected += 1;
        if selected == 12 { break; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(32))]
        #[test]
        fn matches_oracle(bits in 0u32..4096, n in 2usize..13, steps in 1usize..2000) {
            let rule = Rule::from_bits(bits, n);
            let mut b = Board::new(rule.clone(), 4096);
            let mut oracle = turmite::reference::Ant::new(rule);
            for _ in 0..steps {
                let p = b.pos;
                prop_assert_eq!(b.step(), oracle.colour_at(p));
                oracle.step();
                prop_assert_eq!(b.pos, oracle.pos);
                prop_assert_eq!(b.heading, oracle.heading);
                prop_assert_eq!(b.cells[b.index(p)], oracle.colour_at(p));
            }
        }
    }
}
