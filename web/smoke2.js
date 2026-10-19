const assert = require('node:assert/strict');
const page = require('./smoke_harness')(process.argv[2]);
try {
  const d = page.document;
  page.frame();
  for (const selector of ['#mech-diagram svg', '#chart-long svg', '#chart-profile svg', '#chart-knob svg', '#occ img']) {
    assert.ok(d.querySelector(selector), `${selector} renders`);
  }
  assert.ok(d.querySelectorAll('#loose-ends li').length > 0, 'open questions render');
  assert.ok(d.querySelectorAll('#chart-long path').length > 0, 'growth curves render');
  for (const selector of ['#cap-long', '#modes-text', '#knob-intro', '#cap-knob']) {
    assert.ok(d.querySelector(selector).textContent.trim(), `${selector} has text`);
  }
  const rendered = d.body.cloneNode(true);
  rendered.querySelectorAll('script, style').forEach(node => node.remove());
  assert.doesNotMatch(rendered.textContent, /undefined|NaN|\[object Object\]/, 'rendered text contains no invalid values');
  console.log('PASS: research charts, captions, portraits and rendered values');
} finally {
  page.close();
}
