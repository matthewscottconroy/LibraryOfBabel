# Chapter 03: The Hankel Transform

The Fourier transform is naturally adapted to Cartesian coordinates on $\mathbb{R}^n$: its kernel $e^{-i\mathbf{k}\cdot\mathbf{x}}$ decomposes a function on $\mathbb{R}^n$ into plane waves. But many physical and mathematical problems possess cylindrical symmetry: they are described in polar or cylindrical coordinates, and the natural decomposition is not into plane waves but into cylindrical waves characterized by Bessel functions.

The **Hankel transform** is the integral transform that plays the role of the Fourier transform for radially symmetric functions in cylindrical or spherical coordinates. Its kernel is the Bessel function $J_\nu(\rho r)$, and it arises naturally when applying the 2D Fourier transform to functions that depend only on the radial variable $r = \sqrt{x^2 + y^2}$.

## Chapter Overview

**Section 01: Bessel Functions and Cylindrical Problems** introduces the Bessel functions $J_\nu(x)$ as solutions to Bessel's differential equation
$$x^2 y'' + xy' + (x^2 - \nu^2)y = 0,$$
which arises from separation of variables for Laplace's equation, the wave equation, and the heat equation in cylindrical coordinates. The Bessel functions of the first kind $J_\nu$ are the physically relevant solutions (bounded at the origin). Their orthogonality on $(0, \infty)$ with respect to the weight $r$:
$$\int_0^\infty J_\nu(\lambda r)J_\nu(\mu r)\,r\,dr = \frac{1}{\lambda}\delta(\lambda - \mu),$$
motivates the Hankel transform $\mathcal{H}_\nu[f](\rho) = \int_0^\infty f(r)J_\nu(\rho r)r\,dr$ as the expansion of $f$ in the Bessel function basis.

## Connection to 2D Fourier Transform

For a function $f(x,y) = g(r)$ depending only on $r = \sqrt{x^2+y^2}$ (radially symmetric), the 2D Fourier transform evaluated at wavevector $\boldsymbol{\xi}$ with $|\boldsymbol{\xi}| = \rho$ depends only on $\rho$ and equals $2\pi\mathcal{H}_0[g](\rho)$. More generally, the Fourier transform in $n$ dimensions of a radially symmetric function reduces to a Hankel transform of order $(n-2)/2$.

## Applications

The Hankel transform solves PDE problems with cylindrical symmetry in the same way the Fourier transform solves problems on $\mathbb{R}$. For the heat equation $u_t = \alpha^2\Delta u$ on $\mathbb{R}^2$ with radially symmetric initial data $u(r, 0) = f(r)$, taking the Hankel transform in $r$ converts the 2D Laplacian $\Delta$ to multiplication by $-\rho^2$, reducing the PDE to an ODE in $t$ for each $\rho$.
