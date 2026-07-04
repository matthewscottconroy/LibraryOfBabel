# Chapter 13: Formal Verification and Applications

The payoff chapter: the logic developed throughout this book, deployed to prove programs, circuits, protocols, and mathematics itself correct — with a machine checking every step.

## Overview

Program verification starts from formal specifications and the distinction between **partial correctness** $\{P\}\ C\ \{Q\}$ and **total correctness** $[P]\ C\ [Q]$. **Hoare logic** (Hoare 1969) derives triples by rules for assignment ($\{P[E/x]\}\ x := E\ \{P\}$), sequencing, conditionals, and while loops — where the creative step is finding a **loop invariant** $I$ satisfying $\{I \land B\}\ C\ \{I\}$ — plus the consequence rule; the system is sound and, by Cook's theorem, relatively complete. Dijkstra's **weakest precondition calculus** turns verification into calculation: $\{P\}\ C\ \{Q\}$ iff $P \Rightarrow \mathrm{wp}(C, Q)$, with $\mathrm{wp}$ a predicate transformer computed backwards through the program — the engine of verification-condition generators (Dafny, Why3, Frama-C) backed by SMT solvers. The chapter also introduces **separation logic** (points-to $e_1 \mapsto e_2$, separating conjunction $P * Q$, and the frame rule) for modular reasoning about the heap.

Hardware verification applies the same logic to circuits: **equivalence checking** (is there an input on which two circuits differ? — a SAT query), motivated by the \$475M Pentium FDIV recall, and **model checking**: a system is a Kripke structure $\mathcal{M} = (S, S_0, R, L)$, a specification is a CTL formula ($AG\, p$, $EF\, p$, $AG(p \to AF\, q)$), and the Clarke–Emerson–Sifakis labeling algorithm decides $\mathcal{M} \vDash \phi$ by fixpoint computation in $O(|S| \cdot |\phi|)$. The state-explosion problem is met by BDD-based **symbolic model checking** (McMillan), SAT-based **bounded model checking**, and **CEGAR** abstraction refinement — now industrial practice from Intel's FPUs to AWS's TLA+ specifications.

The remaining sections turn to mathematics itself and to applied logic. Why formalize: the Kepler conjecture and Flyspeck, Gonthier's Coq proof of the four-color theorem, the Liquid Tensor Experiment in Lean. **Lean 4 with Mathlib** (over a million lines; tactics `simp`, `ring`, `linarith`, `omega`, `aesop`, `decide`) and **Coq/Rocq** with its standard library and Mathematical Components (the 170,000-line Feit–Thompson odd-order formalization) both implement the Calculus of Inductive Constructions with a small trusted kernel. Applied logic covers **SAT solvers** (DPLL to CDCL with clause learning and VSIDS) and **SMT solvers** (SAT plus theories: linear arithmetic, arrays, bitvectors, uninterpreted functions — Z3 from Python); logic in machine learning (neural-network verification, Markov logic networks, inductive logic programming); cryptography (RSA correctness via Euler's theorem, zero-knowledge proofs, protocol verifiers ProVerif and Tamarin, lattice-based post-quantum security via LWE); databases (Codd's relational model, SQL as first-order logic, Datalog, query containment); and hardware design (Shannon's Boolean-algebra-as-circuits, ATPG as SAT).

## Why It Matters

This is the book's capstone. Hoare triples instantiate Chapter 4's proof systems over program states; model checking evaluates Chapter 12's (and Chapter 14's) temporal formulas over Kripke structures; Lean and Coq implement Chapter 11's dependent type theory; SAT revisits Chapter 2's satisfiability. And Chapter 10's Rice's theorem explains the field's shape: full automation is impossible, so verification combines decidable procedures (SAT/SMT, model checking) with human-supplied invariants and interactive proof.

## Chapter Roadmap

1. [Program Verification](01_program_verification/01_program_correctness.md) — specifications, partial vs. total correctness; further files develop the Hoare rules with a verified integer-division example, the weakest-precondition calculus, and Hoare logic in Coq.
2. [Hardware Verification](02_hardware_verification/01_circuit_verification.md) — RTL and gate-level verification, SAT-based equivalence checking; a second file on CTL model checking, BDDs, bounded model checking, and CEGAR.
3. [Mathematics Formalization](03_mathematics_formalization/01_why_formalize.md) — why machine-check proofs (Flyspeck, four-color, Liquid Tensor); overviews of Lean 4's Mathlib and Coq's standard library with MathComp.
4. [Applied Logic](04_applied_logic/01_sat_and_smt_solvers.md) — DPLL/CDCL SAT and SMT solving with Z3; companions include a Python SAT solver and files on logic in machine learning, cryptography, databases, and hardware design.

## Prerequisites

First-order logic and proof systems (Chapters 3–5) for Hoare logic; modal and temporal logic (Chapter 12) for model checking; type theory (Chapter 11) for the Lean/Coq material; computability (Chapter 10) for the limits of automation.
