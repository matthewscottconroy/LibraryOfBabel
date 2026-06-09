"""
Real Analysis — integration (difficulty 5).
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
            "Every monotone bounded function on [a,b] is Riemann integrable.",
            True,
            "True. A monotone function has at most countably many discontinuities,\n"
            "which form a set of measure zero. By Lebesgue's criterion, the function\n"
            "is Riemann integrable."
        ),
        (
            "The function f(x) = 1 if x ∈ ℚ, 0 if x ∉ ℚ (Dirichlet function) is Riemann integrable on [0,1].",
            False,
            "False. For any partition, upper sums equal 1 and lower sums equal 0.\n"
            "Since upper ≠ lower, the Riemann integral does not exist.\n"
            "(However, the Lebesgue integral exists and equals 0.)"
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="integration", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Lebesgue's criterion: f is Riemann integrable iff its discontinuities form a set of measure zero.",
        explanation=expl,
        problem_type="true_false",
    )
