// Turmite simulator for the page. Same conventions as the Rust crate:
// heading 0=N,1=E,2=S,3=W; read colour c, turn by rule[c], set c+1 mod n, move.
const DX = [0, 1, 0, -1], DY = [-1, 0, 1, 0];

class Ant {
  // track = true also keeps the "cup" bookkeeping of src/probe.rs: which cells were ever visited, how many
  // of their neighbours are still fresh (unv), the area A, the edge length P, and per-octave counters
  // for c = (new cells) / (steps starting on an edge cell) and kappa = (edge-step fraction) / (P / A).
  constructor(rule, size = 2048, track = false) {
    this.rule = rule;
    this.n = rule.length;
    this.size = size;
    this.cells = new Uint8Array(size * size);
    this.turn = Uint8Array.from(rule, ch => (ch === 'R' ? 1 : 3));
    this.next = Uint8Array.from({ length: this.n }, (_, i) => (i + 1) % this.n);
    const c = size >> 1;
    this.x = c; this.y = c; this.h = 0; this.steps = 0;
    this.minx = c; this.maxx = c; this.miny = c; this.maxy = c;
    this.escaped = false;
    this.track = track;
    if (track) {
      this.unv = new Uint8Array(size * size).fill(255); // 255 = never visited, else count of fresh neighbours
      this.area = 0; this.perim = 0;
      this.win = { boundary: 0, fresh: 0, steps: 0 };   // current octave (2^(j-1), 2^j]
      this.last = null;                                  // last completed octave
      this.mark = 1024;
      this.hist = [];                                    // [steps, area] at each octave end
      this.visit(c * size + c);
    }
  }
  visit(idx) {
    const s = this.size, unv = this.unv;
    let u = 0;
    for (const d of [-s, 1, s, -1]) {
      const j = idx + d;
      if (unv[j] === 255) u++;
      else { unv[j]--; if (unv[j] === 0) this.perim--; }
    }
    unv[idx] = u;
    if (u > 0) this.perim++;
    this.area++;
  }
  closeWindow() {
    const w = this.win;
    this.last = {
      end: this.steps, area: this.area, perim: this.perim,
      c: w.boundary ? w.fresh / w.boundary : 0,
      kappa: (w.boundary / w.steps) / (this.perim / this.area),
      rate: w.fresh / w.steps,                        // new cells per step, observed
    };
    this.hist.push([this.steps, this.area]);
    this.win = { boundary: 0, fresh: 0, steps: 0 };
    this.mark *= 2;
  }
  // Slope of log A against log t over the last two completed octaves (area exponent, 2/3 for the cup picture).
  exponent() {
    const h = this.hist;
    if (h.length < 3) return null;
    const [t0, a0] = h[h.length - 3], [t1, a1] = h[h.length - 1];
    return Math.log(a1 / a0) / Math.log(t1 / t0);
  }
  run(k) {
    if (this.escaped) return;
    if (this.track) return this.runTracked(k);
    const { cells, turn, next, size } = this;
    let { x, y, h, minx, maxx, miny, maxy } = this;
    let i = 0;
    for (; i < k; i++) {
      const idx = y * size + x;
      const col = cells[idx];
      h = (h + turn[col]) & 3;
      cells[idx] = next[col];
      x += DX[h]; y += DY[h];
      if (x < 0 || y < 0 || x >= size || y >= size) { this.escaped = true; i++; x -= DX[h]; y -= DY[h]; break; }
      if (x < minx) minx = x; else if (x > maxx) maxx = x;
      if (y < miny) miny = y; else if (y > maxy) maxy = y;
    }
    Object.assign(this, { x, y, h, minx, maxx, miny, maxy });
    this.steps += i;
  }
  runTracked(k) {
    const { cells, turn, next, size, unv } = this;
    let { x, y, h } = this;
    for (let i = 0; i < k; i++) {
      const idx = y * size + x;
      if (unv[idx] > 0) this.win.boundary++;           // the ant stands on an edge cell (it is always visited)
      const col = cells[idx];
      h = (h + turn[col]) & 3;
      cells[idx] = next[col];
      x += DX[h]; y += DY[h];
      if (x < 1 || y < 1 || x >= size - 1 || y >= size - 1) { this.escaped = true; x -= DX[h]; y -= DY[h]; break; }
      const j = y * size + x;
      if (unv[j] === 255) { this.visit(j); this.win.fresh++; if (x < this.minx) this.minx = x; else if (x > this.maxx) this.maxx = x; if (y < this.miny) this.miny = y; else if (y > this.maxy) this.maxy = y; }
      this.steps++; this.win.steps++;
      if (this.steps === this.mark) this.closeWindow();
    }
    this.x = x; this.y = y; this.h = h;
  }
}

if (typeof module !== 'undefined') module.exports = { Ant };
