"""Update the shareable write-up from committed pilot summaries, without execution."""
from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[3]
EXP = Path(__file__).resolve().parents[1]
path = EXP / "notebook.ipynb"
book = json.loads(path.read_text())
cells = [c for c in book["cells"] if c["id"] not in {"pilot-provenance", "pilot-audit", "pilot-interpretation"}]
cells[0]["source"] = [
    "# What does a new cell cost?\n\n",
    "This experiment asks whether a local boundary-event model can predict births per boundary read, including effects of later letters. See [the protocol](README.md) and [the append-only log](NOTEBOOK.md). The ordered-event recorder and stationary solver are calibrated. The excluded pilot failed its common-horizon gate; the main cohort remains unrun.\n\n",
    "The immediate birth is fixed by the current turn and visited-neighbour mask. The unknown is their distribution. Births are not assumed independent regeneration points, and a pooled cross-rule transition matrix must respect target-rule-compatible successor states."
]
for c in cells:
    if c["id"] == "design-audit":
        c["source"] = [line.replace("Scientific results: none; instrument, calibration and pilot pending.", "This cell audits planned assignments only; pilot outcomes are audited below.") for line in c["source"]]
    if c["id"] == "method":
        c["source"] = [
            "## Method and controls\n",
            "The source recorder is compared event by event to a sparse independent path oracle, plus exact totals against the existing aggregate tracer. The solver is calibrated on known two-state reward chains and challenged by hidden history. Rotor and random-walk controls use eight seeds each. The excluded pilot uses four families, three arms and two interior seeds, with horizons 2^22..2^26. Commands, seeds, thresholds and binary hashes are in the README and result provenance.\n\n",
            "No main transition model is fitted. Draft P1–P4 remain untested; G1's solver calibration does not calibrate an ant state/backoff model. The support repeat at k=26 is a registered feasibility diagnostic after the horizon failure, not a rescue of the gate."
        ]
    if c["id"] == "limits":
        c["source"] = [
            "## What remains unestablished\n",
            "There is no validated local renewal model, no derived intercept or slope, and no new mechanism claim about kappa. The pilot gate failed; no families were replaced and no validation outcomes were opened. Unseen-state mass is only a feasibility check, not a measure of predictive accuracy. Coarsening can improve support while destroying relevant information.\n\n",
            "Next decisions must address both sampling failures and target-rule-compatible transitions. A good total birth rate alone cannot show Markov closure; the hidden-history calibration supplies a counterexample."
        ]
def markdown(identity, text):
    return {"cell_type": "markdown", "id": identity, "metadata": {}, "source": text.splitlines(keepends=True)}
def code(identity, text):
    return {"cell_type": "code", "id": identity, "metadata": {}, "execution_count": None, "outputs": [], "source": text.splitlines(keepends=True)}
cells.extend([
    markdown("pilot-provenance", "## Pilot provenance and results\nOnly committed JSON/CSV summaries are read. Every input is checked for content changes and hashed; ignored event dumps are not notebook inputs. Uncertainty below is between seeds, not a fit standard error.\n"),
    code("pilot-audit", '''files = [EXP / 'results' / name for name in ['calibration.json', 'pilot_report.json', 'pilot_diagnostics.json', 'pilot_diagnostics.csv', 'support_report.json']]
for p in files:
    subprocess.run(['git', 'ls-files', '--error-unmatch', str(p.relative_to(ROOT))], cwd=ROOT, check=True, capture_output=True)
    subprocess.run(['git', '-c', 'core.filemode=false', 'diff', '--exit-code', 'HEAD', '--', str(p.relative_to(ROOT))], cwd=ROOT, check=True, capture_output=True)
    print(hashlib.sha256(p.read_bytes()).hexdigest(), p.relative_to(ROOT))
calibration, controls, diagnostic, support = [json.loads(p.read_text()) for p in [files[0], files[1], files[2], files[4]]]
assert calibration['passed']
print('Synthetic reward solver: maximum |c error|', calibration['max_absolute_error'], 'signed bias', calibration['signed_bias'], 'between-seed sd', calibration['between_seed_sd'])
print('Hidden-history control: pooled c', calibration['negative_control']['pooled_c'], 'conditional contrast', calibration['negative_control']['contrast'])
for walker, result in controls['controls'].items():
    assert result['passed'] and result['n'] == 8
    print(walker, 'c mean', result['mean'], 'between-seed sd', result['between_seed_sd'], 'escapes', result['escapes'])
assert controls['thread_check']['identical']
assert controls['horizon']['selected_log2'] is None
print('Original pilot horizon verdict:', controls['horizon']['status'])
print('Usable k=26 runs:', diagnostic['k26_usable_runs'], '/', diagnostic['k26_n'])
for name, result in support['models'].items():
    print(name, 'unseen read mass median', result['median_unseen_mass'], 'maximum', result['max_unseen_mass'], 'runs >5%', result['runs_over_5pct'])
print('These outputs are calibration and feasibility evidence; P1-P4 are untested.')
'''),
    markdown("pilot-interpretation", "## Interpretation and next discrimination\nThe controls validate recording and the narrow solver task. They do not validate a model of ant state frequencies. The original pilot failed to supply a common sampling window; its support diagnostic cannot retroactively make that gate pass. Full eight-turn states may encode family identity and lose transfer support. A smaller state still needs a generative transition rule consistent with the target rule and a test against hidden history.\n")
])
for cell in cells:
    if cell["cell_type"] == "code":
        cell["execution_count"] = None
        cell["outputs"] = []
        cell["metadata"].pop("execution", None)
book["cells"] = cells
path.write_text(json.dumps(book, indent=1, ensure_ascii=False) + "\n")
print(path.relative_to(ROOT))
