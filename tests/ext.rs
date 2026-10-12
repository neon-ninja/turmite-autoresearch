//! Oracle tests for the extensions in `src/ext.rs`: independent HashMap versions.
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use turmite::ext::*;
use turmite::reference::Ant;
use turmite::walkers::mix64;
use turmite::{DIRS, Rule};

const SIZE: usize = 512;

/// Obvious multi-ant turmite with optional noise and random background.
fn ref_sq(tm: &Turmite, o: &SqOpts, steps: usize) -> (Vec<Vec<(i64, i64)>>, usize) {
    let m = o.ants.max(1);
    let mut cells: HashMap<(i64, i64), u8> = HashMap::new();
    let mut ants: Vec<((i64, i64), u8, u8)> = (0..m).map(|i| ((2 * i as i64, 0), 0u8, 0u8)).collect();
    let mut seen: HashSet<(i64, i64)> = ants.iter().map(|a| a.0).collect();
    let mut rng = o.noise_seed;
    let thr = if o.noise > 0.0 { (o.noise * 2f64.powi(64)) as u64 } else { 0 };
    let mut log = vec![];
    for _ in 0..steps {
        for a in ants.iter_mut() {
            let col = *cells.entry(a.0).or_insert_with(|| match o.bg_seed {
                Some(s) => bg_colour(s, a.0 .0, a.0 .1, tm.colours),
                None => 0,
            });
            let (w, mut turn, ns) = tm.table[a.2 as usize * tm.colours + col as usize];
            if thr != 0 {
                rng = rng.wrapping_add(0x9E37_79B9_7F4A_7C15);
                let r = mix64(rng);
                if r < thr {
                    turn = if mix64(r ^ 0xD1B5_4A32_D192_ED03) & 1 == 0 { 1 } else { 3 };
                }
            }
            cells.insert(a.0, w);
            a.2 = ns;
            a.1 = (a.1 + turn) & 3;
            let d = DIRS[a.1 as usize];
            a.0 = (a.0 .0 + d.0 as i64, a.0 .1 + d.1 as i64);
            seen.insert(a.0);
        }
        log.push(ants.iter().map(|a| a.0).collect());
    }
    (log, seen.len())
}

fn check_sq(tm: Turmite, o: SqOpts, steps: usize) -> Result<(), TestCaseError> {
    let (log, area) = ref_sq(&tm, &o, steps);
    let mut s = SqSim::new(tm, SIZE, &o);
    for l in &log {
        s.step();
        prop_assert_eq!(&s.positions(), l);
        if s.escaped() { return Ok(()); }
    }
    prop_assert_eq!(s.area() as usize, area);
    Ok(())
}

proptest! {
    #[test]
    fn sq_matches_reference_ant(bits in 1u32..1024, n in 2usize..10, steps in 1usize..2500) {
        let rule = Rule::from_bits(bits, n);
        let tm = Turmite::from_rule(&rule);
        let mut a = Ant::new(rule);
        let mut s = SqSim::new(tm, SIZE, &SqOpts::default());
        for _ in 0..steps {
            a.step();
            s.step();
            prop_assert_eq!(s.positions()[0], (a.pos.0 as i64, a.pos.1 as i64));
            if s.escaped() { return Ok(()); }
        }
        prop_assert_eq!(s.cell(a.pos.0 as i64, a.pos.1 as i64), a.colour_at(a.pos));
    }

    #[test]
    fn turmite_tables(states in 1usize..4, colours in 2usize..4, id in 0u64..u64::MAX / 2, steps in 1usize..1500) {
        check_sq(Turmite::from_id(states, colours, id), SqOpts::default(), steps)?;
    }

    #[test]
    fn many_ants(bits in 1u32..256, n in 2usize..8, ants in 1usize..5, steps in 1usize..1500) {
        let tm = Turmite::from_rule(&Rule::from_bits(bits, n));
        check_sq(tm, SqOpts { ants, ..Default::default() }, steps)?;
    }

    #[test]
    fn noise_and_background(bits in 1u32..256, n in 2usize..8, seed in 0u64..1000, noise in 0.0f64..0.3, bg in proptest::bool::ANY, steps in 1usize..1500) {
        let tm = Turmite::from_rule(&Rule::from_bits(bits, n));
        let o = SqOpts { ants: 1, noise, noise_seed: seed, bg_seed: if bg { Some(seed ^ 77) } else { None } };
        check_sq(tm, o, steps)?;
    }

    #[test]
    fn hex_matches_reference(letters in proptest::collection::vec(0usize..4, 2..8), steps in 1usize..3000) {
        let rule: String = letters.iter().map(|&i| ['r', 'R', 'l', 'L'][i]).collect();
        let turns = hex_turns(&rule).unwrap();
        let mut s = HexSim::new(turns.clone(), SIZE);
        let mut cells: HashMap<(i64, i64), u8> = HashMap::new();
        let (mut p, mut h) = ((0i64, 0i64), 0u8);
        let mut seen = HashSet::from([p]);
        for _ in 0..steps {
            let c = *cells.get(&p).unwrap_or(&0);
            h = (h + turns[c as usize]) % 6;
            cells.insert(p, (c + 1) % turns.len() as u8);
            let d = HEX_DIRS[h as usize];
            p = (p.0 + d.0 as i64, p.1 + d.1 as i64);
            seen.insert(p);
            s.step();
            prop_assert_eq!((s.q, s.r), p);
            if s.escaped() { return Ok(()); }
        }
        prop_assert_eq!(s.area() as usize, seen.len());
    }

    #[test]
    fn cube_matches_reference(letters in proptest::collection::vec(0usize..4, 2..8), steps in 1usize..3000) {
        let rule: String = letters.iter().map(|&i| ['L', 'R', 'U', 'D'][i]).collect();
        let mut s = CubeSim::new(&rule, 128);
        // reference: rotate with explicit 3x3 matrices instead of (forward, up) vectors
        let mut cells: HashMap<[i64; 3], u8> = HashMap::new();
        let mut p = [0i64; 3];
        // rows: forward, up, right as columns of a rotation matrix
        let mut m = [[1i64, 0, 0], [0, 0, 1], [0, -1, 0]]; // f = x, u = z, right = f x u = -y
        let mut seen = HashSet::from([p]);
        let rules: Vec<char> = rule.chars().collect();
        for _ in 0..steps {
            let c = *cells.get(&p).unwrap_or(&0);
            let (f, u, r) = (m[0], m[1], m[2]);
            m = match rules[c as usize] {
                'R' => [r, u, [-f[0], -f[1], -f[2]]],
                'L' => [[-r[0], -r[1], -r[2]], u, f],
                'U' => [u, [-f[0], -f[1], -f[2]], r],
                'D' => [[-u[0], -u[1], -u[2]], f, r],
                _ => unreachable!(),
            };
            cells.insert(p, (c + 1) % rules.len() as u8);
            for a in 0..3 {
                p[a] += m[0][a];
            }
            seen.insert(p);
            s.step();
            prop_assert_eq!(s.pos, p);
            if s.escaped() { return Ok(()); }
            prop_assert_eq!(s.f.map(|x| x as i64), m[0]);
            prop_assert_eq!(s.u.map(|x| x as i64), m[1]);
        }
        prop_assert_eq!(s.area() as usize, seen.len());
    }
}

