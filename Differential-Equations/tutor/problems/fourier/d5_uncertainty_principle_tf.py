"""
Fourier Analysis — fourier transform (difficulty 5).
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
            "The Heisenberg uncertainty principle in signal processing states:\n"
            "  Δt · Δω ≥ 1/2 (time-bandwidth product is bounded below).",
            True,
            "True. A signal cannot be simultaneously concentrated in both\n"
            "time and frequency. Mathematically: σ_t · σ_ω ≥ 1/2,\n"
            "where σ_t and σ_ω are standard deviations in time and frequency domains."
        ),
        (
            "The Gaussian function f(t) = e^(-t²) achieves the minimum time-bandwidth product.",
            True,
            "True. The Gaussian is the unique minimizer of the Heisenberg uncertainty inequality.\n"
            "F{e^(-t²)} = √π·e^(-ω²/4), and σ_t·σ_ω = 1/2 (equality case)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="fourier_transform", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="The Heisenberg uncertainty principle is a mathematical theorem about Fourier transform pairs.",
        explanation=expl,
        problem_type="true_false",
    )
