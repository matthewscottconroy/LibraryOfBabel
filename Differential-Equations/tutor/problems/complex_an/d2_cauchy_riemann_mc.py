"""
Complex Analysis — analytic functions (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    case, analytic, expl = random.choice([
        (
            "f(z) = z² = (x²−y²) + i·(2xy)",
            "Analytic everywhere",
            "u=x²−y², v=2xy. uₓ=2x=vᵧ, uᵧ=−2y=−vₓ. C-R satisfied ∀(x,y).\n"
            "f is entire (analytic everywhere)."
        ),
        (
            "f(z) = z̄ = x − iy  (complex conjugate)",
            "Nowhere analytic",
            "u=x, v=−y. uₓ=1, vᵧ=−1. C-R requires uₓ=vᵧ → 1=−1. Fails.\n"
            "z̄ is nowhere analytic."
        ),
        (
            "f(z) = eˣ cos y + i·eˣ sin y  (= eᶻ)",
            "Analytic everywhere",
            "u=eˣcos y, v=eˣsin y. uₓ=eˣcos y=vᵧ, uᵧ=−eˣsin y=−vₓ. C-R holds.\n"
            "This is eᶻ, an entire function."
        ),
        (
            "f(z) = |z|²  (= x²+y², real-valued except at 0)",
            "Only at z = 0",
            "u=x²+y², v=0. uₓ=2x=vᵧ=0 requires x=0; uᵧ=2y=−vₓ=0 requires y=0.\n"
            "C-R holds only at the origin."
        ),
    ])
    choices = ["Analytic everywhere", "Nowhere analytic", "Only at z = 0", "Only on the real axis"]
    return Problem(
        topic=TOPIC, subtopic="analytic_functions", difficulty=2,
        question=f"Where is the function analytic (satisfies Cauchy-Riemann equations)?\n\n  {case}",
        answer=analytic,
        hint="Check C-R: uₓ = vᵧ and uᵧ = −vₓ, where f = u + iv.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
