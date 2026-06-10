# Chapter 16: Identity Types and Paths

## The Central Object of HoTT

If forced to name the single most important construction in HoTT, it would be the identity type. Not because it's the most complex (it's actually quite simply stated), but because it's the one that changes everything.

In classical mathematics, two things are either equal or they're not. Equality is a relation — it either holds or it doesn't. In HoTT, equality is a *type*: for any type $A$ and terms $a, b : A$, the identity type $a =_A b$ is a genuine type with potentially many elements.

An element $p : a =_A b$ is a *path* from $a$ to $b$ — a witness of equality, a reason why $a$ and $b$ are the same. There can be many different paths between the same two points (as there are many different paths from New York to London), and these paths can themselves be compared.

This might sound strange. But it's exactly what makes HoTT match homotopy theory: in a topological space, there can be many non-homotopic paths between two points, and the space of all paths from $a$ to $b$ has its own homotopy type.

## What's New in This Chapter

We've seen the identity type before (Chapter 9, MLTT). We've seen it categorically (Chapters 11, 15). Now we fully develop it with the homotopy-theoretic perspective in mind.

The key new ideas:
1. **Identity types are spaces.** Not just a formal relation, but a genuine type with its own elements and higher structure.
2. **Paths concatenate and invert.** The groupoid laws hold (but propositionally, not definitionally).
3. **Higher paths exist.** The type $p = q$ (paths between paths) forms 2-cells, and this tower goes on forever.
4. **Transport generalizes substitution.** Moving along a path in $A$ transports elements along a type family over $A$.
5. **ap is functoriality.** Every function is "continuous" — it maps paths to paths.
6. **Function extensionality** (unprovable from J alone) says homotopic functions are equal.

## The Homotopy Perspective Throughout

The driving metaphor throughout this chapter:
- Types are spaces
- Terms are points
- Identity proofs are paths
- Higher identity proofs are homotopies

We'll derive every construction from the J rule, but always with an eye toward what it means geometrically. The J rule says: to prove something about all paths, prove it for constant paths. This is path induction — the type-theoretic counterpart of homotopy invariance.

## Chapter Roadmap

**Section 1:** The identity type as a path space. The homotopy interpretation. Reflexivity as the constant path. Path induction (J rule) as contractibility of the total path space.

**Section 2:** Concatenation and inversion. The groupoid laws. Why they hold propositionally, not definitionally.

**Section 3:** Higher paths — paths between paths. The ∞-groupoid tower. The Eckmann-Hilton argument.

**Section 4:** Transport and ap. What it means to "move" along a path in a dependent type. Functions act on paths.

**Section 5:** Paths in specific types — products, dependent pairs, function types. Function extensionality.

## Prerequisites and Connections

Builds on:
- MLTT (Chapter 9): J rule, definitional vs. propositional equality
- Homotopy theory (Chapter 14): path spaces, fundamental group
- Simplicial sets (Chapter 15): path objects in the simplicial model

Connects forward to:
- h-levels (Chapter 17): when the path types are simple (sets, propositions)
- Univalence (Chapter 18): paths in the universe = equivalences
- HITs (Chapter 19): spaces defined by specifying paths
- Synthetic homotopy (Chapter 20): computing with paths
