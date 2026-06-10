"""
Tests for the question generator — Claude API integration (mocked).
These tests never make real API calls.
Cache/path operations are patched on quiz_app.base_generator where they live.
"""
import json
import uuid
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

import quiz_app.base_generator as base_gen_mod
from quiz_app.generator import (
    ClaudeGenerator,
    _add_to_cache,
    _load_cache,
    _parse_and_validate,
    _pop_from_cache,
    _save_cache,
)
from quiz_app.models import Question


# ── Fixtures ──────────────────────────────────────────────────────────────────

@pytest.fixture(autouse=True)
def temp_cache_dir(tmp_path, monkeypatch):
    """Redirect cache operations to a temp directory."""
    monkeypatch.setattr(base_gen_mod, "CACHE_DIR", tmp_path)

    def _fake_cache_path(chapter, difficulty):
        return tmp_path / f"ch{chapter:02d}_{difficulty}.json"

    monkeypatch.setattr(base_gen_mod, "_cache_path", _fake_cache_path)
    yield tmp_path


VALID_JSON = json.dumps({
    "text": "What is the univalence axiom?",
    "choices": ["Option A", "Option B", "Option C", "Option D"],
    "answer": 0,
    "explanation": "The univalence axiom states that equivalent types are equal.",
    "tags": ["HoTT", "univalence"],
})

INVALID_JSON_ANSWER_OOB = json.dumps({
    "text": "Test question?",
    "choices": ["A", "B"],
    "answer": 5,
    "explanation": "Explanation.",
    "tags": ["test"],
})

INVALID_JSON_MISSING_FIELD = json.dumps({
    "text": "Test?",
    "choices": ["A", "B"],
    "explanation": "Expl.",
    "tags": [],
})

WRAPPED_IN_MARKDOWN = f"```json\n{VALID_JSON}\n```"


# ── _parse_and_validate ───────────────────────────────────────────────────────

class TestParseAndValidate:
    def test_valid_json_produces_question(self):
        q = _parse_and_validate(VALID_JSON, chapter=18, difficulty="intermediate")
        assert q is not None
        assert isinstance(q, Question)
        assert q.chapter == 18
        assert q.difficulty == "intermediate"
        assert q.generated is True

    def test_strips_markdown_code_fences(self):
        q = _parse_and_validate(WRAPPED_IN_MARKDOWN, chapter=0, difficulty="beginner")
        assert q is not None

    def test_answer_out_of_range_returns_none(self):
        q = _parse_and_validate(INVALID_JSON_ANSWER_OOB, chapter=0, difficulty="beginner")
        assert q is None

    def test_missing_field_returns_none(self):
        q = _parse_and_validate(INVALID_JSON_MISSING_FIELD, chapter=0, difficulty="beginner")
        assert q is None

    def test_malformed_json_returns_none(self):
        q = _parse_and_validate("{ not valid json", chapter=0, difficulty="beginner")
        assert q is None

    def test_empty_string_returns_none(self):
        q = _parse_and_validate("", chapter=0, difficulty="beginner")
        assert q is None

    def test_generated_flag_set_to_true(self):
        q = _parse_and_validate(VALID_JSON, chapter=5, difficulty="advanced")
        assert q.generated is True

    def test_question_id_assigned(self):
        q = _parse_and_validate(VALID_JSON, chapter=5, difficulty="advanced")
        assert q.question_id != ""


# ── Cache operations ──────────────────────────────────────────────────────────

