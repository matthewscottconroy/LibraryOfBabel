# Chapter 6: Set Theory

A single primitive relation — membership, $\in$ — suffices, in principle, to encode every object of mathematics: numbers, functions, relations, spaces. This chapter builds that foundation and confronts the paradox that nearly destroyed it.

## Overview

The chapter's central question: can a few axioms provide a coherent foundation for all of mathematics without collapsing into inconsistency? We begin with naive set theory — membership ($x \in A$), roster and set-builder notation, the empty set $\emptyset$, and the **Axiom of Extensionality** (sets are equal iff they have the same elements), with equality proved by double inclusion. Then the crisis: **Russell's paradox** (1901). Unrestricted comprehension lets us form $R = \{x \mid x \notin x\}$, whence $R \in R \iff R \notin R$ — the contradiction that broke Frege's *Grundgesetze* and forced the axiomatic responses surveyed here: Zermelo's restricted **Separation**, Russell's type theory, and NBG's proper classes.

The working toolkit follows: union, intersection, complement, and difference obeying **De Morgan's laws** and forming a Boolean algebra; ordered pairs via the **Kuratowski encoding** $(a,b) = \{\{a\},\{a,b\}\}$ and Cartesian products; and the power set $\mathcal{P}(A)$ with **Cantor's theorem** ($|A| < |\mathcal{P}(A)|$, by the diagonal set $D = \{x \in A \mid x \notin f(x)\}$). Binary relations $R \subseteq A \times B$, classified by reflexivity, symmetry, antisymmetry, and transitivity, yield **equivalence relations** (with the partition theorem, e.g. congruence mod $n$) and **partial orders** (posets, lattices, well-orders). Functions are total, single-valued relations; injections, surjections, and bijections govern composition, inverses, and cardinality.

Cardinality is the chapter's climax: the pigeonhole principle for finite sets; countable sets ($\mathbb{Z}$, $\mathbb{Q}$, $\mathbb{N} \times \mathbb{N}$ via Cantor pairing); the uncountability of $\mathbb{R}$ by **Cantor's diagonal argument**; the hierarchy $\aleph_0 < 2^{\aleph_0} = \mathfrak{c}$; and cardinals versus ordinals ($\omega$, $\omega + 1$, $\varepsilon_0$). Finally, the ten **ZFC axioms** (Extensionality, Empty Set, Pairing, Union, Power Set, Separation, Replacement, Infinity, Foundation, Choice), the cumulative hierarchy $V_\alpha$, the **Axiom of Choice** with its equivalents (**Zorn's Lemma**, the **Well-Ordering Theorem**) and strange consequences (Banach-Tarski), and the independence of the **Continuum Hypothesis** from ZFC (Gödel 1938, Cohen 1963).

## Why It Matters

ZFC is the standard foundation for formal mathematics, and this chapter supplies vocabulary the rest of the book assumes: well-orderings and ordinals underlie well-founded induction (Chapter 7); model-theoretic structures (Chapter 9) are sets; forcing and incompleteness (Chapter 10) live inside set theory; type theory (Chapter 11) is the rival foundation that blocks Russell's paradox at the type level. In Lean 4 and Coq, `Set α` is the predicate type $\alpha \to \mathrm{Prop}$, and the universe hierarchy plays ZF's role of forbidding a set of all sets.

## Chapter Roadmap

1. [Naive Set Theory](01_naive_set_theory/01_sets_elements_notation.md) — membership, notation, the empty set, and extensionality; further files cover subsets and double-inclusion proofs, then Russell's paradox and its resolutions (Separation, type theory, NBG).
2. [Set Operations](02_set_operations/01_union_and_intersection.md) — union and intersection as a Boolean algebra; companion files treat complement/difference and De Morgan's laws, Cartesian products via Kuratowski pairs, the power set with Cantor's theorem, plus a Python file.
3. [Relations](03_relations/01_binary_relations.md) — binary relations as subsets of $A \times B$; later files classify relation properties and closures, develop equivalence relations and partitions, and study partial orders, lattices, and well-orders.
4. [Functions](04_functions/01_functions_as_relations.md) — functions as total single-valued relations; further files cover injections/surjections/bijections, composition and (left/right) inverses, and a Haskell companion.
5. [Cardinality](05_cardinality/01_finite_sets.md) — finite counting and the pigeonhole principle; subsequent files prove countability of $\mathbb{Q}$, uncountability of $\mathbb{R}$ by diagonalization, and contrast cardinals with ordinals.
6. [Axiomatic Set Theory](06_axiomatic_set_theory/01_zf_axioms.md) — the ZF axioms one by one and the cumulative hierarchy; the second file examines the Axiom of Choice, its equivalents, and Banach-Tarski.
7. [Sets in Lean and Coq](07_sets_in_lean_and_coq/01_sets_in_lean4.lean) — sets as predicates ($\alpha \to \mathrm{Prop}$) in Lean 4, with a parallel Coq development.

## Prerequisites

Chapters 1–3: first-order logic, since every ZF axiom is a first-order sentence over $\in$. Chapter 5's strategies are used throughout — double inclusion is direct proof, while Russell's paradox and Cantor's theorem are proofs by contradiction via diagonalization.
