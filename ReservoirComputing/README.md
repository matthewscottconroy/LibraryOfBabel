# Reservoir Computing

A textbook on reservoir computing — echo state networks, liquid state machines,
next-generation and physical reservoirs — built from first principles in ten
units, from the dynamical-systems prehistory to the research frontier. Paired
with 17 hands-on computational labs. Roughly 446,000 words.

**Status:** Complete — 10 units plus 17 labs.

## Where to Start

The manuscript lives in **[book/](book/)**. Begin with
**[book/README.md](book/README.md)**, which lays out the ten units, the reading
paths for different backgrounds (mathematician, engineer, neuroscientist,
hacker), and the conventions for Math Boxes, worked examples, thought
experiments, and programming projects.

The [master outline](reservoir_computing_book_outline.md) gives the full
structural map.

## The Labs

**[labs/](labs/)** holds 17 progressively harder computational experiments, each
in its own numbered directory — from fading memory and the echo state property
through NARMA-10, Mackey-Glass and Lorenz prediction, FORCE learning,
conceptors, deep ESNs, NVAR, and physical reservoir computing. They are meant to
be worked alongside the corresponding chapters. The labs are Python and rely on
the usual scientific stack (NumPy, SciPy, Matplotlib).

## Unit Overview

| Unit | Title |
|------|-------|
| I | The Prehistory — Dynamical Systems and the Problem of Time |
| II | The Reservoir — Core Theory and Architecture |
| III | Hyperparameters, Initialization, and Reservoir Design |
| IV | Learning in the Reservoir — Beyond Linear Readouts |
| V | Reservoir Computing at Scale and in Depth |
| VI | Next-Generation Reservoir Computing |
| VII | Physical Reservoir Computing |
| VIII | Applications |
| IX | Advanced Theory |
| X | Reservoir Computing at the Frontier |

## Commands

Run from the repository root.

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py ReservoirComputing --pdf

# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../ReservoirComputing

# Validate before opening a PR
python3 tools/validate.py
```

See [PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
