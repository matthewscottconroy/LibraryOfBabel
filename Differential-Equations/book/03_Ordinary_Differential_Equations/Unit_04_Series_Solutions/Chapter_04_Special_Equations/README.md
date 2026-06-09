# Chapter 4: Special Equations and Their Polynomials

Several second-order linear ODEs arising naturally in mathematical physics and applied mathematics have polynomial solutions for special values of their parameters. These polynomial solutions, the Legendre polynomials, Hermite polynomials, Laguerre polynomials, and Chebyshev polynomials, form orthogonal families with rich algebraic and analytic properties. They serve as basis functions for Fourier-type expansions in the appropriate weighted inner product spaces.

## Overview of the Special Equations

**Legendre's equation** $(1-x^2)y'' - 2xy' + n(n+1)y = 0$ arises from the Laplace equation in spherical coordinates. For non-negative integer $n$, one solution is the Legendre polynomial $P_n(x)$; the other is the Legendre function of the second kind $Q_n(x)$, which is singular at $x = \pm 1$.

**Bessel's equation** $x^2 y'' + xy' + (x^2 - \nu^2)y = 0$ arises from the Laplace or Helmholtz equation in cylindrical coordinates. Its solutions are the Bessel functions $J_\nu(x)$ and $Y_\nu(x)$.

**Hermite's equation** $y'' - 2xy' + 2ny = 0$ arises as the radial equation in quantum harmonic oscillator theory. For non-negative integers $n$, one solution is the Hermite polynomial $H_n(x)$.

**Laguerre's equation** $xy'' + (1-x)y' + ny = 0$ arises in the hydrogen atom's radial Schrodinger equation. For non-negative integers $n$, one solution is the Laguerre polynomial $L_n(x)$.

**The hypergeometric equation** is a unifying framework containing most of the above as special cases.

**Chebyshev polynomials** arise from the equation $(1-x^2)y'' - xy' + n^2 y = 0$ and are fundamental in approximation theory and numerical analysis.

## Common Themes

All these polynomial families are orthogonal on appropriate intervals with appropriate weight functions. This orthogonality is a consequence of the Sturm-Liouville structure of the underlying equations (Unit 8). The polynomial solutions terminate because the recurrence relation for the series solution breaks off at a finite term, which happens precisely for the integer or half-integer values of the parameter.
