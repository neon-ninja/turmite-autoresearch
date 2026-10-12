const { chromium } = require("playwright-core");
(async () => {
  const b = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium_headless_shell-1194/chrome-linux/headless_shell" });
  const errs = [];
  for (const [name, w, scheme] of [["desk-light", 1200, "light"], ["desk-dark", 1200, "dark"], ["phone-dark", 400, "dark"]]) {
    const p = await b.newPage({ viewport: { width: w, height: 900 }, colorScheme: scheme, deviceScaleFactor: 1 });
    p.on("pageerror", (e) => errs.push(name + ": " + e.message));
    await p.goto("file:///home/claude/out/explorer.html");
    await p.waitForTimeout(1500);
    const over = await p.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
    const el = await p.$("#cuberoot");
    const box = await el.boundingBox();
    console.log(name, "page overflow px:", over, "section height:", Math.round(box.height));
    await el.screenshot({ path: `/home/claude/out/${name}.png` });
    await p.close();
  }
  console.log("errors:", errs);
  await b.close();
})();
