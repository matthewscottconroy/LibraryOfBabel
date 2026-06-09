"""
Partial Differential Equations — heat equation (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "pdes"


def generate() -> Problem:
    L = random.choice([1, sp.pi])
    n_val = random.choice([1, 2])
    k = random.choice([1, 2])
    L_str = "π" if L == sp.pi else str(L)
    # u_n = sin(nπx/L) · exp(-k(nπ/L)²t)
    lam = sp.Rational(n_val, 1)**2 * sp.pi**2 / L**2 if L != sp.pi else sp.Integer(n_val**2)
    ans_str = f"sin({n_val}πx/{L_str})·exp(−{k}·({n_val}π/{L_str})²·t)"
    return Problem(
        topic=TOPIC, subtopic="heat_equation", difficulty=2,
        question=(
            f"The heat equation uₜ = {k}uₓₓ on [0, {L_str}] with\n"
            f"u(0,t) = u({L_str},t) = 0.\n\n"
            f"Write the n={n_val} normal mode solution."
        ),
        answer=ans_str,
        hint="Normal modes: uₙ = sin(nπx/L)·exp(−k(nπ/L)²t).",
        explanation=(
            f"Separation of variables gives:\n"
            f"  X'' + λX = 0,  X(0)=X({L_str})=0  →  Xₙ = sin({n_val}πx/{L_str})\n"
            f"  T' + {k}λT = 0  →  Tₙ = exp(−{k}·({n_val}π/{L_str})²·t)\n"
            f"  u_{n_val} = {ans_str}"
        ),
        problem_type="multiple_choice",
        choices=[
            ans_str,
            f"cos({n_val}πx/{L_str})·exp(−{k}({n_val}π/{L_str})²t)",
            f"sin({n_val}πx/{L_str})·exp(+{k}({n_val}π/{L_str})²t)",
            f"sin({n_val}x)·exp(−{k}{n_val}²t)",
        ],
    )
