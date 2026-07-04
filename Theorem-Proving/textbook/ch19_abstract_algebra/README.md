# Chapter 19: Abstract Algebra and Logic

Groups, rings, fields, and lattices — the algebraic structures that underlie logic, number theory, and geometry. And, running the other direction, the logic that makes algebra rigorous — until logic itself becomes an algebra.

## Overview

This chapter treats algebra as applied logic. The **group axioms** — associativity, identity, inverses over the signature $(\cdot, {}^{-1}, e)$ — form a purely *equational* first-order theory, and the first theorems of group theory (uniqueness of identity and inverses, the cancellation laws, $(ab)^{-1} = b^{-1}a^{-1}$) are literal derivations in equational logic. From subgroups, homomorphisms, kernels, and **normal subgroups** we build quotient groups and prove the **first isomorphism theorem** and **Lagrange's theorem** (the order of a subgroup divides the order of the group, via cosets). **Ring and field axioms** follow the same pattern: ideals and quotient rings, integral domains and zero divisors, the characteristic of a ring, and the theorem that $\mathbb{Z}/p\mathbb{Z}$ is a field exactly when $p$ is prime.

Logic then draws a striking decidability map. The word problem for finitely presented groups is undecidable (**Novikov–Boone**), and the first-order theory of groups is undecidable (Tarski; Mal'cev) — yet the theory of *abelian* groups is decidable (**Szmielew, 1955**). The theory of algebraically closed fields (**ACF**) is complete and decidable by quantifier elimination; the reals form a decidable **real closed field** (**Tarski's 1951 decision procedure for RCF**, hence for elementary geometry); **Presburger arithmetic** $(\mathbb{Z}; +)$ is decidable while $(\mathbb{Z}; +, \cdot)$ is undecidable, and Hilbert's tenth problem is unsolvable (**Matiyasevich**).

Finally, algebra absorbs logic itself. Lattices have two equivalent definitions (order-theoretic and equational); **distributive and modular lattices** are characterized by the forbidden sublattices $M_3$ and $N_5$ (Birkhoff); complete lattices carry the **Knaster–Tarski fixed-point theorem**. The **Lindenbaum–Tarski algebra** turns propositional logic into a **Boolean algebra**, yielding an algebraic completeness proof via ultrafilters and the **Stone representation theorem**, while **Heyting algebras** play the same role for intuitionistic logic. **Universal algebra** generalizes everything: varieties, free algebras, **Birkhoff's completeness theorem for equational logic**, the **HSP theorem** (why fields have no equational axiomatization), and **Knuth–Bendix completion** for term rewriting. In Lean 4's Mathlib, the algebraic hierarchy lives as typeclasses, and tactics like `ring`, `abel`, and `group` are decision procedures for equational fragments.

## Why It Matters

Algebra is the proving ground where logic's big questions get concrete answers: which theories are complete, which are decidable, and what a decision procedure looks like in practice. The boundary between Presburger arithmetic and full arithmetic, or between abelian and arbitrary groups, is the working border of automated reasoning — the same border every SMT solver and every Lean tactic navigates. And the Lindenbaum–Tarski construction shows the two subjects are one: a logic *is* an algebra, and completeness *is* representation.

## Chapter Roadmap

1. [Groups](01_groups/01_groups.md) — group axioms as an equational theory; equationally proved basic theorems; quotients, isomorphism theorems, Lagrange; undecidability of the theory of groups vs. Szmielew's decidability for abelian groups; the Novikov–Boone word problem.
2. [Rings and Fields](02_rings/01_rings_and_fields.md) — ring axioms, ideals, quotient rings; domains, fields, characteristic; $\mathbb{Z}/p\mathbb{Z}$; the decidability map: ACF, Tarski's RCF, Presburger, Hilbert's tenth problem.
3. [Lattices and Boolean Algebras](03_lattices/01_lattices.md) — lattices as posets and as algebras (with the equivalence proof); $M_3$, $N_5$, distributivity; Knaster–Tarski; Boolean algebras, the Lindenbaum–Tarski algebra, Stone representation, Heyting algebras.
4. [Universal Algebra and Equational Logic](04_universal/01_varieties.md) — signatures, congruences, free algebras; Birkhoff's completeness theorem; the HSP theorem; term rewriting and Knuth–Bendix completion.
5. [Algebra in Lean](05_lean/01_algebra_in_lean.md) — the Mathlib typeclass hierarchy; a formal cancellation proof; `ring`, `abel`, `group`, `omega` as decision procedures; the Feit–Thompson formalization.

## Prerequisites

- **Chapter 3** (first-order logic): signatures, theories, models.
- **Chapter 6** (set theory): relations, equivalence classes, partitions.
- **Chapter 8** (number theory): Bézout's identity, modular arithmetic.
- **Chapter 9** (model theory, helpful): completeness, categoricity, quantifier elimination, the Łoś–Vaught test.
