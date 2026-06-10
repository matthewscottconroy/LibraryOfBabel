# References: The Curry-Howard Correspondence

## Primary Sources

**Howard, W.A. (1980). The Formulae-as-Types Notion of Construction.** In J.P. Seldin and J.R. Hindley (eds.), *To H.B. Curry: Essays on Combinatory Logic, Lambda Calculus and Formalism*, Academic Press, 479–490. Originally circulated as a 1969 manuscript. The foundational document of the Curry-Howard correspondence as a precise theorem. Short, clear, essential.

**Curry, H.B. and Feys, R. (1958). *Combinatory Logic, Vol. I*.** North-Holland. The systematic treatment of combinatory logic in which Curry noted the correspondence between combinator types and logical axioms. Chapters 7–9 are most relevant.

**Martin-Löf, P. (1984). *Intuitionistic Type Theory*.** Bibliopolis, Naples. (Notes from 1980 Padova lectures.) Establishes the full dependent type theory (MLTT) with $\Pi$, $\Sigma$, and identity types. The introduction ("On the Meanings of the Logical Constants and the Justifications of the Logical Laws") is the philosophical cornerstone.

**Coquand, T. and Huet, G. (1988). The Calculus of Constructions.** *Information and Computation*, 76(2-3), 95–120. The foundational paper for the type theory underlying Coq. Defines CoC, proves its properties, and establishes it as a foundation for both programming and theorem proving.

## Textbooks and Survey Articles

**Sorensen, M.H. and Urzyczyn, P. (2006). *Lectures on the Curry-Howard Isomorphism*.** Studies in Logic and the Foundations of Mathematics, Vol. 149. Elsevier. The most comprehensive textbook treatment of the Curry-Howard correspondence, covering propositional logic, system F, dependent types, and extensions. Mathematically rigorous. Chapters 1–6 are directly relevant here.

**Girard, J.-Y., Lafont, Y., and Taylor, P. (1989). *Proofs and Types*.** Cambridge University Press. (Available free online.) An accessible introduction to the correspondence, written by one of its major developers. Part I (Chapters 1–5) covers STLC and the basic correspondence. Part II covers System F. Girard's commentary is invaluable.

**Thompson, S. (1991). *Type Theory and Functional Programming*.** Addison-Wesley. An introduction to Martin-Löf Type Theory aimed at computer scientists, with a focus on program development and the Curry-Howard correspondence. Accessible and practical.

**Pierce, B.C. (2002). *Types and Programming Languages*.** MIT Press. The standard reference for type theory in the programming languages tradition. Chapters 1–15 cover STLC, product types, sum types, normalization, and the metatheory. Chapter 29 covers type operators (a bridge to System F).

## For Proof Assistants

**The Coq Development Team. The Coq Reference Manual.** Available at coq.inria.fr. The definitive technical reference for Coq's type theory (the Calculus of Inductive Constructions) and proof language.

**The Lean 4 Community. Mathematics in Lean.** Available at leanprover-community.github.io. An accessible introduction to formalizing mathematics in Lean 4, with a focus on Mathlib.

**Gonthier, G. (2008). Formal Proof: The Four-Color Theorem.** *Notices of the AMS*, 55(11), 1382–1393. A readable account of the Coq formalization of the Four Color Theorem, accessible to mathematicians without type theory background.

**Hales, T. et al. (2017). A Formal Proof of the Kepler Conjecture.** *Forum of Mathematics, Pi*, 5, e2. The Flyspeck project results: a machine-verified proof of Kepler's conjecture in Isabelle/HOL and HOL Light.

## For the Connection to HoTT

**Univalent Foundations Program. (2013). *Homotopy Type Theory: Univalent Foundations of Mathematics*.** Institute for Advanced Study. Available at homotopytypetheory.org. Chapters 1–2 develop the basic type theory (MLTT + univalence) in detail. The Curry-Howard perspective is present throughout.

**Barendregt, H. (1992). Lambda Calculi with Types.** In S. Abramsky, D.M. Gabbay, and T.S.E. Maibaum (eds.), *Handbook of Logic in Computer Science, Vol. 2*. Clarendon Press, 117–309. The definitive survey of the lambda cube and the spectrum of type theories from STLC to CoC. Technical but comprehensive.
