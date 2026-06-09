"""
Smoke tests for the CSSB quiz app.
Validates question bank integrity, blank matching, mastery consistency,
and core adaptive selection.
"""
import pytest
from quiz_app.question_bank import QUESTIONS, build
from quiz_app.config import CHAPTER_META, PHASE_NAMES, INITIAL_MASTERY, DIFF_BEGINNER_MAX


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
        result = filter_by_scope(QUESTIONS, Scope.phase(1))
        assert all(q.phase == 1 for q in result)
        assert len(result) > 0

    def test_filter_by_scope_tag(self):
        from quiz_app.adaptive import filter_by_scope, Scope
        result = filter_by_scope(QUESTIONS, Scope.tag("crispr"))
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
        rec.schedule_review(correct=True)
        assert rec.interval_days == 14
        rec.schedule_review(correct=False)
        assert rec.interval_days == 1

    def test_scope_label_includes_name(self):
        from quiz_app.adaptive import Scope
        label = Scope.chapter(10).label
        assert "Ch.10" in label
        assert CHAPTER_META[10]["name"] in label

    def test_phase_scope_label(self):
        from quiz_app.adaptive import Scope
        label = Scope.phase(1).label
        assert "Bioinformatics" in label


class TestBlankMatching:
    """Tests for the fill-in-the-blank answer matching logic."""

    def setup_method(self):
        from quiz_app.ui import _match_blank
        self.match = _match_blank

    # ── Should match ──────────────────────────────────────────────────────────

    def test_exact_match(self):
        assert self.match("Hill equation", ["Hill equation"]) is True

    def test_case_insensitive(self):
        assert self.match("aggagg", ["AGGAGG"]) is True

    def test_normalized_hyphen(self):
        assert self.match("Michaelis-Menten", ["michaelis-menten", "MM"]) is True

    def test_article_prefix_multi_word(self):
        # "the Hill equation" should match "Hill equation" (1 extra word, multi-word answer)
        assert self.match("the Hill equation", ["Hill equation"]) is True

    def test_synonym_accepted(self):
        assert self.match("JC69", ["Jukes-Cantor", "JC69"]) is True

    def test_multi_word_extra_words(self):
        # "the negative binomial distribution" should match "negative binomial"
        assert self.match("the negative binomial distribution", ["negative binomial"]) is True

    def test_whitespace_normalized(self):
        assert self.match("negative  binomial", ["negative binomial"]) is True

    # ── Should NOT match ──────────────────────────────────────────────────────

    def test_substring_not_accepted(self):
        # "me" is a substring of "membrane" but must NOT match
        assert self.match("me", ["membrane"]) is False

    def test_superset_single_word_not_accepted(self):
        # "membrane potential" must NOT match single-word answer "membrane"
        assert self.match("membrane potential", ["membrane"]) is False

    def test_completely_wrong(self):
        assert self.match("photosynthesis", ["Michaelis-Menten"]) is False

    def test_empty_input_not_accepted(self):
        assert self.match("", ["translation"]) is False

    def test_too_many_extra_words_not_accepted(self):
        # 3 extra words beyond a 1-token answer — should not match
        assert self.match("the wonderful membrane protein", ["membrane"]) is False


class TestMasteryConsistency:
    """Verifies that INITIAL_MASTERY is consistent across the system."""

    def test_initial_mastery_below_beginner_threshold(self):
        # New users must see beginner questions, not intermediate
        assert INITIAL_MASTERY < DIFF_BEGINNER_MAX, (
            f"INITIAL_MASTERY ({INITIAL_MASTERY}) must be < DIFF_BEGINNER_MAX "
            f"({DIFF_BEGINNER_MAX}) so new users receive beginner questions"
        )

    def test_chapter_mastery_default_uses_initial(self):
        from quiz_app.adaptive import _chapter_mastery
        from quiz_app.models import UserProfile
        import uuid
        profile = UserProfile(user_id=str(uuid.uuid4()), name="Test")
        # Chapter 0 has never been seen — should return INITIAL_MASTERY, not 0.5
        assert _chapter_mastery(profile, 0) == INITIAL_MASTERY

    def test_mastery_summary_default_uses_initial(self):
        from quiz_app.adaptive import mastery_summary
        from quiz_app.models import UserProfile
        import uuid
        profile = UserProfile(user_id=str(uuid.uuid4()), name="Test")
        rows = mastery_summary(profile)
        unstarted = [r for r in rows if not r["started"]]
        assert all(r["score"] == INITIAL_MASTERY for r in unstarted), (
            "mastery_summary should use INITIAL_MASTERY for unstarted chapters"
        )

    def test_mastery_record_initial_score(self):
        from quiz_app.models import MasteryRecord
        rec = MasteryRecord()
        assert rec.score == INITIAL_MASTERY, (
            f"MasteryRecord default score ({rec.score}) must equal INITIAL_MASTERY ({INITIAL_MASTERY})"
        )
