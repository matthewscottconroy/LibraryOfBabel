"""
Propositional Resolution Theorem Prover
Chapter 4, Section 3

Implements refutation-resolution for propositional logic.
A formula is a theorem iff its negation is unsatisfiable (derives □).
"""

from typing import FrozenSet, List, Optional, Set


Clause = FrozenSet[str]  # literals; negation is represented as "~p"


def negate(lit: str) -> str:
    return lit[1:] if lit.startswith('~') else '~' + lit


def resolve(c1: Clause, c2: Clause) -> Optional[Clause]:
    """Try to resolve two clauses. Returns resolvent or None if no resolution possible."""
    for lit in c1:
        if negate(lit) in c2:
            return frozenset((c1 - {lit}) | (c2 - {negate(lit)}))
    return None


def resolution_refutation(clauses: List[Clause]) -> bool:
    """
    Return True iff the clause set is unsatisfiable (refutation exists).
    Uses a simple set-of-support strategy.
    """
    clause_set: Set[Clause] = set(clauses)
    new: Set[Clause] = set()

    while True:
        pairs = [(c1, c2) for c1 in clause_set for c2 in clause_set if c1 != c2]
        for c1, c2 in pairs:
            resolvent = resolve(c1, c2)
            if resolvent is None:
                continue
            if frozenset() in [resolvent]:  # empty clause = contradiction
                return True
            new.add(resolvent)
        if new.issubset(clause_set):
            return False  # no new clauses derivable; satisfiable
        clause_set |= new


def prove(premises: List[List[str]], conclusion: List[str]) -> bool:
    """
    Prove that conclusion follows from premises by refuting premises + negation(conclusion).
    Premises and conclusion are lists of clauses (each clause is a list of literals).
    """
    all_clauses = [frozenset(c) for c in premises + [conclusion]]
    return resolution_refutation(all_clauses)


if __name__ == '__main__':
    # Example 1: Modus Ponens
    # Premises: {p}, {~p, q}  Conclusion: {q}
    # Negate conclusion: {~q}
    print("Modus Ponens:")
    result = resolution_refutation([
        frozenset(['p']),
        frozenset(['~p', 'q']),
        frozenset(['~q']),  # negated conclusion
    ])
    print(f"  Refutation found (= valid): {result}")  # expected: True

    # Example 2: Invalid argument
    # Premises: {p -> q} = {~p, q}, Conclusion: {p}  (affirming consequent)
    print("Affirming the Consequent (invalid):")
    result = resolution_refutation([
        frozenset(['~p', 'q']),
        frozenset(['q']),
        frozenset(['~p']),  # negated conclusion (¬p)
    ])
    print(f"  Refutation found (= valid): {result}")  # expected: False

    # Example 3: Hypothetical Syllogism
    # {~p,q}, {~q,r}, {p} |- {r}
    print("Hypothetical Syllogism:")
    result = resolution_refutation([
        frozenset(['~p', 'q']),
        frozenset(['~q', 'r']),
        frozenset(['p']),
        frozenset(['~r']),  # negated conclusion
    ])
    print(f"  Refutation found (= valid): {result}")  # expected: True
