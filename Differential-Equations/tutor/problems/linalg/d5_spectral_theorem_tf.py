"""
Linear Algebra — eigentheory (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "Every real symmetric matrix has real eigenvalues.",
            True,
            "True. The Spectral Theorem guarantees that real symmetric matrices\n"
            "have only real eigenvalues and are orthogonally diagonalizable."
        ),
        (
            "A matrix A and its transpose Aᵀ always have the same eigenvalues.",
            True,
            "True. det(A−λI) = det((A−λI)ᵀ) = det(Aᵀ−λI), so characteristic\n"
            "polynomials are identical."
        ),
        (
            "If A is invertible then 0 is not an eigenvalue of A.",
            True,
            "True. λ=0 is an eigenvalue iff det(A)=0 iff A is singular.\n"
            "So A invertible ↔ 0 is not an eigenvalue."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="eigentheory", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Think about the relationship between det(A−λI) and properties of eigenvalues.",
        explanation=expl,
        problem_type="true_false",
    )
