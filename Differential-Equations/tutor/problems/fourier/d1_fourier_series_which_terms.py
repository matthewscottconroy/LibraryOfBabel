"""
Fourier Analysis — fourier series (difficulty 1).
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
            "f(x) = x³ on [−π, π]  (extend periodically)",
            "Sine series only (bₙ terms)",
            "f is ODD, so all aₙ = 0. The Fourier series contains only sine terms."
        ),
        (
            "f(x) = x² on [−π, π]  (extend periodically)",
            "Cosine series only (a₀ and aₙ terms)",
            "f is EVEN, so all bₙ = 0. The series has only cosine and constant terms."
        ),
        (
            "f(x) = x + x² on [−π, π]",
            "Both sine and cosine terms",
            "f is neither even nor odd (x² is even, x is odd).\n"
            "The Fourier series contains both aₙ and bₙ terms."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="fourier_series", difficulty=1,
        question=f"What terms appear in the Fourier series of:\n\n  {case}",
        answer=correct,
        hint="Even functions → cosine series; odd functions → sine series.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=["Sine series only (bₙ terms)", "Cosine series only (a₀ and aₙ terms)",
                 "Both sine and cosine terms", "Constant term only"],
    )
