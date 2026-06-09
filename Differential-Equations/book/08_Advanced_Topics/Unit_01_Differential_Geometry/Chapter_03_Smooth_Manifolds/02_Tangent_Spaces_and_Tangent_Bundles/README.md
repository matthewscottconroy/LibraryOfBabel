# Tangent Spaces and Tangent Bundles

On a smooth manifold, there is no canonical way to compare vectors at different points: the notion of "parallel" has no intrinsic meaning without additional structure (a connection). But at each individual point, the collection of velocity vectors of all smooth curves through that point forms a well-defined vector space, the tangent space. The tangent bundle assembles all tangent spaces into a single manifold, providing the natural setting for vector fields and differential equations on manifolds.

## Tangent Vectors via Curves

Let $M$ be a smooth $n$-manifold and $p \in M$. A **tangent vector at $p$** (in the curve definition) is an equivalence class of smooth curves $\gamma: (-\varepsilon, \varepsilon) \to M$ with $\gamma(0) = p$, where two curves are equivalent if they have the same derivative in every local chart:

$$\gamma_1 \sim \gamma_2 \iff \frac{d}{dt}(\phi \circ \gamma_1)\bigg|_{t=0} = \frac{d}{dt}(\phi \circ \gamma_2)\bigg|_{t=0}$$

for some (equivalently, every) chart $\phi$ at $p$. The equivalence class of $\gamma$ is the tangent vector $\gamma'(0) \in T_pM$.

In a chart $(U, \phi)$ with $\phi = (x^1, \ldots, x^n)$, the curve $\gamma(t) = \phi^{-1}(\phi(p) + te_i)$ (moving in the $x^i$ direction) has tangent vector denoted $\partial/\partial x^i|_p$. These form a basis for $T_pM$: every tangent vector is $v = \sum_i v^i \partial/\partial x^i|_p$ for unique $v^i \in \mathbb{R}$.

## Tangent Vectors as Derivations

An equivalent definition, more algebraically natural, identifies tangent vectors with derivations. A **derivation at $p$** is a linear map $X_p: C^\infty(M) \to \mathbb{R}$ satisfying the Leibniz rule:

$$X_p(fg) = f(p) X_p(g) + g(p) X_p(f).$$

The set of all derivations at $p$ is a vector space; one can show it is $n$-dimensional, with basis $\{\partial/\partial x^i|_p\}$ acting by $\partial/\partial x^i|_p (f) = \partial(f \circ \phi^{-1})/\partial x^i|_{\phi(p)}$. The two definitions (via curves and via derivations) are equivalent: the derivation corresponding to $\gamma'(0)$ is $f \mapsto (f \circ \gamma)'(0)$.

## The Differential of a Map

For a smooth map $F: M \to N$ and a point $p \in M$, the **differential** (or **pushforward**) $dF_p: T_pM \to T_{F(p)}N$ is the linear map defined by:

$$dF_p(v)(f) = v(f \circ F), \quad v \in T_pM, f \in C^\infty(N).$$

In coordinates $(x^i)$ near $p$ and $(y^j)$ near $F(p)$, the matrix of $dF_p$ in the coordinate bases is the Jacobian $(\partial F^j/\partial x^i)_{ji}$.

The chain rule holds: $d(G \circ F)_p = dG_{F(p)} \circ dF_p$.

A smooth map $F$ is an **immersion** if $dF_p$ is injective for all $p$, and a **submersion** if $dF_p$ is surjective for all $p$. An injective immersion with image a submanifold and with everywhere injective derivative is an **embedding**.

## The Tangent Bundle

The **tangent bundle** is the disjoint union

$$TM = \bigsqcup_{p \in M} T_pM = \{(p, v) : p \in M, v \in T_pM\}.$$

There is a natural projection $\pi: TM \to M$, $(p, v) \mapsto p$. The fiber $\pi^{-1}(p) = T_pM$ is the tangent space at $p$.

