"""
QuizSession — orchestrates a single quiz run.

Ties together: adaptive selection, optional Claude generation,
question presentation, mastery updates, and session persistence.
"""
from __future__ import annotations

import hashlib
import threading
import time
import uuid
from datetime import datetime, timezone
from typing import Optional

from .adaptive import Scope, filter_by_scope, select_questions, weak_topics, preferred_difficulty
from .config import CHAPTER_META, AUTO_STOP_WINDOW, AUTO_STOP_THRESHOLD
from .generator import ClaudeGenerator
from .models import MasteryRecord, Question, SessionResult, UserProfile
from .question_bank import QUESTIONS
from .storage import save_profile
from . import ui


def _question_text_hash(text: str) -> str:
    """Short stable hash of question text — used to track generated question strike counts."""
    return hashlib.sha256(text.lower().strip().encode()).hexdigest()[:16]


class QuizSession:
    """
    Runs one quiz: selects N questions, presents them, updates mastery.
    """

    def __init__(
        self,
        profile: UserProfile,
        scope: Scope,
        n: int,
        generator: Optional[ClaudeGenerator] = None,
        *,
        generated_ratio: float = 0.35,
        pool: Optional[list[Question]] = None,
        time_limit_secs: Optional[int] = None,
        adaptive_stop: bool = True,
    ):
        self.profile          = profile
        self.scope            = scope
        self.n                = n
        self.generator        = generator
        self.generated_ratio  = generated_ratio
        self.pool             = pool  # override question bank when provided
        self.time_limit_secs  = time_limit_secs
        self.adaptive_stop    = adaptive_stop

    # ── Public entry point ────────────────────────────────────────────────────

    def run(self) -> SessionResult:
        questions = self._assemble_questions()
        if not questions:
            ui.print_wrap("No questions match your selection. Try a broader scope.")
            return self._empty_result()

        score          = 0
        streak         = 0
        streak_max     = 0
        wrong_chs      : list[int] = []
        n_generated    = 0
        mastery_deltas : list[float] = []
        per_diff       : dict[str, list[int]] = {
            d: [0, 0] for d in ("beginner", "intermediate", "advanced")
        }
        per_chapter    : dict[int, list[int]] = {}  # {chapter: [correct, total]}
        start = time.monotonic()

        try:
            for i, q in enumerate(questions, 1):
                # Snapshot mastery before answering for plateau detection
                ch_rec_before = self.profile.mastery.get(q.chapter)
                score_before  = ch_rec_before.score if ch_rec_before else 0.5

                correct, flagged, confidence = ui.present_question(
                    q, i, len(questions),
                    running_correct=score,
                    time_limit_secs=self.time_limit_secs,
                )
                self.profile.record_answer(
                    q.chapter, correct, q.question_id,
                    confidence=confidence,
                )

                # Update review queue based on correctness and manual flag
                if correct and not flagged:
                    self.profile.remove_from_review_queue(q.question_id)
                else:
                    self.profile.add_to_review_queue(q.question_id)

                # Track generated question quality via strike count
                if q.generated and not correct:
                    h = _question_text_hash(q.text)
                    self.profile.generated_strikes[h] = (
                        self.profile.generated_strikes.get(h, 0) + 1
                    )

                bucket = per_diff.setdefault(q.difficulty, [0, 0])
                ch_stat = per_chapter.setdefault(q.chapter, [0, 0])
                bucket[1] += 1
                ch_stat[1] += 1
                if correct:
                    score += 1
                    bucket[0] += 1
                    ch_stat[0] += 1
                    streak += 1
                    streak_max = max(streak_max, streak)
                else:
                    wrong_chs.append(q.chapter)
                    streak = 0

                if q.generated:
                    n_generated += 1

                # Plateau detection: auto-stop when mastery has stopped moving
                score_after = self.profile.mastery[q.chapter].score
                mastery_deltas.append(abs(score_after - score_before))
                if (
                    self.adaptive_stop
                    and len(mastery_deltas) >= AUTO_STOP_WINDOW
                    and sum(mastery_deltas[-AUTO_STOP_WINDOW:]) < AUTO_STOP_THRESHOLD
                    and i < len(questions)  # don't prompt on the last question
                ):
                    save_profile(self.profile)
                    print()
                    ui.print_wrap(
                        f"Your mastery has plateaued over the last {AUTO_STOP_WINDOW} "
                        "questions. You may have reached your limit for this session."
                    )
                    if not ui.confirm("Continue?", default=True):
                        break
                    mastery_deltas = []  # reset to avoid re-triggering immediately

                save_profile(self.profile)

        except ui.QuizQuit:
            pass  # partial session — fall through to summary

        duration = time.monotonic() - start
        wrong_names = list(dict.fromkeys(
            CHAPTER_META.get(ch, {}).get("name", f"Ch.{ch}")
            for ch in wrong_chs
        ))

        total_answered = sum(b[1] for b in per_diff.values())
        ui.show_session_summary(score, total_answered, wrong_names, duration,
                                streak_max, per_diff, n_generated, per_chapter)

        result = SessionResult(
            session_id     = str(uuid.uuid4()),
            timestamp      = datetime.now(timezone.utc).isoformat(),
            scope_label    = self.scope.label,
            n_questions    = total_answered,
            n_correct      = score,
            duration_secs  = duration,
            wrong_chapters = list(set(wrong_chs)),
            per_difficulty = per_diff,
            streak_max     = streak_max,
            n_generated    = n_generated,
        )
        self.profile.session_history.append(result.to_dict())
        save_profile(self.profile)

        self._start_background_prefill()

        return result

    # ── Question assembly ─────────────────────────────────────────────────────

    def _assemble_questions(self) -> list[Question]:
        """
        Mix static bank questions with Claude-generated ones.
        Falls back entirely to static when the generator is unavailable.
        Filters out generated questions that have accumulated too many strikes.
        """
        bank = self.pool if self.pool is not None else QUESTIONS

        n_generated = (
            max(1, int(self.n * self.generated_ratio))
            if (self.generator and self.generator.available)
            else 0
        )
        n_static = self.n - n_generated

        static = select_questions(bank, self.profile, n_static, self.scope)
        selected_ids: set[str] = {q.question_id for q in static}

        generated: list[Question] = []
        if n_generated > 0 and self.generator:
            chapter, difficulty = self._target_chapter_and_difficulty()
            weaks = weak_topics(self.profile)
            mastery_rec = self.profile.mastery.get(chapter)
            mastery_pct = int((mastery_rec.score if mastery_rec else 0.5) * 100)

            for _ in range(n_generated):
                q = self.generator.get_question(chapter, difficulty, mastery_pct, weaks)
                if q:
                    # Filter out questions with too many strikes (repeatedly wrong on them)
                    if self.profile.generated_strikes.get(_question_text_hash(q.text), 0) < 2:
                        generated.append(q)
                        continue
                # Fall back to a static question
                remaining = [x for x in bank if x.question_id not in selected_ids]
                extra = select_questions(remaining, self.profile, 1, self.scope)
                if extra:
                    static.extend(extra)
                    selected_ids.add(extra[0].question_id)

        combined = _interleave(static, generated)
        return combined[: self.n]

    def _target_chapter_and_difficulty(self) -> tuple[int, str]:
        """Pick the best chapter+difficulty for generation based on scope/mastery."""
        if self.scope.kind == "chapter" and isinstance(self.scope.value, int):
            ch = self.scope.value
        elif self.scope.kind == "phase" and isinstance(self.scope.value, int):
            ph = self.scope.value
            chs = [k for k, v in CHAPTER_META.items() if v["phase"] == ph]
            ch = min(chs, key=lambda c: self.profile.mastery.get(c, MasteryRecord()).score) if chs else 0
        elif self.scope.kind in ("adaptive", "all"):
            started = [c for c in self.profile.mastery if self.profile.mastery[c].total_seen > 0]
            ch = min(started, key=lambda c: self.profile.mastery[c].score) if started else 0
        else:
            ch = 0

        rec  = self.profile.mastery.get(ch)
        diff = preferred_difficulty(rec.score if rec else 0.5)
        return ch, diff

    def _empty_result(self) -> SessionResult:
        return SessionResult(
            session_id    = str(uuid.uuid4()),
            timestamp     = "",
            scope_label   = self.scope.label,
            n_questions   = 0,
            n_correct     = 0,
            duration_secs = 0.0,
            wrong_chapters= [],
        )

    # ── Background cache prefill ──────────────────────────────────────────────

    def _start_background_prefill(self) -> None:
        """
        After a session ends, pre-generate questions for the user's weakest
        chapters so the next session's Claude questions are instant.
        """
        if not (self.generator and self.generator.available):
            return
        started = [c for c, r in self.profile.mastery.items() if r.total_seen > 0]
        if not started:
            return

        weakest = sorted(started, key=lambda c: self.profile.mastery[c].score)[:3]
        weaks   = weak_topics(self.profile)
        profile_snapshot = {
            ch: self.profile.mastery[ch].score for ch in weakest
        }

        def _prefill() -> None:
            for ch in weakest:
                mastery_pct = int(profile_snapshot.get(ch, 0.5) * 100)
                self.generator.prefill_for_chapter(ch, mastery_pct, weaks)  # type: ignore[union-attr]

        threading.Thread(target=_prefill, daemon=True).start()


# ── Helpers ───────────────────────────────────────────────────────────────────

def _interleave(a: list, b: list) -> list:
    """Interleave two lists roughly evenly."""
    if not b:
        return a
    combined = []
    bi = 0
    step = max(1, len(a) // (len(b) + 1))
    for i, item in enumerate(a):
        combined.append(item)
        if bi < len(b) and (i + 1) % step == 0:
            combined.append(b[bi])
            bi += 1
    combined.extend(b[bi:])
    return combined
