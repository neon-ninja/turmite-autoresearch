/* Illustrations evaluate stated assumptions or display saved measurements.
   No simulation, fitted estimator, animation or network request runs here. */
(function () {
  'use strict';
  function init(document) {
    const root = document.querySelector('.preliminary-results');
    if (!root) return;
    const data = JSON.parse(document.getElementById('preliminary-data').textContent);
    const get = id => document.getElementById(id);
    const width = get('width-factor');
    width.addEventListener('input', () => {
      get('width-label').textContent = `${width.value}×`;
      get('width-time').textContent = String(Number(width.value) ** 3);
    });
    const lead = get('lead-length');
    function showLead() {
      const m = Number(lead.value), cost = data.law.a + data.law.b * m;
      get('lead-label').textContent = `${m} turn${m === 1 ? '' : 's'}`;
      const rule = get('lead-rule');
      rule.replaceChildren(document.createTextNode('R'.repeat(m)));
      const suffix = document.createElement('span'); suffix.textContent = 'L…'; rule.append(suffix);
      rule.setAttribute('aria-label', `Example rule with an opening run of ${m} right turn${m === 1 ? '' : 's'}`);
      get('lead-prediction').textContent = `Model: about ${cost.toFixed(2)} boundary steps per new cell (c ≈ ${(1 / cost).toFixed(3)}).`;
    }
    lead.addEventListener('input', showLead); showLead();
    const toggle = get('occupation-toggle');
    let adjusted = true;
    function showOccupation() {
      toggle.setAttribute('aria-pressed', String(adjusted));
      toggle.textContent = `Adjust for growth: ${adjusted ? 'on' : 'off'}`;
      get('occupation-mode').textContent = adjusted ? 'Occupation / each walker’s uniform reference' : 'Original probe κ · endpoint normalization';
      const rows = [{ key: 'ant', value: adjusted ? data.ant_ratio : data.ant_raw },
        ...data.walkers.map((w, i) => ({ key: String(i), value: adjusted ? w.ratio : w.raw }))];
      for (const row of rows) {
        get(`occupation-${row.key}`).textContent = `${row.value.toFixed(2)}${adjusted ? '×' : ''}`;
        root.querySelector(`[data-occupation="${row.key}"]`).style.width = `${row.value * 50}%`;
      }
    }
    toggle.addEventListener('click', () => { adjusted = !adjusted; showOccupation(); }); showOccupation();
    root.querySelectorAll('.result-controls, .result-toggle').forEach(el => { el.hidden = false; });
  }
  if (typeof module !== 'undefined') module.exports = { init };
  if (typeof document !== 'undefined') init(document);
})();
