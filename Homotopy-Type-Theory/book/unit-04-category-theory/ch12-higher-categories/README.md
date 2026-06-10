# Chapter 12: Higher Category Theory and the Homotopy Hypothesis

## The Hook: Grothendieck's Letter

In 1983, Alexander Grothendieck wrote a letter to Daniel Quillen that ran to hundreds of pages. The letter was titled "Pursuing Stacks." In it, Grothendieck formulated what he called the "homotopy hypothesis":

*Homotopy types are the same as ∞-groupoids.*

This was not a vague philosophical claim. Grothendieck meant it precisely: there should be an equivalence of mathematical structures between homotopy types (topological spaces considered up to weak homotopy equivalence) and ∞-groupoids (higher categorical structures where every morphism at every level is invertible).

Grothendieck was 55 years old and had largely withdrawn from official mathematical life five years earlier. Yet this letter, circulated informally, shaped the development of higher category theory for the next four decades.

Why is the homotopy hypothesis deep? Because it unifies two apparently different mathematical traditions:
- *Algebraic topology*: the study of spaces and their homotopy groups, built on continuous maps, deformation retractions, and geometric intuition
- *Category theory*: the abstract study of mathematical structure, built on objects, morphisms, and composition

The hypothesis says these traditions are studying the same thing. A homotopy type is an ∞-groupoid. An ∞-groupoid is a homotopy type. There is no difference.

And this matters for HoTT: types in MLTT are ∞-groupoids, by virtue of their iterated identity type structure. So types in HoTT *are* homotopy types. Not by analogy — by identity.

## The Connection to Types

Here is why every MLTT type is an ∞-groupoid.

A type $A$ has elements (points). Between any two elements $a, b : A$, there is the identity type $a =_A b$ — the type of proofs that $a = b$. We call elements of $a =_A b$ *paths* from $a$ to $b$.

Between two paths $p, q : a =_A b$, there is the identity type $p =_{a=_Ab} q$ — the type of proofs that two paths are equal. We call these *2-paths* or *homotopies*.

This tower continues: 3-paths, 4-paths, and so on. And at every level, the paths are invertible: if $p : a = b$ then $p^{-1} : b = a$; if $\alpha : p = q$ then $\alpha^{-1} : q = p$; and the composition of a path with its inverse is (homotopic to) the identity.

This is exactly the data of an ∞-groupoid: a collection of objects (elements), morphisms (paths), 2-morphisms (2-paths), ... with all morphisms invertible at every level.

So the homotopy hypothesis *for MLTT* is a theorem (in a precise sense): types in MLTT form ∞-groupoids, and the ∞-groupoid structure is encoded in the identity types and the operations on them (concatenation, inversion, $J$ eliminator).

## What This Chapter Covers

**Section 1: 2-Categories and Bicategories.** The first step above ordinary categories. Objects, 1-morphisms, and 2-morphisms. Strict vs. weak: in a strict 2-category, composition is strictly associative; in a bicategory, it is associative only up to a natural isomorphism (the associator). The coherence theorem for bicategories: every bicategory is equivalent to a strict 2-category.

**Section 2: Groupoids.** The 1-dimensional version of homotopy types. Groupoids are categories where every morphism is invertible. The fundamental groupoid $\Pi_1(X)$ of a topological space captures its path structure. Types in MLTT are groupoids (1-groupoids), and UIP is the statement that all 2-paths are trivial.

**Section 3: The Homotopy Hypothesis.** Grothendieck's conjecture and its various formulations. Simplicial sets as a model: Kan complexes are ∞-groupoids. The Quillen equivalence between simplicial sets and topological spaces. The higher groupoid structure of types in MLTT.

**Section 4: (∞,1)-Categories.** The generalization of ordinary categories to the homotopy-coherent setting. In an (∞,1)-category, all k-morphisms for k ≥ 2 are invertible. Quasi-categories (Joyal, Lurie): simplicial sets satisfying the inner horn filling condition. The (∞,1)-category of Kan complexes is the prototypical example.

**Section 5: ∞-Groupoids and Kan Complexes.** Multiple equivalent definitions of ∞-groupoids. Kan complexes: simplicial sets where all horns fill. The Quillen model structure on simplicial sets: fibrant objects are Kan complexes. Why Kan complexes are the right notion of "space" in the ∞-categorical setting. The connection to HoTT: types are Kan complexes in Voevodsky's model.

## Why This Is Difficult

Higher category theory is difficult because *strict* is easy to define but *weak* is what actually occurs in nature.

A strict 2-category has associativity on the nose: $(h \circ g) \circ f = h \circ (g \circ f)$. The strict axioms are easy to write down and reason about. But most naturally occurring 2-categories are not strict: in the 2-category $\mathbf{Cat}$ of categories, functors, and natural transformations, composition of functors is associative *up to natural isomorphism*, not literally.

When you go to ∞-categories, the coherence conditions — the data needed to say "composition is associative up to higher isomorphisms, and these isomorphisms satisfy coherence conditions that are themselves coherent, and so on" — become combinatorially complex. The simplicial approach (Joyal-Lurie) bypasses this by encoding all the coherences implicitly in the horn-filling conditions.

This is where the mathematical depth lies. The homotopy hypothesis is not just a slogan — it is a theorem that requires the full machinery of simplicial homotopy theory to state and prove.
