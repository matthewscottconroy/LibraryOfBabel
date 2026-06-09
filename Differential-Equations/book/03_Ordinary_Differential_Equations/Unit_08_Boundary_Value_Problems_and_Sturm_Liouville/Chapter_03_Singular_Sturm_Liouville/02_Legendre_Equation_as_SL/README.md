# Legendre Equation as a Sturm-Liouville Problem

The Legendre equation $(1-x^2)y'' - 2xy' + \lambda y = 0$ on $[-1,1]$ is one of the most important singular Sturm-Liouville problems. Written in SL form, it is $[(1-x^2)y']' + \lambda y = 0$, with $p(x) = 1-x^2$, $q(x) = 0$, and $w(x) = 1$. The coefficient $p$ vanishes at both endpoints $x = \pm 1$, making both endpoints singular. The SL framework explains why the Legendre polynomials are orthogonal on $[-1,1]$ with weight $1$, form a complete basis, and satisfy the specific eigenvalue equation with $\lambda = n(n+1)$.

## Setup as an SL Problem

The Legendre equation in SL form:

$$\frac{d}{dx}\!\left[(1-x^2)\frac{dy}{dx}\right] + \lambda y = 0, \qquad -1 < x < 1.$$

Since $p(x) = 1-x^2 \to 0$ as $x \to \pm 1$, both endpoints are singular. The weight function is $w(x) = 1$, and the inner product is the standard $L^2[-1,1]$ inner product $\langle f,g\rangle = \int_{-1}^1 f(x)g(x)\,dx$.

The Weyl classification at $x = 1$ (and by symmetry, at $x = -1$): the equation near $x = 1$ behaves like $(1-x^2)y'' = (2x)y' \approx 2(1-x)y'$ for $x$ near $1$. The indicial equation (in terms of $u = 1-x$) gives exponents $r = 0$ and $r = 0$ (a repeated root), with solutions $\phi_1 \approx 1$ (bounded) and $\phi_2 \approx \ln(1-x)$ (unbounded). Both solutions have finite $L^2[-1,1]$ norm: $\int_{-1}^1 |\ln(1-x)|^2\,dx < +\infty$. This is the limit-circle case — both solutions are square-integrable, and a boundary condition (boundedness) must be imposed at each endpoint.

The boundary condition at each singular endpoint is: **$y$ must be bounded** (equivalently, square-integrable near $\pm 1$). This excludes the Legendre function of the second kind $Q_\lambda(x)$, which has a logarithmic singularity at $x = \pm 1$ for all $\lambda$.

## Eigenvalues and Eigenfunctions

With the boundedness condition imposed at both endpoints, the eigenvalue problem has solutions only for $\lambda = n(n+1)$, $n = 0, 1, 2, \ldots$. The proof: for general $\lambda$, the bounded solution near $x = 1$ can be continued as a power series; requiring boundedness also at $x = -1$ forces the series to terminate, which occurs exactly when $\lambda = n(n+1)$.

The eigenfunctions are the Legendre polynomials $P_n(x)$:

$$P_0(x) = 1, \quad P_1(x) = x, \quad P_2(x) = \frac{1}{2}(3x^2-1), \quad P_3(x) = \frac{1}{2}(5x^3-3x), \ldots$$

Each $P_n$ is a polynomial of degree $n$, even for even $n$ and odd for odd $n$, normalized by $P_n(1) = 1$.

## Orthogonality from SL Theory

By the general SL orthogonality theorem (since the Legendre operator is self-adjoint on $L^2[-1,1]$ with the appropriate domain), eigenfunctions corresponding to different eigenvalues are orthogonal:

$$\int_{-1}^1 P_m(x)P_n(x)\,dx = 0, \qquad m \neq n.$$

The norms: $\int_{-1}^1 [P_n(x)]^2\,dx = 2/(2n+1)$.

This orthogonality arises from the SL self-adjointness. The Lagrange identity gives:

$$\int_{-1}^1\{P_n[(1-x^2)P_m']' - P_m[(1-x^2)P_n']'\}\,dx = [(1-x^2)(P_nP_m' - P_mP_n')]_{-1}^1.$$

The boundary term vanishes because $(1-x^2) \to 0$ at $x = \pm 1$ and the Legendre polynomials are bounded: $(1-x^2)^{1/2}P_n'(x)$ remains bounded at $\pm 1$ (since $P_n$ are polynomials). The integrand equals $(\lambda_m - \lambda_n)P_nP_m$, giving orthogonality.

This is the SL proof of Legendre polynomial orthogonality — cleaner and more systematic than computing the integrals directly.

## Completeness and Legendre Series

By the SL completeness theorem, the Legendre polynomials form a complete orthonormal basis for $L^2[-1,1]$: any $f \in L^2[-1,1]$ can be expanded as:

$$f(x) = \sum_{n=0}^\infty c_n P_n(x), \qquad c_n = \frac{2n+1}{2}\int_{-1}^1 f(x)P_n(x)\,dx.$$

This **Legendre series** converges in $L^2$ and pointwise at continuity points of $f$ (under mild smoothness assumptions). It is the appropriate expansion for functions defined on $[-1,1]$, in the same way that the Fourier series is the appropriate expansion for periodic functions.

**Application.** In solving Laplace's equation $\nabla^2 u = 0$ in a sphere (in spherical coordinates), the angular dependence separates into the Legendre equation (in $\cos\theta$). The bounded solutions are $P_n(\cos\theta)$, and the general solution for a problem with azimuthal symmetry is $u(r,\theta) = \sum_{n=0}^\infty (A_n r^n + B_n r^{-(n+1)})P_n(\cos\theta)$. The Legendre series of the boundary data determines the coefficients $A_n$ and $B_n$. This is the canonical application of Legendre polynomials as SL eigenfunctions.

## Connection to Associated Legendre Functions

When azimuthal symmetry is absent, Laplace's equation in spherical coordinates produces the **associated Legendre equation**:

$$[(1-x^2)y']' + \left[n(n+1) - \frac{m^2}{1-x^2}\right]y = 0.$$

This is a singular SL problem with weight $w = 1$ on $[-1,1]$, and with a singular potential $m^2/(1-x^2)$ at $x = \pm 1$. The eigenfunctions (bounded solutions for $|m| \leq n$) are the associated Legendre functions $P_n^m(x)$, which together with the Fourier functions $e^{im\phi}$ form the spherical harmonics $Y_n^m(\theta,\phi)$ — the complete orthonormal basis for $L^2$ on the sphere. Spherical harmonics are thus the SL eigenfunctions for the angular part of Laplace's equation in three dimensions, and their properties (orthogonality, addition theorem, multipole expansion) all flow from the SL framework.
