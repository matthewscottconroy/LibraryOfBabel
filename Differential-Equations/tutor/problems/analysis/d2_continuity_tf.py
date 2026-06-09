"""
Real Analysis — continuity (difficulty 2).
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
            "Every differentiable function is continuous.",
            True,
            "Differentiability implies continuity: if f'(a) exists, f is continuous at a.\n"
            "The converse is false: |x| is continuous at 0 but not differentiable there."
        ),
        (
            "Every continuous function is differentiable.",
            False,
            "False. f(x) = |x| is continuous everywhere but not differentiable at x = 0."
        ),
        (
            "A function can be continuous at a point but not differentiable there.",
            True,
            "True. f(x) = |x| at x = 0 is the classic example."
        ),
        (
            "If f is continuous on [a,b], then f attains its maximum value on [a,b].",
            True,
            "This is the Extreme Value Theorem: a continuous function on a closed bounded\n"
            "interval attains both its maximum and minimum."
        ),
        (
            "If f is continuous on (a,b), then f is bounded on (a,b).",
            False,
            "False. f(x) = 1/x is continuous on (0,1) but unbounded.\n"
            "Boundedness requires a CLOSED interval by the Extreme Value Theorem."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="continuity", difficulty=2,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Think of a specific example to test or disprove.",
        explanation=expl,
        problem_type="true_false",
    )
