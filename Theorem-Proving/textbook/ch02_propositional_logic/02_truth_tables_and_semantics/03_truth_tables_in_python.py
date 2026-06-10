"""
Truth Tables in Python
Chapter 2, Section 2

Uses itertools and sympy to build truth tables programmatically.
"""

from itertools import product
from sympy.logic.boolalg import And, Or, Not, Implies, Equivalent, truth_table
from sympy.abc import p, q, r


def print_truth_table(formula, variables):
    """Print a truth table for the given sympy formula."""
    headers = [str(v) for v in variables] + [str(formula)]
    print(' | '.join(headers))
    print('-' * (len(' | '.join(headers))))
    for row in truth_table(formula, variables):
        values = row[0] + (row[1],)
        print(' | '.join(str(int(v)) for v in values))


if __name__ == '__main__':
    print("=== Modus Ponens: (p & (p->q)) -> q ===")
    mp = Implies(And(p, Implies(p, q)), q)
    print_truth_table(mp, [p, q])

    print()
    print("=== De Morgan: ~(p & q) <-> (~p | ~q) ===")
    dem = Equivalent(Not(And(p, q)), Or(Not(p), Not(q)))
    print_truth_table(dem, [p, q])

    print()
    print("=== Exclusive Or (XOR): (p | q) & ~(p & q) ===")
    xor = And(Or(p, q), Not(And(p, q)))
    print_truth_table(xor, [p, q])

    print()
    print("=== Exercise: Is (p->q) -> ((q->r) -> (p->r)) a tautology? ===")
    hyp_syl = Implies(Implies(p, q), Implies(Implies(q, r), Implies(p, r)))
    print_truth_table(hyp_syl, [p, q, r])
