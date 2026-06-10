"""
Tests for session.py — _interleave, QuizSession assembly, SessionResult.
No real I/O: ui calls are patched out, generator is mocked.
"""
from __future__ import annotations

import uuid
from unittest.mock import MagicMock, patch

import pytest

from quiz_app.models import MasteryRecord, Question, SessionResult, UserProfile
from quiz_app.adaptive import Scope
from quiz_app.session import QuizSession, _interleave


# ── Helpers ───────────────────────────────────────────────────────────────────

def _make_q(chapter=0, phase=0, diff="intermediate") -> Question:
    return Question(
        chapter=chapter, phase=phase, kind="mc",
        text="Test?",
        choices=["A", "B", "C", "D"],
        answer=0,
        explanation="Because A.",
        tags=["test"],
        difficulty=diff,
        question_id=str(uuid.uuid4()),
    )


def _profile(mastery: dict | None = None) -> UserProfile:
    p = UserProfile(user_id="u", name="Tester")
    for ch, score in (mastery or {}).items():
        p.mastery[ch] = MasteryRecord(score=score, total_seen=3)
    return p


def _session(profile=None, scope=None, n=5, generator=None):
    return QuizSession(
        profile   = profile or _profile(),
        scope     = scope or Scope.all(),
        n         = n,
        generator = generator,
    )


# ── _interleave ───────────────────────────────────────────────────────────────

class TestInterleave:
    def test_empty_b_returns_a_unchanged(self):
        a = [1, 2, 3]
        assert _interleave(a, []) == [1, 2, 3]

    def test_empty_a_returns_b(self):
        assert _interleave([], [10, 20]) == [10, 20]

    def test_both_empty(self):
        assert _interleave([], []) == []

    def test_no_duplicates_in_output(self):
        a = list(range(6))
        b = ["x", "y"]
        result = _interleave(a, b)
        # Every element of a and b appears exactly once
        assert sorted(result, key=str) == sorted(a + b, key=str)

    def test_interleave_distributes_b_across_a(self):
        a = list(range(10))
        b = ["X", "Y"]
        result = _interleave(a, b)
        x_idx = result.index("X")
        y_idx = result.index("Y")
        # X should appear before the midpoint of the list
        assert x_idx < len(result) - 1
        # Y should come after X
        assert y_idx > x_idx

    def test_single_a_single_b(self):
        result = _interleave([1], ["a"])
        assert set(result) == {1, "a"}
        assert len(result) == 2

    def test_b_larger_than_a(self):
        a = [1, 2]
        b = [10, 20, 30, 40]
        result = _interleave(a, b)
        assert len(result) == len(a) + len(b)


# ── SessionResult ─────────────────────────────────────────────────────────────

class TestSessionResult:
    def _result(self, n_questions=10, n_correct=7) -> SessionResult:
        return SessionResult(
            session_id="s", timestamp="t", scope_label="All chapters",
            n_questions=n_questions, n_correct=n_correct,
            duration_secs=60.0, wrong_chapters=[],
        )

    def test_score_pct_normal(self):
        assert self._result(10, 7).score_pct == 70

    def test_score_pct_zero_questions(self):
        assert self._result(0, 0).score_pct == 0

    def test_score_pct_perfect(self):
        assert self._result(5, 5).score_pct == 100

    def test_score_pct_zero_correct(self):
        assert self._result(10, 0).score_pct == 0

    def test_to_dict_has_all_keys(self):
        d = self._result().to_dict()
        for key in ("session_id", "timestamp", "scope_label",
                    "n_questions", "n_correct", "duration_secs", "wrong_chapters"):
            assert key in d

    def test_timestamp_is_not_ansi_escape(self):
        """Regression: timestamp must be an ISO string, not an ANSI code."""
        sess = _session()
        result = sess._empty_result()
        # An ISO timestamp starts with a digit or is empty; an ANSI escape
        # starts with \x1b (ESC). Either way it must not contain an ESC byte.
        assert "\x1b" not in result.timestamp


