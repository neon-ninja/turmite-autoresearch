use rayon::prelude::*;
use std::io::Write;
use turmite::probe::probe_walker;
use turmite::walkers::{PatchedAnt, RandomWalk, Rotor, Walker, splitmix_next};
use turmite::{DIRS, Rule};

const UNV: u8 = u8::MAX;

struct Region {
    unv: Vec<u8>,
    nbr: [isize; 4],
    area: u64,
    /// pc[u]: visited cells with exactly u unvisited neighbours (pc[0] = interior).
    pc: [u64; 5],
}

impl Region {
    fn new(size: usize) -> Region {
        let s = size as isize;
        Region { unv: vec![UNV; size * size], nbr: [-s, 1, s, -1], area: 0, pc: [0; 5] }
    }
    fn visited(&self, idx: usize) -> bool {
        self.unv[idx] != UNV
    }
    fn visit(&mut self, idx: usize) {
        let mut u = 0u8;
        for d in self.nbr {
            let j = (idx as isize + d) as usize;
            if self.unv[j] == UNV {
                u += 1;
            } else {
                self.pc[self.unv[j] as usize] -= 1;
                self.unv[j] -= 1;
                self.pc[self.unv[j] as usize] += 1;
            }
        }
        self.unv[idx] = u;
        self.pc[u as usize] += 1;
        self.area += 1;
    }
    fn shares(&self) -> [f64; 5] {
        let a = self.area as f64;
        [0.0, self.pc[1] as f64 / a, self.pc[2] as f64 / a, self.pc[3] as f64 / a, self.pc[4] as f64 / a]
    }
    fn scan(&self) -> [u64; 5] {
        let mut pc = [0u64; 5];
        for &u in &self.unv {
            if u != UNV {
                pc[u as usize] += 1;
            }
        }
        pc
    }
}

#[derive(Clone, Debug, Default)]
struct Win {
    k: u32,
    area: u64,
    p: [u64; 5],
    new_cells: u64,
    b: [u64; 5],
    f: [u64; 5],
    m: [f64; 5],
}

struct Out {
    wins: Vec<Win>,
    escaped: bool,
}

/// One step of the walker: given the step-start cell index, return the step-end index.
trait Mover {
    fn start_cell(&mut self, cur: usize, region: &Region) -> usize;
    fn advance(&mut self, at: usize, colour: &mut [u8], size: usize) -> (usize, i64, i64);
}

struct WalkerMover<W: Walker> {
    w: W,
    h: u8,
    x: i64,
    y: i64,
}

impl<W: Walker> Mover for WalkerMover<W> {
    fn start_cell(&mut self, cur: usize, _: &Region) -> usize {
        cur
    }
    fn advance(&mut self, at: usize, colour: &mut [u8], size: usize) -> (usize, i64, i64) {
        self.w.step(&mut colour[at], &mut self.h);
        let (dx, dy) = DIRS[self.h as usize];
        self.x += dx as i64;
        self.y += dy as i64;
        let s = size as isize;
        ((at as isize + dy as isize * s + dx as isize) as usize, self.x, self.y)
    }
}

struct MeanField {
    st: u64,
    cells: Vec<usize>,
    r: u64,
}

impl Mover for MeanField {
    fn start_cell(&mut self, _: usize, _: &Region) -> usize {
        self.r = splitmix_next(&mut self.st);
        self.cells[((self.r >> 8) % self.cells.len() as u64) as usize]
    }
    fn advance(&mut self, at: usize, _: &mut [u8], size: usize) -> (usize, i64, i64) {
        let s = size as isize;
        let d = [-s, 1, s, -1][(self.r & 3) as usize];
        let j = (at as isize + d) as usize;
        let c = (size / 2) as i64;
        let (x, y) = ((j % size) as i64 - c, (j / size) as i64 - c);
        (j, x, y)
    }
}

