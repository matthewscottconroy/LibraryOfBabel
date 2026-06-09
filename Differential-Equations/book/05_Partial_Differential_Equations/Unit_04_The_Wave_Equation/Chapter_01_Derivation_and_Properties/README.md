# Chapter 1: Derivation and Properties of the Wave Equation

The wave equation is derived from Newton's second law applied to a continuous medium. The most concrete derivation — the vibrating string — not only produces the equation but also identifies the wave speed $c = \sqrt{T/\rho}$ in terms of physical parameters (tension $T$ and linear mass density $\rho$), provides boundary conditions from the physics, and motivates the initial conditions (displacement and velocity at $t=0$).

## Structure of the Chapter

**Section 1: Vibrating String Derivation** applies Newton's second law to an infinitesimal element of a taut string. The transverse displacement $u(x,t)$ satisfies $u_{tt} = c^2 u_{xx}$ under the assumptions of small deflection (so $\sin\theta \approx \tan\theta = u_x$), constant tension $T$, and no damping. The wave speed is $c = \sqrt{T/\rho}$.

**Section 2: d'Alembert's Solution** presents the complete solution to the Cauchy problem on $\mathbb{R}$ via the characteristic coordinates $\xi = x+ct$, $\eta = x-ct$. The general solution $u = f(x+ct) + g(x-ct)$ is a superposition of a right-traveling wave $g$ and a left-traveling wave $f$, each propagating at speed $c$ without changing shape. The formula

$$u(x,t) = \frac{\phi(x+ct) + \phi(x-ct)}{2} + \frac{1}{2c}\int_{x-ct}^{x+ct}\psi(s)\,ds$$

gives the unique solution for initial data $u(x,0) = \phi(x)$ and $u_t(x,0) = \psi(x)$.

**Section 3: Domain of Dependence and Influence** analyzes the causal structure of the wave equation. The solution at $(x_0, t_0)$ depends only on initial data in the interval $[x_0 - ct_0, x_0 + ct_0]$ — the domain of dependence. Conversely, a perturbation of initial data at $x_0$ influences only points $(x,t)$ with $|x-x_0| \leq ct$ — the domain of influence. This finite propagation speed distinguishes the wave equation from the heat equation (infinite propagation speed) and has profound implications for numerical methods (the CFL condition, Chapter 9) and physics (relativistic causality).

## Key Theorems Previewed

**Well-posedness of the Cauchy problem.** For $\phi \in C^2(\mathbb{R})$ and $\psi \in C^1(\mathbb{R})$, d'Alembert's formula gives the unique classical solution. The solution depends continuously on the data: $\|u(\cdot,t)\|_{L^\infty} \leq \|\phi\|_{L^\infty} + t\|\psi\|_{L^\infty}$.

**Energy conservation.** The energy $E(t) = \frac{1}{2}\int[(u_t)^2 + c^2(u_x)^2]\,dx$ is conserved: $dE/dt = 0$. This is the mechanical energy (kinetic + potential) of the string, and its conservation implies uniqueness and continuous dependence.

**Finite propagation speed.** This is the most distinctive property of hyperbolic equations and will be developed rigorously via the theory of characteristics.
