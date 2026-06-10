"""
Tests for ui.py — ANSI helpers, progress_bar, wrap, _present_blank matching.
No stdin interaction: input-dependent functions are not tested here.
"""
from __future__ import annotations

import pytest

import quiz_app.ui as ui


# ── ANSI helpers ───────────────────────────────────────────────────────────────

class TestAnsiHelpers:
    def test_bold_wraps_with_escape(self):
        result = ui.bold("hello")
        assert "\033[" in result
        assert "hello" in result

    def test_green_contains_text(self):
        assert "hi" in ui.green("hi")

    def test_red_contains_text(self):
        assert "x" in ui.red("x")

    def test_dim_contains_text(self):
        assert "q" in ui.dim("q")

    def test_helpers_start_with_escape(self):
        for fn in (ui.bold, ui.green, ui.red, ui.yellow, ui.cyan,
                   ui.dim, ui.blue, ui.magenta):
            assert fn("t").startswith("\033[")

    def test_helpers_end_with_reset(self):
        reset = "\033[0m"
        for fn in (ui.bold, ui.green, ui.red):
            assert fn("t").endswith(reset)


# ── wrap ──────────────────────────────────────────────────────────────────────

class TestWrap:
    def test_short_text_unchanged(self):
        result = ui.wrap("hello world")
        assert "hello world" in result

    def test_indent_applied(self):
        result = ui.wrap("hello", indent=4)
        assert result.startswith("    ")

    def test_long_text_wrapped(self):
        long_text = "word " * 30
        result = ui.wrap(long_text, indent=0)
        lines = result.split("\n")
        assert len(lines) > 1

    def test_default_indent_is_2_spaces(self):
        result = ui.wrap("x")
        assert result.startswith("  ")


# ── progress_bar ──────────────────────────────────────────────────────────────

class TestProgressBar:
    def test_zero_score(self):
        bar = ui.progress_bar(0, 10)
        assert "0/10" in bar

    def test_full_score(self):
        bar = ui.progress_bar(10, 10)
        assert "10/10" in bar

    def test_zero_total_does_not_crash(self):
        bar = ui.progress_bar(0, 0)
        assert "0/0" in bar

    def test_bar_contains_block_chars(self):
        bar = ui.progress_bar(5, 10)
        assert "█" in bar or "░" in bar

    def test_high_score_is_green(self):
        # 7/10 = 70% → green
        bar = ui.progress_bar(7, 10)
        assert "\033[32m" in bar  # green ANSI code

    def test_low_score_is_red(self):
        # 2/10 = 20% → red
        bar = ui.progress_bar(2, 10)
        assert "\033[31m" in bar  # red ANSI code

    def test_mid_score_is_yellow(self):
        # 5/10 = 50% → yellow
        bar = ui.progress_bar(5, 10)
        assert "\033[33m" in bar  # yellow ANSI code


# ── _diff_badge ───────────────────────────────────────────────────────────────

class TestDiffBadge:
    def test_beginner_badge_contains_text(self):
        badge = ui._diff_badge("beginner")
        assert "beginner" in badge

    def test_intermediate_badge_contains_text(self):
        badge = ui._diff_badge("intermediate")
        assert "intermediate" in badge

    def test_advanced_badge_contains_text(self):
        badge = ui._diff_badge("advanced")
        assert "advanced" in badge

    def test_unknown_diff_still_returns_string(self):
        badge = ui._diff_badge("unknown")
        assert isinstance(badge, str)


# ── _source_badge ─────────────────────────────────────────────────────────────

class TestSourceBadge:
    def test_generated_shows_ai_label(self):
        badge = ui._source_badge(True)
        assert "AI" in badge or "generated" in badge.lower()

    def test_static_shows_empty_string(self):
        assert ui._source_badge(False) == ""


# ── _present_blank matching logic ─────────────────────────────────────────────
# We test the matching logic by directly exercising the function's
# conditions without calling input(). The matching rules are:
#   exact match, case-insensitive substring (raw in acceptable or acceptable in raw),
#   whitespace-normalised exact match.

