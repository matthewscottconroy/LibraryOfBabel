# ∞-Groupoids and Kan Complexes

## The Right Notion of Space

What is a "space" in the homotopy-theoretic sense? Not a topological space per se — two homeomorphic spaces are "the same" in topology, but so are two spaces related by a homotopy equivalence. The right notion is a *homotopy type*: an equivalence class of topological spaces under weak homotopy equivalence.

The homotopy hypothesis tells us: homotopy types are the same as ∞-groupoids. The question is: what *is* an ∞-groupoid?

Multiple answers have been proposed: globular sets, simplicial sets (Kan complexes), cubical sets, complete Segal spaces, and others. All are equivalent as models of "∞-groupoids." The most commonly used in practice — and the one underlying Voevodsky's model of HoTT — is Kan complexes (fibrant simplicial sets).

## Simplicial Sets

**Definition.** The *simplex category* $\Delta$ has objects $[n] = \{0, 1, \ldots, n\}$ (for each $n \geq 0$) and morphisms the order-preserving maps.

A *simplicial set* is a functor $X : \Delta^{op} \to \mathbf{Set}$. Explicitly:
- $X_n = X([n])$ is the set of *$n$-simplices*
- *Face maps* $d_i : X_n \to X_{n-1}$ for $0 \leq i \leq n$ (deleting the $i$-th vertex)
- *Degeneracy maps* $s_i : X_n \to X_{n+1}$ for $0 \leq i \leq n$ (repeating the $i$-th vertex)

satisfying the *simplicial identities*:
$$d_i d_j = d_{j-1} d_i \quad (i < j)$$
$$s_i s_j = s_{j+1} s_i \quad (i \leq j)$$
$$d_i s_j = s_{j-1} d_i \quad (i < j), \quad d_j s_j = d_{j+1} s_j = \mathsf{id}, \quad d_i s_j = s_j d_{i-1} \quad (i > j+1)$$

**Geometric intuition:**
- $X_0$: points (0-simplices)
- $X_1$: edges (1-simplices), with face maps giving the two endpoints
- $X_2$: triangles (2-simplices), with face maps giving the three edges
- $X_3$: tetrahedra (3-simplices), and so on

## Standard Simplices and Horns

**Standard simplex $\Delta[n]$:** The representable simplicial set $\Delta[n] = \mathsf{Hom}_\Delta(-, [n])$. Its $k$-simplices are the order-preserving maps $[k] \to [n]$. Geometrically: the standard $n$-simplex (an $n$-dimensional solid).

**Boundary $\partial\Delta[n]$:** The simplicial set obtained by removing the non-degenerate $n$-simplex from $\Delta[n]$. Geometrically: the boundary of the $n$-simplex (an $(n-1)$-sphere).

**Horn $\Lambda^k[n]$:** The simplicial set obtained from $\Delta[n]$ by removing the $k$-th face and the interior. Geometrically: the $n$-simplex with the $k$-th face and interior removed — like a cup with one face open.

## Kan Complexes

**Definition.** A simplicial set $K$ is a *Kan complex* (or *fibrant simplicial set*) if for every horn $\Lambda^k[n] \to K$ (for any $n \geq 1$ and $0 \leq k \leq n$), there exists an extension to $\Delta[n] \to K$.

This is the *all-horn filling condition* (inner and outer horns). Geometrically: any "cup" with one face missing can be filled in.

**Why Kan complexes are ∞-groupoids.** The horn-filling condition encodes all the ∞-groupoid operations:

- *Composition*: given 1-simplices $f : x \to y$ and $g : y \to z$ (the inner horn $\Lambda^1[2]$), the filler gives a 2-simplex with the composition $g \circ f : x \to z$ as the missing face.
- *Inverses*: given a 1-simplex $f : x \to y$ (the outer horn $\Lambda^0[2]$ or $\Lambda^2[2]$), the filler gives a 2-simplex with the inverse $f^{-1} : y \to x$.
- *Associativity*: given three composable morphisms, the inner horn $\Lambda^2[3]$ gives a filler witnessing the associativity up to homotopy (a 2-simplex).
- *Higher coherences*: higher-dimensional fillers give all the higher coherence data.

The outer horn filling (for $\Lambda^0[n]$ and $\Lambda^n[n]$) is what ensures invertibility of morphisms — quasi-categories (inner horn filling only) have non-invertible morphisms, while Kan complexes have all morphisms invertible.

## The Fundamental ∞-Groupoid

For any topological space $X$, the singular simplicial set $\mathsf{Sing}(X)$ is a Kan complex. This is the fundamental ∞-groupoid of $X$:

$$\Pi_\infty(X) = \mathsf{Sing}(X)$$

Its structure:
- 0-simplices $\mathsf{Sing}(X)_0 = X$ (the points of $X$)
- 1-simplices $\mathsf{Sing}(X)_1 = \{$continuous maps $[0,1] \to X\}$ (paths in $X$)
- 2-simplices $\mathsf{Sing}(X)_2 = \{$continuous maps $\Delta^2 \to X\}$ (triangles / homotopies in $X$)
- $n$-simplices: continuous maps $\Delta^n \to X$

