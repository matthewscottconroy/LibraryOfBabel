# LibraryOfBabel

A repository of long-form textbooks, each paired with an adaptive quiz and a
repeatable pipeline for generating accurate, pedagogically sound learning
material on demand.

The project has two goals. The first is the content itself: fifteen
book-length treatments of hard subjects, from abstract algebra to the
philosophy of time, written to take a motivated reader from first principles to
the research frontier. The second is the *process* — a cultivated workflow for
producing text, exercises, labs, quizzes, and study guides that is consistent
enough to apply to any new subject. See **[PROCESS.md](PROCESS.md)** for how a
book gets made and **[CONTRIBUTING.md](CONTRIBUTING.md)** for how to help. To
generate a brand-new book on demand, use the reusable
**[SEED_NEW_BOOK.md](SEED_NEW_BOOK.md)** prompt — it interviews you for the
subject and, always, the author's speaking voice, then produces a book in this
same format.

## The Books

| Book | Focus | Approx. words | Status |
|------|-------|---------------|--------|
| [Abstract-Algebra](Abstract-Algebra/) | Logic through the Langlands program; 54-chapter book + 54 Rust demos | ~852K | Complete |
| [BasalCognition](BasalCognition/) | Intelligence before the brain — cognition in non-neural life; 13 units, 41 chapters | ~470K | Complete |
| [Computational-Systems-Synthetic-Biology](Computational-Systems-Synthetic-Biology/) | A knowledge ladder from math bedrock to research-grade synthetic biology, with simulations | ~843K | Complete |
| [Differential-Equations](Differential-Equations/) | Applied analysis (ODEs, PDEs, Fourier, complex) plus an adaptive problem tutor | ~488K | Complete |
| [Dynamical-Systems](Dynamical-Systems/) | Dynamical systems and information theory; 43 chapters | ~297K | Complete |
| [Epistemology](Epistemology/) | Graduate analytic epistemology; 10 topics with three reading paths | ~423K | Complete |
| [General-Relativity](General-Relativity/) | First principles to gravitational physics; 16 units | ~389K | Complete |
| [Homotopy-Type-Theory](Homotopy-Type-Theory/) | Foundations through research formalization; 27 chapters + Rust demos | ~956K | Complete |
| [Metaphysics](Metaphysics/) | Systematic analytic metaphysics; 13 modules, 2,000+ files | ~1.23M | Complete |
| [PhilosophyOfMind](PhilosophyOfMind/) | Ancient soul theories to machine consciousness; 20 topics | ~577K | Complete |
| [PhilosophyOfTime](PhilosophyOfTime/) | The metaphysics and physics of time; 12 units, 39 chapters | ~505K | Complete |
| [PhotonicComputing](PhotonicComputing/) | Computing with light, from Maxwell to quantum photonics; 28 chapters, 10 units | — | Complete |
| [ReservoirComputing](ReservoirComputing/) | Reservoir computing from first principles; 10 units + 17 labs | ~446K | Complete |
| [Telecommunications](Telecommunications/) | Networking in dependency order — signals to Internet; 14 units, 72 chapters, labs + tools + project | ~860K | Complete |
| [Theorem-Proving](Theorem-Proving/) | Interactive theorem proving; 22 chapters, proofs in Lean/Coq/Python | — | Complete |

A dash in the word column means the book is complete but not yet word-counted.

## Quickstart

All commands run from the repository root. Building requires Python 3 (plus
`pandoc` and `xelatex` for PDF output); the quiz requires a Rust toolchain.

```bash
# Build a book to PDF (also --html, --markdown, --check)
python3 tools/build_book.py Epistemology --pdf
# Output lands in Epistemology/output/ (gitignored)

# Take the adaptive quiz for a book
cd quiz && cargo run -p quiz-cli -- --subject ../Epistemology

# Validate the whole repository (run this before every PR)
python3 tools/validate.py
```

Generating new quiz questions calls the Claude API and needs
`ANTHROPIC_API_KEY` in your environment. See [PROCESS.md](PROCESS.md#question-bank-generation)
for details.

## Repository Layout

```
LibraryOfBabel/
├── README.md              # This file
├── PROCESS.md             # How a book gets made (the generation pipeline)
├── SEED_NEW_BOOK.md       # Reusable prompt to generate a new book on demand
├── LICENSE                # CC BY-NC-SA 4.0 (free, non-commercial, share-alike)
├── CONTRIBUTING.md        # How to contribute
├── tools/                 # Shared tooling
│   ├── build_book.py      # Book builder (PDF/HTML/Markdown/check) driven by book.toml
│   └── validate.py        # Repository validator (also run in CI)
├── quiz/                  # One Rust workspace for all books
│   ├── quiz-core/         # Shared engine: config, question bank, generator
│   ├── quiz-cli/          # Terminal quiz runner
│   ├── quiz-tui/          # Full-screen terminal UI
│   └── quiz-web/          # Browser front end
└── <BookDir>/             # One directory per book (see the table above)
    ├── book.toml          # Build configuration
    ├── subject.toml       # Quiz configuration (chapters, phases, prompt, model)
    ├── questions/         # Question bank: chNN.../NNN.json
    └── …                  # Book content (units / chapters / sections)
```

Every book plugs into the same tooling: `book.toml` tells `build_book.py` how to
assemble the PDF, and `subject.toml` tells the quiz engine which chapters to
draw questions from. New books follow the conventions in
[PROCESS.md](PROCESS.md#directory-conventions-for-new-books).

## License

All content and software in this repository is licensed under
[Creative Commons Attribution-NonCommercial-ShareAlike 4.0](LICENSE)
(CC BY-NC-SA 4.0). In short: the books are **free to read, share, and adapt**,
**no one may profit from them or from works derived from them**, and any
adaptation you distribute must stay **equally free** under the same license,
with credit to this project. See [LICENSE](LICENSE) for the full terms.
