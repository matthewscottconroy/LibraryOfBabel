# Groupoids

## The Simplest Higher Structure

A *groupoid* is a category where every morphism is an isomorphism — where every arrow has an inverse. Groupoids are thus the 1-dimensional analogue of homotopy types.

Why are groupoids the right bridge between category theory and homotopy theory? Because the fundamental group of a space is a special case of the fundamental *groupoid*. And the fundamental groupoid captures more information (it works with multiple basepoints, making it better-suited for spaces with complicated connectivity).

More deeply: every type in MLTT is a groupoid (when you truncate the identity types at level 1). The groupoid laws (transitivity, symmetry, reflexivity of the identity) correspond to the type-theoretic operations on paths. And UIP is exactly the statement that the groupoid is a discrete groupoid — a groupoid with only identity morphisms.

## Definition

**Definition.** A *groupoid* is a category $\mathcal{G}$ in which every morphism $f : a \to b$ is an isomorphism: there exists $f^{-1} : b \to a$ with $f^{-1} \circ f = \mathsf{id}_a$ and $f \circ f^{-1} = \mathsf{id}_b$.

Equivalently: a groupoid is a small category in which the hom-sets $\mathsf{Hom}(a, b)$ play the role of "path spaces" from $a$ to $b$.

**Examples:**

**Groups as groupoids.** A group $G$ is a groupoid with one object: $\mathbf{B}G$ has a single object $*$ and $\mathsf{Hom}(*, *) = G$. Composition is multiplication; inverses are group inverses. This is the delooping of $G$.

**Sets as discrete groupoids.** A set $S$ is a discrete groupoid: the groupoid with objects $S$ and only identity morphisms. The hom-set $\mathsf{Hom}(a,b) = \{*\}$ if $a = b$ and $\emptyset$ otherwise. Every morphism (only the identities) is an isomorphism (trivially). Sets embed into groupoids as the discrete objects.

**Equivalence relations as groupoids.** An equivalence relation $\sim$ on a set $S$ defines a groupoid: objects are elements of $S$; $\mathsf{Hom}(a,b) = \{*\}$ if $a \sim b$ and $\emptyset$ otherwise. This is the "setoid" groupoid.

**The fundamental groupoid.** For a topological space $X$, the *fundamental groupoid* $\Pi_1(X)$ has:
- Objects: points $x \in X$
- Morphisms $\Pi_1(X)(x, y)$: homotopy classes of paths from $x$ to $y$ (continuous maps $\gamma : [0,1] \to X$ with $\gamma(0) = x$, $\gamma(1) = y$, considered up to homotopy relative to endpoints)
- Composition: concatenation of paths
- Inverses: reversal of paths

The fundamental *group* $\pi_1(X, x_0) = \Pi_1(X)(x_0, x_0)$ is the automorphism group of the basepoint $x_0$ in the fundamental groupoid.

## Groupoids in Type Theory

In MLTT, every type $A$ gives a groupoid (when we restrict to 1-truncated types, but the structure is present in general).

**The groupoid of a type:**
- Objects: elements $a : A$
- Morphisms $a \to b$: paths $p : a =_A b$
- Composition: path concatenation $p \cdot q : a =_A c$ for $p : a =_A b$ and $q : b =_A c$
- Identity morphisms: $\mathsf{refl}_a : a =_A a$
- Inverses: $p^{-1} : b =_A a$ for $p : a =_A b$

The groupoid laws hold *propositionally* in MLTT:
- Associativity: $p \cdot (q \cdot r) =_{a=d} (p \cdot q) \cdot r$
- Left unit: $\mathsf{refl} \cdot p = p$
- Right unit: $p \cdot \mathsf{refl} = p$
- Left inverse: $p^{-1} \cdot p = \mathsf{refl}$
- Right inverse: $p \cdot p^{-1} = \mathsf{refl}$

These are not equalities but *paths between paths* — elements of 2-dimensional identity types. The groupoid laws are thus not strict but propositional.

**UIP and discrete groupoids.** UIP (Uniqueness of Identity Proofs) says: for any $p, q : a =_A b$, there is a path $p = q$ (i.e., $p$ and $q$ are propositionally equal). In groupoid terms: all morphisms between the same pair of objects are equal — the hom-sets have at most one element. This is exactly the condition that the groupoid is *equivalent to a discrete groupoid* (a set, viewed as a groupoid with only identity morphisms).

