"""
Fourier Analysis — discrete fourier (difficulty 4).
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
            "The DFT of the sequence [1, 0, 0, 0] (N=4) is:",
            "[1, 1, 1, 1]",
            "DFT: Xₖ = Σₙ xₙ e^(-2πink/N). With x₀=1 and all others 0:\n"
            "Xₖ = 1·e^0 = 1 for all k = 0,1,2,3. So DFT = [1,1,1,1]."
        ),
        (
            "The Fast Fourier Transform (FFT) reduces DFT computation from O(N²) to:",
            "O(N log N)",
            "The Cooley-Tukey FFT algorithm exploits the symmetry of the DFT\n"
            "twiddle factors to reduce computation from O(N²) to O(N log N).\n"
            "This is one of the most important algorithms in applied mathematics."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="discrete_fourier", difficulty=4,
        question=f"Discrete Fourier Transform:\n\n  {case}",
        answer=correct,
        hint="DFT: Xₖ = Σₙ xₙ e^(-2πink/N) for n=0,…,N-1.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "[1, 0, −1, 0]", "[1, i, −1, −i]", "O(N²)"],
    )
