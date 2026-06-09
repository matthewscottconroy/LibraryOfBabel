"""
Fourier Analysis — fourier transform (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    prop, correct, expl = random.choice([
        (
            "F{f'(x)}  (Fourier transform of the derivative)",
            "iω · F{f}(ω)",
            "Differentiation in time domain multiplies by iω in frequency domain:\n"
            "F{f'(x)} = iω · F̂(ω)."
        ),
        (
            "F{f(x−a)}  (time shift)",
            "e^(−iωa) · F̂(ω)",
            "Time-shifting by a multiplies the transform by e^(-iωa) (phase shift):\n"
            "F{f(x−a)}(ω) = e^(-iωa) F̂(ω)."
        ),
        (
            "F{f(x)·g(x)}  (product in time domain)",
            "(1/2π)(F̂ * Ĝ)(ω)  (convolution in frequency domain)",
            "Convolution theorem: multiplication in time ↔ convolution in frequency.\n"
            "F{fg} = (1/2π) F̂ * Ĝ."
        ),
        (
            "F{(x·f(x))}  (multiplication by x)",
            "i · d/dω [F̂(ω)]",
            "Multiplication by x in the time domain corresponds to\n"
            "differentiation in the frequency domain: F{xf(x)} = i·dF̂/dω."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="fourier_transform", difficulty=3,
        question=f"What is:\n\n  {prop}",
        answer=correct,
        hint="Recall the Fourier transform operational properties: shift, derivative, convolution.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "F̂(ω)/iω", "ω²·F̂(ω)", "F̂(ω+a)"],
    )