fn run<M: Mover>(mut mv: M, mut colour: Vec<u8>, size: usize, max_log2: u32, check: bool) -> Out {
    let c = (size / 2) as i64;
    let mut region = Region::new(size);
    let mut idx = (c * size as i64 + c) as usize;
    region.visit(idx);
    let mut share = region.shares();
    let mut newborn: usize = usize::MAX;
    let mut wins = vec![];
    let mut escaped = false;
    let mut t: u64 = 0;
    for k in 0..max_log2 {
        let t_end = 1u64 << (k + 1);
        let mut w = Win { k, ..Default::default() };
        while t < t_end {
            let at = mv.start_cell(idx, &region);
            let u = region.unv[at] as usize;
            for v in 1..5 {
                w.m[v] += share[v];
            }
            if u > 0 {
                w.b[u] += 1;
                if at == newborn {
                    w.f[u] += 1;
                }
            }
            let (j, x, y) = mv.advance(at, &mut colour, size);
            idx = j;
            t += 1;
            newborn = usize::MAX;
            if !region.visited(idx) {
                w.new_cells += 1;
                region.visit(idx);
                share = region.shares();
                newborn = idx;
            }
            if x.abs() >= c - 2 || y.abs() >= c - 2 {
                escaped = true;
                break;
            }
        }
        if escaped {
            break;
        }
        w.area = region.area;
        w.p = region.pc;
        if check {
            assert_eq!(region.scan(), region.pc, "class counts disagree with a board scan at k={k}");
        }
        if k >= 8 {
            wins.push(w);
        }
    }
    Out { wins, escaped }
}

/// The mean-field walker needs the growing cell list inside the mover; run it with its own loop
/// wrapper so the list stays in sync (cells are appended in visit order).
fn run_mf(seed: u64, size: usize, max_log2: u32, check: bool) -> Out {
    // Seed mixing as in examples/kappa_uniform_baseline.rs.
    let st = seed.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ 0x2545_F491_4F6C_DD1D;
    let c = size / 2;
    let start = c * size + c;
    let mv = MeanFieldSync { inner: MeanField { st, cells: vec![start], r: 0 }, seen: 1 };
    run(mv, vec![0u8; size * size], size, max_log2, check)
}

/// MeanField whose cell list is refreshed from the region's visit order: a cell is new exactly when
/// the region's area grows, and the only cell that can become visited in a step is the step's end.
struct MeanFieldSync {
    inner: MeanField,
    seen: u64,
}

impl Mover for MeanFieldSync {
    fn start_cell(&mut self, cur: usize, region: &Region) -> usize {
        if region.area > self.seen {
            debug_assert_eq!(region.area, self.seen + 1);
            self.inner.cells.push(cur);
            self.seen = region.area;
        }
        self.inner.start_cell(cur, region)
    }
    fn advance(&mut self, at: usize, colour: &mut [u8], size: usize) -> (usize, i64, i64) {
        self.inner.advance(at, colour, size)
    }
}

fn walker_out<W: Walker>(w: W, size: usize, max_log2: u32, check: bool) -> Out {
    let c = (size / 2) as i64;
    let mut colour = vec![0u8; size * size];
    if W::RANDOM_INIT {
        for (i, cell) in colour.iter_mut().enumerate() {
            *cell = w.initial_cell((i % size) as i64 - c, (i / size) as i64 - c);
        }
    }
    run(WalkerMover { w, h: 0, x: 0, y: 0 }, colour, size, max_log2, check)
}

fn check_against_probe<W: Walker>(w: W, out: &Out, size: usize, max_log2: u32) {
    let pr = probe_walker(w, size, 8, max_log2);
    assert_eq!(pr.escaped, out.escaped, "escape flag");
    assert_eq!(pr.windows.len(), out.wins.len(), "window count");
    for (a, b) in pr.windows.iter().zip(&out.wins) {
        let p: u64 = b.p[1..].iter().sum();
        let bs: u64 = b.b[1..].iter().sum();
        let fresh: u64 = (1..5).map(|u| u as u64 * b.b[u]).sum();
        assert_eq!(a.t_end.trailing_zeros(), b.k + 1, "k");
        assert_eq!((a.area, a.perimeter, a.new_cells, a.boundary_steps, a.fresh_nbrs), (b.area, p, b.new_cells, bs, fresh), "window k={}", b.k);
    }
}
