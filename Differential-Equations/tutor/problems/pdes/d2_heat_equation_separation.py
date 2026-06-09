"""
Partial Differential Equations — separation of variables (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "pdes"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "u(x,t) = X(x)T(t) is substituted into uₜ = k·uₓₓ",
            "X·T' = k·X''·T  →  T'/kT = X''/X = −λ",
            "Substituting: X(x)T'(t) = k·X''(x)T(t).\n"
            "Divide by kXT: T'/(kT) = X''/X = −λ (separation constant).\n"
            "Each side depends on different variables, so both equal −λ."
        ),
        (
            "With homogeneous Dirichlet BCs X(0) = X(L) = 0 for X'' + λX = 0",
            "λₙ = (nπ/L)² and Xₙ = sin(nπx/L)",
            "The Sturm-Liouville eigenvalue problem X'' + λX = 0, X(0)=X(L)=0\n"
            "has eigenvalues λₙ = (nπ/L)² with eigenfunctions sin(nπx/L), n=1,2,3,…"
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="separation_of_variables", difficulty=2,
        question=f"Separation of variables for the heat equation:\n\n  {case}",
        answer=correct,
        hint="Assume u = X(x)T(t) and divide by XT to separate variables.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct,
                 "X·T' = k·X·T  →  T'/kT = 1",
                 "No separation is possible",
                 "λ must always equal zero"],
    )
