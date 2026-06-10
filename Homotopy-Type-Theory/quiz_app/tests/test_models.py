"""Tests for models.py — Question, MasteryRecord, UserProfile."""
from datetime import date, timedelta

import pytest
from quiz_app.models import MasteryRecord, Question, UserProfile


# ── Question ──────────────────────────────────────────────────────────────────

class TestQuestion:
    def _mc(self, **kw) -> Question:
        defaults = dict(
            chapter=0, phase=0, kind="mc",
            text="What is 2+2?",
            choices=["3", "4", "5", "6"],
            answer=1,
            explanation="2+2=4.",
            tags=["math"],
            difficulty="beginner",
        )
        defaults.update(kw)
        return Question(**defaults)

    def test_valid_mc_passes_validation(self):
        assert self._mc().validate() == []

    def test_auto_question_id(self):
        q = self._mc()
        assert q.question_id != ""

    def test_unique_ids(self):
        assert self._mc().question_id != self._mc().question_id

    def test_invalid_answer_out_of_range(self):
        q = self._mc(answer=10)
        errs = q.validate()
        assert any("index" in e for e in errs)

    def test_tf_must_have_two_choices(self):
        q = Question(
            chapter=0, phase=0, kind="tf",
            text="Is 2+2=4?", choices=["True", "False", "Maybe"],
            answer=0, explanation="Yes.", tags=[], difficulty="beginner",
        )
        errs = q.validate()
        assert any("2 choices" in e for e in errs)

    def test_blank_answer_must_be_string(self):
        q = Question(
            chapter=0, phase=0, kind="blank",
            text="The ___ is a type.",
            choices=["interval"],
            answer="interval",
            explanation="The interval is a type.",
            tags=[], difficulty="beginner",
        )
        assert q.validate() == []

    def test_unknown_difficulty_flagged(self):
        q = self._mc(difficulty="expert")
        errs = q.validate()
        assert any("difficulty" in e for e in errs)

    def test_empty_explanation_flagged(self):
        q = self._mc(explanation="   ")
        errs = q.validate()
        assert any("explanation" in e for e in errs)

    def test_round_trip_serialisation(self):
        q = self._mc()
        q2 = Question.from_dict(q.to_dict())
        assert q2.text == q.text
        assert q2.choices == q.choices
        assert q2.answer == q.answer
        assert q2.difficulty == q.difficulty

    def test_from_dict_preserves_question_id(self):
        q = self._mc()
        q2 = Question.from_dict(q.to_dict())
        assert q2.question_id == q.question_id


# ── MasteryRecord ─────────────────────────────────────────────────────────────

class TestMasteryRecord:
    def test_initial_accuracy_is_zero(self):
        assert MasteryRecord().accuracy == 0.0

    def test_accuracy_after_answers(self):
        rec = MasteryRecord(total_seen=4, total_correct=3)
        assert rec.accuracy == pytest.approx(0.75)

    def test_round_trip(self):
        rec = MasteryRecord(score=0.7, total_seen=10, total_correct=8)
        rec2 = MasteryRecord.from_dict(rec.to_dict())
        assert rec2.score == rec.score
        assert rec2.total_seen == rec.total_seen

    def test_round_trip_preserves_next_review(self):
        rec = MasteryRecord(next_review="2026-05-10")
        rec2 = MasteryRecord.from_dict(rec.to_dict())
        assert rec2.next_review == "2026-05-10"

    def test_is_due_false_when_empty(self):
        assert MasteryRecord().is_due is False

    def test_is_due_false_for_future_date(self):
        future = (date.today() + timedelta(days=7)).isoformat()
        assert MasteryRecord(next_review=future).is_due is False

    def test_is_due_true_for_past_date(self):
        past = (date.today() - timedelta(days=1)).isoformat()
        assert MasteryRecord(next_review=past).is_due is True

    def test_is_due_true_for_today(self):
        today = date.today().isoformat()
        assert MasteryRecord(next_review=today).is_due is True

    def test_schedule_review_sets_future_date(self):
        rec = MasteryRecord(score=0.8)
        rec.schedule_review(correct=True)
        assert rec.next_review != ""
        assert rec.next_review >= date.today().isoformat()

    def test_schedule_review_wrong_gives_tomorrow(self):
        rec = MasteryRecord(score=0.5)
        rec.schedule_review(correct=False)
        tomorrow = (date.today() + timedelta(days=1)).isoformat()
        assert rec.next_review == tomorrow

    def test_review_interval_wrong_resets_to_1(self):
        rec = MasteryRecord()
        rec.schedule_review(correct=True)
        assert rec.interval_days > 1
        rec.schedule_review(correct=False)
        assert rec.interval_days == 1

    def test_review_interval_grows_on_correct(self):
        rec = MasteryRecord()
        prev = rec.interval_days
        rec.schedule_review(correct=True)
        assert rec.interval_days > prev


# ── UserProfile ───────────────────────────────────────────────────────────────

