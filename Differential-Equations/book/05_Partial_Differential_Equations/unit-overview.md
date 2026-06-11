# Unit Overview: Partial Differential Equations

## Why PDEs Are Inevitable

A partial differential equation involves an unknown function of several independent variables together with its partial derivatives. The step from ODEs to PDEs is not merely a generalization; it represents a qualitative shift in complexity, richness, and depth. ODEs describe the evolution of a finite-dimensional state (position and velocity of a particle, charge in a circuit). PDEs describe fields — temperature throughout a solid, the amplitude of a vibrating membrane, the electric potential surrounding a charge distribution — which are infinite-dimensional objects. The solution to an ODE is a function of one variable. The solution to a PDE is a function of several variables, whose behavior in space and in time are coupled by the equation.

The three canonical second-order linear PDEs — the heat equation $u_t = \kappa\Delta u$, the wave equation $u_{tt} = c^2\Delta u$, and Laplace's equation $\Delta u = 0$ — are not merely examples. They are the archetypes of three fundamentally different behaviors (diffusion, propagation, and equilibrium), and the entire classification of second-order linear PDEs reduces to which of these archetypes a given equation resembles. Every technique developed in this unit — separation of variables, method of characteristics, Fourier series, Green's functions, energy methods — was first understood in the context of these three equations and then generalized.

## Classification: Elliptic, Parabolic, Hyperbolic

A second-order linear PDE in two variables has the form $Au_{xx} + Bu_{xy} + Cu_{yy} + Du_x + Eu_y + Fu = G$.

The discriminant $\Delta = B^2 - 4AC$ determines the type:
- $\Delta < 0$: **elliptic** (prototype: Laplace's equation $u_{xx} + u_{yy} = 0$).
- $\Delta = 0$: **parabolic** (prototype: heat equation $u_t = u_{xx}$).
- $\Delta > 0$: **hyperbolic** (prototype: wave equation $u_{tt} = c^2 u_{xx}$).

This classification is not arbitrary; it corresponds to the geometric structure of characteristic lines (curves along which information propagates). Elliptic equations have no real characteristics; hyperbolic equations have two families; parabolic equations have one. The structure of well-posed problems — what initial/boundary data are appropriate, and whether a solution exists, is unique, and depends continuously on data — differs fundamentally between the three types.

## Method of Characteristics

The method of characteristics applies primarily to first-order PDEs and hyperbolic second-order PDEs.

**First-order linear PDE.** Consider $a(x,y)u_x + b(x,y)u_y = c(x,y)u + d(x,y)$.

The characteristic curves are solutions to $dx/ds = a$, $dy/ds = b$. Along a characteristic, $du/ds = cu + d$, which is an ODE. Thus the PDE reduces to a family of ODEs parametrized by the initial curve.

**Transport Equation.** Simplest case: $u_t + cu_x = 0$, $u(x,0) = f(x)$. Characteristics: $x - ct = \text{const}$. Solution: $u(x,t) = f(x-ct)$. The initial profile translates rigidly at speed $c$.

**Nonlinear first-order PDE (quasi-linear).** $u_t + F(u)_x = 0$ (conservation law). Characteristics: $dx/dt = F'(u)$ — the characteristic speed depends on the solution itself. Two characteristics can converge and produce a **shock** (discontinuity in $u$) even from smooth initial data, at time $T_{\text{shock}} = -1/\min_x[F''(u_0)u_0']$ (if this is positive).

**Theorem (Method of Characteristics for $F(x,y,u,u_x,u_y)=0$).** The characteristic curves satisfy the characteristic ODEs (Charpit equations). Along each characteristic, the PDE reduces to an ODE system. Existence and uniqueness of solutions follow from Picard-Lindelöf applied to this system, valid until characteristics cross.

## The Heat Equation

$u_t = \kappa u_{xx}$ on $0 < x < L$, $t > 0$, with Dirichlet boundary conditions $u(0,t) = u(L,t) = 0$ and initial condition $u(x,0) = f(x)$.

**Separation of Variables.** Assume $u(x,t) = X(x)T(t)$. Then $T'/(\kappa T) = X''/X = -\lambda$ (the separation constant must be the same constant on both sides). The Sturm-Liouville problem $X'' + \lambda X = 0$, $X(0) = X(L) = 0$ has eigenvalues $\lambda_n = n^2\pi^2/L^2$ and eigenfunctions $X_n(x) = \sin(n\pi x/L)$ for $n = 1, 2, 3, \ldots$ The time factors are $T_n(t) = e^{-\kappa n^2\pi^2 t/L^2}$.

