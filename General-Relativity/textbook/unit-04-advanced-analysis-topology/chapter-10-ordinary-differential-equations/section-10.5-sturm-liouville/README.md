# Section 10.5: Sturm-Liouville Theory

---

## Section Introduction

The **Sturm-Liouville problem** is the study of the eigenvalue problem for a second-order linear ODE of the form $-(py')' + qy = \lambda wy$ on an interval $[a,b]$, with appropriate boundary conditions. Here $p(x) > 0$, $w(x) > 0$ (a weight function), and $q(x)$ are given functions, and $\lambda$ is the eigenvalue to be determined. Legendre's equation, Bessel's equation, the quantum harmonic oscillator, and the Schrödinger equation on a bounded domain are all Sturm-Liouville problems.

The Sturm-Liouville operator $L = -\frac{d}{dx}\left(p\frac{d}{dx}\right) + q$ is **self-adjoint** with respect to the weighted inner product $\langle f,g\rangle = \int_a^b f(x)g(x)w(x)\,dx$: one verifies that $\langle Lf,g\rangle = \langle f,Lg\rangle$ for functions satisfying the boundary conditions. Self-adjointness has immediate consequences: all eigenvalues are real; eigenfunctions corresponding to distinct eigenvalues are orthogonal with respect to the weighted inner product.

The **spectral theorem for Sturm-Liouville operators**: the eigenvalues form a discrete infinite sequence $\lambda_1 < \lambda_2 < \lambda_3 < \cdots\to+\infty$, and the eigenfunctions $\{y_n\}$ form a complete orthonormal set in the Hilbert space $L^2([a,b], w\,dx)$. Every function in this space can be expanded in the eigenfunctions: $f(x) = \sum_n c_n y_n(x)$ with $c_n = \langle y_n,f\rangle$. This is the **generalized Fourier series** — the classical Fourier series, Legendre series, and Bessel series are all special cases.

The connection to GR: the linearized perturbation equations for black holes reduce to Sturm-Liouville problems (or their relativistic generalizations). The **quasinormal modes** of a black hole are the eigenvalues of a Sturm-Liouville-type problem with appropriate boundary conditions at the horizon and at infinity. The spectrum of quasinormal modes is the "ringing" signature of a black hole after a perturbation — it is what LIGO detects in the ringdown phase of a binary merger.

---

## Subsections

- [10.5.1: The Sturm-Liouville Equation](10.5.1-equation.md)
- [10.5.2: Self-Adjointness and Orthogonality](10.5.2-self-adjoint.md)
- [10.5.3: Eigenvalue Spectrum and Completeness](10.5.3-spectrum.md)
- [10.5.4: Generalized Fourier Series](10.5.4-fourier-expansion.md)
- [10.5.5: Classical Examples: Legendre, Bessel, Hermite](10.5.5-examples.md)
