"""
Adaptive question selection engine.

Selects questions based on:
  1. Chapter mastery: weakest chapters get higher weight.
  2. Difficulty match: difficulty level matched to mastery score.
  3. Freshness: recently seen questions are deprioritised.
  4. Phase gating: chapters far ahead of the user's frontier get lower weight.
  5. Spaced repetition: chapters due for review get a strong boost.
  6. Per-question SR: individual questions due for review get boosted.
  7. Scope: the user's chosen chapter/phase/tag filter.
"""
from __future__ import annotations

import random
from dataclasses import dataclass
from typing import Optional

from .config import (
    BLANK_WEIGHT_BONUS,
    DIFF_BEGINNER_MAX,
    DIFF_INTERMEDIATE_MAX,
    INITIAL_MASTERY,
    RECENCY_PENALTY,
    RECENCY_WINDOW,
    CHAPTER_META,
    PHASE_NAMES,
)
from .models import Question, UserProfile


# ── Scope ─────────────────────────────────────────────────────────────────────

@dataclass
class Scope:
    """Describes which portion of the question bank to quiz on."""
    kind:  str   # "all" | "phase" | "chapter" | "tag" | "adaptive" | "due" | "review"
    value: Optional[int | str | list] = None

    @classmethod
    def all(cls)                              -> "Scope": return cls("all")
    @classmethod
    def phase(cls, ph: int)                   -> "Scope": return cls("phase", ph)
    @classmethod
    def chapter(cls, ch: int)                 -> "Scope": return cls("chapter", ch)
    @classmethod
    def tag(cls, t: str)                      -> "Scope": return cls("tag", t)
    @classmethod
    def adaptive(cls)                         -> "Scope": return cls("adaptive")
    @classmethod
    def due(cls)                              -> "Scope": return cls("due")
    @classmethod
    def review(cls, question_ids: list[str])  -> "Scope": return cls("review", question_ids)

    @property
    def label(self) -> str:
        if self.kind == "adaptive":
            return "Adaptive (focus on weaknesses)"
        if self.kind == "all":
            return "All chapters"
        if self.kind == "phase":
            name = PHASE_NAMES.get(self.value, f"Phase {self.value}")
            return name
        if self.kind == "chapter":
            name = CHAPTER_META.get(self.value, {}).get("name", f"Ch.{self.value}")
            return f"Ch.{self.value} — {name}"
        if self.kind == "tag":
            return f"Tag: {self.value}"
        if self.kind == "due":
            return "Due for review"
        if self.kind == "review":
            n = len(self.value) if isinstance(self.value, list) else 0
            return f"Re-quiz wrong answers ({n})"
        return self.kind


# ── Difficulty helpers ────────────────────────────────────────────────────────

def preferred_difficulty(mastery: float) -> str:
    """Map mastery score to preferred difficulty level."""
    if mastery < DIFF_BEGINNER_MAX:
        return "beginner"
    if mastery < DIFF_INTERMEDIATE_MAX:
        return "intermediate"
    return "advanced"


def difficulty_weight(question_diff: str, mastery: float) -> float:
    """
    Weight for a question given the user's current mastery.
    The preferred difficulty gets weight 1.0; others get partial weight.
    """
    preferred = preferred_difficulty(mastery)
    if question_diff == preferred:
        return 1.0
    diffs = ["beginner", "intermediate", "advanced"]
    qi = diffs.index(question_diff)
    pi = diffs.index(preferred)
    distance = abs(qi - pi)
    return 0.35 if distance == 1 else 0.05


# ── Phase gating ──────────────────────────────────────────────────────────────

def _user_phase_frontier(profile: UserProfile) -> int:
    """
    Return the highest phase the user has meaningfully engaged with
    (seen at least 3 questions in any chapter of that phase).
    """
    phases = [
        CHAPTER_META[ch]["phase"]
        for ch, rec in profile.mastery.items()
        if ch in CHAPTER_META and rec.total_seen >= 3
    ]
    return max(phases, default=0)


def _phase_gate_factor(chapter: int, frontier: int) -> float:
    """
    Reduce weight for chapters far ahead of the user's current frontier.
    Chapters in the current or next phase are unpenalised.
    """
    ch_phase = CHAPTER_META.get(chapter, {}).get("phase", 0)
    gap = ch_phase - frontier
    if gap <= 1:  return 1.0   # at or one phase ahead: no penalty
    if gap == 2:  return 0.5
    if gap == 3:  return 0.2
    return 0.05                # 4+ phases ahead: strongly deprioritise


# ── Core selector ─────────────────────────────────────────────────────────────

def _chapter_mastery(profile: UserProfile, chapter: int) -> float:
    rec = profile.mastery.get(chapter)
    return rec.score if rec else INITIAL_MASTERY


def _chapter_weight(profile: UserProfile, chapter: int, adaptive: bool,
                    frontier: int = 0) -> float:
    """Weight for selecting a question from a given chapter."""
    if not adaptive:
        return 1.0

    # Base weight from mastery: unseen chapters get a moderate exploration boost
    if chapter not in profile.mastery:
        base = 0.8
    else:
        mastery = _chapter_mastery(profile, chapter)
        base = max(0.1, 1.2 - mastery)

    # Review boost: chapters due for spaced repetition override phase gating
    rec = profile.mastery.get(chapter)
    if rec and rec.is_due:
        return base * 2.0

    # Phase gating: suppress chapters the user hasn't reached yet
    return base * _phase_gate_factor(chapter, frontier)