Composition of paths corresponds to the horn filling: given paths $f : x \to y$ and $g : y \to z$, the horn $\Lambda^1[2] \to \mathsf{Sing}(X)$ given by $f$ and $g$ fills to a 2-simplex, with the third edge being a composite path $g \cdot f : x \to z$.

The fundamental group $\pi_1(X, x) = \Pi_\infty(X)(x, x) / \sim$ (homotopy classes of loops). The higher homotopy groups $\pi_n(X, x)$ are the higher-dimensional automorphisms.

## Homotopy Groups of Kan Complexes

For a Kan complex $K$ and a 0-simplex (vertex) $v \in K_0$, the homotopy groups are defined combinatorially:

$$\pi_n(K, v) = \{f \in K_n : d_i(f) = s_0^{n-1}(v) \text{ for all } i\} / \sim$$

where two $n$-simplices with degenerate boundary at $v$ are homotopic if there is an $(n+1)$-simplex relating them. For $n = 0$: connected components. For $n = 1$: the fundamental group. For $n \geq 2$: abelian groups.

These homotopy groups agree with the topological homotopy groups via the Quillen equivalence: $\pi_n(|K|, |v|) \cong \pi_n(K, v)$.

## The Quillen Model Structure

The *Kan-Quillen model structure* on $\mathbf{sSet}$ is a model category structure where:
- *Weak equivalences*: simplicial maps inducing isomorphisms on all homotopy groups (and bijection on $\pi_0$)
- *Cofibrations*: monomorphisms (injective on all $K_n$)
- *Fibrations*: maps satisfying the right lifting property with respect to all horn inclusions (i.e., Kan fibrations)

In particular: Kan complexes are the *fibrant objects* of this model structure.

**The Quillen equivalence.** The adjunction $|{-}| \dashv \mathsf{Sing}$ between $\mathbf{sSet}$ and $\mathbf{Top}$ is a Quillen equivalence (both functors preserve weak equivalences between fibrant-cofibrant objects). This is the precise statement of the homotopy hypothesis.

## Multiple Models of ∞-Groupoids

Several equivalent models of ∞-groupoids exist, all Quillen equivalent to Kan complexes:

1. **Kan complexes** (simplicial sets satisfying all horn filling): the most commonly used in HoTT
2. **Complete Segal spaces** (Rezk): simplicial spaces satisfying Segal and completeness conditions
3. **Quasi-categories** (Joyal): not ∞-groupoids in general, but ∞-groupoids = quasi-categories where all 1-simplices are invertible
4. **Cubical sets** (Kan, and later CCHM): simplicial sets with cubes instead of simplices; underlying the cubical model of HoTT
5. **∞-groupoids as globular sets** (Batanin, Tamsamani): using globe-shaped cells instead of simplices

The equivalences between these models are theorems, not just analogies. Any construction in one model can be faithfully translated to another.

For HoTT: Voevodsky's model uses Kan complexes; cubical type theory uses cubical sets. Both give correct models of HoTT (with univalence), but their computational properties differ.

## ∞-Groupoids in HoTT

In HoTT, types are ∞-groupoids. The ∞-groupoid structure of a type $A$ is given by:

- 0-cells: elements $a : A$
- 1-cells (morphisms): paths $p : a =_A b$
- 2-cells: paths between paths $\alpha : p =_{a=b} q$
- $n$-cells: $n$-th iterated identity types

The composition of cells at each level is the path concatenation at that level. Inverses are path inversion. The coherence data (associativity, unit laws at each level) are all propositionally provable using path induction.

**The Kan condition for types.** In HoTT, types satisfy the Kan condition: horn filling corresponds to the fact that given "partial data" (a horn of paths and higher paths), you can always complete it to a full simplex. This is provable from the $J$ eliminator and the groupoid operations on paths.

The Kan property of types is what makes the simplicial set model work: types in HoTT are interpreted as Kan complexes, and the horn-filling conditions are satisfied because path induction allows you to complete any horn.

## The Univalence Axiom and the Object Classifier

The Univalence Axiom says: $(A =_\mathcal{U} B) \simeq (A \simeq B)$ — the path space of the universe is the space of equivalences.

In terms of Kan complexes: the universe $\mathcal{U}$ is interpreted as a Kan complex (the "universe of small Kan complexes"), and the path space $\mathsf{Path}_\mathcal{U}(A, B)$ is equivalent to the Kan complex of equivalences from $A$ to $B$.

This is the ∞-categorical *object classifier*: an object $\mathcal{U}$ in an ∞-topos such that maps $X \to \mathcal{U}$ correspond to "families of objects over $X$." The universal property of $\mathcal{U}$ is that any family of ∞-groupoids parametrized by $X$ corresponds to a map $X \to \mathcal{U}$.

Univalence is the statement that the path space of $\mathcal{U}$ is the right one: paths between types are equivalences, not mere bijections. This is what makes the universe behave correctly as an ∞-categorical object classifier.

The existence of such an object classifier in the ∞-category of Kan complexes ($\mathcal{S}$) is the key property that makes $\mathcal{S}$ an ∞-topos — and the model-theoretic fact that Voevodsky used to prove the consistency of HoTT + Univalence.
