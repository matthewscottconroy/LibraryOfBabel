"""Unit tests for quiz_app.adaptive."""
import random
import pytest
from quiz_app.adaptive import (
    Scope,
    difficulty_weight,
    filter_by_scope,
    mastery_summary,
    preferred_difficulty,
    select_questions,
    weak_topics,
    _phase_gate_factor,
    _user_phase_frontier,
)
from quiz_app.models import MasteryRecord, Question, UserProfile


# ── Helpers ───────────────────────────────────────────────────────────────────

def _q(chapter=0, phase=0, difficulty="intermediate", tags=None, **kwargs) -> Question:
    return Question(
        chapter=chapter, phase=phase, kind="mc",
        text="Q?", choices=["A", "B", "C", "D"], answer=0,
        explanation="E.", tags=tags or [],
        difficulty=difficulty,
    )


def _profile(*chapters: int) -> UserProfile:
    """Create a profile that has seen 5 questions in each listed chapter."""
    p = UserProfile(user_id="u1", name="T")
    for ch in chapters:
        p.mastery[ch] = MasteryRecord(score=0.5, total_seen=5)
    return p


# ── Scope ─────────────────────────────────────────────────────────────────────

class TestScope:
    def test_label_adaptive(self):
        assert "Adaptive" in Scope.adaptive().label

    def test_label_chapter(self):
        assert "5" in Scope.chapter(5).label

    def test_label_phase(self):
        assert "3" in Scope.phase(3).label

    def test_label_tag(self):
        assert "groups" in Scope.tag("groups").label


# ── preferred_difficulty ──────────────────────────────────────────────────────

class TestPreferredDifficulty:
    def test_low_mastery(self):
        assert preferred_difficulty(0.1) == "beginner"

    def test_mid_mastery(self):
        assert preferred_difficulty(0.5) == "intermediate"

    def test_high_mastery(self):
        assert preferred_difficulty(0.9) == "advanced"

    def test_boundary_beginner(self):
        # exactly at DIFF_BEGINNER_MAX (0.35) should be intermediate
        assert preferred_difficulty(0.35) == "intermediate"

    def test_boundary_intermediate(self):
        # exactly at DIFF_INTERMEDIATE_MAX (0.68) should be advanced
        assert preferred_difficulty(0.68) == "advanced"


# ── difficulty_weight ─────────────────────────────────────────────────────────

class TestDifficultyWeight:
    def test_preferred_gets_full_weight(self):
        # mastery 0.1 → preferred = beginner
        assert difficulty_weight("beginner", 0.1) == pytest.approx(1.0)

    def test_one_step_away(self):
        # mastery 0.1 → preferred = beginner; intermediate is 1 step away
        assert difficulty_weight("intermediate", 0.1) == pytest.approx(0.35)

    def test_two_steps_away(self):
        # mastery 0.1 → preferred = beginner; advanced is 2 steps away
        assert difficulty_weight("advanced", 0.1) == pytest.approx(0.05)


# ── filter_by_scope ───────────────────────────────────────────────────────────

class TestFilterByScope:
    def _pool(self):
        return [
            _q(chapter=0, phase=0, tags=["logic"]),
            _q(chapter=1, phase=0, tags=["sets"]),
            _q(chapter=2, phase=1, tags=["linear-algebra", "logic"]),
            _q(chapter=8, phase=2, tags=["groups"]),
        ]

    def test_all(self):
        pool = self._pool()
        assert len(filter_by_scope(pool, Scope.all())) == 4

    def test_adaptive_returns_all(self):
        pool = self._pool()
        assert len(filter_by_scope(pool, Scope.adaptive())) == 4

    def test_phase_filter(self):
        pool = self._pool()
        result = filter_by_scope(pool, Scope.phase(0))
        assert all(q.phase == 0 for q in result)
        assert len(result) == 2

    def test_chapter_filter(self):
        pool = self._pool()
        result = filter_by_scope(pool, Scope.chapter(2))
        assert len(result) == 1
        assert result[0].chapter == 2

    def test_tag_filter(self):
        pool = self._pool()
        result = filter_by_scope(pool, Scope.tag("logic"))
        assert len(result) == 2  # ch0 and ch2 have "logic" tag

    def test_tag_filter_case_insensitive(self):
        pool = self._pool()
        result = filter_by_scope(pool, Scope.tag("LINEAR"))
        assert len(result) == 1

    def test_no_match_returns_empty(self):
        pool = self._pool()
        result = filter_by_scope(pool, Scope.chapter(99))
        assert result == []


# ── select_questions ──────────────────────────────────────────────────────────

