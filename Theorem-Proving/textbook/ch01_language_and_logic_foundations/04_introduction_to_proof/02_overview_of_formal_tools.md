# Overview of Formal Tools

## Overview
This section introduces the software tools used throughout this textbook. Each tool
embodies different aspects of formal logic and proof, and they complement each other.

## Tarski's World / Language Proof and Logic (LPL)
**What it is**: Software accompanying Barwise & Etchemendy's textbook *Language, Proof and Logic*.
Includes Tarski's World (blocks-world FOL models), Fitch (natural deduction checker), and Boole.
**Modern alternative**: Carnap (carnap.io) — free, web-based, supports FOL, propositional logic,
  natural deduction, and truth tables. Open source.
**What you learn**: FOL semantics, natural deduction, the relationship between syntax and models.

## Lean 4
**What it is**: A proof assistant and functional programming language based on dependent type
theory. Actively developed; has Mathlib (a large library of formalized mathematics).
**Install**: `curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh`
**What you learn**: Dependent types, tactic-based proving, large-scale mathematics formalization.

## Coq
**What it is**: The oldest major proof assistant still in active use. Based on the Calculus of
Inductive Constructions. Mature ecosystem; used in major verification projects (CompCert, etc.).
**Install**: Available via system package managers; `opam` is recommended.
**What you learn**: CIC type theory, tactics, inductive types, program extraction.

## Python
**What it is**: Used for computational explorations — truth table generators, SAT solving (via Z3),
Turing machine simulators, combinatorial verification.
**Key libraries**: `z3-solver`, `sympy.logic`, `nltk`, `lark`

## Haskell
**What it is**: A purely functional language with a strong type system. Proofs-as-programs ideas
are visible in everyday Haskell; equational reasoning is natural.
**What you learn**: Type classes, pattern matching, algebraic data types, lazy evaluation,
  the connection between functional programs and logical proofs.

## Other Tools
- **Isabelle/HOL**: another major proof assistant; strong automation via Sledgehammer
- **Agda**: dependently typed; syntax close to mathematical notation; used in PL research
- **Z3**: SMT solver by Microsoft Research; callable from Python, Haskell, and many languages
- **Alloy**: lightweight formal modeling tool for relational specifications
- **TLA+**: temporal logic of actions; used for distributed systems verification (AWS, Azure)

## Exercises
See `problems/ch01_language_and_logic_foundations/` for getting-started exercises.
