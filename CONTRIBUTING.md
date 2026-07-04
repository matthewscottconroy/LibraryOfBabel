# Contributing

Thanks for helping improve LibraryOfBabel. This is a short guide; the full
pipeline lives in [PROCESS.md](PROCESS.md).

## Before You Open a PR

Run the validator from the repository root and make sure it passes:

```bash
python3 tools/validate.py
```

CI runs the same check plus `cargo test` on the `quiz/` workspace. Both must be
green to merge.

## Content Conventions

- **Voice:** rigorous but readable. Motivate an idea before formalizing it; say
  so plainly when something is contested or unresolved.
- **Structure:** keep directory nesting consistent within a book (unit →
  chapter → section → subsection). The builder maps directory depth to heading
  levels, so irregular nesting breaks the PDF.
- **Back-matter:** each chapter ends with the standard apparatus — exercises,
  key concepts, key researchers, and an annotated bibliography. Support files
  (`exercises.md`, `further_reading.md`, and the like) sit at the level they
  describe.
- **No built artifacts.** Do not commit anything under a book's `output/`
  directory (PDFs, combined Markdown). Builds are reproducible from source.
- **Citations:** cite non-obvious claims; flag preliminary or contested findings
  in the text.

## Questions

Quiz questions are one JSON file per question under `<BookDir>/questions/`. Use
the canonical schema — fields, types, and an example are in
[PROCESS.md](PROCESS.md#the-question-json-schema). New or edited questions must
parse cleanly (the validator enforces this). Generating questions calls the
Claude API and needs `ANTHROPIC_API_KEY`; see
[PROCESS.md](PROCESS.md#question-bank-generation).

## Adding a Book

New books follow the directory conventions, required files, and stage order in
[PROCESS.md](PROCESS.md#directory-conventions-for-new-books). In brief: a
kebab-case top-level directory, an outline, consistent unit/chapter/section
nesting, and `book.toml`, `subject.toml`, and `questions/` at the root.
