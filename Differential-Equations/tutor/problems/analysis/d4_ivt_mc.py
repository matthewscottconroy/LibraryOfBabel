"""
Real Analysis — continuity (difficulty 4).
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
            "f(x) = x³ − x − 1 on [1, 2].  Does f have a zero in (1,2)?",
            "Yes (Intermediate Value Theorem)",
            "f(1) = 1−1−1 = −1 < 0.  f(2) = 8−2−1 = 5 > 0.\n"
            "f is continuous and changes sign, so by IVT, ∃c ∈ (1,2) with f(c) = 0."
        ),
        (
            "A continuous f: [0,1]→[0,1] must have:",
            "At least one fixed point (f(c) = c)",
            "Consider g(x) = f(x)−x. g(0) = f(0)−0 ≥ 0, g(1) = f(1)−1 ≤ 0.\n"
            "By IVT, ∃c ∈ [0,1] with g(c) = 0, i.e. f(c) = c.  (Brouwer in 1D.)"
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="continuity", difficulty=4,
        question=f"Intermediate Value Theorem:\n\n  {case}",
        answer=correct,
        hint="IVT: if f is continuous on [a,b] and f(a), f(b) have opposite signs, there's a zero.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "No, f might not have a zero", "Only if f is differentiable",
                 "Not enough information"],
    )
