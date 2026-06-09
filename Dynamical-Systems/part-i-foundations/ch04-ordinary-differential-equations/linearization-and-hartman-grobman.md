# 4.4 Linearization and the Hartman-Grobman Theorem

Near a hyperbolic equilibrium, the nonlinear system behaves like its linearization. This is a remarkable and useful fact — it means you can understand the local geometry of trajectories by looking at a much simpler, linear system. The Hartman-Grobman theorem makes this precise.

## 4.4.1 Equilibria and Linearization

**Definition 4.4.1.** A point $p \in M$ is an *equilibrium* (or *fixed point*) of $\dot{x} = f(x)$ if $f(p) = 0$.

Near a equilibrium $p$, Taylor expand: $f(x) = Df(p)(x-p) + O(\|x-p\|^2)$. The *linearization* at $p$ is the linear system $\dot{y} = Df(p) y$ (where $y = x - p$).

The question: how much does the linearization tell you about the nonlinear system? For non-hyperbolic equilibria, not much — the higher-order terms can completely change the behavior (e.g., they can break a center into a spiral). But for hyperbolic equilibria, the linearization captures the topology of trajectories exactly.

**Definition 4.4.2.** An equilibrium $p$ is *hyperbolic* if $Df(p)$ is a hyperbolic matrix (all eigenvalues have nonzero real part).

## 4.4.2 Hartman-Grobman Theorem

**Theorem 4.4.3 (Hartman-Grobman).** Let $p$ be a hyperbolic equilibrium of the $C^1$ vector field $f$. Then there exists a homeomorphism $h: U \to V$ (between neighborhoods of $p$ and $0$) that *conjugates* the nonlinear flow to the linear flow:
$$h \circ \Phi_t = e^{tA} \circ h \quad \text{on } U \cap \Phi_{[-T,T]}(U)$$
where $A = Df(p)$.

What this is really saying: near a hyperbolic equilibrium, the nonlinear flow is *topologically conjugate* to its linearization. There's a continuous change of coordinates that maps one to the other. The phase portrait of the nonlinear system looks qualitatively the same as the linearization's phase portrait — same topology of trajectories, even if the precise shapes differ.

**Remark 4.4.4.** The homeomorphism $h$ is in general not differentiable at $p$. Smooth conjugacy requires Sternberg's theorem and nonresonance conditions on the eigenvalues. The nonresonance conditions prevent certain algebraic relations between eigenvalues that would force higher-order corrections.

This theorem is proved by a contraction mapping argument in the space of continuous functions that conjugate the flows. The idea: write the conjugacy $h = \text{id} + u$ where $u$ is a small correction, and show that the equation $h \circ \Phi_t = e^{tA} \circ h$ becomes a fixed-point equation for $u$ in a suitable function space. The hyperbolicity condition (no imaginary axis eigenvalues) is what makes this map contractive.

## 4.4.3 Stable and Unstable Manifolds

The stable manifold theorem shows that the stable and unstable subspaces of the linearization extend to nonlinear invariant manifolds of the full system:

**Theorem 4.4.5 (Stable Manifold Theorem / Hadamard-Perron).** Let $p$ be a hyperbolic equilibrium with $A = Df(p)$ having stable subspace $E^s$ and unstable subspace $E^u$. Then there exist:
- A *local stable manifold* $W^s_{\text{loc}}(p)$: a $C^1$ submanifold tangent to $E^s$ at $p$, with $\Phi_t(x) \to p$ exponentially as $t \to +\infty$ for $x \in W^s_{\text{loc}}(p)$.
- A *local unstable manifold* $W^u_{\text{loc}}(p)$: tangent to $E^u$ at $p$, with $\Phi_t(x) \to p$ exponentially as $t \to -\infty$ for $x \in W^u_{\text{loc}}(p)$.

The *global stable manifold* is $W^s(p) = \bigcup_{t \leq 0} \Phi_t(W^s_{\text{loc}}(p))$ and similarly for $W^u(p)$.

*(proof sketch)* The stable manifold is the fixed point of a graph transform operator. Write the flow in the splitting $E^s \oplus E^u$. A graph over $E^s$ is mapped to a graph under the flow if and only if it is a fixed point of a certain contraction. The contraction mapping theorem gives the unique fixed point, which is the stable manifold.

Here's the key idea behind the proof. Consider graphs of the form $\{(x, g(x)) : x \in E^s\}$ where $g: E^s \to E^u$ is a Lipschitz function. Flowing such a graph forward under the dynamics for a short time gives another graph (by hyperbolicity — the strong expansion in the $E^u$ direction "straightens" the graph). This flow-on-graphs map is a contraction in a suitable metric, so by the Banach Fixed Point Theorem it has a unique fixed point — that's the local stable manifold.

**Application.** The stable and unstable manifolds of hyperbolic fixed points are the skeleton around which dynamics organizes itself. Their intersections — where a piece of $W^u(p)$ intersects $W^s(q)$ for hyperbolic points $p$ and $q$ — are the *homoclinic* and *heteroclinic* orbits that generate complicated dynamics. Poincaré was the first to recognize that the intersection of stable and unstable manifolds of the same fixed point (a *transverse homoclinic point*) implies chaotic behavior. This is developed in Chapter 9.
