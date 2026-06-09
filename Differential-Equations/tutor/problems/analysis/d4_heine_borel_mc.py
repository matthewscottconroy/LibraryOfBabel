"""
Real Analysis — compactness (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "Which of the following subsets of ℝ is compact?",
            "[0, 1]",
            "By Heine-Borel: a subset of ℝⁿ is compact iff it is closed AND bounded.\n"
            "[0,1] is closed (contains its endpoints) and bounded → compact.\n"
            "(0,1) is bounded but not closed; [0,∞) is closed but not bounded."
        ),
        (
            "A compact set K ⊂ ℝ must satisfy:",
            "K is closed and bounded",
            "Heine-Borel theorem: in ℝⁿ, compact ⟺ closed and bounded.\n"
            "Equivalently: every open cover has a finite subcover."
        ),
        (
            "If f: K→ℝ is continuous and K is compact, then f is:",
            "Uniformly continuous and attains its max/min",
            "Continuous images of compact sets are compact: f(K) is compact in ℝ,\n"
            "hence closed and bounded, so f attains max/min.\n"
            "Also, continuous functions on compact sets are uniformly continuous (Heine-Cantor)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="compactness", difficulty=4,
        question=f"Compactness (Heine-Borel):\n\n  {case}",
        answer=correct,
        hint="In ℝⁿ: compact ⟺ closed AND bounded (Heine-Borel theorem).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "(0, 1)", "[0, ∞)", "ℝ"],
    )
