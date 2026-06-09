"""
Real Analysis — MVT (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    a, b = sorted(random.sample(range(1, 6), 2))
    c_vals = [2, 3, 4]
    c = random.choice(c_vals)
    # Use f(x) = x^2, MVT gives f'(c) = (f(b)-f(a))/(b-a) = (b+a)
    # c = (a+b)/2 for f(x)=x²
    f_a, f_b = a**2, b**2
    avg_rate = (f_b - f_a) / (b - a)
    c_exact = sp.Rational(a + b, 2)
    return Problem(
        topic=TOPIC, subtopic="MVT", difficulty=4,
        question=(
            f"The Mean Value Theorem guarantees a c ∈ ({a}, {b}) such that\n"
            f"f'(c) = (f({b}) − f({a})) / ({b} − {a})  for  f(x) = x².\n\n"
            f"Find the exact value of c."
        ),
        answer=c_exact,
        hint="Compute f'(x) = 2x, set 2c = (b²−a²)/(b−a) = a+b, solve for c.",
        explanation=(
            f"f'(x) = 2x.  Average rate = ({b**2}−{a**2})/({b}−{a}) = {avg_rate}.\n"
            f"Set 2c = {avg_rate} → c = {c_exact}."
        ),
        problem_type="numeric",
    )
