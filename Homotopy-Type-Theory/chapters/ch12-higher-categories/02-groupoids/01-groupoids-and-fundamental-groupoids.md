# 2.1 Groupoids and the Fundamental Groupoid

## What is a Groupoid?

You already know what a group is: a set with a binary operation that's associative, has an identity, and has inverses. A groupoid is a generalization: instead of having one object (the operation applies to everything in the set), a groupoid has *multiple objects*, and the "multiplication" only makes sense between compatible pairs.

The formal definition is clean:

**Definition 2.1 (Groupoid).** A *groupoid* is a small category in which every morphism is an isomorphism.

That's it. Every arrow $f : a \to b$ in the category has a two-sided inverse $f^{-1} : b \to a$ with $f^{-1} \circ f = \mathsf{id}_a$ and $f \circ f^{-1} = \mathsf{id}_b$.

Let's unpack what this gives us. A groupoid $\mathcal{G}$ has:
- A set $\mathsf{Ob}(\mathcal{G})$ of *objects* (sometimes called *vertices* or *points*)
- For each pair of objects $a, b$: a set $\mathsf{Hom}(a, b)$ of *morphisms from $a$ to $b$*
- For each object $a$: an identity morphism $\mathsf{id}_a : a \to a$
- Composition: $g \circ f : a \to c$ for $f : a \to b, g : b \to c$
- Inverses: $f^{-1} : b \to a$ for $f : a \to b$

satisfying the usual associativity, unit, and inverse laws.

## Groups vs. Groupoids

The relationship is simple: a group is a one-object groupoid.

If $\mathcal{G}$ has exactly one object $*$, then $\mathsf{Hom}(*,*)$ is a set, and composition gives it the structure of a group (the identity morphism is the unit, and all morphisms are invertible by definition). So $\mathcal{G}$ is exactly a group.

Going the other way: given a group $G$, we can form the *delooping* $\mathbf{B}G$ — the one-object groupoid with $\mathsf{Hom}(*,*) = G$.

This perspective is useful throughout HoTT. When we say "a type is a group" (in some suitable sense), what we really mean is "its loop space has a group structure," which is reflected in the fact that for a type $A$ with a distinguished point $a : A$, the type $a =_A a$ (the type of loops at $a$) forms a group.

## Examples

**Discrete groupoids.** Given any set $S$, the *discrete groupoid* has $S$ as objects and only identity morphisms. No non-trivial morphisms exist. Categorically, this is the "set viewed as a category with only identities."

**Pair groupoid.** Given a set $S$, the *pair groupoid* $S \times S$ has $S$ as objects and exactly one morphism between any two objects (i.e., $\mathsf{Hom}(a,b) = \{*\}$ for all $a, b \in S$). This is the "maximally connected" groupoid on $S$.

**Fundamental groupoid** (see below). The most important example for us: the groupoid $\Pi_1(X)$ of a topological space.

**Gauge groupoid.** In physics: given a principal $G$-bundle $P \to X$, the *gauge groupoid* has points of $X$ as objects and isomorphisms of fibers as morphisms.

**Action groupoid.** Given a group $G$ acting on a set $S$: objects are elements of $S$, morphisms $s \to t$ are group elements $g \in G$ with $g \cdot s = t$. This captures the orbit structure.

## The Fundamental Groupoid

Here's the most important groupoid for our purposes.

**Definition 2.2 (Fundamental Groupoid).** For a topological space $X$, define the *fundamental groupoid* $\Pi_1(X)$ as follows:
- Objects: points $x \in X$
- Morphisms from $x$ to $y$: homotopy classes of continuous paths $\gamma : [0,1] \to X$ with $\gamma(0) = x, \gamma(1) = y$
- Composition: concatenation of paths (then take homotopy class)
- Identity at $x$: the constant path $c_x(t) = x$ for all $t$
- Inverse of $[\gamma]$: the reversed path $[\bar\gamma]$ where $\bar\gamma(t) = \gamma(1-t)$

