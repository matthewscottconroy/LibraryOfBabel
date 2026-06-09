"""
Real Analysis — series (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    num = random.choice([1, 2, 3])
    den = random.choice([3, 4, 5])
    r = sp.Rational(num, den)
    total = r / (1 - r)
    return Problem(
        topic=TOPIC, subtopic="series", difficulty=2,
        question=(
            f"Find the sum of the geometric series:\n\n"
            f"  Σ ({num}/{den})ⁿ,   n = 0 to ∞"
        ),
        answer=total,
        hint="For |r| < 1: Σ rⁿ = 1/(1−r) (starting at n=0).",
        explanation=(
            f"r = {num}/{den}, and |r| < 1, so the series converges.\n"
            f"Sum = 1 / (1 − {num}/{den}) = 1 / ({den-num}/{den}) = {den}/{den-num} = {total}"
        ),
        problem_type="numeric",
    )
