//! Extensions beyond the L/R square-lattice ant (frontier items 24-28): multi-state turmites,
//! several ants on one board, turn noise, a random background, hexagonal-lattice and cubic-lattice
//! ants. Each is a small array-based simulator exposing the `Sim` trait; `tests/ext.rs` checks each
//! against an independent HashMap implementation (and the square ones against `reference::Ant`).

use crate::walkers::mix64;
use crate::{DIRS, Rule};

/// What the growth harness needs from a simulator.
pub trait Sim {
    fn step(&mut self);
    /// Cells visited so far by any ant.
    fn area(&self) -> u64;
    /// Largest side of the bounding box of visited cells.
    fn extent(&self) -> u32;
    /// An ant has come within two cells of the board edge.
    fn escaped(&self) -> bool;
}

/// Quarter turns clockwise: 0 = straight (N), 1 = right (R), 2 = u-turn (U), 3 = left (L).
pub const TURN_CHARS: [char; 4] = ['N', 'R', 'U', 'L'];

/// Multi-state turmite: for (state, colour) write a colour, turn, move to a state.
#[derive(Clone, Debug)]
pub struct Turmite {
    pub states: usize,
    pub colours: usize,
    /// Index `state * colours + colour` -> (write colour, turn, next state).
    pub table: Vec<(u8, u8, u8)>,
}

impl Turmite {
    /// The L/R ant: one state, colour c -> c+1 mod n, turn rule[c].
    pub fn from_rule(rule: &Rule) -> Turmite {
        let n = rule.len();
        Turmite {
            states: 1,
            colours: n,
            table: (0..n).map(|c| (((c + 1) % n) as u8, rule.0[c].delta(), 0)).collect(),
        }
    }

    /// Table number `id` in base `4 * colours * states`; digit `e` of entry `i` decodes as
    /// write = e % colours, turn = (e / colours) % 4, next = e / (4 * colours).
    pub fn from_id(states: usize, colours: usize, mut id: u64) -> Turmite {
        let base = (4 * colours * states) as u64;
        let mut table = vec![];
        for _ in 0..states * colours {
            let e = (id % base) as usize;
            id /= base;
            table.push(((e % colours) as u8, ((e / colours) % 4) as u8, (e / (4 * colours)) as u8));
        }
        Turmite { states, colours, table }
    }
}

/// Options for `SqSim`.
#[derive(Clone, Debug, Default)]
pub struct SqOpts {
    /// Number of ants (they step in index order, all within one tick). Starts at offsets
    /// (2i, 0) heading north in state 0. 0 is treated as 1.
    pub ants: usize,
    /// Probability per step that the turn is replaced by a uniformly random L or R. 0 = none.
    pub noise: f64,
    /// Seed for the noise stream.
    pub noise_seed: u64,
    /// If set, the background colours are i.i.d. uniform (hash of this seed and the cell).
    pub bg_seed: Option<u64>,
}

pub fn bg_colour(seed: u64, x: i64, y: i64, colours: usize) -> u8 {
    let key = ((x as i32 as u32 as u64) << 32) | (y as i32 as u32 as u64);
    (mix64(mix64(seed ^ 0x2545_F491_4F6C_DD1D) ^ key) % colours as u64) as u8
}

struct AntState {
    idx: usize,
    h: u8,
    s: u8,
    x: i64,
    y: i64,
}

pub struct SqSim {
    pub tm: Turmite,
    size: usize,
    cells: Vec<u8>,
    vis: Vec<bool>,
    ants: Vec<AntState>,
    area: u64,
    min: (i64, i64),
    max: (i64, i64),
    noise_thr: u64,
    rng: u64,
    escaped: bool,
    pub steps: u64,
}

