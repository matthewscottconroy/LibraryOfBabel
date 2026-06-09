"""
Real Analysis — continuity (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "f(x) = x² is uniformly continuous on ℝ.",
            False,
            "False. For any δ > 0, take x = n+δ/2, y = n. Then |x−y| = δ/2 < δ but\n"
            "|f(x)−f(y)| = |2nδ/2 + δ²/4| → ∞ as n → ∞, so ε-independence fails."
        ),
        (
            "Every continuous function on a closed bounded interval is uniformly continuous.",
            True,
            "True. This is the Heine-Cantor theorem: continuity on a compact set implies\n"
            "uniform continuity."
        ),
        (
            "f(x) = sin(x²) is uniformly continuous on ℝ.",
            False,
            "False. The function oscillates increasingly rapidly, so small |x−y|\n"
            "can give |sin(x²)−sin(y²)| close to 2 for large x."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="continuity", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Recall: uniform continuity requires a single δ that works for ALL pairs x, y.",
        explanation=expl,
        problem_type="true_false",
    )
