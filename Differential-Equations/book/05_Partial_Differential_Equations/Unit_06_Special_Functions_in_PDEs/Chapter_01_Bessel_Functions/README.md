# Chapter 1: Bessel Functions

Bessel functions are the eigenfunctions of the radial part of the Laplacian in cylindrical coordinates. Whenever a PDE is separated in cylindrical coordinates $(r,\theta,z)$, the radial factor satisfies Bessel's equation, and the boundary conditions on a cylinder or disk force the solution to be a Bessel function evaluated at a specific zero of $J_\nu$ or its derivative. This chapter develops the complete theory: the power series definition via Frobenius, the second solution $Y_\nu$, the zeros, orthogonality, Bessel-Fourier series, and the modified Bessel functions $I_\nu$ and $K_\nu$ that appear in problems with real (non-oscillatory) radial dependence.

## Bessel's Equation

When $\Delta u = 0$ or $\Delta u = \pm\lambda^2 u$ is separated in cylindrical coordinates with $u = R(r)\Theta(\theta)Z(z)$, the azimuthal factor $\Theta = e^{\pm in\theta}$ requires $n \in \mathbb{Z}$ for periodicity, and the radial factor satisfies **Bessel's equation of order $\nu = n$**:

$$r^2 R'' + rR' + (\mu^2 r^2 - n^2)R = 0, \qquad \text{or equivalently} \quad R'' + \frac{1}{r}R' + \left(\mu^2 - \frac{n^2}{r^2}\right)R = 0.$$

With the substitution $x = \mu r$, this becomes the standard form:

$$x^2 y'' + xy' + (x^2 - \nu^2)y = 0. \tag{Bessel}$$

The Frobenius method (expanding $y = x^\rho\sum_{k=0}^\infty a_k x^k$ and finding the indicial equation $\rho^2 - \nu^2 = 0$) gives the fundamental solutions $J_\nu(x)$ and $Y_\nu(x)$ discussed in Section 1.

## Structure of This Chapter

**Section 1: Bessel Equation Revisited** rederives Bessel's equation from scratch — both from the cylindrical Laplacian and from physical problems (vibrating circular membrane, cylindrical heat flow, Schrödinger equation in a cylinder). The Frobenius solution gives the power series:

$$J_\nu(x) = \sum_{k=0}^\infty \frac{(-1)^k}{k!\,\Gamma(k+\nu+1)}\left(\frac{x}{2}\right)^{2k+\nu},$$

valid for all $x > 0$ when $\nu \geq 0$. The second solution $Y_\nu$ is constructed for non-integer $\nu$ as a linear combination $(J_\nu\cos\nu\pi - J_{-\nu})/\sin\nu\pi$, and for integer $n$ via a limiting procedure, producing a logarithmic singularity at the origin.

**Section 2: Properties and Zeros** covers the qualitative behavior of Bessel functions: the oscillatory decay $J_\nu(x) \sim \sqrt{2/\pi x}\cos(x - \nu\pi/2 - \pi/4)$ for large $x$, the recursion relations $(d/dx)[x^\nu J_\nu(x)] = x^\nu J_{\nu-1}(x)$ and $(d/dx)[x^{-\nu}J_\nu(x)] = -x^{-\nu}J_{\nu+1}(x)$, and the zeros $0 < j_{\nu,1} < j_{\nu,2} < \cdots$ of $J_\nu$. The zeros are simple, interlace with the zeros of $J_{\nu+1}$, and satisfy $j_{\nu,n} \approx (n + \nu/2 - 1/4)\pi$ for large $n$.

**Section 3: Orthogonality and Bessel Series** establishes the weighted orthogonality:

$$\int_0^R J_\nu\!\left(\frac{j_{\nu,m}}{R}r\right)J_\nu\!\left(\frac{j_{\nu,n}}{R}r\right)r\,dr = \frac{R^2}{2}[J_{\nu+1}(j_{\nu,n})]^2\delta_{mn},$$

and uses it to expand arbitrary functions in Bessel-Fourier series. The complete solution of the heat equation on a disk and the wave equation on a circular membrane are derived.

**Section 4: Modified Bessel Functions** treats $I_\nu(x) = i^{-\nu}J_\nu(ix)$ and $K_\nu(x)$ — the solutions of the modified Bessel equation $x^2y'' + xy' - (x^2+\nu^2)y = 0$ (with a sign change in the $x^2$ coefficient). Unlike $J_\nu$, the modified Bessel functions are monotone (not oscillatory): $I_\nu$ grows exponentially and $K_\nu$ decays exponentially as $x \to \infty$. They appear in problems where the separation constant has the "wrong" sign: for example, the steady-state heat distribution in a cylinder with heat source, or the radial wave function in quantum mechanics for a repulsive potential.

## Key Theorems

**Theorem (Sturm-Liouville form of Bessel's equation).** Bessel's equation of order $\nu$ on $[0,R]$ can be written in Sturm-Liouville form:

$$-\frac{d}{dr}\left(r\frac{dR}{dr}\right) + \frac{\nu^2}{r}R = \lambda r R,$$

with weight function $w(r) = r$ and $p(r) = r$, $q(r) = \nu^2/r$. With the boundary condition $R(R) = 0$ (Dirichlet) and boundedness at $r=0$, the eigenvalues are $\lambda_n = (j_{\nu,n}/R)^2 > 0$ and eigenfunctions $R_n(r) = J_\nu(j_{\nu,n}r/R)$.

**Theorem (completeness).** The Bessel functions $\{J_\nu(j_{\nu,n}r/R)\}_{n=1}^\infty$ form a complete orthogonal system in $L^2([0,R]; r\, dr)$. Every $f \in L^2([0,R]; r\,dr)$ satisfies:

$$f(r) = \sum_{n=1}^\infty c_n J_\nu\!\left(\frac{j_{\nu,n}}{R}r\right), \qquad c_n = \frac{2}{R^2[J_{\nu+1}(j_{\nu,n})]^2}\int_0^R f(r) J_\nu\!\left(\frac{j_{\nu,n}}{R}r\right)r\,dr,$$

with convergence in $L^2([0,R]; r\, dr)$.

## Applications Preview

The heat equation on a disk $r < R$ with $u|_{r=R} = 0$ and $u(r,\theta,0) = f(r,\theta)$ has solution:

$$u(r,\theta,t) = \sum_{n=1}^\infty\sum_{m=-\infty}^\infty c_{nm}\, e^{-\kappa(j_{|m|,n}/R)^2 t}\, J_{|m|}\!\left(\frac{j_{|m|,n}}{R}r\right)e^{im\theta},$$

showing that each mode decays with rate $\kappa(j_{|m|,n}/R)^2$ — the lowest mode (slowest decay) has rate $\kappa(j_{0,1}/R)^2$ where $j_{0,1} \approx 2.405$. The wave equation on a circular drumhead has fundamental frequency proportional to $j_{0,1}/(R)$, which is why circular drums of the same radius have the same fundamental note regardless of their tension-to-density ratio (only the pitch changes, not the harmonic structure).
