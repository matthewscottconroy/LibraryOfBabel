"""
Question bank — loads all questions from questions/<chapter>/*.json.
Adding or removing questions requires no code changes: drop/delete JSON files.
"""
from __future__ import annotations

import json
from pathlib import Path

from .models import Question

QUESTIONS_DIR = Path(__file__).parent.parent / "questions"


def _load_questions() -> list[Question]:
    questions: list[Question] = []
    for path in sorted(QUESTIONS_DIR.glob("**/*.json")):
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
            q = Question.from_dict(data)
            errs = q.validate()
            if not errs:
                questions.append(q)
        except Exception:
            pass
    return questions


QUESTIONS: list[Question] = _load_questions()

# Compatibility alias for tests and tooling that call build()
def build() -> list[Question]:
    return _load_questions()
