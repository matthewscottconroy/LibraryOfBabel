# 13.3 Quadratic Polynomials: $f_c(z) = z^2 + c$

The family $\{f_c(z) = z^2 + c : c \in \mathbb{C}\}$ is the simplest nontrivial family of polynomials. Every quadratic polynomial is conjugate to exactly one member of this family (via a linear change of variables). So this family is not just a convenient example — it is, up to conjugacy, *all* quadratic polynomials.

The unique critical point of $f_c$ is $z = 0$ (where $f_c'(z) = 2z = 0$). This is the only place where $f_c$ fails to be a local homeomorphism. The *critical orbit* is:
$$0 \mapsto c \mapsto c^2+c \mapsto (c^2+c)^2+c \mapsto \cdots$$

**Dichotomy:** By Theorem 13.1.3, $\mathcal{K}(f_c)$ is connected iff the critical orbit $\{f_c^n(0)\}_{n \geq 0}$ is bounded. This gives the exact characterization of the Mandelbrot set.

## 13.3.1 The Mandelbrot Set

**Definition 13.3.1.** The *Mandelbrot set* is:
$$\mathcal{M} = \{c \in \mathbb{C} : \mathcal{K}(f_c) \text{ is connected}\} = \{c \in \mathbb{C} : f_c^n(0) \not\to \infty\}.$$

Benoit Mandelbrot studied this set computationally in 1980, and it bears his name — though Douady and Hubbard developed the rigorous theory in a landmark series of papers in the 1980s.

**Theorem 13.3.2 (Basic Properties of $\mathcal{M}$).**
1. $\mathcal{M}$ is compact, connected, and full (it has no "holes" — its complement is connected).
2. $\mathcal{M}$ is symmetric under complex conjugation: $c \in \mathcal{M}$ iff $\bar{c} \in \mathcal{M}$.
3. The *main cardioid* is the set $\{c : f_c$ has an attracting fixed point$\}$; it is an open region bounded by a cardioid curve. The *period-2 bulb* is attached to the main cardioid and contains parameters where $f_c$ has an attracting period-2 orbit.
4. The boundary $\partial\mathcal{M}$ has Hausdorff dimension $2$ (Shishikura's theorem, 1998).

Shishikura's theorem is stunning: $\partial\mathcal{M}$ looks like a complicated curve, but its Hausdorff dimension is $2$ — the maximum possible for any planar set. The boundary of the Mandelbrot set is as geometrically complex as a filled region, even though it has zero area. This is a measure of the extraordinary intricacy of the set.

## The MLC Conjecture

**The MLC Conjecture:** Is $\mathcal{M}$ locally connected?

Local connectivity of $\mathcal{M}$ would mean: every point $c \in \mathcal{M}$ has arbitrarily small connected neighborhoods within $\mathcal{M}$. Equivalently (by the Carathéodory theorem), it would mean that the Riemann map $\Phi: \hat{\mathbb{C}} \setminus \mathcal{M} \to \hat{\mathbb{C}} \setminus \bar{\mathbb{D}}$ (the Böttcher coordinate for $\mathcal{M}$ in the parameter plane) extends continuously to the boundary.

A positive answer to MLC would give a complete combinatorial description of the Mandelbrot set: every parameter $c \in \partial\mathcal{M}$ would be described by its "external angle" in $[0,1)$, and the bifurcation structure of the family would be completely encoded by angle arithmetic under the doubling map.

**Theorem 13.3.3 (Yoccoz, 1990).** $\mathcal{M}$ is locally connected at all *finitely renormalizable* parameters.

What this is saying is: Yoccoz proved MLC at all parameters where the complexity of $f_c$ is "bounded" (finitely renormalizable — there are only finitely many scales at which the map "looks like a quadratic polynomial again"). The infinitely renormalizable case — parameters like the tip of the Mandelbrot set where period-doubling cascades accumulate — remains open. This is where the deepest and most active research lives.

The structure of $\mathcal{M}$ at all scales is one of the great ongoing research programs in mathematics.
