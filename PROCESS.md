# PROCESS — How a Book Gets Made

This document describes the repeatable pipeline behind every book in this
repository: how a subject goes from an empty directory to a complete textbook
with a working quiz, and the conventions that keep fourteen very different books
buildable by the same tooling.

If you are adding a new book or a new chapter, read this end to end. If you just
want to build or quiz an existing book, see the Quickstart in the root
[README](README.md). To have an AI generate a whole new book on demand, use the
[SEED_NEW_BOOK.md](SEED_NEW_BOOK.md) prompt — it drives this same pipeline and
always interviews for the author's speaking voice first.

---

## The Pipeline

A book moves through the following stages. Early books were built ad hoc; new
books should follow this order.

1. **Outline.** Write a single outline file at the book root
   (`<subject>_outline.md` or `OUTLINE.md`) that fixes the units, chapters, and
   the through-line of the argument before any prose is written. The outline is
   the contract; everything downstream refers back to it.

2. **Expand units and chapters.** Fill the hierarchy top-down: a unit
   introduction that orients the reader, then chapters, then sections, then the
   finest-grained subsection files. Prose is written to a consistent voice —
   rigorous but readable, motivating each idea before formalizing it.

3. **Per-chapter back-matter.** Each chapter closes with the standard apparatus:
   exercises (with a range of difficulty), thought experiments, lab or
   programming projects where applicable, a glossary of key concepts, short
   profiles of key researchers, and an annotated bibliography. Support files
   (`exercises.md`, `further_reading.md`, `references_and_primary_sources.md`,
   etc.) live at the level they describe.

