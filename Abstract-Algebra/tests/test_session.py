"""Unit tests for quiz_app.session helpers and QuizSession."""
import pytest
from unittest.mock import MagicMock, patch
from quiz_app.session import QuizSession, _interleave
from quiz_app.adaptive import Scope
from quiz_app.models import MasteryRecord, Question, UserProfile


# ── _interleave ───────────────────────────────────────────────────────────────

class TestInterleave:
    def test_empty_b(self):
        assert _interleave([1, 2, 3], []) == [1, 2, 3]

    def test_empty_a(self):
        result = _interleave([], [10, 20])
        assert 10 in result and 20 in result

    def test_both_empty(self):
        assert _interleave([], []) == []

    def test_all_elements_present(self):
        a = [1, 2, 3, 4]
        b = [10, 20]
        result = _interleave(a, b)
        assert set(result) == set(a + b)

    def test_no_duplicates(self):
        a = [1, 2, 3, 4, 5]
        b = [10, 20]
        result = _interleave(a, b)
        assert len(result) == len(a) + len(b)

    def test_b_distributed(self):
        """b elements should not all be at the start or all at the end."""
        a = list(range(10))
        b = [99]
        result = _interleave(a, b)
        idx = result.index(99)
        # With step = max(1, 10//2) = 5, 99 appears at position 5
        assert 0 < idx < len(result)


# ── QuizSession._target_chapter_and_difficulty ────────────────────────────────

class TestTargetChapterAndDifficulty:
    def _session(self, scope: Scope, mastery: dict[int, float]) -> QuizSession:
        profile = UserProfile(user_id="u", name="T")
        for ch, score in mastery.items():
            profile.mastery[ch] = MasteryRecord(score=score, total_seen=5)
        return QuizSession(profile, scope, 10)

    def test_chapter_scope_uses_that_chapter(self):
        sess = self._session(Scope.chapter(3), {3: 0.6})
        ch, diff = sess._target_chapter_and_difficulty()
        assert ch == 3

    def test_adaptive_picks_weakest(self):
        sess = self._session(Scope.adaptive(), {0: 0.8, 1: 0.2, 2: 0.5})
        ch, diff = sess._target_chapter_and_difficulty()
        assert ch == 1  # weakest

    def test_no_mastery_defaults_to_chapter_0(self):
        profile = UserProfile(user_id="u", name="T")
        sess = QuizSession(profile, Scope.adaptive(), 10)
        ch, diff = sess._target_chapter_and_difficulty()
        assert ch == 0

    def test_difficulty_matches_mastery(self):
        # score 0.1 → beginner
        sess = self._session(Scope.chapter(0), {0: 0.1})
        _, diff = sess._target_chapter_and_difficulty()
        assert diff == "beginner"

        # score 0.9 → advanced
        sess = self._session(Scope.chapter(0), {0: 0.9})
        _, diff = sess._target_chapter_and_difficulty()
        assert diff == "advanced"


# ── QuizSession.run ───────────────────────────────────────────────────────────

def _make_question(chapter=0, difficulty="beginner") -> Question:
    return Question(
        chapter=chapter, phase=0, kind="mc",
        text="Q?", choices=["A", "B", "C", "D"], answer=0,
        explanation="E.", tags=[], difficulty=difficulty,
    )


class TestQuizSessionRun:
    def _run_session(self, questions, answers):
        """
        Run a QuizSession with a mocked UI (no terminal I/O) and mocked storage.
        `answers` is a list of booleans corresponding to each question.
        """
        profile = UserProfile(user_id="u", name="T")
        scope = Scope.all()
        sess = QuizSession(profile, scope, len(questions), generator=None)

        answer_iter = iter(answers)

        def _mock_present(*a, **kw):
            correct = next(answer_iter)
            return correct, False, 2  # (correct, flagged, confidence)

        with patch("quiz_app.session.select_questions", return_value=questions), \
             patch("quiz_app.session.ui.present_question", side_effect=_mock_present), \
             patch("quiz_app.session.ui.show_session_summary"), \
             patch("quiz_app.session.save_profile"):
            result = sess.run()

        return result, profile

    def test_perfect_score(self):
        qs = [_make_question() for _ in range(5)]
        result, _ = self._run_session(qs, [True] * 5)
        assert result.n_correct == 5
        assert result.n_questions == 5
        assert result.score_pct == 100

    def test_all_wrong(self):
        qs = [_make_question() for _ in range(5)]
        result, _ = self._run_session(qs, [False] * 5)
        assert result.n_correct == 0
        assert result.score_pct == 0

    def test_streak_counted(self):
        qs = [_make_question() for _ in range(6)]
        # wrong, then 4 correct, then wrong → max streak = 4
        answers = [False, True, True, True, True, False]
        result, _ = self._run_session(qs, answers)
        assert result.streak_max == 4

    def test_wrong_chapters_tracked(self):
        qs = [_make_question(chapter=0), _make_question(chapter=3)]
        result, _ = self._run_session(qs, [False, False])
        assert 0 in result.wrong_chapters
        assert 3 in result.wrong_chapters

    def test_mastery_updated_per_question(self):
        qs = [_make_question(chapter=0), _make_question(chapter=0)]
        _, profile = self._run_session(qs, [True, False])
        rec = profile.mastery.get(0)
        assert rec is not None
        assert rec.total_seen == 2
        assert rec.total_correct == 1

    def test_session_added_to_history(self):
        qs = [_make_question()]
        _, profile = self._run_session(qs, [True])
        assert len(profile.session_history) == 1
        assert profile.session_history[0]["n_questions"] == 1

    def test_empty_pool_returns_empty_result(self):
        profile = UserProfile(user_id="u", name="T")
        sess = QuizSession(profile, Scope.chapter(99), 10, generator=None)
        with patch("quiz_app.session.ui.print_wrap"), \
             patch("quiz_app.session.save_profile"):
            result = sess.run()
        assert result.n_questions == 0


# ── QuizSession._assemble_questions ──────────────────────────────────────────

class TestAssembleQuestions:
    def test_no_generator_all_static(self):
        profile = UserProfile(user_id="u", name="T")
        qs = [_make_question(chapter=i % 3) for i in range(10)]
        sess = QuizSession(profile, Scope.all(), 5, generator=None)
        with patch("quiz_app.session.QUESTIONS", qs), \
             patch("quiz_app.session.select_questions", return_value=qs[:5]):
            result = sess._assemble_questions()
        assert len(result) <= 5
        assert all(not q.generated for q in result)

    def test_generator_unavailable_falls_back_to_static(self):
        profile = UserProfile(user_id="u", name="T")
        qs = [_make_question() for _ in range(5)]
        gen = MagicMock()
        gen.available = False
        sess = QuizSession(profile, Scope.all(), 5, generator=gen)
        with patch("quiz_app.session.QUESTIONS", qs), \
             patch("quiz_app.session.select_questions", return_value=qs):
            result = sess._assemble_questions()
        gen.get_question.assert_not_called()
