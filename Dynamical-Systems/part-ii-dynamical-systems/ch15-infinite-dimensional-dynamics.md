# Chapter 15 — Infinite-Dimensional and Random Dynamics

> *When the phase space is a function space, the system is a PDE. When the system is driven by noise, the orbits are random. Both cases require the full theory of functional analysis and stochastic calculus.*

**Prerequisites:** Chapters 1 (Banach/Hilbert spaces, semigroups), 2 (measure theory, probability), 4 (ODEs).

**What this chapter builds:** The theory of $C_0$-semigroups as the abstraction of flows on Banach spaces; global attractors for dissipative PDEs; inertial manifolds; stochastic differential equations; and random dynamical systems with random attractors.

---

## 15.1 $C_0$-Semigroups

**Definition 15.1.1.** A *strongly continuous one-parameter semigroup* ($C_0$-semigroup) on a Banach space $X$ is a family $\{T(t)\}_{t \geq 0} \subseteq \mathcal{B}(X)$ satisfying:
1. $T(0) = I$ (identity at time 0)
2. $T(t+s) = T(t)T(s)$ for all $s, t \geq 0$ (semigroup property)
3. $\lim_{t \to 0^+} T(t)x = x$ for all $x \in X$ (strong continuity)

The *infinitesimal generator* is $\mathcal{A}x = \lim_{t \to 0^+} \frac{T(t)x - x}{t}$ with domain $D(\mathcal{A})$.

**Theorem 15.1.2 (Hille-Yosida).** $\mathcal{A}: D(\mathcal{A}) \to X$ is the generator of a $C_0$-semigroup iff:
- $\mathcal{A}$ is closed and densely defined
- The resolvent $(\lambda I - \mathcal{A})^{-1}$ exists for all $\lambda > \omega$ (some $\omega \in {\mathbb R}$) with $\|(\lambda I - \mathcal{A})^{-n}\| \leq M/(\lambda - \omega)^n$

**Examples:**
- Heat equation on $L^2(\Omega)$: $\mathcal{A} = \Delta$ (Laplacian with boundary conditions), $T(t) = e^{t\Delta}$ (heat semigroup)
- Translation semigroup on $L^2({\mathbb R})$: $T(t)f(x) = f(x+t)$, generator $\mathcal{A} = d/dx$
- Delay differential equations can be reformulated as $C_0$-semigroups on $C([-h, 0]; {\mathbb R}^n)$

---

## 15.2 Global Attractors

For dissipative PDE systems, the long-time behavior concentrates on a finite-dimensional object.

**Definition 15.2.1.** A *semiflow* on a Banach space $X$ is a family $\{S(t)\}_{t \geq 0}$ of continuous maps $S(t): X \to X$ with $S(0) = \text{id}$ and $S(t+s) = S(t) \circ S(s)$.

**Definition 15.2.2.** A *global attractor* for a semiflow $\{S(t)\}$ is a compact set $\mathcal{A} \subseteq X$ that is:
- Invariant: $S(t)\mathcal{A} = \mathcal{A}$ for all $t \geq 0$
- Attracting: $\text{dist}(S(t)B, \mathcal{A}) \to 0$ as $t \to \infty$ for every bounded set $B$

**Theorem 15.2.3 (Existence of Global Attractors).** A semiflow $\{S(t)\}$ has a global attractor iff it is:
1. *Asymptotically compact*: for every bounded sequence $(x_n)$ and $t_n \to \infty$, $(S(t_n)x_n)$ has a convergent subsequence
2. *Pointwise dissipative*: every orbit enters a fixed bounded set

**Example 15.2.4 (2D Navier-Stokes Equations).** The 2D incompressible Navier-Stokes equations:
$$\partial_t u + (u \cdot \nabla)u = \nu\Delta u - \nabla p + f, \quad \nabla \cdot u = 0$$
on a periodic domain. Temam proved:
- The semiflow has a global attractor $\mathcal{A}$ in $H = \{u \in L^2 : \nabla \cdot u = 0\}$
- $\dim_H(\mathcal{A}) \leq C(\nu, f)$ (finite Hausdorff dimension, depending on viscosity and forcing)
- The Lyapunov dimension estimate: $\dim_H(\mathcal{A}) \lesssim (L/\ell_d)^2$ where $\ell_d$ is the Kolmogorov dissipation scale

