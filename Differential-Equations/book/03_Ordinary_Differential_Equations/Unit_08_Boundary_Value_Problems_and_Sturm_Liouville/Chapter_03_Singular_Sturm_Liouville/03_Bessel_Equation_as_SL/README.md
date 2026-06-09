# Bessel Equation as a Sturm-Liouville Problem

The Bessel equation $x^2y'' + xy' + (\lambda x^2 - \nu^2)y = 0$ arises when solving Laplace's equation or the wave equation in cylindrical coordinates by separation of variables. Written in SL form, it is $[xy']' + (\lambda x - \nu^2/x)y = 0$ on $(0,R)$, a singular SL problem with $p(x) = x$, $w(x) = x$, and $q(x) = \nu^2/x$. The singular endpoint is $x = 0$ (where $p(0) = 0$ and $q(0) = \infty$); the endpoint $x = R$ is regular with a standard Dirichlet or Neumann boundary condition. The eigenfunctions are the Bessel functions $J_\nu(\sqrt{\lambda}x)$, and the completeness theorem gives the **Fourier-Bessel series** — the appropriate expansion for functions defined on a disk or cylinder.

## SL Form of the Bessel Equation

Dividing the Bessel equation $x^2y'' + xy' + (\lambda x^2 - \nu^2)y = 0$ by $x$ and rearranging:

$$\frac{1}{x}\frac{d}{dx}\!\left(x\frac{dy}{dx}\right) + \left(\lambda - \frac{\nu^2}{x^2}\right)y = 0.$$

This is the SL form $Ly = \lambda y$ with:

$$L = -\frac{1}{x}\left[\frac{d}{dx}\!\left(x\frac{d}{dx}\right) - \frac{\nu^2}{x}\right], \qquad p(x) = x, \quad q(x) = \frac{\nu^2}{x}, \quad w(x) = x.$$

The inner product for the weighted $L^2$ space is $\langle f,g\rangle = \int_0^R f(x)g(x)x\,dx$.

## Singular Endpoint at $x = 0$

The endpoint $x = 0$ is singular because $p(0) = 0$. The Bessel equation has a regular singular point at $x = 0$. The Frobenius method gives two solutions:

$$y_1(x) = J_\nu(x) = \sum_{m=0}^\infty \frac{(-1)^m}{m!\,\Gamma(m+\nu+1)}\left(\frac{x}{2}\right)^{2m+\nu}, \qquad \text{(bounded, } \sim x^\nu \text{ as } x \to 0)$$

$$y_2(x) = Y_\nu(x), \qquad \text{(singular, } Y_\nu \sim \ln x \text{ for } \nu = 0, \text{ or } \sim x^{-\nu} \text{ for } \nu > 0)$$

The square-integrability condition with weight $w = x$: $\int_0^\epsilon |y_2(x)|^2 x\,dx$. For $Y_\nu$:

If $\nu = 0$: $Y_0 \sim \ln x$, so $|Y_0|^2 x \sim (\ln x)^2 x \to 0$, and $\int_0^\epsilon (\ln x)^2 x\,dx < \infty$ (integrable). This is limit-circle.

If $0 < \nu < 1$: $Y_\nu \sim x^{-\nu}$, so $|Y_\nu|^2 x \sim x^{1-2\nu}$, and $\int_0^\epsilon x^{1-2\nu}\,dx < \infty$ iff $1 - 2\nu > -1$ iff $\nu < 1$. Both $J_\nu$ and $Y_\nu$ are square-integrable: limit-circle case, boundary condition needed.

If $\nu \geq 1$: $|Y_\nu|^2 x \sim x^{1-2\nu}$, integrable iff $\nu < 1$: for $\nu \geq 1$, $Y_\nu \notin L^2(0,\epsilon; x\,dx)$. Only $J_\nu$ is square-integrable: limit-point case, no boundary condition needed.

In all cases, the physically relevant boundary condition at $x = 0$ is **boundedness** (equivalently, the solution is $J_\nu$, which is bounded at the origin). The singular behavior of $Y_\nu$ at $x = 0$ excludes it from the domain of the SL operator.

## Eigenvalue Problem on $(0, R)$

With the boundedness condition at $x = 0$ and the Dirichlet condition $y(R) = 0$ at $x = R$, the eigenvalue problem is: find $\lambda$ such that $J_\nu(\sqrt{\lambda}R) = 0$. Let $\alpha_{\nu,n}$ denote the $n$-th positive zero of $J_\nu$ ($n = 1, 2, 3, \ldots$). Then:

$$\lambda_n = \left(\frac{\alpha_{\nu,n}}{R}\right)^2, \qquad \phi_n(x) = J_\nu\!\left(\frac{\alpha_{\nu,n}}{R}x\right), \qquad n = 1, 2, 3, \ldots$$

These are the eigenvalues and eigenfunctions of the Bessel SL problem. All eigenvalues are positive (since $\nu \geq 0$, the Rayleigh quotient is positive).

## Orthogonality: The Fourier-Bessel Orthogonality

By the SL orthogonality theorem, with weight $w(x) = x$:

$$\int_0^R J_\nu\!\left(\frac{\alpha_{\nu,m}}{R}x\right)J_\nu\!\left(\frac{\alpha_{\nu,n}}{R}x\right)x\,dx = 0, \qquad m \neq n.$$

The norms: using the Bessel function identity $\int_0^R [J_\nu(\alpha_{\nu,n}x/R)]^2 x\,dx = \frac{R^2}{2}[J_{\nu+1}(\alpha_{\nu,n})]^2$ (the Lommel formula).

The Lagrange identity gives:

$$\int_0^R\{J_{\nu,m}[xJ_{\nu,n}']' - J_{\nu,n}[xJ_{\nu,m}']'\}\,dx = [x(J_{\nu,m}J_{\nu,n}' - J_{\nu,n}J_{\nu,m}')]_0^R,$$

where $J_{\nu,n} = J_\nu(\alpha_{\nu,n}x/R)$. The boundary term at $R$ vanishes because $J_{\nu,m}(R) = J_{\nu,n}(R) = 0$ (both vanish at the zeros). The boundary term at $0$ vanishes because $J_{\nu,n} \sim x^\nu \to 0$ and $J_{\nu,n}' \sim x^{\nu-1}$, so $x \cdot (J_{\nu,m}J_{\nu,n}' - J_{\nu,n}J_{\nu,m}') \sim x^{2\nu} \to 0$ (for $\nu \geq 0$). The integrand on the left equals $(\lambda_m - \lambda_n)J_{\nu,m}J_{\nu,n} \cdot x$. For $m \neq n$ ($\lambda_m \neq \lambda_n$), orthogonality follows.

## Fourier-Bessel Series and Completeness

By the completeness theorem for singular SL problems (which holds for the Bessel problem), any $f \in L^2((0,R), x\,dx)$ can be expanded as:

$$f(x) = \sum_{n=1}^\infty c_n J_\nu\!\left(\frac{\alpha_{\nu,n}}{R}x\right), \qquad c_n = \frac{2}{R^2[J_{\nu+1}(\alpha_{\nu,n})]^2}\int_0^R f(x)J_\nu\!\left(\frac{\alpha_{\nu,n}}{R}x\right)x\,dx.$$

This is the **Fourier-Bessel series** (or Bessel-Fourier expansion). It converges in $L^2((0,R),x\,dx)$ and pointwise (under smoothness conditions).

## Application to the Heat Equation in a Disk

Consider $u_t = \kappa\nabla^2 u$ on the disk $r < R$ with $u(R,t) = 0$ (zero temperature at the boundary) and initial condition $u(r,\theta,0) = f(r)$ (radially symmetric). Separation $u = R(r)T(t)$ gives $T'/T = -\kappa\lambda$ and $r^{-1}(rR')' + \lambda R = 0$ (the Bessel equation with $\nu = 0$). The bounded solution is $R(r) = J_0(\sqrt{\lambda}r)$, and the eigenvalues are $\lambda_n = (\alpha_{0,n}/R)^2$.

The solution:

$$u(r,t) = \sum_{n=1}^\infty c_n J_0\!\left(\frac{\alpha_{0,n}}{R}r\right)e^{-\kappa(\alpha_{0,n}/R)^2 t},$$

with $c_n = \frac{2}{R^2[J_1(\alpha_{0,n})]^2}\int_0^R f(r)J_0(\alpha_{0,n}r/R)r\,dr$.

Each term decays exponentially in time, with higher modes decaying faster. The initial data is expanded in the Fourier-Bessel series, and the solution for $t > 0$ is obtained by multiplying each term by $e^{-\kappa\lambda_n t}$ — the SL eigenfunction method applied to the heat equation in cylindrical geometry.

This example illustrates the general principle: whenever separation of variables in a particular coordinate system produces a Bessel equation, the Fourier-Bessel expansion provides the correct eigenfunction expansion for solving boundary value problems with cylindrical or circular symmetry.
