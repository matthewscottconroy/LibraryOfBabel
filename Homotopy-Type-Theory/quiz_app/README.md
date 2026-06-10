# HoTT Adaptive Quiz Application

A self-contained Python package that quizzes you on all 27 chapters of the Homotopy Type Theory curriculum. It tracks your mastery per chapter, adjusts question difficulty and topic focus based on your performance, and optionally calls the Claude API to generate entirely new questions on demand.

---

## Quick Start

```bash
# From the repository root
export ANTHROPIC_API_KEY=your_key_here   # optional — enables AI-generated questions
python3 -m quiz_app
```

No build step. No virtualenv required beyond installing the one optional dependency.

```bash
pip install anthropic   # optional — only needed for AI-generated questions
```

All other functionality (static question bank, adaptive engine, persistence) runs on the Python standard library.

---

## Features

| Feature | Details |
|---------|---------|
| **119 static questions** | All 27 chapters covered, three difficulty levels |
| **Adaptive selection** | Weighted by chapter weakness, difficulty match, and recency |
| **Per-chapter mastery tracking** | Persistent score (0.0–1.0) per chapter, saved between sessions |
| **Multiple profiles** | Each user has a named profile stored in `data/progress/` |
| **AI-generated questions** | Claude reads the actual chapter text to write new questions |
| **Question cache** | Generated questions saved to disk; re-used across sessions |
| **Three question types** | Multiple-choice, true/false, fill-in-the-blank |
| **Mastery dashboard** | Visual █/░ progress bars per chapter organized by phase |

---

## Running the Application

```bash
python3 -m quiz_app
```

On first run you will be prompted for a name. Your profile is stored and loaded automatically on subsequent runs. Multiple profiles can coexist.

### Main Menu

```
[1] Adaptive quiz          — focus on weakest chapters at the right difficulty
[2] Custom quiz            — choose a phase, chapter, or tag
[3] Progress dashboard     — visual mastery overview, all 27 chapters
[4] Switch profile
[5] Quit
```

### Scope options (Custom quiz)

```
[1] All chapters
[2] Single phase    — e.g. Phase 5 (Core HoTT)
[3] Single chapter  — e.g. Chapter 18 (Univalence)
[4] Tag             — e.g. "paths", "category-theory", "cubical"
```

---

## Running the Tests

```bash
# From the repository root
python3 -m pytest quiz_app/tests/ -v
```

96 tests, no network calls, no API key required. All tests use `unittest.mock` or temp directories.

```bash
# Run a specific test file
python3 -m pytest quiz_app/tests/test_adaptive.py -v

# Run with coverage
pip install pytest-cov
python3 -m pytest quiz_app/tests/ --cov=quiz_app --cov-report=term-missing
```

---

## Architecture

```
quiz_app/
├── __init__.py          # package root
├── main.py              # entry point: profile selection, menu loop
├── config.py            # all constants, paths, chapter metadata
├── models.py            # Question, MasteryRecord, UserProfile, SessionResult
├── question_bank.py     # 119 static questions, factory helpers
├── adaptive.py          # difficulty mapping, weighted selection, mastery summaries
├── generator.py         # Claude API integration, caching
├── session.py           # quiz loop, question assembly, profile updates
├── storage.py           # atomic file read/write for profiles
├── ui.py                # ANSI terminal rendering
├── requirements.txt
├── data/
│   ├── progress/        # one JSON file per user profile
│   └── cache/           # cached AI-generated questions
└── tests/
    ├── test_models.py
    ├── test_adaptive.py
    ├── test_storage.py
    ├── test_question_bank.py
    └── test_generator.py
```

Data flow for a quiz session:

```
main.py
  └─ QuizSession.run()
       ├─ adaptive.select_questions()   — weighted sampling from static bank
       ├─ ClaudeGenerator.get_question() — cache hit or API call
       ├─ ui.present_question()         — render and read answer
       ├─ UserProfile.record_answer()   — update mastery score
       └─ storage.save_profile()        — persist after every answer
```

---

## Module Reference

### `config.py` — Constants and chapter metadata

Central location for every tunable value. Edit this file to change model, cache limits, or difficulty thresholds without touching any other module.

**Key constants:**

| Constant | Default | Meaning |
|----------|---------|---------|
| `CLAUDE_MODEL` | `"claude-sonnet-4-6"` | Model used for question generation |
| `CHAPTER_EXCERPT_CHARS` | `4_000` | Characters of chapter text sent to Claude |
| `CACHE_CAP_PER_DIFFICULTY` | `20` | Max cached questions per chapter × difficulty |
| `LEARNING_RATE_CORRECT` | `0.15` | Mastery gain per correct answer |
| `LEARNING_RATE_WRONG` | `0.10` | Mastery loss per wrong answer |
| `INITIAL_MASTERY` | `0.50` | Starting mastery for a chapter you haven't seen |
| `RECENCY_WINDOW` | `60` | Recent = seen within this many answers |
| `RECENCY_PENALTY` | `0.05` | Weight reduction for recently seen questions |
| `DIFF_BEGINNER_MAX` | `0.35` | Mastery ≤ 0.35 → prefer beginner questions |
| `DIFF_INTERMEDIATE_MAX` | `0.68` | Mastery 0.35–0.68 → prefer intermediate |

