# Programs and Machines

**A First Course in Computer Science, in Java**

A textbook that starts at voltage and finishes at undecidability, using Java as
its instrument. No prior programming is assumed.

## What makes this book different

Most introductions to Java teach the language and hope the computer science
arrives by osmosis. This one inverts that. Every Java feature is introduced only
after the idea underneath it has been built, so the syntax arrives as an answer
to a question you are already asking.

Java does not appear until Chapter 5. By then you will know why a fixed-width
integer must wrap around, and `int` will be an instance of something you
understand rather than a rule you were handed.

The book has one claim, made eight times over:

> A computer holds patterns and changes them. Everything else — numbers, text,
> objects, programs, meaning — is an agreement we have layered on top.

## Structure

| Unit | Title | What it settles |
|---|---|---|
| I | Representation | What a machine can hold: bits, integers, floats, text |
| II | Computation | State, the step, choice, repetition, the loop invariant |
| III | Abstraction by Procedure | Methods, the call stack, parameter passing, recursion |
| IV | Compound Data | Arrays, collections, generics, text processing |
| V | Objects, State, and Identity | Classes, references, equality, inheritance, polymorphism |
| VI | Programs as Data | Grammars, parsing, and an interpreter we write ourselves |
| VII | The World Outside the Program | Exceptions, files, events, concurrency |
| VIII | Limits and Cost | Complexity, information, undecidability |

Full blueprint: [the outline](programs_and_machines_outline.md).

## Who it is for

Someone taking a first or second course in programming, or teaching themselves.
The only prerequisite is comfort with high-school algebra.

A reader who finishes should find the standard second-semester Java topics —
arrays and `ArrayList`, inheritance and polymorphism, text processing, wrapper
classes, file I/O, exceptions, GUIs, recursion — already familiar, because each
will have been derived rather than announced.

## Building the book

From the repository root:

```bash
python3 tools/build_book.py programs-and-machines --pdf       # PDF
python3 tools/build_book.py programs-and-machines --markdown out.md
python3 tools/build_book.py programs-and-machines --check     # manifest coverage
```

## Running the quiz

```bash
cd quiz && cargo run -q -p quiz-cli -- --subject ../programs-and-machines --stats
```

## Validating

```bash
python3 tools/validate.py --book programs-and-machines   # repo-wide checks
cd programs-and-machines && python3 tools-lint.py        # this book's contract
```

`tools-lint.py` parses the outline and checks the tree against it, along with the
prose failure modes that have each cost a rebuild. It must print `clean` before a
commit. `tools-fix.py` repairs the mechanical ones.

## Conventions

- Directories are kebab-case. Units open with `intro.md`; chapters and sections
  open with `README.md`.
- Heading levels come from directory depth: part → chapter → section → lesson.
  Unit intros and chapter overviews are flowing prose without subheadings, so
  that a third-level heading always means "a section".
- Every chapter carries `exercises.md`, `further-reading.md`,
  `important-concepts.md`, and `important-researchers.md`.
- US spelling. Code is Java 17+, and every program compiles and runs as printed.
- Exercises marked **[carries forward]** introduce something a later chapter
  assumes.

## Licence

Inherits the repository license: [CC BY-NC-SA 4.0](../LICENSE).
