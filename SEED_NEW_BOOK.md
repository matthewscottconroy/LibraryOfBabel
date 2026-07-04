# Seed Prompt — Generate a New Textbook

This document is a **reusable prompt**. Give it (or point an AI agent at it) to
produce a brand-new textbook + adaptive-quiz program that slots into this
repository in the **same format as the fourteen existing books**, passes
`python3 tools/validate.py`, and loads in the quiz engine.

It is the "seed" the root README promises: a repeatable pipeline for generating
accurate, pedagogically sound, self-contained learning material on demand.

> Companion reference: [PROCESS.md](PROCESS.md) documents the directory
> conventions, the `subject.toml` / `book.toml` shapes, and the question schema
> in full. This file is the *prompt*; PROCESS.md is the *spec*. When a detail
> here is terse, defer to PROCESS.md and to an existing book (e.g.
> `ReservoirComputing/`, `PhilosophyOfTime/`) as the worked example.

---

## HOW TO USE

Paste everything below the line into a capable coding agent working inside this
repository, or invoke it however you drive generation. The agent must run the
**interview first**, then scaffold, then generate, then validate. Do not skip
the interview — in particular, **the author's speaking voice must always be
elicited and never assumed.**

===============================================================================
PROMPT BEGINS
===============================================================================

You are going to write a complete graduate-level textbook and its adaptive-quiz
program, as a new top-level directory in the LibraryOfBabel repository, matching
the structure, depth, and tooling of the existing books. Work in phases and do
not proceed to writing prose until Phase 0 is complete.

## Phase 0 — Interview the user (MANDATORY; ask, do not assume)

Ask the user the following before writing anything. Ask them together, accept
their answers, and confirm your understanding back to them in a short spec.

1. **Subject and title.** What is the book about? What should it be called?

