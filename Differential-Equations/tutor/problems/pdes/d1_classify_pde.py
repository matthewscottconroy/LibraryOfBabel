"""
Partial Differential Equations — classification (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "pdes"


def generate() -> Problem:
    case, classification, expl = random.choice([
        (
            "uₜ = k·uₓₓ  (heat / diffusion equation)",
            "Parabolic",
            "B²−4AC = 0−4(k)(0) = 0. Parabolic.\n"
            "Models diffusion; solutions smooth out over time."
        ),
        (
            "uₜₜ = c²·uₓₓ  (wave equation)",
            "Hyperbolic",
            "B²−4AC = 0−4(c²)(−1) = 4c² > 0. Hyperbolic.\n"
            "Models wave propagation; has characteristic lines."
        ),
        (
            "uₓₓ + uᵧᵧ = 0  (Laplace equation)",
            "Elliptic",
            "B²−4AC = 0−4(1)(1) = −4 < 0. Elliptic.\n"
            "Models steady states; solutions are harmonic functions."
        ),
        (
            "uₓₓ + 2uₓᵧ + uᵧᵧ = 0",
            "Parabolic",
            "A=1, B=2, C=1. B²−4AC = 4−4 = 0. Parabolic.\n"
            "(Note: A+C form: (∂/∂x + ∂/∂y)²u = 0 — degenerate.)"
        ),
        (
            "uₓₓ − uₓᵧ − 2uᵧᵧ = 0",
            "Hyperbolic",
            "A=1, B=−1, C=−2. B²−4AC = 1+8 = 9 > 0. Hyperbolic."
        ),
    ])
    choices = ["Elliptic", "Parabolic", "Hyperbolic"]
    return Problem(
        topic=TOPIC, subtopic="classification", difficulty=1,
        question=f"Classify the second-order PDE:\n\n  {case}",
        answer=classification,
        hint="Compute discriminant B²−4AC. Negative: elliptic; zero: parabolic; positive: hyperbolic.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
