# 15.2 Global Attractors

The central question of infinite-dimensional dynamics is: what is the long-time behavior of a PDE? For a dissipative PDE, energy is removed from the system (by viscosity, damping, or diffusion), and in the long run, the dynamics should concentrate on a smaller, "simpler" set. That set is the global attractor.

The global attractor is the PDE analog of the strange attractor for a finite-dimensional dissipative system. But there is a crucial difference: for a PDE, the phase space is infinite-dimensional, and a compact invariant set in an infinite-dimensional space is a genuinely different kind of object than a compact invariant set in $\mathbb{R}^n$.

## Semiflows and Attractors

**Definition 15.2.1.** A *semiflow* on a Banach space $X$ is a family $\{S(t)\}_{t \geq 0}$ of continuous maps $S(t): X \to X$ with $S(0) = \text{id}$ and $S(t+s) = S(t) \circ S(s)$. (The difference from a $C_0$-semigroup: here $S(t)$ need not be linear.)

For a semilinear PDE $\dot{u} = \mathcal{A}u + F(u)$ where $\mathcal{A}$ generates a linear $C_0$-semigroup and $F$ is a (typically nonlinear) lower-order term, the solution map $S(t): u_0 \mapsto u(t)$ is a nonlinear semiflow on an appropriate function space.

**Definition 15.2.2.** A *global attractor* for a semiflow $\{S(t)\}$ is a compact set $\mathcal{A} \subseteq X$ that is:
- *Invariant*: $S(t)\mathcal{A} = \mathcal{A}$ for all $t \geq 0$
- *Attracting*: $\text{dist}(S(t)B, \mathcal{A}) \to 0$ as $t \to \infty$ for every bounded set $B \subseteq X$

where $\text{dist}(C, D) = \sup_{x \in C} \inf_{y \in D} \|x - y\|$ is the Hausdorff semi-distance.

**Theorem 15.2.3 (Existence of Global Attractors).** A semiflow $\{S(t)\}$ has a global attractor if and only if it is:
1. *Asymptotically compact*: for any bounded sequence $(x_n)$ in $X$ and any $t_n \to \infty$, the sequence $(S(t_n)x_n)$ has a convergent subsequence.
2. *Pointwise dissipative*: there exists a bounded set $B_0$ (the "absorbing ball") such that for every $x \in X$, $S(t)x \in B_0$ for all large $t$.

What this is saying is: the system must both "compress" bounded sets (dissipativity: all orbits are eventually bounded) and have "compact eventual regularity" (asymptotic compactness: orbits in the far future lie in a compact set). When both hold, the $\omega$-limit set of any bounded set is compact and invariant, and the global attractor is the union of all such $\omega$-limit sets.

## The 2D Navier-Stokes Equations

The most important example is the 2D incompressible Navier-Stokes equations:
$$\partial_t u + (u \cdot \nabla)u = \nu\Delta u - \nabla p + f, \quad \nabla \cdot u = 0,$$
on a periodic domain (or a bounded domain with Dirichlet conditions). Here $u(x,t)$ is the fluid velocity, $p$ is pressure, $\nu > 0$ is viscosity, and $f$ is a fixed external forcing.

**Example 15.2.4 (2D Navier-Stokes Attractor).** Temam proved:
- The semiflow of the 2D Navier-Stokes equations has a global attractor $\mathcal{A}$ in $H = \{u \in L^2 : \nabla \cdot u = 0\}$ (the space of divergence-free square-integrable velocity fields).
- The global attractor has finite Hausdorff dimension: $\dim_H(\mathcal{A}) \leq C(\nu, f)$ where the constant depends on the viscosity and forcing.
- The Lyapunov dimension estimate gives $\dim_H(\mathcal{A}) \lesssim (L/\ell_d)^2$ where $L$ is the domain scale and $\ell_d = (\nu^3/\varepsilon)^{1/4}$ is the Kolmogorov dissipation scale. This is the mathematical content of the claim that 2D turbulence has finitely many degrees of freedom.

Why is the 2D case tractable when the 3D case is not? In 2D, the Sobolev inequality and the structure of the Navier-Stokes equations (specifically, the absence of vortex stretching) give an a priori bound on the $H^1$ norm of solutions. This regularity is what makes the semiflow asymptotically compact. In 3D, global regularity of solutions — let alone existence of a finite-dimensional attractor — is an open problem (indeed, one of the Clay Millennium Problems).

The finite Hausdorff dimension of the attractor is the rigorous foundation for the claim that turbulent flow, despite living in an infinite-dimensional phase space, has finitely many effective degrees of freedom.
