---
title: What a green check can tell us
description: Automating instrument checks exposes the boundary between reproducible code and a credible scientific explanation.
author: Turmite researchers
status: Project note
tags: [reproducibility, instruments]
---

The most useful continuous-integration check for this project is agreement
between the fast simulator and its reference. We have invested in that
comparison, but without an automatic release-test job it still depends on
someone remembering to run it. The new [CI checks](../../.github/CI.md) move
that obligation into the development process.

There is a tempting overstatement here: if every check passes, the research is
reproducible. An oracle comparison can establish that two implementations
agree on the cases exercised. A comparison across thread counts can expose
scheduling-dependent output. Neither establishes that an estimator answers
the physical question we meant to ask. Two implementations can share a
convention error, and a perfectly deterministic fit can carry finite-window
bias.

That boundary shaped the checks. A small seeded batch compares CSV bytes;
an analysis check compares against exact synthetic answers and an independent
calculation. Large sweeps and growth-law fits remain explicit experiments,
with their own controls and provenance. The browser checks inspect displayed
statistics and interactions; they cannot establish the claims those numbers
illustrate.

The GPU boundary is especially concrete. A hosted machine can compile the
Rust integration without ever compiling or executing the runtime CUDA
kernels. Calling that a GPU correctness check would hide the most interesting
failure mode. Hardware oracle tests therefore have a separate, manual job
whose evidence includes the executable hashes and device information.

The next useful extension would be a small frozen calibration case that is
sensitive to a known instrument bias. It would need a tolerance justified by
the calibration record, rather than a golden number copied from today's
output. Until then, a green check says something useful about the machinery;
the scientific interpretation still needs its own argument.
