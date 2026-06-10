# Important Thinkers: Cubical Type Theory and Computational HoTT

## Thierry Coquand (1961–present)
*The foundational architect of cubical type theory; co-inventor of the Calculus of Constructions; the driving force behind making HoTT computationally real.*

Thierry Coquand is a professor at Chalmers University of Technology in Gothenburg, and one of the most important figures in the history of type theory and proof assistants. With Gérard Huet, he developed the Calculus of Constructions in 1988, which became the foundation for the Coq proof assistant and, through a long lineage, for Lean 4 and Agda. His early work established the theoretical framework in which modern proof assistants operate.

His contribution to the content of this chapter, however, is more specific and more recent: the development of *cubical type theory*. The problem was known: axiomatic HoTT (with univalence as a postulate) breaks canonicity. Coquand's group, working with Marc Bezem and Simon Huber in the early 2010s, found the solution. They observed that if you replace the identity type with functions out of a formal interval, then univalence can be proved from the structure of the interval, and canonicity is restored. The 2016 paper "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom" (with Cohen, Huber, and Mörtberg) is the founding document of the field.

Coquand's influence pervades every section of this chapter. The `I` type, the `i0` and `i1` endpoints, the `~_` complement, the `hcomp` composition, the `Glue` type, the `ua` function — these are all his inventions (jointly with collaborators). More deeply, the decision to make paths be *functions* rather than *inductively defined objects* is Coquand's foundational insight, and it is what makes everything else work.

---

## Anders Mörtberg (1988–present)
*The primary developer of Cubical Agda; the leader of the cubical library; author of the implementations that made computational HoTT practical.*

Anders Mörtberg is a professor at Stockholm University, having previously held positions at Carnegie Mellon University and the Institute for Advanced Study. He was a co-author of the foundational 2016 cubical type theory paper and is the primary person responsible for translating that theory into working software: Cubical Agda and its library.

Mörtberg's specific contribution to this chapter is the implementation layer. Cubical type theory is a mathematical theory; making it run in Agda required implementing the interval sort, the boundary condition checking for path types, the `hcomp` box-filling algorithm, the `transp` computation rules for each type former, and the `Glue` type constructor with its coherence conditions. Each of these required both theoretical precision (getting the computation rules exactly right) and engineering skill (making the implementation efficient enough to use in practice).

The Cubical Agda library, which Mörtberg continues to develop and maintain, is the primary repository of formalized HoTT with computational content. The circle, the fundamental group theorem, the Freudenthal suspension theorem, Brunerie's theorem — all of these were formalized by Mörtberg and collaborators, using the infrastructure he built. His 2019 ICM proceedings paper "Cubical Methods in HoTT and UF" provides an accessible introduction to the state of the field.

Perhaps most significantly, Mörtberg (with Axel Ljungström) performed the 2022 optimization that made Brunerie's theorem actually computable: reducing the normalization of the Brunerie number from hours to seconds. This required a deep understanding of both the mathematics (the Hopf fibration, the James construction) and the computation (where the normalizer was spending time and how to restructure the proof to make it faster).

---

## Simon Huber (present)
*Co-inventor of cubical type theory; implementer of the cubical type checker; the engineer who made the mathematical theory run.*

Simon Huber completed his PhD at Chalmers under Coquand's supervision, with his thesis directly on cubical type theory and its implementation. He is a co-author of the foundational 2016 paper and the primary implementer of the first cubical type checker (in Haskell) that demonstrated the theory was computationally viable.

Huber's contribution is at the intersection of theory and implementation. The type checking algorithm for cubical type theory is significantly more complex than for Martin-Löf type theory: checking that a term has a path type requires checking boundary conditions at the interval endpoints; implementing `hcomp` requires finding a "fill" for partial cubes; implementing `Glue` requires working with partial equivalences. Huber worked out the details of these algorithms and implemented them.

For readers of this chapter, Huber's work is most visible in the connection between the mathematical definitions (given in the chapter) and the Agda syntax that implements them. Every time Cubical Agda accepts a path definition by checking its boundary conditions, it is using algorithms that trace back to Huber's thesis.

---

## Guillaume Brunerie (present)
*The mathematician whose thesis provided the computational proof that π₄(S³) = ℤ/2ℤ; a pioneer of synthetic homotopy theory in HoTT.*

Guillaume Brunerie completed his PhD at the University of Nice Sophia Antipolis, supervised by Carlos Simpson. His 2016 thesis, "On the Homotopy Groups of Spheres in HoTT," proved two things: first, the theoretical result that the Brunerie number (an integer defined by abstract HoTT constructions) equals $\pm 2$; and second, that this implies $\pi_4(S^3) \cong \mathbb{Z}/2\mathbb{Z}$.

The significance: this was the first HoTT proof of a non-trivial homotopy group calculation beyond $\pi_n(S^n) = \mathbb{Z}$. The Hopf fibration, the Blakers-Massey theorem, the freudenthal suspension theorem, and a careful calculation of the Hopf invariant — all formalized in HoTT. The thesis also showed that the Brunerie number could, in principle, be computed, establishing the blueprint for the eventual Cubical Agda implementation.

Brunerie's work connects this chapter to the research frontier. The formalization of his thesis by Mörtberg, Ljungström, and collaborators in Cubical Agda is one of the landmark achievements of the field. For readers of this curriculum, it is the proof that HoTT's synthetic approach to homotopy theory is not just a foundational framework but a practical tool for doing mathematics.

---

## Axel Ljungström (present)
*Cubical Agda contributor; co-author of the optimized formalization of Brunerie's theorem; researcher in synthetic homotopy theory.*

Axel Ljungström is a doctoral student (as of 2025) at Stockholm University, working with Mörtberg on the formalization of homotopy theory in Cubical Agda. His most significant contribution to the topics in this chapter is the 2022 joint work with Mörtberg on optimizing the computation of the Brunerie number.

The original formalization of Brunerie's theorem, while correct, was computationally intractable: normalizing the Brunerie number took hours even on fast hardware. Ljungström and Mörtberg identified the bottlenecks — specific algebraic simplifications and abstraction points where the normalizer was doing unnecessary work — and restructured the proof to make the computation tractable. The result: the Brunerie number now normalizes to 2 in seconds.

This optimization is not merely a technical improvement. It demonstrates something philosophically important: that the computational content of HoTT proofs is not just theoretically present but practically accessible. The proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ is a computation you can run on your laptop.

---

## Ulf Norell (present)
*The primary designer and implementer of modern Agda; the person who made Agda into the proof assistant underlying Cubical Agda.*

Ulf Norell completed his PhD at Chalmers, with his thesis providing the first complete implementation of Agda 2 — the modern Agda that all current work builds on. Before Norell's thesis, Agda existed as a research prototype but was not practically usable for substantial formalization work. Norell's implementation of dependent pattern matching, universe polymorphism, and the interactive proof mode (holes, case splitting, auto) transformed it into a real tool.

Norell's work is infrastructure: it is the foundation that Mörtberg and others built Cubical Agda on. Without Agda's general dependent type theory, interactive editing support, and Haskell compilation backend, Cubical Agda would not exist. The `{-# OPTIONS --cubical #-}` pragma that appears at the top of every Cubical Agda file activates extensions to Norell's base system.

For this chapter, Norell's work is most visible in the interactive development workflow — holes (`?`), case splitting (`C-c C-c`), auto (`C-c C-a`), and normalization (`C-c C-n`). These features were designed by Norell and have shaped how every Agda user thinks about theorem proving.
