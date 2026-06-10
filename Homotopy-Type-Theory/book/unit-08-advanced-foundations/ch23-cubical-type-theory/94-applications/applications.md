# Applications — Chapter 23: Cubical Type Theory

## Application 1: Cubical Agda Library Development

**The context.** The cubical-agda library (`github.com/agda/cubical`) is the primary repository of formalized mathematics in cubical type theory. It contains proofs of results that were either impossible or practically infeasible in Book HoTT.

**The application.** Mathematicians and type theorists use Cubical Agda to:

*Synthetic homotopy groups of spheres*: The library contains computations of $\pi_n(S^m)$ for small $n, m$, including $\pi_1(S^1) = \mathbb{Z}$, $\pi_2(S^2) = \mathbb{Z}$, $\pi_3(S^3) = \mathbb{Z}$, and the Brunerie computation of $\pi_4(S^3) = \mathbb{Z}/2$. These proofs use `ua` and transport through `ua` essentially — without computational univalence, they would be stuck.

*Algebraic structures*: Group theory, ring theory, and module theory formalized with the correct notion of equivalence. Isomorphic groups are provably equal (by univalence), so any group-theoretic statement is automatically structure-invariant.

*Cohomology*: Integral cohomology groups $H^n(X; \mathbb{Z})$ for spaces $X$ defined as HITs. The cohomology groups are computed using Eilenberg-MacLane spaces $K(\mathbb{Z}, n)$, themselves definable as HITs. Cubical transport computes the cohomology maps.

**The significance.** The library demonstrates that cubical type theory is not a foundational curiosity — it is a practical tool for formalizing contemporary mathematics.

## Application 2: Formal Verification of Homotopy Equivalences

**The context.** In pure mathematics and in theoretical computer science, one frequently needs to establish that two spaces (or types) are equivalent. Classical proofs of homotopy equivalence are often informal, relying on geometric intuition.

**The application.** Cubical Agda provides a framework where homotopy equivalences can be *formally verified*:

*The Seifert-van Kampen theorem*: If $A = B \cup C$ (a pushout of spaces) and $B, C, B \cap C$ are path-connected, then $\pi_1(A) = \pi_1(B) *_{\pi_1(B \cap C)} \pi_1(C)$ (the amalgamated product). This is formalized in Cubical Agda using pushout HITs and the cubical transport along `ua`.

*The Blakers-Massey theorem*: A connectivity theorem about pushouts, proved synthetically in Cubical Agda. The proof uses `hcomp` to fill boxes corresponding to null-homotopies.

*Freudenthal suspension theorem*: If $A$ is $n$-connected, the suspension map $\pi_k(A) \to \pi_{k+1}(\Sigma A)$ is an isomorphism for $k \leq 2n$. Formalized using truncations and `hcomp`.

**In programming languages**: Type equivalences correspond to data structure isomorphisms. If two data structures (types) are equivalent, any algorithm for one automatically gives an algorithm for the other via transport. Cubical Agda's computationally valid transport means the "transferred" algorithm is not just formally correct — it *runs*.

## Application 3: Program Transfer via Computational Univalence

**The context.** In programming, one often develops an algorithm for an *abstract* type (e.g., a list) and wants to transfer it to a *concrete* type (e.g., an array backed by a specific memory layout). If the two types are isomorphic, the transfer should be automatic.

**The application.** Computational univalence makes this transfer executable:

1. Prove an equivalence $e : \mathsf{List}\; \mathbb{N} \simeq \mathsf{Array}\; \mathbb{N}$ — a bijection between the two representations.
2. Apply `ua(e)` to get a path in the type universe.
3. Transport any algorithm $f : \mathsf{List}\; \mathbb{N} \to \mathsf{List}\; \mathbb{N}$ to an algorithm $\mathsf{transport}(\mathsf{ap}_\mathsf{hom}\, (\mathsf{ua}(e)), f) : \mathsf{Array}\; \mathbb{N} \to \mathsf{Array}\; \mathbb{N}$.
4. The computation rule gives: this transported algorithm, when run, applies $e^{-1}$, then $f$, then $e$ — the correct "port" of the algorithm.

In Cubical Agda, this is not just a formal construction — the transported algorithm is actually *executable* and produces the *correct output* by the canonicity theorem.

**The Coq Equations project** and the **Data.Coerce** library in Haskell attempt something similar without full computational univalence. Computational univalence is the correct foundational account of why program transfer works and when it is valid.

## Application 4: Formal Verification of Geometric Algorithms

**The context.** Computational geometry algorithms (Voronoi diagrams, Delaunay triangulations, convex hull algorithms) are notoriously difficult to prove correct. Degenerate cases, numerical precision issues, and topological invariants all complicate verification.

