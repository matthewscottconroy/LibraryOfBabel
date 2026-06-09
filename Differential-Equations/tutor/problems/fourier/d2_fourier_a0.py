"""
Fourier Analysis — fourier coefficients (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    case, a0, expl = random.choice([
        (
            "f(x) = x  on [−π, π]",
            0,
            "a₀ = (1/π)∫_{-π}^{π} x dx = 0  (integrand is odd on symmetric interval)."
        ),
        (
            "f(x) = 1  on [−π, π]",
            2,
            "a₀ = (1/π)∫_{-π}^{π} 1 dx = (1/π)·2π = 2."
        ),
        (
            "f(x) = |x|  on [−π, π]",
            "π",
            "a₀ = (1/π)∫_{-π}^{π}|x| dx = (2/π)∫₀^π x dx = (2/π)·π²/2 = π."
        ),
        (
            "f(x) = x²  on [−π, π]",
            "2π²/3",
            "a₀ = (1/π)∫_{-π}^π x² dx = (2/π)·π³/3 = 2π²/3."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="fourier_coefficients", difficulty=2,
        question=(
            f"Compute the constant term a₀ in the Fourier series of:\n\n"
            f"  {case}\n\n"
            f"(Convention: a₀ = (1/L)∫_{{-L}}^L f(x) dx  where L = π)"
        ),
        answer=a0,
        hint="a₀ = (1/π)∫_{-π}^{π} f(x) dx.  Exploit symmetry if f is even or odd.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[str(a0), "0", "π", "2π", "π/2", "2π²/3"],
    )
