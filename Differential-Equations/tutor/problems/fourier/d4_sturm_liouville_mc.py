"""
Fourier Analysis — sturm liouville (difficulty 4).
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
            "The Sturm-Liouville eigenfunctions {φₙ} on [a,b] form:",
            "A complete orthogonal basis for L²([a,b])",
            "Sturm-Liouville theory: eigenfunctions corresponding to distinct eigenvalues\n"
            "are orthogonal w.r.t. the weight function, and form a complete basis\n"
            "for L² with that weight."
        ),
        (
            "For a regular Sturm-Liouville problem, the eigenvalues are:",
            "Real, discrete, and ordered λ₁ < λ₂ < λ₃ < ...",
            "Regular SL problems have real, discrete eigenvalues forming a monotone\n"
            "sequence tending to +∞, with each eigenvalue simple (1-dimensional eigenspace)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="sturm_liouville", difficulty=4,
        question=f"Sturm-Liouville theory:\n\n  {case}",
        answer=correct,
        hint="Regular S-L eigenvalues are real, simple, and form an increasing sequence.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "Complex eigenvalues with non-negative imaginary part",
                 "Continuous spectrum only", "Finite set of eigenvalues"],
    )
