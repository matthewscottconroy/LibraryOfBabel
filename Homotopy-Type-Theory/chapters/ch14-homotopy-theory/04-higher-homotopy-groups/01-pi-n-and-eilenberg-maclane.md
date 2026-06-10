# 4.1 Higher Homotopy Groups and Eilenberg-MacLane Spaces

## Beyond the Fundamental Group

The fundamental group $\pi_1(X, x_0)$ measures 1-dimensional holes in $X$. But spaces can have more complex topology: 2-dimensional holes (like in $S^2$), 3-dimensional holes (like in $S^3$), and so on.

To capture $n$-dimensional holes, we need the *$n$th homotopy group* $\pi_n(X, x_0)$: the set of homotopy classes of maps from the $n$-sphere $S^n$ to $X$, with basepoint sent to basepoint.

**Definition 4.1 (Higher Homotopy Groups).** For $n \geq 1$ and a pointed space $(X, x_0)$:
$$\pi_n(X, x_0) = [(S^n, s_0), (X, x_0)]$$

the set of homotopy classes of continuous maps $(S^n, s_0) \to (X, x_0)$ (sending the basepoint $s_0 \in S^n$ to $x_0 \in X$), where homotopies are also required to fix the basepoint.

The group structure comes from "pinching": take two maps $f, g : S^n \to X$ and combine them by collapsing the equator of $S^n$ to a point, getting a wedge $S^n \vee S^n$, and then applying $f$ on one half and $g$ on the other.

**For $n \geq 2$: abelian groups.** By the Eckmann-Hilton argument (Chapter 12), $\pi_n(X)$ is abelian for all $n \geq 2$. For $n = 1$, $\pi_1$ can be non-abelian (as with free groups).

## Key Computations

**$\pi_n(S^n) = \mathbb{Z}$.** The generator is the identity map $\mathsf{id} : S^n \to S^n$. Maps $S^n \to S^n$ are classified up to homotopy by their *degree* (an integer measuring how many times one sphere wraps around the other). Degree $d$ and degree $d'$ maps are homotopic iff $d = d'$.

**$\pi_k(S^n) = 0$ for $k < n$.** There are no non-trivial maps from $S^k$ to $S^n$ when $k < n$: you can't wrap a lower-dimensional sphere around a higher-dimensional one non-trivially. Formally, this follows from cellular approximation and the cellular structure of spheres.

**$\pi_k(S^n)$ for $k > n$: surprising and hard.** Higher homotopy groups of spheres are the central computational challenge of algebraic topology. Key examples:
- $\pi_3(S^2) = \mathbb{Z}$ (from the Hopf fibration)
- $\pi_4(S^3) = \mathbb{Z}/2$ (found by Serre, 1951)
- $\pi_5(S^2) = \mathbb{Z}/2$
- These groups are known up to dimension ~80 via computer computation

**$\pi_1(S^1) = \mathbb{Z}$** (fundamental result, proved via covering spaces).

**$\pi_n(K(G,n)) = G$ and $\pi_k(K(G,n)) = 0$ for $k \neq n$** (defining property of Eilenberg-MacLane spaces, see below).

## The Loop Space

An alternative but equivalent way to define $\pi_n$: via *iterated loop spaces*.

**Definition 4.2 (Loop Space).** The *loop space* $\Omega(X, x_0)$ is the space of loops at $x_0$:
$$\Omega(X, x_0) = \{\gamma : [0,1] \to X \mid \gamma(0) = \gamma(1) = x_0\}$$
with the compact-open topology, and basepoint $c_{x_0}$ (the constant loop).

**Theorem 4.3.** $\pi_n(X, x_0) \cong \pi_{n-1}(\Omega(X, x_0), c_{x_0})$.

In other words: an $n$-sphere's worth of loops in $X$ is the same as an $(n-1)$-sphere's worth of loops in $\Omega X$. So:
$$\pi_n(X) = \pi_1(\Omega^{n-1} X) = \pi_0(\Omega^n X)$$

The $n$th homotopy group of $X$ is the 0th homotopy group (connected components) of the $n$-fold loop space $\Omega^n X$.

In HoTT:
- $\Omega A = (a_0 = a_0)$ (the loop type — loops at the basepoint)
- $\Omega^n A$ = the $n$-fold iterated loop type
- $\pi_n(A) = \|\Omega^n A\|_0$ (the 0-truncation of the $n$-fold loop space)

## Eilenberg-MacLane Spaces

Eilenberg-MacLane spaces are the "atoms" of homotopy theory — spaces with homotopy concentrated in a single dimension.

**Definition 4.4 (Eilenberg-MacLane Space).** A space $K(G, n)$ is an *Eilenberg-MacLane space* of type $(G, n)$ (for a group $G$ and $n \geq 1$) if:
$$\pi_k(K(G, n)) = \begin{cases} G & k = n \\ 0 & k \neq n \end{cases}$$

For $n \geq 2$, $G$ must be abelian. For $n = 1$, $G$ can be any group.

**Existence and uniqueness:** Such spaces exist for any $G, n$, and they are unique up to homotopy equivalence. The proof: build $K(G,n)$ by starting with a CW complex with $\pi_n = G$ and then systematically killing all other homotopy groups.

