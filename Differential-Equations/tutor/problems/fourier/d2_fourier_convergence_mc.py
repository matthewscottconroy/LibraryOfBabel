"""
Fourier Analysis — convergence (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "f(x) = x on [−π, π], extended periodically.  At x = π?",
            "0",
            "At a jump discontinuity, the Fourier series converges to the average\n"
            "of the left and right limits: [f(π⁻) + f(π⁺)]/2 = [π + (−π)]/2 = 0."
        ),
        (
            "f(x) is continuous and piecewise smooth on [−π, π].  At every point?",
            "f(x)",
            "Dirichlet's theorem: for piecewise smooth continuous f,\n"
            "the Fourier series converges to f(x) at every point."
        ),
        (
            "The 'Gibbs phenomenon' refers to:",
            "Overshoot near jump discontinuities (~9% of the jump)",
            "Near a jump discontinuity, partial sums of the Fourier series\n"
            "overshoot by approximately 9% of the jump height, regardless of\n"
            "how many terms are included."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="convergence", difficulty=2,
        question=f"Fourier series convergence:\n\n  {case}",
        answer=correct,
        hint="At discontinuities: series converges to the average of left/right limits.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "Does not converge", "f(x)", "0", "Undershoot near discontinuities"],
    )
