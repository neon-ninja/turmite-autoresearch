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
  console.log("errors:", errors);
  console.log("fates:", d.querySelectorAll(".fate").length, "rows:", d.querySelectorAll("#tbody tr").length, d.querySelector("#f-count").textContent);
  console.log("hud:", d.querySelector("#hud-steps").textContent);
  console.log("chart:", d.querySelector("#cap-decades").textContent);
  console.log("bars:", [...d.querySelectorAll("#chart-decades text.v")].map(t => t.textContent).join(","));
  // click a table row (a late highway) and let it fast-forward
  d.querySelector("#tbody tr").dispatchEvent(new dom.window.MouseEvent("click", { bubbles: true }));
  setTimeout(() => {
    console.log("after row click:", d.querySelector("#hud-rule").textContent, d.querySelector("#hud-steps").textContent, "|", d.querySelector("#hud-note").textContent);
    process.exit(0);
  }, 15000);
}, 1500);
