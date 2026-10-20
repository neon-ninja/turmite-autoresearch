//! Minimal little-endian binary writer/reader for on-disk checkpoints (no serde: the crate stays
//! dependency-light). Layout is private to the checkpointing code that uses it.

#[derive(Default)]
pub struct Wr(pub Vec<u8>);

impl Wr {
    pub fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    pub fn bool(&mut self, v: bool) {
        self.0.push(v as u8);
    }
    pub fn u64(&mut self, v: u64) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    pub fn i64(&mut self, v: i64) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    pub fn f64(&mut self, v: f64) {
        self.0.extend_from_slice(&v.to_bits().to_le_bytes());
    }
    pub fn opt_u64(&mut self, v: Option<u64>) {
        self.bool(v.is_some());
        self.u64(v.unwrap_or(0));
    }
    pub fn bytes(&mut self, v: &[u8]) {
        self.u64(v.len() as u64);
        self.0.extend_from_slice(v);
    }
    pub fn u64s(&mut self, v: &[u64]) {
        self.u64(v.len() as u64);
        for &x in v {
            self.u64(x);
        }
    }
}

pub struct Rd<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Rd<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Rd { b, p: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self.p.checked_add(n).filter(|&e| e <= self.b.len()).ok_or("checkpoint truncated")?;
        let s = &self.b[self.p..end];
        self.p = end;
        Ok(s)
    }
    pub fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    pub fn bool(&mut self) -> Result<bool, String> {
        Ok(self.u8()? != 0)
    }
    pub fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    pub fn i64(&mut self) -> Result<i64, String> {
        Ok(i64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    pub fn f64(&mut self) -> Result<f64, String> {
        Ok(f64::from_bits(self.u64()?))
    }
    pub fn opt_u64(&mut self) -> Result<Option<u64>, String> {
        let some = self.bool()?;
        let v = self.u64()?;
        Ok(some.then_some(v))
    }
    pub fn bytes(&mut self) -> Result<&'a [u8], String> {
        let n = self.u64()? as usize;
        self.take(n)
    }
    pub fn u64s(&mut self) -> Result<Vec<u64>, String> {
        let n = self.u64()? as usize;
        (0..n).map(|_| self.u64()).collect()
    }
    pub fn done(&self) -> bool {
        self.p == self.b.len()
    }
}

/// Rows `rlo..=rhi`, columns `clo..=chi` of a row-major `size`-wide board.
pub fn put_crop(w: &mut Wr, board: &[u8], size: usize, bbox: Option<(usize, usize, usize, usize)>) {
    match bbox {
        None => w.bool(false),
        Some((rlo, rhi, clo, chi)) => {
            w.bool(true);
            for v in [rlo, rhi, clo, chi] {
                w.u64(v as u64);
            }
            for r in rlo..=rhi {
                w.0.extend_from_slice(&board[r * size + clo..=r * size + chi]);
            }
        }
    }
}

pub fn get_crop(r: &mut Rd, board: &mut [u8], size: usize) -> Result<(), String> {
    if !r.bool()? {
        return Ok(());
    }
    let (rlo, rhi, clo, chi) = (r.u64()? as usize, r.u64()? as usize, r.u64()? as usize, r.u64()? as usize);
    if rhi >= size || chi >= size || rlo > rhi || clo > chi {
        return Err("checkpoint bounding box out of range".into());
    }
    for row in rlo..=rhi {
        let n = chi - clo + 1;
        board[row * size + clo..=row * size + chi].copy_from_slice(r.take(n)?);
    }
    Ok(())
}