`CHAPTER_META` maps chapter index (0–26) to `{phase, name, file}`.

---

### `models.py` — Data structures

**`Question`** (frozen-ish dataclass):

```python
@dataclass
class Question:
    chapter: int
    phase: int
    kind: str           # "mc" | "tf" | "blank"
    text: str
    choices: list[str]  # ["A text", "B text", "C text", "D text"] for mc
    answer: int | str   # index for mc/tf, string for blank
    explanation: str
    tags: list[str]
    difficulty: str     # "beginner" | "intermediate" | "advanced"
    generated: bool     # True if from Claude, False if static
    question_id: str    # UUID, auto-assigned if not provided
```

`q.validate()` returns a list of error strings; empty list means valid. Call before using any question.

**`MasteryRecord`** (per-chapter):

```python
@dataclass
class MasteryRecord:
    score: float        # 0.0–1.0
    total_seen: int
    total_correct: int
    last_seen: str      # ISO timestamp
```

`record.accuracy` — `total_correct / total_seen` or `0.0`.

**`UserProfile`**:

```python
@dataclass
class UserProfile:
    user_id: str
    name: str
    mastery: dict[int, MasteryRecord]  # keyed by chapter index
    seen_question_ids: list[str]        # rolling window of last 200
    session_history: list[dict]
```

`profile.record_answer(chapter, correct, question_id)` — the single method that updates everything. Mastery update rule:

```
correct:  score = min(1.0, score + 0.15 * (1 - score))
wrong:    score = max(0.0, score - 0.10 * score)
```

The multiplicative form means mastery changes faster at the extremes and slower in the middle, preventing wild swings for a single answer.

---

### `adaptive.py` — Selection engine

**Scope** — defines what portion of the question bank is eligible:

```python
Scope.all()           # every question in the bank
Scope.adaptive()      # all chapters, but weight by weakness
Scope.phase(n)        # questions from phase n only
Scope.chapter(n)      # questions from chapter n only
Scope.tag("paths")    # questions with this tag (case-insensitive)
```

**`preferred_difficulty(mastery: float) -> str`**

Maps a mastery score to a difficulty label using the configured thresholds:

```
mastery ≤ 0.35  → "beginner"
mastery ≤ 0.68  → "intermediate"
mastery > 0.68  → "advanced"
```

**`difficulty_weight(question_diff, mastery) -> float`**

Returns a weight (0.0–1.0) for how well a question's difficulty matches the user's current level:

```
matching difficulty   → 1.0
one level off         → 0.35
two levels off        → 0.05
```

**`select_questions(pool, profile, n, scope, rng) -> list[Question]`**

Weighted sampling without replacement. Each question's weight is:

```
weight = chapter_weight × difficulty_weight × recency_factor
```

Where:
- `chapter_weight` = `max(0.1, 1.2 - mastery)` in adaptive mode (weak chapters score higher), `1.0` otherwise
- `difficulty_weight` — from the function above
- `recency_factor` = `0.05` if the question was seen in the last 60 answers, `1.0` otherwise

Weights are floored at `1e-6` so every question retains a nonzero probability; this prevents the session from silently running out of candidates.

---

### `generator.py` — Claude integration

**Cache layer** (disk, per chapter × difficulty):

```
data/cache/ch18_intermediate.json   ← list of question dicts
```

The cache is checked before any API call. When a new question is generated, two additional questions are pre-generated into the cache in the same call (`_prefill_cache`), so subsequent requests are near-instant.

**`ClaudeGenerator`**:

```python
gen = ClaudeGenerator()               # reads ANTHROPIC_API_KEY from env
gen.available                         # True if key + anthropic installed
gen.get_question(chapter, difficulty, mastery_pct, weak_topics)
```

Attempt order inside `get_question`:

1. Pop one question from disk cache (instant, no API cost)
2. Call the Claude API, cache two extras pre-emptively
3. Return `None` (caller falls back to static bank)

**Prompt design**:

Claude receives:
- The target chapter number and name
- The user's current mastery percentage and difficulty label
- A comma-separated list of the user's weak topics
- Up to 4,000 characters of the actual chapter markdown (code blocks stripped)

Claude is instructed to return only raw JSON matching the `Question` schema. The validator strips markdown code fences if present, then re-validates the structure. The answer index is bounds-checked against the choices list. Any failure returns `None` — no exception is surfaced to the caller.

**Model**: `claude-sonnet-4-6` by default. Change `CLAUDE_MODEL` in `config.py`.

---

### `session.py` — Quiz loop

