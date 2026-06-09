"""Unit tests for quiz_app.models."""
import pytest
from datetime import date, timedelta
from quiz_app.models import MasteryRecord, Question, SessionResult, UserProfile


# ── Question ──────────────────────────────────────────────────────────────────

def _mc(**kwargs) -> Question:
    defaults = dict(
        chapter=0, phase=0, kind="mc",
        text="What is 2+2?",
        choices=["3", "4", "5", "6"],
        answer=1,
        explanation="2+2=4.",
        tags=["arithmetic"],
        difficulty="beginner",
    )
    defaults.update(kwargs)
    return Question(**defaults)


def _tf(**kwargs) -> Question:
    defaults = dict(
        chapter=0, phase=0, kind="tf",
        text="Is the sky blue?",
        choices=["True", "False"],
        answer=0,
        explanation="Yes.",
        tags=[],
        difficulty="beginner",
    )
    defaults.update(kwargs)
    return Question(**defaults)


def _blank(**kwargs) -> Question:
    defaults = dict(
        chapter=0, phase=0, kind="blank",
        text="___ is the identity element of addition.",
        choices=["zero", "0"],
        answer="zero",
        explanation="0 is the additive identity.",
        tags=[],
        difficulty="beginner",
    )
    defaults.update(kwargs)
    return Question(**defaults)


class TestQuestionValidation:
    def test_valid_mc(self):
        assert _mc().validate() == []

    def test_valid_tf(self):
        assert _tf().validate() == []

    def test_valid_blank(self):
        assert _blank().validate() == []

    def test_mc_empty_text(self):
        errs = _mc(text="   ").validate()
        assert any("text" in e for e in errs)

    def test_mc_empty_explanation(self):
        errs = _mc(explanation="  ").validate()
        assert any("explanation" in e for e in errs)

    def test_mc_answer_out_of_range(self):
        errs = _mc(answer=10).validate()
        assert any("out of range" in e for e in errs)

    def test_mc_answer_must_be_int(self):
        errs = _mc(answer="A").validate()  # type: ignore[arg-type]
        assert any("int" in e for e in errs)

    def test_mc_too_many_choices(self):
        errs = _mc(choices=["a", "b", "c", "d", "e"], answer=0).validate()
        assert any("2-4" in e for e in errs)

    def test_mc_too_few_choices(self):
        errs = _mc(choices=["only one"], answer=0).validate()
        assert any("2-4" in e for e in errs)

    def test_tf_wrong_choice_count(self):
        errs = _tf(choices=["True", "False", "Maybe"]).validate()
        assert any("exactly 2" in e for e in errs)

    def test_blank_no_choices(self):
        errs = _blank(choices=[]).validate()
        assert any("at least one" in e for e in errs)

    def test_blank_answer_must_be_str(self):
        errs = _blank(answer=0).validate()  # type: ignore[arg-type]
        assert any("string" in e for e in errs)

    def test_unknown_difficulty(self):
        errs = _mc(difficulty="expert").validate()
        assert any("difficulty" in e for e in errs)


class TestQuestionSerialization:
    def test_roundtrip_mc(self):
        q = _mc()
        q2 = Question.from_dict(q.to_dict())
        assert q.text == q2.text
        assert q.answer == q2.answer
        assert q.choices == q2.choices
        assert q.difficulty == q2.difficulty

    def test_roundtrip_blank(self):
        q = _blank()
        q2 = Question.from_dict(q.to_dict())
        assert q2.kind == "blank"
        assert q2.answer == "zero"

    def test_from_dict_defaults(self):
        d = {
            "chapter": 1, "phase": 0, "kind": "mc",
            "text": "Q?", "choices": ["A", "B"], "answer": 0,
            "explanation": "E.",
        }
        q = Question.from_dict(d)
        assert q.difficulty == "intermediate"
        assert q.generated is False
        assert q.tags == []


# ── MasteryRecord ─────────────────────────────────────────────────────────────