impl SqSim {
    pub fn new(tm: Turmite, size: usize, o: &SqOpts) -> SqSim {
        let c = (size / 2) as i64;
        let mut cells = vec![0u8; size * size];
        if let Some(seed) = o.bg_seed {
            for (i, cell) in cells.iter_mut().enumerate() {
                *cell = bg_colour(seed, (i % size) as i64 - c, (i / size) as i64 - c, tm.colours);
            }
        }
        let m = o.ants.max(1);
        let mut vis = vec![false; size * size];
        let mut ants = vec![];
        let mut area = 0;
        for i in 0..m {
            let (x, y) = (2 * i as i64, 0i64);
            let idx = ((c + y) as usize) * size + (c + x) as usize;
            if !vis[idx] {
                vis[idx] = true;
                area += 1;
            }
            ants.push(AntState { idx, h: 0, s: 0, x, y });
        }
        let maxx = 2 * (m as i64 - 1);
        SqSim {
            tm,
            size,
            cells,
            vis,
            ants,
            area,
            min: (0, 0),
            max: (maxx, 0),
            noise_thr: if o.noise > 0.0 { (o.noise * 2f64.powi(64)) as u64 } else { 0 },
            rng: o.noise_seed,
            escaped: false,
            steps: 0,
        }
    }

    pub fn cell(&self, x: i64, y: i64) -> u8 {
        let c = (self.size / 2) as i64;
        self.cells[((c + y) as usize) * self.size + (c + x) as usize]
    }

    pub fn positions(&self) -> Vec<(i64, i64)> {
        self.ants.iter().map(|a| (a.x, a.y)).collect()
    }
}

impl Sim for SqSim {
    fn step(&mut self) {
        let lim = (self.size / 2) as i64 - 2;
        let s = self.size as isize;
        for a in self.ants.iter_mut() {
            let col = self.cells[a.idx] as usize;
            let (w, mut turn, ns) = self.tm.table[a.s as usize * self.tm.colours + col];
            if self.noise_thr != 0 {
                self.rng = self.rng.wrapping_add(0x9E37_79B9_7F4A_7C15);
                let r = mix64(self.rng);
                if r < self.noise_thr {
                    turn = if mix64(r ^ 0xD1B5_4A32_D192_ED03) & 1 == 0 { 1 } else { 3 };
                }
            }
            self.cells[a.idx] = w;
            a.s = ns;
            a.h = (a.h + turn) & 3;
            let (dx, dy) = DIRS[a.h as usize];
            a.x += dx as i64;
            a.y += dy as i64;
            a.idx = (a.idx as isize + dy as isize * s + dx as isize) as usize;
            if !self.vis[a.idx] {
                self.vis[a.idx] = true;
                self.area += 1;
                self.min = (self.min.0.min(a.x), self.min.1.min(a.y));
                self.max = (self.max.0.max(a.x), self.max.1.max(a.y));
            }
            if a.x.abs() >= lim || a.y.abs() >= lim {
                self.escaped = true;
            }
        }
        self.steps += 1;
    }
    fn area(&self) -> u64 {
        self.area
    }
    fn extent(&self) -> u32 {
        ((self.max.0 - self.min.0 + 1).max(self.max.1 - self.min.1 + 1)) as u32
    }
    fn escaped(&self) -> bool {
        self.escaped
    }
}

/// Hexagonal-lattice ant in axial coordinates: six neighbours, the heading is one of six
/// directions 60 degrees apart. Rule letters: `r`/`l` turn 60 degrees right/left, `R`/`L` turn
/// 120 degrees; colour c -> c+1 mod n as for the square ant.
pub const HEX_DIRS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

pub fn hex_turns(rule: &str) -> Result<Vec<u8>, String> {
    rule.chars()
        .map(|c| match c {
            'r' => Ok(1),
            'R' => Ok(2),
            'l' => Ok(5),
            'L' => Ok(4),
            other => Err(format!("bad hex turn {other:?}")),
        })
        .collect()
}

pub struct HexSim {
    turns: Vec<u8>,
    size: usize,
    cells: Vec<u8>,
    vis: Vec<bool>,
    idx: usize,
    h: u8,
    pub q: i64,
    pub r: i64,
    area: u64,
    minq: i64,
    maxq: i64,
    minr: i64,
    maxr: i64,
    escaped: bool,
}

