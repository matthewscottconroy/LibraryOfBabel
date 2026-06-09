"""Unit tests for quiz_app.generator (cache ops and response parsing)."""
import json
import pytest
from pathlib import Path
from unittest.mock import patch

from quiz_app import base_generator as gen_module
from quiz_app.base_generator import (
    _add_to_cache,
    _load_cache,
    _parse_and_validate,
    _pop_from_cache,
    _save_cache,
)
from quiz_app.generator import ClaudeGenerator


# ── Cache helpers ─────────────────────────────────────────────────────────────

@pytest.fixture(autouse=True)
def _tmp_cache(tmp_path, monkeypatch):
    monkeypatch.setattr(gen_module, "CACHE_DIR", tmp_path)
    return tmp_path


class TestCache:
    def _item(self, text="Q?"):
        return {
            "question_id": "id1", "chapter": 0, "phase": 0, "kind": "mc",
            "text": text,
            "choices": ["A", "B", "C", "D"], "answer": 0,
            "explanation": "Exp.", "tags": [], "difficulty": "beginner",
            "generated": True,
        }

    def test_load_empty(self):
        assert _load_cache(0, "beginner") == []

    def test_add_and_load(self):
        item = self._item()
        _add_to_cache(0, "beginner", item)
        items = _load_cache(0, "beginner")
        assert len(items) == 1
        assert items[0]["text"] == "Q?"

    def test_pop_fifo(self):
        _add_to_cache(0, "beginner", self._item("First"))
        _add_to_cache(0, "beginner", self._item("Second"))
        first = _pop_from_cache(0, "beginner")
        assert first["text"] == "First"
        second = _pop_from_cache(0, "beginner")
        assert second["text"] == "Second"
        assert _pop_from_cache(0, "beginner") is None

    def test_pop_empty(self):
        assert _pop_from_cache(0, "beginner") is None

    def test_cap_respected(self):
        from quiz_app.config import CACHE_CAP_PER_DIFFICULTY
        for i in range(CACHE_CAP_PER_DIFFICULTY + 5):
            _add_to_cache(0, "beginner", self._item(f"Q{i}"))
        items = _load_cache(0, "beginner")
        assert len(items) == CACHE_CAP_PER_DIFFICULTY

    def test_separate_by_difficulty(self):
        _add_to_cache(0, "beginner", self._item("Beginner Q"))
        _add_to_cache(0, "advanced", self._item("Advanced Q"))
        assert _load_cache(0, "beginner")[0]["text"] == "Beginner Q"
        assert _load_cache(0, "advanced")[0]["text"] == "Advanced Q"

    def test_separate_by_chapter(self):
        _add_to_cache(0, "intermediate", self._item("Ch0"))
        _add_to_cache(1, "intermediate", self._item("Ch1"))
        assert len(_load_cache(0, "intermediate")) == 1
        assert len(_load_cache(1, "intermediate")) == 1


# ── _parse_and_validate ───────────────────────────────────────────────────────

class TestParseAndValidate:
    def _valid_json(self, **overrides):
        d = {
            "text": "What is a group?",
            "choices": ["A set with a binary op", "A field", "A ring", "A module"],
            "answer": 0,
            "explanation": "A group is a set with a binary operation satisfying four axioms.",
            "tags": ["groups"],
        }
        d.update(overrides)
        return json.dumps(d)

    def test_valid_response(self):
        q = _parse_and_validate(self._valid_json(), chapter=8, difficulty="beginner")
        assert q is not None
        assert q.text == "What is a group?"
        assert q.generated is True
        assert q.kind == "mc"

    def test_strips_markdown_fences(self):
        raw = "```json\n" + self._valid_json() + "\n```"
        q = _parse_and_validate(raw, chapter=8, difficulty="beginner")
        assert q is not None

    def test_invalid_json_returns_none(self):
        assert _parse_and_validate("{not json}", chapter=0, difficulty="beginner") is None

    def test_missing_required_field(self):
        d = {"text": "Q?", "choices": ["A", "B", "C", "D"], "answer": 0}
        # missing "explanation"
        assert _parse_and_validate(json.dumps(d), chapter=0, difficulty="beginner") is None

    def test_invalid_answer_index(self):
        q = _parse_and_validate(self._valid_json(answer=99), chapter=0, difficulty="beginner")
        assert q is None

    def test_too_few_choices(self):
        q = _parse_and_validate(
            self._valid_json(choices=["Only one"], answer=0),
            chapter=0, difficulty="beginner",
        )
        assert q is None

    def test_answer_set_correctly(self):
        q = _parse_and_validate(self._valid_json(answer=2), chapter=8, difficulty="intermediate")
        assert q is not None
        assert q.answer == 2

    def test_chapter_and_difficulty_propagated(self):
        q = _parse_and_validate(self._valid_json(), chapter=5, difficulty="advanced")
        assert q is not None
        assert q.chapter == 5
        assert q.difficulty == "advanced"


# ── ClaudeGenerator ───────────────────────────────────────────────────────────

class TestClaudeGeneratorAvailability:
    def test_no_api_key_not_available(self):
        g = ClaudeGenerator(api_key="")
        assert g.available is False

    def test_no_anthropic_module_not_available(self, monkeypatch):
        monkeypatch.setattr(gen_module, "_anthropic_module", None)  # base_generator module
        g = ClaudeGenerator(api_key="sk-test")
        assert g.available is False

    def test_get_question_returns_none_when_unavailable(self):
        g = ClaudeGenerator(api_key="")
        result = g.get_question(0, "beginner", 50, [])
        assert result is None

    def test_get_question_from_cache(self):
        """get_question should serve from cache without hitting the API."""
        item = {
            "question_id": "cached-id", "chapter": 0, "phase": 0, "kind": "mc",
            "text": "Cached question?",
            "choices": ["A", "B", "C", "D"], "answer": 0,
            "explanation": "Because.", "tags": [], "difficulty": "beginner",
            "generated": True,
        }
        _add_to_cache(0, "beginner", item)

        g = ClaudeGenerator(api_key="")  # no API key — can't generate
        q = g.get_question(0, "beginner", 50, [])
        assert q is not None
        assert q.text == "Cached question?"
        assert q.generated is True
