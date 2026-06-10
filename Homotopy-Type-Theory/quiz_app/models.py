"""
Core data models: Question, MasteryRecord, UserProfile, SessionResult.
All models support dict serialisation for JSON persistence.
"""
from __future__ import annotations

import uuid
from dataclasses import dataclass, field
from datetime import date, datetime, timedelta, timezone
from typing import Any

from .config import INITIAL_MASTERY, RECENCY_WINDOW

# ── Question ──────────────────────────────────────────────────────────────────

QuestionKind = str  # "mc" | "tf" | "blank" | "proof"
Difficulty   = str  # "beginner" | "intermediate" | "advanced"


@dataclass
class Question:
    """A single quiz question (static or Claude-generated)."""

    chapter:     int
    phase:       int
    kind:        QuestionKind
    text:        str
    choices:     list[str]
    answer:      int | str        # index (mc/tf) or canonical string (blank)
    explanation: str
    tags:        list[str]
    difficulty:  Difficulty = "intermediate"
    generated:   bool       = False  # True → came from Claude
    question_id: str        = field(default_factory=lambda: str(uuid.uuid4()))

    # ── validation ────────────────────────────────────────────────────────────

    def validate(self) -> list[str]:
        """Return a list of validation errors (empty = valid)."""
        errs: list[str] = []
        if not self.text.strip():
            errs.append("text is empty")
        if self.kind in ("mc", "tf"):
            if not isinstance(self.answer, int):
                errs.append("answer must be an int index for mc/tf")
            elif not (0 <= self.answer < len(self.choices)):
                errs.append(f"answer index {self.answer} out of range for {len(self.choices)} choices")
        if self.kind == "tf" and len(self.choices) != 2:
            errs.append("tf questions must have exactly 2 choices")
        if self.kind == "mc" and not (2 <= len(self.choices) <= 4):
            errs.append("mc questions must have 2-4 choices")
        if self.kind == "blank" and not self.choices:
            errs.append("blank questions need at least one acceptable answer in choices")
        if self.kind == "blank" and not isinstance(self.answer, str):
            errs.append("blank answer must be a string")
        if self.kind == "proof":
            # choices = list of proof lines (some containing "___")
            # answer  = pipe-separated canonical fills, one per blank across all lines
            if not self.choices:
                errs.append("proof questions need at least one proof line in choices")
            if not isinstance(self.answer, str):
                errs.append("proof answer must be a pipe-separated string of fills")
            else:
                n_blanks = sum(line.count("___") for line in self.choices)
                n_fills  = len([f for f in self.answer.split("|") if f.strip()])
                if n_blanks == 0:
                    errs.append("proof choices must contain at least one '___' blank")
                elif n_fills != n_blanks:
                    errs.append(
                        f"proof answer has {n_fills} fills but choices have {n_blanks} blanks"
                    )
        if not self.explanation.strip():
            errs.append("explanation is empty")
        if self.difficulty not in ("beginner", "intermediate", "advanced"):
            errs.append(f"unknown difficulty: {self.difficulty!r}")
        return errs

    # ── serialisation ─────────────────────────────────────────────────────────

    def to_dict(self) -> dict[str, Any]:
        return {
            "question_id": self.question_id,
            "chapter":     self.chapter,
            "phase":       self.phase,
            "kind":        self.kind,
            "text":        self.text,
            "choices":     self.choices,
            "answer":      self.answer,
            "explanation": self.explanation,
            "tags":        self.tags,
            "difficulty":  self.difficulty,
            "generated":   self.generated,
        }

    @classmethod
    def from_dict(cls, d: dict[str, Any]) -> "Question":
        return cls(
            question_id = d.get("question_id", str(uuid.uuid4())),
            chapter     = d["chapter"],
            phase       = d["phase"],
            kind        = d["kind"],
            text        = d["text"],
            choices     = d["choices"],
            answer      = d["answer"],
            explanation = d["explanation"],
            tags        = d.get("tags", []),
            difficulty  = d.get("difficulty", "intermediate"),
            generated   = d.get("generated", False),
        )


# ── Mastery ───────────────────────────────────────────────────────────────────

def confidence_to_quality(correct: bool, confidence: int) -> int:
    """
    Map (correct, confidence) to SM-2 quality score q ∈ {0..5}.

    SM-2 quality scale:
      5 — perfect recall, no hesitation
      4 — correct with slight hesitation
      3 — correct but required significant effort
      2 — incorrect; correct answer felt easy after seeing it
      1 — incorrect; recalled correct answer with difficulty
      0 — complete blackout

    Our confidence levels (only asked on correct answers):
      3 — certain → q=5
      2 — unsure  → q=4
      1 — guess   → q=3
    Wrong answers default to q=1.
    """
    if not correct:
        return 1
    return {3: 5, 2: 4, 1: 3}.get(confidence, 4)


