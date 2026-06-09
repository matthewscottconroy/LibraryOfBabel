"""
Real Analysis — series (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    series, correct_test, why = random.choice([
        ("Σ n!/nⁿ", "Ratio test",
         "Ratio test: aₙ₊₁/aₙ = (n+1)!/(n+1)^(n+1) · nⁿ/n! → 1/e < 1. Converges."),
        ("Σ 1/(n ln n)", "Integral test",
         "Integral test: ∫ dx/(x ln x) = ln(ln x) → ∞. Diverges."),
        ("Σ (-1)ⁿ/n", "Alternating series test",
         "Alternating, 1/n decreasing to 0. Converges by alternating series test."),
        ("Σ n²·(1/3)ⁿ", "Ratio test",
         "Ratio: (n+1)²/(n²)·(1/3) → 1/3 < 1. Converges."),
        ("Σ sin(1/n²)", "Comparison/limit comparison",
         "sin(1/n²) ≈ 1/n² for large n. Limit comparison with Σ1/n² (converges)."),
    ])
    choices = ["Ratio test", "Root test", "Alternating series test",
               "Integral test", "Comparison/limit comparison"]
    return Problem(
        topic=TOPIC, subtopic="series", difficulty=3,
        question=f"Which convergence test is MOST DIRECTLY applicable to:\n\n  {series}",
        answer=correct_test,
        hint="Consider what structure the series has: alternating, geometric-like, factorials?",
        explanation=why,
        problem_type="multiple_choice",
        choices=choices,
    )
