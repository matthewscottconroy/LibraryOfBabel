# 36.3 Cocycle Superrigidity

The key analytic tool in the Zimmer program is cocycle superrigidity. A cocycle is a "twisted homomorphism" that keeps track of how a group action varies over a space. For smooth actions, the derivative is the canonical cocycle: it measures how the tangent space transforms.

**Definition 36.3.1.** For a measure-preserving action $\Gamma \curvearrowright (X, \mu)$, a *measurable cocycle* is a measurable map $\alpha: \Gamma \times X \to H$ (into a group $H$) satisfying:
$$\alpha(\gamma_1\gamma_2, x) = \alpha(\gamma_1, \gamma_2 \cdot x)\alpha(\gamma_2, x).$$

If $H = GL(n, \mathbb{R})$ and the action is a smooth diffeomorphism on an $n$-dimensional manifold, then $\alpha(\gamma, x) = D_x\gamma$ (the derivative) is a cocycle: the chain rule gives exactly the cocycle equation.

Zimmer's cocycle superrigidity theorem says: for higher-rank lattices, every such cocycle is "algebraic" — it comes from a genuine group homomorphism.

**Theorem 36.3.2 (Zimmer's Cocycle Superrigidity, 1980).** Let $\Gamma \leq G$ (semisimple, higher rank) be a lattice and $\Gamma \curvearrowright (X, \mu)$ an ergodic action. Every measurable cocycle $\alpha: \Gamma \times X \to GL(n, {\mathbb R})$ is cohomologous (a.e.) to a group homomorphism $\rho: \Gamma \to GL(n, {\mathbb R})$ twisted by a measurable map into the Zariski closure of the image.

In other words: up to coboundary (a "change of frame"), every cocycle is a group homomorphism. The $x$-dependence is illusory — it can be removed by a measurable coordinate change.

**Consequence:** The derivative cocycle of a smooth volume-preserving action is constrained. If $\Gamma \curvearrowright M$, the derivative $D\gamma: TM \to TM$ defines a cocycle $\alpha(\gamma, x) = D_x\gamma \in GL(\dim M, {\mathbb R})$. Cocycle superrigidity says this derivative cocycle is "almost algebraic."

What does "almost algebraic" buy you? It means the Lyapunov exponents of the action — the eigenvalues of the derivative, in an asymptotic sense — are not arbitrary. They must come from a representation of the group $G$. And representations of $SL(n, \mathbb{R})$ have dimension at least $n - 1$ (the smallest nontrivial ones). This is how the dimension bound comes in.

The argument is: if the derivative cocycle must come from a representation of $G$, and the smallest faithful representation of $G = SL(n, \mathbb{R})$ has dimension $n - 1$, then the manifold must have dimension at least $n - 1$ to admit the derivative cocycle.
