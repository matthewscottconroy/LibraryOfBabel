# The Dirichlet Problem on the Disk

The Dirichlet problem for Laplace's equation on the disk is the most beautiful and thoroughly studied problem in classical PDE theory. Its explicit solution — the Poisson integral formula — connects harmonic function theory to Fourier analysis, complex analysis, and probability. The derivation by separation of variables in polar coordinates is a model of the method applied in circular geometry.

## The Problem

Find $u(r,\theta)$ satisfying:

$$\Delta u = u_{rr} + \frac{1}{r}u_r + \frac{1}{r^2}u_{\theta\theta} = 0, \qquad 0 < r < R,$$
$$u(R,\theta) = f(\theta), \qquad 0 \leq \theta < 2\pi.$$

The solution must be bounded as $r\to 0$ and $2\pi$-periodic in $\theta$.

## Separation of Variables

Seek $u = R(r)\Theta(\theta)$. Separating:

$$\frac{r(rR')'}{R} = -\frac{\Theta''}{\Theta} = n^2.$$

**Angular equation:** $\Theta'' + n^2\Theta = 0$, with $2\pi$-periodicity giving $n = 0, 1, 2, \ldots$ and $\Theta_n = A_n\cos(n\theta) + B_n\sin(n\theta)$.

**Radial equation (Euler equation):** $r^2 R'' + rR' - n^2 R = 0$, with general solution $R = Cr^n + Dr^{-n}$ for $n \geq 1$ (and $R = C + D\log r$ for $n = 0$). Boundedness at $r=0$ requires $D=0$, so $R_n = r^n$.

## General Harmonic Function on the Disk

$$u(r,\theta) = \frac{a_0}{2} + \sum_{n=1}^\infty r^n(a_n\cos n\theta + b_n\sin n\theta). \tag{1}$$

At $r = R$:

$$f(\theta) = \frac{a_0}{2} + \sum_{n=1}^\infty R^n(a_n\cos n\theta + b_n\sin n\theta).$$

By Fourier orthogonality:

$$a_0 = \frac{1}{\pi}\int_0^{2\pi}f(\theta)\,d\theta, \qquad a_n = \frac{1}{\pi R^n}\int_0^{2\pi}f(\theta)\cos(n\theta)\,d\theta, \qquad b_n = \frac{1}{\pi R^n}\int_0^{2\pi}f(\theta)\sin(n\theta)\,d\theta.$$

## Summing the Series: The Poisson Integral Formula

The series (1) can be summed in closed form. Using complex notation with $z = re^{i\theta}$ and $\zeta = Re^{i\phi}$:

$$u(r,\theta) = \text{Re}\!\left[\frac{1}{2\pi i}\oint_{|\zeta|=R}f(\arg\zeta)\frac{\zeta+z}{\zeta-z}\frac{d\zeta}{\zeta}\right] = \frac{1}{2\pi}\int_0^{2\pi}f(\phi)\cdot\text{Re}\!\left[\frac{Re^{i\phi}+re^{i\theta}}{Re^{i\phi}-re^{i\theta}}\right]d\phi.$$

Computing the real part: $\text{Re}\left[\frac{Re^{i\phi}+re^{i\theta}}{Re^{i\phi}-re^{i\theta}}\right] = \frac{R^2-r^2}{|Re^{i\phi}-re^{i\theta}|^2} = \frac{R^2-r^2}{R^2-2Rr\cos(\theta-\phi)+r^2}$.

**Poisson Integral Formula:**

$$\boxed{u(r,\theta) = \frac{1}{2\pi}\int_0^{2\pi}\frac{R^2-r^2}{R^2-2Rr\cos(\theta-\phi)+r^2}f(\phi)\,d\phi.} \tag{2}$$

The factor $P(r,\theta-\phi;R) = \frac{R^2-r^2}{R^2-2Rr\cos(\theta-\phi)+r^2}/(2\pi)$ is the **Poisson kernel**. It satisfies:

1. $P > 0$ for $r < R$.
2. $\int_0^{2\pi}P\,d\phi = 1$ (it is a probability measure).
3. As $r\to R^-$ with $\theta\neq\phi$, $P\to 0$ (the kernel concentrates at $\phi = \theta$).
4. As $r\to R^-$ with $\theta = \phi$, $P\to +\infty$.

Properties 1-4 show that $P$ is an approximate identity in $\phi$ as $r\to R$: the formula (2) averages $f$ with Gaussian-like weights concentrated near $\phi = \theta$.

## Properties of the Solution

**Existence and uniqueness.** For $f \in L^1(0,2\pi)$, the formula (2) gives a harmonic function in $B_R$, and as $r\to R^-$, $u(r,\theta)\to f(\theta)$ at every Lebesgue point of $f$ (in particular, at every continuity point of $f$).

**Regularity.** For any $r < R$, $u(r,\cdot)$ is $C^\infty$ (in fact real-analytic). If $f\in C^k(\partial B_R)$, then $u\in C^k(\bar B_R)$.

**Mean value property.** At $r=0$: $u(0) = \frac{1}{2\pi}\int_0^{2\pi}f(\phi)\,d\phi$ — the value at the center is the average of $f$ over the boundary.

**Maximum principle.** The Poisson kernel is positive, so the formula immediately shows $\min f \leq u(r,\theta) \leq \max f$ for all $r < R$.

## Example: $f(\theta) = \cos\theta$

By the Fourier coefficient formula: $a_1 = 1/R$, all others zero. The solution is $u(r,\theta) = (r/R)\cos\theta$.

Check: this is the real part of $z/R$ (analytic function), confirming it is harmonic.

## Example: $f(\theta) = 1$ for $0<\theta<\pi$ and $f(\theta)=-1$ for $\pi<\theta<2\pi$

This is the signum function on the boundary. The Fourier coefficients: $a_n = 0$ (odd boundary data means $a_n = 0$, $b_n$ for even $n$ vanish by antisymmetry). $b_n = \frac{4}{n\pi R^n}$ for odd $n$. The solution is a Fourier series that converges in $B_R$ and reproduces the jump discontinuity at $\theta = 0, \pi$ on the boundary — but inside the disk, the solution is smooth (harmonic).

The Poisson formula gives $u(r,\theta) = \frac{2}{\pi}\arctan\!\left(\frac{2Rr\sin\theta}{R^2-r^2}\right)$ in closed form (derivable by contour integration), which equals $\pm 1$ on the appropriate boundary arcs and transitions smoothly inside.
