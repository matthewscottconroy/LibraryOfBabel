"""
Fourier Analysis — fourier coefficients (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    case, bn, n_val, expl = random.choice([
        (
            "f(x) = x  on [−π, π]",
            "2(−1)^(n+1)/n",
            "n",
            "bₙ = (1/π)∫_{-π}^π x sin(nx) dx = (2/π)∫₀^π x sin(nx) dx\n"
            "Integration by parts: = 2(−1)^(n+1)/n."
        ),
        (
            "f(x) = 1 on (0, π), f(x) = 0 on (−π, 0)",
            "1/n · (1−(−1)^n)/π · π = (1−(−1)^n)/(nπ) · π → (1−(−1)^n)/(n·π)·π",
            "n",
            "bₙ = (1/π)∫₀^π 1·sin(nx) dx = (1/π)[−cos(nx)/n]₀^π\n"
            "= (1/nπ)(1 − cos(nπ)) = (1−(−1)^n)/(nπ)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="fourier_coefficients", difficulty=3,
        question=(
            f"Compute the Fourier coefficient bₙ for:\n\n"
            f"  {case}\n\n"
            f"Result: bₙ = ?"
        ),
        answer=bn,
        hint="bₙ = (1/π)∫_{-π}^{π} f(x)sin(nx) dx.  Use integration by parts if needed.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[bn, "0", "2/n", "(−1)^n/n"],
    )