class TestCache:
    def _q_dict(self) -> dict:
        q = _parse_and_validate(VALID_JSON, chapter=18, difficulty="intermediate")
        return q.to_dict()

    def test_add_and_load(self):
        d = self._q_dict()
        _add_to_cache(18, "intermediate", d)
        items = _load_cache(18, "intermediate")
        assert len(items) == 1

    def test_pop_removes_from_cache(self):
        d = self._q_dict()
        _add_to_cache(18, "intermediate", d)
        popped = _pop_from_cache(18, "intermediate")
        assert popped is not None
        assert _load_cache(18, "intermediate") == []

    def test_pop_empty_cache_returns_none(self):
        assert _pop_from_cache(99, "beginner") is None

    def test_cache_respects_cap(self, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "CACHE_CAP_PER_DIFFICULTY", 3)
        d = self._q_dict()
        for _ in range(10):
            _add_to_cache(18, "intermediate", d)
        assert len(_load_cache(18, "intermediate")) <= 3

    def test_cache_survives_round_trip(self):
        d = self._q_dict()
        _add_to_cache(0, "beginner", d)
        popped = _pop_from_cache(0, "beginner")
        assert popped["text"] == d["text"]


# ── ClaudeGenerator ───────────────────────────────────────────────────────────

def _mock_client(response_text: str) -> MagicMock:
    client = MagicMock()
    msg    = MagicMock()
    msg.content = [MagicMock(text=response_text)]
    client.messages.create.return_value = msg
    return client


class TestClaudeGenerator:
    def test_available_false_without_key(self):
        gen = ClaudeGenerator(api_key="")
        assert gen.available is False

    def test_available_true_with_key(self):
        with patch("quiz_app.base_generator._anthropic_module") as mock_anth:
            mock_anth.Anthropic.return_value = MagicMock()
            gen = ClaudeGenerator(api_key="fake-key")
            assert gen.available is True

    def test_get_question_returns_cached_without_api(self):
        gen = ClaudeGenerator(api_key="")
        q_dict = _parse_and_validate(VALID_JSON, 18, "intermediate").to_dict()
        _add_to_cache(18, "intermediate", q_dict)
        result = gen.get_question(18, "intermediate", 50, [])
        assert result is not None
        assert result.text == q_dict["text"]
        assert gen._client is None

    def test_get_question_calls_api_when_cache_empty(self, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "_pick_kind", lambda diff: "mc")
        gen = ClaudeGenerator(api_key="fake-key")
        gen._client = _mock_client(VALID_JSON)
        gen._prefill_cache = lambda *a, **kw: None
        result = gen.get_question(18, "intermediate", 50, [])
        assert result is not None
        assert gen._client.messages.create.called

    def test_get_question_returns_none_when_unavailable_and_no_cache(self):
        gen = ClaudeGenerator(api_key="")
        result = gen.get_question(18, "intermediate", 50, [])
        assert result is None

    def test_get_question_retries_on_bad_json(self, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "_pick_kind", lambda diff: "mc")
        gen = ClaudeGenerator(api_key="fake-key")
        bad_msg  = MagicMock()
        bad_msg.content = [MagicMock(text="{ bad }")]
        good_msg = MagicMock()
        good_msg.content = [MagicMock(text=VALID_JSON)]
        gen._client = MagicMock()
        gen._client.messages.create.side_effect = [bad_msg, good_msg]
        gen._prefill_cache = lambda *a, **kw: None
        result = gen._generate(18, "intermediate", 50, [])
        assert result is not None

    def test_api_exception_returns_none(self):
        gen = ClaudeGenerator(api_key="fake-key")
        gen._client = MagicMock()
        gen._client.messages.create.side_effect = Exception("network error")
        result = gen._generate(18, "intermediate", 50, [], retries=1)
        assert result is None

    def test_generated_question_passes_validation(self, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "_pick_kind", lambda diff: "mc")
        gen = ClaudeGenerator(api_key="fake-key")
        gen._client = _mock_client(VALID_JSON)
        gen._prefill_cache = lambda *a, **kw: None
        q = gen._generate(18, "intermediate", 50, [])
        assert q is not None
        assert q.validate() == []

    def test_prefill_cache_respects_cap(self, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "CACHE_CAP_PER_DIFFICULTY", 3)
        gen = ClaudeGenerator(api_key="fake-key")
        gen._client = _mock_client(VALID_JSON)
        for _ in range(2):
            _add_to_cache(18, "intermediate",
                          _parse_and_validate(VALID_JSON, 18, "intermediate").to_dict())
        gen._prefill_cache(18, "intermediate", 50, [], n=5)
        total = len(_load_cache(18, "intermediate"))
        assert total <= 3

    def test_prefill_cache_noop_when_full(self, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "CACHE_CAP_PER_DIFFICULTY", 2)
        gen = ClaudeGenerator(api_key="fake-key")
        gen._client = MagicMock()
        for _ in range(2):
            _add_to_cache(18, "intermediate",
                          _parse_and_validate(VALID_JSON, 18, "intermediate").to_dict())
        gen._prefill_cache(18, "intermediate", 50, [], n=5)
        assert not gen._client.messages.create.called

    def test_get_question_fresh_id_on_cache_hit(self):
        q_dict = _parse_and_validate(VALID_JSON, 18, "intermediate").to_dict()
        original_id = q_dict["question_id"]
        _add_to_cache(18, "intermediate", q_dict)
        gen = ClaudeGenerator(api_key="")
        result = gen.get_question(18, "intermediate", 50, [])
        assert result is not None
        assert result.question_id != original_id


