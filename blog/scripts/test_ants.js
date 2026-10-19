// Run from the repository root: node blog/scripts/test_ants.js
// Uses the simulator's pinned jsdom dependency: npm ci --prefix web.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { JSDOM } = require('../../web/node_modules/jsdom');
const { Ant } = require('../../web/sim');
const { Trail } = require('../assets/ants');
const source = fs.readFileSync(path.join(__dirname, '../assets/ants.js'), 'utf8');

// Compare every step to an independent sparse oracle, including cells that cycle to zero.
for (const rule of ['RL', 'RLRLLL', 'LLRR']) {
  const trail = new Trail(rule), cells = new Map();
  let x = 0, y = 0, h = 0;
  for (let step = 1; step <= 10000; step++) {
    const key = `${x},${y}`, colour = cells.get(key) || 0;
    h = (h + (rule[colour] === 'R' ? 1 : -1) + 4) % 4;
    cells.set(key, (colour + 1) % rule.length);
    if (h === 0) y--; else if (h === 1) x++; else if (h === 2) y++; else x--;
    trail.run(1);
    assert.deepEqual([trail.x, trail.y, trail.heading, trail.steps], [x, y, h, step]);
    assert.equal(trail.cells.get(key).colour, cells.get(key));
  }
  // Also compare the full displayed history to the existing web simulator.
  const ant = new Ant(rule, 1024);
  ant.run(10000);
  assert.deepEqual([ant.x - 512, ant.y - 512, ant.h], [x, y, h]);
  for (const cell of trail.cells.values()) {
    assert.equal(ant.cells[(cell.y + 512) * 1024 + cell.x + 512], cell.colour);
  }
}

function checkControls(reduced) {
  const html = fs.readFileSync(path.join(__dirname, '../index.html'), 'utf8');
  const dom = new JSDOM(html, { runScripts: 'outside-only', pretendToBeVisual: true });
  const w = dom.window, d = w.document;
  const frames = new Map();
  let nextId = 0, now = 0, observer;
  w.matchMedia = query => ({ matches: query.includes('reduced-motion') && reduced, addEventListener() {} });
  w.HTMLCanvasElement.prototype.getContext = () => ({ fillRect() {}, beginPath() {}, arc() {}, fill() {}, stroke() {} });
  w.requestAnimationFrame = cb => { frames.set(++nextId, cb); return nextId; };
  w.cancelAnimationFrame = id => frames.delete(id);
  w.IntersectionObserver = class { constructor(cb) { observer = cb; } observe() {} };
  const advance = () => {
    const callbacks = [...frames.values()];
    frames.clear(); now += 100;
    callbacks.forEach(cb => cb(now));
  };
  const steps = () => Number(d.querySelector('.ant-steps').textContent.replace(/[^0-9]/g, ''));
  w.eval(source);
  assert.equal(d.querySelector('.ant-playground').hidden, false);
  assert.equal(steps(), 4000);
  observer([{ isIntersecting: true }]);
  const pause = d.querySelector('#ants-pause');
  if (reduced) {
    assert.equal(frames.size, 0, 'reduced motion starts with a still preview');
    pause.click();
  }
  advance(); advance();
  assert.ok(steps() > 4000);
  pause.click();
  const paused = steps();
  advance();
  assert.equal(steps(), paused);
  d.querySelector('#ants-reset').click();
  assert.equal(steps(), 0);
  assert.equal(frames.size, 0, 'reset preserves pause');
  pause.click(); advance(); advance();
  assert.ok(steps() > 0);
  observer([{ isIntersecting: false }]);
  assert.equal(frames.size, 0, 'offscreen ants stop scheduling frames');
  observer([{ isIntersecting: true }]);
  for (let i = 0; i < 340; i++) advance();
  assert.equal(steps(), 100000);
  assert.equal(frames.size, 0, 'finished runs stop scheduling frames');
  pause.click(); advance(); advance();
  assert.ok(steps() > 0 && steps() < 100000, 'play restarts a completed run');
  dom.window.close();
}
checkControls(false);
checkControls(true);
console.log('PASS: sparse oracle, web simulator agreement, pause/play/reset, reduced motion, offscreen suspension and run limit');
