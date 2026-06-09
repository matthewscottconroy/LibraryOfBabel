# Abstract Algebra Adaptive Quiz — Documentation

A terminal-based, adaptive quiz system covering all 28 chapters of the Abstract Algebra curriculum: from logic and set theory through Lie theory, representation theory, and the foundations of mathematics. Questions come from a curated static bank (160+) and from Claude AI on demand. Progress is tracked per-chapter with spaced repetition.

---

## Quick Start

```bash
# Interactive menus (recommended for first use)
python3 -m quiz_app

# Direct-mode shortcuts
python3 -m quiz_app --chapter 8 --n 15       # 15 questions on Chapter 8
python3 -m quiz_app --phase 2                 # quiz on all of Phase 2
python3 -m quiz_app --tag groups              # questions tagged "groups"
python3 -m quiz_app --study 12               # read chapter 12, then optionally quiz
python3 -m quiz_app --profile Alice           # load a named profile

# Profile management
python3 -m quiz_app --list-profiles           # print all saved profiles
python3 -m quiz_app --delete-profile Alice    # delete a profile
python3 -m quiz_app --profile Alice --export  # export progress CSV to stdout
```

### AI-generated questions

Set `ANTHROPIC_API_KEY` in your environment to enable Claude-generated questions (35% of each session by default). Without the key, the quiz falls back entirely to the static bank.

```bash
export ANTHROPIC_API_KEY=sk-ant-...
python3 -m quiz_app
```

---

## Architecture

```
quiz_app/
├── __main__.py        # entry point: python3 -m quiz_app
├── main.py            # argparse CLI, onboarding flow, main event loop
├── models.py          # Question, MasteryRecord, UserProfile, SessionResult
├── config.py          # constants, CHAPTER_META, PHASE_NAMES
├── adaptive.py        # Scope, weighted question selection, phase gating
├── session.py         # QuizSession: orchestrates one quiz run
├── generator.py       # ClaudeGenerator: API calls + disk cache
├── storage.py         # JSON persistence for UserProfile
├── question_bank.py   # 160+ static questions (mc, tf, fill-in-the-blank)
└── ui.py              # ANSI terminal rendering, input helpers
```

### Module responsibilities

| Module | Responsibility |
|---|---|
| `models` | Pure data: dataclasses with to_dict / from_dict, mastery arithmetic |
| `config` | Single source of truth for constants and chapter/phase metadata |
| `adaptive` | Question selection algorithm; no I/O |
| `session` | Orchestrates one run: assembles pool, calls UI, updates mastery |
| `generator` | Claude API with disk cache; caller-transparent fallback to None |
| `storage` | Atomic JSON load/save; profile CRUD |
| `ui` | All terminal output and input; no business logic |
| `main` | Wires everything together; handles CLI args and the interactive menu loop |

---

## Data Models

### `Question`

```python
@dataclass
class Question:
    chapter:     int          # 0–27
    phase:       int          # 0–8
    kind:        str          # "mc" | "tf" | "blank"
    text:        str
    choices:     list[str]    # mc: 2–4 choices; tf: ["True","False"]; blank: acceptable answers
    answer:      int | str    # index for mc/tf; canonical string for blank
    explanation: str
    tags:        list[str]
    difficulty:  str          # "beginner" | "intermediate" | "advanced"
    generated:   bool         # True if produced by Claude
    question_id: str          # UUID
```

Validate with `q.validate() -> list[str]` — returns a list of error strings (empty = valid).

### `MasteryRecord`

Tracks per-chapter learning state.

```python
@dataclass
class MasteryRecord:
    score:         float   # 0.0 (none) → 1.0 (expert), starts at 0.5
    total_seen:    int
    total_correct: int
    last_seen:     str     # ISO datetime
    next_review:   str     # ISO date for spaced repetition ("" = unscheduled)
```

Key properties: `accuracy`, `is_due`.

### `UserProfile`

```python
@dataclass
class UserProfile:
    user_id:           str
    name:              str
    mastery:           dict[int, MasteryRecord]   # chapter → record
    seen_question_ids: list[str]                  # last 200, for recency
    session_history:   list[dict]                 # SessionResult.to_dict() entries
    onboarded:         bool
    created_at:        str
    last_seen:         str
```

