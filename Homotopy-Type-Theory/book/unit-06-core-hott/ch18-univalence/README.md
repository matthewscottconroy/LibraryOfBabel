# Chapter 18: Equivalences and the Univalence Axiom

Vladimir Voevodsky stated the Univalence Axiom in 2006, and when he did, he knew it would be controversial. The axiom says: if two types are equivalent — if there is a bijection between them that is an isomorphism in every possible sense — then they are equal. Not just isomorphic. Equal. As the same type.

This means that Z/2Z and {0,1} with mod-2 addition are not just isomorphic groups — they are the same group. It sounds like saying that identical twins are the same person. It feels wrong.

But here is the thing. In classical set theory, equality is set membership equality. Two sets are equal iff they have the same elements. A group is a set with extra structure, so two groups are equal iff they are the same set — which means they have the same elements, which almost never happens. Z/2Z = {[0], [1]} and {0,1} are different sets (one has equivalence classes as elements, the other has integers), so they are different groups, even though they are isomorphic.

This is the set-theoretic account of equality for groups, and it is unsatisfying. It means the group-theoretic notion of "the same group" — isomorphism — and the set-theoretic notion of "the same set" — equality — come apart. Mathematicians deal with this by saying "up to isomorphism" constantly and treating isomorphic groups as the same for all practical purposes while knowing that formally they are not.

Voevodsky asked: what if we made this formal? What if we had a type theory where "equal" means "equivalent" — where the formal equality relation and the mathematical isomorphism relation coincide?

The answer is the Univalence Axiom: `(A =_{Type} B) ≃ (A ≃ B)`. A path between two types is an equivalence between them. And since paths carry data — since a path from A to B in the universe is a specific equivalence, not just the fact of equivalence — the Univalence Axiom captures not just "equivalent" but "equivalent via this specific equivalence."

This is not a confusion. It is the correct statement of what mathematicians have always meant. When a mathematician says "Z/2Z and {0,1} are the same group," they mean "there is a specific isomorphism between them" — the canonical one, sending [0] to 0 and [1] to 1. This specific isomorphism is a path in the universe. Different isomorphisms are different paths. The paths can be compared (are they the same isomorphism?) and composed (if A ≃ B and B ≃ C, then A ≃ C).

Once you understand this, the Univalence Axiom looks inevitable. It is not a bold claim. It is the formal expression of something that was always true and never before sayable.

## What Univalence Gives Us

The Univalence Axiom is not an isolated statement. It propagates through the entire type theory, giving us:

**Function extensionality.** Two functions that agree on all inputs are equal. This follows from univalence via the interval type.

**Propositional extensionality.** Two logically equivalent propositions are equal. A direct consequence.

**Structure invariance.** Any property of types that is expressible in type theory is automatically preserved by equivalences. No more "up to isomorphism" — the preservation is exact.

**The universe has interesting path structure.** The loop space of the universe at A is the type of self-equivalences of A. For A = Bool, this is Z/2Z. For A = Fin(n), this is the symmetric group Sₙ.

## How This Chapter is Organized

Section 1 develops the theory of equivalences carefully. The naive notion — "a function with a two-sided inverse" — is not a proposition. Three better notions are: bi-invertible maps (separate left and right inverses), half-adjoint equivalences (one inverse plus a coherence condition), and contractible fibers (every element has exactly one preimage). All three are propositions and are logically equivalent.

Section 2 states the Univalence Axiom precisely, defines the key map `idToEquiv : (A=B) → (A≃B)`, and states that univalence is the assertion that this map is an equivalence. The inverse `ua : (A≃B) → (A=B)` and its computation rule are the tools for using univalence in proofs.

Section 3 draws consequences: function extensionality, propositional extensionality, and the general structure invariance principle.

Section 4 gives concrete examples: the two paths on Bool, the automorphisms of finite sets, and the loop space of the universe.

After this chapter, nothing in HoTT is quite the same. Equality has been promoted from a relation to a type to a space to an equivalence. We are ready for higher inductive types, which are the spaces we want to reason about.