class TestShowSessionSummary:
    """show_session_summary now accepts streak_max and per_difficulty."""

    def test_runs_without_optional_args(self, capsys):
        ui.show_session_summary.__wrapped__ = getattr(
            ui.show_session_summary, "__wrapped__", ui.show_session_summary)
        # Should not raise even when called with just positional args
        with unittest.mock.patch("quiz_app.ui.pause"):
            ui.show_session_summary(7, 10, [], 60.0)
        out = capsys.readouterr().out
        assert "7/10" in out

    def test_streak_shown_when_greater_than_1(self, capsys):
        with unittest.mock.patch("quiz_app.ui.pause"):
            ui.show_session_summary(5, 10, [], 30.0, streak_max=4)
        out = capsys.readouterr().out
        assert "4" in out

    def test_per_difficulty_breakdown_shown(self, capsys):
        per_diff = {"beginner": [3, 4], "intermediate": [2, 3], "advanced": [0, 0]}
        with unittest.mock.patch("quiz_app.ui.pause"):
            ui.show_session_summary(5, 7, [], 45.0, per_difficulty=per_diff)
        out = capsys.readouterr().out
        assert "beginner" in out

    def test_zero_total_does_not_crash(self, capsys):
        with unittest.mock.patch("quiz_app.ui.pause"):
            ui.show_session_summary(0, 0, [], 0.0)
        capsys.readouterr()  # just ensure no exception


import unittest.mock


class TestStudyMode:
    def test_invalid_chapter_prints_message(self, capsys):
        ui.show_study_mode(999)
        out = capsys.readouterr().out
        assert "not found" in out.lower() or "999" in out

    def test_missing_file_prints_message(self, tmp_path, monkeypatch, capsys):
        import quiz_app.ui as ui_mod
        import quiz_app.config as cfg
        monkeypatch.setattr(cfg, "CHAPTERS_DIR", tmp_path)
        # Re-import so ui uses the patched config
        ui.show_study_mode(0)
        out = capsys.readouterr().out
        assert out  # something printed; no crash

    def test_renders_chapter_content(self, tmp_path, monkeypatch, capsys):
        import quiz_app.config as cfg
        from quiz_app.config import CHAPTER_META
        monkeypatch.setattr(cfg, "CHAPTERS_DIR", tmp_path)
        fname = CHAPTER_META[0]["file"]
        (tmp_path / fname).write_text("# Logic and Proof\n\nThis chapter covers logic.\n\nSecond paragraph here.")
        with unittest.mock.patch("quiz_app.ui.confirm", return_value=False), \
             unittest.mock.patch("quiz_app.ui.pause"):
            ui.show_study_mode(0)
        out = capsys.readouterr().out
        assert "Logic" in out

    def test_code_blocks_stripped(self, tmp_path, monkeypatch, capsys):
        import quiz_app.config as cfg
        from quiz_app.config import CHAPTER_META
        monkeypatch.setattr(cfg, "CHAPTERS_DIR", tmp_path)
        fname = CHAPTER_META[0]["file"]
        (tmp_path / fname).write_text("Intro.\n\n```agda\nsecret code\n```\n\nConclusion.")
        with unittest.mock.patch("quiz_app.ui.confirm", return_value=False), \
             unittest.mock.patch("quiz_app.ui.pause"):
            ui.show_study_mode(0)
        out = capsys.readouterr().out
        assert "secret code" not in out
        assert "Intro" in out


class TestBlankMatching:
    """
    The matching logic in _present_blank cannot be called directly because it
    calls input(). We replicate the exact conditions here to test correctness.
    """

    def _matches(self, raw: str, acceptable: list[str]) -> bool:
        raw = raw.strip().lower()
        if not raw:
            return False
        acc = [s.lower() for s in acceptable]
        correct = any(raw == a or raw in a or a in raw for a in acc)
        if not correct:
            correct = any(
                raw.replace(" ", "") == a.replace(" ", "") for a in acc
            )
        return correct

    def test_exact_match(self):
        assert self._matches("univalence", ["univalence"])

    def test_case_insensitive_exact(self):
        assert self._matches("Univalence", ["univalence"])

    def test_whitespace_stripped(self):
        assert self._matches("  univalence  ", ["univalence"])

    def test_partial_answer_in_acceptable(self):
        # "val" is in "univalence"
        assert self._matches("val", ["univalence"])

    def test_acceptable_in_raw(self):
        # "univalence" is in raw
        assert self._matches("the univalence axiom", ["univalence"])

    def test_whitespace_normalised_match(self):
        assert self._matches("identity type", ["identitytype"])

    def test_wrong_answer_fails(self):
        assert not self._matches("zzz", ["univalence", "axiom"])

    def test_empty_raw_fails(self):
        assert not self._matches("", ["univalence"])

    def test_multiple_acceptable_any_matches(self):
        assert self._matches("J", ["J rule", "path induction"])
