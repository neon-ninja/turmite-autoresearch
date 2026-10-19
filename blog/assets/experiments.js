const search = document.querySelector('#experiment-search');
const cards = [...document.querySelectorAll('.experiment-card')];
search.addEventListener('input', () => {
  const terms = search.value.toLocaleLowerCase().trim().split(/\s+/).filter(Boolean);
  let visible = 0;
  for (const card of cards) {
    card.hidden = !terms.every(term => card.textContent.toLocaleLowerCase().includes(term));
    if (!card.hidden) visible++;
  }
  document.querySelector('#experiment-count').textContent = `${visible} of ${cards.length} experiments`;
});
