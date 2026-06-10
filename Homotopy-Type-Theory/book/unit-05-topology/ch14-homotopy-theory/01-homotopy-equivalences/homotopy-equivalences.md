# Homotopy Equivalences

## Homotopy Between Maps

Two continuous maps $f, g : X \to Y$ between topological spaces are *homotopic* if one can be continuously deformed into the other. The deformation is encoded by a one-parameter family:

**Definition.** A *homotopy* from $f$ to $g$ is a continuous map $H : X \times [0,1] \to Y$ satisfying $H(x, 0) = f(x)$ and $H(x, 1) = g(x)$ for all $x \in X$. We write $f \simeq g$ and say $f$ and $g$ are *homotopic*.

Think of $t$ as time. The map $H_t : X \to Y$ defined by $H_t(x) = H(x, t)$ is a "snapshot" at time $t$: at $t = 0$ we have $H_0 = f$; at $t = 1$ we have $H_1 = g$. As $t$ moves from $0$ to $1$, $H_t$ continuously interpolates between $f$ and $g$.

Homotopy is an equivalence relation on the set of continuous maps from $X$ to $Y$:
- *Reflexivity:* $f \simeq f$ via the constant homotopy $H(x,t) = f(x)$.
- *Symmetry:* If $H$ is a homotopy from $f$ to $g$, then $H'(x,t) = H(x, 1-t)$ is a homotopy from $g$ to $f$.
- *Transitivity:* If $H : f \simeq g$ and $K : g \simeq h$, concatenate: $L(x,t) = H(x, 2t)$ for $t \leq 1/2$ and $L(x,t) = K(x, 2t-1)$ for $t \geq 1/2$. Continuity of $L$ follows from the pasting lemma (the two pieces agree at $t = 1/2$ since $H(x,1) = g(x) = K(x,0)$).

The equivalence class $[f]$ of $f$ under homotopy is the *homotopy class* of $f$. The set of homotopy classes of maps from $X$ to $Y$ is denoted $[X, Y]$.

## Canonical Examples of Homotopies

**Straight-line homotopy.** For any two continuous maps $f, g : X \to \mathbb{R}^n$ into Euclidean space (or any convex subset thereof), the straight-line homotopy $H(x,t) = (1-t)f(x) + tg(x)$ is a well-defined continuous map: it stays in $\mathbb{R}^n$ (since $\mathbb{R}^n$ is convex), and it satisfies $H(x,0) = f(x)$ and $H(x,1) = g(x)$. This shows that any two maps into a convex set are homotopic, so $[X, \mathbb{R}^n]$ is always a single homotopy class (the maps form one "shape").

**Null-homotopic maps.** A map $f : X \to Y$ is *null-homotopic* if $f \simeq c_{y_0}$ for some constant map $c_{y_0}(x) = y_0$. Null-homotopy means: $f$ can be continuously contracted to a single point. Every map into a contractible space is null-homotopic. The identity map on $S^n$ is not null-homotopic for $n \geq 1$ (this is a non-trivial theorem, proved via homotopy groups).

## Homotopy Equivalence of Spaces

**Definition.** A continuous map $f : X \to Y$ is a *homotopy equivalence* if there exists a continuous map $g : Y \to X$ (a *homotopy inverse* of $f$) with $g \circ f \simeq \mathsf{id}_X$ and $f \circ g \simeq \mathsf{id}_Y$. When such an equivalence exists, we write $X \simeq Y$ and say $X$ and $Y$ are *homotopy equivalent* or have the *same homotopy type*.

Note the weakening from homeomorphism: a homeomorphism requires $g \circ f = \mathsf{id}_X$ and $f \circ g = \mathsf{id}_Y$ (on the nose, not just up to homotopy). Homotopy equivalence allows the identity conditions to hold up to continuous deformation. Every homeomorphism is a homotopy equivalence, but many homotopy equivalences are not homeomorphisms.

Homotopy equivalence is an equivalence relation on the class of topological spaces: the homotopy type of $X$ is its equivalence class. Homotopy invariants — the fundamental group, higher homotopy groups, homology groups — are invariants of the homotopy type, not just the homeomorphism type.

## Contractible Spaces

A space $X$ is *contractible* if $X \simeq \{*\}$ — it is homotopy equivalent to a single point. Equivalently: there exists a point $x_0 \in X$ and a homotopy $H : X \times [0,1] \to X$ with $H(x,0) = x$ (starts as identity) and $H(x,1) = x_0$ (ends at the constant map to $x_0$). Such a homotopy is a *contraction* of $X$ to $x_0$.

Examples of contractible spaces:
- $\mathbb{R}^n$: contract via $H(x,t) = (1-t)x$.
- Any convex subset of $\mathbb{R}^n$: same construction.
- The cone $CX = (X \times [0,1]) / (X \times \{1\})$: contract to the cone point via $H([(x,s)], t) = [(x, s + t(1-s))]$.
- The infinite-dimensional sphere $S^\infty$: contractible by a subtle argument using the shift map.