Key methods: `record_answer(chapter, correct, question_id)`, `chapters_due_for_review()`, `weakest_chapters(top_n)`.

### `SessionResult`

Immutable record of one completed quiz run, stored in `profile.session_history`.

```python
@dataclass
class SessionResult:
    session_id:     str
    timestamp:      str
    scope_label:    str
    n_questions:    int
    n_correct:      int
    duration_secs:  float
    wrong_chapters: list[int]
    per_difficulty: dict[str, list[int]]   # {"beginner": [correct, total], ...}
    streak_max:     int
    n_generated:    int
```

Property: `score_pct -> int` (0–100).

---

## Adaptive Selection Algorithm

`adaptive.select_questions(pool, profile, n, scope)` assigns a weight to every candidate question and samples without replacement.

**Weight for question `q`:**
```
weight(q) = chapter_weight(q.chapter)
           × difficulty_weight(q.difficulty, mastery(q.chapter))
           × recency_factor(q.question_id)
```

### Chapter weight (`adaptive`)

| Condition | Effect |
|---|---|
| Chapter not yet seen | weight × 0.8 (moderate exploration boost) |
| Chapter seen | weight = max(0.1, 1.2 − mastery_score) |
| Chapter due for review | weight × 2.0 (spaced-repetition override) |
| Chapter far ahead of phase frontier | weight × phase_gate_factor |

### Phase gate

| Phase gap (chapter_phase − frontier) | Factor |
|---|---|
| ≤ 1 | 1.0 (no penalty) |
| 2 | 0.5 |
| 3 | 0.2 |
| ≥ 4 | 0.05 |

The "frontier" is the highest phase in which the user has seen ≥ 3 questions in any chapter.

### Difficulty matching

| User's mastery | Preferred difficulty |
|---|---|
| < 0.35 | beginner |
| 0.35–0.68 | intermediate |
| ≥ 0.68 | advanced |

A question at the preferred difficulty gets weight 1.0. One step away: 0.35. Two steps away: 0.05.

### Recency

Questions seen in the last 60 are penalised by a factor of 0.05.

### Spaced repetition intervals

| Mastery score | Correct | Next review |
|---|---|---|
| Any | Wrong | +1 day |
| < 0.50 | Correct | +1 day |
| 0.50–0.70 | Correct | +3 days |
| 0.70–0.90 | Correct | +7 days |
| ≥ 0.90 | Correct | +14 days |

---

## Question Types

### Multiple Choice (`mc`)

- 2–4 choices (static bank always uses 4, Claude always generates 4).
- `answer`: 0-based index of the correct choice.
- Presented as A/B/C/D.

### True/False (`tf`)

- Exactly 2 choices: `["True", "False"]`.
- `answer`: 0 (True) or 1 (False).
- Accepts "A"/"T"/"True" or "B"/"F"/"False".

### Fill-in-the-Blank (`blank`)

- `choices`: list of acceptable answers (synonyms/abbreviations).
- `answer`: canonical string (typically `choices[0].lower()`).
- Matching rules: exact match (case-insensitive, whitespace-normalised) or the response is a suffix of an acceptable answer (≥ 4 characters). Short substrings are not accepted to prevent trivially matching any sentence containing a keyword.

---

## Claude Question Generator

`ClaudeGenerator` produces multiple-choice questions on demand.

**Cache-first policy:**
1. Try to pop a pre-generated question from disk (`quiz_app/data/cache/ch{N}_{diff}.json`).
2. If cache is empty, call the Claude API.
3. On success, background-generate 2 more into the cache (without triggering another background fill — the `_from_prefill` guard prevents infinite recursion).
4. On failure (API error, invalid JSON, validation failure), return `None`; the session falls back to a static question.

The cache cap is 20 questions per chapter per difficulty level (configurable via `CACHE_CAP_PER_DIFFICULTY` in config.py).

After a session, `QuizSession._start_background_prefill()` pre-warms the cache for the user's 3 weakest chapters (5 questions per difficulty).

---

## Persistence

Profiles are stored as JSON in `quiz_app/data/progress/<sanitised-name>.json`. Writes are atomic: data is written to a `.tmp` file first, then renamed.

```
quiz_app/data/
├── progress/
│   ├── alice.json
│   └── bob.json
└── cache/
    ├── ch00_beginner.json
    ├── ch00_intermediate.json
    └── ch08_advanced.json
```

