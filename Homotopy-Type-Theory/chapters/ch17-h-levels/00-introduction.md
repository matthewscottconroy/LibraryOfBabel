# Chapter 17: H-Levels and Truncations

## Not All Types Are Created Equal

We've been building up HoTT's picture of types as spaces, paths as identities, and the rich homotopy structure that emerges. But there's a striking variation: not all types have equally complex structure.

Some types are dead simple — there's only one element, and the only path is reflexivity. Other types have finitely many elements but no interesting paths between them. Still others have elements and interesting paths but nothing above the paths. And then there are types like the circle $S^1$, where the path structure is genuinely infinite and nontrivial.

The concept of *h-level* (homotopy level) gives us a precise vocabulary for this variation. It measures "how much higher homotopy structure" a type has, and it forms a hierarchy:

$$\text{Contractible} \subset \text{Propositions} \subset \text{Sets} \subset \text{1-Types} \subset \text{2-Types} \subset \cdots$$

This hierarchy isn't just a mathematical curiosity — it's one of the organizing principles of HoTT. It tells us:
- **When can we ignore proof terms?** Exactly when we're working with propositions.
- **When does classical set-theoretic mathematics apply?** Exactly when our types are sets.
- **How does the complexity of structure scale?** Predictably through the h-level hierarchy.

## The Central Definitions

The h-level hierarchy is defined by how far the tower of identity types goes before becoming trivial:

- **Contractible** ($(-2)$-type): There is exactly one element, up to paths.
- **Propositions** ($(-1)$-type): Any two elements are equal (there is at most one element up to equality).
- **Sets** ($0$-type): Paths between elements are unique when they exist (no interesting 2-paths).
- **1-Types** ($1$-type): 2-paths between paths are unique (no interesting 3-paths).
- **$n$-Types**: The tower stabilizes at level $n$.

And for types that don't fit any level — types like the circle, the 2-sphere, the universe — the tower never fully stabilizes.

## The Key Operations: Truncations

Given a type $A$ at any h-level, we can "force" it down to a lower h-level by taking its *truncation* $\|A\|_n$. The $n$-truncation keeps all information up to h-level $n$ and collapses everything above.

The most important truncations are:
- $\|A\| = \|A\|_{-1}$: The *propositional truncation* — the mere proposition that $A$ is inhabited.
- $\|A\|_0$: The *set truncation* — the set of connected components of $A$.

Truncations are higher inductive types, which we'll define properly in Chapter 19. Here, we focus on their properties and the h-level hierarchy itself.

## Why H-Levels Matter

The h-level hierarchy matters for several reasons:

**Proof relevance.** In HoTT, proofs are terms, and different proofs of the same thing can be different terms. For propositions, all proofs are equal — proof is irrelevant. For sets, paths between elements are propositions. As we go up the h-level hierarchy, proof relevance increases at each level.

**Mathematical practice.** Most of everyday mathematics happens at h-level 0 (sets) or h-level 1 (groupoids). The h-level hierarchy gives a precise framework for where different mathematical concepts live.

**The Curry-Howard correspondence.** Propositions (h-level $-1$) correspond to logical propositions in the traditional sense. Sets (h-level 0) correspond to mathematical sets. The hierarchy extends this correspondence upward into higher-dimensional mathematics.

**Univalence stratification.** The Univalence axiom (Chapter 18) says that paths in the universe correspond to equivalences. The h-level of types in the universe reflects the complexity of their automorphism groups — propositions have trivial automorphisms (the identity), while sets have discrete automorphism groups, and higher types can have rich automorphism structures.

## Chapter Roadmap

**Section 1:** Contractible types — the simplest types, equivalent to the unit type. They're the "base case" of the h-level hierarchy.

**Section 2:** Mere propositions (h-props) — types where any two elements are equal. They play the role of truth values in the constructive logic within HoTT.

**Section 3:** Sets (h-sets) — types with discrete path structure. Classical mathematics happens here. Hedberg's theorem: decidable equality implies h-set.

**Section 4:** General $n$-types — the full hierarchy. What it means to be an $n$-type, the cumulative structure, and connections to classical homotopy theory.

**Section 5:** Truncations — the operation that forces any type to be an $n$-type. Propositional and set truncation in detail, with their universal properties.

## Prerequisites and Connections

Builds on:
- Chapter 16 (Identity Types): The tower of identity types, transport, groupoid structure
- Chapter 15 (Simplicial Sets): The simplicial model and the homotopy-theoretic interpretation

Connects forward to:
- Chapter 18 (Univalence): Paths in the universe, equivalences, structure invariance
- Chapter 19 (Higher Inductive Types): Truncations as HITs, the circle as a non-set
- Chapter 20 (Synthetic Homotopy): Computing homotopy groups, which requires h-levels
