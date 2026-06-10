# 4.1 Truncations as Higher Inductive Types

## Revisiting Truncations

In Chapter 17, we used propositional truncation $\|A\|$ and set truncation $\|A\|_0$ as key tools in the h-level hierarchy. We described them by their universal properties. Now we see how they're actually *defined* as HITs.

The key idea: a truncation is a HIT that adds paths to a type until all "unwanted" higher structure is made trivial. To make a type into a proposition, add paths between every pair of elements. To make a type into a set, add paths between every pair of paths.

## The Propositional Truncation

**Definition 4.1 (Propositional truncation as a HIT).** The *propositional truncation* $\|A\|$ is defined by:
- Constructor: $|{-}| : A \to \|A\|$
- Path constructor: $\mathsf{squash} : \prod_{x, y : \|A\|}\, x = y$

The path constructor says: any two elements of $\|A\|$ are equal. This forces $\|A\|$ to be a proposition.

**The eliminator.** To prove $P(t)$ for all $t : \|A\|$, where $P : \|A\| \to \mathsf{Type}$:
- If $P(t)$ is a mere proposition for all $t$: it suffices to prove $P(|a|)$ for all $a : A$.

More precisely: the elimination principle for $\|A\|$ into a family of propositions. The key restriction: you can only eliminate into propositions. You cannot extract elements of a non-propositional type from $\|A\|$.

**The universal property (revisited).** For a proposition $P$:
$$(A \to P) \simeq (\|A\| \to P)$$

This is exactly the statement that $\|A\|$ is the "free proposition on $A$" or the "propositional reflection of $A$."

**Why the restriction?** The squash condition identifies all elements of $\|A\|$. If you could eliminate into a non-propositional $B$, you'd need to send all elements of $\|A\|$ to the same element of $B$ (since any two elements are equal in $\|A\|$, and maps must respect equality). So you'd only get constant functions, which is not the same as eliminating into $A \to B$.

## The Set Truncation

**Definition 4.2 (Set truncation as a HIT).** The *set truncation* $\|A\|_0$ is defined by:
- Constructor: $|{-}|_0 : A \to \|A\|_0$
- 2-path constructor: $\mathsf{squash}_0 : \prod_{x, y : \|A\|_0}\, \prod_{p, q : x = y}\, p = q$

The 2-path constructor says: any two parallel paths in $\|A\|_0$ are equal. This forces all identity types of $\|A\|_0$ to be propositions, making $\|A\|_0$ a set.

**The universal property.** For a set $B$ (h-set):
$$(A \to B) \simeq (\|A\|_0 \to B)$$

Maps from $A$ to a set factor uniquely through $\|A\|_0$.

**Example:** $\|S^1\|_0 = \mathbf{1}$. The circle is connected, so its set of connected components has one element. The set truncation collapses the entire circle to a point.

## The General n-Truncation

**Definition 4.3 ($n$-truncation as a HIT).** For $n \geq -2$, the $n$-truncation $\|A\|_n$ is defined by:
- Constructor: $|{-}|_n : A \to \|A\|_n$
- For each $k > n$: a path constructor that makes all $k$-dimensional identity types contractible

More precisely, for $n \geq 0$: add path constructors that say "for any $(k+1)$-dimensional sphere in $\|A\|_n$ with $k > n$, there is a filler" — making $\|A\|_n$ into an $n$-type.

The general construction requires higher-dimensional path constructors (not just point and 1-path constructors), making it a "truly higher" inductive type.

**The universal property.** For an $n$-type $B$:
$$(A \to B) \simeq (\|A\|_n \to B)$$

$\|A\|_n$ is the "free $n$-type on $A$."

## Why Truncations Are HITs and Not Something Simpler

You might wonder: can't we define truncation without HITs, as a quotient or something?

For propositional truncation, the answer depends on what foundational tools you have:
- In set theory: $\|A\|$ is just "is $A$ nonempty?" — a proposition.
- In type theory without HITs: You'd need to axiomatize $\|A\|$ separately.
- In HoTT with HITs: $\|A\|$ is the HIT above, with a clear definition and computation rules.

For set truncation $\|A\|_0$, it's the set of connected components of $A$ — this can be defined as a quotient type (identify elements connected by paths), but the higher truncations genuinely need HITs (or higher quotients that are themselves HITs).

**The HIT definition is canonical.** The HIT definition of truncation satisfies the universal property definitionally — the eliminator exactly captures when you can map out of the truncation.

## Truncations and the Circle

The propositional truncation and the circle interact in interesting ways:

**$\|S^1\|_{-1} \simeq \mathbf{1}$.** The circle is inhabited (has a basepoint), so $\|S^1\|_{-1}$ is contractible (the proposition "the circle has an element" is true).

**$\|S^1\|_0 \simeq \mathbf{1}$.** The circle is connected (any two points are connected by a path), so its set of connected components is a single element.