Profile files are never loaded at startup — only on profile selection — so a corrupt profile for one user doesn't affect others.

---

## Curriculum Coverage

28 chapters across 9 phases:

| Phase | Chapters | Topics |
|---|---|---|
| 0 | 0–1 | Logic, Sets, Proof, Cardinality |
| 1 | 2–7 | Linear Algebra (vector spaces → tensors) |
| 2 | 8–9 | Group Theory |
| 3 | 10–12 | Rings, Modules, Fields & Galois Theory |
| 4 | 13 | Category Theory |
| 5 | 14–16 | Homological Algebra, Derived Functors, Spectral Sequences |
| 6 | 17–18 | Representations of Finite Groups, Character Theory |
| 7 | 19–22 | Lie Groups & Algebras, Highest Weight Theory, Advanced Repr. |
| 8 | 23–26 | Set Theory, Model Theory, Categorical Foundations, HoTT |

Chapter 27 is the overview/index.

---

## Running Tests

```bash
# All tests
python -m pytest tests/ -v

# A single module
python -m pytest tests/test_models.py -v

# With coverage (if pytest-cov installed)
python -m pytest tests/ --cov=quiz_app --cov-report=term-missing
```

The test suite (148 tests) covers:

| File | What's tested |
|---|---|
| `test_models.py` | Question validation (all 3 kinds), MasteryRecord scheduling, UserProfile mastery tracking, SessionResult round-trip |
| `test_adaptive.py` | Scope, preferred_difficulty, difficulty_weight, filter_by_scope, select_questions, phase gate, mastery_summary, weak_topics |
| `test_storage.py` | Profile path sanitisation, save/load round-trip, atomic write, corrupt-file handling, fallback search, CRUD |
| `test_generator.py` | Cache FIFO, cap enforcement, chapter/difficulty isolation, _parse_and_validate (valid + 6 failure modes), ClaudeGenerator availability + cache-hit path |
| `test_ui.py` | _normalize_blank, _blank_matches (10 cases), progress_bar |
| `test_session.py` | _interleave, _target_chapter_and_difficulty, QuizSession.run (score, streak, wrong chapters, mastery update, history), empty pool, generator fallback |

All tests use only `pytest` and the standard library. They mock `ui`, `save_profile`, and `select_questions` where needed so no terminal I/O or disk writes occur during the session tests.

---

## Configuration Reference

`quiz_app/config.py`

| Constant | Default | Meaning |
|---|---|---|
| `CLAUDE_MODEL` | `claude-sonnet-4-6` | Model used for generation |
| `CHAPTER_EXCERPT_CHARS` | 4000 | Characters of chapter markdown sent to Claude |
| `GENERATE_BATCH_SIZE` | 5 | Target cache size per prefill run |
| `CACHE_CAP_PER_DIFFICULTY` | 20 | Max cached questions per (chapter, difficulty) |
| `LEARNING_RATE_CORRECT` | 0.15 | Mastery score increase on correct answer |
| `LEARNING_RATE_WRONG` | 0.10 | Mastery score decrease on wrong answer |
| `INITIAL_MASTERY` | 0.50 | Starting mastery for any chapter |
| `RECENCY_PENALTY` | 0.05 | Weight factor applied to recently-seen questions |
| `RECENCY_WINDOW` | 60 | Number of recent question IDs tracked |
| `DIFF_BEGINNER_MAX` | 0.35 | Mastery below this → prefer beginner questions |
| `DIFF_INTERMEDIATE_MAX` | 0.68 | Mastery below this → prefer intermediate questions |

---

## Known Limitations

- **Fill-in-the-blank** questions require typing recognisable keywords. Synonyms not listed in `choices` are not accepted, even if mathematically equivalent.
- **Claude generation** only produces multiple-choice questions (4 choices). True/false and fill-in-the-blank questions come entirely from the static bank.
- **Thread safety**: the background cache prefill runs in a daemon thread that reads and writes cache files without locking. In practice a race is harmless (worst case: one extra question is dropped from the cache), but it is not formally safe for concurrent quiz sessions with a shared data directory.
- **Profile names** are sanitised to `[a-z0-9\-_]`. Two names that differ only in punctuation (e.g. "O'Brien" and "OBrien") will share a profile file.
