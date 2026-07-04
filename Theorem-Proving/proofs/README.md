# Proofs

This directory contains formal and informal proofs of key mathematical theorems,
each organized in its own subdirectory. Each proof directory contains:

- `README.md`: theorem statement, informal sketch, and notes
- `paper_proof.md`: full pen-and-paper proof
- `lean_proof.lean` (where present): Lean 4 formalization
- `coq_proof.v` (where present): Coq formalization
- `python_verify.py` (where applicable): computational verification or demo

## Organization

```
proofs/
├── 01_propositional_logic/    modus ponens, De Morgan, tautologies
├── 02_predicate_logic/        quantifier laws, syllogisms
├── 03_set_theory/             Cantor, Schröder-Bernstein, Russell
├── 04_number_theory/          primes, sqrt(2), FTA, Fermat
├── 05_induction/              sum formulas, tree properties
├── 06_analysis/               IVT, uncountability of ℝ
├── 07_combinatorics/          pigeonhole, inclusion-exclusion
├── 08_temporal_logic/         LTL safety and recurrence for a two-state system
├── 09_mereology/              overlap implies product in GEM
└── 10_category_theory/        free group universal property (adjunction)
```
