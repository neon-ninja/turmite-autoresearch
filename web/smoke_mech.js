const assert = require('node:assert/strict');
const page = require('./smoke_harness')(process.argv[2]);
try {
  const { document: d, window: w } = page;
  d.querySelector('#rule-input').value = 'RLR';
  d.querySelector('#btn-load').click();
  const checkbox = d.querySelector('#chk-mech');
  checkbox.checked = true;
  checkbox.dispatchEvent(new w.Event('change', { bubbles: true }));
  d.querySelector('#speed').value = 4;
  d.querySelector('#speed').dispatchEvent(new w.Event('input', { bubbles: true }));
  page.frame();
  assert.equal(d.querySelector('#hud-rule').textContent, 'RLR');
  assert.equal(d.querySelector('#mech-readout').hidden, false, 'overlay readout is visible');
  const readout = d.querySelector('#mech-readout').textContent;
  assert.match(readout, /Last doubling of time/, 'overlay completes an octave window');
  assert.match(readout, /κ/, 'overlay reports kappa');
  assert.doesNotMatch(readout, /undefined|NaN|Infinity/);
  checkbox.checked = false;
  checkbox.dispatchEvent(new w.Event('change', { bubbles: true }));
  page.frame();
  assert.equal(d.querySelector('#mech-readout').hidden, true, 'overlay can be disabled');
  console.log('PASS: mechanism overlay produces finite window statistics and toggles off');
} finally {
  page.close();
}
