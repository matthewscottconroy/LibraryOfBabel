# 20.3 Dual Connections and Dually Flat Geometry

In Riemannian geometry, there is a canonical connection associated to any metric — the Levi-Civita connection, which is both torsion-free and metric-compatible. Information geometry replaces this single canonical connection with a *pair* of dual connections, and the interplay between them is the source of the theory's power.

**Definition 20.3.1 (Statistical Manifold with Dual Connections).** A *statistical manifold* $(\mathcal{S}, g, \nabla, \nabla^*)$ has a Riemannian metric $g$ and two torsion-free affine connections $\nabla$ and $\nabla^*$ (the *dual pair*) satisfying:
$$Z\langle g(X, Y)\rangle = g(\nabla_Z X, Y) + g(X, \nabla^*_Z Y)$$
for all vector fields $X, Y, Z$.

This compatibility condition generalizes metric-compatibility ($\nabla g = 0$) to the dual setting. The Levi-Civita connection is the special case $\nabla = \nabla^*$ — it is self-dual. In information geometry, the two connections are genuinely different.

**Definition 20.3.2 ($\alpha$-Connections).** The *$\alpha$-connection* interpolates between the dual pair:
$$\nabla^{(\alpha)} = \frac{1+\alpha}{2}\nabla^{(e)} + \frac{1-\alpha}{2}\nabla^{(m)},$$
where:
- *$e$-connection* ($\alpha = 1$): the *exponential connection* (associated to exponential families)
- *$m$-connection* ($\alpha = -1$): the *mixture connection*
- $\alpha = 0$: the Levi-Civita connection

The $e$- and $m$-connections are dual to each other: $(\nabla^{(e)})^* = \nabla^{(m)}$.

**Theorem 20.3.3 (Exponential Families are $e$-Flat).** An exponential family is *$e$-flat*: the $e$-connection has zero curvature. Equivalently, the natural parameters $(\theta_i)$ form an affine coordinate system for $\nabla^{(e)}$.

What does flatness mean geometrically? Under the $e$-connection, the "straight lines" are exponential families themselves — one-dimensional subfamilies of the form $\{p_{\theta_0 + t v} : t \in \mathbb{R}\}$ for a fixed direction $v$. The exponential family is flat in the same sense that Euclidean space is flat: there is a global coordinate system in which all "straight lines" are actual straight lines.

Similarly, the mixture family $\{(1-t) p + t q : t \in [0,1]\}$ (convex combinations of two distributions) is $m$-flat — flat under the $m$-connection.

The flatness of exponential families under the $e$-connection means that many statistical problems — maximum likelihood estimation, hypothesis testing, sufficient statistics — have clean solutions in terms of affine geometry in the natural parameter space. The MLE is the $e$-projection of the empirical distribution onto the model manifold; hypothesis testing error exponents are geodesic distances under the appropriate connection.

This is the geometric unification that information geometry provides: statistical procedures that seem like independent techniques are all, at the geometric level, projections and distances in the space of distributions.
