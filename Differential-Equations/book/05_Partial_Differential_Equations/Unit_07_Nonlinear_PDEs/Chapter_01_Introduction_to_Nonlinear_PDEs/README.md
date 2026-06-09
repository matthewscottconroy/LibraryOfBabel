# Chapter 1: Introduction to Nonlinear PDEs

The boundary between linear and nonlinear PDEs is not merely a technical distinction but a fundamental qualitative divide. Linear PDEs have a solution theory built on superposition, Fourier analysis, spectral theory, and Green's functions — tools developed over two centuries. Nonlinear PDEs require a different toolkit: energy methods, maximum principles adapted to nonlinear settings, phase plane analysis, geometric methods, and entirely new solution concepts (weak solutions, viscosity solutions, renormalization group ideas). This chapter surveys the landscape of nonlinear PDEs, classifying them by the type of nonlinearity, describing the new phenomena that arise, and introducing the analytical framework.

## Classification by Nonlinearity

Recall that a second-order PDE in two variables is:

$$F(x,y,u,u_x,u_y,u_{xx},u_{xy},u_{yy}) = 0.$$

The hierarchy of nonlinearity (from least to most):

**Semilinear:** The highest-order terms are linear with variable (but $u$-independent) coefficients; only lower-order terms are nonlinear:

$$\sum_{|\alpha|=2}a_\alpha(x)\partial^\alpha u + f(x,u,\nabla u) = 0.$$