**Smooth structure on $TM$.** For each chart $(U, \phi)$ of $M$, define $\tilde{\phi}: \pi^{-1}(U) \to \phi(U) \times \mathbb{R}^n$ by $\tilde{\phi}(p, v) = (\phi(p), v^1, \ldots, v^n)$ where $v = \sum v^i \partial/\partial x^i|_p$. These maps form an atlas for $TM$, making it a smooth $2n$-manifold. The transition maps for $TM$ are:

$$\tilde{\phi}_\beta \circ \tilde{\phi}_\alpha^{-1}(x, w) = \left(\phi_{\beta\alpha}(x), D\phi_{\beta\alpha}(x) \cdot w\right),$$

where $\phi_{\beta\alpha} = \phi_\beta \circ \phi_\alpha^{-1}$ is the transition map of $M$ and $D\phi_{\beta\alpha}$ is its Jacobian. This transition is smooth, confirming $TM$ is a smooth manifold.

## Vector Fields

A **vector field** on $M$ is a smooth section of the tangent bundle: a smooth map $X: M \to TM$ with $\pi \circ X = \text{id}_M$. In local coordinates, $X = \sum_{i=1}^n X^i(x) \partial/\partial x^i$ where $X^i: U \to \mathbb{R}$ are smooth functions. The space of smooth vector fields is denoted $\mathfrak{X}(M)$.

**Integral curves.** An **integral curve** of $X$ through $p_0$ is a smooth curve $\gamma: I \to M$ with $\gamma(0) = p_0$ and $\gamma'(t) = X(\gamma(t))$ for all $t \in I$. The existence and uniqueness theorem for ODEs guarantees that integral curves exist and are unique for short times (when $M$ is compact, for all time). The flow of $X$ is the map $\Phi_t(p) = \gamma_p(t)$ where $\gamma_p$ is the integral curve through $p$.

**Lie bracket.** The **Lie bracket** of vector fields $X, Y \in \mathfrak{X}(M)$ is the vector field $[X, Y]$ defined by:

$$[X, Y](f) = X(Y(f)) - Y(X(f)).$$

In coordinates: $[X, Y]^k = \sum_i \left(X^i \frac{\partial Y^k}{\partial x^i} - Y^i \frac{\partial X^k}{\partial x^i}\right)$.

The Lie bracket measures the failure of the flows of $X$ and $Y$ to commute: $[\Phi_t^X, \Phi_s^Y] = \Phi_{-t}^X \circ \Phi_{-s}^Y \circ \Phi_t^X \circ \Phi_s^Y$ has derivative proportional to $[X, Y]$ at $(t, s) = (0, 0)$.

## Cotangent Bundle and 1-Forms

The **cotangent space** $T^*_pM$ is the dual of $T_pM$: the space of linear functionals $T_pM \to \mathbb{R}$. The **cotangent bundle** $T^*M = \bigsqcup_p T^*_pM$ is a smooth $2n$-manifold. A **smooth 1-form** (or **covector field**) is a smooth section of $T^*M$. In coordinates, $\omega = \sum_i \omega_i \, dx^i$ where $dx^i(p): T_pM \to \mathbb{R}$ is the dual basis element.

For a smooth function $f: M \to \mathbb{R}$, the **differential** $df: M \to T^*M$ is defined by $df_p(v) = v(f)$. In coordinates, $df = \sum_i \frac{\partial f}{\partial x^i} dx^i$.

## The Tangent Bundle and ODEs on Manifolds

The fundamental role of the tangent bundle in dynamical systems is this: a first-order autonomous ODE on a manifold $M$ is precisely a vector field $X \in \mathfrak{X}(M)$. The solutions are the integral curves. This perspective unifies all the ODE theory studied in previous modules with the geometric language of manifolds, and it is the starting point for the geometric theory of dynamical systems on manifolds—symplectic mechanics, geodesic flows, and Hamiltonian systems.
