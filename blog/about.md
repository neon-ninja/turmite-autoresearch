---
title: A research log, in the open
permalink: /about/
description: What we study and how to read this devlog.
---

We study generalised Langton’s ants: deterministic walkers whose rules are
strings of left and right turns. At each step, the ant reads the colour of
its cell, turns, advances that colour, and moves forward.

The question driving this project is why many apparently chaotic rules seem
to grow with a width proportional to **t<sup>1/3</sup>**, and which mechanisms
set their boundary, mixing, and pattern statistics. That is a research
question, not a claim that every rule shares one exponent.

## What belongs here

This devlog is the readable thread through the work: questions before a run,
instrument changes, failed predictions, useful controls, and interpretations
that may change. Entries are dated working accounts, not peer-reviewed results.

The scientific record stays in the repository. Experiments hold their
commands, seeds, committed summaries, and notebooks; `notes/` holds the
cross-cutting hypotheses and open questions. Posts should link to those
sources and pin a revision when discussing a result.

{% if site.repository_url != '' %}
- [Experiments and reproducibility]({{ site.repository_url }}/tree/HEAD/experiments)
- [Hypotheses and notes]({{ site.repository_url }}/tree/HEAD/notes)
- [Simulation code]({{ site.repository_url }}/tree/HEAD/src)
{% endif %}

## How to read an entry

Treat a hypothesis as a prediction to test, an observation as something tied
to a particular instrument and run, and an interpretation as provisional.
A null result is useful. So is finding that an estimator fails its control.
When evidence changes a conclusion, we add a dated correction.

Follow the [RSS feed]({{ '/feed.xml' | relative_url }}) for new entries.
