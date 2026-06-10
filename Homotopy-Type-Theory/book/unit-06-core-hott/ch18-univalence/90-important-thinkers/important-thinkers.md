# Important Thinkers: Univalence

## Vladimir Voevodsky (1966–2017)

Voevodsky is the originator of the Univalence Axiom, and his story is worth telling in some detail, because it illuminates how the axiom arose and why it matters.

Voevodsky won the Fields Medal in 2002 for his work on motivic cohomology — a theory that applies homotopy theory to algebraic geometry. His proofs were so complex and technically demanding that he himself was not certain they were correct. He discovered errors in some of his earlier work after the fact, and he came to believe that informal mathematical proofs were fundamentally unreliable at the necessary level of complexity.

This concern drove him toward formal verification. He began learning Coq and asked: what is the right foundation for formalizing modern algebraic geometry and homotopy theory? He found that ZFC set theory, while technically adequate, was poorly suited to the mathematics he was trying to formalize — it was too low-level, too set-theoretic, too far from the homotopy-theoretic intuitions that guide the mathematics.

Around 2005-2006, he realized that Martin-Lof type theory, with the homotopy interpretation of identity types, provided a much better fit. Types are spaces. Paths are proofs. Higher paths are coherences. This is the language of homotopy theory.

The Univalence Axiom was his key contribution to this program: the formal statement that equivalent types are equal, capturing the mathematical principle that has always guided homotopy-theoretic mathematics.

His proof of consistency via the simplicial set model was a technical tour de force, drawing on the full machinery of modern algebraic topology and category theory. It established that the Univalence Axiom is not just intuitively correct but formally consistent.

Voevodsky spent the last decade of his life on the Voevodsky Mathematical Library (a Coq library implementing HoTT) and the UniMath project, which aims to formalize large parts of mathematics in univalent foundations. He died unexpectedly in 2017 at age 51.

## Thierry Coquand and Cubical Type Theory

Coquand's work on cubical type theory (developed with collaborators Marc Bezem, Simon Huber, and Anders Mortberg) transformed the Univalence Axiom from a mysterious axiom into a computable theorem.

Cubical type theory introduces an explicit interval type I with endpoints 0 and 1, and Kan operations (composition and filling of cubes) as computation rules. From these rules, Univalence can be *proved* — the function `ua` is explicitly constructed and satisfies its computation rules definitionally.

This is a profound advance: it means that in cubical type theory, proofs using Univalence are *computable* — they can be run as programs, not just verified as proofs. The abstraction of "equivalent types are equal" becomes a concrete computational procedure.

## Steve Awodey

Awodey's work bridging category theory and type theory provided much of the conceptual framework for understanding Univalence. His 2012 paper "Type Theory and Homotopy" gave an accessible account of why identity types should be interpreted as path spaces and why Univalence is the natural axiom to add.

Awodey also contributed to the philosophical case for Univalence, arguing that it provides the formal expression of the mathematical principle of "structural invariance" — the idea that mathematicians never use non-structural properties of their objects (properties that would not be preserved by isomorphism). Univalence makes this principle a theorem.

## Peter Aczel, Benedikt Ahrens, and the UniMath Project

The UniMath project, initiated by Voevodsky and now maintained by a team including Ahrens and others, is the ongoing effort to develop a large library of mathematics in Coq based on univalent foundations. The library contains significant portions of category theory, algebra, and set theory, all formalized in a way that is automatically invariant under equivalence.

Ahrens's contributions include the development of "displayed categories" — a technique for building categories by adding structure level by level, using the Sigma-type path characterization and Univalence to ensure that the resulting categories are well-behaved.

## The IAS Special Year 2012-2013

The HoTT Book itself was the product of a special year at the Institute for Advanced Study in Princeton, organized by Voevodsky, Awodey, and others. Over 40 researchers spent the year developing the foundations and applications of homotopy type theory together. The collaborative writing of the HoTT Book — published simultaneously in print and as an open-access document — is one of the landmarks of modern mathematical collaboration.

The special year produced not just the book but also significant advances in synthetic homotopy theory (Chapter 20), the formalization of higher algebra, and the foundations of modular mathematics in HoTT.
