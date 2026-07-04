# Chapter 7: Induction and Recursion

The twin engines of discrete mathematics and functional programming — induction proves properties of infinitely many objects in finitely many steps, and recursion defines functions whose apparent circularity is tamed by well-foundedness.

## Overview

The chapter's central question: when is a definition by recursion well-defined, and how far can induction be generalized beyond $\mathbb{N}$? We begin with the **principle of mathematical induction** — prove $P(0)$, prove $\forall k\,(P(k) \to P(k+1))$, conclude $\forall n\, P(n)$ — and its status as the fifth **Peano axiom**, the minimality condition that characterizes $\mathbb{N}$ up to isomorphism. Worked examples include Gauss's sum $\sum_{i=1}^n i = n(n+1)/2$, the geometric sum, divisibility facts like $3 \mid 4^n - 1$, and inequalities like $2^n > n$, along with classic pitfalls (the all-horses-are-the-same-color fallacy, missing base cases, off-by-one errors, unusable induction hypotheses). **Strong induction** (complete or course-of-values induction) assumes $P(m)$ for all $m \leq k$; it is equivalent in power to weak induction but essential when the recursion reaches back more than one step, as in $F_n < 2^n$ for Fibonacci numbers and the existence of prime factorizations.

**Structural induction** generalizes to inductively defined data — lists, binary trees, formulas: prove $P$ for each constructor, assuming it for all sub-components. Examples: $\mathrm{length}(l +\!\!+\, m) = \mathrm{length}(l) + \mathrm{length}(m)$ and "a binary tree has one more leaf than internal nodes." The deepest generalization is **well-founded induction**: a relation $\prec$ is well-founded iff there is no infinite descending chain (equivalently, every non-empty subset has a $\prec$-minimal element), and then $\forall x\,((\forall y \prec x\, P(y)) \to P(x))$ suffices to conclude $\forall x\, P(x)$ — no separate base case needed. Well-foundedness powers **termination proofs**: the Euclidean algorithm terminates because $a \bmod b < b$; the Ackermann function terminates by the lexicographic order on $\mathbb{N} \times \mathbb{N}$; merge sort by list length; the Collatz conjecture remains open precisely because no measure is known. Ordinal measures and Peano arithmetic's proof-theoretic ordinal $\varepsilon_0$ make a cameo.

On the definitional side, **Dedekind's recursion theorem** guarantees that $f(0) = c$, $f(n+1) = g(n, f(n))$ determines a unique total function — resolving the apparent circularity — and the **well-founded recursion theorem** extends this to any well-founded relation. **Primitive recursion** (factorial, length, reversal) is contrasted with general recursion.

## Why It Matters

Every data type in Lean 4 and Coq is an inductive type; every function on it is defined by structural or well-founded recursion (`termination_by` in Lean supplies the measure); every property is proved with the `induction` tactic. Induction and recursion are two sides of one coin — proving versus defining — identified by Curry-Howard (Chapter 11). The chapter deepens Chapter 5's induction strategy, uses Chapter 6's well-orderings, and feeds Chapter 8 (number-theoretic induction), Chapter 10 (primitive and general recursive functions), and Chapter 13 (verification).

## Chapter Roadmap

1. [Mathematical Induction](01_mathematical_induction/01_principle_of_induction.md) — the induction principle and Peano axioms; further files give a gallery of weak-induction examples, a Lean 4 tactic file, strong induction (Fibonacci bounds, prime factorization), and a Coq companion.
2. [Structural Induction](02_structural_induction/01_induction_on_data_structures.md) — induction on lists and inductive types; companion files prove tree lemmas (leaves vs internal nodes, height bounds) and give a Coq development.
3. [Well-Founded Induction](03_well_founded/01_well_founded_induction.md) — well-founded relations, minimal-counterexample arguments, lexicographic and multiset orders, ordinal measures; the second file applies them to termination proofs (merge sort, Collatz as open problem).
4. [Recursion](04_recursion/01_recursive_definitions.md) — recursive definitions, the recursion theorem, and primitive recursion, with companion Haskell and Python implementations.

## Prerequisites

Chapter 5's treatment of induction as a proof strategy; Chapter 6's ordinals, well-orderings, and set-theoretic machinery behind the recursion theorem. Familiarity with first-order quantifiers (Chapter 3) is assumed throughout.
