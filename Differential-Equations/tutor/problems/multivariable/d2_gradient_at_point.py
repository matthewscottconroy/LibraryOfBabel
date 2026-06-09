"""
Multivariable Calculus — gradient (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    a = random.choice([-2, -1, 1, 2])
    b = random.choice([-2, -1, 1, 2])
    p, q = random.randint(-2, 2), random.randint(-2, 2)
    f = a*x**2 + b*x*y + b*y**2
    fx = sp.diff(f, x).subs([(x, p), (y, q)])
    fy = sp.diff(f, y).subs([(x, p), (y, q)])
    correct = f"({fx}, {fy})"
    choices = [correct,
               f"({fy}, {fx})",
               f"({fx+1}, {fy-1})",
               f"({-fx}, {-fy})"]
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="gradient", difficulty=2,
        question=(
            f"Compute ∇f at the point ({p}, {q}) for:\n\n"
            f"  f(x, y) = {f}"
        ),
        answer=correct,
        hint="∇f = (∂f/∂x, ∂f/∂y); evaluate both partials at the given point.",
        explanation=(
            f"∂f/∂x = {sp.diff(f,x)}  →  at ({p},{q}): {fx}\n"
            f"∂f/∂y = {sp.diff(f,y)}  →  at ({p},{q}): {fy}\n"
            f"∇f({p},{q}) = ({fx}, {fy})"
        ),
        problem_type="multiple_choice",
        choices=choices,
    )
