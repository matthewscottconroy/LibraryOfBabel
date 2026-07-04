# The Road to General Relativity

A complete textbook that builds general relativity from the ground up — starting
with logic, sets, and calculus and arriving, sixteen units later, at Einstein's
field equations, gravitational waves, relativistic cosmology, and the research
frontier. By the time you reach the field equations, you understand every symbol
in them. Roughly 389,000 words.

**Status:** Complete — 16 units spanning 58+ chapters.

## Where to Start

The book and its full pedagogical apparatus live in
**[textbook/](textbook/)**. Begin with **[textbook/README.md](textbook/README.md)**,
which explains how to use the book, the rigor-and-intuition philosophy, the
per-chapter structure (exercises, thought experiments, laboratory projects,
important concepts and researchers, references), and gives the complete
unit-by-unit overview.

For the exhaustive structural map, see the
[master outline](general_relativity_outline.md).

## Unit Overview

| Unit | Title |
|------|-------|
| I | Logic, Proof, and the Language of Mathematics |
| II | Calculus |
| III | Multivariable Mathematics and Linear Algebra |
| IV | Advanced Analysis and Topology |
| V | Classical Mechanics |
| VI | Classical Electromagnetism |
| VII | Special Relativity |
| VIII | Differential Geometry |
| IX | Foundations of General Relativity |
| X | Exact Solutions and Classical Tests |
| XI | Gravitational Waves |
| XII | Relativistic Cosmology |
| XIII | Advanced Formulations of GR |
| XIV | Quantum Effects in Curved Spacetime |
| XV | Singularity Theorems and Global Methods |
| XVI | Frontiers |

## Commands

Run from the repository root.

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py General-Relativity --pdf

# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../General-Relativity

# Validate before opening a PR
python3 tools/validate.py
```

See [PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
