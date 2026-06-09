"""
Ordinary Differential Equations — qualitative (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """Autonomous dy/dx = r(y-a)(y-b), identify stable/unstable equilibria."""
    a, b = sorted(random.sample([-2, -1, 0, 1, 2], 2))
    r = random.choice([-1, 1])
    stmt, correct, expl = random.choice([
        (
            f"dy/dx = {r}(y−{a})(y−{b}),  equilibria at y={a} and y={b}",
            f"y={a} {'stable' if r>0 else 'unstable'}, y={b} {'unstable' if r>0 else 'stable'}",
            (f"Linearize near each equilibrium by examining the sign of f'(y).\n"
             f"f(y) = {r}(y−{a})(y−{b});  f'(y) = {r}[(y−{b})+(y−{a})] = {r}(2y−{a+b})\n"
             f"At y={a}: f'({a}) = {r}({a}-{b}) {'< 0 (stable)' if r*(a-b)<0 else '> 0 (unstable)'}\n"
             f"At y={b}: f'({b}) = {r}({b}-{a}) {'< 0 (stable)' if r*(b-a)<0 else '> 0 (unstable)'}")
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="qualitative", difficulty=1,
        question=f"For the autonomous ODE:\n\n  {stmt.split(',')[0]}\n\nClassify the equilibria.",
        answer=correct,
        hint="An equilibrium y* is stable if f'(y*) < 0, unstable if f'(y*) > 0.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[
            correct,
            f"y={a} unstable, y={b} stable" if "stable" in correct.split(",")[0] else f"y={a} stable, y={b} unstable",
            "Both stable",
            "Both unstable",
        ],
    )
