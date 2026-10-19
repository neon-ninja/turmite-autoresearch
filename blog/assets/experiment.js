// Deep links must reveal headings and outputs inside the research record.
function revealExperimentAnchor() {
  let anchor;
  try { anchor = decodeURIComponent(location.hash.slice(1)); } catch { return; }
  const target = document.getElementById(anchor);
  if (!target) return;
  let parent = target;
  while (parent) {
    if (parent.tagName === 'DETAILS') parent.open = true;
    parent = parent.parentElement;
  }
  target.scrollIntoView();
}
window.addEventListener('hashchange', revealExperimentAnchor);
revealExperimentAnchor();
