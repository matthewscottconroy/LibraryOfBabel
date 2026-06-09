"""
Static question bank — loads from quiz_app/questions/ directory.

Directory layout:
    quiz_app/questions/
        ch00-logic-sets-and-proof/      ← one subdirectory per chapter
            induction.yaml
            logic.yaml
            ...
        ch08-group-theory-foundations/
            lagrange.yaml
            sylow.yaml
            ...

Each .yaml file defines exactly one question. The chapter number and phase are
inferred from the directory name prefix (ch{NN}-...). Chapter metadata (phase,
full name) is resolved from config.CHAPTER_META.

Question schema
───────────────
Multiple-choice (kind: mc):
    kind: mc
    difficulty: beginner | intermediate | advanced
    text: "Question text (use ___ for blanks if desired)"
    choices:
      - "Choice A"
      - "Choice B"
      - "Choice C"
      - "Choice D"
    answer: 0          # zero-based index of correct choice
    explanation: "Why the answer is correct (≥2 sentences)."
    tags:
      - tag1
      - tag2

True/False (kind: tf):
    kind: tf
    difficulty: beginner
    text: "A statement that is definitively true or false."
    answer: true       # or false
    explanation: "..."
    tags: [...]

Fill-in-the-blank (kind: blank):
    kind: blank
    difficulty: intermediate
    text: "Sentence with ___ where the key term belongs."
    choices:           # all acceptable answers; first is canonical
      - primary answer
      - accepted synonym
    explanation: "..."
    tags: [...]

Proof-scaffold (kind: proof):
    kind: proof
    difficulty: intermediate | advanced
    text: "Statement of the theorem being proved."
    proof_lines:
      - "Line 1 of proof (no blank)"
      - "Line 2 containing ___ at a key step"
      - "Line 3 containing ___ at another step"
    fills:             # one entry per ___ in proof_lines, in order
      - canonical fill for first blank
      - canonical fill for second blank
    explanation: "Why the blanked steps are the key ideas."
    tags: [...]
"""
from __future__ import annotations

from pathlib import Path
from typing import Optional

from .config import CHAPTER_META
from .models import Question

QUESTIONS_DIR = Path(__file__).parent / "questions"

try:
    import yaml as _yaml

    def _parse_file(path: Path) -> Optional[dict]:
        try:
            return _yaml.safe_load(path.read_text(encoding="utf-8"))
        except Exception:
            return None

except ImportError:
    import json as _json  # type: ignore[no-redef]

    def _parse_file(path: Path) -> Optional[dict]:   # type: ignore[misc]
        try:
            return _json.loads(path.read_text(encoding="utf-8"))
        except Exception:
            return None


def _chapter_from_dirname(name: str) -> Optional[int]:
    try:
        return int(name[2:4])
    except (ValueError, IndexError):
        return None


def _question_from_dict(
    data: dict, ch: int, phase: int, stem: str
) -> Optional[Question]:
    kind = data.get("kind")
    if kind not in ("mc", "tf", "blank", "proof"):
        return None

    text        = str(data.get("text", "")).strip()
    explanation = str(data.get("explanation", "")).strip()
    tags        = [str(t) for t in data.get("tags", [])]
    difficulty  = str(data.get("difficulty", "intermediate"))
    question_id = f"ch{ch:02d}:{stem}"

    if kind == "mc":
        choices = [str(c) for c in data.get("choices", [])]
        try:
            answer: int | str = int(data["answer"])
        except (KeyError, TypeError, ValueError):
            return None
    elif kind == "tf":
        raw    = data.get("answer", True)
        answer = 0 if raw in (True, "true", "True", "yes", 1) else 1
        choices = ["True", "False"]
    elif kind == "proof":
        choices = [str(l) for l in data.get("proof_lines", [])]
        fills   = [str(f).strip() for f in data.get("fills", [])]
        answer  = "|".join(fills)
    else:  # blank
        choices = [str(c) for c in data.get("choices", [])]
        answer  = choices[0].lower() if choices else ""

    q = Question(
        chapter=ch, phase=phase, kind=kind,
        text=text, choices=choices, answer=answer,
        explanation=explanation, tags=tags,
        difficulty=difficulty, question_id=question_id,
    )
    return q if not q.validate() else None


def _load_chapter(ch: int, ch_dir: Path) -> list[Question]:
    phase = CHAPTER_META.get(ch, {}).get("phase", 0)
    questions: list[Question] = []
    for path in sorted(ch_dir.glob("*.yaml")):
        data = _parse_file(path)
        if not isinstance(data, dict):
            continue
        q = _question_from_dict(data, ch, phase, path.stem)
        if q:
            questions.append(q)
    return questions


def load_questions() -> list[Question]:
    """Load all questions from the questions/ directory tree."""
    if not QUESTIONS_DIR.exists():
        return []
    questions: list[Question] = []
    for ch_dir in sorted(QUESTIONS_DIR.iterdir()):
        if not ch_dir.is_dir() or ch_dir.name.startswith("."):
            continue
        ch = _chapter_from_dirname(ch_dir.name)
        if ch is None:
            continue
        questions.extend(_load_chapter(ch, ch_dir))
    return questions


QUESTIONS = load_questions()