impl HexSim {
    pub fn new(turns: Vec<u8>, size: usize) -> HexSim {
        let c = size / 2;
        let idx = c * size + c;
        let mut vis = vec![false; size * size];
        vis[idx] = true;
        HexSim { turns, size, cells: vec![0; size * size], vis, idx, h: 0, q: 0, r: 0, area: 1, minq: 0, maxq: 0, minr: 0, maxr: 0, escaped: false }
    }
}

impl Sim for HexSim {
    fn step(&mut self) {
        let n = self.turns.len() as u8;
        let c = self.cells[self.idx];
        self.h = (self.h + self.turns[c as usize]) % 6;
        self.cells[self.idx] = (c + 1) % n;
        let (dq, dr) = HEX_DIRS[self.h as usize];
        self.q += dq as i64;
        self.r += dr as i64;
        self.idx = (self.idx as isize + dr as isize * self.size as isize + dq as isize) as usize;
        if !self.vis[self.idx] {
            self.vis[self.idx] = true;
            self.area += 1;
            self.minq = self.minq.min(self.q);
            self.maxq = self.maxq.max(self.q);
            self.minr = self.minr.min(self.r);
            self.maxr = self.maxr.max(self.r);
        }
        let lim = (self.size / 2) as i64 - 2;
        if self.q.abs() >= lim || self.r.abs() >= lim {
            self.escaped = true;
        }
    }
    fn area(&self) -> u64 {
        self.area
    }
    fn extent(&self) -> u32 {
        ((self.maxq - self.minq + 1).max(self.maxr - self.minr + 1)) as u32
    }
    fn escaped(&self) -> bool {
        self.escaped
    }
}

/// Cubic-lattice ant. Orientation = (forward f, up u), unit axis vectors; right = f x u.
/// Letters: `L`/`R` yaw 90 degrees left/right about u; `U`/`D` pitch up/down about right.
pub type V3 = [i8; 3];

fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn neg(a: V3) -> V3 {
    [-a[0], -a[1], -a[2]]
}

/// Apply a turn letter to an orientation.
pub fn turn3(c: char, f: V3, u: V3) -> (V3, V3) {
    let right = cross(f, u);
    match c {
        'R' => (right, u),
        'L' => (neg(right), u),
        'U' => (u, neg(f)),
        'D' => (neg(u), f),
        _ => panic!("bad 3d turn {c}"),
    }
}

pub struct CubeSim {
    letters: Vec<char>,
    size: usize,
    cells: Vec<u8>,
    vis: Vec<bool>,
    pub pos: [i64; 3],
    pub f: V3,
    pub u: V3,
    area: u64,
    min: [i64; 3],
    max: [i64; 3],
    escaped: bool,
}

impl CubeSim {
    pub fn new(rule: &str, size: usize) -> CubeSim {
        let mut s = CubeSim {
            letters: rule.chars().collect(),
            size,
            cells: vec![0; size * size * size],
            vis: vec![false; size * size * size],
            pos: [0; 3],
            f: [1, 0, 0],
            u: [0, 0, 1],
            area: 1,
            min: [0; 3],
            max: [0; 3],
            escaped: false,
        };
        let i = s.index();
        s.vis[i] = true;
        s
    }
    fn index(&self) -> usize {
        let c = (self.size / 2) as i64;
        (((self.pos[2] + c) as usize * self.size + (self.pos[1] + c) as usize) * self.size) + (self.pos[0] + c) as usize
    }
}

impl Sim for CubeSim {
    fn step(&mut self) {
        let i = self.index();
        let n = self.letters.len() as u8;
        let c = self.cells[i];
        let (f, u) = turn3(self.letters[c as usize], self.f, self.u);
        self.f = f;
        self.u = u;
        self.cells[i] = (c + 1) % n;
        for a in 0..3 {
            self.pos[a] += f[a] as i64;
        }
        let j = self.index();
        if !self.vis[j] {
            self.vis[j] = true;
            self.area += 1;
            for a in 0..3 {
                self.min[a] = self.min[a].min(self.pos[a]);
                self.max[a] = self.max[a].max(self.pos[a]);
            }
        }
        let lim = (self.size / 2) as i64 - 2;
        if self.pos.iter().any(|p| p.abs() >= lim) {
            self.escaped = true;
        }
    }
    fn area(&self) -> u64 {
        self.area
    }
    fn extent(&self) -> u32 {
        (0..3).map(|a| self.max[a] - self.min[a] + 1).max().unwrap() as u32
    }
    fn escaped(&self) -> bool {
        self.escaped
    }
}