**$\|S^1\|_1 \simeq S^1$.** The circle is already a 1-type (its $\pi_1 = \mathbb{Z}$ is the only non-trivial homotopy group, and $\pi_k(S^1) = 0$ for $k \geq 2$). So the 1-truncation doesn't change it.

**The Postnikov section $\tau_1 S^1 = S^1$.** This means $S^1$ is its own first Postnikov section — it has h-level 1 exactly.

## Truncations and Homotopy Groups

The $n$-truncation precisely captures the homotopy groups:

**Theorem 4.4.** For any type $A$ and basepoint $a : A$:
- $\pi_k(\|A\|_n, |a|_n) \cong \pi_k(A, a)$ for $k \leq n$
- $\pi_k(\|A\|_n, |a|_n) = \mathbf{1}$ for $k > n$

The truncation kills all homotopy groups above level $n$ and preserves those at or below level $n$.

**Proof sketch.** The universal property of $\|A\|_n$ and the fact that spheres $S^k$ are $k$-types (so maps $S^k \to \|A\|_n$ correspond to maps $S^k \to A$ for $k \leq n$, and are trivial for $k > n$). $\square$

**Corollary.** The sequence $A \to \|A\|_n \to \|A\|_{n-1} \to \cdots \to \|A\|_0 \to \|A\|_{-1}$ is the Postnikov tower of $A$, each map killing one layer of homotopy.

## The Loop Space Sequence

There's a beautiful interplay between suspension and truncation:

**Theorem 4.5.** $\Omega \|A\|_{n+1} \simeq \|\Omega A\|_n$ (the loop space of the $(n+1)$-truncation is the $n$-truncation of the loop space).

*Proof sketch.* The loop space "lowers the h-level by 1": if $A$ is an $n$-type, then $\Omega A$ is an $(n-1)$-type. So $\|A\|_{n+1}$ is an $(n+1)$-type, and $\Omega \|A\|_{n+1}$ is an $n$-type. One shows that this $n$-type has the right universal property to be the $n$-truncation of $\Omega A$. $\square$

**Consequence for homotopy groups.** The $n$-th homotopy group $\pi_n(A, a) = \pi_1(\Omega^{n-1}(A, a))$ can be computed by:
$$\pi_n(A, a) = \|\Omega^n(A, a)\|_0$$

(the 0-truncation of the $n$-fold loop space).

And the fibration sequence for truncations:
$$\|\Omega A\|_{n-1} \to \|A\|_n \to \|A\|_{n-1}$$

gives a long exact sequence relating consecutive levels of the Postnikov tower.

## Classifying Space HITs

A special class of HITs worth mentioning: *classifying spaces* $BG$ for a group $G$.

**Definition 4.6 (Classifying space / Delooping).** For a group $G$, the classifying space $BG$ is the 1-type with:
- One point $* : BG$
- Loops: $\Omega(BG, *) \simeq G$

In HoTT, $BG$ can be constructed as a HIT:
- Point constructor: $* : BG$
- 1-path constructor: $g : * = *$ for each $g : G$ (one loop for each group element)
- 2-path constructor: $\mathsf{mul}(g, h) : g \cdot h = g_1 \cdot h_1$ (paths enforcing the group law)
- Truncation: $BG$ is truncated to a 1-type

The exact construction requires some care to ensure the 2-paths give the correct group multiplication.

**Why classifying spaces matter.** $BG$ classifies $G$-torsors: maps $X \to BG$ correspond to $G$-principal bundles over $X$. In synthetic HoTT, this correspondence is proved using the HIT structure of $BG$ and the universal property.

**The case $G = \mathbb{Z}$.** $B\mathbb{Z}$ is the circle $S^1$! This is because $\Omega(S^1, \mathsf{base}) \simeq \mathbb{Z}$ (the main theorem of synthetic homotopy theory). So the circle is the classifying space of the integers — a beautiful connection between topology, algebra, and type theory.

## Summary

Truncations, as HITs, give:

| HIT | What it does | Universal property |
|---|---|---|
| $\|A\|_{-1}$ | Forces $A$ to be a proposition | $A \to P \simeq \|A\|_{-1} \to P$ for propositions $P$ |
| $\|A\|_0$ | Forces $A$ to be a set | $A \to S \simeq \|A\|_0 \to S$ for sets $S$ |
| $\|A\|_n$ | Forces $A$ to be an $n$-type | $A \to T \simeq \|A\|_n \to T$ for $n$-types $T$ |

The HIT definition makes the universal property a theorem (following from the eliminator) rather than an axiom. And it gives truncations a canonical position in the type theory: they're not special-purpose additions but instances of a general pattern (higher inductive types with truncation constructors).

The interplay between truncations, suspensions, and loop spaces is the heart of synthetic homotopy theory — the tools that let us compute homotopy groups and prove classical theorems purely within type theory.
