# Direct Proof

## Overview
A **direct proof** of P → Q assumes P and derives Q through a chain of valid inferences.
It is the default proof strategy: when you do not know which strategy to use, try direct proof first.

## Learning Objectives
- State the method of direct proof
- Write direct proofs for conditional claims
- Identify when a direct proof is (un)natural

## Method
To prove P → Q:
1. Assume P (introduce the antecedent as a hypothesis)
2. Use definitions, axioms, previously proved theorems, and inference rules
3. Derive Q

## Example 1: If n is even, then n² is even
**Proof**: Assume n is even. Then n = 2k for some integer k. So n² = (2k)² = 4k² = 2(2k²).
Since 2k² is an integer, n² = 2(2k²) is even. □

## Example 2: If a | b and b | c, then a | c (divisibility is transitive)
**Proof**: Assume a | b and b | c. Then b = am and c = bn for some integers m, n.
Thus c = bn = (am)n = a(mn). Since mn is an integer, a | c. □

## When Direct Proof Is Difficult
Direct proof fails when the hypothesis P does not give you enough to work with directly.
This is a signal to try:
- Proof by contrapositive (for conditionals about divisibility, parity)
- Proof by contradiction (for irrationality, non-existence claims)
- Proof by cases (when the hypothesis splits naturally)

## Lean 4
See `textbook/ch05_proof_strategies/01_direct_proof/02_direct_proof_in_lean.lean`

## Exercises
See `problems/ch05_proof_strategies/02_direct_proof_exercises.md`