We need to check this is well-defined:
- Composition is well-defined: if $\gamma \simeq \gamma'$ and $\delta \simeq \delta'$ (rel endpoints), then $\delta \circ \gamma \simeq \delta' \circ \gamma'$
- Inverses work: $[\bar\gamma] \circ [\gamma] = [\mathsf{id}_x]$ (the concatenation of a path with its reverse is homotopic to the constant path)
- Associativity: $(\delta \circ \gamma) \circ \eta \simeq \delta \circ (\gamma \circ \eta)$ (path concatenation is associative up to homotopy)

All these are standard results in algebraic topology. The key insight: we *need* to take homotopy classes of paths (not paths themselves) to get strict associativity. Without the quotient, concatenation is only associative up to homotopy — giving a "weak" groupoid structure.

**The fundamental group as a special case.** The *fundamental group* $\pi_1(X, x_0)$ at a basepoint $x_0$ is the automorphism group of $x_0$ in $\Pi_1(X)$:
$$\pi_1(X, x_0) = \mathsf{Aut}_{\Pi_1(X)}(x_0) = \mathsf{Hom}_{\Pi_1(X)}(x_0, x_0)$$

The fundamental group only sees the loops at a single basepoint. The fundamental groupoid sees everything: it captures not just loops but paths between any two points, making it basepoint-free.

**Why the fundamental groupoid is better.** The standard textbook approach to fundamental groups requires choosing a basepoint, and statements about fundamental groups always come with basepoint dependencies. Changing the basepoint changes the group (to an isomorphic but not equal group), and tracking these isomorphisms is messy.

The fundamental groupoid sidesteps this completely: it's defined without choosing a basepoint, and it contains all the basepoint information simultaneously. The fundamental groups at different basepoints are the automorphism groups at the corresponding objects.

For a path-connected space, all the fundamental groups $\pi_1(X, x_0)$ are isomorphic (via conjugation by any path from $x_0$ to $x_1$), and the fundamental groupoid is equivalent (as a groupoid) to the one-object groupoid $\mathbf{B}\pi_1(X, x_0)$ for any basepoint. But for non-connected spaces (or spaces where connectivity matters), the fundamental groupoid is strictly more informative.

## Computing $\Pi_1(X)$ for Key Spaces

**$\Pi_1(\text{point})$.** A single point has only the identity morphism. $\Pi_1(*) \cong \mathbf{B}\{e\}$ (the one-object groupoid with trivial group).

**$\Pi_1([0,1])$.** The interval $[0,1]$ is contractible. Any two paths from $x$ to $y$ are homotopic (there's a unique homotopy class for each pair $(x,y)$). So $\Pi_1([0,1])$ is the pair groupoid of $[0,1]$.

**$\Pi_1(S^1)$.** The circle. Fix basepoint $* \in S^1$. The loops at $*$ form the group $\pi_1(S^1,*) = \mathbb{Z}$ (the integers, with generator being "go around once"). So $\mathsf{Aut}_{\Pi_1(S^1)}(*) = \mathbb{Z}$. For any two points $x, y \in S^1$, there's exactly one homotopy class of paths from $x$ to $y$ for each integer $n$ (the path that "wraps around $n$ times while going from $x$ to $y$"). So $\Pi_1(S^1) \cong \mathbf{B}\mathbb{Z}$ as a groupoid (since $S^1$ is connected, up to equivalence it's the one-object groupoid with automorphism group $\mathbb{Z}$).

**$\Pi_1(\mathbb{R}^n)$.** Euclidean space is contractible. Any path between two points is homotopic to the unique straight-line path. So $\Pi_1(\mathbb{R}^n)$ is the pair groupoid of $\mathbb{R}^n$ — there's exactly one morphism from $x$ to $y$ for any $x, y \in \mathbb{R}^n$.

## Types in MLTT are Groupoids

Now let's connect this to type theory. We've already seen (Chapter 9) that every type $A$ has:
- Objects: terms $a : A$
- "Morphisms" from $a$ to $b$: identity proofs $p : a =_A b$
- Composition: path concatenation (defined via J)
- Identities: reflexivity $\mathsf{refl}_a$
- Inverses: path inversion (defined via J)

**Theorem 2.3 (Types are Groupoids).** Every type $A$ in MLTT is a groupoid with respect to its identity type structure.

