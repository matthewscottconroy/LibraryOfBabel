# Adaptive Quiz (Library of Babel)

A single Rust workspace that powers the adaptive quiz for **every** book in this
repository. Point it at any book's directory and it loads that book's
`subject.toml` plus its question bank and runs an adaptive quiz — SM-2 spaced
repetition, mastery-weighted question selection, per-chapter mastery tracking,
session history, and progress export.

Previously each book carried its own byte-identical copy of this app under
`<Book>/quiz-app/`. Those copies have been consolidated here; each book now
keeps only its own `subject.toml` (and its `questions/` bank).

## Layout

```
quiz/
├── Cargo.toml         # virtual workspace
├── Cargo.lock         # committed (do not gitignore)
├── quiz-core/         # the engine (library) — all quiz logic lives here
│   ├── src/
│   │   ├── question.rs   # Question enum (mc/tf/blank/proof) + bank loaders
│   │   ├── config.rs     # subject.toml loader + subject resolution
│   │   ├── mastery.rs     # SM-2 MasteryRecord
│   │   ├── profile.rs     # UserProfile, SessionResult
│   │   ├── adaptive.rs    # Scope + select_questions (mastery/phase weighting)
│   │   ├── session.rs     # Session tracker
│   │   ├── storage.rs     # atomic JSON profile persistence
│   │   └── generator.rs   # Anthropic API question generation + cache
│   └── tests/            # integration test + fixtures (real seed files)
├── quiz-cli/          # interactive terminal app (ANSI menus)
├── quiz-tui/          # full-screen TUI (ratatui)
└── quiz-web/          # local web app (axum), http://localhost:3000
```

`quiz-core` is the single source of truth. The three front-ends (`cli`, `tui`,
`web`) are thin shells over it.

## Running

All binaries take a `--subject <dir>` flag pointing at a book directory (the one
that contains `subject.toml`). Resolution order:

1. `--subject <dir>` (or `-s <dir>`)
2. the `QUIZ_SUBJECT` environment variable
3. the current working directory

```sh
cd quiz

# Interactive CLI against any book:
cargo run -p quiz-cli -- --subject ../Homotopy-Type-Theory
cargo run -p quiz-cli -- --subject ../Epistemology

# Full-screen TUI:
cargo run -p quiz-tui -- --subject ../PhotonicComputing

# Local web app (serves http://localhost:3000):
cargo run -p quiz-web -- --subject ../General-Relativity

# Or via the environment variable:
QUIZ_SUBJECT=../ReservoirComputing cargo run -p quiz-cli
```

Print per-chapter question counts and exit (no interaction):

```sh
cargo run -p quiz-cli -- --subject ../Homotopy-Type-Theory --stats
```

`quiz-tui` and `quiz-web` also accept a single positional quiz file
(`{title, questions: [...]}` JSON object) as a secondary mode:

```sh
cargo run -p quiz-web -- path/to/one-quiz.json
```

## Question bank layout

A subject's bank lives at `<subject_dir>/questions/`, organised into chapter
subdirectories whose names start with `ch` followed by a number (any suffix):

```
<Book>/questions/
├── ch00-logic-and-proof/
│   ├── 001.json          # one Question per file
│   ├── 002.json
│   └── ...
├── ch01-set-theory/
│   └── ...
```

`load_question_bank` scans every `chNN…` subdirectory recursively and reads all
`*.json` files. Each file may contain **either** a single Question object
**or** a JSON array of Question objects. Files whose name starts with `_` are
skipped (reserved for legacy/unconverted seeds, e.g. `_legacy_sample.json`).

### JSON schema (one Question)

```json
{
  "question_id": "40bf6ac9-b039-4330-8b65-25b2bb4d84bf",
  "chapter": 0,
  "phase": 0,
  "kind": "mc",
  "text": "A proof by contradiction assumes the ___ of the conclusion…",
  "choices": ["converse", "negation", "contrapositive", "obverse"],
  "answer": 1,
  "explanation": "Reductio ad absurdum assumes ¬P and derives ⊥.",
  "tags": ["logic"],
  "difficulty": "beginner",
  "generated": false
}
```

| Field         | Required | Notes |
|---------------|----------|-------|
| `question_id` | optional | UUID string; a fresh one is generated if omitted |
| `chapter`     | yes      | chapter index (matches `subject.toml` `[[chapters]].index`) |
| `phase`       | yes      | curriculum phase index |
| `kind`        | yes      | `mc`, `tf`, `blank`, or `proof` |
| `text`        | yes      | the question / statement / theorem |
| `choices`     | yes      | see per-kind meaning below |
| `answer`      | yes      | index (`mc`/`tf`) or string (`blank`/`proof`) |
| `explanation` | yes      | shown after answering |
| `tags`        | optional | list of strings (default `[]`) |
| `difficulty`  | optional | `beginner`, `intermediate`, or `advanced` (default `intermediate`) |
| `generated`   | optional | `true` if AI-generated (default `false`) |

Per-kind meaning of `choices` / `answer`:

- **`mc`** — `choices` = 2–4 options; `answer` = 0-based index of the correct one.
- **`tf`** — `choices` = `["True","False"]`; `answer` = `0` (True) or `1` (False).
- **`blank`** — `choices` = acceptable answers (primary first); `answer` = the
  primary canonical answer (string, case-insensitive match).
- **`proof`** — `choices` = proof lines (some containing `___`); `answer` =
  pipe-joined (`|`) canonical fills, one per blank in order.

## AI question generation

`quiz-core`'s `generator.rs` can synthesise fresh questions via the Anthropic
Messages API. Configuration:

- **API key** — set `ANTHROPIC_API_KEY` in the environment. If it is absent,
  generation is disabled gracefully (static banks still work).
- **Model** — resolved in priority order:
  1. the `model` key in `subject.toml`
  2. the `ANTHROPIC_MODEL` environment variable
  3. the built-in default (`claude-sonnet-5`)

Example `subject.toml` header:

```toml
title = "Homotopy Type Theory Adaptive Quiz"
model = "claude-sonnet-5"          # optional
chapters_dir = "./chapters"
system_prompt = "You are an expert quiz question generator for…"
```

Generated questions are cached under the subject's data directory so repeat
runs can serve them without another API call.

## Where progress is stored

Each subject keeps its own profiles and exports, namespaced by a slug derived
from its title (lowercase, hyphenated):

```
~/.local/share/quiz/<title-slug>/
├── profiles/            # one <name>.json UserProfile per learner (atomic writes)
└── exports/             # progress_export_YYYY-MM-DD.md reports
```

For example, the "HoTT Adaptive Quiz" title stores profiles under
`~/.local/share/quiz/hott-adaptive-quiz/profiles/`. A corrupt profile file is
renamed to `<name>.backup.json` rather than being lost.

## Development

```sh
cd quiz
cargo check --workspace
cargo test  --workspace
```

`reqwest` is configured with `rustls-tls` (no OpenSSL/native-tls dependency), so
the workspace builds on machines without OpenSSL headers.