class TestMasteryRecord:
    def test_accuracy_zero_seen(self):
        rec = MasteryRecord()
        assert rec.accuracy == 0.0

    def test_accuracy(self):
        rec = MasteryRecord(total_seen=4, total_correct=3)
        assert rec.accuracy == pytest.approx(0.75)

    def test_is_due_no_date(self):
        assert MasteryRecord().is_due is False

    def test_is_due_past(self):
        yesterday = (date.today() - timedelta(days=1)).isoformat()
        rec = MasteryRecord(next_review=yesterday)
        assert rec.is_due is True

    def test_is_due_future(self):
        tomorrow = (date.today() + timedelta(days=1)).isoformat()
        rec = MasteryRecord(next_review=tomorrow)
        assert rec.is_due is False

    def test_is_due_today(self):
        rec = MasteryRecord(next_review=date.today().isoformat())
        assert rec.is_due is True

    def test_schedule_review_first_correct(self):
        # SM-2: first recall (rep_count=0, quality=5) → interval=1 day, rep_count=1
        rec = MasteryRecord()
        rec.schedule_review(quality=5)
        expected = (date.today() + timedelta(days=1)).isoformat()
        assert rec.next_review == expected
        assert rec.interval_days == 1
        assert rec.rep_count == 1

    def test_schedule_review_growing_interval(self):
        # SM-2 schedule: rep0→1d, rep1→6d, rep2→round(6*EF), ...
        rec = MasteryRecord()
        rec.schedule_review(quality=5)   # rep_count 0→1, interval=1
        assert rec.interval_days == 1
        rec.schedule_review(quality=5)   # rep_count 1→2, interval=6
        assert rec.interval_days == 6
        # EF is updated before computing interval; compute expected using post-update EF
        ef_after = min(3.0, rec.easiness_factor + 0.1)
        rec.schedule_review(quality=5)   # rep_count 2→3, interval=round(6*ef_after)
        assert rec.interval_days == round(6 * ef_after)

    def test_schedule_review_capped_at_365(self):
        # Set up a record with large interval and high rep_count to trigger cap
        rec = MasteryRecord(interval_days=200, easiness_factor=2.5, rep_count=10)
        rec.schedule_review(quality=5)
        assert rec.interval_days <= 365

    def test_schedule_review_wrong_resets(self):
        rec = MasteryRecord(interval_days=30, rep_count=5)
        rec.schedule_review(quality=1)   # quality < 3 → reset
        assert rec.interval_days == 1
        assert rec.rep_count == 0
        expected = (date.today() + timedelta(days=1)).isoformat()
        assert rec.next_review == expected

    def test_roundtrip(self):
        rec = MasteryRecord(score=0.7, total_seen=10, total_correct=7,
                            last_seen="2025-01-01", next_review="2025-01-08")
        rec2 = MasteryRecord.from_dict(rec.to_dict())
        assert rec2.score == rec.score
        assert rec2.total_seen == rec.total_seen
        assert rec2.next_review == rec.next_review


# ── UserProfile ───────────────────────────────────────────────────────────────

class TestUserProfile:
    def _profile(self) -> UserProfile:
        return UserProfile(user_id="u1", name="Tester")

    def test_record_answer_correct(self):
        p = self._profile()
        new_score = p.record_answer(chapter=0, correct=True, question_id="q1")
        rec = p.mastery[0]
        assert rec.total_seen == 1
        assert rec.total_correct == 1
        assert new_score > 0.30  # started at INITIAL_MASTERY=0.30, moved up

    def test_record_answer_wrong(self):
        p = self._profile()
        new_score = p.record_answer(chapter=0, correct=False, question_id="q1")
        rec = p.mastery[0]
        assert rec.total_seen == 1
        assert rec.total_correct == 0
        assert new_score < 0.30  # moved down from 0.30

    def test_seen_ids_trimmed(self):
        from quiz_app.config import RECENCY_WINDOW
        p = self._profile()
        for i in range(RECENCY_WINDOW + 50):
            p.record_answer(0, True, f"q{i}")
        assert len(p.seen_question_ids) == RECENCY_WINDOW

    def test_chapters_due_for_review(self):
        p = self._profile()
        p.record_answer(0, True, "q0")
        # Force next_review to yesterday
        p.mastery[0].next_review = (date.today() - timedelta(days=1)).isoformat()
        assert 0 in p.chapters_due_for_review()

    def test_weakest_chapters(self):
        p = self._profile()
        p.mastery[0] = MasteryRecord(score=0.2)
        p.mastery[1] = MasteryRecord(score=0.8)
        p.mastery[2] = MasteryRecord(score=0.5)
        weak = p.weakest_chapters(top_n=2)
        assert weak[0] == 0
        assert weak[1] == 2

    def test_total_answered(self):
        p = self._profile()
        p.record_answer(0, True, "q1")
        p.record_answer(0, False, "q2")
        p.record_answer(1, True, "q3")
        assert p.total_answered() == 3

    def test_roundtrip(self):
        p = self._profile()
        p.record_answer(0, True, "q1")
        p.onboarded = True
        p2 = UserProfile.from_dict(p.to_dict())
        assert p2.name == p.name
        assert p2.user_id == p.user_id
        assert p2.mastery[0].total_seen == 1
        assert p2.onboarded is True


# ── SessionResult ─────────────────────────────────────────────────────────────

class TestSessionResult:
    def _result(self, **kwargs) -> SessionResult:
        defaults = dict(
            session_id="s1", timestamp="2025-01-01T00:00:00",
            scope_label="Adaptive", n_questions=10, n_correct=7,
            duration_secs=120.0, wrong_chapters=[2, 3],
        )
        defaults.update(kwargs)
        return SessionResult(**defaults)

    def test_score_pct(self):
        assert self._result(n_questions=10, n_correct=7).score_pct == 70

    def test_score_pct_zero_questions(self):
        assert self._result(n_questions=0, n_correct=0).score_pct == 0

    def test_score_pct_rounds(self):
        assert self._result(n_questions=3, n_correct=2).score_pct == 67

    def test_roundtrip(self):
        r = self._result()
        r2 = SessionResult.from_dict(r.to_dict())
        assert r2.session_id == r.session_id
        assert r2.n_questions == r.n_questions
        assert r2.n_correct == r.n_correct
        assert r2.wrong_chapters == r.wrong_chapters

    def test_from_dict_missing_fields(self):
        r = SessionResult.from_dict({})
        assert r.n_questions == 0
        assert r.wrong_chapters == []
