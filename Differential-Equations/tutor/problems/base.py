"""Problem dataclass and answer-checking logic."""

from __future__ import annotations

import concurrent.futures
from dataclasses import dataclass, field
from typing import Any, Optional

import sympy as sp

# Standard symbols available for parsing user answers
_x, _y, _z, _t, _n = sp.symbols("x y z t n")
_LOCALS: dict[str, Any] = {
    "x": _x, "y": _y, "z": _z, "t": _t, "n": _n,
    "exp": sp.exp, "sin": sp.sin, "cos": sp.cos,
    "tan": sp.tan, "sec": sp.sec, "csc": sp.csc, "cot": sp.cot,
    "asin": sp.asin, "acos": sp.acos, "atan": sp.atan,
    "sinh": sp.sinh, "cosh": sp.cosh, "tanh": sp.tanh,
    "ln": sp.log, "log": sp.log, "log2": lambda a: sp.log(a, 2),
    "sqrt": sp.sqrt, "Abs": sp.Abs, "abs": sp.Abs,
    "pi": sp.pi, "e": sp.E, "oo": sp.oo, "inf": sp.oo,
    "I": sp.I, "C1": sp.Symbol("C1"), "C2": sp.Symbol("C2"),
}
_TRANSFORMS = sp.parsing.sympy_parser.standard_transformations + (
    sp.parsing.sympy_parser.implicit_multiplication_application,
    sp.parsing.sympy_parser.convert_xor,
)

_SYMPY_TIMEOUT = 5.0   # seconds before giving up on a simplification


def _parse(s: str) -> sp.Expr:
    s = s.strip()
    s = s.replace("e^(", "exp(").replace("e^", "exp(")
    open_p = s.count("(")
    close_p = s.count(")")
    if open_p > close_p:
        s += ")" * (open_p - close_p)
    return sp.parse_expr(s, local_dict=_LOCALS, transformations=_TRANSFORMS)


def _try_fn(fn, expr) -> Optional[sp.Expr]:
    """Run a sympy simplification with a timeout; return None on timeout/error."""
    try:
        with concurrent.futures.ThreadPoolExecutor(max_workers=1) as ex:
            fut = ex.submit(fn, expr)
            return fut.result(timeout=_SYMPY_TIMEOUT)
    except Exception:
        return None


def _equiv(user: sp.Expr, correct: sp.Expr) -> bool:
    """True if two sympy expressions are mathematically equal."""
    diff = user - correct
    for fn in (
        sp.simplify, sp.expand, sp.trigsimp, sp.radsimp,
        sp.factor, sp.cancel, sp.powsimp,
        lambda e: sp.logcombine(e, force=True),
        sp.combsimp,
    ):
        try:
            result = _try_fn(fn, diff)
            if result is not None and result == 0:
                return True
        except Exception:
            pass
    return False


@dataclass
class Problem:
    topic: str
    subtopic: str
    difficulty: int
    question: str
    answer: Any
    hint: str
    explanation: str
    problem_type: str      # "symbolic" | "multiple_choice" | "true_false" | "numeric"
    choices: Optional[list[str]] = None
    _choice_letter: str = field(default="", repr=False)   # correct letter for MC
    _answer_str: str = field(default="", repr=False)

    # ── Display ───────────────────────────────────────────────────────────────

    def answer_display(self) -> str:
        if self.problem_type == "multiple_choice":
            ans_str = str(self.answer)
            if self.choices:
                for i, ch in enumerate(self.choices):
                    if ch.lower() == ans_str.lower():
                        letter = chr(ord("A") + i)
                        self._choice_letter = letter
                        return f"{letter}. {ch}"
            return ans_str
        if self._answer_str:
            return self._answer_str
        if self.problem_type == "symbolic" and isinstance(self.answer, sp.Basic):
            return str(self.answer)
        if self.problem_type == "true_false":
            return "True" if self.answer else "False"
        return str(self.answer)

    # ── Checking ──────────────────────────────────────────────────────────────

    def check_answer(self, raw: str) -> tuple[bool, str]:
        raw = raw.strip()
        if not raw:
            return False, "Please enter an answer."

        if self.problem_type == "true_false":
            return self._check_tf(raw), ""
        if self.problem_type == "multiple_choice":
            return self._check_mc(raw), ""
        if self.problem_type == "numeric":
            return self._check_numeric(raw)
        if self.problem_type == "symbolic":
            return self._check_symbolic(raw)
        return False, "Unknown problem type."

    def _check_tf(self, s: str) -> bool:
        sl = s.lower()
        if sl in ("t", "true", "yes", "1", "y"):
            user = True
        elif sl in ("f", "false", "no", "0", "n"):
            user = False
        else:
            return False
        return user == self.answer

    def _check_mc(self, s: str) -> bool:
        sl = s.strip().lower()
        if len(sl) == 1 and sl.isalpha():
            idx = ord(sl) - ord("a")
            if self.choices and 0 <= idx < len(self.choices):
                return self.choices[idx].lower() == str(self.answer).lower()
        # Accept full text
        return sl == str(self.answer).lower()

    def _check_numeric(self, s: str) -> tuple[bool, str]:
        try:
            val = float(sp.sympify(s))
            correct = float(self.answer)
            tol = max(1e-9, abs(correct) * 1e-4)
            return abs(val - correct) <= tol, ""
        except Exception:
            return False, f"Could not parse '{s}' as a number."

    def _check_symbolic(self, s: str) -> tuple[bool, str]:
        try:
            user_expr = _parse(s)
        except Exception:
            return False, (
                f"Could not parse '{s}'. "
                "Use Python syntax: ** for powers, exp() for e^x, log() for ln."
            )
        try:
            ok = _equiv(user_expr, self.answer)
            return ok, ""
        except Exception:
            return False, "Could not verify — try simplifying your expression."
