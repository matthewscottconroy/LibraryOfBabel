"""
Shared question-generation engine.

Subclass BaseGenerator and set _SYSTEM_PROMPT to add app-specific context.
Improvements over the original:
  - _prefill_cache runs in a daemon thread (no latency on generation)
  - Generates MC, TF, and fill-in-the-blank questions
  - Question kind chosen probabilistically by difficulty
"""
from __future__ import annotations

import json
import random
import re
import threading
import time
import uuid
from datetime import datetime, timedelta
from pathlib import Path
from typing import Optional

try:
    import anthropic as _anthropic_module
except ImportError:
    _anthropic_module = None  # type: ignore

from .config import (
    ANTHROPIC_API_KEY,
    CACHE_CAP_PER_DIFFICULTY,
    CACHE_DIR,
    CACHE_MAX_AGE_DAYS,
    CHAPTER_EXCERPT_CHARS,
    CHAPTERS_DIR,
    CLAUDE_MODEL,
    CHAPTER_META,
    DIFFICULTY_LEVELS,
)
from .models import Question


# ── Kind selection ────────────────────────────────────────────────────────────

# Probability of generating each kind by difficulty level
# "proof" questions test argument construction — reserved for intermediate/advanced
_KIND_WEIGHTS: dict[str, dict[str, float]] = {
    "beginner":     {"mc": 0.55, "tf": 0.45, "blank": 0.00, "proof": 0.00},
    "intermediate": {"mc": 0.50, "tf": 0.10, "blank": 0.20, "proof": 0.20},
    "advanced":     {"mc": 0.30, "tf": 0.00, "blank": 0.35, "proof": 0.35},
}


def _pick_kind(difficulty: str) -> str:
    weights = _KIND_WEIGHTS.get(difficulty, {"mc": 1.0, "tf": 0.0, "blank": 0.0})
    kinds = list(weights.keys())
    probs = [weights[k] for k in kinds]
    return random.choices(kinds, weights=probs, k=1)[0]


# ── Cache helpers ─────────────────────────────────────────────────────────────

def _cache_path(chapter: int, difficulty: str) -> Path:
    return CACHE_DIR / f"ch{chapter:02d}_{difficulty}.json"


def _load_cache(chapter: int, difficulty: str) -> list[dict]:
    p = _cache_path(chapter, difficulty)
    if not p.exists():
        return []
    try:
        return json.loads(p.read_text())
    except Exception:
        return []


def _save_cache(chapter: int, difficulty: str, items: list[dict]) -> None:
    _cache_path(chapter, difficulty).write_text(json.dumps(items, indent=2))


def _is_stale(item: dict) -> bool:
    """Return True if the cached question is older than CACHE_MAX_AGE_DAYS."""
    ts = item.get("_cached_at")
    if not ts:
        return False
    try:
        cached = datetime.fromisoformat(ts)
        return datetime.now() - cached > timedelta(days=CACHE_MAX_AGE_DAYS)
    except (ValueError, TypeError):
        return False


def _add_to_cache(chapter: int, difficulty: str, item: dict) -> None:
    items = _load_cache(chapter, difficulty)
    # Purge stale entries while we have the file open
    items = [i for i in items if not _is_stale(i)]
    if len(items) < CACHE_CAP_PER_DIFFICULTY:
        item = dict(item)
        item["_cached_at"] = datetime.now().isoformat()
        items.append(item)
        _save_cache(chapter, difficulty, items)


def _pop_from_cache(chapter: int, difficulty: str) -> Optional[dict]:
    items = _load_cache(chapter, difficulty)
    if not items:
        return None
    # Skip stale entries
    while items and _is_stale(items[0]):
        items.pop(0)
    if not items:
        _save_cache(chapter, difficulty, [])
        return None
    item = items.pop(0)
    _save_cache(chapter, difficulty, items)
    return item


# ── Chapter content ───────────────────────────────────────────────────────────