# ── _load_chapter_excerpt / _build_user_prompt ────────────────────────────────

from quiz_app.generator import _load_chapter_excerpt, _build_user_prompt


class TestLoadChapterExcerpt:
    def test_unknown_chapter_returns_empty(self):
        assert _load_chapter_excerpt(999) == ""

    def test_missing_file_returns_chapter_name(self, tmp_path, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "CHAPTERS_DIR", tmp_path)
        excerpt = _load_chapter_excerpt(0)
        assert "Logic" in excerpt

    def test_excerpt_stripped_of_code_blocks(self, tmp_path, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "CHAPTERS_DIR", tmp_path)
        from quiz_app.config import CHAPTER_META
        fname = CHAPTER_META[0]["file"]
        fpath = tmp_path / fname
        fpath.parent.mkdir(parents=True, exist_ok=True)
        fpath.write_text("intro\n```agda\nsome code\n```\nconclusion")
        excerpt = _load_chapter_excerpt(0)
        assert "some code" not in excerpt
        assert "intro" in excerpt
        assert "conclusion" in excerpt

    def test_excerpt_truncated_to_cap(self, tmp_path, monkeypatch):
        monkeypatch.setattr(base_gen_mod, "CHAPTERS_DIR", tmp_path)
        monkeypatch.setattr(base_gen_mod, "CHAPTER_EXCERPT_CHARS", 50)
        from quiz_app.config import CHAPTER_META
        fname = CHAPTER_META[0]["file"]
        fpath = tmp_path / fname
        fpath.parent.mkdir(parents=True, exist_ok=True)
        fpath.write_text("x" * 200)
        excerpt = _load_chapter_excerpt(0)
        assert len(excerpt) <= 50


class TestBuildUserPrompt:
    def test_prompt_contains_chapter_name(self):
        prompt = _build_user_prompt(18, "Univalence", "advanced", 80, [], "some text")
        assert "Univalence" in prompt

    def test_prompt_contains_difficulty(self):
        prompt = _build_user_prompt(18, "Univalence", "advanced", 80, [], "")
        assert "advanced" in prompt

    def test_prompt_contains_mastery_pct(self):
        prompt = _build_user_prompt(18, "Univalence", "intermediate", 55, [], "")
        assert "55" in prompt

    def test_prompt_contains_weak_topics(self):
        prompt = _build_user_prompt(18, "Univalence", "beginner", 30,
                                    ["Identity Types", "H-Levels"], "")
        assert "Identity Types" in prompt

    def test_prompt_contains_chapter_excerpt(self):
        prompt = _build_user_prompt(18, "Univalence", "intermediate", 50, [],
                                    "The univalence axiom states")
        assert "univalence axiom" in prompt

    def test_prompt_requests_json_output(self):
        prompt = _build_user_prompt(0, "Logic", "beginner", 20, [], "")
        assert "JSON" in prompt
