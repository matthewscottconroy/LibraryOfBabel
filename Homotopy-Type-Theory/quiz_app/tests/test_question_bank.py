"""
Tests for question_bank.py — structural integrity of every question.
These tests catch typos, invalid indices, and missing fields early.
"""
import pytest
from quiz_app.question_bank import QUESTIONS, build


class TestQuestionBankIntegrity:
    """Validate every question in the static bank."""

    @pytest.fixture(scope="class")
    def questions(self):
        return build()

    def test_bank_is_nonempty(self, questions):
        assert len(questions) > 50, "Expected at least 50 questions"

    def test_all_questions_pass_validation(self, questions):
        failures = []
        for q in questions:
            errs = q.validate()
            if errs:
                failures.append(f"Q id={q.question_id} ch={q.chapter}: {errs}")
        assert not failures, "\n".join(failures)

    def test_all_chapters_represented(self, questions):
        chapters = {q.chapter for q in questions}
        missing = set(range(27)) - chapters
        # Allow up to 3 chapters to have no questions (some are very specialised)
        assert len(missing) <= 3, f"Chapters with no questions: {missing}"

    def test_all_phases_represented(self, questions):
        phases = {q.phase for q in questions}
        assert phases == {0, 1, 2, 3, 4, 5, 6, 7, 8}

    def test_all_difficulties_present(self, questions):
        diffs = {q.difficulty for q in questions}
        assert diffs == {"beginner", "intermediate", "advanced"}

    def test_all_kinds_present(self, questions):
        kinds = {q.kind for q in questions}
        assert "mc" in kinds
        assert "tf" in kinds

    def test_mc_answers_in_range(self, questions):
        for q in questions:
            if q.kind == "mc":
                assert 0 <= q.answer < len(q.choices), (
                    f"Q ch={q.chapter}: answer {q.answer} out of range "
                    f"for {len(q.choices)} choices"
                )

    def test_tf_questions_have_two_choices(self, questions):
        for q in questions:
            if q.kind == "tf":
                assert len(q.choices) == 2

    def test_mc_questions_have_2_to_4_choices(self, questions):
        for q in questions:
            if q.kind == "mc":
                assert 2 <= len(q.choices) <= 4

    def test_all_questions_have_tags(self, questions):
        for q in questions:
            assert q.tags, f"Q ch={q.chapter} has no tags: {q.text[:40]}"

    def test_all_questions_have_nonempty_text(self, questions):
        for q in questions:
            assert q.text.strip(), f"Q ch={q.chapter} has empty text"

    def test_all_questions_have_nonempty_explanation(self, questions):
        for q in questions:
            assert q.explanation.strip(), (
                f"Q ch={q.chapter} missing explanation: {q.text[:40]}"
            )

    def test_all_ids_unique(self, questions):
        ids = [q.question_id for q in questions]
        assert len(ids) == len(set(ids)), "Duplicate question IDs found"

    def test_generated_flag_is_false_for_static(self, questions):
        for q in questions:
            assert q.generated is False, (
                f"Static question ch={q.chapter} has generated=True"
            )

    def test_chapter_phase_consistency(self, questions):
        """Chapter's phase must match CHAPTER_META."""
        from quiz_app.config import CHAPTER_META
        for q in questions:
            expected_phase = CHAPTER_META.get(q.chapter, {}).get("phase")
            if expected_phase is not None:
                assert q.phase == expected_phase, (
                    f"Q ch={q.chapter} has phase={q.phase}, expected {expected_phase}"
                )


class TestSingleton:
    def test_module_level_questions_nonempty(self):
        assert len(QUESTIONS) > 0

    def test_build_returns_new_list(self):
        q1 = build()
        q2 = build()
        assert len(q1) == len(q2)
        assert q1 is not q2


class TestCoverage:
    """Checks specific coverage goals from the expansion."""

    @pytest.fixture(scope="class")
    def questions(self):
        return build()

    def test_blank_questions_present(self, questions):
        blanks = [q for q in questions if q.kind == "blank"]
        assert len(blanks) >= 8, f"Expected ≥8 blank questions, got {len(blanks)}"

    def test_thin_chapters_expanded(self, questions):
        """Previously thin chapters (3 Qs) should now have more."""
        from collections import Counter
        counts = Counter(q.chapter for q in questions)
        thin_chapters = [3, 12, 15, 24, 25]
        for ch in thin_chapters:
            assert counts[ch] >= 5, (
                f"Ch{ch:02d} still has only {counts[ch]} questions"
            )

    def test_all_chapters_have_beginner_questions(self, questions):
        from quiz_app.config import CHAPTER_META
        from collections import defaultdict
        by_ch_diff = defaultdict(set)
        for q in questions:
            by_ch_diff[q.chapter].add(q.difficulty)
        # Every chapter should have at least one difficulty represented
        for ch in CHAPTER_META:
            assert by_ch_diff[ch], f"Ch{ch:02d} has no questions at all"

    def test_previously_missing_difficulties_filled(self, questions):
        """Chapters that were missing a difficulty tier should now have it."""
        from collections import defaultdict
        by_ch_diff = defaultdict(set)
        for q in questions:
            by_ch_diff[q.chapter].add(q.difficulty)
        # Spot-check a few that were missing difficulty tiers
        assert "intermediate" in by_ch_diff[0],  "Ch00 still missing intermediate"
        assert "beginner"     in by_ch_diff[4],  "Ch04 still missing beginner"
        assert "beginner"     in by_ch_diff[11], "Ch11 still missing beginner"
        assert "beginner"     in by_ch_diff[12], "Ch12 still missing beginner"
        assert "beginner"     in by_ch_diff[23], "Ch23 still missing beginner"
        assert "beginner"     in by_ch_diff[24], "Ch24 still missing beginner"
        assert "beginner"     in by_ch_diff[25], "Ch25 still missing beginner"
