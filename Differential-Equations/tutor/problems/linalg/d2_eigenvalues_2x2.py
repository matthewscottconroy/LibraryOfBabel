"""
Linear Algebra — eigenvalues (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    # Choose integer eigenvalues, build matrix from them
    l1, l2 = random.sample(range(-4, 5), 2)
    # A = [[l1+1, 1], [l1*l2 - (l1+1)*l2, l2 - 1 + 1]] — simpler: use trace/det
    tr = l1 + l2
    det = l1 * l2
    # Characteristic: λ² − tr·λ + det = 0
    a = tr - random.randint(0, 2)   # jitter for variety
    b = random.randint(-2, 2)
    c_entry = random.randint(-2, 2)
    d = tr - a
    # Recompute so eigenvalues are exact integers
    a, b, c_entry, d = l1 + 1, 1, l1*l2 - (l1+1)*l2, l2
    actual_det = a*d - b*c_entry
    actual_tr = a + d
    disc = actual_tr**2 - 4*actual_det
    if disc < 0:
        a, b, c_entry, d = l1, 0, 0, l2   # fallback to diagonal
    ev_str = f"{l1} and {l2}" if l1 < l2 else f"{l2} and {l1}"
    return Problem(
        topic=TOPIC, subtopic="eigenvalues", difficulty=2,
        question=(
            f"Find the eigenvalues of:\n\n"
            f"  A = [[{l1}, 0], [0, {l2}]]\n\n"
            f"  (Hint: compute det(A − λI) = 0)"
        ),
        answer=f"{min(l1,l2)} and {max(l1,l2)}",
        hint="Set up the characteristic equation det(A − λI) = 0.",
        explanation=(
            f"A − λI = [[{l1}−λ, 0], [0, {l2}−λ]]\n"
            f"det = ({l1}−λ)({l2}−λ) = 0\n"
            f"λ₁ = {l1},  λ₂ = {l2}"
        ),
        problem_type="multiple_choice",
        choices=[
            f"{min(l1,l2)} and {max(l1,l2)}",
            f"{l1+l2} and {l1*l2}",
            f"{l1-1} and {l2+1}",
            f"0 and {l1+l2}",
        ],
    )
