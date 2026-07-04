# Photonic Computing

A comprehensive textbook on computing with light — from Maxwell's equations and
the quantum structure of the photon to silicon photonics, optical neural
networks, neuromorphic photonics, and quantum photonic processors. It develops
every mathematical tool when the physics calls for it, and explains the *reasons*
behind the field's engineering choices rather than just cataloguing them.

**Status:** In progress — 28 chapters across 10 units. Units I–IV and VI
(chapters 1–10, 15–16) are complete; Units V, VII, VIII, IX, and X are being
written from the [master outline](photonic_computing_book_outline.md) and
currently have partial coverage (some sections are still empty placeholders).
See the Completion column below.

## Where to Start

The manuscript and its full apparatus live in **[book/](book/)**. Begin with
**[book/README.md](book/README.md)**, which covers who the book is for, the
assumed background (calculus, linear algebra, intro physics, basic Python), how
to use it, the per-chapter structure, and the notation conventions.

For the complete structural map, see the
[master outline](photonic_computing_book_outline.md).

## Unit Overview

| Unit | Chapters | Theme | Completion |
|------|----------|-------|------------|
| I | 1–3 | The nature of light: electromagnetism, wave optics, light–matter interaction | Complete |
| II | 4–5 | The laser and photodetectors | Complete |
| III | 6–8 | Guided-wave photonics, silicon photonics, photonic crystals | Complete |
| IV | 9–10 | Information theory and optical communications | Complete |
| V | 11–14 | Classical photonic computing: Fourier optics to diffractive networks | Partial |
| VI | 15–16 | Neuromorphic photonics: spiking networks, optical synapses | Complete |
| VII | 17–22 | Quantum photonics and quantum computing with light | Partial |
| VIII | 23–24 | Fabrication and simulation tools | Partial |
| IX | 25 | Benchmarking and the computing landscape | Partial |
| X | 26–28 | Industry, research groups, and the frontiers | Partial |

## Commands

Run from the repository root.

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py PhotonicComputing --pdf

# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../PhotonicComputing

# Validate before opening a PR
python3 tools/validate.py
```

See [PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
