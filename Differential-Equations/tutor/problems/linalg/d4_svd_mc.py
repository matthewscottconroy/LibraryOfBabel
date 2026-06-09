"""
Linear Algebra — svd (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "The singular values of a matrix A are defined as:",
            "The square roots of the eigenvalues of AᵀA",
            "Singular values σᵢ = √λᵢ where λᵢ are eigenvalues of AᵀA (which are ≥ 0).\n"
            "SVD: A = UΣVᵀ where U, V are orthogonal and Σ is diagonal with σᵢ ≥ 0."
        ),
        (
            "The rank of a matrix A equals:",
            "The number of non-zero singular values",
            "The rank equals the number of non-zero singular values in the SVD.\n"
            "This gives a numerically stable way to compute rank."
        ),
        (
            "For a rank-k approximation (low-rank approximation) of matrix A, the best choice is:",
            "The sum of the first k terms of the SVD: A_k = Σᵢ₌₁ᵏ σᵢ uᵢvᵢᵀ",
            "Eckart-Young theorem: the best rank-k approximation in the Frobenius norm\n"
            "is given by truncating the SVD to the top k singular values/vectors."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="svd", difficulty=4,
        question=f"Singular Value Decomposition (SVD):\n\n  {case}",
        answer=correct,
        hint="SVD: A = UΣVᵀ. Singular values are square roots of eigenvalues of AᵀA.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "The eigenvalues of A", "The diagonal entries of A",
                 "The square roots of eigenvalues of AAᵀ + AᵀA"],
    )
