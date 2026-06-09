"""
Multivariable Calculus — double integrals (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    a, b = sorted(random.sample(range(0, 4), 2))
    c, d = sorted(random.sample(range(0, 4), 2))
    coeff = random.choice([1, 2, 3])
    f = coeff * x * y
    # ∫∫ c·xy dy dx over [a,b]×[c,d]
    inner = sp.integrate(f, (y, c, d))
    ans = sp.integrate(inner, (x, a, b))
    return Problem(
        topic=TOPIC, subtopic="double_integrals", difficulty=2,
        question=(
            f"Evaluate the double integral:\n\n"
            f"  ∫∫ {coeff}xy dA,   where R = [{a},{b}] × [{c},{d}]"
        ),
        answer=ans,
        hint="Integrate with respect to y first (holding x fixed), then integrate with respect to x.",
        explanation=(
            f"∫_{{{a}}}^{{{b}}} ∫_{{{c}}}^{{{d}}} {coeff}xy dy dx\n"
            f"Inner: ∫_{{{c}}}^{{{d}}} {coeff}xy dy = {coeff}x · [y²/2]_{{{c}}}^{{{d}}}"
            f" = {sp.simplify(inner)}\n"
            f"Outer: ∫_{{{a}}}^{{{b}}} {sp.simplify(inner)} dx = {ans}"
        ),
        problem_type="numeric",
    )
