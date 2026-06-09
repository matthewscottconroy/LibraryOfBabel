# 3.3 Homotopy and the Fundamental Group

Two maps between topological spaces can be "continuously deformed" into each other — this is the notion of homotopy. The fundamental group measures how many essentially different loops a space has, and it's one of the primary invariants for distinguishing spaces up to homotopy equivalence.

## 3.3.1 Homotopy of Maps

**Definition 3.3.1.** Two continuous maps $f, g: X \to Y$ are *homotopic* (written $f \simeq g$) if there exists a continuous $H: X \times [0,1] \to Y$ with $H(x, 0) = f(x)$ and $H(x, 1) = g(x)$ for all $x$. The map $H$ is called a *homotopy*.

Think of $H$ as a "movie" — at time $t = 0$ you see the map $f$, at time $t = 1$ you see the map $g$, and for intermediate $t$ you see a continuous interpolation. The homotopy is a continuous deformation of one map into the other.

If $f, g: (X, x_0) \to (Y, y_0)$ are maps of pointed spaces and $H(x_0, t) = y_0$ for all $t$, the homotopy is *based* — the base point is fixed throughout the deformation.

**Definition 3.3.2.** A continuous map $f: X \to Y$ is a *homotopy equivalence* if there exists $g: Y \to X$ with $g \circ f \simeq \text{id}_X$ and $f \circ g \simeq \text{id}_Y$. Spaces related by a homotopy equivalence have the same *homotopy type*.

Homotopy equivalence is weaker than homeomorphism: spaces that look different geometrically can have the same homotopy type. A disk is homotopy equivalent to a point (contract everything to the center). The punctured plane $\mathbb{R}^2 \setminus \{0\}$ is homotopy equivalent to the circle $S^1$ (retract radially). These spaces are not homeomorphic but they have the same "topological complexity."

## 3.3.2 The Fundamental Group

The fundamental group captures the most basic topological complexity of a space: how many essentially different closed loops are there?

**Definition 3.3.3.** A *loop* based at $x_0 \in X$ is a continuous $\gamma: [0,1] \to X$ with $\gamma(0) = \gamma(1) = x_0$. The *fundamental group* $\pi_1(X, x_0)$ is the set of homotopy classes of loops based at $x_0$, with group operation given by concatenation:
$$(\gamma * \delta)(t) = \begin{cases} \gamma(2t) & t \leq 1/2 \\ \delta(2t - 1) & t \geq 1/2. \end{cases}$$

Concatenation means: first traverse $\gamma$, then traverse $\delta$. The group operation is well-defined on homotopy classes (concatenating homotopic loops gives homotopic results). The identity element is the constant loop at $x_0$, and the inverse of $[\gamma]$ is $[\bar{\gamma}]$ where $\bar{\gamma}(t) = \gamma(1-t)$ (reverse the loop).

The fundamental group is the first in a family of algebraic invariants. Let's see what it computes in familiar spaces:

**Examples 3.3.4.**
- $\pi_1(\mathbb{R}^n) = \{e\}$ (trivial) — $\mathbb{R}^n$ is simply connected. Every loop can be contracted to a point.
- $\pi_1(S^1) \cong \mathbb{Z}$ — loops around the circle are classified by winding number. A loop that goes around twice is not homotopic to one that goes around once.
- $\pi_1(\mathbb{T}^2) \cong \mathbb{Z}^2$ for the 2-torus — two independent loops (around each circle factor).
- $\pi_1(\Sigma_g) \cong \langle a_1, b_1, \ldots, a_g, b_g : [a_1,b_1]\cdots[a_g,b_g] = 1\rangle$ for a genus-$g$ surface.

**Application in Dynamics.** The fundamental group constrains dynamical behaviors on a given space. On $S^1$, the Poincaré rotation number is a homomorphism from dynamics to $\mathbb{R}/\mathbb{Z}$, and its rationality or irrationality determines whether orbits are periodic or dense. On surfaces, the Lefschetz Fixed Point Theorem (Section 3.7) uses the Euler characteristic, which is computable from the fundamental group via the Euler-Poincaré formula.

## 3.3.3 Higher Homotopy Groups

The pattern of the fundamental group generalizes:

**Definition 3.3.5.** The *$n$-th homotopy group* $\pi_n(X, x_0)$ for $n \geq 1$ consists of homotopy classes of maps $(S^n, *) \to (X, x_0)$. For $n \geq 2$, $\pi_n$ is abelian.

The higher homotopy groups measure whether maps from spheres can be contracted. $\pi_2(X)$ detects "2-dimensional holes" (surfaces that can't be collapsed). Higher homotopy groups become increasingly subtle — the homotopy groups of spheres are notoriously difficult to compute.

**Remark 3.3.6 (Connection to HoTT).** From the perspective of Homotopy Type Theory, the homotopy groups $\pi_n(X)$ are the *truncations* of the $\infty$-groupoid structure of the space $X$. Each element of $\pi_n(X, x_0)$ is a homotopy class of maps, and the composition law is a higher-dimensional generalization of path concatenation. The Hopf fibration $S^3 \to S^2$ — the generator of $\pi_3(S^2) \cong \mathbb{Z}$ — is one of the basic computations that motivates the machinery of HoTT.