2. **The author's speaking voice / tone / style — ALWAYS ASK THIS; NEVER PICK IT
   YOURSELF.** This is the single most important input and must be captured in
   detail, because it governs every sentence you write. Elicit a concrete voice
   specification, not a one-word answer. Prompt for:
   - Register and formality (e.g. rigorous-and-formal; warm-and-conversational;
     dry-and-witty; Socratic; lecture-from-a-beloved-professor; terse-and-
     technical; lyrical-and-historical).
   - Person and stance (first-person "I"/"we", or impersonal; opinionated or
     neutral; does the author argue and take sides, or present-and-let-decide?).
   - Sentence rhythm and vocabulary (long periodic sentences vs. short punchy
     ones; plain words vs. specialist diction; how much metaphor/analogy).
   - Use of humor, historical asides, worked-example density, direct address to
     the reader, epigraphs, and signposting.
   - Any authors/books whose voice to emulate (e.g. "like Feynman", "like
     Nagel", "like a Terence Tao blog post") — as a *reference*, not a copy.
   If the user gives a thin answer, ask a focused follow-up or offer 3–4
   concrete voice options to react to. Then **write a 4–8 sentence VOICE SPEC**
   and get the user's sign-off. **Thread this voice consistently through 100% of
   the generated prose** — intros, sections, exercises, explanations, even
   question explanations. When you fan work out to sub-agents, pass the VOICE
   SPEC verbatim to each so the voice stays uniform.

3. **Audience and level.** Who is it for (advanced undergrad, graduate,
   self-learner with X background)? What prerequisites may be assumed?

4. **Scope and shape.** Roughly how many units and chapters? Is there a natural
   phase/tier progression (foundations → core → frontier)? Any must-cover topics
   or explicit non-goals?

5. **Notation and conventions.** Field-specific conventions to fix up front
   (e.g. metric signature, index conventions, which definitions/terminology).

6. **Companion assets (optional).** Does the book warrant runnable labs/demos
   (like ReservoirComputing's `labs/` or Abstract-Algebra's `demos/`), an
   adaptive tutor (like Differential-Equations' `tutor.py`), or is it prose +
   quiz only?

7. **Directory name.** Pick a top-level directory name. Match the repo's
   dominant style; kebab-case is preferred for new books (e.g.
   `Quantum-Information`).

## Phase 1 — Outline

Write a single `<subject>_outline.md` (or `OUTLINE.md`) at the book root: the
full hierarchical blueprint — units → chapters → sections → subsections — with a
one-line summary per node, plus per-chapter Key Concepts / Key Figures /
Exercises / Further Reading stubs. This is the source of truth for structure;
everything downstream follows it. Confirm the outline with the user before
expanding.

## Phase 2 — Directory conventions (match the existing books exactly)

Create, under the new book directory:

```
<Book>/
  README.md                 # front door: what it is, who it's for, how to
                            #   build the PDF and run the quiz, start-here
  <subject>_outline.md      # the Phase 1 blueprint (kept; not built into PDF)
  book.toml                 # build manifest (see below)
  subject.toml              # quiz config (see below)
  book/                     # OR numbered topic dirs at root — pick ONE layout
    preface.md              #   and use it consistently
    unit-01-<slug>/
      intro.md              # unit intro (an "intro-like" file sorts first)
      chapter-01-<slug>/
        README.md           # chapter overview (400–800 words)
        section-01-<slug>/
          01-<topic>.md     # the actual lesson prose (~800–1500 words)
          02-<topic>.md
        exercises.md        # per-chapter back matter
        further-reading.md
        important-concepts.md
        important-researchers.md
      ...
    appendices/             # optional
  questions/                # one JSON file per question (schema below)
    ch00/001.json ...
```

Conventions:
- Pick ONE nesting/intro-file convention and hold it for the whole book. The
  existing books vary (`intro.md` vs `introduction.md` vs `00-unit-survey.md`
  vs `README.md` as body; kebab vs snake) — internal consistency matters more
  than which you pick. Prefer kebab-case dirs and `intro.md`/`README.md` intros
  for a new book.
- Every chapter carries the four back-matter files (exercises, further-reading,
  important-concepts, important-researchers).
- No zero-byte files. Every prose file is finished, in the VOICE SPEC, with
  real citations only (never invent authors, years, papers, or numbers).

## Phase 3 — Write the content

Expand the outline into finished prose, unit by unit, in the VOICE SPEC.
Graduate-level rigor; worked examples with real numbers/derivations; correct
domain content. Fan out across sub-agents if available, but give each the VOICE
SPEC, the outline slice, and the notation conventions so output is uniform and
book-grounded. Verify: `find <Book> -name '*.md' -empty` returns nothing.

## Phase 4 — Wire the quiz config (`subject.toml`)

At the book root. `chapters_dir` is relative to the book root; each `[[chapters]]`
entry maps a quiz "chapter" to a real content file that exists on disk.

```toml
title = '<Book> Adaptive Quiz'
chapters_dir = './'                 # or './book', './curriculum', etc.
system_prompt = """You are an expert quiz question generator for <subject> ...
List the topics; state the required precision/level."""

[[chapters]]
index = 0
phase = 0
name = 'First Unit Name'
file = 'unit-01-<slug>/intro.md'    # MUST resolve under chapters_dir
# ... one block per chapter; index is 0-based; phase groups chapters into tiers
```

Validate every `file` resolves (`python3 tools/validate.py` checks this).

## Phase 5 — Generate the question bank (`questions/chNN/NNN.json`)

Write ~10–12 questions per chapter, one JSON object per file, grounded in the
book's own content (read the chapters you wrote). **Match this schema exactly —
the Rust quiz engine is strict about the per-kind `answer` typing:**

```json
{
  "question_id": "<uuid 8-4-4-4-12 hex, distinct per question>",
  "chapter": 0,                     // int, matches the subject.toml index
  "phase": 0,                       // int, matches the subject.toml phase
  "kind": "mc",                     // "mc" | "tf" | "blank"
  "text": "…",
  "choices": ["…", "…", "…", "…"],  // mc: 4 strings; tf: ["True","False"];
                                    //   blank: acceptable-answer strings
  "answer": 1,                      // mc/tf: INTEGER index into choices.
                                    //   blank: the canonical answer STRING
                                    //   (equal to choices[0]) — NOT an index.
  "explanation": "why right AND why the tempting distractors are wrong",
  "tags": ["lowercase-topic"],
  "difficulty": "beginner",         // beginner | intermediate | advanced
  "generated": true
}
```

Rules: vary which mc index is correct (never always 0); distractors must be
plausible misconceptions, not filler; difficulty mix ~40/40/20; explanations
carry the VOICE SPEC too. For `blank`, `text` contains a literal `___` and
`answer` is the string `choices[0]` (a common mistake is to write `answer: 0` —
the engine rejects that).

## Phase 6 — Build manifest (`book.toml`) and verification

Write `book.toml` so `tools/build_book.py <Book> --check` captures every content
file (recurse the source root; set `part_level` for `\part` injection; list any
`front_matter`/`back_matter`; `exclude` the outline and anything not meant for
the PDF):

```toml
title = "<Book>"
subtitle = "…"
front_matter = ["book/preface.md"]
[[sources]]
root = "book"
recursive = true
part_level = 1
```

Then verify (all must pass, no exceptions):
- `python3 tools/validate.py` → the new book is OK, 0 unexpected failures.
- `python3 tools/build_book.py <Book> --check` → every content file captured.
- `cd quiz && cargo run -q -p quiz-cli -- --subject ../<Book> --stats` → prints a
  per-chapter TOTAL with no error (proves the bank loads in the engine).
- Optionally build a markdown artifact (`--markdown out.md`) and sanity-check
  its size/word count against the prose you wrote.

## Phase 7 — Docs and licensing

- Write the book's `README.md` (front door + the three commands: build, quiz,
  validate). Add a one-line entry for the new book to the root `README.md`
  table.
- The new book inherits the repository's license automatically
  ([CC BY-NC-SA 4.0](LICENSE) — free, non-commercial, share-alike). Do not add
  content under incompatible terms.

## Deliverable

A new top-level book directory that is internally consistent, written entirely
in the user's specified VOICE SPEC, complete (no empty files), with a full
question bank that validates and loads in the engine, and correct `subject.toml`
/ `book.toml`. Report per-unit word counts, the question count, and the three
verification results.

===============================================================================
PROMPT ENDS
===============================================================================

---

## Note to maintainers

The one non-negotiable that distinguishes a good result here is **voice**. Every
existing book has a consistent authorial character; a new book must too, and
that character is the user's to choose, never the generator's to default. The
interview step exists to force that choice. If you adapt this seed, keep the
"always prompt for the author's speaking voice" requirement intact.