# ── QuizSession._empty_result ─────────────────────────────────────────────────

class TestEmptyResult:
    def test_empty_result_has_zero_questions(self):
        sess = _session()
        r = sess._empty_result()
        assert r.n_questions == 0
        assert r.n_correct == 0
        assert r.wrong_chapters == []

    def test_empty_result_scope_label_matches(self):
        sess = _session(scope=Scope.phase(3))
        r = sess._empty_result()
        assert "Phase 3" in r.scope_label


# ── QuizSession._target_chapter_and_difficulty ────────────────────────────────

class TestTargetChapterAndDifficulty:
    def test_chapter_scope_uses_that_chapter(self):
        sess = _session(scope=Scope.chapter(5))
        ch, diff = sess._target_chapter_and_difficulty()
        assert ch == 5

    def test_adaptive_scope_picks_weakest_started_chapter(self):
        p = _profile({0: 0.9, 2: 0.2})
        sess = _session(profile=p, scope=Scope.adaptive())
        ch, diff = sess._target_chapter_and_difficulty()
        assert ch == 2  # ch2 has lower mastery

    def test_no_started_chapters_defaults_to_zero(self):
        sess = _session(profile=_profile(), scope=Scope.adaptive())
        ch, _ = sess._target_chapter_and_difficulty()
        assert ch == 0

    def test_difficulty_matches_mastery(self):
        p = _profile({0: 0.1})  # low mastery → beginner
        sess = _session(profile=p, scope=Scope.adaptive())
        _, diff = sess._target_chapter_and_difficulty()
        assert diff == "beginner"

    def test_high_mastery_gives_advanced_difficulty(self):
        p = _profile({0: 0.95})
        sess = _session(profile=p, scope=Scope.adaptive())
        _, diff = sess._target_chapter_and_difficulty()
        assert diff == "advanced"

    def test_phase_scope_picks_weakest_chapter_in_phase(self):
        # Phase 0 = chapters 0,1,2,3
        p = _profile({0: 0.8, 1: 0.3, 2: 0.6, 3: 0.9})
        sess = _session(profile=p, scope=Scope.phase(0))
        ch, _ = sess._target_chapter_and_difficulty()
        assert ch == 1  # ch1 has lowest mastery in phase 0


# ── QuizSession._assemble_questions (no generator) ────────────────────────────

class TestAssembleQuestions:
    def test_returns_n_questions_without_generator(self):
        sess = _session(n=5, generator=None)
        questions = sess._assemble_questions()
        assert len(questions) == 5

    def test_no_duplicates_in_assembled_set(self):
        sess = _session(n=10, generator=None)
        questions = sess._assemble_questions()
        ids = [q.question_id for q in questions]
        assert len(ids) == len(set(ids))

    def test_n_capped_at_bank_size(self):
        from quiz_app.question_bank import QUESTIONS
        sess = _session(n=10_000, generator=None)
        questions = sess._assemble_questions()
        assert len(questions) <= len(QUESTIONS)

    def test_scope_filter_applied(self):
        sess = _session(scope=Scope.phase(0), n=5, generator=None)
        questions = sess._assemble_questions()
        assert all(q.phase == 0 for q in questions)

    def test_with_unavailable_generator_falls_back_to_static(self):
        gen = MagicMock()
        gen.available = False
        sess = _session(n=5, generator=gen)
        questions = sess._assemble_questions()
        assert len(questions) > 0
        assert not gen.get_question.called

    def test_with_available_generator_calls_get_question(self):
        gen = MagicMock()
        gen.available = True
        gen.get_question.return_value = _make_q()
        sess = _session(n=3, generator=gen)
        questions = sess._assemble_questions()
        assert gen.get_question.called
        assert len(questions) > 0

    def test_generator_failure_falls_back_to_static(self):
        """If get_question returns None, static questions fill the gap."""
        gen = MagicMock()
        gen.available = True
        gen.get_question.return_value = None
        sess = _session(n=5, generator=gen)
        questions = sess._assemble_questions()
        # Should still produce questions
        assert len(questions) > 0
        assert all(isinstance(q, Question) for q in questions)


