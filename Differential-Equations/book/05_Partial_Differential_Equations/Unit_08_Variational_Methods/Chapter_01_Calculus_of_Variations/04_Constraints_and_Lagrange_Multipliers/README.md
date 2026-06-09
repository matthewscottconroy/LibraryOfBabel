# Constraints and Lagrange Multipliers in the Calculus of Variations

Many variational problems have constraints: find the function that minimizes an energy subject to a side condition. The prototype is the **isoperimetric problem**: among all closed curves of length $L$, find the one enclosing maximum area. The answer (the circle) was known to the Greeks, but the proof requires the calculus of variations. Lagrange multipliers handle such constraints exactly as in finite-dimensional optimization: adjoin the constraint with a multiplier to the functional, then apply the unconstrained Euler-Lagrange theory. This section develops the Lagrange multiplier method and applies it to isoperimetric problems and eigenvalue problems.

## Equality Constraints: The Lagrange Multiplier Method

**Setup.** Minimize $\mathcal{E}[u] = \int_a^b L(x,u,u')\,dx$ subject to the constraint $\mathcal{G}[u] = \int_a^b M(x,u,u')\,dx = C$ (a constant).

**Method.** Form the augmented functional:

$$\mathcal{F}[u;\lambda] = \mathcal{E}[u] - \lambda(\mathcal{G}[u] - C) = \int_a^b[L(x,u,u') - \lambda M(x,u,u')]\,dx + \lambda C.$$

The Euler-Lagrange equation for $\mathcal{F}$ with $\lambda$ fixed is:

$$(L-\lambda M)_u - \frac{d}{dx}(L-\lambda M)_{u'} = 0,$$

i.e., $L_u - \frac{d}{dx}L_{u'} = \lambda\left[M_u - \frac{d}{dx}M_{u'}\right]$.

This is solved simultaneously with the constraint $\mathcal{G}[u] = C$, with $\lambda$ as the unknown multiplier. The equation says: the (unconstrained) gradient of $\mathcal{E}$ equals $\lambda$ times the (unconstrained) gradient of $\mathcal{G}$ — the gradients are proportional at the constrained optimum.

## Isoperimetric Problems

**The classical isoperimetric problem.** Among all curves of fixed perimeter $P$, find the one enclosing maximum area. In parametric form: the curve $(x(t),y(t))$ has perimeter $\int_0^1\sqrt{\dot x^2+\dot y^2}\,dt = P$ and enclosed area $A = \frac{1}{2}\int_0^1(x\dot y - y\dot x)\,dt$ (Green's formula). Maximize $A$ subject to fixed perimeter $P$.

The Euler-Lagrange equations with Lagrange multiplier $\lambda$:

$$\frac{d}{dt}\frac{\dot x}{\sqrt{\dot x^2+\dot y^2}} = -\frac{\lambda\dot y}{2}, \qquad \frac{d}{dt}\frac{\dot y}{\sqrt{\dot x^2+\dot y^2}} = \frac{\lambda\dot x}{2}.$$

These say that the curvature $\kappa = 1/R$ equals $-\lambda$ (a constant) — the curve has constant curvature: a circle. The radius $R = 1/|\lambda|$ is determined by $2\pi R = P$, giving $R = P/(2\pi)$ and $A = \pi R^2 = P^2/(4\pi)$ — the isoperimetric inequality $4\pi A \leq P^2$, with equality for the circle.

**Dido's problem.** Among all curves of fixed length $P$ joining two fixed points on the $x$-axis and enclosing maximum area with the $x$-axis: the answer is a semicircle. This is a constrained optimization (fixed endpoints plus fixed length).

## Eigenvalue Problems as Constrained Optimization

The eigenvalue problem $-\Delta u = \lambda u$ in $\Omega$ with $u = 0$ on $\partial\Omega$ can be stated variationally:

**Rayleigh quotient.** The smallest eigenvalue $\lambda_1$ is:

$$\lambda_1 = \min_{u\in H^1_0(\Omega),\,u\neq 0}\frac{\int_\Omega|\nabla u|^2\,dx}{\int_\Omega u^2\,dx} = \min_{u\in H^1_0(\Omega),\,\|u\|_{L^2}=1}\int_\Omega|\nabla u|^2\,dx. \tag{Rayleigh}$$

**Derivation via Lagrange multipliers.** Minimize $\mathcal{E}[u] = \int_\Omega|\nabla u|^2\,dx$ subject to $\mathcal{G}[u] = \int_\Omega u^2\,dx = 1$.

The augmented functional: $\mathcal{F}[u;\lambda] = \int_\Omega|\nabla u|^2\,dx - \lambda\int_\Omega u^2\,dx$.

Euler-Lagrange equation: $-2\Delta u - 2\lambda u = 0$, i.e., $-\Delta u = \lambda u$.

The multiplier $\lambda$ is the eigenvalue! The Lagrange multiplier condition produces the eigenvalue equation: the constrained minimum of the Dirichlet energy (normalized by $L^2$ norm) is the first eigenvalue, and the minimizer is the first eigenfunction.

**Higher eigenvalues (min-max theorem):** The $k$-th eigenvalue is:

$$\lambda_k = \min_{\substack{u\in H^1_0(\Omega), \|u\|_{L^2}=1 \\ u\perp u_1,\ldots,u_{k-1}}}\int_\Omega|\nabla u|^2\,dx = \min_{V_k}\max_{u\in V_k,\|u\|_{L^2}=1}\int_\Omega|\nabla u|^2\,dx,$$

where the outer minimum is over all $k$-dimensional subspaces $V_k\subset H^1_0(\Omega)$ (Courant min-max principle).

**Application: domain monotonicity.** If $\Omega_1\subset\Omega_2$, then $\lambda_1(\Omega_1) \geq \lambda_1(\Omega_2)$ (larger domain = smaller first eigenvalue). This follows from the Rayleigh quotient: any function in $H^1_0(\Omega_1)$ can be extended by zero to $H^1_0(\Omega_2)$, and the extension has the same $L^2$ norm and Dirichlet energy, so $\lambda_1(\Omega_2) \leq R[u] = \lambda_1(\Omega_1)$.

**Faber-Krahn inequality.** Among all domains $\Omega$ of fixed volume $|\Omega|$, the ball $B$ minimizes the first eigenvalue: $\lambda_1(\Omega) \geq \lambda_1(B)$. The proof uses the Schwarz symmetrization and the Rayleigh quotient — a deep application of the variational characterization.

## Integral Constraints of General Form

**Problem.** Minimize $\mathcal{E}[u] = \int_\Omega G(u)\,dx$ subject to $\int_\Omega u\,dx = M$ (conservation of mass) and $u \geq 0$ (positivity constraint).

For the equality constraint: form $\mathcal{F}[u;\lambda] = \int_\Omega G(u)\,dx - \lambda(\int_\Omega u\,dx - M)$. E-L: $G'(u) = \lambda$ (pointwise). If $G$ is strictly convex, this gives $u = (G')^{-1}(\lambda)$ — a constant! The constant value is determined by the mass constraint $u = M/|\Omega|$. The unique minimizer is the constant function.

