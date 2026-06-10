# Section 11.3: Separation of Variables

---

## Section Introduction

**Separation of variables** is the single most powerful elementary technique for solving PDEs. The idea is to seek solutions of the form $u(\mathbf{x},t) = X(\mathbf{x})T(t)$ — a product of a function of position alone and a function of time alone. Substituting into the PDE and dividing through, one finds that each side of the equation must equal the same constant (the **separation constant**). This converts the PDE into two (or more) ODEs, which can then be solved by the techniques of Chapters 10 and earlier.

For the heat equation $u_t = \alpha u_{xx}$ on $[0,L]$ with $u(0,t)=u(L,t)=0$: substituting $u=X(x)T(t)$ gives $T'/(\alpha T) = X''/X = -\lambda$ (constant). The space part $X'' + \lambda X = 0$ with $X(0) = X(L) = 0$ is a Sturm-Liouville problem (Section 10.5) with eigenvalues $\lambda_n = (n\pi/L)^2$ and eigenfunctions $X_n = \sin(n\pi x/L)$. The time part $T_n(t) = e^{-\alpha\lambda_n t}$. The general solution is the superposition $u = \sum_n c_n\sin(n\pi x/L)e^{-\alpha(n\pi/L)^2 t}$, and the initial condition $u(x,0) = f(x)$ determines the $c_n$ as Fourier sine coefficients.

In three dimensions with spherical symmetry, separation of variables in spherical coordinates produces the **spherical harmonics** $Y_\ell^m(\theta,\phi)$ — the angular parts of solutions to Laplace's equation. The hydrogen atom wave functions are products of spherical harmonics and radial functions (Laguerre polynomials). Spherical harmonics appear throughout physics: in multipole expansions, in quantum angular momentum, and in the decomposition of temperature fluctuations in the cosmic microwave background.

In GR, perturbations of spherically symmetric black holes (Schwarzschild or Reissner-Nordström) are decomposed in spherical harmonics, reducing the PDE to a one-dimensional ODE — the **Regge-Wheeler** or **Zerilli** equation. This is separation of variables in a black hole spacetime.

---

## Subsections

- [11.3.1: The Method of Separation of Variables](11.3.1-method.md)
- [11.3.2: The Heat Equation on a Bounded Domain](11.3.2-heat.md)
- [11.3.3: The Wave Equation and Standing Waves](11.3.3-wave.md)
- [11.3.4: Laplace's Equation in Spherical Coordinates and Spherical Harmonics](11.3.4-spherical.md)
- [11.3.5: Limitations and Extensions](11.3.5-limitations.md)