Examples of non-contractible spaces:
- $S^n$ for $n \geq 1$: $\pi_n(S^n) = \mathbb{Z} \neq 0$.
- Any space with non-trivial fundamental group.
- Any space with non-trivial homology.

Contractible spaces are the "trivial" spaces in homotopy theory — the analogue of the zero group in algebra. Every homotopy-theoretic invariant of a contractible space is trivial.

## Deformation Retracts

**Definition.** A *deformation retract* of $X$ onto a subspace $A \subseteq X$ is a homotopy $H : X \times [0,1] \to X$ satisfying:
1. $H(x, 0) = x$ for all $x \in X$ (starts as identity).
2. $H(x, 1) \in A$ for all $x \in X$ (ends in $A$).
3. $H(a, t) = a$ for all $a \in A$ and $t \in [0,1]$ ($A$ is fixed throughout).

If $A$ is a deformation retract of $X$, then the inclusion $A \hookrightarrow X$ is a homotopy equivalence (with homotopy inverse $r : X \to A$ defined by $r(x) = H(x,1)$).

**Key examples:**
- $S^{n-1}$ is a deformation retract of $\mathbb{R}^n \setminus \{0\}$ (punctured Euclidean space): the deformation is radial projection $H(x, t) = (1-t)x + t \cdot x/|x|$. So $\mathbb{R}^n \setminus \{0\} \simeq S^{n-1}$.
- $S^1$ is a deformation retract of the Möbius band (retract to the central circle).
- Any spanning tree $T$ in a connected graph $\Gamma$ is contractible, and $\Gamma$ deformation retracts to $\Gamma/T$ — a wedge of circles, one for each edge not in $T$.

## Homotopy Invariants

A property $P$ of topological spaces is a *homotopy invariant* if $X \simeq Y$ implies $P(X) \iff P(Y)$.

Homotopy invariants include:
- Path-connectedness (and $\pi_0$ — the set of path-components).
- The fundamental group $\pi_1(X, x_0)$.
- All higher homotopy groups $\pi_n(X, x_0)$.
- All homology groups $H_n(X; G)$.
- All cohomology groups $H^n(X; G)$.
- Being contractible.
- Being a $K(G, n)$ space (Eilenberg-MacLane space).

Properties that are NOT homotopy invariants:
- Being a manifold of dimension $n$ (since $\mathbb{R}$ and $\mathbb{R}^2$ are not homotopy equivalent but you can have homotopy equivalent manifolds of different dimensions if one is contractible).
- Being Hausdorff.
- Being compact (contractible non-compact spaces exist).
- Cardinality.

## The Homotopy Category

The *homotopy category* $\mathsf{Ho}(\mathbf{Top})$ has topological spaces as objects and homotopy classes of continuous maps as morphisms. Two spaces are isomorphic in $\mathsf{Ho}(\mathbf{Top})$ if and only if they are homotopy equivalent.

Warning: $\mathsf{Ho}(\mathbf{Top})$ is poorly behaved as a category. Products in **Top** (the usual topological product $X \times Y$) are not products in $\mathsf{Ho}(\mathbf{Top})$. The homotopy category loses important information — it remembers only the homotopy classes of maps, not the maps themselves or the homotopies between them.

This deficiency motivates richer structures:
- *Model categories* (Quillen): categories with classes of cofibrations, fibrations, and weak equivalences, where the homotopy theory is properly formalized.
- *(∞,1)-categories*: categories where morphisms between morphisms between... morphisms are systematically included. The (∞,1)-category of spaces includes not just homotopy classes of maps but the entire space of maps between spaces, up to all higher homotopies.

HoTT is, in a precise sense, the internal language of (∞,1)-toposes — and the homotopy types of spaces are the objects of the fundamental (∞,1)-topos.

## The HoTT Translation

The translation of homotopy theory into HoTT is the following dictionary:

| Classical | HoTT |
|---|---|
| Continuous map $f : X \to Y$ | Function $f : A \to B$ |
| Homotopy $H : f \simeq g$ | Term of $\prod_{a:A} f(a) = g(a)$ |
| Homotopy equivalence $X \simeq Y$ | Equivalence $A \simeq B$ |
| Contractible space | Contractible type: $\mathsf{isContr}(A) := \sum_{a:A} \prod_{b:A} a = b$ |
| Deformation retract of $X$ onto $A$ | Section of the fibration $A \to X$ |
| Homotopy class $[f]$ | Propositional truncation $\|A \to B\|_{-1}$ |

The deep insight: in HoTT, homotopies are *first-class mathematical objects* — they are terms of types. A homotopy $f \sim g$ is not merely a statement about the existence of a deformation; it is an explicit term, a proof, that can be manipulated, composed, and reasoned about. This is the type-theoretic version of the ∞-groupoid structure of homotopy types.
