# Important Thinkers: Chapter 9

## Per Martin-Löf (1942–): The Philosopher-Logician

We have mentioned Martin-Löf before, but his contribution to Chapter 9 is direct enough to warrant a fuller account. Martin-Löf's deepest contribution is not a theorem but a *framework*: the idea that mathematical knowledge has a specific structure that can be formalized, and that this structure is not classical logic but constructive type theory.

Martin-Löf is a philosophical realist about mathematics — he believes mathematical objects exist independently of our constructions — but a methodological constructivist about proof. For him, a proof must demonstrate how to construct the object it claims to exist. The identity type is part of this constructivism: equality must be witnessed, not just asserted.

His 1984 Bibliopolis lectures are remarkable documents. They present not just the formal rules but an extended philosophical justification for each rule, based on what he calls "meaning explanations" — accounts of what the judgments mean in terms of construction and verification. No other foundational system has this combination of formal precision and philosophical depth.

His distinction between the *intensional* and *extensional* identity type was not motivated by homotopy theory — that came later — but by the question of what proof equality means. Does knowing that a = b (propositionally) tell you *how* a equals b? Martin-Löf said yes: the proof carries the information. This was the seed of HoTT, planted without knowing what would grow.

## Martin Hofmann (1966–2011) and Thomas Streicher (1958–)

The 1994 paper "The Groupoid Interpretation of Type Theory" by Martin Hofmann and Thomas Streicher is one of the most consequential in the history of type theory. It showed two things simultaneously:

First: UIP (Uniqueness of Identity Proofs) is not provable from J. The proof is by constructing a model — the groupoid model — in which all the rules of intensional MLTT hold but UIP fails.

Second: the groupoid model provides a semantic interpretation in which types are groupoids, elements are objects, and identity proofs are morphisms. This is the first geometric interpretation of the identity type.

Hofmann died tragically in a mountaineering accident in 2011 at age 44, before seeing the full development of HoTT. His other contributions include work on categorical models of type theory, linear logic, and game semantics. Streicher remains active and has contributed extensively to category-theoretic semantics of type theory.

## Vladimir Voevodsky (1966–2017)

If Martin-Löf planted the seed and Hofmann-Streicher watered it, Voevodsky made the garden. His 2006 discovery (at the Institute for Advanced Study) of the homotopy interpretation of type theory — that types are infinity-groupoids, identity proofs are paths, and equivalences are homotopy equivalences — transformed a technical observation about groupoids into a new foundation for mathematics.

Voevodsky was a Fields Medalist (2002) for his work on motivic cohomology and the proof of the Milnor conjecture. He came to type theory through a concern about the reliability of mathematical proof: he had found errors in published proofs (including proofs of his own theorems) that had gone unnoticed for years. He concluded that informal mathematical proof is not reliable enough for the level of complexity of modern mathematics, and that formalization was necessary.

His solution was the Univalent Foundations program: develop mathematics in MLTT with the Univalence Axiom, formalized in Coq. The Coq formalization would constitute a certified proof that the mathematics was correct.

Voevodsky's formalization project (UniMath) is ongoing. His 2010 Coq lectures at IAS and his contributions to the HoTT Book (as one of its primary authors) are the most direct sources for the homotopy interpretation.

He died in 2017 at age 51. The HoTT community continues his program.

## Thierry Coquand and Cubical Type Theory

Voevodsky's Univalence Axiom has one defect: it is an *axiom*. It is consistent with intensional MLTT but not a *theorem* of it. This means that programs extracted from proofs using univalence may not actually compute — the univalence axiom lacks a computation rule.

Thierry Coquand (with Simon Huber, Cyril Cohen, Anders Mörtberg, and others) addressed this by developing *cubical type theory*, a computational interpretation of HoTT in which univalence has a computation rule. In cubical type theory, the interval [0,1] is a primitive type, paths are functions from the interval to types, and the computation rules for transport and the J rule are derived from the geometry of cubes.

Coquand's Cubical Agda (built on cubical type theory) is the current state of the art for doing HoTT with a fully computational system — one in which every proof gives a computable program. This resolves the "missing computation rule" problem for univalence.

## Christine Paulin-Mohring and Inductive Types

Christine Paulin-Mohring's contribution to the FIEC framework is concrete: she worked out how to add inductive type definitions to the Calculus of Constructions in a controlled way, creating the Calculus of Inductive Constructions (CIC) that underlies Coq. Her key insight was the positivity condition: inductive types are consistent (and their recursors are well-typed and terminating) precisely when the type being defined appears only in positive positions in constructor arguments.

This technical condition is what allows all the inductive types of Section 2 — ℕ, List, Vec, W-types — to be defined safely, without introducing inconsistency. The positivity checker in Coq and Agda implements her condition.

## Robert Constable and the NuPRL Team

Robert Constable led the NuPRL project at Cornell, which produced the first major proof assistant based on type theory. NuPRL uses *extensional* type theory (with the reflection rule), making type-checking undecidable but allowing more flexible mathematical reasoning.

The NuPRL tradition has contributed significantly to the foundations of type theory, particularly in understanding the semantic content of type-theoretic equality and in developing the *propositions-as-types* principle in its full generality. Stuart Allen's refinement type theory (underlying NuPRL4) and the PRL group's work on computational type theory are important alternatives to the intensional MLTT tradition.

Constable's book *Implementing Mathematics with the Nuprl Proof Development System* (with collaborators, 1986) was the first book-length treatment of a working proof assistant based on type theory.