@dataclass
class MasteryRecord:
    """
    Per-chapter (or per-question) mastery tracking using the SM-2 algorithm.

    SM-2 interval schedule:
      I(1) = 1 day,  I(2) = 6 days,  I(n) = round(I(n-1) * EF) for n > 2
    Easiness factor (EF) update after response of quality q:
      EF' = EF + 0.1 - (5-q)*(0.08 + (5-q)*0.02),  clamped to [1.3, 3.0]
    If q < 3 the interval resets to 1 and rep_count resets to 0.
    """
    score:           float = field(default_factory=lambda: INITIAL_MASTERY)
    total_seen:      int   = 0
    total_correct:   int   = 0
    last_seen:       str   = ""
    next_review:     str   = ""   # ISO date (YYYY-MM-DD); "" = not yet scheduled
    interval_days:   int   = 1    # current interval in days
    easiness_factor: float = 2.5  # SM-2 EF, clamped to [1.3, 3.0]
    rep_count:       int   = 0    # consecutive successful recalls (resets on q < 3)

    @property
    def accuracy(self) -> float:
        if self.total_seen == 0:
            return 0.0
        return self.total_correct / self.total_seen

    @property
    def is_due(self) -> bool:
        """True when today is on or past the scheduled review date."""
        if not self.next_review:
            return False
        return date.today().isoformat() >= self.next_review

    def schedule_review(self, quality: int) -> None:
        """
        Advance the SM-2 schedule given a response quality q ∈ {0..5}.
        Updates interval_days, easiness_factor, rep_count, and next_review.
        """
        ef = self.easiness_factor + 0.1 - (5 - quality) * (0.08 + (5 - quality) * 0.02)
        self.easiness_factor = max(1.3, min(3.0, ef))

        if quality < 3:
            self.interval_days = 1
            self.rep_count = 0
        else:
            if self.rep_count == 0:
                self.interval_days = 1
            elif self.rep_count == 1:
                self.interval_days = 6
            else:
                self.interval_days = max(1, round(self.interval_days * self.easiness_factor))
            self.interval_days = min(self.interval_days, 365)
            self.rep_count += 1

        self.next_review = (date.today() + timedelta(days=self.interval_days)).isoformat()

    def to_dict(self) -> dict[str, Any]:
        return {
            "score":           self.score,
            "total_seen":      self.total_seen,
            "total_correct":   self.total_correct,
            "last_seen":       self.last_seen,
            "next_review":     self.next_review,
            "interval_days":   self.interval_days,
            "easiness_factor": self.easiness_factor,
            "rep_count":       self.rep_count,
        }

    @classmethod
    def from_dict(cls, d: dict[str, Any]) -> "MasteryRecord":
        return cls(
            score           = d.get("score", INITIAL_MASTERY),
            total_seen      = d.get("total_seen", 0),
            total_correct   = d.get("total_correct", 0),
            last_seen       = d.get("last_seen", ""),
            next_review     = d.get("next_review", ""),
            interval_days   = d.get("interval_days", 1),
            easiness_factor = d.get("easiness_factor", 2.5),
            rep_count       = d.get("rep_count", 0),
        )


# ── UserProfile ───────────────────────────────────────────────────────────────