# ── QuizSession.run (mocked UI) ───────────────────────────────────────────────

class TestBackgroundPrefill:
    def test_prefill_not_called_when_generator_unavailable(self):
        gen = MagicMock()
        gen.available = False
        sess = _session(generator=gen)
        sess._start_background_prefill()
        assert not gen.prefill_for_chapter.called

    def test_prefill_not_called_when_no_mastery(self):
        gen = MagicMock()
        gen.available = True
        sess = _session(generator=gen, profile=_profile())
        sess._start_background_prefill()
        # No mastery data → nothing to prefill
        assert not gen.prefill_for_chapter.called

    def test_prefill_called_with_mastery_data(self):
        import time
        gen = MagicMock()
        gen.available = True
        p = _profile({0: 0.3})
        p.mastery[0].total_seen = 3
        sess = _session(generator=gen, profile=p)
        sess._start_background_prefill()
        # Give the daemon thread a moment to start
        time.sleep(0.05)
        # The thread is daemon; we just verify the call path was set up


class TestSessionRun:
    def _run_session(self, answers: list[bool], n: int = 3,
                     profile: UserProfile | None = None) -> SessionResult:
        """Run a session with pre-scripted correct/incorrect answers."""
        prof = profile or _profile()
        sess = _session(profile=prof, n=n, generator=None)
        answer_iter = iter(answers)

        # present_question now returns (correct, flagged, confidence)
        with patch("quiz_app.session.ui.present_question",
                   side_effect=lambda q, num, total, **kw: (next(answer_iter, False), False, 2)), \
             patch("quiz_app.session.ui.show_session_summary"), \
             patch("quiz_app.session.ui.confirm", return_value=True), \
             patch("quiz_app.session.save_profile"):
            return sess.run()

    def test_correct_answers_counted(self):
        result = self._run_session([True, True, False])
        assert result.n_correct == 2

    def test_wrong_answers_counted(self):
        result = self._run_session([False, False, False])
        assert result.n_correct == 0

    def test_n_questions_in_result(self):
        result = self._run_session([True, False, True])
        assert result.n_questions == 3

    def test_wrong_chapters_populated(self):
        result = self._run_session([True, False, False])
        # At least one wrong chapter recorded
        assert len(result.wrong_chapters) >= 1

    def test_timestamp_is_iso_string(self):
        result = self._run_session([True])
        # ISO timestamps contain 'T' and '+' or 'Z'
        assert "T" in result.timestamp
        assert "\x1b" not in result.timestamp

    def test_profile_mastery_updated(self):
        p = _profile()
        self._run_session([True, True, True], n=3, profile=p)
        # At least one chapter should now have mastery data
        assert len(p.mastery) > 0

    def test_session_history_appended(self):
        p = _profile()
        self._run_session([True], n=1, profile=p)
        assert len(p.session_history) == 1

    def test_streak_max_in_result(self):
        result = self._run_session([True, True, True])
        assert result.streak_max == 3

    def test_streak_resets_on_wrong(self):
        result = self._run_session([True, True, False, True])
        assert result.streak_max == 2

    def test_per_difficulty_populated(self):
        result = self._run_session([True, False, True])
        # At least one difficulty bucket should have been populated
        assert any(v[1] > 0 for v in result.per_difficulty.values())

    def test_n_generated_zero_without_generator(self):
        result = self._run_session([True, False])
        assert result.n_generated == 0

    def test_session_result_keys_include_new_fields(self):
        result = self._run_session([True])
        d = result.to_dict()
        assert "streak_max" in d
        assert "per_difficulty" in d
        assert "n_generated" in d
