# Epistemology: A Graduate Curriculum

A comprehensive, graduate-level treatment of analytic epistemology — from the
Meno and the Theaetetus through Gettier, reliabilism, contextualism, formal
Bayesian epistemology, and the social and applied frontier. Roughly 423,000
words across ten topic areas.

**Status:** Complete — 10 topics, each with a unit survey and essay questions.

## Structure

The material is organized into ten numbered topic directories. Each contains a
`00-unit-survey.md` (read this first — it maps the topic's central questions,
positions, and reading recommendations), an `essay-questions.md` set, and
subdirectories developing the material in depth.

| # | Topic |
|---|-------|
| 01 | [Foundations and Historical Survey](01_Foundations_and_Historical_Survey/) |
| 02 | [The Nature of Knowledge](02_The_Nature_of_Knowledge/) |
| 03 | [Sources of Knowledge](03_Sources_of_Knowledge/) |
| 04 | [Justification and Warrant](04_Justification_and_Warrant/) |
| 05 | [Skepticism](05_Skepticism/) |
| 06 | [Truth and Reality](06_Truth_and_Reality/) |
| 07 | [Philosophy of Mind](07_Philosophy_of_Mind/) |
| 08 | [Formal Epistemology](08_Formal_Epistemology/) |
| 09 | [Social and Political Epistemology](09_Social_and_Political_Epistemology/) |
| 10 | [Applied and Contemporary Epistemology](10_Applied_and_Contemporary_Epistemology/) |

## Where to Start

Read topic 01's survey for orientation, then follow one of the three structured
reading paths in **[reading-paths.md](reading-paths.md)**:

- **Analytic Epistemology Core** — the canonical literature, Gettier to
  knowledge-first (for philosophy graduate students; 16–20 weeks).
- **Formal Epistemology** — probability, decision theory, epistemic logic, and
  belief revision (assumes first-order logic and basic probability; 12–16 weeks).
- **Applied Track** — expertise, misinformation, AI, and the ethics of belief
  (accessible without a philosophy background; 10–12 weeks).

## Commands

Run from the repository root.

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py Epistemology --pdf

# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../Epistemology

# Validate before opening a PR
python3 tools/validate.py
```

See [PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
