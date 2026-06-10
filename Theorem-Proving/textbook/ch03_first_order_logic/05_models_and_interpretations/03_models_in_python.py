"""
First-Order Logic Models in Python
Chapter 3, Section 5

A simple blocks-world model checker demonstrating FOL semantics.
Structures are represented as Python objects; satisfaction is recursive evaluation.
"""

from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Set, Tuple


@dataclass
class BlocksWorld:
    """A simple first-order structure for Tarski's World style reasoning."""
    domain: Set[str]
    # Unary predicates: name -> set of individuals that satisfy it
    predicates: Dict[str, Set[str]] = field(default_factory=dict)
    # Binary relations: name -> set of (a, b) pairs
    relations: Dict[str, Set[Tuple[str, str]]] = field(default_factory=dict)
    # Constants: name -> individual
    constants: Dict[str, str] = field(default_factory=dict)

    def satisfies_atom(self, pred: str, args: List[str], env: Dict[str, str]) -> bool:
        """Check if a ground atomic sentence holds."""
        resolved = [env.get(a, self.constants.get(a, a)) for a in args]
        if len(resolved) == 1:
            return resolved[0] in self.predicates.get(pred, set())
        elif len(resolved) == 2:
            return tuple(resolved) in self.relations.get(pred, set())
        return False

    def eval_formula(self, formula: dict, env: Dict[str, str]) -> bool:
        """Recursively evaluate a formula."""
        match formula['type']:
            case 'atom':
                return self.satisfies_atom(formula['pred'], formula['args'], env)
            case 'eq':
                a = env.get(formula['left'], self.constants.get(formula['left'], formula['left']))
                b = env.get(formula['right'], self.constants.get(formula['right'], formula['right']))
                return a == b
            case 'not':
                return not self.eval_formula(formula['body'], env)
            case 'and':
                return self.eval_formula(formula['left'], env) and self.eval_formula(formula['right'], env)
            case 'or':
                return self.eval_formula(formula['left'], env) or self.eval_formula(formula['right'], env)
            case 'implies':
                return (not self.eval_formula(formula['ant'], env)) or self.eval_formula(formula['con'], env)
            case 'forall':
                x = formula['var']
                return all(self.eval_formula(formula['body'], {**env, x: d}) for d in self.domain)
            case 'exists':
                x = formula['var']
                return any(self.eval_formula(formula['body'], {**env, x: d}) for d in self.domain)
            case _:
                raise ValueError(f"Unknown formula type: {formula['type']}")


def atom(pred, *args):
    return {'type': 'atom', 'pred': pred, 'args': list(args)}

def forall(x, body):
    return {'type': 'forall', 'var': x, 'body': body}

def exists(x, body):
    return {'type': 'exists', 'var': x, 'body': body}

def neg(f):       return {'type': 'not', 'body': f}
def conj(l, r):   return {'type': 'and', 'left': l, 'right': r}
def disj(l, r):   return {'type': 'or', 'left': l, 'right': r}
def impl(a, c):   return {'type': 'implies', 'ant': a, 'con': c}


if __name__ == '__main__':
    # Build a small blocks world: 3 blocks, 2 cubes and 1 tetrahedron
    world = BlocksWorld(
        domain={'a', 'b', 'c'},
        predicates={
            'Cube': {'a', 'b'},
            'Tet':  {'c'},
            'Small': {'a', 'c'},
            'Large': {'b'},
        },
        relations={
            'LeftOf': {('a', 'b'), ('a', 'c'), ('b', 'c')},
            'SameSize': {('a', 'a'), ('a', 'c'), ('c', 'a'), ('b', 'b'), ('c', 'c')},
        },
        constants={'a': 'a', 'b': 'b', 'c': 'c'},
    )

    # ∀x Cube(x)  →  should be False (c is a Tet)
    f1 = forall('x', atom('Cube', 'x'))
    print(f"∀x Cube(x): {world.eval_formula(f1, {})} (expected False)")

    # ∃x Tet(x)  →  should be True
    f2 = exists('x', atom('Tet', 'x'))
    print(f"∃x Tet(x): {world.eval_formula(f2, {})} (expected True)")

    # ∀x(Tet(x) → Small(x))  →  should be True (only c is Tet and c is Small)
    f3 = forall('x', impl(atom('Tet', 'x'), atom('Small', 'x')))
    print(f"∀x(Tet(x)→Small(x)): {world.eval_formula(f3, {})} (expected True)")

    # ∃x∃y(Cube(x) ∧ Cube(y) ∧ ¬(x=y))  →  True (a and b are distinct cubes)
    f4 = exists('x', exists('y', conj(conj(atom('Cube','x'), atom('Cube','y')),
                                      neg({'type':'eq','left':'x','right':'y'}))))
    print(f"∃x∃y(Cube(x)∧Cube(y)∧x≠y): {world.eval_formula(f4, {})} (expected True)")
