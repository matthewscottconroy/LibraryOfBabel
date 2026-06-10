# 2.1 Continuous Maps and Homeomorphisms

## The Definition of Continuity

We've set up topological spaces precisely so we can define continuity cleanly.

**Definition 2.1 (Continuous Map).** A function $f : X \to Y$ between topological spaces is *continuous* if for every open set $V \subseteq Y$, the preimage $f^{-1}(V) = \{x \in X \mid f(x) \in V\}$ is open in $X$.

Equivalently (and more categorically): $f$ is continuous iff $f^{-1}$ sends open sets to open sets, i.e., $f^{-1} : \tau_Y \to \tau_X$.

**Checking continuity.** To verify $f$ is continuous, you only need to check preimages of *basis elements* (since all open sets are unions of basis elements, and preimages preserve unions). For the standard topology on $\mathbb{R}$, you only need to check that preimages of open intervals are open.

**Examples:**
- Every constant function $f(x) = c$ is continuous: $f^{-1}(V) = X$ if $c \in V$, and $\emptyset$ if $c \notin V$ — both open.
- The identity $\mathsf{id}_X : X \to X$ is continuous: $\mathsf{id}_X^{-1}(V) = V$, which is open.
- Projections $\pi_1 : X \times Y \to X$ are continuous: $\pi_1^{-1}(U) = U \times Y$, which is open in the product topology.

**Theorem 2.2.** The composition of continuous maps is continuous.

*Proof.* Let $f : X \to Y$ and $g : Y \to Z$ be continuous. For any open $W \subseteq Z$:
$$(g \circ f)^{-1}(W) = f^{-1}(g^{-1}(W))$$
Since $g$ is continuous, $g^{-1}(W)$ is open in $Y$. Since $f$ is continuous, $f^{-1}(g^{-1}(W))$ is open in $X$. So $g \circ f$ is continuous. $\square$

This theorem, combined with the fact that identities are continuous, establishes:

**Corollary 2.3.** Topological spaces and continuous maps form a category, denoted **Top**.

## Characterizations of Continuity

There are several equivalent ways to state continuity — each illuminates a different aspect.

**Theorem 2.4 (Equivalent Characterizations).** For $f : X \to Y$, the following are equivalent:
1. $f$ is continuous ($f^{-1}$ preserves open sets)
2. $f^{-1}$ preserves closed sets: for every closed $C \subseteq Y$, $f^{-1}(C)$ is closed in $X$
3. For every $x \in X$ and every open $V \ni f(x)$ in $Y$, there is an open $U \ni x$ in $X$ with $f(U) \subseteq V$
4. For every $x \in X$ and every open $V \ni f(x)$, the preimage $f^{-1}(V)$ is an open neighborhood of $x$

*Proof.* (1) ↔ (2): $f^{-1}(Y \setminus C) = X \setminus f^{-1}(C)$, so preimage commutes with complement. Open $\leftrightarrow$ closed.

(1) → (3): Let $V$ be open with $f(x) \in V$. Then $U = f^{-1}(V)$ is open and $x \in U$, with $f(U) \subseteq V$.

(3) → (4): Given open $V \ni f(x)$, take $U = f^{-1}(V)$. Then $f(U) \subseteq V$, and $U$ is a neighborhood of $x$.

(4) → (1): For any open $V \subseteq Y$, check $f^{-1}(V)$ is open: for any $x \in f^{-1}(V)$, we have $f(x) \in V$, so by (4), $f^{-1}(V)$ is a neighborhood of $x$. Hence $f^{-1}(V)$ is open. $\square$

Characterization (3) is the most geometric: continuity means small neighborhoods around $f(x)$ pull back to small neighborhoods around $x$.

## Homeomorphisms

A homeomorphism is the "isomorphism" in the category **Top**: a continuous bijection with a continuous inverse.

**Definition 2.5 (Homeomorphism).** A function $f : X \to Y$ is a *homeomorphism* if:
1. $f$ is a bijection
2. $f$ is continuous
3. $f^{-1}$ is continuous

Two spaces are *homeomorphic*, written $X \cong Y$, if a homeomorphism exists between them.

**Warning:** A continuous bijection is not necessarily a homeomorphism. Example: $f : [0, 2\pi) \to S^1$, $f(t) = (\cos t, \sin t)$. This is a continuous bijection, but $f^{-1}$ is not continuous at $f(0) = (1,0)$ (arbitrarily close points on $S^1$ near $(1,0)$ come from points near $0$ and near $2\pi$, which are not close in $[0, 2\pi)$).

**Theorem 2.6.** If $f : X \to Y$ is a continuous bijection, $X$ is compact, and $Y$ is Hausdorff, then $f$ is a homeomorphism.

This theorem is extremely useful in practice: once you have a continuous bijection between a compact space and a Hausdorff space, you get a homeomorphism for free.

## What Homeomorphism Means

Homeomorphic spaces are "topologically identical" — they have the same open sets after relabeling. Every topological property is preserved by homeomorphism.

**Topological invariants** are properties preserved by homeomorphisms:
- Number of connected components
- Compactness
- Hausdorff property
- Fundamental group $\pi_1(X)$
- All homotopy groups $\pi_n(X)$
- Homology groups $H_n(X)$
- Being a manifold (and its dimension)

If two spaces have different values of any topological invariant, they can't be homeomorphic.

