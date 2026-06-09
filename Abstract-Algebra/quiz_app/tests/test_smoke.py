"""
Smoke tests for the Abstract Algebra quiz app.
Validates question bank integrity and core adaptive selection.
"""
import pytest
from quiz_app.question_bank import QUESTIONS, build
from quiz_app.config import CHAPTER_META, PHASE_NAMES


class TestQuestionBank:

    @pytest.fixture(scope="class")
    def questions(self):
        return build()

    def test_bank_nonempty(self, questions):
        assert len(questions) >= 50

    def test_all_questions_valid(self, questions):
        failures = [
            f"  ch={q.chapter} id={q.question_id}: {q.validate()}"
            for q in questions if q.validate()
        ]
        assert not failures, "Validation failures:\n" + "\n".join(failures)

    def test_all_chapters_covered(self, questions):
        chapters_with_questions = {q.chapter for q in questions}
        all_chapters = set(CHAPTER_META.keys())
        uncovered = all_chapters - chapters_with_questions
        assert not uncovered, f"Chapters with no questions: {uncovered}"

    def test_all_phases_covered(self, questions):
        phases_with_questions = {q.phase for q in questions}
        all_phases = set(PHASE_NAMES.keys())
        uncovered = all_phases - phases_with_questions
        assert not uncovered, f"Phases with no questions: {uncovered}"

    def test_all_difficulties_present(self, questions):
        diffs = {q.difficulty for q in questions}
        assert diffs == {"beginner", "intermediate", "advanced"}

    def test_mc_answers_in_range(self, questions):
        bad = [
            f"ch={q.chapter}"
            for q in questions
            if q.kind == "mc" and not (0 <= q.answer < len(q.choices))
        ]
        assert not bad, f"MC answer out of range: {bad}"

    def test_tf_has_two_choices(self, questions):
        bad = [q.chapter for q in questions if q.kind == "tf" and len(q.choices) != 2]
        assert not bad, f"TF with wrong choice count in chapters: {bad}"

    def test_blank_has_choices(self, questions):
        bad = [q.chapter for q in questions if q.kind == "blank" and not q.choices]
        assert not bad, f"Blank questions with no choices in chapters: {bad}"

    def test_explanations_nonempty(self, questions):
        bad = [q.chapter for q in questions if not q.explanation.strip()]
        assert not bad, f"Questions with empty explanations in chapters: {bad}"


class TestAdaptiveSelection:

    def test_filter_by_scope_chapter(self):
        from quiz_app.adaptive import filter_by_scope, Scope
        result = filter_by_scope(QUESTIONS, Scope.chapter(0))
        assert all(q.chapter == 0 for q in result)
        assert len(result) > 0

    def test_filter_by_scope_phase(self):
        from quiz_app.adaptive import filter_by_scope, Scope
        result = filter_by_scope(QUESTIONS, Scope.phase(2))
        assert all(q.phase == 2 for q in result)
        assert len(result) > 0

    def test_select_questions_returns_n(self):
        from quiz_app.adaptive import select_questions, Scope
        from quiz_app.models import UserProfile
        import uuid
        profile = UserProfile(user_id=str(uuid.uuid4()), name="Test")
        result = select_questions(QUESTIONS, profile, 5, Scope.all())
        assert len(result) == 5

    def test_select_questions_no_duplicates(self):
        from quiz_app.adaptive import select_questions, Scope
        from quiz_app.models import UserProfile
        import uuid
        profile = UserProfile(user_id=str(uuid.uuid4()), name="Test")
        result = select_questions(QUESTIONS, profile, 20, Scope.all())
        ids = [q.question_id for q in result]
        assert len(ids) == len(set(ids))

    def test_mastery_record_sr_growth(self):
        from quiz_app.models import MasteryRecord
        rec = MasteryRecord()
        assert rec.interval_days == 1
        rec.schedule_review(correct=True)
        assert rec.interval_days == 3
        rec.schedule_review(correct=True)
        assert rec.interval_days == 7
        rec.schedule_review(correct=False)
        assert rec.interval_days == 1

    def test_scope_label_includes_name(self):
        from quiz_app.adaptive import Scope
        label = Scope.chapter(0).label
        assert "Ch.0" in label
        assert CHAPTER_META[0]["name"] in label
