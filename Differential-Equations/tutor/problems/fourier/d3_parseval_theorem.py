"""
Fourier Analysis — parseval (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    case, parseval_result, expl = random.choice([
        (
            "f(x) = x  on [−π, π]  (use Σ 1/n² = π²/6)",
            "π²/3",
            "Parseval: (1/π)∫_{-π}^π x² dx = Σ (aₙ²+bₙ²)/2 + a₀²/2.\n"
            "∫_{-π}^π x² dx = 2π³/3.  (1/π)·2π³/3 = 2π²/3.\n"
            "bₙ = 2(−1)^(n+1)/n, so Σbₙ² = 4Σ1/n² = 4π²/6 = 2π²/3. ✓\n"
            "(1/π)∫x² dx = 2π²/3 = Σbₙ²/2 ... see Parseval more carefully:\n"
            "Standard form gives 2·Σ1/n² = π²/3, i.e. Σ1/n² = π²/6. ✓"
        ),
        (
            "Parseval's theorem relates the L²-norm of f to its Fourier coefficients via:",
            "(1/π)∫_{-π}^π |f|² dx = a₀²/2 + Σ(aₙ²+bₙ²)",
            "Parseval's identity: ‖f‖² (in L²) equals the sum of squares of Fourier\n"
            "coefficients. This is the analogue of the Pythagorean theorem for\n"
            "orthonormal bases in function space."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="parseval", difficulty=3,
        question=f"Parseval's theorem:\n\n  {case}",
        answer=parseval_result,
        hint="Parseval: (1/π)∫|f|² dx = a₀²/2 + Σₙ(aₙ² + bₙ²).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[parseval_result,
                 "(1/π)∫_{-π}^π |f|² dx = Σ aₙ",
                 "π²/6",
                 "2π²/3"],
    )
