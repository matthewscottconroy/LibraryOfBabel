# Unit IV Problems: Advanced Analysis and Topology

*Metric spaces, open sets, continuity, manifolds, and the topological foundations of differential geometry used throughout GR.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Metric Spaces and Topology

**Problem 1.1** ★
A metric space $(X, d)$ satisfies: (i) $d(x,y)\geq0$ and $d(x,y)=0\Leftrightarrow x=y$; (ii) $d(x,y) = d(y,x)$; (iii) $d(x,z)\leq d(x,y)+d(y,z)$ (triangle inequality).

(a) Show that the "discrete metric" $d(x,y) = 0$ if $x=y$, $d(x,y) = 1$ if $x\neq y$, is indeed a metric. What do the open balls $B_r(x)$ look like for $r < 1$ and $r > 1$?

(b) On $\mathbb{R}^n$, show that the $L^\infty$ metric $d_\infty(\mathbf{x},\mathbf{y}) = \max_i|x_i - y_i|$ satisfies the triangle inequality.

(c) Does the Lorentzian "distance" $\eta(v,v) = -v_0^2 + v_1^2 + v_2^2 + v_3^2$ define a metric on $\mathbb{R}^{3,1}$? Which axiom fails?

**Problem 1.2** ★
Open sets and continuity:

(a) Prove: in a metric space, the open ball $B_r(x) = \{y: d(x,y) < r\}$ is an open set.
(b) Prove: the intersection of finitely many open sets is open. Give a counterexample showing the intersection of infinitely many open sets need not be open.
(c) A function $f: X\to Y$ (between metric spaces) is continuous iff for every open set $U\subset Y$, the preimage $f^{-1}(U)$ is open in $X$. Verify this for $f = \sin: \mathbb{R}\to[-1,1]$ and the open interval $U = (-1/2, 1/2)$.

**Problem 1.3** ★★
Compactness and completeness:

(a) The Heine-Borel theorem: a subset of $\mathbb{R}^n$ is compact iff it is closed and bounded. Verify for the unit sphere $S^2\subset\mathbb{R}^3$.

(b) A Cauchy sequence in a metric space has the property that $d(x_m, x_n)\to 0$ as $m,n\to\infty$. Show that $\mathbb{Q}$ is not complete by exhibiting a Cauchy sequence of rationals converging to $\sqrt{2}$.

(c) In GR, geodesic completeness means that every geodesic can be extended to all parameter values $\lambda\in(-\infty,\infty)$. Why is this related to metric completeness? (Hint: consider the Cauchy completion of the metric space of points along the geodesic.)

**Problem 1.4** ★★
The topology of spacetime: the Minkowski space $\mathbb{R}^{3,1}$ has a natural topology (as a topological space, ignoring the metric signature).

(a) Is Minkowski space simply connected? (Does every closed loop contract to a point?) What about $\mathbb{R}^{3,1}\setminus\{0\}$ (Minkowski space with the origin removed)?

(b) The Schwarzschild exterior spacetime (the region $r > 2M$) is topologically $\mathbb{R}^2\times S^2$. What is the fundamental group $\pi_1$?

(c) The topology of the Kruskal extension of Schwarzschild: there are two exterior regions. What additional identifications are made? Is the extended spacetime simply connected?

---

## Part 2: Manifolds

**Problem 2.1** ★★
A smooth $n$-manifold $M$ is a topological space with an atlas: a collection of charts $(U_\alpha, \phi_\alpha)$ where $U_\alpha\subset M$ are open sets covering $M$ and $\phi_\alpha: U_\alpha\to\mathbb{R}^n$ are homeomorphisms, with smooth transition maps $\phi_\beta\circ\phi_\alpha^{-1}$ on overlaps.

(a) The 2-sphere $S^2$: describe two charts (stereographic projection from the north and south poles). Compute the transition map on the overlap.

(b) Why does $S^2$ require at least two charts? (Hint: think about whether a single chart can cover all of $S^2$.)

