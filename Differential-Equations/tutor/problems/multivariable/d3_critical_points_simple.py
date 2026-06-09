"""
Multivariable Calculus — optimization (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    a = random.choice([-4, -3, -2, 2, 3, 4])
    b = random.choice([-3, -2, 2, 3])
    # f = x^3 + ax + b*y^2 → critical at x where 3x²+a=0, y=0
    f = x**3 + a*x + b*y**2
    fx = sp.diff(f, x)
    fy = sp.diff(f, y)
    sols = sp.solve([fx, fy], [x, y])
    if not sols or (isinstance(sols, list) and len(sols) == 0):
        # fallback: simple f
        f = x**2 + a*x + b*y**2
        fx = sp.diff(f, x)
        fy = sp.diff(f, y)
        sols = sp.solve([fx, fy], [x, y])
    if isinstance(sols, dict):
        sols = [sols]
    pts = [(s.get(x, s[0]) if isinstance(s, dict) else s[0],
            s.get(y, s[1]) if isinstance(s, dict) else s[1]) for s in sols]
    pts_str = ", ".join(f"({p[0]}, {p[1]})" for p in pts if p[0].is_real)
    if not pts_str:
        pts_str = "None (no real critical points)"
    choices = [pts_str, "(0, 0)", f"({a}, 0)", "None (no real critical points)"]
    choices = list(dict.fromkeys(choices))  # deduplicate
    return Problem(
        topic=TOPIC, subtopic="optimization", difficulty=3,
        question=f"Find all critical points of:\n\n  f(x, y) = {f}",
        answer=pts_str,
        hint="Set ∂f/∂x = 0 and ∂f/∂y = 0 simultaneously.",
        explanation=(
            f"∂f/∂x = {fx} = 0\n"
            f"∂f/∂y = {fy} = 0\n"
            f"Solving: critical point(s) at {pts_str}"
        ),
        problem_type="multiple_choice",
        choices=choices[:4],
    )
