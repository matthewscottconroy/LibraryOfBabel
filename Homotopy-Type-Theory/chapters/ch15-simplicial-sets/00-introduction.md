# Chapter 15: Simplicial Homotopy Theory

## The Key Idea

Topology is beautiful but messy. To talk about a topological space rigorously, you need to work with open sets, continuous functions, $\varepsilon$-$\delta$ arguments, and all the machinery of point-set topology. This is fine when you're actually working with specific spaces, but it's inconvenient for the *foundations* of homotopy theory — the subject that studies spaces only up to homotopy equivalence.

What if we could capture homotopy-theoretic information *combinatorially*, without any topology at all? No continuous functions, no open sets — just sets and maps between them, organized in a specific pattern. That's what simplicial sets do.

A **simplicial set** is a purely combinatorial object: a collection of "simplices" (vertices, edges, triangles, tetrahedra, ...) organized by face and degeneracy maps. No metric, no topology. Just combinatorial data.

And yet — and this is the key theorem — **Kan complexes** (a special class of simplicial sets satisfying a "horn-filling condition") are equivalent to homotopy types. You can do all of homotopy theory in the combinatorial world of simplicial sets, without ever invoking topological spaces.

This matters for HoTT because the simplicial set model is how we know HoTT is consistent. Voevodsky's model:
- Types = Kan complexes
- Terms = elements of simplicial sets
- Identity types = path spaces (simplicial path spaces)
- Univalence = a theorem about the "universe" Kan complex

Understanding simplicial sets is understanding *why* HoTT works.

## The Triangle Picture

Here's the big picture with three connected worlds:

```
Topological Spaces
       ↕   (Quillen equivalence)
Simplicial Sets (Kan complexes)
       ↕   (HoTT interpretation)  
Types in HoTT
```

- The top equivalence (Quillen, 1967): Kan complexes and topological spaces carry the same homotopy-theoretic information. You can translate freely between them.

- The bottom equivalence (Voevodsky, 2009): Types in HoTT correspond to Kan complexes. The type-theoretic operations (Π, Σ, identity types, universes) correspond to simplicial-set operations (products, fibrations, path spaces, universe objects).

Together: HoTT captures exactly the homotopy-theoretic content of topological spaces.

## Why Simplicial Sets?

Before simplicial sets, homotopy theory was done entirely in topological spaces. Simplicial sets offer several advantages:

**1. Combinatorial and computable.** A simplicial set is just a collection of sets with maps between them. There's nothing "continuous" — no topology, no analysis. This makes it easier to define things precisely and to work computationally.

**2. Model structure is cleaner.** The Quillen model structure on simplicial sets is one of the cleanest known model structures: cofibrations are just injections, fibrations (Kan fibrations) have an elegant universal property, and weak equivalences are well-behaved.

**3. The universe is itself a Kan complex.** The most important feature for HoTT: the "universe" of small Kan complexes is itself a Kan complex. This is what makes Univalence true — paths in the universe correspond to equivalences of types.

**4. Connection to higher category theory.** Simplicial sets are the natural model for (∞,1)-categories (quasi-categories), (∞,∞)-categories, and all of higher category theory. They're the lingua franca of modern abstract homotopy theory.

## Chapter Roadmap

**Section 1 (The Simplex Category):** The simplex category $\Delta$: objects are finite ordinals, morphisms are order-preserving maps. Face and degeneracy maps. The simplicial identities.

**Section 2 (Simplicial Sets):** A simplicial set as a functor $\Delta^{op} \to \mathbf{Set}$. Vertices, edges, triangles. Standard simplices $\Delta[n]$. Geometric realization. The singular complex of a topological space.

**Section 3 (Kan Complexes):** The horn-filling condition. Why Kan complexes model ∞-groupoids. Homotopy groups of Kan complexes. The Quillen equivalence between Kan complexes and homotopy types.

**Section 4 (The Model Structure):** The Quillen model structure on simplicial sets. Kan fibrations. Cofibrations as injections. The role of model structures in formalizing homotopy theory.

**Section 5 (The HoTT Model):** Voevodsky's simplicial set model of HoTT. How each type-theoretic construction corresponds to a simplicial construction. Why Univalence holds.

## Prerequisites and Connections

This chapter builds on:
- Category theory (Chapter 10): functors, natural transformations, adjunctions
- Topology (Chapter 13): topological spaces, continuous maps
- Homotopy theory (Chapter 14): homotopy equivalences, fundamental group

This chapter connects to:
- Identity types (Chapter 16): identity types are path spaces in the simplicial model
- Univalence (Chapter 18): the central theorem about the simplicial model
- Cubical type theory (Chapter 23): the cubical analog of simplicial sets

## A Note on Technicality

This chapter is more technically dense than the previous ones. Simplicial sets require careful attention to the combinatorics of face and degeneracy maps, and the model structure requires understanding lifting properties.

The key insight to hold onto throughout: simplicial sets are just a very clever way of organizing combinatorial data. The face maps tell you "which face you're looking at"; the degeneracy maps tell you "this is a trivial simplex that came from a lower dimension." Everything else follows from being systematic about these two operations.

The Kan condition (horn-filling) is the key property: it says "given all but one face, you can fill in the missing one." This is exactly the property that makes a simplicial set model a space (where you can always complete a partial map), and specifically an ∞-groupoid (where you can always invert morphisms and compose them).