---

## 15.3 Inertial Manifolds

**Definition 15.3.1.** An *inertial manifold* for a semiflow $\{S(t)\}$ is a finite-dimensional positively invariant $C^1$ manifold $\mathcal{M}$ that:
1. Contains the global attractor: $\mathcal{A} \subseteq \mathcal{M}$
2. Attracts all orbits exponentially: $\text{dist}(S(t)u, \mathcal{M}) \leq C e^{-\alpha t}$ for some $C, \alpha > 0$

**Theorem 15.3.2 (Foias-Sell-Temam, 1988).** Under a *spectral gap condition* on the linear part of the equation, a semilinear parabolic PDE has an inertial manifold.

*Consequence:* The PDE is reduced to a finite-dimensional ODE on the inertial manifold — finite-dimensional dynamics captures all asymptotic behavior. This is the rigorous foundation for the claim that the Navier-Stokes attractor has finite degrees of freedom.

---

## 15.4 Stochastic Differential Equations

**Definition 15.4.1 (Itô SDE).** A *stochastic differential equation* (SDE) is:
$$dX_t = b(X_t)\,dt + \sigma(X_t)\,dW_t, \quad X_0 = x,$$
where $W_t$ is a standard Brownian motion and $b, \sigma$ are the drift and diffusion coefficients.

The solution $X_t$ is a stochastic process (a path-valued random variable).

### 15.4.1 Itô's Formula

