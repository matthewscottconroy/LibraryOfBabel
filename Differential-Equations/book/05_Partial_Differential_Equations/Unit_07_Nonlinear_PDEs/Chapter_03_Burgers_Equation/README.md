# Chapter 3: Burgers' Equation

Burgers' equation $u_t + uu_x = \varepsilon u_{xx}$ is the simplest PDE that combines nonlinear wave propagation (the $uu_x$ term) with diffusion (the $\varepsilon u_{xx}$ term). Introduced by Johannes Martinus Burgers (1948) as a model for turbulence, it is now understood as the canonical nonlinear diffusion equation — a prototype for the Navier-Stokes equations in one spatial dimension, and the simplest PDE where shock formation, shock structure, and the vanishing viscosity limit can all be analyzed explicitly.

## The Two Regimes

**Inviscid Burgers equation ($\varepsilon = 0$):** $u_t + uu_x = 0$.

This is a quasilinear first-order conservation law. Along characteristics $dx/dt = u$, the solution is constant. But characteristics emanating from different initial points can intersect in finite time (when $u_0' < 0$ somewhere), after which the smooth solution no longer exists. Continuation as a **weak solution** requires choosing an appropriate jump condition (Rankine-Hugoniot) and an entropy condition. This was studied extensively in Unit 2.

**Viscous Burgers equation ($\varepsilon > 0$):** $u_t + uu_x = \varepsilon u_{xx}$.

The addition of the viscous term $\varepsilon u_{xx}$ (from physical viscosity in fluids) regularizes the solution: shocks become smooth transition layers of width $\sim\varepsilon$. Remarkably, the viscous Burgers equation is **exactly solvable** via the Hopf-Cole transformation, reducing it to the linear heat equation.

## Physical Origins

**Gas dynamics.** In 1D gas dynamics, the momentum equation (Navier-Stokes in 1D) for velocity $u(x,t)$ in a pressureless gas is exactly Burgers' equation. The convective term $uu_x$ is nonlinear advection (the gas carries itself); the $\varepsilon u_{xx}$ term is viscous momentum diffusion.

**Traffic flow.** A fundamental diagram model for traffic density $\rho$ with flux $F(\rho) = \rho v(\rho)$ gives $\rho_t + F(\rho)_x = 0$. For the Greenshields model $v = v_{\max}(1-\rho/\rho_{\max})$: $F = v_{\max}\rho(1-\rho/\rho_{\max})$. The change of variables $u = F'(\rho) = v_{\max}(1-2\rho/\rho_{\max})$ converts this (locally) to a form close to Burgers' equation.

**Random matrices and KPZ.** Burgers' equation is the Cole-Hopf dual of the KPZ (Kardar-Parisi-Zhang) equation $h_t = \nu h_{xx} + \frac{\lambda}{2}h_x^2 + \eta$ (stochastic PDE for interface growth), which arises in polymer growth, directed random matrices, and the Tracy-Widom distribution from random matrix theory.

## Structure of This Chapter

**Section 1: Hopf-Cole Transformation** derives the exact linearization of Burgers' equation. The substitution $u = -2\varepsilon(\log\phi)_x = -2\varepsilon\phi_x/\phi$ transforms the viscous Burgers equation into the heat equation $\phi_t = \varepsilon\phi_{xx}$. With the heat kernel available, the exact solution $\phi$ can be written down explicitly, giving a complete formula for $u$ in terms of the initial data. This allows:
- Explicit solutions for Riemann initial data (the inviscid shock problem).
- Analysis of the shock layer structure: the shock has width $\sim\varepsilon$ and connects two constant states with a $\tanh$-like profile.
- Rigorous proof of the vanishing viscosity limit $\varepsilon\to 0^+$ (the viscous solution converges to the entropy solution of the inviscid equation).

**Section 2: Shock Formation** revisits the inviscid Burgers equation from the perspective of the Hopf-Cole formula. As $\varepsilon\to 0^+$, the solution formula concentrates near the steepest descent of a related functional. The resulting large-deviation/saddle-point analysis recovers the inviscid entropy solution directly from the viscous regularization. The shock position is determined by the Maxwell area rule (equal areas principle), and the shock speed satisfies the Rankine-Hugoniot condition.

## Key Results

**Theorem (Hopf-Cole linearization).** If $\phi$ solves the heat equation $\phi_t = \varepsilon\phi_{xx}$ with $\phi > 0$, then $u = -2\varepsilon(\log\phi)_x$ solves Burgers' equation $u_t + uu_x = \varepsilon u_{xx}$.

**Theorem (global existence).** For any $u_0 \in L^\infty(\mathbb{R})$, the viscous Burgers equation has a unique smooth solution $u \in C^\infty(\mathbb{R}\times(0,\infty))$ with $\|u(\cdot,t)\|_{L^\infty} \leq \|u_0\|_{L^\infty}$.

**Theorem (vanishing viscosity).** As $\varepsilon\to 0^+$, the viscous Burgers solution $u^\varepsilon$ converges (locally uniformly away from shocks, in $L^1_{\text{loc}}$ everywhere) to the unique entropy solution of the inviscid Burgers equation.

These theorems make Burgers' equation the most completely understood nonlinear PDE for conservation laws, and it serves as a testing ground for numerical methods (upwind schemes, Godunov's method, entropy fixes) that are then applied to more complex systems (Euler equations, Navier-Stokes).
