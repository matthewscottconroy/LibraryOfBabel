"""
Linear Algebra — linear systems (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    # Generate solution first, then build system
    x1, x2 = random.randint(-3, 3), random.randint(-3, 3)
    a, b = random.choice([(1,2),(2,1),(1,3),(3,1),(2,3)])
    c, d = random.choice([(1,1),(2,3),(3,2),(1,-1)])
    rhs1 = a*x1 + b*x2
    rhs2 = c*x1 + d*x2
    det = a*d - b*c
    if det == 0:
        a, b, c, d = 1, 2, 3, 1
        rhs1 = a*x1 + b*x2
        rhs2 = c*x1 + d*x2
    return Problem(
        topic=TOPIC, subtopic="linear_systems", difficulty=2,
        question=(
            f"Solve the linear system:\n\n"
            f"  {a}x + {b}y = {rhs1}\n"
            f"  {c}x + {d}y = {rhs2}"
        ),
        answer=f"x = {x1}, y = {x2}",
        hint="Use substitution or elimination, or Cramer's rule.",
        explanation=(
            f"Using Cramer's rule:  det(A) = {a}·{d} − {b}·{c} = {a*d-b*c}\n"
            f"x = ({rhs1}·{d} − {b}·{rhs2}) / det = {x1}\n"
            f"y = ({a}·{rhs2} − {rhs1}·{c}) / det = {x2}"
        ),
        problem_type="multiple_choice",
        choices=[
            f"x = {x1}, y = {x2}",
            f"x = {x2}, y = {x1}",
            f"x = {-x1}, y = {-x2}",
            f"x = {x1+1}, y = {x2-1}",
        ],
    )