This means: the groupoid laws — associativity, left/right units, left/right inverses — all hold for path concatenation.

But here's the subtlety: these laws hold *propositionally*, not definitionally. That is, there are proofs:
- $\mathsf{assoc} : (p \cdot q) \cdot r =_{a =_A d} p \cdot (q \cdot r)$
- $\mathsf{left\_unit} : \mathsf{refl} \cdot p =_{a =_A b} p$
- $\mathsf{right\_unit} : p \cdot \mathsf{refl} =_{a =_A b} p$
- $\mathsf{left\_inv} : p^{-1} \cdot p =_{a =_A a} \mathsf{refl}$
- $\mathsf{right\_inv} : p \cdot p^{-1} =_{a =_A b'} \mathsf{refl}$ (wait, this doesn't typecheck directly)

Actually: $p \cdot p^{-1} =_{b =_A b} \mathsf{refl}$ and $p^{-1} \cdot p =_{a =_A a} \mathsf{refl}$.

These equalities are themselves elements of identity types — they're *paths between paths*, i.e., 2-dimensional structure. And the groupoid laws between these 2-dimensional paths form a further layer, and so on.

This is why types are not just groupoids — they're *∞-groupoids*. The groupoid structure at each level is encoded in the next level's identity types.

## Groupoids in HoTT: The Language of 0-Types

In HoTT, groupoids appear as a special case: the 1-types (or *h-groupoids*). A type $A$ is a 1-type (or *groupoid-level*) if all its 2-dimensional identity types are trivial (i.e., any two paths between paths are equal). Formally:

$$\mathsf{is\text{-}1\text{-}type}(A) := \prod_{a, b : A} \prod_{p, q : a = b} \prod_{r, s : p = q} (r = s)$$

For a 1-type, the path structure is exactly a groupoid: objects, morphisms (paths), with the groupoid laws. Higher structure is trivial.

Sets (0-types) are even simpler: all path types are trivial. They're discrete groupoids (only identity morphisms).

And at the other extreme, types with non-trivial homotopy at all levels (like the circle $S^1$ or $K(\mathbb{Z},1)$) are genuine ∞-groupoids.

## The Hofmann-Streicher Groupoid Model

We saw in Chapter 11 that the *groupoid model* of MLTT (Hofmann-Streicher, 1994) is the first model where UIP fails:
- Types are (small) groupoids
- Terms of type $A$ are objects of the groupoid $A$
- The identity type $a =_A b$ is the set of morphisms $\mathsf{Hom}_A(a, b)$
- Reflexivity: the identity morphism $\mathsf{id}_a$
- J rule: path induction via the "transport" structure of groupoids

In this model, UIP says: any two morphisms $f, g : a \to b$ are equal. But in a groupoid, $\mathsf{Hom}(a,b)$ can have multiple elements! For example, in $\mathbf{B}\mathbb{Z}$ (the one-object groupoid with automorphisms $\mathbb{Z}$), $\mathsf{Hom}(*,*)= \mathbb{Z}$, which has many elements.

So UIP fails in the groupoid model. This was the first proof that MLTT without UIP is consistent with having types that have non-trivial identity type structure.

The groupoid model doesn't validate Univalence (it's "only" 1-dimensional — it doesn't see higher homotopy groups). For full Univalence, we need the simplicial set model. But the groupoid model was historically crucial for establishing that UIP is not provable in MLTT.

## Summary

| Structure | Example | Key Property |
|---|---|---|
| Group | $(\mathbb{Z}, +)$ | One object, all morphisms invertible |
| Discrete groupoid | A set $S$ | Many objects, only identities |
| Pair groupoid on $S$ | Any set | One morphism between any pair |
| $\mathbf{B}G$ | Delooping of $G$ | One object, automorphisms = $G$ |
| $\Pi_1(X)$ | Fundamental groupoid | Objects = points, morphisms = path classes |
| Type $A$ in MLTT | Any type | Objects = terms, morphisms = identity proofs |

Groupoids sit at the first rung of the ∞-groupoid ladder. They're rich enough to fail UIP (as the groupoid model shows) but not rich enough to model Univalence or higher homotopy theory. For that, we need the full ∞-groupoid structure — which is what we build next.
