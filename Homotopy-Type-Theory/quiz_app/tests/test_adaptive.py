"""Tests for adaptive.py — difficulty mapping, question selection, scope."""
import random

import pytest

from quiz_app.adaptive import (
    Scope,
    filter_by_scope,
    difficulty_weight,
    preferred_difficulty,
    select_questions,
    mastery_summary,
    weak_topics,
    _chapter_mastery,
    _chapter_weight,
    _user_phase_frontier,
    _phase_gate_factor,
)
from quiz_app.models import MasteryRecord, Question, UserProfile


# ── Fixtures ──────────────────────────────────────────────────────────────────

def _make_q(chapter=0, phase=0, diff="intermediate", kind="mc",
            qid=None) -> Question:
    import uuid
    return Question(
        chapter=chapter, phase=phase, kind=kind,
        text="Test question?",
        choices=["A", "B", "C", "D"],
        answer=0,
        explanation="Because A.",
        tags=["test"],
        difficulty=diff,
        question_id=qid or str(uuid.uuid4()),
    )


def _profile(mastery_scores: dict | None = None) -> UserProfile:
    p = UserProfile(user_id="u", name="Test")
    for ch, score in (mastery_scores or {}).items():
        p.mastery[ch] = MasteryRecord(score=score, total_seen=5)
    return p


# ── preferred_difficulty ──────────────────────────────────────────────────────

class TestPreferredDifficulty:
    def test_low_mastery_gives_beginner(self):
        assert preferred_difficulty(0.0) == "beginner"
        assert preferred_difficulty(0.3) == "beginner"

    def test_mid_mastery_gives_intermediate(self):
        assert preferred_difficulty(0.5) == "intermediate"

    def test_high_mastery_gives_advanced(self):
        assert preferred_difficulty(0.9) == "advanced"
        assert preferred_difficulty(1.0) == "advanced"

    def test_threshold_boundaries(self):
        # Just below beginner threshold
        assert preferred_difficulty(0.349) == "beginner"
        # At or just above intermediate threshold
        assert preferred_difficulty(0.68) == "advanced"


# ── difficulty_weight ─────────────────────────────────────────────────────────

class TestDifficultyWeight:
    def test_matching_difficulty_gives_full_weight(self):
        assert difficulty_weight("beginner", 0.1) == pytest.approx(1.0)
        assert difficulty_weight("intermediate", 0.5) == pytest.approx(1.0)
        assert difficulty_weight("advanced", 0.9) == pytest.approx(1.0)

    def test_adjacent_difficulty_gives_partial_weight(self):
        w = difficulty_weight("intermediate", 0.1)  # preferred=beginner
        assert 0.1 < w < 1.0

    def test_far_difficulty_gives_low_weight(self):
        w = difficulty_weight("advanced", 0.1)  # preferred=beginner
        assert w < 0.2


# ── filter_by_scope ──────────────────────────────────────────────────────────

class TestFilterByScope:
    def _pool(self):
        return [
            _make_q(chapter=0, phase=0, diff="beginner"),
            _make_q(chapter=1, phase=0, diff="intermediate"),
            _make_q(chapter=10, phase=3, diff="advanced"),
        ]

    def test_all_returns_all(self):
        pool = self._pool()
        assert len(filter_by_scope(pool, Scope.all())) == 3

    def test_adaptive_returns_all(self):
        pool = self._pool()
        assert len(filter_by_scope(pool, Scope.adaptive())) == 3

    def test_phase_filter(self):
        pool = self._pool()
        result = filter_by_scope(pool, Scope.phase(0))
        assert all(q.phase == 0 for q in result)
        assert len(result) == 2

    def test_chapter_filter(self):
        pool = self._pool()
        result = filter_by_scope(pool, Scope.chapter(10))
        assert len(result) == 1
        assert result[0].chapter == 10

    def test_tag_filter(self):
        import uuid
        pool = [
            Question(0, 0, "mc", "q", ["A","B"], 0, "e", ["HoTT", "paths"],
                     "intermediate", False, str(uuid.uuid4())),
            Question(1, 0, "mc", "q", ["A","B"], 0, "e", ["algebra"],
                     "intermediate", False, str(uuid.uuid4())),
        ]
        result = filter_by_scope(pool, Scope.tag("hott"))
        assert len(result) == 1

    def test_empty_pool_returns_empty(self):
        assert filter_by_scope([], Scope.all()) == []


# ── select_questions ──────────────────────────────────────────────────────────

