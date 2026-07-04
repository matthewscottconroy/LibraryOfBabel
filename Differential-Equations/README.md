# Differential Equations and Applied Analysis

Two things live here: a book-length treatment of applied analysis — from real
analysis and multivariable calculus through ODEs, Fourier analysis, PDEs, and
complex analysis — and an adaptive command-line tutor that drills the same
material with generated problems and ELO-based progression. Roughly 488,000
words in the book.

**Status:** Complete — 9-part book with worked solutions, plus the tutor.

## The Book

The manuscript is in **[book/](book/)**, organized into nine parts. Each part
directory contains its units and a **`solutions.md`** with worked solutions to
that part's exercises.

| Part | Topic |
|------|-------|
| [00](book/00_Foundations/) | Foundations (real analysis, linear algebra essentials) |
| [01](book/01_Multivariable_Calculus/) | Multivariable Calculus |
| [02](book/02_Vector_Calculus/) | Vector Calculus |
| [03](book/03_Ordinary_Differential_Equations/) | Ordinary Differential Equations |
| [04](book/04_Fourier_Analysis/) | Fourier Analysis |
| [05](book/05_Partial_Differential_Equations/) | Partial Differential Equations |
| [06](book/06_Complex_Analysis/) | Complex Analysis |
| [07](book/07_Dynamical_Systems_and_Chaos/) | Dynamical Systems and Chaos |
| [08](book/08_Advanced_Topics/) | Advanced Topics |

See also [historical-notes.md](book/historical-notes.md).

## The Adaptive Tutor

`tutor.py` (backed by the [`tutor/`](tutor/) package) is a terminal problem
trainer: it generates problems across the book's topics, tracks a per-topic ELO
rating in a local SQLite database, selects harder problems as you improve, and
gates advanced topics behind prerequisites. It accepts symbolic answers
(via SymPy) and multiple-choice.

Full usage — commands, answer syntax, the mastery dashboard, and how the
adaptive algorithm and topic-unlock chain work — is in
**[TUTOR_README.md](TUTOR_README.md)**.

```bash
pip install -r requirements.txt   # sympy, rich
python tutor.py                    # start a session
python tutor.py --stats            # mastery dashboard
```

## Commands

Run from the repository root.

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py Differential-Equations --pdf

# Take the adaptive quiz (distinct from the problem tutor above)
cd quiz && cargo run -p quiz-cli -- --subject ../Differential-Equations

# Validate before opening a PR
python3 tools/validate.py
```

See [PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
