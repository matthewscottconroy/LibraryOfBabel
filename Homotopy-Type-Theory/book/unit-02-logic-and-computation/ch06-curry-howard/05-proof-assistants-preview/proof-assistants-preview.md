# Proof Assistants: A Preview

## The Dream of Formalized Mathematics

For as long as there have been formal proof systems, mathematicians have dreamed of a machine that could check whether a proof is valid. The dream became conceivable in the 1960s, when computers became powerful enough to represent and manipulate symbolic expressions. It became a reality — partly — through the Curry-Howard correspondence.

The correspondence gives the key insight: a type checker *is* a proof checker. If proofs are terms and propositions are types, then checking whether a term has a given type is exactly the same computational problem as checking whether a proof establishes a given proposition. And type checking, unlike proof search, is decidable (for well-designed type theories) — it can be done mechanically, in finite time, with a correct algorithm.

This is what makes proof assistants possible. Lean 4, Coq, Agda, and Isabelle are not just clever programs that search for proofs. They are implementations of the Curry-Howard correspondence: the user writes a term (a proof), and the type checker verifies that the term has the claimed type (proves the claimed proposition). No oracle is needed. No mathematical intuition is required of the machine. Only mechanical type checking.

## Automath: The First Proof Assistant

The first system to implement the propositions-as-types idea was *Automath*, developed by Nicolaas Govert de Bruijn at Eindhoven starting in 1967. De Bruijn, a mathematician working on combinatorics and analysis, wanted a system in which entire books of mathematics could be formalized and checked mechanically.

Automath's design was directly inspired by the observation that mathematical proofs have a tree structure that can be encoded as typed terms. De Bruijn developed a notation for dependent types (which he called "imbedded" or "extended") and showed that large portions of classical mathematics — including significant parts of analysis — could be formalized in Automath.

The system was years ahead of its time. De Bruijn also invented the *de Bruijn index* convention for avoiding variable capture in lambda calculus (replacing named variables with numbers representing how many binders away the variable is declared), which is still used in modern proof assistant implementations.

Automath was not widely adopted, partly because computers were expensive and partly because the mathematical community was not yet ready for formalization. But it established the technical foundations and the conceptual framework that all subsequent proof assistants built on.

## Coq: The Calculus of Inductive Constructions

The Coq proof assistant (now called Rocq) was developed at INRIA starting in the late 1980s by Thierry Coquand and Gérard Huet, based on the *Calculus of Constructions* (CoC) that Coquand introduced in his doctoral thesis. The system was later extended to the *Calculus of Inductive Constructions* (CIC), adding inductive types for natural numbers, lists, trees, and other recursive data structures.

Coq implements the Curry-Howard correspondence at industrial scale. A Coq proof is a term of the CIC type theory, and the kernel is a type checker that verifies the term has the claimed type. The proof term can be enormous (millions of nodes for a significant result), but the type checker is small and highly trustworthy.

The most famous result formalized in Coq is the proof of the *Four Color Theorem* by Gonthier and Werner (2005). The classical 1976 Appel-Haken proof was the first to use a computer for mathematical verification, but it was not a formal proof in any strict sense — it was an informal proof that reduced the problem to a computer case analysis. The Gonthier-Werner proof is a complete formal proof in Coq, verified mechanically from first principles, that every planar map can be colored with four colors. It is approximately 60,000 lines of Coq.

## Agda: Dependent Types for Functional Programmers

Agda is a proof assistant developed at Chalmers University of Technology, based on Martin-Löf Type Theory with a focus on *pattern matching* and *termination checking*. Agda's type system is closer to MLTT than Coq's is, and it is the primary implementation language for HoTT research: the HoTT book was accompanied by a large Agda library formalizing its results.

Agda's design philosophy emphasizes transparency: the user writes programs (which are proofs) in a high-level functional programming style, and the system verifies their types (which are propositions). Pattern matching on inductive types is first-class. Termination is checked by a structural recursion checker (ensuring all recursive calls are on structurally smaller arguments, guaranteeing termination).

HoTT formalization in Agda uses a variant called *Homotopy Type Theory Agda* (HoTT-Agda), which postulates the univalence axiom and provides higher inductive types. The formalization validates that the mathematical content of the HoTT book is type-theoretically sound.

## Lean 4: The Industrial Proof Assistant

