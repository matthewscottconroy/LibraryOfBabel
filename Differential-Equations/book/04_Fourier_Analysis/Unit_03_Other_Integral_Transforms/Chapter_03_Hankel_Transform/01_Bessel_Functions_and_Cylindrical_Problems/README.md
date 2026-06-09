# Bessel Functions and Cylindrical Problems

When Laplace's equation $\Delta u = 0$ is written in polar coordinates $(r, \theta)$ in the plane and solved by separation of variables $u(r,\theta) = R(r)\Theta(\theta)$, the radial factor $R$ satisfies the **Bessel equation**. Bessel functions are therefore the natural basis for problems with circular symmetry, playing the same role in cylindrical geometry that trigonometric functions play in Cartesian geometry.

## Bessel's Equation

Bessel's equation of order $\nu \geq 0$ is
$$x^2 y'' + xy' + (x^2 - \nu^2)y = 0, \quad x > 0.$$

This can be rewritten as $(xy')' + (x - \nu^2/x)y = 0$ or in self-adjoint form $-(xy')'/x + (\nu^2/x^2)y = \lambda y$ with $\lambda = 1$ — it is a Sturm-Liouville problem on $(0,\infty)$.

## Series Solution: Bessel Functions of the First Kind

The **Bessel function of the first kind of order $\nu$**, denoted $J_\nu(x)$, is the solution to Bessel's equation regular (bounded) at $x = 0$:
$$J_\nu(x) = \sum_{m=0}^\infty \frac{(-1)^m}{m!\,\Gamma(m+\nu+1)}\left(\frac{x}{2}\right)^{2m+\nu}.$$

For non-negative integer $n$, the Gamma function satisfies $\Gamma(m+n+1) = (m+n)!$ and $J_n(-x) = (-1)^n J_n(x)$.

**Key values and behavior:**
- $J_0(0) = 1$, $J_n(0) = 0$ for $n \geq 1$.
- For large $x$: $J_\nu(x) \sim \sqrt{2/(\pi x)}\cos(x - \nu\pi/2 - \pi/4)$ as $x \to \infty$. The Bessel functions decay as $1/\sqrt{x}$ and oscillate, somewhat like a decaying sinusoidal function.
- Zeros of $J_\nu$: each $J_\nu$ has infinitely many positive zeros $0 < j_{\nu,1} < j_{\nu,2} < \cdots$ accumulating at infinity. These are crucial for eigenvalue problems.

## Orthogonality

**Theorem.** On $[0, 1]$ with weight $r$, the functions $\{J_\nu(j_{\nu,k}r)\}_{k=1}^\infty$ (where $j_{\nu,k}$ are the positive zeros of $J_\nu$) form a complete orthogonal set:
$$\int_0^1 J_\nu(j_{\nu,k}r)J_\nu(j_{\nu,m}r)\,r\,dr = \frac{1}{2}[J_{\nu+1}(j_{\nu,k})]^2\delta_{km}.$$

On $(0,\infty)$ the orthogonality is distributional:
$$\int_0^\infty J_\nu(\lambda r)J_\nu(\mu r)\,r\,dr = \frac{1}{\lambda}\delta(\lambda - \mu), \quad \lambda, \mu > 0.$$

This is the orthogonality relation underlying the Hankel transform.

## Origin in Separation of Variables

**Laplace's equation in cylindrical coordinates.** For $u(r, \theta, z)$ satisfying $\Delta u = u_{rr} + \frac{1}{r}u_r + \frac{1}{r^2}u_{\theta\theta} + u_{zz} = 0$, with $u(r,\theta,z) = R(r)\Theta(\theta)Z(z)$:
- $\Theta$ satisfies $\Theta'' = -n^2\Theta$, giving $\Theta = e^{\pm in\theta}$ (integer $n$).
- $Z$ satisfies $Z'' = \lambda^2 Z$, giving $Z = e^{\pm\lambda z}$.
- $R$ satisfies $r^2 R'' + rR' + (\lambda^2 r^2 - n^2)R = 0$: Bessel's equation with substitution $x = \lambda r$ and order $\nu = n$.

The solution bounded at $r = 0$ is $R(r) = J_n(\lambda r)$. The general bounded solution is
$$u(r,\theta,z) = \sum_{n=-\infty}^\infty \int_0^\infty A_n(\lambda)J_n(\lambda r)e^{in\theta}e^{-\lambda z}\,d\lambda.$$

For **axially symmetric problems** (no $\theta$-dependence), only $n = 0$ terms survive and $u(r,z) = \int_0^\infty A(\lambda)J_0(\lambda r)e^{-\lambda z}\,d\lambda$, which is a Hankel transform.

## The Hankel Transform

**Definition.** The **Hankel transform of order $\nu$** of $f : (0,\infty) \to \mathbb{R}$ is
$$\mathcal{H}_\nu[f](\rho) = \int_0^\infty f(r)\,J_\nu(\rho r)\,r\,dr.$$

The **inverse Hankel transform** has the same form:
$$f(r) = \int_0^\infty \mathcal{H}_\nu[f](\rho)\,J_\nu(\rho r)\,\rho\,d\rho.$$

This self-reciprocal property (the transform and its inverse have the same form) follows from the orthogonality of Bessel functions: the Hankel transform is its own inverse (up to normalization), just as the Fourier transform satisfies $\mathcal{F}^2[f](x) = f(-x)$.

## Hankel Transform and the 2D Fourier Transform

**Theorem.** Let $f : \mathbb{R}^2 \to \mathbb{R}$ be radially symmetric: $f(\mathbf{x}) = g(r)$ where $r = |\mathbf{x}|$. Then the 2D Fourier transform $\hat{f}(\boldsymbol{\xi}) = \int_{\mathbb{R}^2}f(\mathbf{x})e^{-2\pi i\boldsymbol{\xi}\cdot\mathbf{x}}\,d^2x$ is also radially symmetric: $\hat{f}(\boldsymbol{\xi}) = G(\rho)$ where $\rho = |\boldsymbol{\xi}|$, and
$$G(\rho) = 2\pi\int_0^\infty g(r)J_0(2\pi\rho r)\,r\,dr = 2\pi\mathcal{H}_0[g](2\pi\rho).$$

**Proof.** In polar coordinates for $\boldsymbol{\xi} = (\rho\cos\alpha, \rho\sin\alpha)$ and $\mathbf{x} = (r\cos\theta, r\sin\theta)$:
$$\hat{f}(\boldsymbol{\xi}) = \int_0^\infty \int_0^{2\pi} g(r)e^{-2\pi i\rho r\cos(\theta-\alpha)}r\,d\theta\,dr.$$
The inner integral $\int_0^{2\pi} e^{-2\pi i\rho r\cos\phi}\,d\phi = 2\pi J_0(2\pi\rho r)$ (the integral representation of $J_0$). Therefore:
$$\hat{f}(\boldsymbol{\xi}) = 2\pi\int_0^\infty g(r)J_0(2\pi\rho r)\,r\,dr.$$

## Worked Example: Heat Equation in a Cylinder

Consider $u_t = \Delta u$ on $\mathbb{R}^2 \times (0,\infty)$ with $u(r, 0) = f(r)$ (radially symmetric). Taking $\mathcal{H}_0$ in $r$:

The 2D Laplacian in radial coordinates is $\Delta = \partial_r^2 + \frac{1}{r}\partial_r$. For the Hankel transform:
$$\mathcal{H}_0\!\left[\frac{\partial^2 f}{\partial r^2} + \frac{1}{r}\frac{\partial f}{\partial r}\right](\rho) = -\rho^2\mathcal{H}_0[f](\rho).$$
(This follows from Bessel's equation: if $R(r) = J_0(\rho r)$, then $R'' + \frac{1}{r}R' = -\rho^2 R$, so integration by parts gives the result.)

The PDE becomes $\hat{u}_t(\rho, t) = -\rho^2\hat{u}(\rho, t)$, with solution $\hat{u}(\rho, t) = \hat{f}(\rho)e^{-\rho^2 t}$. Inverting:
$$u(r, t) = \int_0^\infty \hat{f}(\rho)e^{-\rho^2 t}J_0(\rho r)\rho\,d\rho.$$

If $f(r) = \delta(r)/(2\pi r)$ (a point source at the origin), then $\hat{f}(\rho) = 1/(2\pi)$ (using the Hankel transform of the delta function), and the solution is the 2D heat kernel $u(r,t) = \frac{1}{4\pi t}e^{-r^2/(4t)}$.
