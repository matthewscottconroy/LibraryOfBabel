# SAT and SMT Solvers

## Overview
Modern **SAT solvers** decide propositional satisfiability in practice with remarkable
efficiency (despite NP-completeness in theory). **SMT solvers** (Satisfiability Modulo
Theories) combine SAT with decision procedures for theories (linear arithmetic, arrays,
bitvectors, strings). Both are indispensable tools in formal verification.

## Learning Objectives
- Understand how DPLL and CDCL work (conceptually)
- Use Z3 from Python to solve logical problems
- Encode problems as SMT constraints

## The DPLL Algorithm (1960)
Davis-Putnam-Logemann-Loveland: systematic backtracking search over truth assignments.
Key operations: unit propagation, pure literal elimination, branching.

## CDCL: Conflict-Driven Clause Learning
Modern SAT solvers use CDCL (Chaff 2001, MiniSat):
- Unit propagation
- **Conflict analysis**: when a contradiction is found, learn a new clause from the conflict
  (non-chronological backjumping)
- **VSIDS heuristic**: variable ordering based on recent conflict involvement

This makes industrial SAT solvers handle millions of variables in practice.

## SMT: Theories Built on SAT
SMT solvers combine a CDCL-based SAT engine with decision procedures for specific theories:
- **Linear arithmetic** (LIA, LRA): ax + by ≤ c constraints
- **Arrays**: read/write axioms
- **Bitvectors**: fixed-width machine arithmetic
- **Uninterpreted functions** (EUF): equality with function symbols
- **Strings**: string operations and regular expressions

## Z3 in Python
```python
from z3 import *

x, y = Ints('x y')
s = Solver()
s.add(x + y == 10, x > 3, y > 3)
if s.check() == sat:
    print(s.model())   # x=4, y=6 (or similar)
```

See `textbook/ch13_formal_verification_and_applications/04_applied_logic/02_sat_solver_in_python.py`

## Applications
- Hardware verification: every CPU design goes through a formal equivalence checking step
- Software verification: Dafny, F*, Whiley use Z3 as their backend solver
- Security: symbolic execution tools (KLEE, angr) use SMT for path constraints
- Planning and scheduling: constraint satisfaction encoded as SMT
- Cryptanalysis: SAT attacks on stream ciphers

## Exercises
See `problems/ch13_applications/02_sat_encoding_exercises.md`
