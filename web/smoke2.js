const { JSDOM } = require("jsdom"); const fs = require("fs");
const errors = [];
const dom = new JSDOM(fs.readFileSync(process.argv[2], "utf8"), { runScripts: "dangerously", pretendToBeVisual: true,
  beforeParse(w) {
    w.HTMLCanvasElement.prototype.getContext = () => ({ createImageData: (a, b) => ({ data: new Uint8ClampedArray(a * b * 4) }), putImageData() {}, beginPath() {}, arc() {}, stroke() {} });
    w.matchMedia = () => ({ matches: false, addEventListener() {} });
    w.addEventListener("error", (e) => errors.push(e.message));
  } });
dom.virtualConsole.on("jsdomError", (e) => errors.push(String(e)));
setTimeout(() => {
  const d = dom.window.document, txt = (s) => d.querySelector(s)?.textContent.replace(/\s+/g, " ").trim();
  console.log("errors:", errors);
  for (const id of ["#mech-diagram svg", "#chart-long svg", "#chart-profile svg", "#chart-knob svg", "#occ img"]) console.log(id, !!d.querySelector(id));
  console.log("occ imgs:", d.querySelectorAll("#occ img").length, "loose ends:", d.querySelectorAll("#loose-ends li").length, "long paths:", d.querySelectorAll("#chart-long path").length);
  for (const s of ["#cap-long", "#modes-text", "#knob-intro", "#cap-knob", "#loose-ends"]) console.log("\n" + s + ": " + txt(s));
  // look for leftover template junk
  const all = d.body.textContent;
  console.log("\nundefined/NaN in text:", /undefined|NaN|\[object/.test(all));
  process.exit(0);
}, 1500);
