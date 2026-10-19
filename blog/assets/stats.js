/* Saved-data instruments for research posts. No network or simulation required. */
(function () {
  'use strict';
  const mean = xs => xs.reduce((a, b) => a + b, 0) / xs.length;
  function slope(xs, ys) {
    const x = mean(xs), y = mean(ys);
    return xs.reduce((s, v, i) => s + (v - x) * (ys[i] - y), 0) /
      xs.reduce((s, v) => s + (v - x) ** 2, 0);
  }
  function uniform(a, beta) {
    const q = 1 - a * (1 - beta);
    return Math.abs(q) < 1e-10 ? 2 * Math.LN2 : -2 * Math.expm1(-q * Math.LN2) / q;
  }
  function fit(run, lo, hi) {
    const rows = run.windows.filter(r => r[0] >= lo && r[0] <= hi);
    if (rows.length !== hi - lo + 1) return null;
    const log = i => rows.map(r => Math.log(r[i]));
    const a = slope(log(1), log(2)), beta = slope(log(2), log(3));
    const kap = rows.map(r => 2 * r[4] * r[2] / (r[1] * r[3]));
    return { a, beta, ratio: mean(kap) / uniform(a, beta),
      g: slope(rows.map(r => r[0]), kap.map(Math.log)) };
  }
  function summarize(data, lo, hi) {
    if (!Number.isInteger(lo) || !Number.isInteger(hi) || hi <= lo) throw Error('Need at least two windows');
    const levels = [...new Set(data.runs.map(r => r.eps))].sort((a, b) => a - b);
    return levels.map(eps => {
      const runs = data.runs.filter(r => r.eps === eps);
      const fits = runs.map(r => fit(r, lo, hi)).filter(Boolean);
      const ratios = fits.map(f => f.ratio), avg = mean(ratios);
      return { eps, total: runs.length, n: fits.length,
        escaped: runs.filter(r => r.escaped).length,
        ratio: fits.length ? avg : null,
        sd: fits.length > 1 ? Math.sqrt(ratios.reduce((s, v) => s + (v - avg) ** 2, 0) / (fits.length - 1)) : null,
        a: fits.length ? mean(fits.map(f => f.a)) : null,
        stationary: fits.filter(f => Math.abs(f.g) < 0.05).length };
    });
  }
  function mount(root) {
    const data = JSON.parse(root.querySelector('[data-stats-data]').textContent);
    if (!data || data.kind !== 'noisy-rotor') return;
    const controls = Object.fromEntries([...root.querySelectorAll('[data-control]')].map(el => [el.dataset.control, el]));
    const svg = root.querySelector('svg'), table = root.querySelector('tbody');
    const fmt = x => x === null ? 'unavailable' : x.toFixed(3);
    const el = (name, attrs, text) => {
      const node = document.createElementNS('http://www.w3.org/2000/svg', name);
      Object.entries(attrs).forEach(([k, v]) => node.setAttribute(k, v));
      if (text !== undefined) node.textContent = text;
      svg.append(node);
      return node;
    };
    function render(changed) {
      let lo = +controls.lo.value, hi = +controls.hi.value;
      if (lo >= hi) {
        if (changed === 'lo') controls.hi.value = hi = lo + 1;
        else controls.lo.value = lo = hi - 1;
      }
      const rows = summarize(data, lo, hi), index = +controls.eps.value, selected = rows[index];
      for (const [key, value] of Object.entries({ eps: selected.eps, lo, hi })) {
        root.querySelector(`[data-value="${key}"]`).textContent = value;
        controls[key].setAttribute('aria-valuetext', key === 'eps' ? String(value) : `2 to the power ${value} steps`);
      }
      svg.replaceChildren();
      const max = Math.max(2, ...rows.filter(r => r.n).map(r => r.ratio + (r.sd || 0))) * 1.12;
      const x = i => 65 + i * 66, y = v => 265 - v / max * 215;
      // Measured epsilon levels are categorical: deliberately no interpolation.
      for (let tick = 0; tick <= 4; tick++) {
        const value = max * tick / 4;
        el('line', { x1: 60, x2: 610, y1: y(value), y2: y(value), class: 'stats-grid' });
        el('text', { x: 48, y: y(value) + 4, 'text-anchor': 'end' }, value.toFixed(1));
      }
      el('line', { x1: 60, x2: 610, y1: y(1), y2: y(1), class: 'stats-reference' });
      el('text', { x: 65, y: 21 }, 'κ / κ uniform · dashed line = uniform reference (1)');
      rows.forEach((row, i) => {
        el('text', { x: x(i), y: 288, 'text-anchor': 'middle' }, row.eps);
        if (!row.n) {
          el('text', { x: x(i), y: 254, 'text-anchor': 'middle' }, '×');
          return;
        }
        if (row.sd !== null) {
          const top = y(row.ratio + row.sd), bottom = y(row.ratio - row.sd);
          el('path', { d: `M${x(i)},${top}V${bottom}M${x(i)-5},${top}h10M${x(i)-5},${bottom}h10`, class: 'stats-whisker' });
        }
        const point = el('circle', { cx: x(i), cy: y(row.ratio), r: i === index ? 7 : 4,
          class: i === index ? 'stats-selected' : 'stats-point' });
        const title = document.createElementNS(svg.namespaceURI, 'title');
        title.textContent = `ε=${row.eps}: ${fmt(row.ratio)} ± ${fmt(row.sd)}, n=${row.n}`;
        point.append(title);
      });
      el('text', { x: 330, y: 314, 'text-anchor': 'middle' }, 'Noise probability ε · measured levels, equally spaced');
      table.replaceChildren();
      rows.forEach(row => {
        const tr = document.createElement('tr');
        if (row.eps === selected.eps) tr.className = 'stats-active-row';
        [row.eps, `${row.n} / ${row.total}`, row.n ? `${fmt(row.ratio)} ± ${row.sd === null ? 'sd unavailable' : fmt(row.sd)}` : 'No usable runs',
          fmt(row.a), `${row.stationary} / ${row.n}`].forEach(value => {
          const td = document.createElement('td'); td.textContent = value; tr.append(td);
        });
        table.append(tr);
      });
      root.querySelector('.stats-summary').textContent = `ε = ${selected.eps}; window ends 2^${lo}…2^${hi} steps. ` +
        (selected.n ? `Ratio ${fmt(selected.ratio)}; between-seed sd ${fmt(selected.sd)}; ${selected.n} of ${selected.total} seeds usable. Area exponent ${fmt(selected.a)}. ${selected.stationary} of ${selected.n} usable seeds stationary.` : 'No seed has every selected window.');
      root.querySelector('.stats-warning').textContent = `${selected.escaped} of ${selected.total} runs at this noise level eventually left the board. ` +
        (selected.n < selected.total ? 'The available seeds are a selected subset; later windows may favour runs that stayed on the board longer. ' : '') +
        (selected.n && selected.stationary < selected.n ? 'Drifting seeds contribute to this mean; it is not a stationary estimate.' : '');
    }
    Object.entries(controls).forEach(([key, input]) => input.addEventListener('input', () => render(key)));
    root.querySelector('[data-reset]').addEventListener('click', () => {
      controls.eps.value = 1; controls.lo.value = 12; controls.hi.value = 22; render();
    });
    render();
    root.querySelector('.stats-controls').hidden = false;
    root.querySelector('[data-live]').hidden = false;
    root.querySelector('[data-fallback]').hidden = true;
  }
  if (typeof module !== 'undefined') module.exports = { slope, uniform, fit, summarize };
  if (typeof document !== 'undefined') document.querySelectorAll('[data-stats-explorer]').forEach(root => {
    try { mount(root); } catch (error) { console.error('Statistics explorer unavailable:', error); }
  });
})();
