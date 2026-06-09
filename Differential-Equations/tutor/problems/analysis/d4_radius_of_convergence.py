"""
Real Analysis — series (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    center, radius, series_str, expl = random.choice([
        (0, 1,
         "Σ xⁿ  (geometric series)",
         "Ratio test: |xⁿ⁺¹/xⁿ| = |x| < 1. Radius R = 1."),
        (0, sp.oo,
         "Σ xⁿ/n!  (Maclaurin series for eˣ)",
         "Ratio: |x|/(n+1) → 0 for all x. Radius R = ∞."),
        (0, 2,
         "Σ xⁿ/2ⁿ",
         "Ratio: |x/2| < 1 ↔ |x| < 2. Radius R = 2."),
        (0, 3,
         "Σ xⁿ/3ⁿ",
         "Ratio: |x/3| < 1 ↔ |x| < 3. Radius R = 3."),
    ])
    ans = sp.oo if radius is sp.oo else sp.Integer(radius)
    return Problem(
        topic=TOPIC, subtopic="series", difficulty=4,
        question=f"Find the radius of convergence of:\n\n  {series_str}",
        answer=ans,
        hint="Apply the ratio test: lim |aₙ₊₁/aₙ| < 1.",
        explanation=expl,
        problem_type="numeric",
    )
