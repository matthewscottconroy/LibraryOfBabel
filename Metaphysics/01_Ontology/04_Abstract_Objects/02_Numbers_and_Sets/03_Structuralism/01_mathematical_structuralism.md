# Mathematical Structuralism

Mathematical structuralism is the view that mathematics is about structures — abstract patterns of relations among objects — rather than about particular objects with intrinsic properties. The natural numbers, on this view, are not things with their own individual natures; they are positions in the natural number structure, defined by their structural relationships (0 is not before anything, 1 is immediately after 0, 2 is immediately after 1, and so on).

The motivation for structuralism comes directly from Benacerraf's argument. If numbers can be identified with any of infinitely many different set-sequences, and if there is no non-arbitrary reason to choose one identification over another, then numbers are not any particular sets. What arithmetic is about is not sets or any other particular objects but the structural pattern that any sequence satisfying the Peano axioms exhibits.

Structuralism comes in several versions, distinguished by their metaphysical commitments. Ante rem structuralism (Shapiro) holds that structures exist as abstract objects prior to and independently of any objects that might instantiate them. The natural number structure exists, and the particular set-sequences satisfying the Peano axioms are just different ways of instantiating it. This is a form of Platonism about structures rather than about individual mathematical objects.

In re structuralism holds that structures exist only insofar as they are instantiated. There is no abstract structure floating free of instantiations; the structure is just what all the instantiations have in common. This is more nominalist-friendly but faces difficulties when dealing with structures that have no physical instantiations.

Modal structuralism (Hellman) reads mathematical claims as modal: arithmetic claims are true iff they would be true in any possible omega-sequence. This avoids positing abstract structures while preserving the content of mathematical claims.

## The Core Insight

The structuralist insight is that mathematical identity is exhausted by structural properties. Consider the question: is the number 2 a member of the number 4? On the von Neumann construction, yes (since 4 = {0, 1, 2, 3} and 2 ∈ 4). On the Zermelo construction, no (since 4 = {{{{∅}}}}, which contains only {{∅, {∅}}} as a member). This is a difference between the two constructions, but it is not a difference in the arithmetic properties of 2 and 4. In arithmetic, there is no fact about membership relations among numbers — membership is a set-theoretic notion, not an arithmetic one.

The structuralist conclusion: numbers have only arithmetic properties. "The number 2 is the predecessor of 3" is true; "the number 2 is a member of 4" is neither true nor false — it is a category mistake, like asking whether the rook is older than the bishop in chess. Chess pieces have only chess properties; numbers have only arithmetic properties.

This position, as Benacerraf articulated it, implies that numbers are *positions in a structure*, not objects with independent natures. A position has its identity fixed entirely by its structural relations to other positions.

## Structuralism and Mathematical Practice

Mathematical practice supports structuralism. Mathematicians routinely:

- Study groups, rings, fields, and other abstract algebraic structures without specifying particular sets as their models.
- Prove theorems about "the" natural numbers, meaning any progression satisfying the Peano axioms — not a particular set.
- Develop category theory, which is explicitly about structural relationships (morphisms) rather than internal constitution of objects.
- Treat isomorphic structures as "the same" for mathematical purposes — two groups that are isomorphic are mathematically indistinguishable.

All of this is consistent with structuralism: mathematicians work with structural patterns, and the particular objects that instantiate those patterns are interchangeable.

**Isomorphism invariance** is perhaps the most important observation. A mathematical property is invariant under isomorphism if it holds of one structure iff it holds of every isomorphic structure. For example, "being cyclic" (having a single generator) is a group property that is isomorphism-invariant. By contrast, "having {∅, {∅}} as a member" is not isomorphism-invariant — it is a property of a particular set-theoretic representation, not of the abstract group structure.

Structuralism predicts: only isomorphism-invariant properties are genuine mathematical properties. This prediction aligns with mathematical practice, where mathematicians never appeal to non-invariant properties in proving mathematical theorems.

## Objections to Structuralism

**The individuation problem**: If mathematical objects are positions in structures, what individuates those positions? In a symmetric structure (like the complex numbers, which have an automorphism swapping i and -i), two positions can be structurally indiscernible. If positions are individuated only by their structural properties, then structurally indiscernible positions should be identical — but i ≠ -i.

Ante rem structuralists respond that positions are individuated partly by being *different positions*, not purely by their structural properties. The complex number structure simply has two square roots of -1 as a structural fact, even if the two cannot be distinguished by structural predicates.

**The coherence challenge**: What makes a structural description coherent? If any coherent structural description corresponds to an existing structure (ante rem view), we need an account of coherence. If coherence = logical consistency, then the ontology of mathematics is determined by logic — a claim that reverts to logicism.

**Mathematical objects across structures**: The number 2 appears in the natural number structure, the integer structure, the rational number structure, and the real number structure. If positions are individuated within their structures, then there are multiple distinct objects all called "2." But mathematicians treat these as the same object. Shapiro's response: these are different positions in different structures that are connected by *inclusion maps* — structure-preserving functions that embed smaller structures into larger ones.
