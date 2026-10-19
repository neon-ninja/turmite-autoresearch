// Blank-grid ants. Same convention as src/reference.rs and web/sim.js:
// north = 0; read, turn, increment colour, then move. No wrapping at the edge.
(() => {
  'use strict';
  class Trail {
    constructor(rule) {
      this.rule = rule;
      this.cells = new Map();
      this.x = this.y = this.heading = this.steps = 0;
      this.minX = this.maxX = this.minY = this.maxY = 0;
    }
    run(count) {
      for (let i = 0; i < count; i++) {
        const key = `${this.x},${this.y}`;
        const colour = this.cells.get(key)?.colour || 0;
        this.heading = (this.heading + (this.rule[colour] === 'R' ? 1 : 3)) % 4;
        this.cells.set(key, { x: this.x, y: this.y, colour: (colour + 1) % this.rule.length });
        this.x += [0, 1, 0, -1][this.heading];
        this.y += [-1, 0, 1, 0][this.heading];
        this.minX = Math.min(this.minX, this.x);
        this.maxX = Math.max(this.maxX, this.x);
        this.minY = Math.min(this.minY, this.y);
        this.maxY = Math.max(this.maxY, this.y);
        this.steps++;
      }
    }
  }
  if (typeof module !== 'undefined') module.exports = { Trail };
  if (typeof document === 'undefined') return;
  const section = document.querySelector('.ant-playground');
  if (!section) return;
  const portraits = [...section.querySelectorAll('.ant-portrait')].map(figure => ({
    figure, canvas: figure.querySelector('canvas'),
    context: figure.querySelector('canvas').getContext('2d'),
    counter: figure.querySelector('.ant-steps'), trail: new Trail(figure.dataset.rule),
  }));
  if (portraits.some(p => !p.context)) return;
  section.hidden = false;
  const pause = document.querySelector('#ants-pause');
  const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)');
  const dark = matchMedia('(prefers-color-scheme: dark)');
  const limit = 100000;
  let playing = !reducedMotion.matches, visible = false, frame = null, previous = null;

  function draw(p) {
    const { trail: t, canvas, context: c } = p;
    const styles = getComputedStyle(section);
    const palette = [styles.getPropertyValue('--panel').trim(), styles.getPropertyValue('--accent').trim(),
      styles.getPropertyValue('--muted').trim(), dark.matches ? '#b6c895' : '#7c9260',
      dark.matches ? '#d7bc83' : '#b79b59', dark.matches ? '#98bbb8' : '#577f7c'];
    c.fillStyle = palette[0];
    c.fillRect(0, 0, canvas.width, canvas.height);
    // Fit the entire history, with room for the next steps. CSS scales this on mobile.
    const scale = Math.min(8, (canvas.width - 40) / (t.maxX - t.minX + 12),
      (canvas.height - 40) / (t.maxY - t.minY + 12));
    const left = canvas.width / 2 - (t.minX + t.maxX) * scale / 2;
    const top = canvas.height / 2 - (t.minY + t.maxY) * scale / 2;
    for (const cell of t.cells.values()) {
      if (!cell.colour) continue;
      c.fillStyle = palette[cell.colour];
      c.fillRect(left + cell.x * scale, top + cell.y * scale, Math.max(1, scale), Math.max(1, scale));
    }
    const x = left + (t.x + 0.5) * scale, y = top + (t.y + 0.5) * scale;
    c.beginPath();
    c.arc(x, y, Math.max(3.5, scale * 0.65), 0, Math.PI * 2);
    c.fillStyle = styles.getPropertyValue('--ink').trim();
    c.fill();
    c.strokeStyle = palette[0];
    c.lineWidth = 1.5;
    c.stroke();
    p.counter.textContent = `${t.steps.toLocaleString('en-NZ')} steps${t.steps === limit ? ' · finished' : ''}`;
  }
  function updateButton() {
    pause.textContent = playing ? 'Pause' : 'Play';
    pause.setAttribute('aria-label', playing ? 'Pause all three ants' : 'Play all three ants');
  }
  function tick(now) {
    frame = null;
    if (!playing || !visible || document.hidden) return;
    // Capping elapsed time avoids catch-up work after a hidden or busy tab.
    const elapsed = previous === null ? 0 : Math.min(100, now - previous);
    if (previous === null || elapsed >= 50) {
      const count = Math.floor(elapsed * 3);
      for (const p of portraits) {
        p.trail.run(Math.min(count, limit - p.trail.steps));
        draw(p);
      }
      previous = now;
    }
    if (portraits.every(p => p.trail.steps === limit)) {
      playing = false;
      updateButton();
    }
    schedule();
  }
  function schedule() {
    if (frame !== null) cancelAnimationFrame(frame);
    frame = null;
    if (playing && visible && !document.hidden) frame = requestAnimationFrame(tick);
  }
  pause.addEventListener('click', () => {
    if (portraits.every(p => p.trail.steps === limit)) reset();
    playing = !playing;
    previous = null;
    updateButton();
    schedule();
  });
  function reset() {
    for (const p of portraits) { p.trail = new Trail(p.figure.dataset.rule); draw(p); }
    previous = null;
  }
  document.querySelector('#ants-reset').addEventListener('click', reset);
  document.addEventListener('visibilitychange', () => { previous = null; schedule(); });
  reducedMotion.addEventListener('change', () => {
    if (reducedMotion.matches) { playing = false; updateButton(); schedule(); }
  });
  dark.addEventListener('change', () => portraits.forEach(draw));
  // A still preview is also useful when motion is disabled or the section is offscreen.
  for (const p of portraits) { p.trail.run(4000); draw(p); }
  updateButton();
  if (typeof IntersectionObserver !== 'undefined') {
    new IntersectionObserver(entries => {
      visible = entries[0].isIntersecting;
      previous = null;
      schedule();
    }).observe(section);
  } else { visible = true; schedule(); }
})();