def _load_chapter_excerpt(chapter: int) -> str:
    meta = CHAPTER_META.get(chapter)
    if not meta:
        return ""

    # Prefer the richer subdirectory section files over the flat summary file.
    # The directory name matches the file name without its extension.
    flat_file = meta["file"]
    subdir = CHAPTERS_DIR / flat_file.replace(".md", "")
    if subdir.is_dir():
        section_files = sorted(subdir.rglob("*.md"))
        if section_files:
            parts = []
            budget = CHAPTER_EXCERPT_CHARS
            for sf in section_files:
                chunk = sf.read_text(errors="replace")
                chunk = re.sub(r"```.*?```", "[code block omitted]", chunk, flags=re.DOTALL)
                parts.append(chunk[:budget])
                budget -= len(chunk)
                if budget <= 0:
                    break
            text = "\n\n".join(parts)
            return text[:CHAPTER_EXCERPT_CHARS]

    path = CHAPTERS_DIR / flat_file
    if not path.exists():
        return f"Chapter: {meta['name']}"
    text = path.read_text(errors="replace")
    text = re.sub(r"```.*?```", "[code block omitted]", text, flags=re.DOTALL)
    return text[:CHAPTER_EXCERPT_CHARS]


# ── Prompt builders ───────────────────────────────────────────────────────────

_MC_SCHEMA = """\
Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "text": "question text here (no leading letter/number)",
  "choices": ["choice A", "choice B", "choice C", "choice D"],
  "answer": 0,
  "explanation": "explanation here (≥2 sentences explaining WHY the answer is correct)",
  "tags": ["tag1", "tag2"]
}"""

_TF_SCHEMA = """\
Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "text": "a clear statement that is definitively true or false",
  "answer": true,
  "explanation": "explanation here (≥2 sentences explaining WHY the statement is true or false)",
  "tags": ["tag1", "tag2"]
}"""

_BLANK_SCHEMA = """\
Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "text": "sentence with ___ where the key term belongs",
  "acceptable": ["primary answer", "accepted synonym"],
  "explanation": "explanation here (≥2 sentences explaining the term)",
  "tags": ["tag1", "tag2"]
}"""

_PROOF_SCHEMA = """\
Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "theorem": "the proposition to be proved (one sentence)",
  "proof_lines": [
    "Step 1: prose line, possibly containing ___ for a key term or short phrase.",
    "Step 2: another line, possibly with ___.",
    "Step 3: another line."
  ],
  "fills": ["fill for blank 1", "fill for blank 2"],
  "explanation": "the complete proof written out in full (≥3 sentences)",
  "tags": ["tag1", "tag2"]
}
Notes:
- Use ___ (three underscores) for exactly 2–3 blanks spread across the proof_lines.
- Each blank should be a short key phrase (a definition term, a rule name, a brief computation step).
- fills must list the canonical answer for each blank in order of appearance.
- The proof should be logically complete; the blanked steps are the pivotal ones."""

_KIND_REQUIREMENTS = {
    "mc": (
        "- Exactly 4 answer choices (A through D).\n"
        "- Exactly one correct answer.\n"
        "- Distractors must represent common misconceptions, not obvious wrong answers.\n"
        + _MC_SCHEMA
    ),
    "tf": (
        "- The statement must be clearly true or false with no ambiguity.\n"
        "- Avoid trivially obvious statements; test conceptual understanding.\n"
        + _TF_SCHEMA
    ),
    "blank": (
        "- Use ___ (three underscores) for exactly one blank.\n"
        "- The blank should be a single key term, symbol, or short phrase.\n"
        "- List 1–3 acceptable answers (primary term + common synonyms/abbreviations).\n"
        + _BLANK_SCHEMA
    ),
    "proof": (
        "- Present a partial proof with 2–3 key steps blanked out.\n"
        "- The blanked steps must be the pivotal logical moves, not trivial transitions.\n"
        "- The student fills in each blank in order.\n"
        + _PROOF_SCHEMA
    ),
}

_DIFF_GUIDANCE = (
    "- beginner: recall of key definitions or straightforward application\n"
    "- intermediate: deeper understanding, knowing *why*, cross-connections\n"
    "- advanced: subtle distinctions, open problems, or research-level nuance"
)