#[test]
fn turn3_group_laws() {
    let (f, u) = ([1i8, 0, 0], [0i8, 0, 1]);
    let mut o = (f, u);
    for _ in 0..4 {
        o = turn3('R', o.0, o.1);
    }
    assert_eq!(o, (f, u));
    let o = turn3('L', turn3('R', f, u).0, turn3('R', f, u).1);
    assert_eq!(o, (f, u));
    let a = turn3('U', f, u);
    assert_eq!(turn3('D', a.0, a.1), (f, u));
}

proptest! {
    #[test]
    fn rotor3_matches_reference(seed in 0u64..1000, steps in 1usize..4000) {
        let mut s = Rotor3::new(seed, 128);
        let mut arrow: HashMap<[i64; 3], u8> = HashMap::new();
        let mut p = [0i64; 3];
        let mut seen = HashSet::from([p]);
        for _ in 0..steps {
            let a = (*arrow.entry(p).or_insert_with(|| arrow3(seed, p)) + 1) % 6;
            arrow.insert(p, a);
            for k in 0..3 { p[k] += DIRS3[a as usize][k]; }
            seen.insert(p);
            s.step();
            prop_assert_eq!(s.pos, p);
            if s.escaped() { return Ok(()); }
        }
        prop_assert_eq!(s.area() as usize, seen.len());
    }
}

/// Independent geometric reference for the triangular-cell ant: Cartesian centroids and normal vectors.
fn tri_centroid(x: i64, y: i64) -> (f64, f64) {
    let h = 3f64.sqrt() / 2.0;
    let up = (x + y) % 2 == 0;
    // Row y spans heights y*h .. (y+1)*h (y grows downward in the picture, as in the lattice); an up triangle has its
    // flat edge at the bottom of the row, so its centroid is 2/3 h from the top; a down triangle 1/3 h from the top.
    (x as f64 * 0.5, y as f64 * h + if up { 2.0 * h / 3.0 } else { h / 3.0 })
}

proptest! {
    #[test]
    fn tri_matches_geometric_reference(bits in 1u32..256, n in 2usize..8, steps in 1usize..4000) {
        let rule = Rule::from_bits(bits, n);
        let mut s = TriSim::new(&rule, 512);
        let c = 256i64;
        let (mut x, mut y) = (c, c);
        // entry direction as a unit vector: the ant starts having come in through the bottom edge of an up triangle
        let mut entry_angle: f64 = 270.0;
        let mut cells: HashMap<(i64, i64), u8> = HashMap::new();
        let mut seen = HashSet::from([(x, y)]);
        for _ in 0..steps {
            let col = *cells.get(&(x, y)).unwrap_or(&0);
            let l = matches!(rule.0[col as usize], turmite::Turn::L);
            let exit_angle = (entry_angle + if l { 120.0 } else { -120.0 }).rem_euclid(360.0);
            cells.insert((x, y), (col + 1) % n as u8);
            // the neighbour is the triangle whose centroid lies 2 * inradius along the exit normal
            let (cx, cy) = tri_centroid(x, y);
            let d = 3f64.sqrt() / 3.0; // 2 * inradius for unit edges
            // lattice y grows downward, so a normal at +90 degrees (up in the picture) is -y
            let (tx, ty) = (cx + d * exit_angle.to_radians().cos(), cy - d * exit_angle.to_radians().sin());
            let found: Vec<(i64, i64)> = [(-1, 0), (1, 0), (0, 1), (0, -1)].iter().map(|(dx, dy)| (x + dx, y + dy)).filter(|&(nx, ny)| {
                let (qx, qy) = tri_centroid(nx, ny);
                (qx - tx).abs() < 1e-9 && (qy - ty).abs() < 1e-9
            }).collect();
            prop_assert_eq!(found.len(), 1);
            (x, y) = found[0];
            entry_angle = (exit_angle + 180.0).rem_euclid(360.0);
            seen.insert((x, y));
            s.step();
            prop_assert_eq!((s.x, s.y), (x, y));
            if s.escaped() { return Ok(()); }
        }
        prop_assert_eq!(s.area() as usize, seen.len());
    }
}
