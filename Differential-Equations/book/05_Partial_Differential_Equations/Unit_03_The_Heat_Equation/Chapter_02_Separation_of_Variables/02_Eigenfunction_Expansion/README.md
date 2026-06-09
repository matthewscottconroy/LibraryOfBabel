# Eigenfunction Expansion for the Heat Equation

The separation of variables method on a slab with Dirichlet conditions is a special case of a much broader framework: the eigenfunction expansion method, which applies to the heat equation on any bounded domain with any of the three standard boundary condition types. The spatial operator $-\Delta$ (the negative Laplacian) is a self-adjoint operator on the appropriate function space, and its eigenfunctions form a complete orthonormal basis. Expanding the solution in this basis reduces the heat equation to infinitely many decoupled first-order ODEs in time.

## The Abstract Framework

Let $\Omega \subset \mathbb{R}^n$ be a bounded domain and consider the problem:

$$u_t = \kappa\,\Delta u \text{ in } \Omega \times (0,\infty), \qquad \mathcal{B}u = 0 \text{ on } \partial\Omega,$$

where $\mathcal{B}$ represents homogeneous boundary conditions (Dirichlet: $u=0$; Neumann: $\partial u/\partial\nu = 0$; or Robin: $\partial u/\partial\nu + \alpha u = 0$).

The corresponding eigenvalue problem is: find $(\lambda, \phi)$ with $\phi \not\equiv 0$ such that

$$-\Delta\phi = \lambda\phi \text{ in } \Omega, \qquad \mathcal{B}\phi = 0 \text{ on } \partial\Omega.$$

**Theorem (Spectral Theorem for the Laplacian).** Under mild regularity conditions on $\Omega$, the eigenvalue problem above has:

1. A countable, discrete sequence of eigenvalues $0 \leq \lambda_1 \leq \lambda_2 \leq \cdots \to +\infty$.
2. Corresponding eigenfunctions $\phi_1, \phi_2, \ldots$ that are smooth in $\Omega$ and form a complete orthonormal basis for $L^2(\Omega)$: $\langle\phi_m,\phi_n\rangle = \delta_{mn}$.
3. For Dirichlet conditions: $\lambda_1 > 0$ (all eigenvalues strictly positive). For Neumann conditions: $\lambda_1 = 0$ with eigenfunction $\phi_1 = |\Omega|^{-1/2}$ (constant).

## Eigenfunction Expansion Solution

Given initial data $f \in L^2(\Omega)$, expand it in the eigenbasis:

$$f = \sum_{n=1}^\infty c_n\phi_n, \qquad c_n = \langle f, \phi_n\rangle = \int_\Omega f(\mathbf{x})\phi_n(\mathbf{x})\,d\mathbf{x}.$$

The solution to the heat equation is:

$$u(\mathbf{x},t) = \sum_{n=1}^\infty c_n\,e^{-\kappa\lambda_n t}\,\phi_n(\mathbf{x}). \tag{1}$$

**Verification.** For each $t > 0$, the exponential factors decay faster than any power of $n$ (since $\lambda_n \to \infty$), so the series converges absolutely in $L^2$ and defines a smooth function. Differentiating term by term:

$$u_t = \sum_n (-\kappa\lambda_n)c_n e^{-\kappa\lambda_n t}\phi_n, \qquad \kappa\Delta u = \kappa\sum_n c_n e^{-\kappa\lambda_n t}(-\lambda_n\phi_n),$$

and these are equal, confirming that (1) satisfies the PDE. At $t=0$: $u(\mathbf{x},0) = \sum_n c_n\phi_n = f(\mathbf{x})$ in $L^2$.

## Sturm-Liouville Problems in One Dimension

On the interval $[0,L]$, the general Sturm-Liouville eigenvalue problem is:

