# 1.4 Completeness and the Contraction Mapping Theorem

## 1.4.1 The Banach Fixed Point Theorem

Here's one of the most useful theorems in all of analysis, and the one that makes dynamical iteration rigorous.

The setup: you have a complete metric space and a map from it to itself that "pulls points together." The theorem says iteration converges — no matter where you start.

**Definition 1.4.1.** A map $f: X \to X$ is a *contraction* if there exists $\lambda \in [0, 1)$ such that $d(f(x), f(y)) \leq \lambda \cdot d(x, y)$ for all $x, y \in X$.

The number $\lambda$ is the *contraction constant*. The condition $\lambda < 1$ is essential — it says the map is strictly contracting. A Lipschitz map with constant exactly $1$ (an isometry) need not have any fixed point at all.

**Theorem 1.4.2 (Banach Fixed Point Theorem / Contraction Mapping Theorem).** Let $(X, d)$ be a complete metric space and $f: X \to X$ a contraction with constant $\lambda$. Then:
1. $f$ has a unique fixed point $x^* \in X$.
2. For any $x_0 \in X$, the iterates $f^n(x_0) \to x^*$ as $n \to \infty$.
3. The rate of convergence is $d(f^n(x_0), x^*) \leq \lambda^n \cdot d(x_0, x^*)$.

*(proof)* The sequence $(f^n(x_0))$ is Cauchy: $d(f^m(x_0), f^n(x_0)) \leq \lambda^{\min(m,n)} d(x_0, f(x_0)) / (1-\lambda)$. By completeness it converges to some $x^*$, and continuity of $f$ gives $f(x^*) = x^*$. Uniqueness: if $x^*, y^*$ are both fixed, then $d(x^*, y^*) = d(f(x^*), f(y^*)) \leq \lambda d(x^*, y^*)$, so $d(x^*, y^*) = 0$.

What this is really saying: if a map is a contraction — it pulls things together — then iteration converges, no matter where you start, and it converges exponentially fast. The fixed point is the attractor, and the contraction constant $\lambda$ is the rate.

The proof is instructive. Existence comes from completeness (the Cauchy sequence converges because there's somewhere for it to go). Uniqueness comes from the contraction condition itself (two fixed points would have to be at distance zero). This is the prototype for existence-and-uniqueness arguments throughout mathematics.

**Application in Dynamics.** The contraction mapping theorem is foundational in at least three ways:

1. *Existence and uniqueness for ODEs (Picard-Lindelöf):* The solution map $T[\varphi](t) = x_0 + \int_0^t f(\varphi(s))\,ds$ is a contraction on a suitable space of curves, so it has a unique fixed point — which is the solution.

2. *Existence of stable manifolds:* The "graph transform" that maps graphs over the stable subspace to other graphs is a contraction. The stable manifold is its unique fixed point. This is developed in Chapter 4.

3. *Fractal attractors:* Iterated function systems — collections of contractions on a complete metric space — have a unique compact invariant set (the attractor), given by the contraction mapping theorem in the space of compact subsets.

The contraction mapping theorem is the place where "dynamical systems" and "analysis" are most visibly the same thing: iteration is the method, fixed points are the objects, and completeness is what makes convergence possible.