`QuizSession(profile, scope, n, generator, generated_ratio=0.35)`:

- `generated_ratio` — up to 35% of questions will be AI-generated if the API is available. The rest come from the static bank.
- Questions are assembled once at session start, then interleaved (generated and static alternated, not batched at the end).
- Profile is saved to disk after every answer, so progress is never lost to a crash.
- Returns a `SessionResult` with session_id, score, duration, and a list of chapters where answers were wrong.

---

### `storage.py` — Profile persistence

Profiles are stored as JSON files under `data/progress/{safe_name}.json`. The filename is derived from the profile name by lowercasing and replacing non-alphanumeric characters with underscores.

Writes are atomic: data is written to a `.tmp` file first, then renamed. This prevents a corrupt profile if the process is killed mid-write.

```python
from quiz_app.storage import load_or_create, save_profile, list_profiles

profile = load_or_create("Alice")   # loads or creates
save_profile(profile)               # atomic write
list_profiles()                     # ["Alice", "Bob"]
```

---

### `question_bank.py` — Static questions

119 questions across all 27 chapters. Factory functions for readability:

```python
mc(chapter, phase, text, choices, answer, explanation, tags, diff)
tf(chapter, phase, text, answer, explanation, tags, diff)
blank(chapter, phase, text, answer, explanation, tags, diff)
```

Distribution:

| Type | Count |
|------|-------|
| Multiple choice | 95 |
| True / false | 19 |
| Fill in blank | 5 |

| Difficulty | Count |
|-----------|-------|
| Beginner | 30 |
| Intermediate | 60 |
| Advanced | 29 |

All questions are validated on import. If any fail `q.validate()`, the module raises at import time — this catches regressions in the static bank immediately.

---

## Configuration

All tunable values live in `config.py`. The most common changes:

**Switch to a different Claude model:**
```python
CLAUDE_MODEL = "claude-opus-4-7"
```

**Increase the chapter context window** (more context → better questions, higher cost):
```python
CHAPTER_EXCERPT_CHARS = 8_000
```

**Adjust how quickly mastery changes:**
```python
LEARNING_RATE_CORRECT = 0.10   # slower gains
LEARNING_RATE_WRONG   = 0.05   # slower losses
```

**Change the difficulty thresholds:**
```python
DIFF_BEGINNER_MAX     = 0.40   # wider beginner band
DIFF_INTERMEDIATE_MAX = 0.75
```

**Increase the pre-generated question cache:**
```python
CACHE_CAP_PER_DIFFICULTY = 50
```

---

## Adding Questions to the Static Bank

Open `question_bank.py` and add a call to `mc()`, `tf()`, or `blank()` in the `build()` function. The chapter and phase must be consistent with `CHAPTER_META` in `config.py`. Run the tests after adding; `test_question_bank.py` will catch any validation failures or chapter/phase mismatches automatically.

Example:

```python
mc(
    chapter=18, phase=5,
    text="Which of the following is a consequence of the univalence axiom?",
    choices=[
        "Function extensionality",
        "The law of excluded middle",
        "Uniqueness of identity proofs",
        "Strict propositionality of all types",
    ],
    answer=0,
    explanation=(
        "Univalence implies function extensionality: if two functions agree on all "
        "inputs, the path over the equivalence gives a proof that they are equal. "
        "LEM and UIP are independent of or inconsistent with univalence."
    ),
    tags=["univalence", "funext"],
    diff="intermediate",
),
```

---

## Test Suite

| File | Tests | What it covers |
|------|-------|----------------|
| `test_models.py` | 23 | Question validation, mastery update math, serialization round-trips |
| `test_adaptive.py` | 27 | Difficulty thresholds, scope filtering, weighted selection statistics |
| `test_storage.py` | 14 | Atomic writes, corrupted file handling, profile lifecycle |
| `test_question_bank.py` | 16 | All 119 questions valid, all chapters/phases/difficulties represented |
| `test_generator.py` | 16 | JSON parsing, cache operations, mocked API calls, retry logic |

The two probabilistic tests in `test_adaptive.py` (`test_recent_questions_deprioritised` and `test_adaptive_focuses_on_weak_chapters`) use 200 trials and loose thresholds (<45% and >55%) — they will not flake under normal conditions.

---

## Data Directory

```
quiz_app/data/
├── progress/
│   ├── alice.json         # UserProfile as JSON
│   └── bob.json
└── cache/
    ├── ch18_intermediate.json    # list of cached Question dicts
    ├── ch20_advanced.json
    └── ...
```

The `data/` directory is created automatically on first run. It is safe to delete `data/cache/` at any time — questions will be re-generated on demand. Deleting `data/progress/` removes all user profiles.

---

## Dependencies

| Package | Required | Purpose |
|---------|----------|---------|
| Python 3.10+ | Yes | dataclasses with `dict | None` union syntax |
| `anthropic` | No | Claude API client; all other features work without it |
| `pytest` | Dev only | Running the test suite |