def _recency_factor(question_id: str, profile: UserProfile) -> float:
    recent = profile.seen_question_ids[-RECENCY_WINDOW:]
    return RECENCY_PENALTY if question_id in recent else 1.0


def _question_sr_factor(question_id: str, profile: UserProfile) -> float:
    """
    Boost individual questions that are due for spaced repetition review.
    Slightly suppress questions that have been reviewed but aren't due yet.
    Has no effect on questions never individually tracked.
    """
    rec = profile.question_mastery.get(question_id)
    if rec is None:
        return 1.0
    if rec.is_due:
        return 3.0   # strongly resurface due questions
    if rec.next_review:
        return 0.7   # gently suppress recently-reviewed, not-yet-due questions
    return 1.0


def filter_by_scope(
    questions: list[Question],
    scope: Scope,
    profile: Optional["UserProfile"] = None,
) -> list[Question]:
    if scope.kind in ("all", "adaptive"):
        return questions
    if scope.kind == "phase":
        return [q for q in questions if q.phase == scope.value]
    if scope.kind == "chapter":
        return [q for q in questions if q.chapter == scope.value]
    if scope.kind == "tag":
        tag = (scope.value or "").lower()
        return [q for q in questions if any(tag in t.lower() for t in q.tags)]
    if scope.kind == "due":
        if profile is None:
            return questions
        due_chs = set(profile.chapters_due_for_review())
        return [q for q in questions if q.chapter in due_chs]
    if scope.kind == "review":
        ids = set(scope.value) if isinstance(scope.value, list) else set()
        return [q for q in questions if q.question_id in ids]
    return questions


def select_questions(
    pool: list[Question],
    profile: UserProfile,
    n: int,
    scope: Scope,
    rng: Optional[random.Random] = None,
) -> list[Question]:
    """
    Return n questions sampled from pool according to the adaptive policy.
    Guarantees no duplicates within the returned list.
    """
    rng = rng or random.Random()

    candidates = filter_by_scope(pool, scope, profile)
    if not candidates:
        return []

    adaptive = (scope.kind == "adaptive")
    frontier = _user_phase_frontier(profile) if adaptive else 0

    weights: list[float] = []
    for q in candidates:
        ch_mastery  = _chapter_mastery(profile, q.chapter)
        blank_bonus = BLANK_WEIGHT_BONUS if q.kind == "blank" else 1.0
        w = (
            _chapter_weight(profile, q.chapter, adaptive, frontier)
            * difficulty_weight(q.difficulty, ch_mastery)
            * _recency_factor(q.question_id, profile)
            * _question_sr_factor(q.question_id, profile)
            * blank_bonus
        )
        weights.append(max(1e-6, w))  # never exactly zero

    # Weighted sampling without replacement
    result: list[Question] = []
    remaining_candidates = list(candidates)
    remaining_weights    = list(weights)

    n = min(n, len(remaining_candidates))
    for _ in range(n):
        total = sum(remaining_weights)
        r = rng.uniform(0, total)
        cumul = 0.0
        # Default to last index as fallback for floating-point edge cases
        chosen_idx = len(remaining_weights) - 1
        for i, w in enumerate(remaining_weights):
            cumul += w
            if r <= cumul:
                chosen_idx = i
                break
        result.append(remaining_candidates.pop(chosen_idx))
        remaining_weights.pop(chosen_idx)

    return result


# ── Summary helpers ───────────────────────────────────────────────────────────

def mastery_summary(profile: UserProfile) -> list[dict]:
    """
    Return a list of dicts describing mastery per chapter, sorted by chapter index.
    Includes chapters with no data (mastery = 0.5, unstarted).
    """
    rows = []
    for ch, meta in CHAPTER_META.items():
        rec = profile.mastery.get(ch)
        rows.append({
            "chapter":     ch,
            "phase":       meta["phase"],
            "name":        meta["name"],
            "score":       rec.score         if rec else INITIAL_MASTERY,
            "seen":        rec.total_seen    if rec else 0,
            "correct":     rec.total_correct if rec else 0,
            "started":     rec is not None,
            "due":         rec.is_due        if rec else False,
            "next_review": rec.next_review   if rec else "",
        })
    return rows


def weak_topics(profile: UserProfile, n: int = 5) -> list[str]:
    """Return the names of the n weakest chapters (by mastery score)."""
    rows = [r for r in mastery_summary(profile) if r["started"]]
    rows.sort(key=lambda r: r["score"])
    return [r["name"] for r in rows[:n]]


def tag_weakness(
    profile: UserProfile,
    questions: list[Question],
    top_n: int = 8,
) -> list[dict]:
    """
    Return the top_n weakest tags by approximate mastery.
    Mastery for a tag is the average mastery of started chapters that contain
    at least one question with that tag.
    """
    tag_chapters: dict[str, set[int]] = {}
    for q in questions:
        for t in q.tags:
            tag_chapters.setdefault(t, set()).add(q.chapter)

    rows = []
    for tag, chapters in tag_chapters.items():
        started = [ch for ch in chapters if ch in profile.mastery]
        if not started:
            continue
        avg_mastery = sum(profile.mastery[ch].score for ch in started) / len(started)
        rows.append({"tag": tag, "mastery": avg_mastery, "chapters": len(chapters)})

    rows.sort(key=lambda r: r["mastery"])
    return rows[:top_n]
