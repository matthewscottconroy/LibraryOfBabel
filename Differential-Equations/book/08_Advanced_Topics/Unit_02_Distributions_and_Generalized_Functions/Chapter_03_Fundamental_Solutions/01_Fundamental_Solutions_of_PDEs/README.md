# Fundamental Solutions of PDEs

A fundamental solution of a differential operator $P(D)$ is a distribution $E$ satisfying $P(D)E = \delta$. It represents the "point source response" of the operator: the solution to the PDE driven by an idealized point source at the origin. From $E$, solutions driven by arbitrary source terms are obtained by convolution. This section computes fundamental solutions for the three canonical operators of mathematical physics: the Laplacian, the heat operator, and the wave operator.

## The Laplacian

The Laplacian $\Delta = \sum_{i=1}^n \partial_{x_i}^2$ governs steady-state heat conduction, electrostatics, and the theory of harmonic functions. Its fundamental solution satisfies $\Delta E = \delta$ in $\mathbb{R}^n$.

**In dimension $n \geq 3$:** The fundamental solution is:

$$E(x) = \frac{1}{n(n-2)\omega_n |x|^{n-2}},$$

where $\omega_n = \pi^{n/2}/\Gamma(n/2 + 1)$ is the volume of the unit ball in $\mathbb{R}^n$.

**Verification (for $n = 3$):** We show $\Delta(1/4\pi|x|) = -\delta$ (note sign convention: $\Delta E = -\delta$ is also common).

For $x \neq 0$, $1/|x|$ is smooth and harmonic: $\Delta(1/|x|) = 0$.

For the distributional equality, use the distributional definition and Green's identity. For any $\phi \in \mathcal{D}(\mathbb{R}^3)$:

$$\left\langle \Delta\frac{1}{|x|}, \phi \right\rangle = \left\langle \frac{1}{|x|}, \Delta\phi \right\rangle = \int_{\mathbb{R}^3} \frac{\Delta\phi(x)}{|x|} \, dx.$$

Split: $\int_{|x|>\varepsilon} \frac{\Delta\phi}{|x|} + \int_{|x|<\varepsilon} \frac{\Delta\phi}{|x|}$. The second integral $\to 0$ as $\varepsilon \to 0$ (since $|\Delta\phi| \leq C$ and $\int_{|x|<\varepsilon} |x|^{-1} d^3x = 4\pi\varepsilon^2/2 \to 0$ in $\mathbb{R}^3$). For the first, integrate by parts twice using Green's second identity on $\{|x| > \varepsilon\}$:

$$\int_{|x|>\varepsilon} \frac{\Delta\phi}{|x|} = \int_{|x|>\varepsilon} \frac{\Delta(1/|x|)}{1}\phi \, dx + \oint_{|x|=\varepsilon} \left[\frac{1}{|x|}\frac{\partial\phi}{\partial r} - \phi\frac{\partial}{\partial r}\frac{1}{|x|}\right] d\sigma.$$

The first term is 0 (since $\Delta(1/|x|) = 0$ for $|x| > 0$). The boundary integral:
- $\frac{\partial\phi}{\partial r}$-term: $\int_{|x|=\varepsilon} \frac{1}{\varepsilon}\frac{\partial\phi}{\partial r} d\sigma = O(\varepsilon^2) \to 0$.
- $\frac{\partial}{\partial r}(1/|x|) = -1/|x|^2 = -1/\varepsilon^2$-term: $\int_{|x|=\varepsilon} \phi \cdot \frac{1}{\varepsilon^2} d\sigma \to \phi(0) \cdot 4\pi\varepsilon^2/\varepsilon^2 = 4\pi\phi(0)$.

So $\langle \Delta(1/|x|), \phi\rangle = 4\pi\phi(0)$, giving $\Delta(1/|x|) = 4\pi\delta$, i.e., $E = 1/(4\pi|x|)$ satisfies $\Delta E = \delta$. $\square$

**In dimension $n = 2$:** The fundamental solution is $E(x) = \frac{1}{2\pi}\log|x|$.

