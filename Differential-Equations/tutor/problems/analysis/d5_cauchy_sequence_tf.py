"""
Real Analysis — sequences (difficulty 5).
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
            "Every Cauchy sequence in ℝ converges.",
            True,
            "True. ℝ is complete: every Cauchy sequence converges to a limit in ℝ.\n"
            "This is equivalent to the Completeness Axiom."
        ),
        (
            "Every convergent sequence is a Cauchy sequence.",
            True,
            "True. If xₙ → L, then for any ε > 0 ∃N: |xₙ−L| < ε/2 for n > N,\n"
            "so |xₙ−xₘ| ≤ |xₙ−L|+|xₘ−L| < ε."
        ),
        (
            "Every Cauchy sequence in ℚ converges in ℚ.",
            False,
            "False. ℚ is NOT complete. The sequence of rational approximations to √2\n"
            "is Cauchy in ℚ but converges to √2 ∉ ℚ."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="sequences", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Think about completeness of ℝ vs incompleteness of ℚ.",
        explanation=expl,
        problem_type="true_false",
    )
