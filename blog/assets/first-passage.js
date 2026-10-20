/* A conditional scaling illustration; no measurements or fitted parameters. */
(() => {
  "use strict";
  const root = document.querySelector(".fp-explorer");
  if (!root) return;
  const names = ["theta", "z", "delta"];
  const inputs = names.map(name => root.querySelector(`#fp-${name}`));
  function update() {
    const [theta, z, delta] = inputs.map(input => Number(input.value));
    names.forEach((name, i) => {
      root.querySelector(`#fp-${name}-value`).textContent = Number(inputs[i].value).toFixed(2);
    });
    const drift = 1 + delta - z * (1 - theta);
    // y is log16(kappa/kappa0); clipping is disclosed in the live caption.
    const points = Array.from({length: 101}, (_, i) => {
      const fraction = i / 100;
      const x = 80 + 570 * fraction;
      const y = 145 - 90 * Math.max(-1, Math.min(1, drift * fraction));
      return `${i ? "L" : "M"}${x.toFixed(2)} ${y.toFixed(2)}`;
    }).join("");
    root.querySelector("#fp-curve").setAttribute("d", points);
    const ratio = Math.pow(16, drift);
    const trend = Math.abs(drift) < 1e-8 ? "stays constant" : drift > 0 ? "rises" : "falls";
    const clipping = Math.abs(drift) > 1 ? " Curve clipped at the plotted range; this is not a plateau." : "";
    const message = `Predicted drift: ${drift.toFixed(2)}. κ ${trend}; at L/L₀ = 16, the relative value is ${ratio.toFixed(3)}.${clipping}`;
    root.querySelector("#fp-verdict").textContent = message;
    root.querySelector("#fp-chart-desc").textContent = `${message} Both axes use logarithmic spacing. Illustrative model, not ant data.`;
  }
  inputs.forEach(input => input.addEventListener("input", update));
  root.querySelector("#fp-reset").addEventListener("click", () => {
    [0.5, 2, 0].forEach((value, i) => { inputs[i].value = value; });
    update();
  });
  root.querySelector(".fp-controls").hidden = false;
  update();
})();
