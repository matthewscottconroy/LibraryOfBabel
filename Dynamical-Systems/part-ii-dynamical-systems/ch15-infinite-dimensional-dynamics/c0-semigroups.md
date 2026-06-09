# 15.1 $C_0$-Semigroups

For a finite-dimensional ODE $\dot{x} = Ax$, the flow is $x(t) = e^{tA}x_0$: the matrix exponential gives the time-$t$ map as a one-parameter group. When we move to PDEs — where the "vector field" is an unbounded differential operator on an infinite-dimensional function space — the matrix exponential needs a replacement. That replacement is the $C_0$-semigroup.

The key adjustment from groups to semigroups is that we only go forward in time. For dissipative PDEs (like the heat equation), the backward problem is ill-posed: you cannot uniquely reconstruct the past from the present. So the flow is not a group but a semigroup — defined only for $t \geq 0$.

**Definition 15.1.1.** A *strongly continuous one-parameter semigroup* ($C_0$-semigroup) on a Banach space $X$ is a family $\{T(t)\}_{t \geq 0} \subseteq \mathcal{B}(X)$ (bounded linear operators on $X$) satisfying:
1. $T(0) = I$ (identity at time 0)
2. $T(t+s) = T(t)T(s)$ for all $s, t \geq 0$ (semigroup property)
3. $\lim_{t \to 0^+} T(t)x = x$ for all $x \in X$ (strong continuity)

The *infinitesimal generator* is:
$$\mathcal{A}x = \lim_{t \to 0^+} \frac{T(t)x - x}{t}$$
with domain $D(\mathcal{A}) = \{x \in X : \text{the above limit exists}\}$.

What this is saying is: the generator $\mathcal{A}$ is the "velocity" of the semigroup at $t = 0$. It plays the role of the matrix $A$ in the finite-dimensional setting: $T(t) = e^{t\mathcal{A}}$ in a formal sense. But for unbounded operators (like differential operators), the "exponential" must be constructed via the semigroup definition, not by power series.

**Theorem 15.1.2 (Hille-Yosida).** A linear operator $\mathcal{A}: D(\mathcal{A}) \to X$ is the generator of a $C_0$-semigroup iff:
- $\mathcal{A}$ is closed (its graph is closed) and densely defined ($D(\mathcal{A})$ is dense in $X$)
- The resolvent $(\lambda I - \mathcal{A})^{-1}$ exists for all $\lambda > \omega$ (for some $\omega \in \mathbb{R}$) with $\|(\lambda I - \mathcal{A})^{-n}\| \leq M/(\lambda - \omega)^n$

The Hille-Yosida theorem characterizes, algebraically, which operators can be "exponentiated" to give semigroups. It is the infinite-dimensional analog of asking which matrices have well-defined matrix exponentials (answer: all of them) — but in infinite dimensions, not all operators have semigroups, and the conditions above are exactly what is needed.

**Examples:**

- *Heat equation* on $L^2(\Omega)$: $\mathcal{A} = \Delta$ (Laplacian with, say, Dirichlet boundary conditions on a bounded domain $\Omega$). The semigroup is $T(t) = e^{t\Delta}$ — the heat semigroup. It regularizes: for any $t > 0$, $T(t)$ maps $L^2$ to $C^\infty$. The domain $D(\Delta) = H^2(\Omega) \cap H^1_0(\Omega)$ (the Sobolev space of functions with two square-integrable derivatives that vanish on the boundary).

- *Translation semigroup* on $L^2(\mathbb{R})$: $T(t)f(x) = f(x+t)$ (shift to the left by $t$). Generator is $\mathcal{A} = d/dx$, with domain $H^1(\mathbb{R})$.

- *Delay differential equations*: the equation $\dot{x}(t) = F(x(t), x(t-h))$ can be reformulated as a $C_0$-semigroup on $C([-h, 0]; \mathbb{R}^n)$ (the space of continuous functions on the delay interval), making it accessible to the semigroup theory even though the original equation is not an ODE.

The semigroup framework is not just a formalism — it gives access to spectral theory, perturbation theory, and regularity results that are hard to obtain directly from the PDE.