@dataclass
class UserProfile:
    """Persistent user record: mastery + session history."""

    user_id:           str
    name:              str
    mastery:           dict[int, MasteryRecord]  = field(default_factory=dict)
    seen_question_ids: list[str]                 = field(default_factory=list)
    session_history:   list[dict[str, Any]]      = field(default_factory=list)
    review_queue:      list[str]                 = field(default_factory=list)
    question_mastery:  dict[str, MasteryRecord]  = field(default_factory=dict)
    generated_strikes: dict[str, int]            = field(default_factory=dict)
    onboarded:         bool                      = False
    created_at:        str                       = field(default_factory=lambda: _now())
    last_seen:         str                       = field(default_factory=lambda: _now())

    # ── helpers ───────────────────────────────────────────────────────────────

    def get_mastery(self, chapter: int) -> MasteryRecord:
        if chapter not in self.mastery:
            self.mastery[chapter] = MasteryRecord()
        return self.mastery[chapter]

    def record_answer(self, chapter: int, correct: bool,
                      question_id: str,
                      lr_correct: float = 0.15,
                      lr_wrong: float   = 0.10,
                      confidence: int   = 2) -> float:
        """Update chapter mastery; also schedule per-question SM-2 SR. Returns new chapter score."""
        quality = confidence_to_quality(correct, confidence)

        rec = self.get_mastery(chapter)
        stability = min(rec.total_seen / 20.0, 1.0)
        eff_correct = lr_correct * (1.0 - 0.5 * stability)
        eff_wrong   = lr_wrong   * (1.0 - 0.5 * stability)
        rec.total_seen += 1
        if correct:
            rec.total_correct += 1
            rec.score = min(1.0, rec.score + eff_correct * (1.0 - rec.score))
        else:
            rec.score = max(0.0, rec.score - eff_wrong * rec.score)
        rec.last_seen = _now()
        rec.schedule_review(quality)

        # Per-question SR tracking (static questions only; generated ones get fresh IDs)
        q_rec = self.question_mastery.setdefault(question_id, MasteryRecord())
        q_rec.total_seen += 1
        if correct:
            q_rec.total_correct += 1
        q_rec.schedule_review(quality)

        self.last_seen = _now()
        self.seen_question_ids.append(question_id)
        if len(self.seen_question_ids) > RECENCY_WINDOW:
            self.seen_question_ids = self.seen_question_ids[-RECENCY_WINDOW:]
        return rec.score

    def add_to_review_queue(self, question_id: str) -> None:
        if question_id not in self.review_queue:
            self.review_queue.append(question_id)

    def remove_from_review_queue(self, question_id: str) -> None:
        try:
            self.review_queue.remove(question_id)
        except ValueError:
            pass

    def chapters_due_for_review(self) -> list[int]:
        """Return chapters whose next_review date is today or earlier."""
        return [ch for ch, rec in self.mastery.items() if rec.is_due]

    def weakest_chapters(self, top_n: int = 5) -> list[int]:
        """Return chapter indices sorted by ascending mastery (weakest first)."""
        return sorted(self.mastery, key=lambda c: self.mastery[c].score)[:top_n]

    def total_answered(self) -> int:
        return sum(r.total_seen for r in self.mastery.values())

    # ── serialisation ─────────────────────────────────────────────────────────

    def to_dict(self) -> dict[str, Any]:
        return {
            "user_id":           self.user_id,
            "name":              self.name,
            "mastery":           {str(k): v.to_dict() for k, v in self.mastery.items()},
            "seen_question_ids": self.seen_question_ids,
            "session_history":   self.session_history,
            "review_queue":      self.review_queue,
            "question_mastery":  {k: v.to_dict() for k, v in self.question_mastery.items()},
            "generated_strikes": self.generated_strikes,
            "onboarded":         self.onboarded,
            "created_at":        self.created_at,
            "last_seen":         self.last_seen,
        }

    @classmethod
    def from_dict(cls, d: dict[str, Any]) -> "UserProfile":
        mastery = {
            int(k): MasteryRecord.from_dict(v)
            for k, v in d.get("mastery", {}).items()
        }
        question_mastery = {
            k: MasteryRecord.from_dict(v)
            for k, v in d.get("question_mastery", {}).items()
        }
        return cls(
            user_id           = d["user_id"],
            name              = d["name"],
            mastery           = mastery,
            seen_question_ids = d.get("seen_question_ids", []),
            session_history   = d.get("session_history", []),
            review_queue      = d.get("review_queue", []),
            question_mastery  = question_mastery,
            generated_strikes = d.get("generated_strikes", {}),
            onboarded         = d.get("onboarded", False),
            created_at        = d.get("created_at", _now()),
            last_seen         = d.get("last_seen", _now()),
        )


# ── SessionResult ─────────────────────────────────────────────────────────────

@dataclass
class SessionResult:
    """Immutable record of one completed quiz session."""
    session_id:      str
    timestamp:       str
    scope_label:     str
    n_questions:     int
    n_correct:       int
    duration_secs:   float
    wrong_chapters:  list[int]
    # Enriched fields
    per_difficulty:  dict[str, list[int]] = field(default_factory=dict)  # {diff: [correct, total]}
    streak_max:      int = 0
    n_generated:     int = 0

    @property
    def score_pct(self) -> int:
        if self.n_questions == 0:
            return 0
        return round(100 * self.n_correct / self.n_questions)

    def to_dict(self) -> dict[str, Any]:
        return {
            "session_id":     self.session_id,
            "timestamp":      self.timestamp,
            "scope_label":    self.scope_label,
            "n_questions":    self.n_questions,
            "n_correct":      self.n_correct,
            "duration_secs":  self.duration_secs,
            "wrong_chapters": self.wrong_chapters,
            "per_difficulty": self.per_difficulty,
            "streak_max":     self.streak_max,
            "n_generated":    self.n_generated,
        }

    @classmethod
    def from_dict(cls, d: dict[str, Any]) -> "SessionResult":
        return cls(
            session_id     = d.get("session_id", str(uuid.uuid4())),
            timestamp      = d.get("timestamp", ""),
            scope_label    = d.get("scope_label", ""),
            n_questions    = d.get("n_questions", 0),
            n_correct      = d.get("n_correct", 0),
            duration_secs  = d.get("duration_secs", 0.0),
            wrong_chapters = d.get("wrong_chapters", []),
            per_difficulty = d.get("per_difficulty", {}),
            streak_max     = d.get("streak_max", 0),
            n_generated    = d.get("n_generated", 0),
        )


def _now() -> str:
    return datetime.now(timezone.utc).isoformat()