**Key examples:**
- $K(\mathbb{Z}, 1) = S^1$: the circle has $\pi_1 = \mathbb{Z}$ and higher $\pi_k = 0$
- $K(\mathbb{Z}/2, 1) = \mathbb{RP}^\infty$: infinite real projective space
- $K(\mathbb{Z}, 2) = \mathbb{CP}^\infty$: infinite complex projective space
- $K(\mathbb{Z}/n, 1) = L^\infty(\mathbb{Z}/n)$: infinite lens space

## The Postnikov Tower

Every space $X$ can be "assembled" from Eilenberg-MacLane spaces via the *Postnikov tower*.

**Definition 4.5 (Postnikov Tower).** The *Postnikov tower* of $X$ is a sequence of fibrations:
$$\cdots \to P_n X \to P_{n-1} X \to \cdots \to P_1 X \to P_0 X = \pi_0(X)$$

where $P_n X$ (the $n$th Postnikov section) is:
- The "truncation" of $X$: kill all homotopy groups above dimension $n$
- Equivalently: the unique (up to homotopy) space with $\pi_k(P_n X) = \pi_k(X)$ for $k \leq n$ and $\pi_k(P_n X) = 0$ for $k > n$

There is a map $X \to P_n X$ (the $n$th Postnikov section map), inducing isomorphisms on $\pi_k$ for $k \leq n$.

**The fibers.** Each successive fiber in the Postnikov tower:
$$\mathsf{fib}(P_n X \to P_{n-1} X) \simeq K(\pi_n(X), n)$$

So the Postnikov tower assembles $X$ from Eilenberg-MacLane spaces $K(\pi_n(X), n)$ via successive fibrations. The extension data (how each layer is attached) is called the *k-invariants*.

**Example.** The torus $T^2$:
- $P_0(T^2) = \{*\}$ (one connected component)
- $P_1(T^2) = K(\mathbb{Z}^2, 1) = T^2$ itself (since $\pi_k(T^2) = 0$ for $k \geq 2$)

So the Postnikov tower of the torus terminates at $P_1 = T^2$. The torus is already a $K(\mathbb{Z}^2, 1)$!

**In HoTT:** The Postnikov tower corresponds to the *truncation* hierarchy. The $n$-truncation $\tau_{\leq n}(A)$ is the $n$th Postnikov section. A type is a Postnikov tower of its homotopy groups, assembled by $k$-invariants.

## The Freudenthal Suspension Theorem

One of the most important theorems about higher homotopy groups:

**Theorem 4.6 (Freudenthal Suspension Theorem).** The suspension map $\sigma : \pi_k(S^n) \to \pi_{k+1}(S^{n+1})$ is an isomorphism for $k < 2n - 1$ and surjective for $k = 2n - 1$.

**Consequence: Stable homotopy groups.** For $k < 2n - 1$, the homotopy group $\pi_k(S^n)$ depends only on the difference $k - n$. These *stable homotopy groups* $\pi_k^s = \pi_{k+n}(S^n)$ (for large $n$) form the *stable homotopy groups of spheres* — a fundamental object in algebraic topology.

**In HoTT:** The Freudenthal suspension theorem has been proved in HoTT (by Lumsdaine and others). It states: if $A$ is $n$-connected, the natural map $A \to \Omega \Sigma A$ is $2n$-connected. This is a theorem of HoTT, proved synthetically.

## Homotopy Groups in HoTT

In HoTT, homotopy groups are defined internally:

**$\pi_n(A, a_0) := \|\Omega^n A\|_0$** where $\Omega^n A = \underbrace{(\ldots ((a_0 = a_0) = \mathsf{refl}_{a_0}) \ldots)}_n$ is the $n$-fold iterated loop type.

Note: This is the *set-truncation* of the $n$-fold loop space — we take homotopy classes (i.e., equivalence classes under homotopy) to get a set.

**For $n \geq 2$:** $\pi_n(A, a_0)$ is abelian (by Eckmann-Hilton). The HoTT proof of Eckmann-Hilton is purely type-theoretic (Chapter 12).

**The circle:** $\pi_1(S^1) = \mathbb{Z}$ in HoTT. This has a full synthetic proof (Licata-Shulman).

**Higher spheres:** $\pi_n(S^n) = \mathbb{Z}$ is known in HoTT but the proof for general $n$ requires more machinery (the Hurewicz theorem, or the Freudenthal theorem).

## Summary

| Space | $\pi_1$ | $\pi_2$ | $\pi_3$ | Notes |
|---|---|---|---|---|
| $S^1$ | $\mathbb{Z}$ | 0 | 0 | $K(\mathbb{Z},1)$ |
| $S^2$ | 0 | $\mathbb{Z}$ | $\mathbb{Z}$ | Hopf fibration gives $\pi_3$ |
| $S^3$ | 0 | 0 | $\mathbb{Z}$ | Quaternions |
| $T^2$ | $\mathbb{Z}^2$ | 0 | 0 | $K(\mathbb{Z}^2, 1)$ |
| $\mathbb{CP}^\infty$ | 0 | $\mathbb{Z}$ | 0 | $K(\mathbb{Z},2)$ |

Higher homotopy groups are the backbone of algebraic topology. The stable homotopy groups of spheres are still not completely understood — this is an active research frontier. In HoTT, computing homotopy groups of HITs is one of the central research programs.
