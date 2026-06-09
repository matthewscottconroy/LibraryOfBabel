# 8.7 Stability in Discrete-Time Systems

The continuous-time theory of stability translates directly to discrete-time maps, with sums replacing integrals and the Jacobian $Dg(x^*)$ replacing the system matrix $A$. We state the discrete-time versions for completeness.

For a discrete map $x_{n+1} = g(x_n)$:

**Definition 8.7.1.** The equilibrium $x^* = 0$ of $x_{n+1} = g(x_n)$ is:
- *Lyapunov stable*: $\forall \varepsilon > 0\ \exists \delta > 0$: $\|x_0\| < \delta \Rightarrow \|g^n(x_0)\| < \varepsilon\ \forall n \geq 0$
- *Asymptotically stable*: stable and $g^n(x_0) \to 0$ as $n \to \infty$

**Theorem 8.7.2.** The linearization $Dg(0)$ determines local stability when all eigenvalues have $|\lambda| \neq 1$:
- $|\lambda_i| < 1$ for all $i$: asymptotically stable
- $|\lambda_i| > 1$ for some $i$: unstable

For discrete Lyapunov functions: $V \geq 0$, $V(0) = 0$, and $V(g(x)) \leq \alpha V(x)$ for some $\alpha < 1$ gives exponential stability.

The condition $|\lambda| < 1$ (for discrete maps) is the analogue of $\text{Re}(\lambda) < 0$ for continuous-time systems. In both cases, the eigenvalues need to be "on the stable side": inside the unit disk for maps, in the left half-plane for flows.

The discrete Lyapunov function condition is also a direct analogue: instead of $\dot{V} \leq -\alpha V$, you need $V(g(x)) \leq \alpha V(x)$ for $\alpha < 1$. Each application of $g$ reduces $V$ by a factor of at most $\alpha$, giving exponential decay after $n$ steps: $V(g^n(x)) \leq \alpha^n V(x)$.

---

## Looking Ahead

This chapter has developed the stability vocabulary that Chapter 9 will use constantly. Hyperbolic sets, Anosov diffeomorphisms, and SRB measures are all defined using the stable/unstable splitting — which is precisely the Lyapunov exponent structure of Definition 8.5.1. The connection is not incidental: a system is hyperbolic exactly when the Lyapunov exponents are bounded away from zero.

Pesin's formula (Theorem 8.5.6) will reappear in Chapter 9 as the key tool for computing the entropy of Anosov diffeomorphisms and connecting it to their geometric structure. The stability theory of this chapter is the bridge between the linear algebra of eigenvalues and the geometry of stable/unstable manifolds.
