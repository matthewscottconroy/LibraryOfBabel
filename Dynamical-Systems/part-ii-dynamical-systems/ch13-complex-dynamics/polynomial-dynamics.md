# 13.2 Polynomial Dynamics

Polynomials are special among rational maps because $\infty$ is a fixed point — in fact, a superattracting one of local degree $d$ (if $p$ has degree $d$, then $p(z) \sim z^d$ near $\infty$). This means the basin of attraction of $\infty$ is well-defined and has a beautifully rigid structure: it is conformally equivalent to the exterior of the unit disk. The theory of this conformal equivalence — Böttcher coordinates — is the main tool for analyzing polynomial dynamics.

## 13.2.1 Filled Julia Sets

For a polynomial, orbits either escape to $\infty$ or remain bounded forever. The bounded ones form the filled Julia set.

**Definition 13.2.1.** The *filled Julia set* is:
$$\mathcal{K}(p) = \{z \in \mathbb{C} : p^n(z) \not\to \infty\}.$$
The Julia set $\mathcal{J}(p) = \partial \mathcal{K}(p)$ is the boundary of the filled Julia set.

**Theorem 13.2.2.** $\mathcal{K}(p)$ is compact and fully invariant ($p^{-1}(\mathcal{K}(p)) = \mathcal{K}(p)$). Moreover, $\mathcal{J}(p) = \mathcal{K}(p)$ iff $\mathcal{K}(p)$ has empty interior.

The interior of $\mathcal{K}(p)$ — when it is nonempty — consists of Fatou components (basins of attracting cycles, Siegel disks). When all critical orbits escape (Cantor Julia set case), $\mathcal{K}(p)$ has no interior.

## 13.2.2 Böttcher Coordinates

The conformal geometry of the basin $\mathcal{A}(\infty) = \hat{\mathbb{C}} \setminus \mathcal{K}(p)$ is controlled by Böttcher's theorem, which gives a canonical coordinate system near $\infty$ that conjugates $p$ to the power map $w \mapsto w^d$.

**Theorem 13.2.3 (Böttcher).** Near $\infty$, there is a unique conformal isomorphism (Böttcher coordinate):
$$\phi: \mathcal{A}(\infty) \xrightarrow{\sim} \hat{\mathbb{C}} \setminus \bar{\mathbb{D}}, \quad \phi(p(z)) = \phi(z)^d.$$

The *Green's function* $G_p(z) = \log|\phi(z)| = \lim_{n \to \infty} d^{-n} \log|p^n(z)|$ measures the "height" of $z$ above the filled Julia set. It satisfies $G_p \geq 0$, $G_p = 0$ on $\mathcal{K}(p)$, and $G_p(p(z)) = d \cdot G_p(z)$.

What this is saying is: the Green's function is a potential-theoretic measure of how quickly orbits escape. Points with large $G_p(z)$ escape quickly; points with $G_p(z) = 0$ don't escape at all. The filled Julia set is exactly the zero set of $G_p$.

**External Rays:** The Böttcher coordinate $\phi$ transforms the polar coordinate structure of the exterior disk $\{|w| > 1\}$ into the basin $\mathcal{A}(\infty)$. The preimages under $\phi$ of radial lines $\{re^{2\pi i\theta} : r > 1\}$ are *external rays* of angle $\theta$. These rays are "gradient lines" of the Green's function and provide a canonical coordinate system in the basin.

When $\mathcal{J}(p)$ is locally connected, the external rays *land*: the limit of the ray as $r \to 1^+$ exists as a point in $\mathcal{J}(p)$. This gives a surjective map from angles $\theta \in [0,1)$ to points in $\mathcal{J}(p)$. Under this map, the dynamics of $p$ on $\mathcal{J}(p)$ corresponds to the doubling map $\theta \mapsto d\theta \pmod 1$ on angles. This is the *combinatorial model* of the Julia set — a symbolic description of the dynamics encoded in the angles of landing rays.

For locally connected Julia sets, the external ray picture gives a complete combinatorial description: the Julia set is a quotient of the circle $\mathbb{R}/\mathbb{Z}$ under the identifications $\theta \sim \theta'$ iff the external rays of angle $\theta$ and $\theta'$ land at the same point. The dynamics is the doubling map, and the whole structure is determined by combinatorics.

This is the key to the combinatorial description of the Mandelbrot set, which we turn to next.