class TestUserProfile:
    def _profile(self) -> UserProfile:
        return UserProfile(user_id="u1", name="Alice")

    def test_initial_total_answered_zero(self):
        assert self._profile().total_answered() == 0

    def test_record_correct_increases_mastery(self):
        p = self._profile()
        before = p.get_mastery(0).score
        p.record_answer(0, True, "q1")
        assert p.get_mastery(0).score > before

    def test_record_wrong_decreases_mastery(self):
        p = self._profile()
        p.mastery[0] = MasteryRecord(score=0.8)
        before = p.mastery[0].score
        p.record_answer(0, False, "q1")
        assert p.mastery[0].score < before

    def test_mastery_stays_in_bounds(self):
        p = self._profile()
        for _ in range(50):
            p.record_answer(0, True, "q")
        assert p.mastery[0].score <= 1.0

        q = self._profile()
        q.mastery[0] = MasteryRecord(score=0.01)
        for _ in range(50):
            q.record_answer(0, False, "q")
        assert q.mastery[0].score >= 0.0

    def test_seen_ids_trimmed_to_200(self):
        p = self._profile()
        for i in range(250):
            p.record_answer(0, True, f"q{i}")
        assert len(p.seen_question_ids) <= 200

    def test_weakest_chapters_sorted(self):
        p = self._profile()
        p.mastery[0] = MasteryRecord(score=0.3)
        p.mastery[1] = MasteryRecord(score=0.8)
        p.mastery[2] = MasteryRecord(score=0.1)
        weakest = p.weakest_chapters(top_n=2)
        assert weakest == [2, 0]

    def test_round_trip_serialisation(self):
        p = self._profile()
        p.record_answer(5, True, "abc")
        p2 = UserProfile.from_dict(p.to_dict())
        assert p2.name == p.name
        assert p2.mastery[5].score == pytest.approx(p.mastery[5].score)
        assert p2.seen_question_ids == p.seen_question_ids

    def test_total_answered_sums_across_chapters(self):
        p = self._profile()
        p.record_answer(0, True, "q1")
        p.record_answer(0, False, "q2")
        p.record_answer(1, True, "q3")
        assert p.total_answered() == 3

    def test_weakest_chapters_empty_when_no_mastery(self):
        p = self._profile()
        assert p.weakest_chapters() == []

    def test_record_answer_appends_question_id(self):
        p = self._profile()
        p.record_answer(0, True, "abc-123")
        assert "abc-123" in p.seen_question_ids

    def test_get_mastery_creates_record_for_new_chapter(self):
        p = self._profile()
        rec = p.get_mastery(99)
        assert rec is not None
        from quiz_app.config import INITIAL_MASTERY
        assert rec.score == pytest.approx(INITIAL_MASTERY)

    def test_mastery_score_update_formula_correct(self):
        p = self._profile()
        p.mastery[0] = MasteryRecord(score=0.5)
        new = p.record_answer(0, True, "q")
        assert new == pytest.approx(0.5 + 0.15 * (1.0 - 0.5))

    def test_wrong_answer_formula_correct(self):
        p = self._profile()
        p.mastery[0] = MasteryRecord(score=0.8)
        new = p.record_answer(0, False, "q")
        assert new == pytest.approx(0.8 - 0.10 * 0.8)

    def test_record_answer_sets_next_review(self):
        p = self._profile()
        p.record_answer(0, True, "q")
        assert p.mastery[0].next_review != ""

    def test_chapters_due_for_review_empty_initially(self):
        p = self._profile()
        assert p.chapters_due_for_review() == []

    def test_chapters_due_for_review_returns_due_chapters(self):
        from datetime import date, timedelta
        p = self._profile()
        past = (date.today() - timedelta(days=1)).isoformat()
        p.mastery[5] = MasteryRecord(score=0.5, next_review=past)
        assert 5 in p.chapters_due_for_review()

    def test_onboarded_field_defaults_false(self):
        p = self._profile()
        assert p.onboarded is False

    def test_onboarded_survives_round_trip(self):
        p = self._profile()
        p.onboarded = True
        p2 = UserProfile.from_dict(p.to_dict())
        assert p2.onboarded is True


# ── SessionResult ─────────────────────────────────────────────────────────────

from quiz_app.models import SessionResult

class TestSessionResult:
    def _result(self, n=10, correct=7) -> SessionResult:
        return SessionResult(
            session_id="s1", timestamp="2026-01-01T00:00:00+00:00",
            scope_label="All", n_questions=n, n_correct=correct,
            duration_secs=30.0, wrong_chapters=[],
        )

    def test_score_pct_normal(self):
        assert self._result(10, 7).score_pct == 70

    def test_score_pct_zero_questions(self):
        assert self._result(0, 0).score_pct == 0

    def test_score_pct_perfect(self):
        assert self._result(5, 5).score_pct == 100

    def test_to_dict_roundtrip_keys(self):
        d = self._result().to_dict()
        expected = {"session_id", "timestamp", "scope_label",
                    "n_questions", "n_correct", "duration_secs", "wrong_chapters",
                    "per_difficulty", "streak_max", "n_generated"}
        assert set(d.keys()) == expected
