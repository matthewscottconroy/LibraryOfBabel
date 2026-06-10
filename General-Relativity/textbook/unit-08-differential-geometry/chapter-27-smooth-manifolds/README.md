# Chapter 27: Smooth Manifolds

---

## Chapter Introduction

A manifold is the mathematical stage on which physics is performed. General relativity's spacetime is a 4-dimensional Lorentzian manifold. The 3-sphere $S^3$ and hyperbolic space $H^3$ appear in cosmology. Configuration spaces of mechanical systems are manifolds. Phase spaces are manifolds. Gauge theory lives on principal fiber bundles over manifolds.

The defining property of a manifold is **local Euclidean-ness**: every point has a neighborhood that looks like $\mathbb{R}^n$. But the global structure can be arbitrarily complex — a torus, a sphere, or a non-compact open set — as long as the patchwork of Euclidean neighborhoods is consistently assembled.

The transition from "a space that locally looks flat" to "a space that has intrinsic curvature" is the central step in differential geometry. It requires: coordinate charts, smooth transition maps, tangent vectors defined intrinsically (not as vectors in an ambient space), differential forms, and finally the connection and curvature tensor.

This chapter builds the foundations: what a smooth manifold is, how coordinates work, and how the functions and maps on manifolds are defined.

---

## Topological Manifolds

A **topological manifold** of dimension $n$ is a topological space $M$ such that:
1. $M$ is Hausdorff: distinct points have disjoint open neighborhoods
2. $M$ is second-countable: the topology has a countable basis
3. $M$ is locally Euclidean: every point $p\in M$ has an open neighborhood $U$ homeomorphic to an open subset of $\mathbb{R}^n$

The homeomorphism $\varphi: U\to\mathbb{R}^n$ is called a **coordinate chart** (or local coordinate system). The pair $(U, \varphi)$ is a chart. A collection of charts $\{(U_\alpha, \varphi_\alpha)\}$ that covers $M$ (every point is in at least one $U_\alpha$) is an **atlas**.

The coordinate functions $x^i = \pi^i\circ\varphi$ (where $\pi^i$ is projection onto the $i$-th component of $\mathbb{R}^n$) give $n$ real-valued functions on $U$ that label each point uniquely in that neighborhood. These are the **local coordinates**.

---

## Smooth Structure and Smooth Manifolds

