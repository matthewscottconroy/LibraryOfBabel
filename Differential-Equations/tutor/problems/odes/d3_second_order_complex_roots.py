"""
Ordinary Differential Equations — second order homogeneous (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """y'' + 2α·y' + (α²+β²)y = 0  → roots α ± βi"""
    alpha = random.choice([0, 1, 2])
    beta = random.choice([1, 2, 3])
    b = 2 * alpha
    c = alpha**2 + beta**2
    # IVP: y(0)=1, y'(0)=0
    if alpha == 0:
        C1, C2_coeff = 1, 0
        ans = sp.cos(beta*x)
    else:
        # y(0)=1 → C1=1; y'(0)=0 → α·C1 + β·C2 = 0 → C2 = -α/β
        C2_val = sp.Rational(-alpha, beta)
        ans = sp.exp(alpha*x)*(sp.cos(beta*x) + C2_val*sp.sin(beta*x)) if alpha else sp.cos(beta*x)
    return Problem(
        topic=TOPIC, subtopic="second_order_homogeneous", difficulty=3,
        question=(
            f"Solve the IVP:\n\n"
            f"  y'' + {b}y' + {c}y = 0,    y(0) = 1,  y'(0) = 0"
        ),
        answer=ans,
        hint=f"Characteristic roots: r = {-alpha} ± {beta}i. General form: e^(αx)(C₁cos(βx)+C₂sin(βx)).",
        explanation=(
            f"Char. eq: r² + {b}r + {c} = 0\n"
            f"Roots: r = {-alpha} ± {beta}i  (α = {-alpha}, β = {beta})\n"
            f"General: y = e^({-alpha}x)[C₁cos({beta}x) + C₂sin({beta}x)]\n"
            f"y(0)=1 → C₁=1;  y'(0)=0 → solve for C₂\n"
            f"∴ y = {sp.pretty(ans, use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
