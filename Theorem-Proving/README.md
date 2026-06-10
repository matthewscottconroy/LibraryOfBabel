# Language, Proof, and Logic

A comprehensive textbook project integrating philosophy, logic, and mathematics as a
unified subject, with formal tools throughout.

## Structure

```
.
├── textbook/     Chapters covering language, logic, proof theory, set theory,
│                 number theory, type theory, modal logic, and verification.
├── proofs/       Example proofs in multiple formats (paper, Lean 4, Coq, Python, Haskell).
└── problems/     Exercises, assignments, and challenges organized by chapter.
```

## Tools Used

| Tool | Purpose |
|------|---------|
| **Tarski's World / Carnap** | FOL semantics, blocks-world models (carnap.io — free) |
| **Lean 4 + Mathlib** | Proof assistant; dependent type theory |
| **Coq** | Proof assistant; calculus of inductive constructions |
| **Python (Z3, sympy)** | Computational exploration, SAT/SMT solving |
| **Haskell** | Functional programming; types-as-propositions |
| **Z3** | SAT/SMT solver (via Python or direct API) |

## Chapter Overview

| Chapter | Topic |
|---------|-------|
| 01 | Language and Logic Foundations |
| 02 | Propositional Logic |
| 03 | First-Order Logic |
| 04 | Proof Systems |
| 05 | Proof Strategies |
| 06 | Set Theory |
| 07 | Induction and Recursion |
| 08 | Number Theory |
| 09 | Model Theory |
| 10 | Computability and Incompleteness |
| 11 | Type Theory and Curry-Howard |
| 12 | Modal and Philosophical Logic |
| 13 | Formal Verification and Applications |

## Getting Started

1. **Install Lean 4**: `curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh`
2. **Install Coq**: via `opam` or system package manager
3. **Install Python tools**: `pip install z3-solver sympy`
4. **Install Haskell**: via `ghcup`; add `QuickCheck` via `cabal`
5. **Carnap** (Tarski's World alternative): visit carnap.io — no installation needed

## Philosophy

This textbook treats logic, proof, and language as a *unified subject* rather than
separate disciplines. Every chapter connects:
- The **philosophical** question (what is meaning? what is proof? what is truth?)
- The **mathematical** formalism (logic, type theory, set theory)
- The **computational** embodiment (proof assistants, SAT solvers, functional programs)
- The **real-world application** (verification, cryptography, AI, databases)

Proofs are not mere exercises — they are the foundational technology of reliable software,
verified hardware, and secure cryptography.
