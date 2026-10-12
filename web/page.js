(() => {
  const $ = (s) => document.querySelector(s);
  const fmt = (n) => Number(n).toLocaleString('en-NZ');
  const BOARD = 2048, MAX_BOARD = 8192;

  // ---------- colours from theme tokens ----------
  let theme = {};
  function readTheme() {
    const cs = getComputedStyle(document.documentElement);
    const hex = (v) => {
      const s = cs.getPropertyValue(v).trim();
      const m = s.match(/^#([0-9a-f]{6})$/i);
      return m ? [parseInt(m[1].slice(0, 2), 16), parseInt(m[1].slice(2, 4), 16), parseInt(m[1].slice(4, 6), 16)] : [128, 128, 128];
    };
    theme = { panel: hex('--panel'), ink: hex('--ink'), stops: ['--c-indigo', '--c-teal', '--c-amber', '--c-rose'].map(hex) };
    theme.edge = hex('--c-rose'); // highlight colour for the edge cells in the cup overlay
  }
  function palette(n) {
    // Colour 0 is the empty board; colours 1..n-1 walk along the ramp.
    const out = [theme.panel];
    for (let c = 1; c < n; c++) {
      const t = n <= 2 ? 0 : (c - 1) / (n - 2);
      const pos = t * (theme.stops.length - 1);
      const i = Math.min(Math.floor(pos), theme.stops.length - 2);
      const f = pos - i;
      const a = theme.stops[i], b = theme.stops[i + 1];
      out.push([0, 1, 2].map((k) => Math.round(a[k] * (1 - f) + b[k] * f)));
    }
    return out;
  }

  // ---------- simulator ----------
  const canvas = $('#board');
  const ctx = canvas.getContext('2d');
  const W = canvas.width, H = canvas.height;
  const img = ctx.createImageData(W, H);
  let ant = null, pal = null, paused = false, ffTarget = 0, note = '';

  function parseRule(s) {
    const r = s.trim().toUpperCase();
    if (!/^[LR]{2,32}$/.test(r)) return null;
    return r;
  }

  let lastOpts = {};
  // "Cup" overlay: edge cells and the c, kappa, P/A bookkeeping of the mechanism (toggling restarts the run,
  // because the visited set has to be tracked from step 0).
  let mechOn = false;
  function load(rule, opts = {}) {
    lastOpts = opts;
    ant = new Ant(rule, opts.size || BOARD, mechOn);
    pal = palette(rule.length);
    $('#rule-input').value = rule;
    $('#hud-rule').textContent = rule;
    $('#err').textContent = '';
    ffTarget = opts.ff || 0;
    note = ffTarget ? 'fast-forwarding…' : '';
    if (opts.speed != null) setSpeed(opts.speed);
    paused = false;
    $('#btn-pause').textContent = 'Pause';
  }

  let speedExp = 1.6;
  function setSpeed(v) {
    speedExp = v;
    $('#speed').value = v;
    $('#speed-val').textContent = fmt(Math.round(10 ** v)) + '/frame';
  }

  function draw() {
    const data = img.data;
    // Square view around everything touched, with a margin; at least 48 cells.
    const w = ant.maxx - ant.minx + 1, h = ant.maxy - ant.miny + 1;
    const side = Math.max(48, Math.ceil(Math.max(w, h) * 1.08));
    const cx = (ant.minx + ant.maxx) / 2, cy = (ant.miny + ant.maxy) / 2;
    const x0 = cx - side / 2, y0 = cy - side / 2;
    const scale = side / W;
    const cells = ant.cells, size = ant.size;
    for (let py = 0; py < H; py++) {
      const gy = Math.floor(y0 + (py + 0.5) * scale);
      const rowOk = gy >= 0 && gy < size;
      const row = gy * size;
      for (let px = 0; px < W; px++) {
        const gx = Math.floor(x0 + (px + 0.5) * scale);
        const c = rowOk && gx >= 0 && gx < size ? cells[row + gx] : 0;
        let col = pal[c];
        if (mechOn && rowOk && gx >= 0 && gx < size) {
          const u = ant.unv[row + gx];
          if (u !== 255 && u > 0) col = theme.edge; // visited cell with a fresh neighbour: the cup wall
        }
        const o = (py * W + px) * 4;
        data[o] = col[0]; data[o + 1] = col[1]; data[o + 2] = col[2]; data[o + 3] = 255;
      }
    }
    ctx.putImageData(img, 0, 0);
    // Mark the ant.
    const ax = (ant.x - x0 + 0.5) / scale, ay = (ant.y - y0 + 0.5) / scale;
    ctx.strokeStyle = `rgb(${theme.ink.join(',')})`;
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    ctx.arc(ax, ay, Math.max(3, 0.7 / scale), 0, Math.PI * 2);
    ctx.stroke();
    $('#hud-steps').textContent = fmt(ant.steps) + ' steps';
    $('#hud-note').textContent = note;
    if (mechOn) mechReadout();
  }

  // Live numbers of the cup picture: dA/dt = c * kappa * (P/A), per step, for the last finished octave.
  function mechReadout() {
    const el = $('#mech-readout');
    const L = ant.last, e = ant.exponent();
    const f2 = (v) => v.toFixed(2), f4 = (v) => v.toFixed(4);
    const now = `area A = ${fmt(ant.area)}, edge P = ${fmt(ant.perim)}, P/A = ${f4(ant.perim / ant.area)}`;
    if (!L) { el.textContent = `${now}. Collecting the first window (1,024 steps)…`; return; }
    el.innerHTML = `${now}.<br>Last doubling of time (${fmt(L.end / 2)} to ${fmt(L.end)} steps): ` +
      `<b>c</b> = ${f2(L.c)} (new cells per step that starts on an edge cell), ` +
      `<b>κ</b> = ${f2(L.kappa)} (time on the edge, relative to its share of the cells), ` +
      `so c·κ·P/A = ${f4(L.c * L.kappa * L.perim / L.area)} against ${f4(L.rate)} new cells per step actually seen.` +
      (e == null ? '' : `<br>Area exponent over the last two doublings: <b>${f2(e)}</b> (two thirds for the cup picture).`);
  }

  function frame() {
    if (ant && !paused && !ant.escaped) {
      if (ffTarget > ant.steps) {
        const t0 = performance.now();
        while (ffTarget > ant.steps && !ant.escaped && performance.now() - t0 < 30) ant.run(Math.min(1e6, ffTarget - ant.steps));
        note = ffTarget > ant.steps ? `fast-forwarding to ${fmt(ffTarget)}…` : '';
      } else {
        ant.run(Math.round(10 ** speedExp));
      }
      if (ant.escaped && ffTarget > ant.steps && ant.size < MAX_BOARD) {
        // The pattern outgrew the board before the target; start again bigger.
        load(ant.rule, { ...lastOpts, size: ant.size * 2 });
        note = `enlarging the board to ${fmt(ant.size)} cells…`;
      } else if (ant.escaped && ffTarget > ant.steps) {
        note = `outgrew the largest board this page can hold (${fmt(ant.size)} cells) at step ${fmt(ant.steps)}`;
      } else if (ant.escaped) {
        note = `left the ${fmt(ant.size)}-cell board at step ${fmt(ant.steps)}`;
      }
      draw();
    }
    requestAnimationFrame(frame);
  }

  $('#btn-load').addEventListener('click', () => {
    const r = parseRule($('#rule-input').value);
    if (!r) { $('#err').textContent = 'Use 2 to 32 letters, each L or R.'; return; }
    load(r);
  });
  $('#rule-input').addEventListener('keydown', (e) => { if (e.key === 'Enter') $('#btn-load').click(); });
  $('#btn-pause').addEventListener('click', () => {
    paused = !paused;
    $('#btn-pause').textContent = paused ? 'Play' : 'Pause';
  });
  $('#speed').addEventListener('input', (e) => setSpeed(Number(e.target.value)));
  $('#chk-mech').addEventListener('change', (e) => {
    mechOn = e.target.checked;
    $('#mech-readout').hidden = !mechOn;
    load(ant ? ant.rule : 'RLR', { ...lastOpts, size: lastOpts.size });
  });

  const presets = [
    ['RL', 'Langton’s ant', {}],
    ['RLR', 'chaos', { speed: 3 }],
    ['LLRR', 'cardioid', { speed: 3.5 }],
    ['RLLLLLRRL', 'square', { speed: 3.5 }],
    ['RRLRLLRLRRLR', '98.8M-step wait', { ff: 98_820_000, speed: 2.5 }],
  ];
  for (const [r, label, opts] of presets) {
    const b = document.createElement('button');
    b.innerHTML = `<span class="rule">${r}</span> · ${label}`;
    b.addEventListener('click', () => load(r, opts));
    $('#presets').appendChild(b);
  }

  function runFromPage(rule, opts) {
    load(rule, opts);
    $('.hero').scrollIntoView({ behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' });
  }

  // ---------- fates ----------
  const fates = [
    ['RL', 'highway', 'highway', 'Langton’s ant at 16,000 steps: a chaotic blob, then the road. Once started it never stops.', { speed: 1.6 }],
    ['RLR', 'chaotic', 'chaotic', 'Still chaotic after 100 million steps. Like almost every chaotic rule, its width grows as the cube root of time.', { speed: 3 }],
    ['LLRR', 'chaotic', 'symmetric', 'Propp’s cardioid. Chaotic in detail, yet whenever the ant comes home the pattern is mirror-symmetric.', { speed: 3.5 }],
    ['RRRRLL', 'counter', 'log grower', 'Only 40 cells across after 100 million steps. Its width grows by two cells each time the step count doubles, like a counter carrying.', { speed: 5 }],
    ['RRLRRRRRRLRL', 'builder', 'square', 'Walks the rim of a growing square with period 24. Each edge imitates a highway perfectly, then turns.', { speed: 4.5 }],
    ['RRLRLLRRR', 'builder', 'arrowhead', 'Lays down two filled triangles that meet at a point, at about two steps per cell.', { speed: 5 }],
    ['RRRLRLLRRRLR', 'expander', 'triangle', 'Chaotic for about 70 million steps, then sweeps out a filled right triangle, each pass longer than the last. It travels forever but never repeats.', { ff: 69_000_000, speed: 4.5 }],
    ['RRLRLLRLRRLR', 'highway', 'late highway', 'Chaotic for 98,841,126 steps, then a highway with period 936. The blob at the end of the road is everything that came before.', { ff: 98_820_000, speed: 2.5 }],
  ];
  for (const [r, cls, label, text, opts] of fates) {
    const el = document.createElement('div');
    el.className = 'fate';
    el.innerHTML = `
      <button class="pic" aria-label="Run ${r}"><img alt="Cell grid of rule ${r}" src="${IMGS[r]}"></button>
      <div class="tag"><span class="rule">${r}</span><span class="pill ${cls}">${label}</span></div>
      <p>${text}</p>`;
    el.querySelector('.pic').addEventListener('click', () => runFromPage(r, opts));
    $('#fate-grid').appendChild(el);
  }

  // ---------- census ----------
  function census() {
    const c = DATA.census;
    const parts = [
      ['highway', 'highways', 'var(--c-teal)'],
      ['builder', 'builders', 'var(--c-amber)'],
      ['log', 'log growers', 'var(--muted)'],
      ['blob', 'chaotic blobs', 'var(--c-rose)'],
    ];
    const total = DATA.total;
    $('#census-bar').setAttribute('aria-label', parts.map(([k, l]) => `${c[k]} ${l}`).join(', '));
    $('#census-bar').innerHTML = parts
      .map(([k, l, col]) => `<div style="flex: ${c[k]} 1 0; background: ${col}" title="${fmt(c[k])} ${l}">${(100 * c[k] / total).toFixed(0)}%</div>`)
      .join('');
    $('#census-legend').innerHTML = parts.map(([k, l, col]) => `<span><i style="background:${col}"></i>${fmt(c[k])} ${l}</span>`).join('');
    $('#census-intro').innerHTML =
      `Every one of the ${fmt(total)} rules, sorted by fate after 100 million steps. Highways are confirmed escapes. Builders lay down territory at under 20 steps per cell (squares, triangles, arrowheads). For the rest I measured how the pattern's width grows with time, fitting <i>width</i> ∝ <i>t</i><sup>α</sup> over the last six doublings. The exponents fall into two tight groups with almost nothing between.`;

    const w = 760, h = 260, m = { l: 44, r: 16, t: 20, b: 44 };
    const iw = w - m.l - m.r, ih = h - m.t - m.b;
    const ymax = Math.ceil(Math.max(...c.hist) / 250) * 250;
    const x = (a) => m.l + (a / c.bins[c.bins.length - 1]) * iw;
    const y = (v) => m.t + ih - (v / ymax) * ih;
    let s = `<svg viewBox="0 0 ${w} ${h}" role="img" aria-label="Histogram of growth exponents">`;
    for (let v = 0; v <= ymax; v += 250) {
      s += `<line x1="${m.l}" x2="${w - m.r}" y1="${y(v)}" y2="${y(v)}" stroke="var(--line)"/>`;
      s += `<text x="${m.l - 8}" y="${y(v) + 4}" text-anchor="end">${fmt(v)}</text>`;
    }
    c.hist.forEach((v, i) => {
      const a0 = c.bins[i], a1 = c.bins[i + 1];
      const col = a1 <= 0.15 ? 'var(--muted)' : 'var(--c-rose)';
      s += `<rect x="${x(a0) + 1}" y="${y(v)}" width="${x(a1) - x(a0) - 2}" height="${y(0) - y(v)}" fill="${col}" opacity="0.85"/>`;
    });
    for (const [a, lab] of [[0, '0'], [0.1, '0.1'], [0.2, '0.2'], [1 / 3, '⅓'], [0.5, '½'], [0.6, '0.6']]) {
      s += `<line x1="${x(a)}" x2="${x(a)}" y1="${y(0)}" y2="${y(0) + 5}" stroke="var(--muted)"/>`;
      s += `<text x="${x(a)}" y="${y(0) + 18}" text-anchor="middle">${lab}</text>`;
    }
    s += `<text x="${x(0.08)}" y="${y(Math.max(...c.hist.slice(0, 6))) - 8}" text-anchor="middle" style="fill: var(--ink)">log growers</text>`;
    s += `<text x="${x(1 / 3) + 10}" y="${m.t + 10}" text-anchor="start" style="fill: var(--ink)">chaotic blobs</text>`;
    s += `<text x="${m.l + iw / 2}" y="${h - 6}" text-anchor="middle">growth exponent α (width ∝ steps^α)</text></svg>`;
    $('#chart-growth').innerHTML = s;
    $('#cap-growth').textContent =
      `Growth exponents of the ${fmt(c.log + c.blob)} rules that are neither highways nor builders, measured from 2²¹ to 2²⁷ steps. Median for the chaotic group: ${c.blob_median}.`;
  }

  // ---------- decades chart ----------
  function decadeChart() {
    const onsets = DATA.highways.filter((h) => !h.late).map((h) => h.onset);
    // Buckets: (0, 100], then (10^k, 10^(k+1)] for k = 2..7.
    const buckets = [{ lo: 0, hi: 100, label: '≤ 10²' }];
    for (let k = 2; k < 8; k++) buckets.push({ lo: 10 ** k, hi: 10 ** (k + 1), label: `10${sup(k)}–10${sup(k + 1)}` });
    for (const b of buckets) b.count = onsets.filter((o) => o > b.lo && o <= b.hi).length;
    const tail = buckets.slice(1);
    const mean = tail.reduce((s, b) => s + b.count, 0) / tail.length;

    const w = 760, h = 300, m = { l: 44, r: 16, t: 24, b: 40 };
    const iw = w - m.l - m.r, ih = h - m.t - m.b;
    const ymax = Math.ceil(Math.max(...buckets.map((b) => b.count)) / 25) * 25;
    const y = (v) => m.t + ih - (v / ymax) * ih;
    const bw = iw / buckets.length;
    let s = `<svg viewBox="0 0 ${w} ${h}" role="img" aria-label="Highways found per decade of steps">`;
    for (let v = 0; v <= ymax; v += 25) {
      s += `<line x1="${m.l}" x2="${w - m.r}" y1="${y(v)}" y2="${y(v)}" stroke="var(--line)" stroke-width="1"/>`;
      s += `<text x="${m.l - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>`;
    }
    buckets.forEach((b, i) => {
      const x = m.l + i * bw + bw * 0.18, bwid = bw * 0.64;
      const fill = i === 0 ? 'var(--muted)' : 'var(--c-teal)';
      s += `<rect x="${x}" y="${y(b.count)}" width="${bwid}" height="${y(0) - y(b.count)}" fill="${fill}" opacity="${i === 0 ? 0.45 : 0.9}" rx="1.5"/>`;
      s += `<text class="v" x="${x + bwid / 2}" y="${y(b.count) - 6}" text-anchor="middle">${b.count}</text>`;
      s += `<text x="${x + bwid / 2}" y="${h - m.b + 18}" text-anchor="middle">${b.label}</text>`;
    });
    s += `<line x1="${m.l + bw}" x2="${w - m.r}" y1="${y(mean)}" y2="${y(mean)}" stroke="var(--c-rose)" stroke-dasharray="5 4" stroke-width="1.5"/>`;
    s += `<text x="${w - m.r}" y="${y(mean) - 6}" text-anchor="end" style="fill: var(--c-rose)">mean ${mean.toFixed(0)} per decade</text>`;
    s += `<text x="${m.l + iw / 2}" y="${h - 4}" text-anchor="middle">step at which the highway begins</text>`;
    s += `</svg>`;
    $('#chart-decades').innerHTML = s;
    $('#cap-decades').textContent =
      `New highways per decade of onset time, all ${fmt(DATA.total)} rules of length 2 to 12, budget ${fmt(DATA.budget)} steps. ` +
      `${buckets[0].count} rules head off within the first hundred steps (grey); after that the count per decade stays between ${Math.min(...tail.map((b) => b.count))} and ${Math.max(...tail.map((b) => b.count))}.`;
  }
  function sup(k) { return '⁰¹²³⁴⁵⁶⁷⁸⁹'[k]; }

  // ---------- one more decade ----------
  function testSection() {
    const t = DATA.test;
    $('#test-intro').innerHTML =
      `If onsets really are spread evenly in log time, running the unresolved rules ten times longer should find about as many new highways as an average decade did before. I took the ${fmt(t.rules)} rules of length 10 or less with no highway by 10⁸ steps (short enough to finish), noted that the six decades from 10² to 10⁸ had yielded ${t.past.join(', ')} highways, and wrote down <b>${t.lo} to ${t.hi}</b> as the prediction. Then I ran all ${fmt(t.rules)} to one billion steps on a 16,384-cell board.`;
    $('#tally').innerHTML = `
      <div><b>${t.lo}–${t.hi}</b><span>new highways predicted, 10⁸ to 10⁹ steps</span></div>
      <div class="miss"><b>${t.found}</b><span>found</span></div>
      <div><b>${t.builders}</b><span>turned out to be square or triangle builders</span></div>
      <div><b>${fmt(t.still)}</b><span>still unresolved at a billion steps</span></div>`;
    $('#test-verdict').innerHTML = t.verdict;

    const vals = [...t.past, t.found];
    const labels = ['10²–10³', '10³–10⁴', '10⁴–10⁵', '10⁵–10⁶', '10⁶–10⁷', '10⁷–10⁸', '10⁸–10⁹'];
    const w = 760, h = 260, m = { l: 44, r: 16, t: 20, b: 40 };
    const iw = w - m.l - m.r, ih = h - m.t - m.b, ymax = 30;
    const y = (v) => m.t + ih - (v / ymax) * ih;
    const bw = iw / vals.length;
    let s = `<svg viewBox="0 0 ${w} ${h}" role="img" aria-label="Highways per decade with prediction band">`;
    for (let v = 0; v <= ymax; v += 10) {
      s += `<line x1="${m.l}" x2="${w - m.r}" y1="${y(v)}" y2="${y(v)}" stroke="var(--line)"/>`;
      s += `<text x="${m.l - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>`;
    }
    const xl = m.l + (vals.length - 1) * bw;
    s += `<rect x="${xl}" y="${y(t.hi)}" width="${bw}" height="${y(t.lo) - y(t.hi)}" fill="var(--c-teal)" opacity="0.16"/>`;
    s += `<text x="${xl + bw / 2}" y="${y(t.hi) - 6}" text-anchor="middle" style="fill: var(--c-teal)">predicted</text>`;
    vals.forEach((v, i) => {
      const last = i === vals.length - 1;
      const x = m.l + i * bw + bw * 0.2, bwid = bw * 0.6;
      s += `<rect x="${x}" y="${y(v)}" width="${bwid}" height="${y(0) - y(v)}" fill="${last ? 'var(--c-rose)' : 'var(--c-teal)'}" opacity="0.9" rx="1.5"/>`;
      s += `<text class="v" x="${x + bwid / 2}" y="${y(v) - 6}" text-anchor="middle">${v}</text>`;
      s += `<text x="${x + bwid / 2}" y="${h - m.b + 18}" text-anchor="middle">${labels[i]}</text>`;
    });
    s += `<text x="${m.l + iw / 2}" y="${h - 4}" text-anchor="middle">step at which the highway begins</text></svg>`;
    $('#chart-test').innerHTML = s;
  }


  // ---------- why a cube root ----------
  const svgEl = (w, h, label, body) => `<svg viewBox="0 0 ${w} ${h}" role="img" aria-label="${label}">${body}</svg>`;
  const pow2 = (k) => `2${String(k).split('').map((d) => '⁰¹²³⁴⁵⁶⁷⁸⁹'[d]).join('')}`;

  function mechDiagram() {
    // A small blob on a grid: # visited, . blank. Edge and fresh cells derived.
    const rows = [
      '..............',
      '.....###......',
      '...#######....',
      '..#########...',
      '..##########..',
      '.###########..',
      '..##########..',
      '...########...',
      '....#####.....',
      '..............',
    ];
    const H = rows.length, W = rows[0].length, cs = 22, pad = 4;
    const vis = (x, y) => y >= 0 && y < H && x >= 0 && x < W && rows[y][x] === '#';
    const nb = [[0, -1], [1, 0], [0, 1], [-1, 0]];
    let s = '';
    for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
      const X = pad + x * cs, Y = pad + y * cs;
      if (vis(x, y)) {
        const edge = nb.some(([dx, dy]) => !vis(x + dx, y + dy));
        s += `<rect x="${X + 1}" y="${Y + 1}" width="${cs - 2}" height="${cs - 2}" rx="2" fill="var(--c-indigo)" opacity="${edge ? 0.35 : 0.2}"/>`;
        if (edge) s += `<rect x="${X + 2}" y="${Y + 2}" width="${cs - 4}" height="${cs - 4}" rx="2" fill="none" stroke="var(--c-teal)" stroke-width="2"/>`;
      } else if (nb.some(([dx, dy]) => vis(x + dx, y + dy))) {
        s += `<rect x="${X + 3}" y="${Y + 3}" width="${cs - 6}" height="${cs - 6}" rx="2" fill="none" stroke="var(--muted)" stroke-dasharray="2 3" stroke-width="1.2"/>`;
      }
    }
    // The ant on an edge cell, stepping out onto fresh ground.
    const ax = pad + 11 * cs + cs / 2, ay = pad + 5 * cs + cs / 2;
    s += `<line x1="${ax}" y1="${ay}" x2="${ax + cs - 4}" y2="${ay}" stroke="var(--c-rose)" stroke-width="2.5" marker-end="url(#arr)"/>`;
    s += `<circle cx="${ax}" cy="${ay}" r="5.5" fill="var(--c-rose)"/>`;
    const defs = `<defs><marker id="arr" viewBox="0 0 10 10" refX="7" refY="5" markerWidth="5" markerHeight="5" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="var(--c-rose)"/></marker></defs>`;
    $('#mech-diagram').innerHTML = svgEl(W * cs + 2 * pad, H * cs + 2 * pad, 'Diagram of the visited region, its edge, and fresh ground', defs + s);
  }

  function longChart() {
    const L = C.long, rules = Object.keys(L);
    const kmin = Math.min(...rules.map((r) => L[r].k[0])), kmax = Math.max(...rules.map((r) => L[r].k.at(-1)));
    const panels = [['c', 'c: chance an edge step breaks new ground', 0, 0.8], ['kappa', 'κ: time on the edge ÷ fair share', 0, 2]];
    const pw = 360, ph = 220, m = { l: 38, r: 10, t: 26, b: 38 }, gap = 40;
    const iw = pw - m.l - m.r, ih = ph - m.t - m.b;
    const x = (k) => m.l + ((k - kmin) / (kmax - kmin)) * iw;
    const tones = ['var(--c-indigo)', 'var(--c-teal)', 'var(--c-amber)', 'var(--c-rose)', 'var(--muted)', 'var(--ink)'];
    let s = '';
    panels.forEach(([key, title, lo, hi], pi) => {
      const ox = pi * (pw + gap);
      const y = (v) => m.t + ih - ((v - lo) / (hi - lo)) * ih;
      s += `<div class="chart">` + `<svg viewBox="0 0 ${pw} ${ph}" role="img" aria-label="${title}">`;
      s += `<text x="${m.l}" y="14" style="fill: var(--ink)">${title}</text>`;
      const step = (hi - lo) / 4;
      for (let v = lo; v <= hi + 1e-9; v += step) {
        s += `<line x1="${m.l}" x2="${pw - m.r}" y1="${y(v)}" y2="${y(v)}" stroke="var(--line)"/>`;
        s += `<text x="${m.l - 6}" y="${y(v) + 4}" text-anchor="end">${+v.toFixed(2)}</text>`;
      }
      for (let k = kmin; k <= kmax; k += 3) s += `<text x="${x(k)}" y="${ph - m.b + 16}" text-anchor="middle">${pow2(k)}</text>`;
      rules.forEach((r, i) => {
        const d = L[r].k.map((k, j) => `${j ? 'L' : 'M'}${x(k).toFixed(1)},${y(L[r][key][j]).toFixed(1)}`).join('');
        s += `<path d="${d}" fill="none" stroke="${tones[i]}" stroke-width="1.6" opacity="0.9"><title>${r}</title></path>`;
      });
      s += `<text x="${m.l + iw / 2}" y="${ph - 4}" text-anchor="middle">steps</text></svg></div>`;
    });
    $('#chart-long').innerHTML = s;
    $('#cap-long').innerHTML = `Six typical rules (${rules.map((r) => `<span class="rule">${r}</span>`).join(', ')}) run from 65 thousand to 4.3 billion steps. Both quantities stay flat: <i>c</i> between ${C.long_c[0]} and ${C.long_c[1]}, <i>κ</i> between ${C.long_kappa[0]} and ${C.long_kappa[1]}. Over the same span their width exponents fall between ${C.long_alpha[0]} and ${C.long_alpha[1]}.`;
  }

  function occupancy() {
    const O = DATA.occ;
    const items = [
      ['RRRRRLLLLLL', 'tethered', 'tethered', 'Time per cell falls off exponentially from home. The edge is almost never visited, so growth nearly stalls: width about <i>t</i><sup>0.15</sup>.'],
      ['RLR', 'mixed', 'well mixed', 'Time per cell is flat right up to the edge. This is the walker the argument assumed, and width grows as <i>t</i><sup>⅓</sup>.'],
      ['RLLLLLRRLRR', 'ferried', 'ferried', 'A chaotic core inside a square frame. Straight rays, short-lived highways, carry the ant out to the frame, where it adds a layer. Width about <i>t</i><sup>½</sup>.'],
    ];
    $('#occ').innerHTML = items.map(([r, cls, label, text]) => `
      <figure>
        <div class="pic"><img alt="Time spent per cell for rule ${r}" src="${IMGS['occ_' + r]}"></div>
        <figcaption><div class="tag"><span class="rule">${r}</span><span class="pill ${cls}">${label}</span></div>${text}</figcaption>
      </figure>`).join('');
    $('#occ-legend').innerHTML = `<span>time per cell, relative to average:</span><span>1/100</span><span class="ramp" style="background: linear-gradient(90deg, ${O.legend.join(',')})"></span><span>100×</span><span><i class="idle"></i>visited earlier, not since</span>`;

    const w = 760, h = 260, m = { l: 52, r: 16, t: 16, b: 40 }, iw = w - m.l - m.r, ih = h - m.t - m.b;
    const x = (v) => m.l + (v / 1.4) * iw;
    const ly = (v) => m.t + ih - ((Math.log10(v) + 3) / 5) * ih; // 1e-3 .. 1e2
    let s = '';
    for (let e = -3; e <= 2; e++) {
      s += `<line x1="${m.l}" x2="${w - m.r}" y1="${ly(10 ** e)}" y2="${ly(10 ** e)}" stroke="var(--line)"/>`;
      s += `<text x="${m.l - 8}" y="${ly(10 ** e) + 4}" text-anchor="end">${e === 0 ? '1' : e > 0 ? (10 ** e) + '×' : '1/' + 10 ** -e}</text>`;
    }
    for (const v of [0, 0.2, 0.4, 0.6, 0.8, 1, 1.2, 1.4]) s += `<text x="${x(v)}" y="${h - m.b + 18}" text-anchor="middle">${v}</text>`;
    const tones = { RRRRRLLLLLL: 'var(--muted)', RLR: 'var(--c-teal)', RLLLLLRRLRR: 'var(--c-amber)' };
    for (const [r] of items) {
      const pts = O.profiles[r].filter(([px, py]) => py != null && py > 1e-3 && px <= 1.4);
      const d = pts.map(([px, py], i) => `${i ? 'L' : 'M'}${x(px).toFixed(1)},${ly(py).toFixed(1)}`).join('');
      s += `<path d="${d}" fill="none" stroke="${tones[r]}" stroke-width="2.2"/>`;
    }
    // Legend, top right, clear of the lines (all three start high on the left).
    items.forEach(([r, , label], i) => {
      const lx = w - m.r - 250, lyy = m.t + 14 + i * 18;
      s += `<line x1="${lx}" x2="${lx + 22}" y1="${lyy - 4}" y2="${lyy - 4}" stroke="${tones[r]}" stroke-width="2.2"/>`;
      s += `<text x="${lx + 30}" y="${lyy}" style="fill: var(--ink)">${r} · ${label}</text>`;
    });
    s += `<text x="${m.l + iw / 2}" y="${h - 4}" text-anchor="middle">distance from centre ÷ equivalent radius</text>`;
    $('#chart-profile').innerHTML = svgEl(w, h, 'Time per cell against distance from the centre', s);

    $('#modes-text').innerHTML = `Across the ${fmt(C.analysed)} growing rules I could fit, what happens at the edge is similar everywhere. The median <i>c</i> is ${C.c[1]} for chaotic rules and ${C.log.c[1]} for slow ones, and the edge exponent is close to ½ for both (medians ${C.beta[1]} and ${C.log.beta[1]}). What differs is how the ant gets to its edge. For chaotic blobs, the number of edge visits in each doubling of time grows as <i>t</i><sup>${C.edge_slope_blob}</sup>, close to the predicted <i>t</i><sup>2/3</sup>. For the slow growers it grows only as <i>t</i><sup>${C.edge_slope_log}</sup>. They reach their edge a nearly fixed number of times per doubling, which is why their growth looks logarithmic.`;
  }

  function knob() {
    const S = C.scatter;
    $('#knob-intro').innerHTML = `If the mechanism is right, the rules whose exponents differ from ⅓ should differ in where the ant spends its time (κ drifting), not in what happens at the edge (<i>c</i> drifting). Each dot is one rule, plotted by how fast that quantity changes over time.`;
    const pw = 360, ph = 280, m = { l: 52, r: 12, t: 26, b: 42 }, gap = 40;
    const iw = pw - m.l - m.r, ih = ph - m.t - m.b;
    const y = (a) => m.t + ih - ((a - 0) / 0.6) * ih;
    const panels = [[1, 'c over time', -1.1, 0.9], [2, 'κ over time', -1.1, 0.9]];
    let s = '';
    panels.forEach(([col, title, lo, hi], pi) => {
      const ox = pi * (pw + gap);
      const x = (v) => m.l + ((Math.max(lo, Math.min(hi, v)) - lo) / (hi - lo)) * iw;
      s += `<div class="chart"><svg viewBox="0 0 ${pw} ${ph}" role="img" aria-label="Width exponent against drift of ${title}"><text x="${m.l}" y="14" style="fill: var(--ink)">${title}</text>`;
      s += `<text transform="translate(11,${m.t + ih / 2}) rotate(-90)" text-anchor="middle">width exponent α</text>`;
      for (const a of [0, 0.1, 0.2, 1 / 3, 0.5]) {
        s += `<line x1="${m.l}" x2="${pw - m.r}" y1="${y(a)}" y2="${y(a)}" stroke="var(--line)"/>`;
        s += `<text x="${m.l - 6}" y="${y(a) + 4}" text-anchor="end">${a === 1 / 3 ? '⅓' : a === 0.5 ? '½' : a}</text>`;
      }
      for (const v of [-1, -0.5, 0, 0.5]) s += `<text x="${x(v)}" y="${ph - m.b + 16}" text-anchor="middle">${v > 0 ? '+' + v : v}</text>`;
      s += `<line x1="${x(0)}" x2="${x(0)}" y1="${m.t}" y2="${m.t + ih}" stroke="var(--muted)" stroke-dasharray="3 3"/>`;
      for (const [code, tone] of [[0, 'var(--c-rose)'], [1, 'var(--muted)']]) {
        const d = S.filter((p) => p[3] === code).map((p) => `M${x(p[col]).toFixed(1)},${y(p[0]).toFixed(1)}h0`).join('');
        s += `<path d="${d}" stroke="${tone}" stroke-width="3.2" stroke-linecap="round" opacity="0.4"/>`;
      }
      s += `<text x="${m.l + iw / 2}" y="${ph - 6}" text-anchor="middle">exponent of ${pi ? 'κ' : 'c'} against time</text></svg></div>`;
    });
    $('#chart-knob').innerHTML = s;
    $('#cap-knob').innerHTML = `${fmt(S.length)} rules: ${fmt(C.blob_n)} chaotic (rose) and ${fmt(C.log_n)} slow growers (grey). In the <i>c</i> panel, <i>c</i> barely drifts for any of them (middle half of chaotic rules within ${C.g_c[0]} to +${C.g_c[2]}). In the κ panel, κ's drift lines up with the exponent (correlation ${C.corr.g_k}, against ${C.corr.g_c} for <i>c</i>). The slow growers sit at κ ∝ <i>t</i><sup>${C.log.g_k[1]}</sup>. Not shown: ${C.bursty} rules whose growth came in one-off bursts, and ${C.unanalysed} that left the board or stopped adding cells within the fitting window.`;
  }

  function looseEnds() {
    const items = [
      `The formula linking the exponent to <i>c</i>, κ and the edge exponent is close to an accounting identity, so its agreement with the data (median error ${C.identity_err} in the area exponent) is a consistency check, not evidence. The evidence is that each ingredient, measured separately, is constant where the mechanism needs it to be.`,
      `Why <i>c</i> is about 0.44, and why the ant is well mixed, were open here. Both are answered <a href="#answers">below</a>.`,
      `My first guess for the fast tail was a cube-root core plus rare long excursions. It was wrong: for the ${C.fastest_n} fastest ants, the spread of where they spend their time grows as <i>t</i><sup>${C.fastest_core}</sup> (median), far faster than a cube root. The rays and frames came from looking at the pictures.`,
      `The edge exponent sits slightly above ½ (median ${C.beta[1]}). I first read that as a ragged edge nudging the width exponent up. The <a href="#calibration">calibration</a> says otherwise: the rotor walker, whose exponent is exactly ⅓, shows the same 0.52 over the same window, so it is a finite-time effect.`,
      `Prior work, found after writing this section: cube-root growth is already known for a close relative, the Eulerian or rotor walker, whose exit direction depends only on the cell and not on how it arrived. Priezzhev, Dhar, Dhar and Krishnamurthy gave the heuristic in 1996, Kapri and Dhar measured a circular region with radius ∝ <i>t</i><sup>1/3</sup> in 2009 (<a href="https://arxiv.org/abs/0906.5506">arXiv:0906.5506</a>), and Bou-Rabee and Peres proved the 2/3 range exponent in 2026 (<a href="https://arxiv.org/abs/2608.23545">arXiv:2608.23545</a>). Their argument is the same in spirit as the one above. What I haven't found is the same law shown for ants, whose turns are relative to their heading, or the edge, mixing and tethering results below.`,
    ];
    $('#loose-ends').innerHTML = items.map((t) => `<li>${t}</li>`).join('');
  }

  const C = DATA.cube;

  // ---------- second follow-up ----------
  const F = DATA.follow;
  const pct = (v) => `${Math.round(v * 100)}%`;

  function chiral() {
    const ch = F.chiral, L = F.letters;
    $('#chiral-intro').innerHTML = `My guess was that edge cells are young, so the turn the ant makes there is set by the rule's first few letters. That part held up. Across ${fmt(L.reduce((a, r) => a + r[2], 0))} rules, the turn for colour 1 alone explains ${pct(F.r2_letters.r1)} of the variation in <i>c</i>, and the turns for colours 1 to 3 explain ${pct(F.r2_letters.r123)}. The turn for colour 6 explains ${(F.r2_letters.r6 * 100).toFixed(1)}%.`;
    const w = 760, h = 300, m = { l: 64, r: 60, t: 10, b: 36 }, iw = w - m.l - m.r, bh = (h - m.t - m.b) / L.length;
    const x = (v) => m.l + (v / 0.7) * iw;
    let s = '';
    for (const v of [0, 0.2, 0.4, 0.6]) {
      s += `<line x1="${x(v)}" x2="${x(v)}" y1="${m.t}" y2="${h - m.b}" stroke="var(--line)"/>`;
      s += `<text x="${x(v)}" y="${h - m.b + 18}" text-anchor="middle">${v}</text>`;
    }
    L.forEach(([p3, c, n], i) => {
      const y = m.t + i * bh;
      const lcount = [...p3].filter((ch) => ch === 'L').length;
      s += `<text x="${m.l - 10}" y="${y + bh / 2 + 4}" text-anchor="end" class="v">${p3}</text>`;
      s += `<rect x="${x(0)}" y="${y + bh * 0.18}" width="${x(c) - x(0)}" height="${bh * 0.64}" rx="1.5" fill="${p3[0] === 'L' ? 'var(--c-teal)' : 'var(--c-amber)'}" opacity="${0.55 + 0.15 * lcount}"/>`;
      s += `<text x="${x(c) + 6}" y="${y + bh / 2 + 4}">${c.toFixed(2)}</text>`;
    });
    s += `<text x="${m.l + iw / 2}" y="${h - 4}" text-anchor="middle">median chance that an edge step breaks new ground</text>`;
    $('#chart-letters').innerHTML = svgEl(w, h, 'c by the turns for colours 1 to 3', s);
    $('#cap-letters').textContent = 'Rules grouped by their turns for colours 1, 2 and 3 (colour 0 always turns R). Teal: colour 1 turns L. Amber: colour 1 turns R.';

    $('#chiral-mech').innerHTML = `What I had wrong was the mechanism. About ${pct(ch.pooled.entered[0])} of all edge steps start on a cell the ant has just stepped onto for the first time. Those are blank, so the ant always turns right there. Turning right on every fresh cell means the ant traces the edge clockwise, which leaves unexplored ground on its <em>left</em>. On young edge cells (colours 1 to 3), fresh ground is to the left ${pct(ch.left_med)} of the time and to the right only ${pct(ch.right_med)} (medians over ${ch.rules} random rules; left wins in ${ch.left_more} of them). So when the ant comes back to a young edge cell, a left turn steps outward and a right turn steps back in.`;
    const rows = [['a cell just entered (colour 0, turns R)', 'entered'], ['a young cell (colours 1 to 3) that turns L', 'young L'], ['a young cell (colours 1 to 3) that turns R', 'young R'], ['an older cell', 'older']];
    $('#tbl-chiral tbody').innerHTML = rows.map(([label, k]) => `<tr><td>${label}</td><td>${pct(ch.pooled[k][0])}</td><td>${pct(ch.pooled[k][1])}</td></tr>`).join('');
    const cm = F.c_model;
    $('#cmodel-text').innerHTML = `A typical rule's 0.44 is just this blend, weighted by how often each kind of cell comes up. To test that it is more than a story, I fitted 13 universal numbers on ${ch.rules} rules (the share of edge steps on each colour, and how often fresh ground lies left or right of each) and used them to predict <i>c</i> from nothing but the letters of each of the other ${fmt(cm.n)} rules. The prediction explains ${pct(cm.r2)} of the variation (correlation ${cm.corr}, median error ${cm.mae_med}). A random-heading estimate predicts almost the same value for every rule, so it explains essentially none of the spread.`;

    const w2 = 520, h2 = 420, m2 = { l: 52, r: 16, t: 14, b: 44 }, iw2 = w2 - m2.l - m2.r, ih2 = h2 - m2.t - m2.b;
    const sx = (v) => m2.l + ((v - 0.1) / 0.6) * iw2, sy = (v) => m2.t + ih2 - ((v - 0.1) / 0.6) * ih2;
    let t = '';
    for (const v of [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]) {
      t += `<line x1="${m2.l}" x2="${w2 - m2.r}" y1="${sy(v)}" y2="${sy(v)}" stroke="var(--line)"/><text x="${m2.l - 8}" y="${sy(v) + 4}" text-anchor="end">${v.toFixed(1)}</text>`;
      t += `<text x="${sx(v)}" y="${h2 - m2.b + 18}" text-anchor="middle">${v.toFixed(1)}</text>`;
    }
    t += `<line x1="${sx(0.1)}" y1="${sy(0.1)}" x2="${sx(0.7)}" y2="${sy(0.7)}" stroke="var(--muted)" stroke-dasharray="4 4"/>`;
    const clamp = (v) => Math.max(0.1, Math.min(0.7, v));
    t += `<path d="${cm.points.map(([p, y]) => `M${sx(clamp(p)).toFixed(1)},${sy(clamp(y)).toFixed(1)}h0`).join('')}" stroke="var(--c-indigo)" stroke-width="3" stroke-linecap="round" opacity="0.3"/>`;
    t += `<text x="${m2.l + iw2 / 2}" y="${h2 - 6}" text-anchor="middle">predicted from the rule's letters</text>`;
    t += `<text transform="translate(12,${m2.t + ih2 / 2}) rotate(-90)" text-anchor="middle">measured c</text>`;
    $('#chart-cmodel').innerHTML = svgEl(w2, h2, 'Predicted against measured c for held-out rules', t);
    $('#cap-cmodel').textContent = `${fmt(cm.n)} rules the model never saw. The dashed line is perfect agreement.`;
  }

  function mixing() {
    const M = F.mixing;
    $('#mix-intro').innerHTML = `My hypothesis was that the interior acts as a random environment. Each interior cell has been visited many times, so its colour is effectively random, and the ant turns right or left by coin toss with a fixed bias. That makes it a persistent random walk, whose spread per step can be worked out exactly. It depends only on <i>q</i>, the share of R letters in the rule.`;
    const pts = M.points, w = 520, h = 400, m = { l: 52, r: 16, t: 14, b: 44 }, iw = w - m.l - m.r, ih = h - m.t - m.b;
    const sx = (v) => m.l + (v / 1.8) * iw, sy = (v) => m.t + ih - (v / 1.8) * ih;
    let s = '';
    for (const v of [0, 0.5, 1, 1.5]) {
      s += `<line x1="${m.l}" x2="${w - m.r}" y1="${sy(v)}" y2="${sy(v)}" stroke="var(--line)"/><text x="${m.l - 8}" y="${sy(v) + 4}" text-anchor="end">${v}</text>`;
      s += `<text x="${sx(v)}" y="${h - m.b + 18}" text-anchor="middle">${v}</text>`;
    }
    s += `<line x1="${sx(0)}" y1="${sy(0)}" x2="${sx(1.8)}" y2="${sy(1.8)}" stroke="var(--muted)" stroke-dasharray="4 4"/>`;
    for (const [rule, pred, meas, tv] of pts) {
      const uni = tv < 0.05;
      s += `<circle cx="${sx(pred)}" cy="${sy(Math.min(meas, 1.8))}" r="4.5" fill="${uni ? 'var(--c-teal)' : 'var(--c-rose)'}" opacity="0.85"><title>${rule}: predicted ${pred}, measured ${meas}</title></circle>`;
    }
    s += `<text x="${m.l + iw / 2}" y="${h - 6}" text-anchor="middle">predicted spread per step</text>`;
    s += `<text transform="translate(12,${m.t + ih / 2}) rotate(-90)" text-anchor="middle">measured spread per step</text>`;
    $('#chart-mix').innerHTML = svgEl(w, h, 'Predicted against measured diffusion', s);
    const within = (lim) => pts.filter((p) => p[3] < 0.05 && Math.abs(p[2] / p[1] - 1) < lim).length;
    const misses = pts.filter((p) => p[3] < 0.05 && Math.abs(p[2] / p[1] - 1) >= 0.2);
    console.assert(misses.every((p) => p[2] > p[1]), 'a miss below the prediction: update the copy');
    $('#cap-mix').innerHTML = `${M.typ_n} typical cube-root rules, measured over the second half of 16.8 million steps. Teal: interior colours close to uniform (${M.uni_n} rules). Rose: noticeably uneven (${M.non_n}). The dashed line is perfect agreement.`;
    $('#mix-text').innerHTML = `Where the interior colours are close to uniform, the formula works: ${within(0.1)} of ${M.uni_n} rules land within 10% of the prediction and ${within(0.2)} within 20%, with no adjustable constants. Successive turns are uncorrelated (median correlation ${M.uni_corr1[1]}), and the spread grows in proportion to time (exponent ${M.uni_expo[0].toFixed(2)}–${M.uni_expo[2].toFixed(2)}). Crossing the whole blob by diffusion takes only ${(M.mix[0] * 100).toFixed(1)}–${(M.mix[2] * 100).toFixed(1)}% of the elapsed time, so the ant crosses its territory hundreds of times every time the clock doubles. That is why it's well mixed. Two cautions. The share of right turns matches the rule's share of R letters almost exactly (within ${(M.q_dev_max * 100).toFixed(1)} points), but that happens by counting whatever the environment, since every cell cycles through its colours in order, so it isn't evidence. And the rules the formula misses all miss in the same direction. Every one of the ${misses.length} uniform-interior rules that is off by more than 20% spreads faster than predicted, and ${misses.filter((p) => Math.abs(p[0].split('').filter((c) => c === 'R').length / p[0].length - 0.5) >= 0.2).length} of them are strongly lopsided (at least 70% of one letter). The ${M.non_n} rules with uneven interiors spread faster still (median ${M.non_ratio[1].toFixed(2)}× the prediction).`;
  }

  function tether() {
    $('#tether-intro').innerHTML = `The slow growers keep most of their time within a few cells of home, and their occupancy falls off exponentially with distance. If something pulls them inward, their average displacement along the outward direction should be negative. For an unbiased walk it is exactly zero. Measured from local motion over 2 to 8 steps, the pull relative to diffusion predicts the occupancy slope:`;
    $('#tbl-tether tbody').innerHTML = F.tether.map(([r, lo, hi, obs]) => `<tr><td class="rule">${r}</td><td>${lo.toFixed(2)} to ${hi.toFixed(2)} per cell</td><td>${obs.toFixed(2)} per cell</td></tr>`).join('');
    $('#runs-text').innerHTML = `Diffusion is the same at every distance from the core, so this is a genuine bias, not the ant lingering where it moves slowly. Which rules get tethered at all is easy to predict from their letters: long runs of the same turn.`;
    const R = F.runs12, w = 760, h = 240, m = { l: 44, r: 12, t: 22, b: 40 }, iw = w - m.l - m.r, ih = h - m.t - m.b, bw = iw / R.length;
    const y = (v) => m.t + ih - v * ih;
    let s = '';
    for (const v of [0, 0.25, 0.5, 0.75, 1]) s += `<line x1="${m.l}" x2="${w - m.r}" y1="${y(v)}" y2="${y(v)}" stroke="var(--line)"/><text x="${m.l - 8}" y="${y(v) + 4}" text-anchor="end">${pct(v)}</text>`;
    R.forEach(([run, frac, n], i) => {
      const x = m.l + i * bw + bw * 0.2;
      s += `<rect x="${x}" y="${y(frac)}" width="${bw * 0.6}" height="${y(0) - y(frac)}" fill="var(--muted)" opacity="0.8" rx="1.5"/>`;
      s += `<text x="${x + bw * 0.3}" y="${y(frac) - 6}" text-anchor="middle">${n}</text>`;
      s += `<text x="${x + bw * 0.3}" y="${h - m.b + 18}" text-anchor="middle" class="v">${run}</text>`;
    });
    s += `<text x="${m.l + iw / 2}" y="${h - 4}" text-anchor="middle">longest run of identical turns</text>`;
    $('#chart-runs').innerHTML = svgEl(w, h, 'Share tethered by longest run', s);
    const last = R.at(-1);
    $('#transplant-intro').innerHTML = `Is home a place stored in the colour field, or something the ant carries with it? I took the final colour fields of seven tethered ants after 16.8 million steps and dropped a fresh ant into each, at home or some distance away, then ran 67 million steps. "Tethered" means it spent at least a quarter of its time within 10 cells of its own recent average position.`;
    const T = F.transplant, S = F.shuffled;
    const cell = (o) => !o ? '<span class="no">left the board</span>' : o.own >= 0.25 ? `<span class="yes">tethered</span> ${pct(o.own)}, at (${o.mean[0].toFixed(0)}, ${o.mean[1].toFixed(0)})` : `<span class="no">spread out</span> ${pct(o.own)}`;
    $('#tbl-transplant tbody').innerHTML = Object.keys(T).map((r) => {
      const sh = S[r] ? S[r].map((o) => (o && o.own >= 0.25 ? '<span class="yes">✓</span>' : '<span class="no">✗</span>')).join(' ') : '<span class="no">not run</span>';
      return `<tr><td class="rule">${r}</td><td>${cell(T[r]['0,0'])}</td><td>${cell(T[r]['20,0'])}</td><td>${cell(T[r]['0,-15'])}</td><td>${sh}</td></tr>`;
    }).join('');
    const n20 = Object.values(T).filter((x) => x['20,0'] && x['20,0'].own >= 0.25).length;
    const n15 = Object.values(T).filter((x) => x['0,-15'] && x['0,-15'].own >= 0.25).length;
    $('#transplant-text').innerHTML = `Home isn't stored. A dropped ant never drifts back to the original home. Instead it usually builds a new one, typically close to where it lands: ${n20} of 7 did from 20 cells east, ${n15} of 7 from 15 cells north. Once formed, a tether stays put: <span class="rule">RRRLLRLLR</span>'s centre didn't move by a tenth of a cell in 67 million steps. Shuffling the colours (same proportions, random places) splits the rules in two. Of the five I tried, the three that re-tether easily also tether in shuffled fields, within about a million steps, for every shuffle. The other two never do, in any of three shuffles. Freezing the colours so they can't change is no use either: the ant immediately locks into a fixed loop. So a tether is a bound state between the ant and the cells it keeps rewriting near itself. It isn't a trap in the field, and it isn't a property of the colour proportions, which barely change with distance from the core.`;
    $('#still-open').innerHTML = [
      `What, cell by cell, points a tethered ant home? Not the colour proportions. It must be the arrangement, but I haven't found the feature.`,
      `Why some tethered rules can nucleate a home anywhere and others only on blank ground or in particular spots.`,
      `Why strongly lopsided rules, and rules with uneven interiors, spread faster than the random-walk formula says. My guess is that recently rewritten cells help the ant out of tight loops, but I haven't tested it.`,
    ].map((x) => `<li>${x}</li>`).join('');
  }


  // ---------- calibration ----------
  function calibration() {
    const K = DATA.calib, A = K.A, B = K.B, Ct = K.contour, P = K.patched;
    const f3 = (v) => v.toFixed(3), f4 = (v) => v.toFixed(4);
    $('#cal-intro').innerHTML = `The cube-root law is proven for the rotor walker, so it is a fair test of my instruments. If they misread the rotor, they could be misreading the ants. I implemented the rotor exactly as Kapri and Dhar describe it (rotate the cell's arrow a quarter-turn clockwise, then step along it, starting from random arrows), checked it against an independent brute-force version, and ran it through the same code as the ants.`;
    const ctr = Ct.final;
    $('#tbl-kd tbody').innerHTML = [
      ['Mean radius of the contour where average visits = 1, at 10⁶ steps', '145.436 ± 0.003', `${f3(ctr.radius)} ± ${f3(ctr.se)} (${fmt(ctr.seeds)} starts)`],
      ['Growth exponent of distance from the start', '0.33 ± 0.01', `${B.nu_local.slice(-2).map(f3).join(', ')} (local, 10⁶ to 10⁷)`],
      ['Growth exponent of the boundary width', '0.136, drifting with N', `${B.delta_local.map((v) => v.toFixed(3)).join(' → ')} (local, 10⁴ to 10⁷)`],
      ['Sites visited exactly 4 times in the last 4S steps', '"most"', `${(B.exactly4[0] * 100).toFixed(1)}%`],
      ['Spread of the visited count between starts', 'shrinks to zero (Bou-Rabee & Peres)', `${(B.S_cv[0] * 100).toFixed(1)}% → ${(B.S_cv.at(-1) * 100).toFixed(1)}% from 10⁴ to 10⁷ steps`],
    ].map(([q, lit, me]) => `<tr><td>${q}</td><td>${lit}</td><td>${me}</td></tr>`).join('');
    $('#cal-kd-text').innerHTML = Ct.verdict;

    const row = (label, d) => `<tr><td>${label}</td><td>${f3(d.alpha_w.median)} (±${f3(d.alpha_w.sd)})</td><td>${f3(d.alpha_rg.median)} (±${f3(d.alpha_rg.sd)})</td><td>${f3(d.a.median)}</td><td>${f3(d.beta.median)}</td><td>${Math.round(d.fill.median * 100)}%</td></tr>`;
    $('#tbl-pipeline tbody').innerHTML = row(`rotor, ${A.rotor.alpha_w.n} starts (true: ⅓)`, A.rotor) + row(`ants, ${fmt(A.ants_core.alpha_w.n)} rules`, A.ants_core) + row(`random walk, ${A.random_walk.alpha_w.n} runs`, A.random_walk);
    $('#cal-pipe-text').innerHTML = `Exponents are medians over runs or rules, with the spread (standard deviation) in brackets. Over the same window, the fits read the rotor's exactly-⅓ exponent as ${f3(A.rotor.alpha_w.median)} from its width and ${f3(A.rotor.alpha_rg.median)} from its radius of gyration, and its area exponent of exactly ⅔ as ${f3(A.rotor.a.median)}. The area keeps converging slowly even at 10⁷ steps (visited sites ÷ N<sup>2/3</sup> is still rising: ${B.S_over_N23[0].toFixed(2)} → ${B.S_over_N23.at(-1).toFixed(2)}), so every finite-window area exponent overshoots. The ants sit right among the rotor's numbers, which is what you would expect if they share its exponent. The random walk is clearly different: an area exponent near 1 with a log correction, a fractal edge, and only ${Math.round(A.random_walk.fill.median * 100)}% of its bounding box visited. One more lesson: <i>c</i> on its own is not diagnostic, because the random walk also gives ${f3(A.random_walk.c.median)}.`;
    $('#cal-noise-text').innerHTML = `How much of the spread between ant rules is real? I ran ${P.rules} chaotic rules from 12 different random 5×5 starting patches each. A single run's width exponent scatters by ±${f3(P.alpha_w.sd_within)}, and its radius exponent by ±${f3(P.alpha_rg.sd_within)}: ants are noisier per run than the rotor. Rules do differ, detectably but slightly. Genuine rule-to-rule differences account for a spread of ±${f3(P.alpha_w.sd_between_true)} in the width exponent (${Math.round(P.alpha_w.icc * 100)}% of the variance) and ±${f3(P.alpha_rg.sd_between_true)} in the radius exponent (${Math.round(P.alpha_rg.icc * 100)}%). The average over rules is ${f4(P.alpha_w.grand_mean)} and ${f4(P.alpha_rg.grand_mean)}. Within this window, I can't tell whether the small differences are different exponents or rules approaching ⅓ at different speeds.`;
  }

  // ---------- table ----------
  const rows = DATA.highways.map((h) => ({
    ...h,
    speed: h.period / Math.max(Math.abs(h.dx), Math.abs(h.dy)),
    drift: Math.hypot(h.dx, h.dy),
  }));
  let sortKey = 'onset', sortDir = -1;
  const lens = [...new Set(rows.map((r) => r.n))].sort((a, b) => a - b);
  for (const n of lens) $('#f-len').insertAdjacentHTML('beforeend', `<option value="${n}">${n}</option>`);
  const maxLog = Math.log10(Math.max(...rows.map((r) => r.onset)));

  function renderTable() {
    const n = $('#f-len').value, q = $('#f-text').value.trim().toUpperCase();
    const list = rows
      .filter((r) => (!n || r.n == n) && (!q || r.rule.includes(q)))
      .sort((a, b) => (a[sortKey] < b[sortKey] ? -1 : a[sortKey] > b[sortKey] ? 1 : 0) * sortDir);
    $('#f-count').textContent = `${fmt(list.length)} highways`;
    $('#tbody').innerHTML = list
      .map((r) => {
        const bw = (Math.log10(Math.max(r.onset, 1)) / maxLog) * 4.5;
        return `<tr data-rule="${r.rule}" data-onset="${r.onset}" data-period="${r.period}">
          <td class="rule">${r.rule}${r.late ? ' ◆' : ''}</td><td>${r.n}</td>
          <td><span class="bar" style="width:${bw.toFixed(2)}rem"></span>${fmt(r.onset)}</td>
          <td>${fmt(r.period)}</td><td>${r.speed.toFixed(1)}</td><td>(${r.dx}, ${r.dy})</td></tr>`;
      })
      .join('');
    document.querySelectorAll('th').forEach((th) => {
      if (th.dataset.k === sortKey) th.setAttribute('aria-sort', sortDir < 0 ? 'descending' : 'ascending');
      else th.removeAttribute('aria-sort');
    });
  }
  document.querySelectorAll('th').forEach((th) =>
    th.addEventListener('click', () => {
      const k = th.dataset.k;
      if (k === sortKey) sortDir = -sortDir; else { sortKey = k; sortDir = k === 'rule' ? 1 : -1; }
      renderTable();
    })
  );
  $('#f-len').addEventListener('change', renderTable);
  $('#f-text').addEventListener('input', renderTable);
  $('#tbody').addEventListener('click', (e) => {
    const tr = e.target.closest('tr');
    if (!tr) return;
    const onset = Number(tr.dataset.onset), period = Number(tr.dataset.period);
    // Land a little before the highway, then play fast enough to see it go.
    const lead = Math.max(3000, period * 30);
    const ff = Math.max(0, onset - lead);
    const speed = Math.min(4, Math.max(1.3, Math.log10(lead / 240)));
    runFromPage(tr.dataset.rule, { ff, speed });
  });

  // ---------- boot ----------
  readTheme();
  const mq = matchMedia('(prefers-color-scheme: dark)');
  const retheme = () => { readTheme(); if (ant) pal = palette(ant.n); };
  mq.addEventListener?.('change', retheme);
  new MutationObserver(retheme).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
  setSpeed(1.6);
  load('RL');
  census();
  decadeChart();
  mechDiagram();
  longChart();
  occupancy();
  knob();
  looseEnds();
  chiral();
  mixing();
  tether();
  calibration();
  testSection();
  renderTable();
  requestAnimationFrame(frame);
})();
