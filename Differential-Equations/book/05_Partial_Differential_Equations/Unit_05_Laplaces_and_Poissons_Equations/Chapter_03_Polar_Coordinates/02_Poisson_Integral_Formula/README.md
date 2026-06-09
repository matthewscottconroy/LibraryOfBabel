# The Poisson Integral Formula

The Poisson integral formula

$$u(r,\theta) = \frac{1}{2\pi}\int_0^{2\pi}\frac{R^2-r^2}{R^2 - 2Rr\cos(\theta-\phi)+r^2}\,f(\phi)\,d\phi \tag{1}$$

is one of the most important explicit representation formulas in analysis. It expresses the value of a harmonic function inside a disk purely in terms of its boundary values, provides quantitative control over the solution, and has applications in complex analysis, probability, and approximation theory.

## The Poisson Kernel

The **Poisson kernel** for the disk of radius $R$ is:

$$P_R(r,\theta) = \frac{R^2-r^2}{R^2 - 2Rr\cos\theta + r^2}, \qquad 0 \leq r < R.$$

The formula (1) is $u(r,\theta) = \frac{1}{2\pi}\int_0^{2\pi}P_R(r,\theta-\phi)f(\phi)\,d\phi$.

**Key properties:**

1. $P_R(r,\theta) > 0$ for $r < R$ (the kernel is positive).
2. $\frac{1}{2\pi}\int_0^{2\pi}P_R(r,\theta)\,d\theta = 1$ for each $r < R$ (normalization).
3. For $\delta > 0$: $\max_{|\theta|>\delta}P_R(r,\theta) \to 0$ as $r\to R^-$ (the kernel concentrates at $\theta = 0$).
4. At $r=0$: $P_R(0,\theta) = 1$ for all $\theta$ (the formula at the center gives the average of $f$).

Properties 1-3 are the conditions for $P_R(r,\cdot)/(2\pi)$ to be an approximate identity as $r\to R$.

## Complex Form

Using $z = re^{i\theta}$ and $\zeta = Re^{i\phi}$, the Poisson kernel is the real part of the Cauchy kernel:

$$\text{Re}\!\left[\frac{\zeta+z}{\zeta-z}\right] = \frac{|\zeta|^2 - |z|^2}{|\zeta-z|^2} = P_R(r,\theta-\phi).$$

The Poisson formula is thus:

$$u(r,\theta) = \text{Re}\!\left[\frac{1}{2\pi i}\oint_{|\zeta|=R}\frac{\zeta+z}{\zeta-z}\frac{f(\arg\zeta)}{|\zeta|}\,|d\zeta|\right],$$

connecting it to the Cauchy integral formula of complex analysis.

## Convergence Theorem

**Theorem.** Let $f \in C(\partial B_R)$. Define $u$ by formula (1) in $B_R$ and $u = f$ on $\partial B_R$. Then $u \in C(\bar B_R)$ and $u$ is harmonic in $B_R$.

**Proof sketch.** Harmonicity of (1) in $B_R$ follows by differentiating under the integral and verifying $\Delta u = 0$ (since $P_R$ is harmonic in $z$ for fixed $\phi$). Continuity up to the boundary: for $f\in C(\partial B_R)$, given $\varepsilon > 0$, choose $\delta$ such that $|f(\phi)-f(\theta)|<\varepsilon$ for $|\phi-\theta|<\delta$. Then $|u(r,\theta)-f(\theta)| = |\frac{1}{2\pi}\int(f(\phi)-f(\theta))P_R(r,\theta-\phi)\,d\phi| \leq \varepsilon + 2\|f\|_\infty\cdot\max_{|\phi-\theta|>\delta}P_R \to \varepsilon$ as $r\to R$.

## Poisson Formula in Higher Dimensions

In $\mathbb{R}^n$ for $n \geq 2$, the Poisson formula for the ball $B_R$ is:

$$u(\mathbf{x}) = \frac{R^2-|\mathbf{x}|^2}{n\omega_n R}\int_{\partial B_R}\frac{f(\mathbf{y})}{|\mathbf{x}-\mathbf{y}|^n}\,dS(\mathbf{y}),$$

where $\omega_n = \pi^{n/2}/\Gamma(n/2+1)$ is the volume of the unit ball. The kernel $(R^2-|\mathbf{x}|^2)/(|\mathbf{x}-\mathbf{y}|^n)$ (the Poisson kernel in $n$ dimensions) satisfies the same properties (positive, integrates to $\omega_n R$, concentrates near $\mathbf{y}=\mathbf{x}$ as $|\mathbf{x}|\to R$).

## Applications

**Dirichlet problem in the upper half-plane.** Taking $R\to\infty$ and mapping the disk to the upper half-plane by a Möbius transformation, the Poisson formula becomes:

$$u(x,y) = \frac{y}{\pi}\int_{-\infty}^\infty\frac{f(t)}{(x-t)^2+y^2}\,dt, \qquad y > 0.$$

The kernel $P(x-t,y) = y/[\pi((x-t)^2+y^2)]$ is the Poisson kernel for the upper half-plane.

**Schwarz-Christoffel formula.** For domains more complex than the disk, conformal mappings transform the Poisson formula into integration against more complicated kernels. This is the basis of classical methods for solving potential problems in engineering.

**Brownian motion.** The Poisson kernel is the probability density for the first exit point of a Brownian motion from $B_R$, started at $\mathbf{x}$ — a direct probabilistic interpretation of the representation formula.
