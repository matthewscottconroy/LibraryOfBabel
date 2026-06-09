"""
Complex Analysis — argument principle (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "The argument principle states: for f meromorphic in D,\n"
            "  (1/2πi)∮_∂D f'(z)/f(z) dz  equals:",
            "Z − P  (zeros minus poles, counted with multiplicity)",
            "The argument principle: the winding number of f(∂D) around 0 equals\n"
            "Z − P, where Z is the number of zeros and P the number of poles inside D,\n"
            "each counted with multiplicity."
        ),
        (
            "Rouché's theorem: if |f(z)| > |g(z)| on ∂D, then f and f+g have:",
            "The same number of zeros inside D",
            "Rouché's theorem is a consequence of the argument principle.\n"
            "Since f dominates g on the boundary, the perturbation g doesn't change\n"
            "the winding number, hence f and f+g have the same number of zeros inside D."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="argument_principle", difficulty=4,
        question=f"Argument principle:\n\n  {case}",
        answer=correct,
        hint="The argument principle counts zeros and poles via the logarithmic derivative.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "Z + P", "Z only", "P only"],
    )