**Theorem 15.4.2 (Itô's Formula).** If $X_t$ satisfies the SDE above and $f: {\mathbb R} \to {\mathbb R}$ is $C^2$, then:
$$df(X_t) = f'(X_t)\,dX_t + \frac{1}{2}f''(X_t)\sigma^2(X_t)\,dt.$$

The extra $\frac{1}{2}f''$ term (Itô correction) distinguishes stochastic calculus from ordinary calculus. It arises from the quadratic variation $[W, W]_t = t$ of Brownian motion.

**Application:** For $f(x) = x^2$: $d(X_t^2) = 2X_t\,dX_t + \sigma^2(X_t)\,dt$. The $\sigma^2 dt$ term represents the "noise" added to energy by the stochastic forcing.

### 15.4.2 The Fokker-Planck Equation

The probability density $\rho(x, t)$ of $X_t$ satisfies the *Fokker-Planck (forward Kolmogorov) equation*:
$$\partial_t \rho = -\partial_x(b\rho) + \frac{1}{2}\partial_x^2(\sigma^2 \rho).$$

The *stationary distribution* $\rho_\infty$ (if it exists) satisfies $-\partial_x(b\rho_\infty) + \frac{1}{2}\partial_x^2(\sigma^2\rho_\infty) = 0$.

**Example 15.4.3 (Ornstein-Uhlenbeck Process).** $dX_t = -\alpha X_t\,dt + \sigma\,dW_t$ (linear SDE). Solution: $X_t = e^{-\alpha t} X_0 + \sigma\int_0^t e^{-\alpha(t-s)}\,dW_s$. Stationary distribution: $N(0, \sigma^2/2\alpha)$.

---

## 15.5 Random Dynamical Systems

A *random dynamical system* (RDS) models the combination of a deterministic evolution with random perturbations.

**Definition 15.5.1 (Cocycle).** A *random dynamical system* over a probability-preserving transformation $\theta: (\Omega, \mathcal{F}, P, \theta)$ (the "driving noise") is a measurable map $\Phi: {\mathbb R}_+ \times \Omega \times X \to X$ satisfying the *cocycle property*:
$$\Phi(t+s, \omega) = \Phi(t, \theta^s\omega) \circ \Phi(s, \omega) \quad P\text{-a.s.}$$

**Example 15.5.2.** The solution $X_t^\omega$ of an SDE defines a random dynamical system with $\Phi(t, \omega)(x) = X_t^\omega(x)$ (solution starting at $x$ with noise realization $\omega$).

### 15.5.1 Random Attractors

**Definition 15.5.3.** A *random attractor* for an RDS $\Phi$ is a family of compact sets $\mathcal{A}(\omega)$ (depending measurably on $\omega$) that is:
- Invariant: $\Phi(t, \omega)\mathcal{A}(\omega) = \mathcal{A}(\theta^t\omega)$
- Pullback attracting: $\text{dist}(\Phi(t, \theta^{-t}\omega)B, \mathcal{A}(\omega)) \to 0$ as $t \to \infty$ for every bounded $B$

The *pullback* is crucial: instead of measuring the current state converging to $\mathcal{A}(\omega)$, we measure the past-started states converging to the attractor at the current noise $\omega$.

**Theorem 15.5.4 (Existence of Random Attractors).** Under conditions analogous to the deterministic case (dissipativity, compactness of pullback), a random dynamical system has a random attractor.

---

## 15.6 Stochastic Bifurcation Theory

In random systems, bifurcations of the *stationary distribution* (phenomenological bifurcation) can differ from bifurcations of the *invariant measure structure* (dynamical bifurcation).

**Phenomenological Bifurcation (P-bifurcation):** The shape of the stationary density changes qualitatively (e.g., unimodal $\to$ bimodal).

**Dynamical Bifurcation (D-bifurcation):** Lyapunov exponents (now random quantities themselves, computed via the multiplicative ergodic theorem for RDS) change sign.

**Example 15.6.1 (Pitchfork in Noise).** For $dX = (\mu X - X^3)\,dt + \sigma\,dW_t$: For $\mu < 0$, stationary density unimodal at 0. For $\mu > \sigma^2/2$ (not at $\mu = 0$!): bimodal density appears. The P-bifurcation is shifted by the noise.

---

## Exercises

**Exercise 15.1.** Show that the heat semigroup $T(t)f = e^{t\Delta}f$ satisfies all properties of a $C_0$-semigroup on $L^2(\Omega)$. Compute its generator. What is the domain $D(\mathcal{A})$?

**Exercise 15.2.** (Navier-Stokes Attractor) The 2D Navier-Stokes semiflow satisfies $\frac{d}{dt}\|u\|^2 \leq -\nu\|u\|_H^2 + \|f\|^2/\nu$ (energy estimate). Use this to show the semiflow is dissipative.

**Exercise 15.3.** Use Itô's formula to compute $d(e^{-\alpha t} X_t^2)$ for the Ornstein-Uhlenbeck process $dX_t = -\alpha X_t\,dt + \sigma\,dW_t$. Conclude the formula for $E[X_t^2]$ and verify the stationary variance $\sigma^2/(2\alpha)$.

**Exercise 15.4.** (Stochastic Logistic) $dX_t = X_t(\mu - X_t)\,dt + \sigma X_t\,dW_t$. Use the substitution $Y_t = 1/X_t$ and Itô's formula to solve this SDE explicitly.

**Exercise 15.5.** Construct a random attractor for the random system $\dot{x} = -x + \xi(t)$ where $\xi$ is Ornstein-Uhlenbeck noise. The random attractor is a single (time-dependent) point $x^*(\omega)$. Find it.

---

## Chapter Notes

For $C_0$-semigroups and the Hille-Yosida theorem: Pazy's *Semigroups of Linear Operators and Applications to Partial Differential Equations* and Engel-Nagel's *One-Parameter Semigroups for Linear Evolution Equations*. For global attractors: Temam's *Infinite-Dimensional Dynamical Systems in Mechanics and Fluid Dynamics* and Robinson's *Infinite-Dimensional Dynamical Systems*.

For stochastic differential equations: Øksendal's *Stochastic Differential Equations* is the accessible introduction; Karatzas-Shreve's *Brownian Motion and Stochastic Calculus* is the rigorous reference. For random dynamical systems: Arnold's (L.) *Random Dynamical Systems* is the comprehensive treatment.
