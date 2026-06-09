"""Unit tests for quiz_app.ui helpers (no terminal I/O required)."""
import pytest
from quiz_app.ui import _blank_matches, _normalize_blank, progress_bar


# ── _normalize_blank ──────────────────────────────────────────────────────────

class TestNormalizeBlank:
    def test_lowercases(self):
        assert _normalize_blank("INDUCTION") == "induction"

    def test_strips_whitespace(self):
        assert _normalize_blank("  ring  ") == "ring"

    def test_collapses_internal_spaces(self):
        assert _normalize_blank("mathematical  induction") == "mathematical induction"


# ── _blank_matches ────────────────────────────────────────────────────────────

class TestBlankMatches:
    def test_exact_match(self):
        assert _blank_matches("induction", ["induction", "mathematical induction"])

    def test_exact_match_case_insensitive(self):
        assert _blank_matches("INDUCTION", ["induction"])

    def test_suffix_match(self):
        # "induction" is a suffix of "mathematical induction"
        assert _blank_matches("induction", ["mathematical induction"])

    def test_full_phrase_match(self):
        assert _blank_matches("mathematical induction", ["mathematical induction"])

    def test_no_match(self):
        assert not _blank_matches("deduction", ["induction", "mathematical induction"])

    def test_short_raw_not_matched_as_suffix(self):
        # "ion" (3 chars) should NOT match "induction" because len < 4
        assert not _blank_matches("ion", ["induction"])

    def test_substring_of_user_input_not_accepted(self):
        # Old bug: "ring" in "studying ring theory in depth" → True.
        # With the new logic this should be False.
        assert not _blank_matches("studying ring theory in depth", ["ring"])

    def test_empty_raw_returns_false(self):
        # Empty string handled before calling _blank_matches in practice,
        # but the function itself should handle it gracefully.
        assert not _blank_matches("", ["induction"])

    def test_multiple_acceptables(self):
        assert _blank_matches("zero", ["0", "zero", "the additive identity"])

    def test_whitespace_normalization_in_acceptable(self):
        assert _blank_matches("mathematical induction", ["mathematical  induction"])


# ── progress_bar ──────────────────────────────────────────────────────────────

class TestProgressBar:
    def test_full_score(self):
        bar = progress_bar(10, 10)
        assert "10/10" in bar

    def test_zero_score(self):
        bar = progress_bar(0, 10)
        assert "0/10" in bar

    def test_zero_total_no_error(self):
        bar = progress_bar(0, 0)
        assert "0/0" in bar

    def test_contains_block_chars(self):
        bar = progress_bar(5, 10)
        assert "█" in bar or "░" in bar