A topological manifold can have many inequivalent smooth structures — this is a subtlety that only appears in dimension $\geq 4$ (Donaldson's exotic $\mathbb{R}^4$). For physics, we always work with smooth manifolds.

**Smooth atlas**: An atlas $\{(U_\alpha, \varphi_\alpha)\}$ is **smooth** (or $C^\infty$) if all transition maps are smooth:
$$\varphi_\beta\circ\varphi_\alpha^{-1}: \varphi_\alpha(U_\alpha\cap U_\beta)\to\varphi_\beta(U_\alpha\cap U_\beta)$$

is a $C^\infty$ diffeomorphism of open subsets of $\mathbb{R}^n$, for all pairs $\alpha$, $\beta$ with $U_\alpha\cap U_\beta\neq\emptyset$.

A **smooth manifold** is a topological manifold equipped with a maximal smooth atlas (a smooth structure).

**Examples**:
- $\mathbb{R}^n$: single chart $\varphi = \text{id}$
- $S^n$ (unit $n$-sphere): atlas with two charts (stereographic projections from north and south poles)
- $T^n = S^1\times\cdots\times S^1$ ($n$-torus): charts from angular coordinates with periodic identifications
- $\mathbb{R}P^n$ (real projective space): $n+1$ charts
- Lie groups $GL(n,\mathbb{R})$, $O(n)$, $SU(n)$: smooth manifolds with group structure

---

## Smooth Maps and Diffeomorphisms

A map $F: M\to N$ between smooth manifolds is **smooth** if in any coordinate charts $(U, \varphi)$ around $p\in M$ and $(V, \psi)$ around $F(p)\in N$, the coordinate representation:
$$\psi\circ F\circ\varphi^{-1}: \varphi(U\cap F^{-1}(V))\to\psi(V)$$

is a $C^\infty$ map between Euclidean spaces.

A **diffeomorphism** is a smooth map with a smooth inverse. Two smooth manifolds are **diffeomorphic** (equivalent as smooth manifolds) if there exists a diffeomorphism between them.

A **smooth curve** is a smooth map $\gamma: I\to M$ where $I\subset\mathbb{R}$ is an open interval.

---

## Smooth Functions on Manifolds

A **smooth function** $f: M\to\mathbb{R}$ is a smooth map to $\mathbb{R}$ (viewed as a 1-dimensional manifold). In local coordinates $(x^1,\ldots,x^n)$: $f$ is smooth iff $f\circ\varphi^{-1}: \mathbb{R}^n\to\mathbb{R}$ is $C^\infty$.

The set of smooth functions on $M$ is denoted $C^\infty(M)$. It is a commutative ring and an infinite-dimensional vector space over $\mathbb{R}$.

**Pullback**: If $F: M\to N$ is smooth and $f\in C^\infty(N)$, then $F^*f = f\circ F\in C^\infty(M)$ is the **pullback** of $f$.

---

## The Tangent Space

The tangent space $T_p M$ at a point $p\in M$ is the space of "directions" at $p$. But how do we define directions intrinsically, without embedding $M$ in $\mathbb{R}^N$?

**Definition via derivations**: A **tangent vector** at $p$ is a derivation of $C^\infty(M)$ at $p$ — a linear map $v: C^\infty(M)\to\mathbb{R}$ satisfying the Leibniz rule:
$$v(fg) = f(p)v(g) + g(p)v(f)$$

This is a purely algebraic definition, coordinate-independent.

In local coordinates $(x^1,\ldots,x^n)$, the partial derivatives $\partial/\partial x^i\big|_p$ are derivations, and they form a basis for $T_p M$:
$$v = v^i\frac{\partial}{\partial x^i}\bigg|_p$$

where $v^i = v(x^i)$ are the components in this basis.

**Dimension of $T_p M$**: $\dim T_p M = \dim M = n$. Each coordinate chart gives an isomorphism $T_p M \cong \mathbb{R}^n$.

**Alternative definition via curves**: A tangent vector at $p$ is an equivalence class of smooth curves $\gamma: (-\varepsilon,\varepsilon)\to M$ with $\gamma(0) = p$, where two curves are equivalent if they have the same velocity $d/dt$ in every coordinate chart. The velocity of $\gamma$ at $t = 0$ in coordinates is $v^i = dx^i(\gamma(t))/dt\big|_{t=0}$.

---

## The Tangent Bundle

The **tangent bundle** $TM$ is the disjoint union $\coprod_{p\in M}T_p M$ with the natural smooth structure making the projection $\pi: TM\to M$ (sending $(p, v)\in T_p M$ to $p$) smooth.

In local coordinates $(x^i)$ on $U\subset M$, the tangent bundle restricted to $U$ is $TU \cong U\times\mathbb{R}^n$ with coordinates $(x^i, v^j)$ where $v = v^j\partial/\partial x^j$.

A **vector field** is a smooth section $X: M\to TM$, i.e., a smooth assignment of a tangent vector $X_p\in T_p M$ to each point $p$. In local coordinates: $X = X^i(x)\partial/\partial x^i$.

Vector fields form a Lie algebra under the **Lie bracket** (commutator):
$$[X, Y]^i = X^j\partial_j Y^i - Y^j\partial_j X^i$$

The Lie bracket measures the non-commutativity of vector field flows and is intrinsic (coordinate-independent).

---

## Submanifolds and Embedded Surfaces

A **submanifold** $S\subset M$ is a subset with a smooth manifold structure making the inclusion map $\iota: S\hookrightarrow M$ smooth and everywhere injective on tangent spaces.

**Regular level sets**: If $f: M\to\mathbb{R}^k$ is smooth and $c\in\mathbb{R}^k$ is a regular value (the Jacobian of $f$ has rank $k$ at every point of $f^{-1}(c)$), then $f^{-1}(c)$ is an embedded submanifold of codimension $k$.

**Examples**:
- $S^{n-1} = f^{-1}(1)$ where $f(x) = |x|^2$ on $\mathbb{R}^n$ (1 is a regular value)
- The horizon $r = r_s$ in Schwarzschild is a null hypersurface (codimension-1 submanifold), but its null character requires care

---

## Orientability and Integration

A smooth manifold $M$ is **orientable** if it has a nowhere-vanishing $n$-form (a volume form). Equivalently: there exists an atlas where all transition maps have positive Jacobian determinant.

**Examples**: $\mathbb{R}^n$, $S^n$, $T^n$ are orientable. The Möbius band and $\mathbb{R}P^{2n}$ are not orientable. All Lorentzian manifolds in GR are assumed orientable (with time-orientability also assumed).

**Integration**: On an oriented $n$-manifold, a compactly supported $n$-form $\omega$ can be integrated:
$$\int_M\omega = \sum_\alpha\int_{\mathbb{R}^n}(\varphi_\alpha^{-1})^*(\rho_\alpha\omega)$$

using a partition of unity $\{\rho_\alpha\}$ subordinate to the atlas. This gives a well-defined real number.

---

## Exercises

**27.1.** *Stereographic projection and $S^2$.*

The 2-sphere $S^2 = \{x\in\mathbb{R}^3: |x| = 1\}$ is covered by two stereographic projection charts:
$$\varphi_N: S^2\setminus\{N\}\to\mathbb{R}^2, \quad \varphi_N(x^1,x^2,x^3) = \left(\frac{x^1}{1-x^3}, \frac{x^2}{1-x^3}\right)$$

(projection from the north pole $N = (0,0,1)$), and $\varphi_S$ (projection from the south pole).

(a) Find the inverse map $\varphi_N^{-1}(u,v)$.

(b) Compute the transition map $\varphi_S\circ\varphi_N^{-1}$ on $\mathbb{R}^2\setminus\{0\}$. Show it is a smooth diffeomorphism.

(c) Verify that $S^2$ with this atlas is a smooth 2-manifold.

---

**27.2.** *Tangent vectors as derivations.*

On $M = \mathbb{R}^n$ with global coordinates $(x^1,\ldots,x^n)$:

(a) Show that $\partial/\partial x^i\big|_p$ is a derivation at $p$.

(b) Show that any derivation $v$ at $p$ is a linear combination $v = v^i\partial/\partial x^i\big|_p$ where $v^i = v(x^i)$.

(c) For $M = S^1$ with coordinate $\theta\in(0, 2\pi)$: what is the tangent space $T_\theta S^1$? What is its dimension?

---

**27.3.** *The Lie bracket.*

Let $X = x^2\partial/\partial x^1$ and $Y = x^1\partial/\partial x^2$ be vector fields on $\mathbb{R}^2$.

(a) Compute $[X, Y]$ using $[X,Y]^i = X^j\partial_j Y^i - Y^j\partial_j X^i$.

(b) Verify that $[X,Y]f = X(Yf) - Y(Xf)$ for $f = x^1 x^2$.

(c) The flow of $X$ starting at $p_0 = (1,0)$: solve $d\gamma/dt = X(\gamma(t))$ with $\gamma(0) = p_0$. What curve does $\gamma$ trace?

---

**Thought Experiment T27.1.** *Why smooth manifolds?*

Physics uses smooth manifolds rather than just topological manifolds because it needs calculus — differentiation, differential equations, integration. The smooth structure (the atlas of coordinate charts and the smooth transition maps) is exactly the minimum structure needed to define these operations intrinsically.

But is the smooth structure physical? Spacetime could, in principle, be a discrete combinatorial structure (like a causal set) that approximates a smooth manifold in some limit. What experimental tests distinguish a smooth continuum from a fine discrete lattice? At what scale would discreteness become observable?

**Thought Experiment T27.2.** *The classification of manifolds.*

In low dimensions: every 1-manifold is either $S^1$ or $\mathbb{R}$. Every 2-manifold is classified by its genus $g$ (number of "handles") and orientability. In 3D: the classification is complete by Thurston's geometrization theorem (proved by Perelman 2003 using Ricci flow). In 4D: the classification is **undecidable** — there is no algorithm that can determine whether two 4-manifolds are diffeomorphic.

What does it mean for a mathematical question to be undecidable? Does the undecidability of 4-manifold classification have implications for physics — for instance, for the problem of classifying all solutions to Einstein's equations?
