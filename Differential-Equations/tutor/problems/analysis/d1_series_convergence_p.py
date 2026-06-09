"""
Real Analysis — series (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    p = random.choice([
        (sp.Rational(1, 2), False, "p = 1/2 < 1"),
        (1,                 False, "p = 1 (harmonic series, diverges)"),
        (sp.Rational(3, 2), True,  "p = 3/2 > 1"),
        (2,                 True,  "p = 2 > 1"),
        (3,                 True,  "p = 3 > 1"),
    ])
    p_val, converges, reason = p
    choices = ["Converges", "Diverges"]
    correct = "Converges" if converges else "Diverges"
    return Problem(
        topic=TOPIC, subtopic="series", difficulty=1,
        question=f"Does the p-series  Σ 1/n^({p_val})  (n=1 to ∞) converge or diverge?",
        answer=correct,
        hint="The p-series Σ 1/nᵖ converges iff p > 1.",
        explanation=(
            f"By the p-series test: Σ 1/nᵖ converges ⟺ p > 1.\n"
            f"Here {reason}, so the series {correct.lower()}s."
        ),
        problem_type="multiple_choice",
        choices=choices,
    )