Examples: $-\Delta u = u^p$ (nonlinear eigenvalue problem / Lane-Emden equation), $u_t = \Delta u + u(1-u)$ (Fisher's equation), nonlinear Schrödinger equation.

**Quasilinear:** The highest-order coefficients depend on $u$ and lower-order derivatives:

$$\sum_{|\alpha|=2}a_\alpha(x,u,\nabla u)\partial^\alpha u + f(x,u,\nabla u) = 0.$$

Examples: $\Delta_p u = \text{div}(|\nabla u|^{p-2}\nabla u)$ ($p$-Laplacian), minimal surface equation $\text{div}(\nabla u/\sqrt{1+|\nabla u|^2}) = 0$, quasi-geostrophic equation.

**Fully nonlinear:** Even the highest-order derivatives appear nonlinearly:

$$F(x,u,\nabla u, D^2 u) = 0.$$

Examples: Monge-Ampere equation $\det(D^2 u) = f(x)$ (optimal transport, differential geometry), Hamilton-Jacobi equations $u_t + H(x,\nabla u) = 0$ (control theory, optics), Bellman equation.

## New Phenomena in Nonlinear PDEs

**Finite-time blow-up.** The semilinear heat equation $u_t = \Delta u + u^p$ for $p > 1$ can develop singularities in finite time even from smooth, compactly supported initial data. The critical power $p_c = 1 + 2/n$ (Fujita exponent) separates global existence (all data, $p > p_c$) from blow-up for all nontrivial data ($1 < p \leq p_c$). The blow-up profile is $u(x,t) \sim C(T-t)^{-1/(p-1)}$ as $t\to T^-$.

**Multiple solutions.** The nonlinear elliptic equation $-\Delta u = u^3$ on a bounded domain may have multiple weak solutions for appropriate boundary data. Bifurcation theory (bifurcation from eigenvalues of the linearized operator) is the primary tool for counting and classifying them.

**Shock formation.** Smooth initial data for the inviscid Burgers equation $u_t + uu_x = 0$ generically develops discontinuities in finite time. The solution must then be interpreted as a weak solution in $L^\infty$, and uniqueness is restored only by adding an entropy condition.

**Solitons and integrable systems.** The KdV equation has exact traveling wave solutions that are perfectly stable under collisions — a nonlinear superposition principle that depends on the integrable structure of the equation. This is not generic: most nonlinear PDEs do not have this property.

**Turing instability and pattern formation.** A spatially uniform equilibrium of a reaction-diffusion system can be stable to spatially uniform perturbations but unstable to spatially varying perturbations of specific wavelength. This is the Turing mechanism for pattern formation.

## The Role of Characteristics

For first-order and hyperbolic equations, characteristics determine the qualitative behavior even in the nonlinear setting:
- For the quasilinear conservation law $u_t + f(u)_x = 0$, characteristics are lines $dx/dt = f'(u)$ along which $u$ is constant — but when $f''(u) \neq 0$, characteristics from different initial points can cross (shock formation).
- For fully nonlinear Hamilton-Jacobi equations, the Charpit equations provide characteristics (bicharacteristics), but they can also break down at corners.

**The role of the sign of nonlinearity.** For the semilinear equation $u_t = \Delta u + f(u)$: if $f$ is "defocusing" ($f'(u) < 0$ for large $u$), the nonlinearity helps control the solution and global existence is more likely. If $f$ is "focusing" ($f'(u) > 0$ for large $u$), the nonlinearity amplifies large values and blow-up is possible.

## Energy Methods for Nonlinear Equations

The energy method — multiplying by $u$ and integrating — extends to many nonlinear settings.

**Example: Fisher's equation.** For $u_t = Du_{xx} + ru(1-u)$ on $[0,L]$ with Dirichlet boundary conditions:

$$\frac{d}{dt}\int_0^L u^2\,dx = -2D\int_0^L (u_x)^2\,dx + 2r\int_0^L u^2(1-u)\,dx.$$

If $0 \leq u \leq 1$ (which holds if $0 \leq u_0 \leq 1$, by the maximum principle), then $u^2(1-u) \geq 0$ and the reaction term is non-negative. The energy may grow (unlike the pure heat equation). However, the $L^\infty$ maximum principle bounds $u \in [0,1]$ uniformly, preventing blow-up.

**Lyapunov functionals.** Many reaction-diffusion equations have a free energy (Lyapunov functional) $\mathcal{E}[u] = \int[\frac{D}{2}|\nabla u|^2 - F(u)]\,dx$ (where $F'(u) = f(u)$) that decreases along solutions: $\frac{d}{dt}\mathcal{E}[u] = -\int|u_t|^2\,dx \leq 0$. This gradient flow structure is the key to proving existence and long-time convergence to equilibria.

## Viscosity Solutions and Hamilton-Jacobi Equations

For the Hamilton-Jacobi equation $u_t + H(x,\nabla u) = 0$, classical solutions may not exist for all time (even for smooth $H$ and $u_0$) because $\nabla u$ can become discontinuous. The correct generalized solution concept is the **viscosity solution**, introduced by Crandall and Lions (1983):

$u$ is a viscosity solution if:
- (Supersolution) For each smooth $\phi$ with $u-\phi$ having a local minimum at $(x_0,t_0)$: $\phi_t(x_0,t_0) + H(x_0,\nabla\phi(x_0,t_0)) \geq 0$.
- (Subsolution) For each smooth $\phi$ with $u-\phi$ having a local maximum at $(x_0,t_0)$: $\phi_t(x_0,t_0) + H(x_0,\nabla\phi(x_0,t_0)) \leq 0$.

Viscosity solutions are unique for a large class of Hamiltonians and are stable under uniform limits, making them the canonical solution concept for fully nonlinear first-order PDEs. The link to optimal control: $u(x,t) = \inf_\gamma\int_0^t L(\gamma,\dot\gamma)\,dt$ (the value function) is the viscosity solution of the Hamilton-Jacobi equation with Hamiltonian $H$ dual to the Lagrangian $L$.

## Structure of This Chapter

**Section 1: Classification and Challenges** elaborates on the semilinear/quasilinear/fully nonlinear hierarchy with detailed examples and a comparison table of the mathematical difficulties at each level.

**Section 2: Semilinear, Quasilinear, and Fully Nonlinear Equations** works through representative examples of each type: the nonlinear Poisson equation, the $p$-Laplacian, the Monge-Ampere equation, and their physical origins in fluid mechanics, differential geometry, and optimal transport.
