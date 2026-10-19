// Uses the existing pinned web/ jsdom dependency.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const { JSDOM } = require('../../web/node_modules/jsdom');
const source = fs.readFileSync(require.resolve('../assets/experiment.js'), 'utf8');
const dom = new JSDOM('<details id="notebook"><summary>Notebook</summary><details id="code"><summary>Code</summary><h2 id="saved-heading">Saved heading</h2></details></details>', {
  url: 'https://example.test/experiments/example/#saved-heading', runScripts: 'outside-only',
});
let scrolled = 0;
dom.window.HTMLElement.prototype.scrollIntoView = () => { scrolled++; };
dom.window.eval(source);
assert.equal(dom.window.document.querySelector('#notebook').open, true);
assert.equal(dom.window.document.querySelector('#code').open, true);
assert.equal(scrolled, 1);
dom.window.document.querySelector('#notebook').open = false;
dom.window.location.hash = '#notebook';
dom.window.dispatchEvent(new dom.window.Event('hashchange'));
assert.equal(dom.window.document.querySelector('#notebook').open, true);
dom.window.location.hash = '#%ZZ';
assert.doesNotThrow(() => dom.window.dispatchEvent(new dom.window.Event('hashchange')));
dom.window.location.hash = '#missing';
assert.doesNotThrow(() => dom.window.dispatchEvent(new dom.window.Event('hashchange')));
dom.window.close();
console.log('Experiment deep links reveal nested records and handle missing or malformed anchors.');