/// (k, area, extent) at t = 2^k for k = min_log2..=max_log2; stops early if an ant escapes.
pub fn growth<S: Sim>(mut sim: S, min_log2: u32, max_log2: u32) -> (Vec<(u32, u64, u32)>, bool) {
    let mut out = vec![];
    let mut t = 0u64;
    for k in min_log2..=max_log2 {
        while t < 1u64 << k {
            sim.step();
            t += 1;
            if sim.escaped() {
                return (out, true);
            }
        }
        out.push((k, sim.area(), sim.extent()));
    }
    (out, false)
}

/// Cubic-lattice rotor-router walker (calibration for `CubeSim`): each site holds an arrow in
/// {+x,+y,+z,-x,-y,-z} (i.i.d. uniform from a seed); a step advances the arrow one place in
/// that cyclic order and moves along it. Rigorous lower bound for the range: c t^(3/4).
pub struct Rotor3 {
    seed: u64,
    size: usize,
    arrow: Vec<u8>,
    vis: Vec<bool>,
    pub pos: [i64; 3],
    area: u64,
    min: [i64; 3],
    max: [i64; 3],
    escaped: bool,
}

pub const DIRS3: [[i64; 3]; 6] = [[1, 0, 0], [0, 1, 0], [0, 0, 1], [-1, 0, 0], [0, -1, 0], [0, 0, -1]];

pub fn arrow3(seed: u64, p: [i64; 3]) -> u8 {
    let k = ((p[0] as i32 as u32 as u64) << 42) ^ ((p[1] as i32 as u32 as u64) << 21) ^ (p[2] as i32 as u32 as u64);
    (mix64(mix64(seed ^ 0x6A09_E667_F3BC_C909) ^ k) % 6) as u8
}

impl Rotor3 {
    pub fn new(seed: u64, size: usize) -> Rotor3 {
        let mut r = Rotor3 { seed, size, arrow: vec![255; size * size * size], vis: vec![false; size * size * size], pos: [0; 3], area: 1, min: [0; 3], max: [0; 3], escaped: false };
        let i = r.index();
        r.arrow[i] = arrow3(seed, [0, 0, 0]);
        r.vis[i] = true;
        r
    }
    fn index(&self) -> usize {
        let c = (self.size / 2) as i64;
        (((self.pos[2] + c) as usize * self.size + (self.pos[1] + c) as usize) * self.size) + (self.pos[0] + c) as usize
    }
}

impl Sim for Rotor3 {
    fn step(&mut self) {
        let i = self.index();
        let a = (self.arrow[i] + 1) % 6;
        self.arrow[i] = a;
        for k in 0..3 {
            self.pos[k] += DIRS3[a as usize][k];
        }
        let j = self.index();
        if self.arrow[j] == 255 {
            self.arrow[j] = arrow3(self.seed, self.pos);
        }
        if !self.vis[j] {
            self.vis[j] = true;
            self.area += 1;
            for k in 0..3 {
                self.min[k] = self.min[k].min(self.pos[k]);
                self.max[k] = self.max[k].max(self.pos[k]);
            }
        }
        let lim = (self.size / 2) as i64 - 2;
        if self.pos.iter().any(|p| p.abs() >= lim) {
            self.escaped = true;
        }
    }
    fn area(&self) -> u64 {
        self.area
    }
    fn extent(&self) -> u32 {
        (0..3).map(|k| self.max[k] - self.min[k] + 1).max().unwrap() as u32
    }
    fn escaped(&self) -> bool {
        self.escaped
    }
}

