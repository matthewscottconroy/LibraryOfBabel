# Important Thinkers: Synthetic Homotopy Theory

## Heinz Hopf (1894–1971)

Heinz Hopf discovered the Hopf fibration in 1931 and used it to compute π₃(S²) = Z — the first computation of a higher homotopy group of a sphere. His paper "Über die Abbildungen der dreidimensionalen Sphäre auf die Kugeloberfläche" (1931) transformed algebraic topology and opened the study of fibrations, fiber bundles, and stable homotopy theory.

Hopf's discovery was a shock to the mathematical community. Before 1931, it was believed (incorrectly) that any map between spheres of different dimensions was "trivial" — homotopic to a constant map. Hopf showed this was false by exhibiting a non-trivial map S³ → S² that generates Z in π₃(S²).

The Hopf fibration S¹ → S³ → S² and its generalizations (the quaternionic Hopf fibration S³ → S⁷ → S⁴ and the octonionic Hopf fibration S⁷ → S¹⁵ → S⁸) are among the most beautiful constructions in mathematics. They are intimately connected to the four normed division algebras (R, C, H, O) and the four values n = 1, 2, 4, 8 for which the Hopf invariant 1 problem has a positive answer (proved by Adams in 1960).

## Daniel Quillen (1940–2011)

Quillen developed the axiomatic framework of model categories — the correct setting for homotopy theory in general categories. His 1967 book "Homotopical Algebra" established that homotopy theory could be done in any "model category," with examples ranging from topological spaces to chain complexes to simplicial sets.

Quillen's model categories provide the mathematical underpinning for the simplicial set model of HoTT. The fact that the simplicial set model validates the Univalence Axiom (proved by Kapulkin and Lumsdaine) is a consequence of the model-categorical structure of simplicial sets.

## Dan Licata and Robert Harper

Licata and Harper's work at Carnegie Mellon University translated the theoretical advances of HoTT into practical proof assistant developments. Their implementation of synthetic homotopy theory in proof assistants — first in the HoTT-Agda library, later in Cubical Agda — made it possible to formally verify the theorems of this chapter.

Their 2013 paper "Calculating the Fundamental Group of the Circle in Homotopy Type Theory" (with Shulman) gave the first fully formalized proof of π₁(S¹) = Z, establishing the encode-decode method as the canonical tool for homotopy group computations.

## Guillaume Brunerie

Brunerie's 2016 PhD thesis is one of the most technically demanding achievements in synthetic homotopy theory. His proof of π₄(S³) = Z/2Z required:
1. The full theory of HITs, Univalence, and truncations.
2. The Hopf fibration in HoTT.
3. A novel "Brunerie number" β, proved to satisfy π₄(S³) = Z/|β|Z.
4. Computer-verified calculation that β = 2.

The last step — computing the Brunerie number — required running a Cubical Agda program that normalizes a type expression, reducing it to 2. This is a genuinely computational achievement: a computation in homotopy theory that required a computer, but is now formally verified.

Brunerie's work opened the path to further computations of homotopy groups of spheres in HoTT, and is ongoing research.

## Favonia (Kuen-Bang Hou)

Favonia (who prefers to use this single name professionally) was a key contributor to the formalization of synthetic homotopy theory in proof assistants. Their thesis "Higher-Dimensional Types in the Mechanization of Homotopy Theory" (2017) provided formal proofs of the van Kampen theorem, the Mayer-Vietoris sequence, and other fundamental results of synthetic homotopy theory.

Favonia's contributions to the HoTT-Agda library and later to Cubical Agda made many of the theoretical results of this chapter machine-verifiable for the first time.

## Mikhail Gromov (classical connection)

Gromov's work on metric geometry and geometric group theory, while not directly part of HoTT, provides the broader context in which synthetic homotopy theory operates. Gromov's insights on the large-scale geometry of groups and spaces — the coarse equivalence philosophy — resonate with the Univalence Axiom's insistence that mathematically meaningful properties are invariant under equivalence.

His concept of "soft analysis" — proving existence results without explicit constructions, relying on general principles — is in the spirit of synthetic homotopy theory, which proves topological theorems using type-theoretic principles rather than explicit geometric constructions.
