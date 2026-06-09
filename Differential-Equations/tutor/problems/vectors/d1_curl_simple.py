"""
Vector Calculus — curl (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "vectors"


def generate() -> Problem:
    a, b, c = [random.randint(1, 3) for _ in range(3)]
    # F = (ay, bz, cx) → curl has clean components
    F1, F2, F3 = a*y, b*z, c*x
    curlx = sp.diff(F3, y) - sp.diff(F2, z)
    curly = sp.diff(F1, z) - sp.diff(F3, x)
    curlz = sp.diff(F2, x) - sp.diff(F1, y)
    correct = f"({curlx}, {curly}, {curlz})"
    wrong1  = f"({curly}, {curlx}, {curlz})"
    wrong2  = f"({-curlx}, {-curly}, {-curlz})"
    wrong3  = f"(0, 0, 0)"
    choices = list({correct, wrong1, wrong2, wrong3})
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="curl", difficulty=1,
        question=(
            f"Compute the curl of:\n\n"
            f"  F = ({F1}, {F2}, {F3})"
        ),
        answer=correct,
        hint="curl F = (∂R/∂y−∂Q/∂z, ∂P/∂z−∂R/∂x, ∂Q/∂x−∂P/∂y)",
        explanation=(
            f"curlₓ = ∂({F3})/∂y − ∂({F2})/∂z = {curlx}\n"
            f"curly = ∂({F1})/∂z − ∂({F3})/∂x = {curly}\n"
            f"curl_z = ∂({F2})/∂x − ∂({F1})/∂y = {curlz}\n"
            f"curl F = ({curlx}, {curly}, {curlz})"
        ),
        problem_type="multiple_choice",
        choices=choices,
    )