class TestSelectQuestions:
    def _big_pool(self, n=30):
        import uuid
        diffs = ["beginner", "intermediate", "advanced"]
        return [
            _make_q(chapter=i % 5, phase=0, diff=diffs[i % 3],
                    qid=str(uuid.uuid4()))
            for i in range(n)
        ]

    def test_returns_correct_count(self):
        pool = self._big_pool()
        rng = random.Random(42)
        result = select_questions(pool, _profile(), 10, Scope.all(), rng)
        assert len(result) == 10

    def test_no_duplicates(self):
        pool = self._big_pool()
        rng = random.Random(42)
        result = select_questions(pool, _profile(), 20, Scope.all(), rng)
        ids = [q.question_id for q in result]
        assert len(ids) == len(set(ids))

    def test_n_larger_than_pool_capped(self):
        pool = self._big_pool(5)
        rng = random.Random(0)
        result = select_questions(pool, _profile(), 100, Scope.all(), rng)
        assert len(result) == 5

    def test_recent_questions_deprioritised(self):
        """Questions seen recently should appear less often."""
        import uuid
        # Build a pool of 10 questions, mark first 5 as recently seen
        pool = [_make_q(qid=f"q{i}") for i in range(10)]
        profile = _profile()
        profile.seen_question_ids = [f"q{i}" for i in range(5)]

        # Run many selections and count how often recent questions appear
        rng = random.Random(7)
        recent_count = 0
        trials = 200
        for _ in range(trials):
            result = select_questions(pool, profile, 1, Scope.all(), rng)
            if result and result[0].question_id in profile.seen_question_ids:
                recent_count += 1
        # Recent questions should appear < 50% of the time (significantly less than 5/10=50%)
        assert recent_count / trials < 0.45

    def test_adaptive_focuses_on_weak_chapters(self):
        """With adaptive scope, weak chapters should be selected more."""
        import uuid
        # 10 questions for ch0 (weak) and 10 for ch1 (strong)
        pool = (
            [_make_q(chapter=0, qid=f"w{i}") for i in range(10)] +
            [_make_q(chapter=1, qid=f"s{i}") for i in range(10)]
        )
        profile = _profile({0: 0.1, 1: 0.9})

        rng = random.Random(99)
        weak_count = sum(
            1 for _ in range(200)
            for q in select_questions(pool, profile, 1, Scope.adaptive(), rng)
            if q.chapter == 0
        )
        # Weak chapter (ch0) should be selected more than 60% of the time
        assert weak_count / 200 > 0.55

    def test_empty_pool_returns_empty(self):
        rng = random.Random(0)
        assert select_questions([], _profile(), 5, Scope.all(), rng) == []


# ── mastery_summary & weak_topics ────────────────────────────────────────────

class TestMasterySummary:
    def test_all_chapters_present(self):
        from quiz_app.config import CHAPTER_META
        p = _profile()
        rows = mastery_summary(p)
        assert len(rows) == len(CHAPTER_META)

    def test_unstarted_chapter_score_is_initial_mastery(self):
        from quiz_app.config import INITIAL_MASTERY
        p = _profile()
        rows = {r["chapter"]: r for r in mastery_summary(p)}
        assert rows[0]["score"] == pytest.approx(INITIAL_MASTERY)
        assert rows[0]["started"] is False

    def test_started_chapter_shows_correct_score(self):
        p = _profile({5: 0.75})
        rows = {r["chapter"]: r for r in mastery_summary(p)}
        assert rows[5]["score"] == pytest.approx(0.75)
        assert rows[5]["started"] is True


class TestWeakTopics:
    def test_returns_correct_number(self):
        p = _profile({0: 0.2, 1: 0.9, 2: 0.3})
        topics = weak_topics(p, n=2)
        assert len(topics) == 2

    def test_weakest_first(self):
        p = _profile({0: 0.9, 1: 0.1, 2: 0.5})
        from quiz_app.config import CHAPTER_META
        topics = weak_topics(p, n=3)
        # Ch1 (score 0.1) should be first
        assert CHAPTER_META[1]["name"] in topics[0]

    def test_empty_profile_returns_empty(self):
        assert weak_topics(_profile(), n=3) == []


# ── _chapter_mastery & _chapter_weight ───────────────────────────────────────

class TestChapterMastery:
    def test_unseen_chapter_returns_initial_mastery(self):
        from quiz_app.config import INITIAL_MASTERY
        p = _profile()
        assert _chapter_mastery(p, 99) == pytest.approx(INITIAL_MASTERY)

    def test_seen_chapter_returns_stored_score(self):
        p = _profile({5: 0.8})
        assert _chapter_mastery(p, 5) == pytest.approx(0.8)