General solution: $u(x,t) = \sum_{n=1}^\infty b_n\sin\!\left(\frac{n\pi x}{L}\right)e^{-\kappa n^2\pi^2 t/L^2}$.

Coefficients from initial condition: $b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx$ (Fourier sine coefficients).

**Maximum Principle.** If $u_t - \kappa u_{xx} = 0$ on $[0,L]\times[0,T]$, then $u$ attains its maximum on the parabolic boundary (the bottom $t=0$ and sides $x=0$, $x=L$). Consequences: uniqueness (if two solutions agree on the boundary, they agree everywhere), stability (small changes in boundary/initial data produce small changes in the solution), and a priori bounds.

**Heat Kernel and Fundamental Solution.** On $\mathbb{R}$, the fundamental solution (the solution with initial data $\delta(x)$) is the Gaussian $K(x,t) = (4\pi\kappa t)^{-1/2}e^{-x^2/(4\kappa t)}$. The solution with general initial data $f$ is $u(x,t) = \int_{-\infty}^\infty K(x-y,t)f(y)\,dy$.

## The Wave Equation

$u_{tt} = c^2 u_{xx}$ on $\mathbb{R}$, $t > 0$, with $u(x,0) = f(x)$, $u_t(x,0) = g(x)$.

**d'Alembert's Formula.** The general solution is $u(x,t) = F(x-ct) + G(x+ct)$ — two traveling waves moving right and left at speed $c$. With initial conditions:
$$u(x,t) = \frac{f(x-ct)+f(x+ct)}{2} + \frac{1}{2c}\int_{x-ct}^{x+ct} g(s)\,ds.$$

The domain of dependence of the point $(x_0,t_0)$ is the interval $[x_0-ct_0, x_0+ct_0]$: the value of $u$ at $(x_0,t_0)$ depends only on initial data in this interval. The cone $|x-x_0| \leq c(t-t_0)$ is the domain of influence of $x_0$ — the region affected by initial data at $x_0$. This is Huygens' principle in 1D.

**Energy Conservation.** Define $E(t) = \frac{1}{2}\int_{-\infty}^\infty (u_t^2 + c^2 u_x^2)\,dx$ (kinetic plus potential energy). Then $dE/dt = 0$: energy is conserved. This gives uniqueness (if two solutions agree on $t=0$, their difference has zero energy) and stability.

**Huygens' Principle in 3D.** In 3 spatial dimensions, the wave equation $u_{tt} = c^2\Delta u$ has the property that the influence of a point source at the origin propagates exactly on the expanding sphere $|\mathbf{x}| = ct$, not inside it. (In 2D, the solution has support on and inside the expanding circle.) This is the mathematical statement of why one hears a sharp click, not a rumble, from a distant explosion: the wave arrives at a definite time and then passes.

## Laplace's and Poisson's Equations

Laplace's equation $\Delta u = 0$ describes equilibrium states: steady-state heat distribution, electrostatic potential in a charge-free region, velocity potential for irrotational incompressible flow. A solution is called **harmonic**.

**Mean Value Property.** $u$ is harmonic on a domain $\Omega$ if and only if for every ball $B_r(\mathbf{x}) \subset\subset \Omega$:
$$u(\mathbf{x}) = \frac{1}{|S_r|}\int_{S_r(\mathbf{x})} u\,dS = \frac{1}{|B_r|}\int_{B_r(\mathbf{x})} u\,dV.$$
The value of a harmonic function at any point equals its average over any sphere (or ball) centered there.

**Maximum Principle.** A nonconstant harmonic function on a bounded domain $\Omega$ attains its maximum and minimum only on the boundary $\partial\Omega$. Corollaries: uniqueness for the Dirichlet problem (specify $u$ on $\partial\Omega$), uniqueness for the Neumann problem (specify $\partial u/\partial n$ on $\partial\Omega$, up to a constant).

**Dirichlet Problem on a Disk.** For $\Delta u = 0$ in the disk $|\mathbf{x}| < R$ with $u = f$ on $|\mathbf{x}| = R$, the solution is given by the Poisson integral formula:
$$u(r,\theta) = \frac{1}{2\pi}\int_0^{2\pi} \frac{R^2 - r^2}{R^2 - 2Rr\cos(\theta-\phi) + r^2} f(\phi)\,d\phi.$$

