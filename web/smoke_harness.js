// Drive the real page with controlled frames; canvas drawing itself is stubbed.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const { JSDOM, VirtualConsole } = require('jsdom');

module.exports = function openPage(path) {
  const errors = [], frames = [];
  const virtualConsole = new VirtualConsole();
  virtualConsole.on('jsdomError', error => errors.push(String(error)));
  const dom = new JSDOM(fs.readFileSync(path, 'utf8'), {
    runScripts: 'dangerously', pretendToBeVisual: true, virtualConsole,
    beforeParse(w) {
      w.HTMLCanvasElement.prototype.getContext = () => ({
        createImageData: (width, height) => ({ data: new Uint8ClampedArray(width * height * 4) }),
        putImageData() {}, beginPath() {}, arc() {}, stroke() {},
      });
      w.matchMedia = () => ({ matches: false, addEventListener() {} });
      w.Element.prototype.scrollIntoView = () => {};
      w.requestAnimationFrame = callback => frames.push(callback);
      w.addEventListener('error', error => errors.push(error.message));
    },
  });
  return {
    window: dom.window,
    document: dom.window.document,
    frame() {
      assert.ok(frames.length, 'page must schedule an animation frame');
      frames.shift()(0);
      assert.deepEqual(errors, [], 'page must run without JavaScript errors');
    },
    close() {
      dom.window.close();
      assert.deepEqual(errors, [], 'page must run without JavaScript errors');
    },
  };
};
