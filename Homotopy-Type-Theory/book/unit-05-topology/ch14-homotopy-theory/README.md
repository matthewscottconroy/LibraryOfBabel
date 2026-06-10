# Chapter 14: Homotopy Theory

In 1904, Henri Poincaré asked: is every simply-connected closed 3-manifold homeomorphic to the 3-sphere? He had just invented the fundamental group. He conjectured the answer was yes and spent years trying to prove it. He was right, but the proof came a century later, and it required understanding spaces in a way Poincaré could not have imagined: as objects in a category, whose homotopy type is an invariant, whose higher homotopy groups encode an infinity of structure. Perelman's proof used Ricci flow — a geometric technique — but its verification used homotopy theory that runs all the way into HoTT.

What does it mean for Poincaré to have "just invented the fundamental group"? It means he had realized that a space carries, as part of its intrinsic structure, a group: the group of loops based at a point, considered up to continuous deformation. A space is simply-connected if this group is trivial — if every loop can be continuously contracted to a point. The 3-sphere is simply-connected. Is every simply-connected closed 3-manifold actually the 3-sphere, or could there be exotic examples that are simply-connected but not the sphere?

The answer required understanding not just the fundamental group but the entire tower of homotopy groups $\pi_n$ for all $n$, the structure of fibrations and their long exact sequences, and the relationship between geometric methods (Ricci flow) and homotopy-theoretic methods (surgery theory). The Poincaré conjecture is, in this sense, a testimony to the power of homotopy theory: a question about spaces turns out to require all of the algebraic machinery that homotopy theory has developed over a century.

## The Coarser Question

Classical topology asks: when are two spaces homeomorphic — topologically identical, with a bijective continuous map with continuous inverse? Homotopy theory asks a coarser question: when are two spaces *homotopy equivalent* — deformable into each other by a continuous deformation that need not be invertible point-by-point?

The real line $\mathbb{R}$ is homotopy equivalent to a single point: you can continuously contract every point to the origin via $H(x,t) = (1-t)x$. But $\mathbb{R}$ is not homeomorphic to a point — they have different cardinalities, different dimensions. Homotopy equivalence throws away more information than homeomorphism. It keeps only the "shape" that matters for algebraic invariants.

And this coarser notion is exactly the right one for Poincaré's question, for HoTT, and for the applications of topology to algebra and computation. The fundamental group, the higher homotopy groups, homology, cohomology — these are all homotopy invariants, not just homeomorphism invariants. They see the shape that homotopy equivalence preserves.

## The HoTT Axis

The connection between homotopy theory and HoTT is not decorative — it is constitutive. When you write an identity type $a =_A b$ in HoTT, you are writing a path space. When you write $f \sim g$ (a homotopy), you are writing a term of $\prod_{a:A} f(a) = g(a)$. When you write $A \simeq B$ (an equivalence of types), you are writing a homotopy equivalence.

The univalence axiom — the deepest axiom of HoTT — says that the type of equivalences $A \simeq B$ is equivalent to the type of identities $A =_{\mathcal{U}} B$ in the universe. This is the type-theoretic assertion that homotopy equivalence is *the* right notion of identity for types. Homeomorphism is too strong (it forgets the homotopy-theoretic structure); homotopy equivalence is exactly right.

Voevodsky proved this is consistent by constructing a model of HoTT in simplicial sets, where $A \simeq B$ literally corresponds to a homotopy equivalence of Kan complexes, and $A =_{\mathcal{U}} B$ corresponds to a path in the universe of Kan complexes. The classical homotopy theory developed in this chapter is the semantic foundation for the type theory.

## What This Chapter Covers

**Section 1 (Homotopy Equivalences)** develops the notion of homotopy between maps and homotopy equivalence between spaces. Contractible spaces, deformation retracts, the homotopy category. The key examples: every convex subset of $\mathbb{R}^n$ is contractible; $\mathbb{R}^2 \setminus \{0\}$ is homotopy equivalent to $S^1$; a connected graph is homotopy equivalent to a wedge of circles.

**Section 2 (Fundamental Group)** constructs $\pi_1(X, x_0)$ as the group of homotopy classes of loops. Path concatenation, inverse loops, the proof (outline) that $\pi_1(S^1) = \mathbb{Z}$. Van Kampen's theorem and its applications: computing $\pi_1$ of the torus, projective plane, and surfaces. The fundamental groupoid.

**Section 3 (Covering Spaces)** establishes the Galois theory of covering spaces: path-lifting, homotopy-lifting, and the classification of covers by subgroups of $\pi_1$. The universal cover, monodromy, and the deck transformation group. The connection to $\Pi$-types in HoTT.

**Section 4 (Higher Homotopy Groups)** introduces $\pi_n(X, x_0)$ for $n \geq 2$ as homotopy classes of maps $S^n \to X$. The Eckmann-Hilton argument showing $\pi_n$ is abelian for $n \geq 2$ — and its HoTT analog. Eilenberg-MacLane spaces. The long exact sequence of a fibration.

**Section 5 (Fibrations)** develops Serre and Hurewicz fibrations, the path-loop fibration, and the long exact sequence. The Hopf fibration $S^1 \to S^3 \to S^2$ and the computation $\pi_3(S^2) = \mathbb{Z}$. Fibrations in HoTT as dependent types.

## The Thread

Throughout this chapter, the key pattern is: *algebraic invariants detect topological difference*. The fundamental group tells you whether a space has 1-dimensional holes. Higher homotopy groups detect higher-dimensional holes. The long exact sequence of a fibration relates the invariants of the pieces to the invariants of the whole. By the end of this chapter, you will have a powerful toolkit for understanding the shape of spaces — the same toolkit, expressed in type-theoretic language, that HoTT uses to prove theorems about types.