/// Triangular-cell ant (frontier item 24): the ant lives on the triangles of the regular triangular tiling, each with
/// three neighbours. Triangle (x, y) points up iff x + y is even; its neighbours are (x-1, y), (x+1, y) and (x, y+1) if it
/// points up, (x-1, y), (x+1, y) and (x, y-1) if it points down. The ant enters a triangle through one edge, reads the
/// colour, and leaves through one of the other two: `L` the next edge counter-clockwise, `R` the next clockwise (so
/// there is no straight-on move, as for the square ant). Edge normals are indexed by angle/60 degrees minus the offset:
/// index i has normal angle 30 + 60 i degrees; up triangles own the even indices (30, 150, 270), down triangles the odd ones.
pub struct TriSim {
    turn: Vec<i8>,
    size: usize,
    cells: Vec<u8>,
    vis: Vec<bool>,
    pub x: i64,
    pub y: i64,
    /// Index of the edge the ant entered through (its normal on the current triangle).
    pub entry: u8,
    area: u64,
    min: (i64, i64),
    max: (i64, i64),
    escaped: bool,
}

impl TriSim {
    pub fn new(rule: &Rule, size: usize) -> TriSim {
        let c = size / 2;
        let c = c - c % 2; // start on an up triangle
        let mut vis = vec![false; size * size];
        vis[c * size + c] = true;
        TriSim {
            turn: rule.0.iter().map(|t| if matches!(t, crate::Turn::L) { 2 } else { -2 }).collect(),
            size,
            cells: vec![0; size * size],
            vis,
            x: c as i64,
            y: c as i64,
            entry: 4,
            area: 1,
            min: (c as i64, c as i64),
            max: (c as i64, c as i64),
            escaped: false,
        }
    }
    pub fn position(&self) -> (i64, i64) {
        let c = ((self.size / 2) - (self.size / 2) % 2) as i64;
        (self.x - c, self.y - c)
    }
}

/// Offset of the neighbour across the edge with normal index `e` from a triangle of the given orientation.
pub fn tri_step(up: bool, e: u8) -> (i64, i64) {
    match (up, e) {
        (true, 0) => (1, 0),
        (true, 2) => (-1, 0),
        (true, 4) => (0, 1),
        (false, 1) => (0, -1),
        (false, 3) => (-1, 0),
        (false, 5) => (1, 0),
        _ => panic!("edge {e} does not belong to a triangle pointing {}", if up { "up" } else { "down" }),
    }
}

impl Sim for TriSim {
    fn step(&mut self) {
        let i = self.y as usize * self.size + self.x as usize;
        let c = self.cells[i];
        let n = self.turn.len() as u8;
        let exit = (self.entry as i8 + self.turn[c as usize]).rem_euclid(6) as u8;
        self.cells[i] = if c + 1 == n { 0 } else { c + 1 };
        let up = (self.x + self.y) % 2 == 0;
        let (dx, dy) = tri_step(up, exit);
        self.x += dx;
        self.y += dy;
        self.entry = (exit + 3) % 6;
        let j = self.y as usize * self.size + self.x as usize;
        if !self.vis[j] {
            self.vis[j] = true;
            self.area += 1;
            self.min = (self.min.0.min(self.x), self.min.1.min(self.y));
            self.max = (self.max.0.max(self.x), self.max.1.max(self.y));
        }
        let m = 2i64;
        if self.x < m || self.y < m || self.x >= self.size as i64 - m || self.y >= self.size as i64 - m {
            self.escaped = true;
        }
    }
    fn area(&self) -> u64 {
        self.area
    }
    fn extent(&self) -> u32 {
        // width in cells counts two columns per unit edge; the row count counts rows of height sqrt(3)/2: report the larger physical size
        let w = (self.max.0 - self.min.0 + 1) as f64 * 0.5;
        let h = (self.max.1 - self.min.1 + 1) as f64 * 0.866;
        w.max(h).ceil() as u32
    }
    fn escaped(&self) -> bool {
        self.escaped
    }
}