**The application.** Cubical type theory offers tools for geometric verification:

*Topological invariants of data structures*: A triangulation of a surface has topological invariants (Euler characteristic, genus). In Cubical Agda, one can define the surface as a HIT (a 2-dimensional CW complex) and prove that the triangulation preserves the invariant. The proof uses `hcomp` to verify that the attachment maps are consistent.

*Path planning with topology*: Robot motion planning in a space with obstacles is equivalent to path-finding in the fundamental group of the free space. Cubical type theory's $\pi_1$ computation tools (loop spaces, van Kampen) can be applied to verify that a planned path is in the correct homotopy class.

*Persistent homology*: The Vietoris-Rips complex of a point cloud has homology groups that capture the topological shape. In Cubical Agda, the Vietoris-Rips complex can be defined, and its homology computed, using the cubical HIT infrastructure.

These applications are largely prospective — the field is young — but the foundational tools are now in place.

## Application 5: Categorical Semantics of Programming Languages

**The context.** Modern programming languages with dependent types (Agda, Coq, Lean, Idris) have a categorical semantics: types are interpreted as objects in a suitable ∞-category, and programs are interpreted as morphisms. The correctness of the semantics requires that the language's equational theory matches the ∞-categorical identities.

**The application.** Cubical type theory provides the correct setting for this:

*Denotational semantics with definitional equality*: In a denotational semantics for a programming language, two programs are semantically equivalent iff they denote the same mathematical object. In cubical type theory, the denotational semantics respects the definitional equalities of the language — programs that are definitionally equal denote *the same* object, not just *equivalent* objects.

*Parametricity in cubical type theory*: Parametric polymorphism (every function from lists to lists must preserve the length structure, etc.) has a natural formulation using the interval. A parametric function $f : \Pi_{A:\mathsf{Type}} F(A)$ must be "natural" — it must commute with all equivalences. This is captured by requiring $f$ to be a function of dimension variables, and the cubical interval provides the naturality conditions.

*Observational type theory*: Sterling and Shulman's work connects observational type theory (where equality is defined by what can be observed about values) to the boundary separation principle of XTT. The connection suggests that XTT's metatheory is the correct framework for verifying observational equivalences in programming language semantics.

## Application 6: Extracting Verified Programs with Correct Computation

**The context.** Proofs in constructive type theory can be *extracted* as programs. But extraction is only useful if the extracted programs are *efficient* — they should compute, not just exist formally.

**The application.** Cubical Agda's canonicity ensures that extracted programs compute correctly:

*Counting proofs*: A proof of $\exists n : \mathbb{N}. P(n)$ in Cubical Agda can be extracted to an algorithm that produces a concrete $n$ satisfying $P$. Unlike Book HoTT, this extraction works even when the proof uses univalence — the computation rules for Glue ensure that the witness can be evaluated.

*Sorting algorithms*: A sorting algorithm formalized in Cubical Agda (with a correctness proof) can be extracted and run. The extraction is not merely a formal possibility; the canonicity theorem guarantees that the extracted program normalizes on every input.

*Verification of cryptographic protocols*: Security protocols are modeled as programs with correctness properties (e.g., "the receiver always recovers the message"). A correctness proof in Cubical Agda can be extracted as a verified implementation of the protocol.

The key point: in Book HoTT, proofs using univalence could not be extracted as running programs. In Cubical Agda, they can. This is the practical significance of canonicity for software verification.

## Application 7: Research in Synthetic Algebraic Geometry

**The context.** Algebraic geometry studies varieties — solution sets of polynomial equations — using the tools of commutative algebra and category theory. Grothendieck's foundations (schemes, étale topology, stacks) are categorical in nature.

**The application.** The *Algebraic Geometry in Cubical Agda* project (Mörtberg, Zeuner, and others) aims to formalize algebraic geometry using cubical type theory:

*Spectra and Zariski topology*: The Zariski spectrum of a commutative ring $R$ is a topological space whose points are prime ideals. In Cubical Agda, this can be defined as a type with a HIT structure capturing the Zariski topology. The cubical `hcomp` handles the patching conditions needed for sheaves on the Zariski site.

*Étale maps*: An étale map of algebraic varieties is a local isomorphism (the categorical analogue of a local diffeomorphism). In Cubical Agda, étale maps are defined using the notion of formal étale maps: maps where the derived functors are trivial. The cubical infrastructure (Glue types, transport) handles the infinitesimal thickening needed.

*Cohomology of algebraic varieties*: Étale cohomology, de Rham cohomology, and their relationship (crystalline cohomology) all have formulations that should be feasible in Cubical Agda, given the existing cohomology infrastructure.

This is a long-term research program, but it represents one of the most exciting frontiers: using computational univalence to do research-level algebraic geometry inside a proof assistant.