$$-(p(x)X')' + q(x)X = \lambda w(x)X, \qquad \alpha_0 X(0) + \beta_0 X'(0) = 0, \quad \alpha_L X(L) + \beta_L X'(L) = 0,$$

where $p, w > 0$ and $q \geq 0$ on $[0,L]$.

**Sturm-Liouville Theorem.** The eigenvalues are real, bounded below, and accumulate only at $+\infty$: $\lambda_1 < \lambda_2 < \cdots \to +\infty$. The eigenfunctions $X_n$ are real, can be chosen to satisfy $\int_0^L X_m X_n w\,dx = \delta_{mn}$ (orthonormality with weight $w$), and form a complete orthonormal basis for $L^2((0,L), w\,dx)$.

The three standard boundary conditions give the following eigensystems on $[0,L]$:

| BC | Eigenvalues $\lambda_n$ | Eigenfunctions $X_n(x)$ |
|----|------------------------|------------------------|
| Dirichlet ($X(0)=X(L)=0$) | $(n\pi/L)^2$, $n \geq 1$ | $\sqrt{2/L}\sin(n\pi x/L)$ |
| Neumann ($X'(0)=X'(L)=0$) | $(n\pi/L)^2$, $n \geq 0$ | $\sqrt{1/L}$ ($n=0$), $\sqrt{2/L}\cos(n\pi x/L)$ ($n\geq 1$) |
| Mixed ($X(0)=0$, $X'(L)=0$) | $((2n-1)\pi/(2L))^2$, $n \geq 1$ | $\sqrt{2/L}\sin((2n-1)\pi x/(2L))$ |

## Example: Neumann Conditions

For the heat equation $u_t = \kappa u_{xx}$ on $[0,L]$ with Neumann conditions $u_x(0,t) = u_x(L,t) = 0$ and initial data $f(x)$:

$$u(x,t) = a_0 + \sum_{n=1}^\infty a_n\cos\!\left(\frac{n\pi x}{L}\right)e^{-\kappa(n\pi/L)^2 t},$$

where $a_0 = \frac{1}{L}\int_0^L f(x)\,dx$ and $a_n = \frac{2}{L}\int_0^L f(x)\cos(n\pi x/L)\,dx$.

The $n=0$ mode is the constant $a_0$, which does not decay. Physically, with insulated boundaries, the total heat content $\int_0^L u\,dx = La_0$ is conserved, and the temperature equilibrates to the average value $a_0$. All higher modes decay exponentially to zero.

## Multi-Dimensional Eigenfunction Expansions

On a rectangle $\Omega = (0,a)\times(0,b)$ with Dirichlet conditions, the eigenfunctions separate:

$$\phi_{mn}(x,y) = \frac{2}{\sqrt{ab}}\sin\!\left(\frac{m\pi x}{a}\right)\sin\!\left(\frac{n\pi y}{b}\right), \qquad \lambda_{mn} = \left(\frac{m\pi}{a}\right)^2 + \left(\frac{n\pi}{b}\right)^2.$$

The solution is:

$$u(x,y,t) = \sum_{m=1}^\infty\sum_{n=1}^\infty c_{mn}\,\phi_{mn}(x,y)\,e^{-\kappa\lambda_{mn}t},$$

where $c_{mn} = \langle f, \phi_{mn}\rangle$. The double Fourier sine series is the natural tool for solving the heat equation on a rectangle.

For disks, cylinders, and spheres, the eigenfunctions are products of Bessel functions (or spherical Bessel functions) with trigonometric functions, as developed in Chapter 4.

## Parseval's Theorem and Energy

The Parseval identity for eigenfunction expansions gives:

$$\|u(\cdot,t)\|_{L^2(\Omega)}^2 = \sum_{n=1}^\infty c_n^2\,e^{-2\kappa\lambda_n t}.$$

This is a Pythagorean identity: the $L^2$ energy in the $n$-th mode at time $t$ is $c_n^2 e^{-2\kappa\lambda_n t}$. The total energy $\sum_n c_n^2 e^{-2\kappa\lambda_n t}$ decreases monotonically to zero (for Dirichlet conditions) — the system loses energy through the cold boundary. The rate of energy loss at time $t$ is

$$\frac{d}{dt}\|u\|_{L^2}^2 = -2\kappa\sum_n \lambda_n c_n^2 e^{-2\kappa\lambda_n t} = -2\kappa\|\nabla u(\cdot,t)\|_{L^2}^2 \leq 0,$$

by Parseval's identity and the fact that $\|\nabla\phi_n\|^2 = \lambda_n$ (since $-\Delta\phi_n = \lambda_n\phi_n$ and integration by parts gives $\|\nabla\phi_n\|^2 = \lambda_n$). This is the energy dissipation identity.
