"""
SAT and SMT Solving in Python with Z3
Chapter 13, Section 4

Demonstrates using the Z3 SMT solver for:
- Propositional satisfiability
- Linear arithmetic constraints
- Puzzle solving via constraint encoding
"""

from z3 import (
    Bool, Int, Real, Solver, sat, unsat, And, Or, Not, Implies,
    Distinct, Sum, If
)


# ── Propositional SAT ─────────────────────────────────────────────────────────

def demo_propositional():
    p, q, r = Bool('p'), Bool('q'), Bool('r')
    s = Solver()

    # Is (p -> q) /\ (q -> r) /\ p /\ ~r satisfiable?
    s.add(Implies(p, q))
    s.add(Implies(q, r))
    s.add(p)
    s.add(Not(r))

    result = s.check()
    print(f"(p→q)∧(q→r)∧p∧¬r satisfiable? {result}")  # unsat (it's a valid argument!)


# ── Checking tautologies ──────────────────────────────────────────────────────

def is_tautology(formula):
    """A formula is a tautology iff its negation is unsatisfiable."""
    s = Solver()
    s.add(Not(formula))
    return s.check() == unsat


def demo_tautologies():
    p, q = Bool('p'), Bool('q')
    formulas = [
        ("p ∨ ¬p", Or(p, Not(p))),
        ("p → p", Implies(p, p)),
        ("(p → q) → (¬q → ¬p)", Implies(Implies(p,q), Implies(Not(q), Not(p)))),
        ("p → q  (not a tautology)", Implies(p, q)),
    ]
    print("Tautology check:")
    for name, f in formulas:
        print(f"  {name}: {is_tautology(f)}")


# ── Integer arithmetic ────────────────────────────────────────────────────────

def demo_integer():
    x, y, z = Int('x'), Int('y'), Int('z')
    s = Solver()
    s.add(x + y + z == 100)
    s.add(x > 0, y > 0, z > 0)
    s.add(x < y, y < z)
    s.add(x * 2 == y)   # y = 2x

    if s.check() == sat:
        m = s.model()
        print(f"Integer solution: x={m[x]}, y={m[y]}, z={m[z]}")
        print(f"  Check: {m[x]}+{m[y]}+{m[z]}={int(str(m[x]))+int(str(m[y]))+int(str(m[z]))}")


# ── N-Queens puzzle ───────────────────────────────────────────────────────────

def n_queens(n: int):
    """Solve the N-Queens problem using Z3."""
    # queens[i] = column of queen in row i
    queens = [Int(f'q_{i}') for i in range(n)]
    s = Solver()

    # Each queen in a valid column
    for q in queens:
        s.add(q >= 0, q < n)

    # No two queens in the same column
    s.add(Distinct(queens))

    # No two queens on the same diagonal
    for i in range(n):
        for j in range(i + 1, n):
            s.add(queens[i] - queens[j] != i - j)
            s.add(queens[i] - queens[j] != j - i)

    if s.check() == sat:
        m = s.model()
        board = [['.'] * n for _ in range(n)]
        for i, q in enumerate(queens):
            board[i][int(str(m[q]))] = 'Q'
        print(f"{n}-Queens solution:")
        for row in board:
            print('  ' + ' '.join(row))
    else:
        print(f"No solution for {n}-Queens")


if __name__ == '__main__':
    print("=== Propositional SAT ===")
    demo_propositional()
    print()
    print("=== Tautology Checking ===")
    demo_tautologies()
    print()
    print("=== Integer Arithmetic ===")
    demo_integer()
    print()
    print("=== 6-Queens ===")
    n_queens(6)
