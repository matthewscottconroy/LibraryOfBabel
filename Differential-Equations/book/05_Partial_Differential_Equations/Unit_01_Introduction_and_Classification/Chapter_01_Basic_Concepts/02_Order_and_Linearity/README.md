# Order and Linearity of Partial Differential Equations

Two of the most important structural attributes of a PDE are its order and its degree of linearity. These are not merely classification labels — they determine which solution techniques are available, what analytical tools apply, and what qualitative behavior is possible. A second-order linear PDE and a first-order nonlinear PDE inhabit different mathematical worlds, and treating them with the same methods leads to failure.

## Order

The **order** of a PDE is the order of the highest partial derivative appearing in the equation. For example:

- $u_x + u_y = 0$ is first-order.
- $u_t = u_{xx}$ (heat equation) is second-order.
- $u_{tt} - u_{xx} = 0$ (wave equation) is second-order.
- $u_{xxxx} + u_{yyyy} = 0$ (biharmonic in 2D) is fourth-order.

Most equations arising in classical physics are first- or second-order. The biharmonic equation and the Euler-Bernoulli beam equation $u_{tt} + u_{xxxx} = 0$ are common fourth-order examples.

Second-order equations receive special attention because the physical laws of mechanics, electrodynamics, and thermodynamics are typically second-order in spatial derivatives (via the divergence theorem applied to flux laws) and at most second-order in time. The classification theory — elliptic, parabolic, hyperbolic — applies directly to second-order equations.

## The Hierarchy of Linearity

The most fundamental structural distinction in PDE theory is between linear and nonlinear equations. Within these broad categories there are important refinements.

**Linear PDEs.** A PDE of order $k$ is linear if it can be written as

$$L[u] = f,$$

where $L$ is a linear differential operator: $L[\alpha u + \beta v] = \alpha L[u] + \beta L[v]$ for all functions $u, v$ and constants $\alpha, \beta$. The coefficients of the derivatives may depend on the independent variables but not on $u$ or its derivatives. If $f = 0$, the equation is **homogeneous**; otherwise it is **nonhomogeneous** (or **inhomogeneous**).

The heat equation $u_t - k u_{xx} = 0$ is linear and homogeneous. The equation $u_t - k u_{xx} = f(x,t)$ is linear and nonhomogeneous.

The **superposition principle** holds for linear homogeneous equations: if $u_1$ and $u_2$ are solutions, so is $c_1 u_1 + c_2 u_2$ for any constants $c_1, c_2$. This is the engine behind Fourier's method: one constructs a complete family of simple solutions (modes) and forms infinite series from them to match arbitrary initial or boundary data.

**Semilinear PDEs.** An equation is semilinear if the highest-order terms are linear (with coefficients depending only on the independent variables), but lower-order terms may be nonlinear. The reaction-diffusion equation

$$u_t - \Delta u = f(u)$$

is semilinear: the principal part $u_t - \Delta u$ is the linear heat operator, but the right side $f(u)$ may be any smooth function of $u$. Fisher's equation $u_t = D u_{xx} + ru(1-u)$ is semilinear.

**Quasilinear PDEs.** An equation is quasilinear if the highest-order derivatives appear linearly, but their coefficients may depend on lower-order derivatives of $u$. The minimal surface equation,

$$\left(1 + u_y^2\right)u_{xx} - 2u_x u_y u_{xy} + \left(1 + u_x^2\right)u_{yy} = 0,$$

is quasilinear. Burgers' equation $u_t + u u_x = 0$ (inviscid) is also quasilinear — the coefficient of $u_x$ is $u$ itself. Many of the most important PDEs in fluid dynamics are quasilinear.

**Fully nonlinear PDEs.** An equation is fully nonlinear if it is genuinely nonlinear in the highest-order derivatives. The eikonal equation $(u_x)^2 + (u_y)^2 = 1$ and the Monge-Ampère equation $u_{xx}u_{yy} - u_{xy}^2 = f$ are fully nonlinear. Such equations require the most sophisticated analytical tools, including viscosity solution theory.

## Why Linearity Matters So Much

For linear equations, the superposition principle transforms the PDE into a problem in functional analysis: find a basis of the solution space, and express arbitrary data in terms of that basis. Fourier series, Fourier transforms, Laplace transforms, and eigenfunction expansions are all implementations of this strategy.

For nonlinear equations, superposition fails entirely. Solutions can interact, form singularities, and exhibit qualitative behaviors — shock formation, pattern formation, solitons — that have no linear analogue. The study of nonlinear PDEs requires different tools: energy methods, comparison principles, phase-plane analysis for traveling waves, and the deep machinery of compensated compactness and viscosity solutions.

## The Principle of Superposition in Practice

For a linear homogeneous PDE $L[u] = 0$, if $\{u_n\}$ is a sequence of solutions, then under appropriate convergence conditions,

$$u = \sum_{n=1}^\infty c_n u_n$$

is also a solution. The key technical issue is the convergence of the series and the permissibility of passing $L$ through the sum. For the heat equation on $[0,L]$ with Dirichlet boundary conditions, the eigenfunctions are $\sin(n\pi x/L)$ and the corresponding solutions are $e^{-k(n\pi/L)^2 t}\sin(n\pi x/L)$. The Fourier series

$$u(x,t) = \sum_{n=1}^\infty b_n e^{-k(n\pi/L)^2 t}\sin\!\left(\frac{n\pi x}{L}\right)$$

converges absolutely and uniformly for $t > 0$ (the exponential damping beats any polynomial growth in the Fourier coefficients), and the coefficients $b_n$ are determined by the initial condition:

$$u(x,0) = f(x) = \sum_{n=1}^\infty b_n \sin\!\left(\frac{n\pi x}{L}\right), \qquad b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

This procedure — separate variables, find eigenfunctions, expand in series — works precisely because the equation is linear and homogeneous. Every step depends on superposition.

## Homogeneous vs. Nonhomogeneous: A Practical Note

For a nonhomogeneous linear PDE $L[u] = f$, the general solution has the form

$$u = u_p + u_h,$$

where $u_p$ is any particular solution of $L[u_p] = f$ and $u_h$ is the general solution of the homogeneous equation $L[u_h] = 0$. This decomposition is identical to the familiar one from ODEs and is used systematically throughout the theory — for example, when reducing nonhomogeneous boundary conditions to homogeneous ones by subtracting a steady-state solution.