def _build_prompt(
    chapter: int,
    chapter_name: str,
    kind: str,
    difficulty: str,
    mastery_pct: int,
    weak_topics: list[str],
    chapter_excerpt: str,
) -> str:
    kind_label = {
        "mc": "multiple-choice", "tf": "true/false",
        "blank": "fill-in-the-blank", "proof": "proof-scaffold",
    }[kind]
    weak_str = ", ".join(weak_topics) if weak_topics else "none identified yet"
    requirements = _KIND_REQUIREMENTS[kind]
    return (
        f"Generate one {difficulty} {kind_label} question for "
        f"Chapter {chapter}: {chapter_name}.\n\n"
        f"Student mastery level: {mastery_pct}% ({difficulty}).\n"
        f"Recent weak areas: {weak_str}.\n\n"
        f"Use this chapter excerpt as context (do not quote it verbatim):\n"
        f"---\n{chapter_excerpt}\n---\n\n"
        f"Requirements:\n"
        f'{requirements}\n\n'
        f'Difficulty "{difficulty}" means:\n{_DIFF_GUIDANCE}\n'
    )


# ── Response parsers ──────────────────────────────────────────────────────────

def _strip_fences(raw: str) -> str:
    raw = raw.strip()
    raw = re.sub(r"^```(?:json)?\s*", "", raw)
    raw = re.sub(r"\s*```$", "", raw)
    return raw


def _parse_mc(data: dict, chapter: int, difficulty: str) -> Optional[Question]:
    q = Question(
        chapter=chapter,
        phase=CHAPTER_META[chapter]["phase"],
        kind="mc",
        text=str(data["text"]).strip(),
        choices=[str(c) for c in data["choices"]],
        answer=int(data["answer"]),
        explanation=str(data["explanation"]).strip(),
        tags=[str(t) for t in data.get("tags", [])],
        difficulty=difficulty,
        generated=True,
        question_id=str(uuid.uuid4()),
    )
    return q if not q.validate() else None


def _parse_tf(data: dict, chapter: int, difficulty: str) -> Optional[Question]:
    raw_answer = data["answer"]
    if isinstance(raw_answer, bool):
        answer_idx = 0 if raw_answer else 1
    else:
        answer_idx = 0 if str(raw_answer).lower() in ("true", "1", "yes") else 1
    q = Question(
        chapter=chapter,
        phase=CHAPTER_META[chapter]["phase"],
        kind="tf",
        text=str(data["text"]).strip(),
        choices=["True", "False"],
        answer=answer_idx,
        explanation=str(data["explanation"]).strip(),
        tags=[str(t) for t in data.get("tags", [])],
        difficulty=difficulty,
        generated=True,
        question_id=str(uuid.uuid4()),
    )
    return q if not q.validate() else None


def _parse_blank(data: dict, chapter: int, difficulty: str) -> Optional[Question]:
    synonyms = [str(s) for s in data["acceptable"]]
    if not synonyms:
        return None
    q = Question(
        chapter=chapter,
        phase=CHAPTER_META[chapter]["phase"],
        kind="blank",
        text=str(data["text"]).strip(),
        choices=synonyms,
        answer=synonyms[0].lower(),
        explanation=str(data["explanation"]).strip(),
        tags=[str(t) for t in data.get("tags", [])],
        difficulty=difficulty,
        generated=True,
        question_id=str(uuid.uuid4()),
    )
    return q if not q.validate() else None


def _parse_proof(data: dict, chapter: int, difficulty: str) -> Optional[Question]:
    theorem    = str(data.get("theorem", "")).strip()
    lines      = [str(l) for l in data.get("proof_lines", [])]
    fills      = [str(f).strip() for f in data.get("fills", [])]
    explanation = str(data.get("explanation", "")).strip()
    if not theorem or not lines or not fills:
        return None
    # answer = pipe-joined fills; choices = proof lines
    q = Question(
        chapter     = chapter,
        phase       = CHAPTER_META[chapter]["phase"],
        kind        = "proof",
        text        = theorem,
        choices     = lines,
        answer      = "|".join(fills),
        explanation = explanation,
        tags        = [str(t) for t in data.get("tags", [])],
        difficulty  = difficulty,
        generated   = True,
        question_id = str(uuid.uuid4()),
    )
    return q if not q.validate() else None


_PARSERS = {"mc": _parse_mc, "tf": _parse_tf, "blank": _parse_blank, "proof": _parse_proof}


def _parse_response(raw: str, kind: str, chapter: int, difficulty: str) -> Optional[Question]:
    raw = _strip_fences(raw)
    try:
        data = json.loads(raw)
    except json.JSONDecodeError:
        return None
    try:
        return _PARSERS[kind](data, chapter, difficulty)
    except (KeyError, TypeError, ValueError):
        return None


# ── Backward-compat aliases (used by existing tests) ─────────────────────────

