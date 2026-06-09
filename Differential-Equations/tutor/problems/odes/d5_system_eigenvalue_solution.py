"""
Ordinary Differential Equations — systems (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """x' = Ax,  A with integer eigenvalues"""
    l1, l2 = random.sample([-3, -2, -1, 1, 2], 2)
    # Diagonal case for clarity
    A11, A12, A21, A22 = l1, 0, 0, l2
    ev_str = f"λ₁ = {l1},  λ₂ = {l2}"
    sol_str = f"C₁e^({l1}t) v₁ + C₂e^({l2}t) v₂"
    case_str = f"A = [[{l1}, 0], [0, {l2}]]"
    return Problem(
        topic=TOPIC, subtopic="systems", difficulty=5,
        question=(
            f"Find the general solution of  x' = Ax  where:\n\n"
            f"  {case_str}\n\n"
            f"Write as a linear combination of exponential solutions."
        ),
        answer=sol_str,
        hint="Find eigenvalues and eigenvectors of A; form Σ cₖe^(λₖt)vₖ.",
        explanation=(
            f"A is diagonal, so eigenvalues are its diagonal entries:\n"
            f"λ₁ = {l1} with eigenvector e₁ = (1,0)\n"
            f"λ₂ = {l2} with eigenvector e₂ = (0,1)\n"
            f"General solution: x(t) = C₁e^({l1}t)(1,0) + C₂e^({l2}t)(0,1)"
        ),
        problem_type="multiple_choice",
        choices=[
            sol_str,
            f"C₁e^({l1+l2}t) + C₂e^({l1*l2}t)",
            f"C₁cos({l1}t) + C₂sin({l2}t)",
            f"C₁e^({-l1}t) + C₂e^({-l2}t)",
        ],
    )
