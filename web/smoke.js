const assert = require('node:assert/strict');
const page = require('./smoke_harness')(process.argv[2]);
try {
  const { document: d, window: w } = page;
  page.frame();
  assert.ok(d.querySelectorAll('.fate').length > 0, 'fate portraits render');
  assert.ok(d.querySelectorAll('#tbody tr').length > 0, 'highway table renders');
  assert.ok(d.querySelector('#cap-decades').textContent.trim(), 'decade caption renders');
  assert.ok(d.querySelectorAll('#chart-decades text.v').length > 0, 'decade bars render');
  const steps = () => Number(d.querySelector('#hud-steps').textContent.replace(/[^0-9]/g, ''));
  assert.ok(steps() > 0, 'initial simulation advances');
  d.querySelector('#btn-pause').click();
  const paused = steps();
  page.frame();
  assert.equal(steps(), paused, 'pause stops the simulation');
  d.querySelector('#btn-pause').click();
  page.frame();
  assert.ok(steps() > paused, 'play resumes the simulation');
  // Select the earliest onset so this check stays inexpensive as the census grows.
  const row = [...d.querySelectorAll('#tbody tr')].sort((a, b) => Number(a.dataset.onset) - Number(b.dataset.onset))[0];
  row.dispatchEvent(new w.MouseEvent('click', { bubbles: true }));
  page.frame();
  assert.equal(d.querySelector('#hud-rule').textContent, row.dataset.rule, 'row loads its rule');
  assert.ok(steps() > 0, 'selected rule advances');
  console.log('PASS: page initialization, simulation, pause/resume and table selection');
} finally {
  page.close();
}
