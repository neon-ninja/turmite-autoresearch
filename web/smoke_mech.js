// Smoke test for the cup overlay: node web/smoke_mech.js web/dist/explorer.html   (needs jsdom on NODE_PATH)
const { JSDOM } = require("jsdom");
const fs = require("fs");
const html = fs.readFileSync(process.argv[2], "utf8");
const errors = [];
const dom = new JSDOM(html, {
  runScripts: "dangerously", pretendToBeVisual: true,
  beforeParse(w) {
    w.HTMLCanvasElement.prototype.getContext = () => ({
      createImageData: (w_, h) => ({ data: new Uint8ClampedArray(w_ * h * 4) }),
      putImageData() {}, beginPath() {}, arc() {}, stroke() {},
    });
    w.matchMedia = () => ({ matches: false, addEventListener() {} });
    w.Element.prototype.scrollIntoView = () => {};
    w.addEventListener("error", (e) => errors.push(e.message));
  },
});
dom.virtualConsole.on("jsdomError", (e) => errors.push(String(e)));
setTimeout(() => {
  const d = dom.window.document;
  d.querySelector("#rule-input").value = "RLR";
  d.querySelector("#btn-load").click();
  const chk = d.querySelector("#chk-mech");
  chk.checked = true;
  chk.dispatchEvent(new dom.window.Event("change", { bubbles: true }));
  d.querySelector("#speed").value = 5;
  d.querySelector("#speed").dispatchEvent(new dom.window.Event("input", { bubbles: true }));
  setTimeout(() => {
    console.log("errors:", errors);
    console.log("readout hidden:", d.querySelector("#mech-readout").hidden);
    console.log("readout:", d.querySelector("#mech-readout").textContent);
    console.log("steps:", d.querySelector("#hud-steps").textContent);
    process.exit(errors.length ? 1 : 0);
  }, 6000);
}, 1500);
