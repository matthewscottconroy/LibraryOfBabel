"""
Ordinary Differential Equations — undetermined coefficients (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """y'' + by' + cy = k·e^(mx)  (no resonance: m not a root)"""
    # roots r1, r2 chosen to be != m
    r1, r2 = -2, -3
    m = random.choice([0, 1, 2])
    k = random.choice([1, 2, 3, 4])
    b_c = -(r1+r2)  # 5
    c_c = r1*r2     # 6
    # Particular: yp = A·e^(mx)
    # m²A + b_c*m*A + c_c*A = k → A(m²+b_c*m+c_c)=k
    denom = m**2 + b_c*m + c_c
    if denom == 0:
        m = 4   # avoid resonance
        denom = m**2 + b_c*m + c_c
    A = sp.Rational(k, denom)
    yp = A * sp.exp(m*x)
    # IVP: y(0)=0, y'(0)=0
    # General: yh = C1*e^(r1*x) + C2*e^(r2*x)
    # y(0): A + C1 + C2 = 0
    # y'(0): m*A + r1*C1 + r2*C2 = 0
    M = sp.Matrix([[1,1],[r1,r2]])
    rhs = sp.Matrix([-A, -m*A])
    C = M.solve(rhs)
    ans = C[0]*sp.exp(r1*x) + C[1]*sp.exp(r2*x) + yp
    return Problem(
        topic=TOPIC, subtopic="undetermined_coefficients", difficulty=4,
        question=(
            f"Solve the IVP:\n\n"
            f"  y'' + {b_c}y' + {c_c}y = {k}e^({m}x),    y(0)=0, y'(0)=0"
        ),
        answer=sp.expand(ans),
        hint=f"Particular solution: try yₚ = Ae^({m}x). Then solve for C₁, C₂ using ICs.",
        explanation=(
            f"Characteristic roots: r = {r1}, {r2}\n"
            f"Particular: yₚ = Ae^({m}x).  A({m}²+{b_c}·{m}+{c_c}) = {k}\n"
            f"→ A = {k}/{denom} = {A}\n"
            f"General: y = C₁e^({r1}x) + C₂e^({r2}x) + {A}e^({m}x)\n"
            f"Apply ICs → C₁={C[0]}, C₂={C[1]}\n"
            f"∴ y = {sp.pretty(sp.expand(ans), use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