class TestChapterWeight:
    def test_non_adaptive_always_1(self):
        p = _profile({0: 0.1})
        assert _chapter_weight(p, 0, adaptive=False) == pytest.approx(1.0)

    def test_unseen_chapter_gets_boost_in_adaptive(self):
        p = _profile()  # no mastery recorded
        w = _chapter_weight(p, 0, adaptive=True)
        assert w == pytest.approx(0.8)

    def test_low_mastery_gets_high_weight(self):
        p = _profile({0: 0.1})
        w = _chapter_weight(p, 0, adaptive=True)
        # max(0.1, 1.2 - 0.1) = 1.1
        assert w == pytest.approx(max(0.1, 1.2 - 0.1))

    def test_high_mastery_gets_low_weight(self):
        p = _profile({0: 1.0})
        w_low = _chapter_weight(p, 0, adaptive=True)
        p2 = _profile({0: 0.1})
        w_high = _chapter_weight(p2, 0, adaptive=True)
        assert w_low < w_high

    def test_weight_never_below_floor(self):
        p = _profile({0: 1.0})
        w = _chapter_weight(p, 0, adaptive=True)
        assert w >= 0.1


# ── Scope.label ───────────────────────────────────────────────────────────────

class TestPhaseFrontier:
    def test_fresh_user_frontier_is_zero(self):
        p = _profile()
        assert _user_phase_frontier(p) == 0

    def test_frontier_follows_most_engaged_phase(self):
        from quiz_app.models import MasteryRecord
        p = _profile()
        # Phase 0 = ch0–3, Phase 3 = ch10–12
        p.mastery[0]  = MasteryRecord(score=0.7, total_seen=5)
        p.mastery[10] = MasteryRecord(score=0.5, total_seen=4)
        # Must have seen ≥3 questions; ch10 is in phase 3
        assert _user_phase_frontier(p) >= 3

    def test_frontier_requires_min_3_seen(self):
        from quiz_app.models import MasteryRecord
        p = _profile()
        p.mastery[10] = MasteryRecord(score=0.8, total_seen=2)  # only 2 seen
        # Ch10 is phase 3 but only 2 seen → doesn't count
        assert _user_phase_frontier(p) == 0


class TestPhaseGateFactor:
    def test_current_phase_no_penalty(self):
        assert _phase_gate_factor(0, 0) == pytest.approx(1.0)   # gap = 0

    def test_one_ahead_no_penalty(self):
        assert _phase_gate_factor(5, 4) == pytest.approx(1.0)   # gap = 1

    def test_two_ahead_half_weight(self):
        assert _phase_gate_factor(10, 1) == pytest.approx(0.5)  # ch10=phase3, frontier=1 → gap=2

    def test_four_ahead_minimal_weight(self):
        # ch20 = phase 5, frontier = 0 → gap = 5 → 0.05
        assert _phase_gate_factor(20, 0) == pytest.approx(0.05)

    def test_behind_frontier_no_penalty(self):
        # gap < 0 → still 1.0
        assert _phase_gate_factor(0, 5) == pytest.approx(1.0)


class TestReviewBoostInChapterWeight:
    def test_due_chapter_gets_boost(self):
        from datetime import date, timedelta
        from quiz_app.models import MasteryRecord
        p = _profile()
        past = (date.today() - timedelta(days=1)).isoformat()
        p.mastery[0] = MasteryRecord(score=0.5, next_review=past)
        w_due    = _chapter_weight(p, 0, adaptive=True, frontier=0)
        # Compare with a non-due chapter at same mastery
        p2 = _profile({0: 0.5})
        w_normal = _chapter_weight(p2, 0, adaptive=True, frontier=0)
        assert w_due > w_normal


class TestMasterySummaryDueField:
    def test_due_field_true_for_due_chapter(self):
        from datetime import date, timedelta
        from quiz_app.models import MasteryRecord
        p = _profile()
        past = (date.today() - timedelta(days=1)).isoformat()
        p.mastery[0] = MasteryRecord(score=0.5, next_review=past)
        rows = {r["chapter"]: r for r in mastery_summary(p)}
        assert rows[0]["due"] is True
        assert rows[1]["due"] is False

    def test_next_review_field_populated(self):
        from datetime import date, timedelta
        from quiz_app.models import MasteryRecord
        p = _profile()
        p.mastery[0] = MasteryRecord(score=0.5, next_review="2026-06-01")
        rows = {r["chapter"]: r for r in mastery_summary(p)}
        assert rows[0]["next_review"] == "2026-06-01"


class TestScopeLabel:
    def test_all_label(self):
        assert Scope.all().label == "All chapters"

    def test_adaptive_label(self):
        assert "Adaptive" in Scope.adaptive().label

    def test_phase_label_includes_number(self):
        assert "3" in Scope.phase(3).label

    def test_chapter_label_includes_number(self):
        assert "10" in Scope.chapter(10).label

    def test_tag_label_includes_tag(self):
        assert "paths" in Scope.tag("paths").label