class TestSelectQuestions:
    def _pool(self, n=10):
        return [_q(chapter=i % 5, phase=0) for i in range(n)]

    def test_returns_correct_count(self):
        pool = self._pool(10)
        p = UserProfile(user_id="u", name="T")
        selected = select_questions(pool, p, 5, Scope.all(), rng=random.Random(42))
        assert len(selected) == 5

    def test_no_duplicates(self):
        pool = self._pool(10)
        p = UserProfile(user_id="u", name="T")
        selected = select_questions(pool, p, 10, Scope.all(), rng=random.Random(0))
        ids = [q.question_id for q in selected]
        assert len(ids) == len(set(ids))

    def test_empty_pool_returns_empty(self):
        p = UserProfile(user_id="u", name="T")
        selected = select_questions([], p, 5, Scope.all())
        assert selected == []

    def test_n_larger_than_pool(self):
        pool = self._pool(3)
        p = UserProfile(user_id="u", name="T")
        selected = select_questions(pool, p, 100, Scope.all(), rng=random.Random(1))
        assert len(selected) == 3

    def test_deterministic_with_seed(self):
        pool = self._pool(20)
        p = UserProfile(user_id="u", name="T")
        s1 = [q.question_id for q in select_questions(pool, p, 5, Scope.all(), rng=random.Random(99))]
        s2 = [q.question_id for q in select_questions(pool, p, 5, Scope.all(), rng=random.Random(99))]
        assert s1 == s2

    def test_scope_respected(self):
        pool = [_q(chapter=0, phase=0), _q(chapter=5, phase=1)]
        p = UserProfile(user_id="u", name="T")
        selected = select_questions(pool, p, 5, Scope.chapter(0), rng=random.Random(0))
        assert all(q.chapter == 0 for q in selected)


# ── phase gate ────────────────────────────────────────────────────────────────

class TestPhaseGate:
    # CHAPTER_META phases: ch0-1=ph0, ch2-7=ph1, ch8-9=ph2, ch10-12=ph3,
    #   ch13=ph4, ch14-16=ph5, ch17-18=ph6, ch19-22=ph7, ch23-27=ph8

    def test_same_phase_no_penalty(self):
        # ch0 is phase 0, frontier 0 → gap 0 ≤ 1 → 1.0
        assert _phase_gate_factor(0, 0) == pytest.approx(1.0)

    def test_one_ahead_no_penalty(self):
        # ch8 is phase 2, frontier 1 → gap 1 ≤ 1 → 1.0
        assert _phase_gate_factor(8, 1) == pytest.approx(1.0)

    def test_two_ahead_penalty(self):
        # ch8 is phase 2, frontier 0 → gap 2 → 0.5
        assert _phase_gate_factor(8, 0) == pytest.approx(0.5)

    def test_far_ahead_heavy_penalty(self):
        # ch23 is phase 8, frontier 0 → gap 8 → 0.05
        assert _phase_gate_factor(23, 0) == pytest.approx(0.05)


class TestUserPhaseFrontier:
    def test_empty_profile(self):
        p = UserProfile(user_id="u", name="T")
        assert _user_phase_frontier(p) == 0

    def test_only_chapters_seen_less_than_3(self):
        p = UserProfile(user_id="u", name="T")
        p.mastery[0] = MasteryRecord(total_seen=2)  # phase 0, not enough
        assert _user_phase_frontier(p) == 0

    def test_reaches_correct_phase(self):
        p = UserProfile(user_id="u", name="T")
        p.mastery[0] = MasteryRecord(total_seen=5)   # ch0 = phase 0
        p.mastery[8] = MasteryRecord(total_seen=5)   # ch8 = phase 2
        assert _user_phase_frontier(p) == 2


# ── mastery_summary / weak_topics ─────────────────────────────────────────────

class TestMasterySummary:
    def test_includes_all_chapters(self):
        from quiz_app.config import CHAPTER_META
        p = UserProfile(user_id="u", name="T")
        rows = mastery_summary(p)
        assert len(rows) == len(CHAPTER_META)

    def test_unstarted_chapter(self):
        from quiz_app.config import INITIAL_MASTERY
        p = UserProfile(user_id="u", name="T")
        rows = mastery_summary(p)
        assert all(r["started"] is False for r in rows)
        assert all(r["score"] == INITIAL_MASTERY for r in rows)

    def test_started_chapter(self):
        p = UserProfile(user_id="u", name="T")
        p.mastery[0] = MasteryRecord(score=0.8, total_seen=3, total_correct=3)
        rows = mastery_summary(p)
        row0 = next(r for r in rows if r["chapter"] == 0)
        assert row0["started"] is True
        assert row0["score"] == pytest.approx(0.8)


class TestWeakTopics:
    def test_returns_names(self):
        p = UserProfile(user_id="u", name="T")
        p.mastery[0] = MasteryRecord(score=0.2, total_seen=3)
        p.mastery[1] = MasteryRecord(score=0.9, total_seen=3)
        names = weak_topics(p, n=1)
        assert len(names) == 1
        # ch0 is weaker, so it should come first
        assert names[0] != ""

    def test_excludes_unstarted(self):
        p = UserProfile(user_id="u", name="T")
        # no mastery at all
        names = weak_topics(p)
        assert names == []
