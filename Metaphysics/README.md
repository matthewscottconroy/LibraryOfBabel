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

The tree nests four levels deep — **module → chapter → section → subsection** —
and the support material lives at the level it describes, not all at the top:

```
Metaphysics/
├── INDEX.md                              ← master index for the whole book
├── 01_Ontology/                          ← MODULE
│   ├── 00-domain-survey.md
│   ├── 00_introduction.md
│   ├── ARGUMENT-MAPS.md                  ← one per module
│   ├── 01_Being_and_Existence/           ← CHAPTER
│   │   ├── 00_introduction.md
│   │   ├── review_questions.md
│   │   ├── 01_What_Is_Ontology/          ← SECTION
│   │   │   ├── 00_introduction.md
│   │   │   ├── 01_Scope_and_Method/      ← SUBSECTION (finest content)
│   │   │   │   └── *.md
│   │   │   ├── 02_The_Fundamental_Question/
│   │   │   ├── exercises.md              ← support files live at SECTION level
│   │   │   ├── further_reading.md
│   │   │   ├── important_figures.md
│   │   │   ├── important_terms_and_ideas.md
│   │   │   └── references_and_primary_sources.md
│   │   └── …
│   └── …
├── 02_Universals_and_Particulars/
│   └── …
└── … (13 modules total)
```

The finest-grained subsection files carry the content: the central argument or
puzzle, formalizations in standard notation (modal logic, mereology, Leibniz's
Law, etc.), the main positions and their objections, and pointers to primary
sources. Each **section** collects its own `exercises.md`, `further_reading.md`,
`important_figures.md`, `important_terms_and_ideas.md`, and
`references_and_primary_sources.md`; each **chapter** adds `review_questions.md`;
each **module** carries a domain survey, an introduction, and an
`ARGUMENT-MAPS.md`; and the repository root holds the master `INDEX.md`.

**Scale:** ~2,052 markdown files across 13 modules.

## Commands

Run from the repository root. The builder walks the directory tree in module
order and shifts Markdown heading levels to match directory depth, so modules
become parts and subdirectories become chapters and sections.

```bash
# Build the book (also --html, --markdown, --check); requires pandoc + xelatex
python3 tools/build_book.py Metaphysics --pdf

# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../Metaphysics

# Validate before opening a PR
python3 tools/validate.py
```

Build output lands in `Metaphysics/output/`, which is gitignored. See
[PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
