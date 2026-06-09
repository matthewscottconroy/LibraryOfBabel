"""
Fourier Analysis — distributions (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "The Dirac delta δ(x) is a function satisfying ∫δ(x)dx = 1.",
            False,
            "False. δ(x) is NOT a function in the classical sense.\n"
            "It is a distribution (generalized function): a continuous linear functional\n"
            "on the space of test functions, defined by ⟨δ, φ⟩ = φ(0)."
        ),
        (
            "The Fourier transform of the Dirac delta is  F{δ(x)} = 1.",
            True,
            "True. By definition: F{δ}(ω) = ∫δ(x)e^{-iωx}dx = e^{0} = 1.\n"
            "Conversely, F{1} = 2πδ(ω)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="distributions", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="δ(x) is a distribution, not a classical function.",
        explanation=expl,
        problem_type="true_false",
    )
