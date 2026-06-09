# The Fundamental Solution: The Heat Kernel

The heat kernel

$$K(x,t) = \frac{1}{\sqrt{4\pi\kappa t}}\,e^{-x^2/(4\kappa t)}, \qquad x \in \mathbb{R},\; t > 0,$$

is the most important single object in the theory of the heat equation. It is the solution corresponding to a point source of heat at $x=0$ and $t=0$ — a delta-function initial condition. Understanding the heat kernel in depth provides insight into the structure of all solutions, the nature of parabolic smoothing, and the connection between diffusion and probability.

## Definition and Verification

The heat kernel is the unique solution of the initial value problem:

$$K_t = \kappa K_{xx}, \qquad K(x,0) = \delta(x).$$

**Verification that $K$ satisfies the heat equation:**

$$K_t = K\cdot\left(-\frac{1}{2t} + \frac{x^2}{4\kappa t^2}\right), \qquad K_{xx} = K\cdot\left(\frac{x^2}{4\kappa^2 t^2} - \frac{1}{2\kappa t}\right).$$

Then $\kappa K_{xx} = K\cdot(x^2/(4\kappa t^2) - 1/(2t)) = K_t$. Confirmed.

**Verification of initial condition:** As $t \to 0^+$, $K(x,t) \to \delta(x)$ in the distributional sense:

1. $K(x,t) > 0$ and $\int_{-\infty}^\infty K(x,t)\,dx = 1$ for all $t > 0$ (the Gaussian integrates to $1$).
2. For any $\varepsilon > 0$: $\int_{|x|>\varepsilon} K(x,t)\,dx = \text{erfc}(\varepsilon/\sqrt{4\kappa t}) \to 0$ as $t\to 0^+$.

So $K(\cdot,t)$ concentrates its mass at $x=0$ as $t\to 0^+$ — it is an approximate identity (a mollifier).

## Properties of the Heat Kernel

**Gaussian profile.** $K(x,t)$ is a Gaussian with mean $0$ and variance $\sigma^2 = 2\kappa t$. The width of the Gaussian (the standard deviation $\sigma = \sqrt{2\kappa t}$) grows as $\sqrt{t}$ — the hallmark of diffusive spreading.

**Normalization.** $\int_{-\infty}^\infty K(x,t)\,dx = 1$ for all $t > 0$. The total "heat" (or probability) is conserved.

**Positivity.** $K(x,t) > 0$ for all $x \in \mathbb{R}$, $t > 0$.

**Symmetry.** $K(-x,t) = K(x,t)$ (even in $x$).

**Semigroup property.** The heat kernels form a semigroup under convolution:

$$K(\cdot,t) * K(\cdot,s) = K(\cdot, t+s),$$

i.e., $\int_{-\infty}^\infty K(x-y,t)K(y,s)\,dy = K(x,t+s)$.

This follows from the fact that the convolution of two Gaussians with variances $\sigma_1^2$ and $\sigma_2^2$ is a Gaussian with variance $\sigma_1^2 + \sigma_2^2$: since $K(\cdot,t)$ has variance $2\kappa t$, the convolution has variance $2\kappa(t+s)$ — exactly $K(\cdot,t+s)$.

**Self-similarity.** $K(x,t) = \lambda^{-1}K(x/\lambda, t/\lambda^2)$ for any $\lambda > 0$. In the similarity variable $\eta = x/\sqrt{4\kappa t}$: $K(x,t) = (4\pi\kappa t)^{-1/2}e^{-\eta^2}$.

## The Heat Kernel in $\mathbb{R}^n$

In $n$ spatial dimensions, the fundamental solution of $u_t = \kappa\Delta u$ is

$$K_n(\mathbf{x},t) = \frac{1}{(4\pi\kappa t)^{n/2}}\exp\!\left(-\frac{|\mathbf{x}|^2}{4\kappa t}\right).$$

This is a product of $n$ one-dimensional heat kernels:

$$K_n(\mathbf{x},t) = K_1(x_1,t)\cdot K_1(x_2,t)\cdots K_1(x_n,t).$$

The solution to the Cauchy problem in $\mathbb{R}^n$ is $u(\mathbf{x},t) = \int_{\mathbb{R}^n}K_n(\mathbf{x}-\mathbf{y},t)f(\mathbf{y})\,d\mathbf{y}$.

The width of the Gaussian in each direction grows as $\sqrt{\kappa t}$, so the heat spreads spherically symmetrically (for the isotropic case), with a "diffusion front" at radius $\sim\sqrt{\kappa t}$.

## The Semigroup Interpretation

The formula $u(\cdot,t) = K(\cdot,t) * f$ can be written as $u(\cdot,t) = S(t)f$, where $S(t)$ is the **heat semigroup** (convolution by the heat kernel). The semigroup property $K(\cdot,t)*K(\cdot,s)=K(\cdot,t+s)$ corresponds to $S(t)S(s) = S(t+s)$: evolving from $0$ to $t$ and then from $t$ to $t+s$ is the same as evolving directly from $0$ to $t+s$.

This semigroup structure underlies the modern treatment of parabolic PDEs via operator semigroup theory. The generator of the heat semigroup is the Laplacian $\kappa\Delta$: in a formal sense, $S(t) = e^{\kappa t\Delta}$. This notation is made rigorous by the spectral theorem for the Laplacian.

## Explicit Computations

**Moment computation.** The $k$-th moment of the heat kernel (for even $k = 2m$):

$$\int_{-\infty}^\infty x^{2m}K(x,t)\,dx = (2\kappa t)^m\cdot(2m-1)!! = (2m-1)(2m-3)\cdots 3\cdot 1 \cdot (2\kappa t)^m.$$

In particular, $\int x^2 K\,dx = 2\kappa t$ (the variance) and $\int x^4 K\,dx = 12(\kappa t)^2$.

**Solution for Gaussian initial data.** If $f(x) = A e^{-\alpha x^2}$, the convolution gives:

$$u(x,t) = A\sqrt{\frac{\pi}{\alpha + 1/(4\kappa t)}}\cdot\frac{1}{\sqrt{4\pi\kappa t}}\cdot e^{-x^2/(4\kappa t + 1/\alpha)} = \frac{A}{\sqrt{1+4\kappa\alpha t}}\,e^{-\alpha x^2/(1+4\kappa\alpha t)}.$$

The amplitude decreases as $(1+4\kappa\alpha t)^{-1/2}$ and the width increases as $\sqrt{1+4\kappa\alpha t}$ — the Gaussian spreads while maintaining unit total integral (times $A\sqrt{\pi/\alpha}$). This is a "spreading Gaussian" solution, the exact solution corresponding to Gaussian initial data.

## The Heat Kernel and the Gamma Function

The heat kernel at $x = 0$ is $K(0,t) = (4\pi\kappa t)^{-1/2}$. Integrating over time:

$$\int_0^T K(0,t)\,dt = \frac{\sqrt{T}}{\sqrt{\pi\kappa}},$$

which grows without bound as $T\to\infty$. This reflects the fact that the origin is visited infinitely often by a Brownian path — Brownian motion in one dimension is recurrent.

In three dimensions, $K_3(\mathbf{0},t) = (4\pi\kappa t)^{-3/2}$, and $\int_0^\infty K_3(\mathbf{0},t)\,dt < \infty$ — the Green's function for the Laplacian in $\mathbb{R}^3$ is $G(\mathbf{x}) = (4\pi\kappa)^{-1}|\mathbf{x}|^{-1}$ (up to constants), reflecting the fact that three-dimensional Brownian motion is transient (a particle escapes to infinity with probability $1$).
