# Metaphysics: A Systematic Introduction

A comprehensive set of lecture notes in analytic metaphysics, organized as a
13-module textbook. Each module covers a major area of the discipline with
full argument formalization, primary source discussion, formal notation, worked
examples, and supporting reference material.

## Modules

| # | Module | Topics |
|---|--------|--------|
| 01 | **Ontology** | Being and existence, substance, properties and relations, abstract objects |
| 02 | **Universals and Particulars** | The problem of universals, trope theory, nominalism, natural kinds |
| 03 | **Identity and Persistence** | Personal identity, material constitution, coincident objects, temporal parts |
| 04 | **Causation and Laws** | Humean and non-Humean causation, laws of nature, dispositions |
| 05 | **Modality** | Possible worlds, necessity and possibility, essence, counterfactuals |
| 06 | **Time and Space** | The nature of time, spacetime, persistence through time, temporal ontology |
| 07 | **Philosophy of Mind** | Mental causation, consciousness, physicalism, intentionality |
| 08 | **Free Will and Agency** | Compatibilism, libertarianism, hard determinism, moral responsibility |
| 09 | **Metametaphysics** | Grounding and fundamentality, ontological commitment, the methodology of metaphysics |
| 10 | **Metaphysics of Science** | Scientific realism, reduction, emergence, the structure of scientific theories |
| 11 | **Mereology** | Parts and wholes, classical mereology, composition, gunk and atomism |
| 12 | **Truth and Reality** | Theories of truth, truthmakers, realism and anti-realism |
| 13 | **Philosophy of Religion** | Arguments for and against theism, divine attributes, religious epistemology |

## Project Structure

```
Metaphysics/
├── 01_Ontology/
│   ├── 01_Being_and_Existence/
│   │   ├── 01_What_Is_Ontology/
│   │   │   ├── 01_*.md          ← topic files
│   │   │   └── …
│   │   └── …
│   ├── exercises.md
│   ├── further_reading.md
│   ├── important_figures.md
│   ├── important_terms_and_ideas.md
│   └── references_and_primary_sources.md
├── 02_Universals_and_Particulars/
│   └── …
└── … (13 modules total)
```

Each numbered file (`01_*.md`, `02_*.md`, …) covers a single topic and
includes: the central argument or puzzle, formalizations in standard notation
(modal logic, mereology, Leibniz's Law, etc.), the main positions and their
objections, and pointers to primary sources. Support files at each level
collect exercises, key figures, terminology, and a bibliography.

**Scale:** 984 content files · 699 support files · 470 directories · ~1,700 markdown files total.

## Building the PDF

A build script assembles the full project into a single PDF textbook.

**Requirements:** `pandoc` (≥ 3.0), `xelatex`, DejaVu Serif font.

```bash
python3 build_pdf.py                        # writes Metaphysics_Textbook.pdf
python3 build_pdf.py ~/Desktop/output.pdf   # custom output path
```

The script walks the directory tree in module order, shifts heading levels to
match the directory depth, and calls pandoc with `--top-level-division=part`
so that modules become LaTeX `\part{}` divisions and subdirectories become
chapters and sections. Build time is roughly 2–5 minutes.
