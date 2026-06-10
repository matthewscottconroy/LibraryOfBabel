# References: Cubical Agda and Computational HoTT

## Primary References

**Cohen, Cyril, Coquand, Thierry, Huber, Simon, and Mörtberg, Anders. "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom." *Types for Proofs and Programs (TYPES)*, 2016.**
The foundational paper of the field. Defines cubical type theory: the interval `I`, path types as functions, the `hcomp` composition, the `Glue` type, and the proof of univalence. Everything in this chapter traces back to this paper. Required reading for anyone who wants to understand the foundations.

**Vezzosi, Andrea, Mörtberg, Anders, and Abel, Andreas. "Cubical Agda: A Dependently Typed Programming Language with Univalence and Higher Inductive Types." *Journal of Functional Programming*, 2021.**
The paper describing Cubical Agda as a practical implementation of cubical type theory. Covers the design of the `--cubical` pragma, the implementation of HITs, and the library structure. The bridge between the mathematics of the 2016 paper and the working proof assistant.

**The Cubical Agda Library.** `github.com/agda/cubical`.
The primary repository for all formalized HoTT with computational content. Contains the circle, spheres, fundamental groups, Brunerie's theorem, and much more. Essential for any practical work in Cubical Agda.

---

## The HoTT Book

**The Univalent Foundations Program. *Homotopy Type Theory: Univalent Foundations of Mathematics*. Institute for Advanced Study, 2013.**
Freely available at `homotopytypetheory.org/book`. The foundational text for HoTT. Covers the mathematical content (identity types, higher inductive types, univalence, homotopy groups) in the axiomatic setting. Cubical Agda provides the computational realization of this book's mathematics.

---

## Agda

**Norell, Ulf. *Towards a Practical Programming Language Based on Dependent Type Theory*. PhD thesis, Chalmers University, 2007.**
The foundational document for modern Agda. Defines the pattern matching algorithm, universe polymorphism, and the interactive elaboration that underlies Cubical Agda. Understanding Agda at this level gives a deep understanding of what `C-c C-c` is really doing.

**The Agda Documentation.** `agda.readthedocs.io`.
The official documentation for Agda, covering syntax, type-checking rules, the cubical extensions, and the standard library. The sections on cubical mode are directly relevant to this chapter.

---

## Brunerie's Theorem

**Brunerie, Guillaume. *On the Homotopy Groups of Spheres in Homotopy Type Theory*. PhD thesis, University of Nice, 2016.**
Available at `arxiv.org/abs/1606.05916`. The thesis proving $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ in HoTT. Introduces the Brunerie number and the chain of constructions (Hopf fibration, James construction, cup product structure) that define it.

**Ljungström, Axel, and Mörtberg, Anders. "Formalizing π₄(S³) = ℤ/2ℤ and Computing the Brunerie Number." *Proceedings of LICS*, 2022.**
The paper describing the Cubical Agda formalization of Brunerie's theorem and the optimizations that made the Brunerie number computation tractable. Shows that the Brunerie number computes to 2 in Cubical Agda.

---

## Homotopy Theory in HoTT

**Hou (Favonia), Kuen-Bang, and Harper, Robert. "Covering Spaces in Homotopy Type Theory." *Types for Proofs and Programs*, 2016.**
The encode-decode method for π₁(S¹) = ℤ, formalized in HoTT. This is the proof structure described in the HITs section and implemented in Cubical Agda.

**Licata, Daniel R., and Shulman, Michael. "Calculating the Fundamental Group of the Circle in HoTT." *Proceedings of LICS*, 2013.**
The first published proof of π₁(S¹) = ℤ in HoTT, using a different (but equivalent) method. Establishes the pattern for synthetic homotopy theory proofs.

**Cavallo, Evan, and Harper, Robert. "Higher Inductive Types in Cubical Computational Type Theory." *Proceedings of POPL*, 2019.**
The treatment of HITs in cubical type theory, including the path constructor semantics and the computation rules for eliminators.

---

## Foundations of Cubical Type Theory

**Bezem, Marc, Coquand, Thierry, and Huber, Simon. "A Model of Type Theory in Cubical Sets." *Types for Proofs and Programs*, 2014.**
The first paper establishing the mathematical semantics of cubical type theory, using the category of cubical sets as a model. Proves that cubical type theory is consistent by exhibiting a model.

**Angiuli, Carlo, Mörtberg, Anders, and Zeuner, Max. "Inductive Types and Their Elimination in Cubical Agda." 2022.**
A more recent treatment of how HITs fit into cubical type theory, with careful attention to the computation rules.

---

## For Further Study

**The HoTT Coq Library.** `github.com/HoTT/HoTT`. HoTT formalized in Coq (with axiomatic univalence). Useful for comparison: the same mathematical content, but in the non-computational (axiomatic) framework.

**Symmetry.** `github.com/UniMath/SymmetryBook`. A HoTT textbook in development, formalized in UniMath (Coq). Covers foundations and algebraic structures from the univalent perspective.

**The agda-unimath library.** `github.com/UniMath/agda-unimath`. A Univalent Mathematics library for Agda (without `--cubical`, using `--without-K`). Provides an alternative to the Cubical library for users who prefer to avoid the cubical primitives.

**Mörtberg, Anders. "Cubical Methods in HoTT and UF." *Proceedings of the ICM*, 2022.**
An accessible survey of the state of the field, written for a general mathematical audience. Covers the main results and open problems in computational HoTT.