4. **Wire up `subject.toml`.** Declare the quiz's view of the book: the chapter
   list, the phase groupings, the generator's system prompt, and (optionally)
   the model to use. See [Quiz configuration](#quiz-configuration-subjecttoml).

5. **Generate the question bank.** Run the quiz-core generator to populate
   `questions/` (roughly ten questions per chapter). See
   [Question-bank generation](#question-bank-generation).

6. **Wire up `book.toml` and build.** Declare the build (title, author, source
   order, output formats) and produce a PDF/HTML/Markdown artifact with
   `tools/build_book.py`. See [Building](#building).

7. **Validate.** Run `tools/validate.py` and fix anything it flags. CI runs the
   same checks plus the quiz test suite. See [Validation and CI](#validation-and-ci).

---

## Directory Conventions for New Books

The fourteen existing top-level directories use inconsistent casing
(`Abstract-Algebra` alongside `PhotonicComputing`); normalizing them was
deliberately skipped to avoid churn (see [Deferred work](#deferred-work)). New
books should follow these conventions:

- **Top-level directory:** kebab-case (e.g. `quantum-information/`).
- **Nesting:** `unit-NN-slug/` → `chNN-slug/` (or `NN_Chapter_Name/`) →
  section files. Keep the depth consistent within a book so the builder can map
  directory depth to heading levels.
- **Intro-file convention:** the orienting file at each level is a fixed name so
  tooling and readers can find it — use `00_introduction.md` (or
  `unit_intro.md`) at the top of each unit/chapter directory. Pick one
  convention per book and hold to it.
- **Required files at the book root:**
  - `book.toml` — build configuration.
  - `subject.toml` — quiz configuration.
  - `questions/` — the generated question bank.
  - an outline file — the structural contract.

### Question-bank layout

Questions are stored as one JSON file per question, grouped by chapter:

```
<BookDir>/questions/
├── ch01/
│   ├── 001.json
│   ├── 002.json
│   └── …
├── ch02/
│   └── …
└── …
```

---

## The Question JSON Schema

Every question is a single JSON object. This is the canonical schema the quiz
engine reads and the generator writes:

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `question_id` | string (UUID) | optional | Stable identifier; assigned when present |
| `chapter` | int | yes | 1-based chapter number |
| `phase` | int | yes | Phase group this chapter belongs to (see `subject.toml`) |
| `kind` | `"mc"` \| `"tf"` \| `"blank"` | yes | Multiple-choice, true/false, or fill-in-the-blank |
| `text` | string | yes | The question stem |
| `choices` | string[] | yes for `mc`/`tf` | Answer options |
| `answer` | int | yes | Index into `choices` of the correct option |
| `explanation` | string | yes | Why the answer is correct (shown after answering) |
| `tags` | string[] | yes | Topic tags for filtering and review |
| `difficulty` | `"beginner"` \| `"intermediate"` \| `"advanced"` | yes | Selection difficulty |
| `generated` | bool | optional | `true` if produced by the generator rather than hand-authored |

Example:

```json
{
  "chapter": 1,
  "phase": 0,
  "kind": "mc",
  "text": "Gettier's 1963 paper showed that the JTB analysis of knowledge fails because:",
  "choices": [
    "Justification is not necessary for knowledge",
    "Truth is not necessary for knowledge",
    "Justified true belief is not sufficient for knowledge",
    "Knowledge is impossible under radical skepticism"
  ],
  "answer": 2,
  "explanation": "Gettier constructed cases of justified true belief that intuitively fail to be knowledge, showing JTB is insufficient and launching the search for a fourth condition.",
  "tags": ["Gettier", "JTB", "knowledge-analysis"],
  "difficulty": "beginner"
}
```

---

## Quiz Configuration (`subject.toml`)

`subject.toml` at each book root tells the quiz engine how to read the book.
Its keys:

- `title` — display name of the quiz.
- `chapters_dir` — path (relative to `subject.toml`) to the directory holding
  the chapter source files.
- `model` — *optional*; the Claude model the generator should use for this book.
  When present it takes priority; otherwise the generator falls back to the
  `ANTHROPIC_MODEL` environment variable and then the default (see below).
- `system_prompt` — the generator's instructions: the subject, the topics to
  cover, and the standard for rigor and distractor quality.
- `[[chapters]]` — one table per chapter, with `index`, `phase`, `name`, and
  `file` (the source file, relative to `chapters_dir`, that the generator reads
  as context).
- `[[phases]]` — one table per phase, with `index` and `name`, grouping
  chapters into progression stages.

---

## Question-Bank Generation

Questions are generated by the generator in `quiz-core`, which reads a book's
`subject.toml`, feeds each chapter's source file to the Claude API, and writes
the resulting questions into `questions/`.

- **Authentication:** set `ANTHROPIC_API_KEY` in your environment. Generation
  will not run without it. (Taking an already-generated quiz needs no key.)
- **Model selection**, in precedence order:
  1. the `model` key in the book's `subject.toml`, if present;
  2. the `ANTHROPIC_MODEL` environment variable, if set;
  3. the default, `claude-sonnet-5`.
- **Volume:** target roughly ten questions per chapter, spread across the three
  difficulty levels.

Generated files should be committed so that quizzing works offline and results
are reproducible.

---

## Building

Books are assembled by the shared builder, driven by each book's `book.toml`:

```bash
python3 tools/build_book.py <BookDir> --pdf        # PDF via pandoc + xelatex
python3 tools/build_book.py <BookDir> --html       # standalone HTML
python3 tools/build_book.py <BookDir> --markdown   # single concatenated Markdown
python3 tools/build_book.py <BookDir> --check      # dry run: structure only, no output
```

Output is written to `<BookDir>/output/`, which is gitignored — **no built
artifacts (PDFs, combined Markdown) are committed to the repository.** The
builder walks the book's directory tree in order and shifts Markdown heading
levels to match directory depth, so consistent nesting matters.

PDF builds require `pandoc` (≥ 3.0) and `xelatex`.

---

## Validation and CI

Before opening a PR, run:

```bash
python3 tools/validate.py
```

The validator checks repository-wide invariants — that each book's `book.toml`
and `subject.toml` are well-formed, that `subject.toml` chapter files exist,
that questions parse against the schema above, and that no built artifacts have
been committed.

GitHub Actions runs the same `validate.py` plus `cargo test` on the `quiz/`
workspace for every push and pull request. Both must pass to merge.

---

## Deferred Work

The following were consciously left for later:

- **Exercise solutions.** Abstract-Algebra, BasalCognition, and Dynamical-Systems
  ship exercises without full worked solutions. (Differential-Equations and
  several others do include `solutions.md` files.)
- **License.** The project is licensed CC BY-NC-SA 4.0 (free, non-commercial,
  share-alike) — see [LICENSE](LICENSE). New books inherit this license; do not
  add content under incompatible terms.
- **Directory-name normalization.** The fourteen existing book directories mix
  kebab-case and PascalCase. Renaming them to a single convention was
  deliberately skipped to avoid large-scale churn and broken references; only
  *new* books are held to the kebab-case convention above.
