# References: STLC and System F

## Primary Sources

**Church, A. (1940). A Formulation of the Simple Theory of Types.** *Journal of Symbolic Logic*, 5(2), 56–68. The original type theory for lambda calculus, introducing simple types (function types from base types) and the foundations of what becomes STLC. Historical context: Church was trying to provide a consistent foundation for his lambda-calculus-based set theory after it was shown inconsistent in its untyped form.

**Girard, J.-Y. (1971). Une extension de l'interprétation de Gödel à l'analyse, et son application à l'élimination des coupures dans l'analyse et la théorie des types.** In J.E. Fenstad (ed.), *Proceedings of the Second Scandinavian Logic Symposium*, North-Holland, 63–92. The original System F paper, introducing second-order polymorphism and proving strong normalization via candidates of reducibility. Girard also uses this to prove the consistency of second-order arithmetic.

**Reynolds, J.C. (1974). Towards a Theory of Type Structure.** In B. Robinet (ed.), *Colloque sur la Programmation*, Lecture Notes in Computer Science 19, Springer, 408–425. Reynolds' independent discovery of System F in the context of programming languages. More accessible than Girard's paper.

**Reynolds, J.C. (1983). Types, Abstraction, and Parametric Polymorphism.** In R.E.A. Mason (ed.), *Information Processing 83*, North-Holland, 513–523. The foundational paper on parametricity. Introduces the logical relations interpretation of polymorphism and derives free theorems from type signatures. Essential.

**Damas, L. and Milner, R. (1982). Principal Type-Schemes for Functional Programs.** *POPL '82*, 207–212. Proves the completeness of Algorithm W for type inference in Hindley-Milner (rank-1 polymorphic) type theory. Short and clean.

## Textbooks

**Pierce, B.C. (2002). *Types and Programming Languages*.** MIT Press. The standard undergraduate/graduate textbook for programming language type theory. Chapters 1–15 cover STLC, product types, sum types, and metatheory (progress and preservation). Chapter 23 covers System F. Chapter 29 covers type operators and kinds. Excellent exercises.

**Pierce, B.C. (ed.) (2005). *Advanced Topics in Types and Programming Languages*.** MIT Press. A collection of advanced topics including dependent types (Chapter 2), type theory for type systems designers (Chapter 1), and more. Complements the basic textbook.

**Barendregt, H. (1984). *The Lambda Calculus: Its Syntax and Semantics*. (Revised edition.)** North-Holland. The comprehensive reference for the untyped lambda calculus. Chapters 1–3 cover syntax and reduction. Chapters 4–8 cover models (domain theory). Chapters 14–16 cover types.

**Girard, J.-Y., Lafont, Y., and Taylor, P. (1989). *Proofs and Types*.** Cambridge University Press. (Available free online.) The most accessible treatment of System F and the Curry-Howard correspondence, written by the inventor of System F. Part II (Chapters 10–14) covers System F in detail.

## For Parametricity

**Wadler, P. (1989). Theorems for Free!** *FPCA '89*, 347–359. A famous and accessible paper on free theorems: Reynolds' parametricity made concrete for programmers. Many examples of types whose inhabitants are forced to behave in specific ways. Essential reading.

**Atkey, R. (2012). Relational Parametricity for Higher Kinds.** *CSL '12*, 46–61. Extends Reynolds' parametricity to System F$\omega$ and higher-kinded type systems. Technical but comprehensive.

## For the Lambda Cube and CoC

**Barendregt, H. (1992). Lambda Calculi with Types.** In S. Abramsky, D.M. Gabbay, and T.S.E. Maibaum (eds.), *Handbook of Logic in Computer Science, Vol. 2*. Clarendon Press, 117–309. The lambda cube is introduced and all eight systems are analyzed. The definitive survey.

**Coquand, T. and Huet, G. (1988). The Calculus of Constructions.** *Information and Computation*, 76(2-3), 95–120. The original CoC paper. Establishes CoC as a foundation for programming and theorem proving.

**Luo, Z. (1994). *Computation and Reasoning: A Type Theory for Computer Science*.** Clarendon Press. An accessible treatment of Extended Calculus of Constructions (ECC), which extends CoC with universes and inductive types. Bridges the lambda cube to MLTT.

## For HoTT Connection

**Voevodsky, V. (2010). Univalent Foundations Project.** Available online. The original manifesto for the univalent foundations program, explaining why HoTT requires a new type-theoretic foundation and how System F and CoC fit into the picture.

**Univalent Foundations Program. (2013). *Homotopy Type Theory: Univalent Foundations of Mathematics*.** Chapter 1 introduces the type theory; the Appendix provides a precise definition of the formal type theory (which extends CoC/MLTT with univalence and HITs).