def _parse_and_validate(raw: str, chapter: int, difficulty: str) -> Optional[Question]:
    """MC-only parse alias kept for backward compatibility."""
    return _parse_response(raw, "mc", chapter, difficulty)


def _build_user_prompt(
    chapter: int,
    chapter_name: str,
    difficulty: str,
    mastery_pct: int,
    weak_topics: list[str],
    chapter_excerpt: str,
) -> str:
    """MC-only prompt alias kept for backward compatibility."""
    return _build_prompt(chapter, chapter_name, "mc", difficulty,
                         mastery_pct, weak_topics, chapter_excerpt)


# ── Base generator class ──────────────────────────────────────────────────────

class BaseGenerator:
    """
    Generates quiz questions via the Anthropic API.
    Subclass and set _SYSTEM_PROMPT for app-specific context.
    Falls back gracefully when the API key is absent or calls fail.
    """

    _SYSTEM_PROMPT: str = ""

    def __init__(self, api_key: str = ANTHROPIC_API_KEY):
        self._api_key = api_key
        self._client  = None
        if api_key and _anthropic_module is not None:
            self._client = _anthropic_module.Anthropic(api_key=api_key)

    @property
    def available(self) -> bool:
        return self._client is not None

    def get_question(
        self,
        chapter: int,
        difficulty: str,
        mastery_pct: int,
        weak_topics: list[str],
    ) -> Optional[Question]:
        """
        Return a Question for the given chapter and difficulty.
        Order: (1) pop from cache, (2) generate live, (3) return None.
        """
        cached = _pop_from_cache(chapter, difficulty)
        if cached:
            q = Question.from_dict(cached)
            q.question_id = str(uuid.uuid4())
            return q

        if not self.available:
            return None

        return self._generate(chapter, difficulty, mastery_pct, weak_topics)

    def _generate(
        self,
        chapter: int,
        difficulty: str,
        mastery_pct: int,
        weak_topics: list[str],
        retries: int = 2,
    ) -> Optional[Question]:
        meta    = CHAPTER_META.get(chapter, {})
        name    = meta.get("name", f"Chapter {chapter}")
        excerpt = _load_chapter_excerpt(chapter)
        kind    = _pick_kind(difficulty)
        prompt  = _build_prompt(chapter, name, kind, difficulty,
                                mastery_pct, weak_topics, excerpt)

        for attempt in range(retries):
            try:
                msg = self._client.messages.create(  # type: ignore[union-attr]
                    model=CLAUDE_MODEL,
                    max_tokens=700,
                    system=self._SYSTEM_PROMPT,
                    messages=[{"role": "user", "content": prompt}],
                )
                raw = msg.content[0].text
                q   = _parse_response(raw, kind, chapter, difficulty)
                if q:
                    # Pre-warm cache in background — does not block caller
                    threading.Thread(
                        target=self._prefill_cache,
                        args=(chapter, difficulty, mastery_pct, weak_topics),
                        kwargs={"n": 2},
                        daemon=True,
                    ).start()
                    return q
            except Exception:
                if attempt < retries - 1:
                    time.sleep(1)

        return None

    def _prefill_cache(
        self,
        chapter: int,
        difficulty: str,
        mastery_pct: int,
        weak_topics: list[str],
        n: int = 2,
    ) -> None:
        existing  = _load_cache(chapter, difficulty)
        remaining = CACHE_CAP_PER_DIFFICULTY - len(existing)
        if remaining <= 0:
            return
        for _ in range(min(n, remaining)):
            q = self._generate(chapter, difficulty, mastery_pct,
                               weak_topics, retries=1)
            if q:
                _add_to_cache(chapter, difficulty, q.to_dict())

    def prefill_for_chapter(
        self,
        chapter: int,
        mastery_pct: int,
        weak_topics: list[str],
    ) -> None:
        """Pre-generate questions across all three difficulties for a chapter."""
        if not self.available:
            return
        for diff in DIFFICULTY_LEVELS:
            existing = _load_cache(chapter, diff)
            needed   = max(0, 5 - len(existing))
            for _ in range(needed):
                q = self._generate(chapter, diff, mastery_pct,
                                   weak_topics, retries=1)
                if q:
                    _add_to_cache(chapter, diff, q.to_dict())
                time.sleep(0.2)
