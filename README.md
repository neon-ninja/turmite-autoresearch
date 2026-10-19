# Turmite

An empirical study of generalised Langton's ants. Rules are strings over
`{L, R}`: the current cell's colour selects a turn, the colour advances modulo
the rule length, and the ant steps forward. Classic Langton's ant is `RL`.

We are investigating why many chaotic rules appear to grow in width roughly
as `t^(1/3)` (area roughly `t^(2/3)`), and how the pattern field, boundary and
mixing statistics produce that behaviour. These are research questions;
measurements, hypotheses and unresolved explanations live together here.

Start with [the open questions](notes/open-questions-hypotheses.md),
[the experiments guide](experiments/README.md), and
[the research blog guide](blog/README.md). The Rust simulation code is in
[`src/`](src/), the browser simulator in [`web/`](web/), and the manuscript and
figure scripts in [`research-outputs/`](research-outputs/).

## Contributing with your agent

Pull requests run the CPU oracle tests, simulator smoke checks, small
reproducibility checks and CUDA compilation. Hardware GPU checks are manual.
See [CI commands and GPU runner setup](.github/CI.md).

You can help by directing your coding or research agent to work on this repo.
Reproductions, careful criticism, negative results, bug fixes and clearer
documentation are all useful contributions. You do not need a new growth
mechanism or a GPU to get started.

1. Fork and clone the repository, then open the clone in your agent's workspace.
2. Have it read [`AGENTS.md`](AGENTS.md) first. That file contains the working
   conventions, validation requirements and reproducibility standards. Its
   Windows paths and hardware details describe the original research machine;
   adapt paths to your checkout and use WSL for toolchains on Windows.
3. Pick one bounded task. Read the relevant notes and the latest entries in the
   linked experiments before treating an older question as still open. Check
   GitHub issues and pull requests for overlapping work; for a substantial
   investigation, open an issue describing the question and proposed test.
4. Give your agent a time and compute budget. Start with a small reproduction
   or control before launching a long sweep.
5. Review its changes and evidence, then submit a focused pull request. Include
   the question or bug, what changed, exact reproduction commands, checks run,
   and limitations or checks skipped. Identify the agent's role and review its
   claims yourself.

Here is a starting prompt to copy and adapt:

```text
Help me contribute to this repository. Read AGENTS.md and follow its
instructions, then read the notes and experiment records relevant to this task:

Task: [one question to test, result to reproduce, bug to fix, or doc to improve]
Budget: [time limit, CPU/GPU availability, and maximum run size]

Check whether the task has already been answered or superseded. Make a small,
reviewable contribution within this scope. For an investigation, register a
timestamped hypothesis and quantitative predictions before measuring, use the
experiment template, and keep the lab notebook as you work. Record exact
commands, source revision, inputs, seeds, parameters and environment. Preserve
existing results and report failed tests and null findings honestly.

Validate changes with the relevant release-mode tests and reference oracle;
calibrate any changed estimator. Keep generated bulk data out of git. Finish
with a diff and a pull-request summary explaining the evidence, uncertainty,
reproduction steps, and anything you could not verify.
```

Good first tasks include reproducing a small committed result, checking a
numerical claim against its producing script, adding an oracle comparison for
a suspected simulation bug, or clarifying an experiment's reproduction steps.
For deeper research, choose an unresolved question and design a test that
distinguishes competing explanations. Record speculation separately from
observations, and preserve predictions when appending corrections.

## Running checks

Use Rust with edition 2024 support and Python 3.12 or newer with `uv` for
analysis. From the checkout in a Linux or WSL shell:

```bash
cargo test --release
uv sync --frozen
```

On the original Windows/WSL setup, set
`UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite` and `UV_LINK_MODE=copy` as described
in [`AGENTS.md`](AGENTS.md). Run Python scripts with `uv run`; use release builds
for simulations. CUDA is optional and has separate setup and oracle checks in
[the experiments guide](experiments/README.md).

For new experiments, copy [`experiments/_template/`](experiments/_template/) to
`experiments/YYYY-MM-DD-slug/`. Include the registered predictions, append-only
lab notebook, shareable notebook, scripts and small reproducible results.
Record seeds and source revisions, quantify uncertainty between seeds, and
keep large raw dumps in the experiment's ignored directories. See
[`AGENTS.md`](AGENTS.md) for the full requirements.

## License

Code is MIT OR Apache-2.0, data (`experiments/*/results/` and Zenodo deposits)
is CC0 1.0, and writing and figures are CC BY 4.0. See
[`LICENSE.md`](LICENSE.md) for exactly what falls under which.