(c) In GR, a spacetime is a 4-dimensional smooth manifold equipped with a Lorentzian metric. The Schwarzschild coordinate chart $(t, r, \theta, \phi)$ fails at $r = 0$ and $r = 2M$. Which failure is a genuine singularity (all charts fail) and which is merely a coordinate singularity (a different chart extends across it)?

**Problem 2.2** ★★
Tangent vectors: at a point $p\in M$, the tangent vector $v$ can be defined as a derivation on smooth functions: $v(fg) = v(f)g(p) + f(p)v(g)$.

(a) Show that $\partial/\partial x^i$ (at $p$) satisfies the derivation property.
(b) Show that $\{\partial/\partial x^1,\ldots,\partial/\partial x^n\}$ are linearly independent. (Hint: apply each to the coordinate function $x^j$.)
(c) The pushforward $f_*: T_p M\to T_{f(p)}N$ of a smooth map $f: M\to N$ acts as $(f_*v)(g) = v(g\circ f)$. Compute the pushforward of $\partial/\partial r$ (in polar coordinates on $\mathbb{R}^2$) to Cartesian coordinates. What is the result in terms of $\partial/\partial x$ and $\partial/\partial y$?

**Problem 2.3** ★★★
The Lie bracket: for vector fields $X, Y$ on $M$, the Lie bracket $[X,Y]$ is defined by $[X,Y]f = X(Yf) - Y(Xf)$.

(a) In coordinates, $[X,Y]^\mu = X^\nu\partial_\nu Y^\mu - Y^\nu\partial_\nu X^\mu$. Derive this from the definition.
(b) Show $[X,Y] = -[Y,X]$ and $[X,[Y,Z]] + [Y,[Z,X]] + [Z,[X,Y]] = 0$ (Jacobi identity).
(c) For the coordinate basis $\{\partial_\mu\}$: $[\partial_\mu, \partial_\nu] = 0$. What does this mean about the choice of coordinates? Can you always find coordinates in which a given set of vector fields becomes a coordinate basis?

**Problem 2.4** ★★★
Differential forms and exterior derivative:

(a) A $p$-form $\omega$ is a totally antisymmetric $(0,p)$-tensor. On $\mathbb{R}^3$: match 0-forms, 1-forms, 2-forms, and 3-forms with scalar functions, vector fields, and volume forms (using the correspondence via the Hodge dual $\star$).

(b) The exterior derivative $d$ maps $p$-forms to $(p+1)$-forms: $(d\omega)_{\mu_0\mu_1\cdots\mu_p} = (p+1)\partial_{[\mu_0}\omega_{\mu_1\cdots\mu_p]}$. Compute $d(f)$ (0-form), $d(A_\mu dx^\mu)$ (1-form), and show $d^2 = 0$.

(c) The de Rham cohomology $H^p(M) = \ker d|_{\Omega^p}/\text{im}\,d|_{\Omega^{p-1}}$ measures "holes" in $M$. For $S^1$: what is $H^0(S^1)$ and $H^1(S^1)$? (This is a topological invariant of the manifold.)

(d) In GR, Maxwell's equations in covariant form: $dF = 0$ (Bianchi identity, equivalent to $\nabla_{[\mu}F_{\nu\rho]} = 0$) and $d\star F = \mu_0\star J$ (source equation). Identify $F$ as a 2-form and write the components of $F$ in terms of $\mathbf{E}$ and $\mathbf{B}$.

**Problem 2.5** ★★★
Fiber bundles (conceptual):

(a) A fiber bundle $\pi: E\to B$ has base $B$, fiber $F$, and total space $E$ such that locally $E\approx U\times F$ (product, for $U\subset B$ open). For the tangent bundle $TM$: what are $E$, $B$, and $F$?

(b) A section of $TM$ is a smooth assignment of a tangent vector at each point — i.e., a vector field. The hairy ball theorem says $S^2$ has no nowhere-vanishing vector field. What does this say about sections of $TS^2$?

(c) A connection on a fiber bundle specifies how to parallel-transport fibers along curves in the base. In the tangent bundle $TM$, a connection is exactly the Levi-Civita connection of GR. In a gauge theory, connections on a principal bundle $P\to M$ with gauge group $G$ correspond to gauge fields. What is the analogue of the curvature tensor?