**Application.** The solution to the Poisson equation $\Delta u = f$ in $\mathbb{R}^n$ (with appropriate decay at infinity) is:

$$u(x) = (E * f)(x) = \int_{\mathbb{R}^n} E(x-y)f(y) \, dy.$$

In $\mathbb{R}^3$: $u(x) = \frac{1}{4\pi}\int_{\mathbb{R}^3} \frac{f(y)}{|x-y|} \, dy$—the Newtonian potential.

## The Heat Operator

The heat operator is $\partial_t - \Delta$ on $\mathbb{R}^n \times \mathbb{R}$. Its fundamental solution satisfies $(\partial_t - \Delta)E = \delta(x)\delta(t)$ in $\mathbb{R}^n \times \mathbb{R}$.

**The heat kernel:**

$$E(x, t) = \begin{cases} \displaystyle\frac{1}{(4\pi t)^{n/2}} e^{-|x|^2/(4t)} & t > 0 \\ 0 & t \leq 0. \end{cases}$$

**Verification.** For $t > 0$, direct computation gives $\partial_t E = \Delta E$ (the Gaussian satisfies the heat equation). As $t \to 0^+$, $E(\cdot, t) \to \delta$ in $\mathcal{D}'$: for any $\phi \in \mathcal{D}$, $\int E(x,t)\phi(x) \, dx = \int \phi(x + y\sqrt{4t})/(4\pi)^{n/2} e^{-|y|^2} \, dy \to \phi(0)$ by dominated convergence. So the jump at $t = 0$ (from 0 to a delta function) accounts for the $\delta(x)\delta(t)$ forcing.

**Causality.** The heat kernel is zero for $t < 0$: heat propagates forward in time. The support of $E$ is $\{(x,t) : t \geq 0\}$.

**Solution formula.** For $(\partial_t - \Delta)u = f$ with $u(x,0) = u_0(x)$:

$$u(x,t) = (E * f)(x,t) + \int_{\mathbb{R}^n} E(x-y,t) u_0(y) \, dy.$$

## The Wave Operator

The wave operator is $\Box = \partial_{tt} - c^2\Delta$ on $\mathbb{R}^n \times \mathbb{R}$. Its fundamental solution satisfies $\Box E = \delta(x)\delta(t)$.

**Causal fundamental solution** (satisfying $E = 0$ for $t < 0$):

- **$n = 1$:** $E(x,t) = \frac{1}{2c} H(t - |x|/c)$ (the Heaviside function on the light cone).

- **$n = 3$ (odd dimension $\geq 3$):** $E(x,t) = \frac{1}{4\pi c^2 t} \delta(ct - |x|) \cdot H(t)$—a delta distribution on the light cone $ct = |x|$. This implies **Huygens' principle**: the effect of a point source at $t = 0$ is felt only on the light cone, not in its interior.

- **$n = 2$ (even dimension):** $E(x,t) = \frac{H(ct - |x|)}{2\pi c\sqrt{c^2t^2 - |x|^2}} \cdot H(t)$—supported on and inside the light cone. Huygens' principle fails: signals travel at all speeds up to $c$.

**Odd vs. even dimension.** The qualitative difference between even and odd dimensions (Huygens' principle vs. its failure) is reflected in the support of $E$. In odd dimensions $\geq 3$, $E$ is supported on the light cone; in even dimensions, $E$ is supported inside the light cone.

## Regularity and the Smoothing Effect

The fundamental solution of the heat operator is smooth for $t > 0$ (even if the initial data is a delta function), reflecting the infinite-speed propagation and smoothing properties of diffusion. In contrast, the wave equation preserves regularity: if the initial data has a singularity, the singularity propagates along characteristics (the light cone). This difference—the heat equation smooths while the wave equation propagates singularities—is one of the fundamental distinctions between parabolic and hyperbolic PDEs.

The distributional fundamental solution framework makes these regularity statements precise: the singular support of $E$ (the set where $E$ fails to be smooth) propagates along characteristics for the wave equation, but shrinks to the empty set for positive times under the heat equation.