**Examples of homeomorphic spaces:**
- $(0, 1) \cong \mathbb{R}$ (via $t \mapsto \tan(\pi(t - 1/2))$)
- Any open interval $(a, b) \cong \mathbb{R}$
- Any bounded convex subset of $\mathbb{R}^n$ with non-empty interior $\cong$ the open ball $B^n$
- A circle $S^1 \cong$ any simple closed curve in $\mathbb{R}^2$
- The surface of a sphere $S^2 \cong$ the surface of a cube

**Examples of non-homeomorphic spaces:**
- $\mathbb{R} \not\cong \mathbb{R}^2$ (removing a point from $\mathbb{R}$ disconnects it; removing a point from $\mathbb{R}^2$ doesn't)
- $S^1 \not\cong S^2$ (different fundamental groups: $\pi_1(S^1) = \mathbb{Z}$, $\pi_1(S^2) = 0$)
- $[0, 1] \not\cong S^1$ (one has boundary points, the other doesn't)
- $\mathbb{R}^m \not\cong \mathbb{R}^n$ for $m \neq n$ (by invariance of domain or homology)

## Open and Closed Maps

Beyond continuity, we sometimes need maps that preserve open or closed sets in the forward direction.

**Definition 2.7.** A map $f : X \to Y$ is:
- *Open* if $f(U)$ is open in $Y$ for every open $U \subseteq X$
- *Closed* if $f(C)$ is closed in $Y$ for every closed $C \subseteq X$

Note: these are different from continuity, which is about *preimages*. Open/closed maps are about *images*.

**Examples:**
- Projections $\pi_1 : X \times Y \to X$ are open maps (images of open boxes are open)
- Quotient maps $q : X \to X/\sim$ are not generally open or closed
- Closed injections $f : A \hookrightarrow X$ (where $A$ is closed in $X$) are closed maps

**Theorem 2.8.** A bijective map $f : X \to Y$ is a homeomorphism iff $f$ is continuous and either open or closed.

## The Role of Continuity in HoTT

In HoTT, every function $f : A \to B$ between types is automatically "continuous" — in the sense that it respects the path structure. Specifically:
- The dependent action on paths (ap): for every $p : a = a'$, $\mathsf{ap}_f(p) : f(a) = f(a')$
- Transport: for every $p : a = a'$ and dependent $b : B(a)$, $\mathsf{transport}^B(p, b) : B(a')$

These are the HoTT analogs of "continuous" maps preserving the topological structure. In fact, in the simplicial set model, every HoTT function corresponds to a continuous map between the corresponding topological spaces.

This is why HoTT doesn't need to specify which functions are continuous — all functions defined in HoTT are automatically "continuous" (i.e., they're all morphisms in the ∞-topos). The type theory enforces continuity by construction.

**Homeomorphism in HoTT.** A homeomorphism corresponds to an *equivalence of types*: a function $f : A \to B$ with a quasi-inverse $g : B \to A$ and homotopies $f \circ g \sim \mathsf{id}_B$ and $g \circ f \sim \mathsf{id}_A$. The Univalence axiom then says: equivalent types are *equal* (as a path in the universe).

## Separation Axioms: Brief Overview

Not all topological spaces are equally well-behaved. The *separation axioms* are a hierarchy of conditions that control how well distinct points can be "separated" by open sets.

**$T_0$ (Kolmogorov):** For any $x \neq y$, there exists an open set containing one but not the other.

**$T_1$ (Fréchet):** For any $x \neq y$, there exist open sets $U \ni x$ with $y \notin U$ and $V \ni y$ with $x \notin V$.

*Equivalent:* All singletons $\{x\}$ are closed.

**$T_2$ (Hausdorff):** For any $x \neq y$, there exist *disjoint* open sets $U \ni x$ and $V \ni y$.

The Hausdorff condition is the most important separation axiom. In a Hausdorff space:
- Limits of sequences/nets are unique (can't converge to two different points)
- Compact subsets are closed
- The product of Hausdorff spaces is Hausdorff

Most spaces in "ordinary" mathematics (metric spaces, manifolds, algebraic varieties over $\mathbb{C}$) are Hausdorff. Non-Hausdorff spaces arise in algebraic geometry (Zariski topology) and domain theory.

**$T_3$ (Regular Hausdorff):** $T_1$ plus: any closed set and a point not in it can be separated by disjoint open sets.

**$T_4$ (Normal Hausdorff):** $T_1$ plus: any two disjoint closed sets can be separated by disjoint open sets.

In HoTT, the analog of Hausdorff is having a *decidable equality* type: a set $A$ has decidable equality if $\prod_{a,b:A} (a = b) + \neg(a = b)$. This is the HoTT analog of "distinct points can be separated."

## Summary

| Concept | Definition | Intuition |
|---|---|---|
| Continuous | $f^{-1}(V)$ open for all open $V$ | Nearness is preserved |
| Homeomorphism | Continuous bijection with continuous inverse | Topological isomorphism |
| Open map | $f(U)$ open for all open $U$ | Opens map to opens |
| Closed map | $f(C)$ closed for all closed $C$ | Closeds map to closeds |

Continuity and homeomorphism are the morphisms and isomorphisms of **Top**. Every topological property is a homeomorphism invariant — invariants are the tools for proving spaces are *not* homeomorphic.