So UIP is the statement that types are (homotopy equivalent to) sets. This is false in the groupoid model: the hom-sets of the groupoid $\Pi_1(S^1)$ are infinite (one element for each integer, representing the winding number of a path).

## Functors and Natural Transformations of Groupoids

A *functor* between groupoids $F : \mathcal{G} \to \mathcal{H}$ is just a functor in the categorical sense: sends objects to objects, morphisms to morphisms, preserves composition and identities. Since all morphisms in groupoids are invertible, $F$ automatically sends inverses to inverses: $F(f^{-1}) = F(f)^{-1}$.

A *natural transformation* $\alpha : F \Rightarrow G$ between functors of groupoids is a family of morphisms $\alpha_x : F(x) \to G(x)$ satisfying naturality: for each $f : x \to y$, $\alpha_y \circ F(f) = G(f) \circ \alpha_x$.

In the groupoid model of MLTT: a functor $F : \Pi_1(A) \to \Pi_1(B)$ is a continuous map $f : A \to B$ (in the underlying topological sense). A natural transformation $\alpha : F \Rightarrow G$ is a homotopy $H : f \simeq g$ between continuous maps. The groupoid of groupoids is thus a model of the category of types and homotopy classes of maps.

## The 2-Category of Groupoids

Groupoids, functors, and natural transformations form a 2-category $\mathbf{Grpd}$. This 2-category is in fact a 2-*groupoid*: all 2-morphisms (natural transformations) are invertible (since in a groupoid, the natural transformation components $\alpha_x$ are invertible morphisms, so $\alpha^{-1}$ exists).

More is true: $\mathbf{Grpd}$ is the appropriate 2-categorical setting for 1-truncated homotopy theory. Topological spaces with $\pi_n = 0$ for $n \geq 2$ (called *1-types* or *$K(\pi, 1)$ spaces*) are classified, up to homotopy equivalence, by groupoids. Two such spaces are homotopy equivalent iff their fundamental groupoids are equivalent as groupoids.

For higher-dimensional spaces, you need ∞-groupoids.

## Groupoids and Sheaves

The connection between groupoids and sheaves is another face of the story. A *sheaf of groupoids* on a topological space $X$ is a functor from the category of open sets of $X$ (with restriction maps) to the category of groupoids. Sheaves of groupoids are the correct notion for moduli problems: the "moduli stack" of geometric objects is a sheaf of groupoids.

For example: the moduli stack of line bundles on a space $X$ is the sheaf of groupoids sending each open set $U \subseteq X$ to the groupoid of line bundles over $U$ (objects are line bundles, morphisms are isomorphisms). This sheaf encodes both the line bundles *and* their symmetries (automorphisms), in a way that a mere sheaf of sets (the "moduli space") would miss.

Stacks (sheaves of groupoids satisfying a descent condition) are the objects of the $(\infty,1)$-topos of stacks — the bridge between algebraic geometry and higher category theory.

## Groupoids in HoTT

In HoTT, the h-level hierarchy is the type-theoretic formulation of the groupoid picture:

- h-level $-2$ (contractible types): the groupoid is trivial — one object, one morphism
- h-level $-1$ (mere propositions / h-props): the groupoid has one object (up to path) — truth values
- h-level $0$ (sets / h-sets): the groupoid is discrete — all morphisms are identities
- h-level $1$ (1-types): the groupoid is a general groupoid — types with groupoid structure and UIP fails
- h-level $n$ (n-types): ∞-groupoids truncated at level $n$

The fundamental groupoid $\Pi_1(A)$ of a type $A$ is the groupoid you get by 1-truncating the ∞-groupoid of $A$: you keep the objects and 1-morphisms (paths) but identify all 2-morphisms. This is the propositional truncation of the identity types: $\Pi_1(A)(a, b) = \|a =_A b\|_0$ (the set-truncation of the path space).

For spaces where the fundamental groupoid is all you need — 1-types — the 1-truncation gives the full homotopy information. For higher-dimensional spaces, you need the full ∞-groupoid structure of all the identity types.