For the positivity constraint: the Lagrange multiplier becomes a measure (a distribution), and the condition is $G'(u) = \lambda + \mu$ where $\mu \geq 0$ and $\mu u = 0$ (complementarity condition). This is the Karush-Kuhn-Tucker condition for the constrained optimization.

## Geometric Constraints: Geodesics

On a Riemannian manifold $(M,g)$, geodesics are curves $\gamma:[0,1]\to M$ minimizing $\text{Length}[\gamma] = \int_0^1\sqrt{g(\dot\gamma,\dot\gamma)}\,dt$. This is an unconstrained variational problem in the space of curves, but equivalent to the constrained problem: minimize $\text{Energy}[\gamma] = \frac{1}{2}\int_0^1 g(\dot\gamma,\dot\gamma)\,dt$ subject to $|\dot\gamma| = \text{const}$ (constant-speed parameterization). The Euler-Lagrange equation for the energy (easier to compute) gives the same geodesics as for length, provided the constraint is imposed. The geodesic equations are $\nabla_{\dot\gamma}\dot\gamma = 0$ (covariant derivative of velocity is zero).

## Variational Characterization of the Poisson Kernel

The Dirichlet problem $\Delta u = 0$ in $B_R$, $u = g$ on $\partial B_R$, has the variational formulation: minimize $\mathcal{E}[u] = \frac{1}{2}\int_{B_R}|\nabla u|^2\,dx$ over $u\in H^1(B_R)$ with $u = g$ on $\partial B_R$. The minimizer is the harmonic function with the given boundary data, and the minimum value is $\mathcal{E}[u^*] = \frac{1}{2}\int_{\partial B_R}g\frac{\partial u^*}{\partial\nu}\,dS$ (by Green's first identity). The Poisson kernel gives $\frac{\partial u^*}{\partial\nu}|_{\partial B_R} = P(\cdot;x)$ (the Poisson kernel as a function of the boundary point, with $x\in B_R$ fixed) — so the minimum energy is $\frac{1}{2}\int_{\partial B_R}g(\mathbf{y})P(\mathbf{x};\mathbf{y})\,dS(\mathbf{y})$.

## Summary

The method of Lagrange multipliers in the calculus of variations is a powerful tool that:
1. **Formulates eigenvalue problems** as constrained minimization of the Rayleigh quotient.
2. **Proves the isoperimetric inequality** by showing the constrained optimum satisfies a constant-curvature condition (circle in 2D, sphere in 3D).
3. **Handles conservation constraints** (fixed mass, charge, etc.) by introducing a multiplier that plays the role of a chemical potential or pressure.
4. **Connects to Lagrangian mechanics**: in classical mechanics, the Lagrange equations are the Euler-Lagrange equations for the action functional $\int(T-V)\,dt$, and constraints (e.g., inextensible strings, rigid bodies) are handled by additional multipliers.