The Poisson kernel $P(r,\theta;\phi) = (R^2-r^2)/(R^2 - 2Rr\cos(\theta-\phi)+r^2)$ is positive, integrates to $2\pi$, and concentrates near $\phi = \theta$ as $r \to R$: it is an approximate identity (like the heat kernel as $t\to 0$).

**Green's Functions.** For Poisson's equation $-\Delta u = f$ on $\Omega$ with $u = 0$ on $\partial\Omega$, the solution is $u(\mathbf{x}) = \int_\Omega G(\mathbf{x},\mathbf{y})f(\mathbf{y})\,d\mathbf{y}$ where $G$ is Green's function for $\Omega$. On $\mathbb{R}^n$: $G(\mathbf{x},\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y})$ where $\Phi$ is the fundamental solution ($\Phi(x) = -\ln|x|/(2\pi)$ in 2D, $\Phi(\mathbf{x}) = 1/(4\pi|\mathbf{x}|)$ in 3D). On a bounded domain, Green's function is $G(\mathbf{x},\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - h(\mathbf{x},\mathbf{y})$ where $h$ corrects for the boundary condition.

## Worked Examples

### Example 1: Shock Formation in a Conservation Law

Consider Burgers' equation: $u_t + uu_x = 0$, $u(x,0) = f(x)$.

Characteristics: $dx/dt = u$ (the characteristic speed equals the solution value, which is constant along each characteristic). So characteristics are straight lines: $x = x_0 + f(x_0)t$.

If $f$ is decreasing somewhere (say $f'(x_0) < 0$), then characteristics starting to the left of $x_0$ move faster than those to its right, and they will cross at time $t_c = -1/\min f'$. At crossing, the solution becomes multivalued — a shock forms.

For $f(x) = 1/(1+x^2)$ (a bump): $f'(x) = -2x/(1+x^2)^2$, minimized at $x = 1/\sqrt{3}$ giving $f'_{\min} \approx -0.65$. Shock forms at $t_c \approx 1.54$.

### Example 2: Heat Equation with Source

Solve $u_t = u_{xx} + \sin(\pi x)$, $u(0,t) = u(1,t) = 0$, $u(x,0) = 0$.

Separation suggests expanding in eigenfunctions $\sin(n\pi x)$. Let $u = \sum_n T_n(t)\sin(n\pi x)$.

The source $\sin(\pi x) = 1\cdot\sin(\pi x)$ (only $n=1$ term). The $n=1$ equation is $T_1' + \pi^2 T_1 = 1$, $T_1(0) = 0$. Solving: $T_1 = (1-e^{-\pi^2 t})/\pi^2$.

For $n \geq 2$: $T_n' + n^2\pi^2 T_n = 0$, $T_n(0) = 0$, so $T_n = 0$.

Solution: $u(x,t) = \frac{1-e^{-\pi^2 t}}{\pi^2}\sin(\pi x)$.

As $t \to \infty$: $u \to \sin(\pi x)/\pi^2$, which is the steady-state solution of $u_{xx} + \sin(\pi x) = 0$ with zero boundary conditions.

### Example 3: d'Alembert on a Semi-Infinite String

String $x > 0$, $t > 0$, with $u(0,t) = 0$ (fixed end), $u(x,0) = f(x)$ for $x > 0$, $u_t(x,0) = 0$.

d'Alembert gives $u = [f(x-ct) + f(x+ct)]/2$ on $\mathbb{R}$, but we need the boundary condition $u(0,t) = 0$.

Extend $f$ to an odd function: $f_{\text{odd}}(x) = -f(-x)$ for $x < 0$. Then the d'Alembert formula with $f_{\text{odd}}$ automatically satisfies $u(0,t) = 0$.

Solution: $u(x,t) = [f_{\text{odd}}(x-ct) + f_{\text{odd}}(x+ct)]/2$, which for $x > ct$ gives $[f(x-ct)+f(x+ct)]/2$ (the incident wave plus a reflected wave traveling in the opposite direction with sign flip).

## Historical Notes

**Jean le Rond d'Alembert (1717–1783)** derived the one-dimensional wave equation in 1747 and immediately gave the general solution $u = F(x-ct) + G(x+ct)$. His paper sparked a controversy with Euler about what "arbitrary function" meant — a dispute that, in retrospect, was one of the first engagements with the deep question of what a function is.

**Leonhard Euler (1707–1783)** independently derived the wave equation and studied its solutions, often reaching different conclusions from d'Alembert due to differing implicit assumptions about regularity.

**Joseph Fourier (1768–1830)** derived the heat equation in *Théorie analytique de la chaleur* (1822) and solved it by separation of variables. His solution method — express the initial condition as a Fourier series; multiply each term by the appropriate decaying exponential — is still the standard approach and is presented verbatim in this unit.

**Siméon Denis Poisson (1781–1840)** studied Laplace's equation and derived the Poisson integral formula for the disk around 1820. He also introduced Poisson's equation $\Delta u = -f$, which models the electrostatic potential due to a charge density $f$.

**George Green (1793–1841)** introduced Green's functions in his 1828 essay on electricity and magnetism. The function $G(\mathbf{x},\mathbf{y})$ bearing his name encodes the response of the system to a point source and provides the fundamental solution formula for Poisson's equation on domains with boundary.

**Bernhard Riemann (1826–1866)** proved the Riemann mapping theorem and studied hyperbolic PDEs, developing the Riemann function as the analogue of Green's function for hyperbolic equations.

**Hermann von Helmholtz (1821–1894)** studied the wave equation for time-periodic solutions, leading to the Helmholtz equation $\Delta u + k^2 u = 0$, which governs acoustic and electromagnetic waves in the frequency domain.

**Sergei Sobolev (1908–1989)** introduced the Sobolev spaces $W^{k,p}$ in the 1930s and 40s, providing the function space framework in which the existence and uniqueness theory of PDEs is properly formulated. His work made possible the variational (weak) approach to PDEs that is now standard in both theory and computation.

## Connections to Other Units

**Prerequisites:**
- Unit 03 (ODEs): separation of variables converts PDEs into ODEs; Sturm-Liouville theory from Unit 03 provides the spectral theory of the resulting ODE eigenvalue problems.
- Unit 04 (Fourier Analysis): Fourier series and transforms are the main tools for solving the heat, wave, and Laplace equations on standard domains.
- Unit 02 (Vector Calculus): the derivation of all three canonical PDEs uses the Divergence Theorem; Green's identities are vector calculus applied to harmonic functions.

**Downstream:**
- Unit 06 (Complex Analysis): harmonic functions are the real parts of analytic functions; the Cauchy-Riemann equations connect complex differentiability to Laplace's equation. The Poisson kernel for the disk is derived most elegantly via the Cauchy integral formula.
- Unit 07 (Dynamical Systems): reaction-diffusion PDEs combine the diffusive structure of the heat equation with nonlinear dynamics; Turing's analysis of pattern formation uses linearization of a nonlinear PDE around a homogeneous state.
- Unit 08 (Advanced Topics): Sobolev spaces provide the rigorous framework for weak solutions; distribution theory extends the notion of solution to non-smooth data; the Lax-Milgram theorem gives existence and uniqueness for abstract variational problems.

## Key Theorems at a Glance

1. **Classification (Elliptic/Parabolic/Hyperbolic):** Determined by sign of $B^2 - 4AC$ for second-order linear PDE; governs well-posedness and appropriate boundary/initial conditions.
2. **Method of Characteristics:** First-order PDE reduces to ODEs along characteristic curves; information propagates along characteristics; shocks form when characteristics cross.
3. **Maximum Principle (Heat Equation):** Solution attains max/min on parabolic boundary; implies uniqueness and stability.
4. **Separation of Variables:** Converts PDE to Sturm-Liouville eigenvalue problem; solution is eigenfunction expansion with time-decaying coefficients.
5. **d'Alembert's Formula:** $u(x,t) = [f(x-ct)+f(x+ct)]/2 + (2c)^{-1}\int_{x-ct}^{x+ct}g$ for the 1D wave equation.
6. **Maximum Principle (Laplace's Equation):** Harmonic functions on a bounded domain attain max/min on boundary only; implies uniqueness for Dirichlet problem.
7. **Mean Value Property:** $u$ harmonic iff equal to its average over every sphere (ball) centered in the domain.
8. **Poisson Integral Formula:** Explicit solution for the Dirichlet problem on a disk.
9. **Fundamental Solutions:** $\Phi(x) = -\ln|x|/(2\pi)$ in 2D, $\Phi(\mathbf{x}) = 1/(4\pi|\mathbf{x}|)$ in 3D; heat kernel $K(x,t) = (4\pi\kappa t)^{-1/2}e^{-x^2/(4\kappa t)}$.
10. **Energy Methods:** Multiplying by $u_t$ and integrating by parts gives conservation or decay of energy; used for uniqueness and stability without explicit solutions.
