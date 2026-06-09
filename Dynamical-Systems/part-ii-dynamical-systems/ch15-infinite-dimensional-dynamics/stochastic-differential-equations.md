# 15.4 Stochastic Differential Equations

So far in this chapter, the dynamics has been deterministic — a PDE driven by a fixed external forcing $f$. Now we introduce randomness. Many physical systems are driven by noise: thermal fluctuations in molecular systems, random external forcings in geophysical models, measurement noise in control systems. The mathematics of random dynamics is stochastic calculus, centered on the theory of stochastic differential equations (SDEs).

The conceptual challenge with noise-driven systems is that Brownian motion $W_t$ — the canonical model of noise — is continuous but nowhere differentiable. Classical ODE theory requires Lipschitz right-hand sides; a term involving $dW_t/dt$ does not make sense classically. Itô's theory resolves this by defining a new integral — the Itô stochastic integral — directly in terms of the Brownian motion, without differentiating it.

**Definition 15.4.1 (Itô SDE).** A *stochastic differential equation* (SDE) is:
$$dX_t = b(X_t)\,dt + \sigma(X_t)\,dW_t, \quad X_0 = x,$$
where $W_t$ is a standard Brownian motion (on some probability space), $b: \mathbb{R}^d \to \mathbb{R}^d$ is the *drift* coefficient, and $\sigma: \mathbb{R}^d \to \mathbb{R}^{d \times m}$ is the *diffusion* coefficient. The notation $dX_t$ means the equation is interpreted in *integral form*: $X_t = x + \int_0^t b(X_s)\,ds + \int_0^t \sigma(X_s)\,dW_s$ where the second integral is the Itô stochastic integral.

The solution $X_t$ is a stochastic process — a path-valued random variable. For Lipschitz $b$ and $\sigma$ with at most linear growth, solutions exist and are unique (the Itô existence-uniqueness theorem, the stochastic analog of the Picard theorem).

## 15.4.1 Itô's Formula

The most important computational tool in stochastic calculus is Itô's formula — the stochastic chain rule. Unlike the classical chain rule, it has an extra term coming from the quadratic variation of Brownian motion.

**Theorem 15.4.2 (Itô's Formula).** If $X_t$ satisfies the SDE above and $f: \mathbb{R} \to \mathbb{R}$ is $C^2$, then:
$$df(X_t) = f'(X_t)\,dX_t + \frac{1}{2}f''(X_t)\sigma^2(X_t)\,dt.$$

The extra $\frac{1}{2}f''$ term is the *Itô correction*. It arises from the quadratic variation of Brownian motion: $[W, W]_t = t$ (not 0, as it would be for a classical smooth path). Formally, $(dW_t)^2 = dt$, and the Taylor expansion of $f(X_{t+dt}) - f(X_t)$ must include the second-order term in $dW_t$.

What this is saying is: stochastic calculus has a "correction" relative to ordinary calculus, coming from the non-trivial quadratic variation of Brownian motion. Smooth curves have zero quadratic variation, so the classical chain rule has no correction. Brownian motion has linear quadratic variation ($[W,W]_t = t$), so the chain rule picks up a $\frac{1}{2}f'' \sigma^2 dt$ term.

**Application:** For $f(x) = x^2$:
$$d(X_t^2) = 2X_t\,dX_t + \sigma^2(X_t)\,dt.$$
The $\sigma^2 dt$ term represents the "energy" added to the system by stochastic forcing. In the Ornstein-Uhlenbeck process below, this noise energy balances the damping to give a stationary state.

## 15.4.2 The Fokker-Planck Equation

The SDE describes individual trajectories. If we want to describe the evolution of the *distribution* of trajectories, we use the *Fokker-Planck equation* (the forward Kolmogorov equation).

The probability density $\rho(x, t)$ of $X_t$ (assuming a density exists) satisfies:
$$\partial_t \rho = -\partial_x(b\rho) + \frac{1}{2}\partial_x^2(\sigma^2 \rho).$$

The first term on the right ($-\partial_x(b\rho)$) is the transport term: the density is convected by the drift $b$. The second term ($\frac{1}{2}\partial_x^2(\sigma^2\rho)$) is the diffusion term: noise spreads the distribution.

The *stationary distribution* $\rho_\infty$ — the time-invariant solution — satisfies $\partial_t \rho_\infty = 0$, giving the ODE:
$$-\partial_x(b\rho_\infty) + \frac{1}{2}\partial_x^2(\sigma^2\rho_\infty) = 0.$$
When a stationary distribution exists and is unique, the SDE defines a *stationary process* and the associated ergodic theory (Chapter 7) applies.

**Example 15.4.3 (Ornstein-Uhlenbeck Process).** $dX_t = -\alpha X_t\,dt + \sigma\,dW_t$ (the simplest mean-reverting SDE). Solution (by Itô's formula or variation of parameters):
$$X_t = e^{-\alpha t} X_0 + \sigma\int_0^t e^{-\alpha(t-s)}\,dW_s.$$
The integral is a Gaussian random variable with mean 0 and variance $\sigma^2(1-e^{-2\alpha t})/(2\alpha)$. As $t \to \infty$, $X_t \to N(0, \sigma^2/(2\alpha))$ — the stationary distribution is a Gaussian with variance $\sigma^2/(2\alpha)$.

The Ornstein-Uhlenbeck process is the stochastic analog of a linearly stable fixed point: the drift $-\alpha X$ pulls orbits back to 0, while the noise $\sigma\,dW_t$ keeps spreading them. The balance gives a Gaussian stationary distribution.