Lean 4, developed by Leonardo de Moura and collaborators at AWS and Microsoft Research, is the most recent major proof assistant and is designed with industrial applicability in mind. Its type theory is a dependent type theory with classical axioms optionally available, universe polymorphism, quotient types, and a highly efficient kernel.

The *Mathlib4* library for Lean 4 is currently the largest formalized mathematics library in the world, containing tens of thousands of theorems across algebra, analysis, topology, category theory, and number theory. It demonstrates that the Curry-Howard correspondence can scale to the full breadth of mathematics at the research frontier.

Lean 4 has been used to formalize Fermat's Last Theorem (in ongoing work by Kevin Buzzard's team), the classification of finite simple groups (preliminary steps), and numerous research papers in algebraic geometry and number theory. The formalization of the *Liquid Tensor Experiment* — a key lemma from condensed mathematics due to Peter Scholze — was a landmark: it formalized a result that Scholze himself described as one of the most technically challenging proofs he had written, and the formalization revealed a simpler structure than the original proof suggested.

## The Flyspeck Project: Formalizing Hales' Proof

Thomas Hales proved the Kepler Conjecture in 1998 — that the densest packing of equal spheres in three dimensions is the face-centered cubic packing. The proof used a combination of formal mathematics and computer-assisted verification of thousands of cases, and it was initially controversial: the Annals of Mathematics reviewers could not verify the computer portion within a reasonable time.

Hales responded by launching the *Flyspeck Project*: a formal verification of the entire proof in Isabelle and HOL Light. The project was completed in 2014, producing a machine-verified proof of the Kepler Conjecture — the first major open problem in geometry to be settled by formal verification.

The Flyspeck project illustrates both the power and the challenge of formalized mathematics. The power: once formalized, there is no doubt. The challenge: formalizing the proof required 20 person-years of effort and resulted in a formalized proof much larger than the original informal proof. The formalization found minor errors in the original proof (all correctable) and clarified several arguments.

## What It Means to Formalize Mathematics

To formalize a mathematical theorem in a proof assistant is to:

1. **Define all objects precisely**: natural numbers, real numbers, groups, topological spaces — all defined from first principles in the type theory, not assumed.
2. **State the theorem exactly**: as a type whose inhabitants would be proofs.
3. **Construct a term of that type**: a proof, written in the language of the type theory, that is verified mechanically by the type checker.

Every step requires bridging the gap between informal mathematical intuition ("it is clear that...") and formal type-theoretic expression ("here is the exact term"). This is nontrivial. Mathematicians rely heavily on context, convention, and shared background. Formalizing requires making all of this explicit.

But the rewards are substantial:
- **Absolute correctness**: a verified proof cannot contain undetected errors.
- **Reusability**: a formalized lemma is available to all future proofs in the library.
- **Program extraction**: constructive proofs yield verified programs automatically.
- **Collaboration at scale**: a large library like Mathlib can be maintained by hundreds of contributors, with the type checker as the arbiter of correctness.

## HoTT in Proof Assistants

HoTT formalization requires proof assistants that support the univalence axiom and higher inductive types. Current options:

- **HoTT-Agda**: postulates univalence, supports HITs by declaration. The HoTT book's formalization is here.
- **Coq + HoTT library**: similar to HoTT-Agda, postulates univalence.
- **Cubical Agda**: implements cubical type theory, giving a *definitional* (computational) interpretation of univalence rather than an axiom. This is the gold standard: every computation in cubical type theory terminates, and univalence holds by computation, not by fiat.
- **Lean 4 + HoTT**: still developing, but Lean's universe polymorphism and higher-order unification make it well-suited.

Cubical Agda is particularly significant because it validates the computational content of HoTT: in cubical type theory, every proof is a program, and every program terminates. The univalence axiom, far from being an addition that breaks computation, is a theorem about the structure of identity types that can be computed. This gives the strongest possible implementation of the Curry-Howard correspondence for HoTT.

The connection between proof assistants and HoTT is not incidental — it is essential. HoTT is a foundation for formalized mathematics, and proof assistants are its implementation. The long-term goal of the univalent foundations program is a system in which all of mathematics can be formalized, verified, and computed in a unified type-theoretic framework. The preview in this section is just that: a preview. The rest of the curriculum is the mathematical content that makes this goal achievable.
