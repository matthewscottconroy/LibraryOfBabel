# Important Thinkers in Abstract Algebra

## Évariste Galois (1811–1832)

Galois invented group theory to solve a question about polynomial equations. His key insight: the solvability of a polynomial p(x) by radicals is equivalent to a property of the group of symmetries of the polynomial's roots — now called the Galois group. Specifically, p(x) is solvable by radicals iff its Galois group is a *solvable group* (one with a chain of normal subgroups with abelian quotients).

To state this theorem, Galois had to invent normal subgroups and quotient groups. He was writing on the night before a duel in which he died at age 20, and his manuscripts — barely legible, hastily written — sat unpublished for 14 years before Liouville recognized their significance. The concepts Galois invented — group, normal subgroup, quotient group — are the organizing concepts of Chapter 2.

## Emmy Noether (1882–1935)

Noether transformed algebra through the *structural* approach: instead of computing with specific objects, prove theorems about all objects satisfying certain axioms. Her 1921 paper on ideals in rings introduced the ascending chain condition (a ring is *Noetherian* if every ascending chain of ideals stabilizes), proved that in a Noetherian ring every ideal is finitely generated, and established the algebraic structure theory that underlies commutative algebra and algebraic geometry.

Her three isomorphism theorems — the First (G/ker φ ≅ Im φ), Second (H/(H∩N) ≅ HN/N), and Third ((G/N)/(K/N) ≅ G/K) — are proved in Chapter 2 for groups. Noether stated and proved them in the ring-theoretic setting, recognizing they expressed a universal structural pattern. Hilbert called her "the most significant creative mathematical genius thus far produced," while institutional sexism prevented her from holding a regular academic position for most of her career.

## Arthur Cayley (1821–1895)

Cayley gave the first abstract definition of a group (1854) and proved what is now called Cayley's theorem: every group is isomorphic to a subgroup of a symmetric group. This theorem — proved in this chapter — shows that abstract groups are exactly as general as permutation groups, and that the symmetric group is the universal case.

Cayley also developed matrix algebra (independently and concurrently with Hamilton and Sylvester), recognizing that matrices multiply non-commutatively and that they form a ring. The examples GL_n(k) and M_n(k) that appear throughout this chapter are Cayley's contribution to the landscape of algebraic examples.

## Niels Henrik Abel (1802–1829)

Abel proved the impossibility of solving the general quintic by radicals — the first impossibility theorem in mathematics, and the direct precursor to Galois theory. Working without the language of groups (which Galois would invent a few years later), Abel identified the structural reason why degree-5 polynomials resist radical solutions: their symmetry group is "too non-commutative." He died of tuberculosis at 26, never holding an academic position.

Abelian groups — groups with commutative multiplication — are named after Abel in recognition of his work on commutative algebraic structures. The classification of finitely generated abelian groups (ℤ^r ⊕ ⊕ℤ/nᵢℤ) is one of the cleanest results in algebra and a preview of the richer structure theorems of module theory.

## Richard Dedekind (1831–1916)

Dedekind invented ideals as a tool for restoring unique factorization in rings of algebraic integers where it fails for elements. In ℤ[√-5], the number 6 factors as 2·3 and also as (1+√-5)(1-√-5) — two distinct factorizations. Dedekind showed that unique factorization holds for *ideals*, not elements: the ideal (6) factors uniquely as a product of prime ideals. This is the central theorem of algebraic number theory, and it requires the full ring-theoretic framework of Chapter 2.

Dedekind also formalized the concept of a *field* (as the right setting for Galois theory), introduced the predecessor of the Dedekind cut construction of ℝ, and developed a remarkably abstract and structural approach to algebra that prefigures Noether's later influence.

## Henri Poincaré (1854–1912)

Poincaré invented algebraic topology. His 1895 paper *Analysis Situs* introduced the fundamental group, homology groups, and the Euler characteristic as algebraic invariants of topological spaces. He recognized that the loops in a topological space form a group — the fundamental group — and that this group captures the one-dimensional "holes" of the space.

The Poincaré conjecture — that every simply connected closed 3-manifold is homeomorphic to the 3-sphere — occupied topologists for a century, finally proved by Perelman in 2003 using Ricci flow. But Poincaré's broader contribution — the idea that algebraic invariants of spaces detect topological properties — is the foundational insight of algebraic topology, and the bridge to HoTT described in Section 5 of this chapter.

## Alexander Grothendieck (1928–2014)

Grothendieck reformulated algebraic geometry using categorical methods, and in doing so, invented or substantially developed: sheaves and topos theory, K-theory (groups built from vector bundles over a space), derived categories, motives, and the étale cohomology that underlies the proof of the Weil conjectures (proved by his student Deligne).

For this chapter, Grothendieck's most relevant contribution is K-theory and the Grothendieck group: given a commutative monoid M (a set with an associative, commutative operation and identity, but no inverses), the *Grothendieck group* K(M) is the "most general" group containing M. It is the free group with the relations of M — the universal property construction applied to monoids. This construction appears in algebraic K-theory and is the group-theoretic version of the type-theoretic *propositional truncation* (making a set out of a type by imposing that all proofs of propositions are equal).

Grothendieck's influence on HoTT is indirect but real: the categorical framework he developed — categories, functors, natural transformations, adjunctions — is the language in which the Curry-Howard correspondence and the connections between algebra and type theory are most clearly expressed.
