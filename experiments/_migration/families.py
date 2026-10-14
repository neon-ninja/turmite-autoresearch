"""One-off migration helper (2026-10-14): assign each legacy file to an experiment family by name."""
import re

OVERRIDE = {  # stem-prefix -> family, checked first (longest prefix wins)
    "1B": "census", "confirmed_1B": "census", "confirmed_v2": "census", "v2_highways": "census",
    "sweep_": "census", "non_highway": "census", "unresolved_le10": "census", "equiv_census": "census",
    "census": "census",
    "run_1e10": "h1", "run_1e11": "h1",
    "armb": "arm", "armc": "arm",
    "calib": "calib",
    "knockout": "knockout", "classko": "knockout",
    "colour": "colourstats", "describe_colours": "colourstats",
    "form_all": "formage", "formage": "formage",
    "mix": "mixing",
    "log": "log",
    "tether": "tether",
    "t1": "t1",
    "hot": "hot",
    "rare_class": "hot",
    "recolour": "recolour",
    "shape": "shape",
    "edge": "edge",
    "probe_": "probe", "analyse_probe": "probe", "growth": "probe", "cube": "probe", "c_model": "probe",
    "c_letters": "probe", "pilot": "probe", "long": "probe", "iso": "probe",
    "transport": "transport",
    "memory": "memory",
    "patched": "patched",
    "patterns": "patterns",
    "layers": "layers",
    "regen_frontier_stats": "census",
    "frontier-2026-10-13": "frontier",
    "verdict": "knockout",
}
LABEL = re.compile(r"^([a-z]+\d+)[a-z]?(?:\d+)?(?=_|\.|$)")


def family(name: str) -> str:
    stem = name
    m = re.match(r"^run_([a-z]+\d+)[a-z]?(?=_|\.|$)", stem)
    if m and not stem.startswith(("run_1e")):
        return m.group(1)
    for k in sorted(OVERRIDE, key=len, reverse=True):
        if stem.startswith(k):
            return OVERRIDE[k]
    m = LABEL.match(stem)
    if m:
        return m.group(1)
    return re.split(r"[_.]", stem)[0]

# families that the generic rules above would split or misplace
OVERRIDE.update({
    "h1e": "highways", "run_1e": "highways", "highway": "highways",
    "contour": "calib", "escape": "c2", "followup": "knockout", "seed": "probe", "sq": "probe",
    "tri": "ext", "tm": "ext", "lastvisits": "misc", "leanbench": "misc", "width": "w",
})

# short human topic appended to the folder name (from notes/frontier-status.md and the results ledger)
TOPIC = {
    "a1": "kappa", "a2": "edge-exponent", "a3": "background-cone-control", "a4": "boundary-width",
    "a5": "home-types", "a6": "corruption-mechanism", "a7": "rrllllllr-self-averaging",
    "arm": "kappa-depletion-arms", "b1": "exact-exponent", "b2": "between-rule-variance",
    "b3": "long-rules", "b4": "unfitted-rules", "b5": "late-highways", "c2": "arrow-bias",
    "c3": "edge-colour-shares", "calib": "instrument-calibration", "census": "sweeps",
    "colourstats": "", "d1": "turmite-sqrt-cluster", "d2": "three-dimensions",
    "d3": "noise", "d4": "many-ants", "dose": "corruption-dose", "e1": "one-sided-hot-region",
    "e2": "sensitive-disc", "e3": "route-collinearity", "e4": "patch-vs-swap",
    "e5": "more-knockout-maps", "e6": "hot-window-age", "edge": "statistics",
    "ext": "other-lattices-turmites-noise", "flip": "damage", "formage": "age",
    "g1": "ribbon-breakout", "h1": "lacunarity-decomposition", "h3": "scaling-collapse",
    "highways": "unresolved-rules-1e10-1e11", "hot": "tiles", "item21": "slow-outliers",
    "item4": "exponent-rotor-calibrated", "j4": "tethered-gyration", "k1": "necklace-icc",
    "knockout": "maps", "layers": "colours", "log": "growers",
    "memory": "walkers", "mixing": "", "patched": "starts",
    "patterns": "fields", "probe": "growth-census", "r1": "width-vs-radius",
    "recolour": "recolour", "shape": "limit", "spread": "home", "t1": "excursion-tomography",
    "tether": "rules", "transport": "forecast", "w": "width-radius-refit",
    "w1": "w1", "w2": "w2", "w3": "w3", "w4": "edge-exponent-2e36",
    "occupancy": "pages", "snapshots": "", "misc": "instruments",
    "item": "", "frontier": "code-patch",
}
