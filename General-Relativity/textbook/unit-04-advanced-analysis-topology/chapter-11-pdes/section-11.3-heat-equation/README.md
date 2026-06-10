# Section 11.3: The Heat Equation and Diffusion

---

## Section Introduction

The heat equation $\partial u / \partial t = \kappa \nabla^2 u$ describes diffusion: the spread of heat (or particles, or probability) from regions of high concentration to regions of low concentration. Unlike the wave equation — which propagates disturbances at a fixed speed — the heat equation **spreads** disturbances instantaneously (in the mathematical idealization), giving it a fundamentally different character.

This instantaneous spreading seems to violate special relativity. And indeed, the standard heat equation is a non-relativistic approximation. The full relativistic treatment of diffusion is more subtle, and in GR, heat and diffusion phenomena in strong gravitational fields require careful treatment. But the parabolic structure of the heat equation — its smoothing properties, its maximum principle, its connection to probability (the Wiener process) — illuminates the mathematical machinery of PDEs in a clean setting.

In GR, the heat equation appears in Ricci flow: Hamilton's (1982) equation for how a Riemannian metric evolves to become more uniform. Ricci flow was the tool Perelman used to prove the Poincaré conjecture in 2003.

---

## 11.3.1 The Heat Equation and Its Solution

The **heat equation** on ℝⁿ for t > 0:

$$\frac{\partial u}{\partial t} = \kappa \nabla^2 u, \quad u(x, 0) = u_0(x)$$

**Fundamental solution** (heat kernel): The special solution with initial data u₀ = δ(x) (a point source of heat at the origin) is:

$$K(x, t) = \frac{1}{(4\pi\kappa t)^{n/2}} \exp\left(-\frac{|x|^2}{4\kappa t}\right)$$

This is a Gaussian (normal distribution) that starts infinitely concentrated at the origin and spreads out over time. The width grows as σ ∝ √(κt) — the hallmark of diffusion.

**General solution**: By superposition:

$$u(x, t) = \int_{\mathbb{R}^n} K(x - y, t) u_0(y) \, dy$$

This convolution formula gives the solution for any initial data u₀. It shows:
1. The solution at time t is the convolution of the initial data with the Gaussian K(·, t).
2. Even if u₀ is discontinuous or has singularities (provided it's integrable), u(x, t) is smooth for any t > 0 — the heat equation is **smoothing**.
3. The solution at (x, t) depends on the initial data everywhere (non-zero contribution from all y) — information travels instantaneously. (This is the non-relativistic approximation; in reality, diffusion is a result of microscopic particle collisions and signals do not travel faster than c.)

**Verification**: K satisfies the heat equation (compute ∂K/∂t = κ∇²K — exercise). And ∫K(x,t)dx = 1 for all t > 0, with K(x,t) → δ(x) as t → 0⁺ in the distributional sense.

---

## 11.3.2 Maximum Principle and Uniqueness

**Theorem** (Maximum Principle): Let u be a smooth solution of the heat equation u_t = κ∇²u on a bounded domain Ω × [0, T]. Then:

$$\max_{\Omega \times [0,T]} u = \max_{\partial_p(\Omega \times [0,T])} u$$

where $\partial_p$ is the **parabolic boundary**: the spatial boundary ∂Ω × [0, T] plus the initial surface Ω × {0} (but NOT the top Ω × {T}).

In words: the maximum of a solution to the heat equation is attained either at the initial time or on the spatial boundary — never in the interior at a later time.

*Proof sketch*: Suppose u attains a maximum at an interior point (x₀, t₀). Then u_t(x₀, t₀) ≥ 0 (u is not decreasing in time at a maximum) and ∇²u(x₀, t₀) ≤ 0 (u has a spatial maximum). The heat equation u_t = κ∇²u would then give 0 ≤ κ · (something ≤ 0) — a contradiction. (A careful argument replaces u by u − εt to handle the equality case.) □

**Consequence** (Uniqueness): Two solutions u and v of the heat equation with the same initial and boundary data satisfy u = v everywhere. Proof: apply the maximum principle to u − v (which satisfies the heat equation with zero initial and boundary data, hence u − v = 0).

**Physical interpretation**: Heat cannot accumulate in the interior without being supplied from the boundary. A hot spot in the interior will always diffuse and cool. The maximum principle captures the second law of thermodynamics in this mathematical setting.

---

## 11.3.3 Separation of Variables and Fourier Series

On the interval [0, L] with boundary conditions u(0, t) = u(L, t) = 0 (ends held at temperature 0):

**Separation**: u = X(x)T(t) gives T'/T = κX''/X = −κλ. The eigenvalue problem X'' + λX = 0, X(0) = X(L) = 0 gives λ_n = (nπ/L)², X_n = sin(nπx/L).

**Time factor**: T_n' = −κλ_n T_n, so T_n(t) = e^{−κλ_n t} = e^{−κ(nπ/L)²t}.

**General solution**:

$$u(x, t) = \sum_{n=1}^\infty b_n e^{-\kappa(n\pi/L)^2 t} \sin\left(\frac{n\pi x}{L}\right)$$

where $b_n = (2/L)\int_0^L u_0(x)\sin(n\pi x/L) dx$ are the Fourier sine coefficients of the initial temperature.

**Long-time behavior**: As t → ∞, all exponentials decay. The slowest-decaying mode is n = 1, with decay rate κ(π/L)². For t ≫ (L/π)²/κ, the solution approaches zero like $b_1 e^{-\kappa(\pi/L)^2 t} \sin(\pi x/L)$ — exponential decay to the equilibrium temperature (zero, in this case).

**The Fourier coefficients encode the initial data**: Hotter initial temperatures (large b_n for small n) take longer to cool. Finer spatial structure (large n modes) decays faster.

---

## 11.3.4 Heat Equation and Probability: The Wiener Process

The heat kernel K(x, t) = (4πκt)^{−n/2} exp(−|x|²/4κt) is precisely the probability density of a particle undergoing **Brownian motion** (the Wiener process): starting at the origin at t = 0, the particle is at position x at time t with probability density K(x, t).

The **Wiener process** W(t) is the mathematically rigorous model for Brownian motion: a random process with W(0) = 0, continuous paths, and independent Gaussian increments W(t) − W(s) ~ N(0, t−s) for 0 ≤ s < t.

The heat equation is the Fokker-Planck equation for the Wiener process. This connection — between PDEs and probability — is deep and powerful:
- **Feynman-Kac formula**: solutions to the heat equation (and more general parabolic PDEs) can be represented as **expected values** over paths of the Wiener process.
- **Path integrals**: in quantum mechanics, the Euclidean path integral (after the Wick rotation t → it) is exactly the Wiener measure. Quantum amplitudes are "sums over paths" weighted by e^{−S_E/ℏ}.
- **GR connection**: The Euclidean path integral for quantum gravity (after Wick rotation g → g_E) involves integration over Euclidean metrics. The heat kernel K(x, y; t) on a curved space (defined by ∂_t K = ∇²K, K(x, y; 0) = δ(x, y)) encodes geometric information: its small-t expansion $K(x, x; t) \sim (4\pi t)^{-n/2}(1 + a_1 R t + a_2 R^2 t^2 + \ldots)$ produces the **heat kernel coefficients** that appear in the one-loop effective action in curved spacetime and in the spectral zeta function.

---

## 11.3.5 Ricci Flow

**Definition** (Hamilton, 1982): **Ricci flow** is the PDE for a family of Riemannian metrics g(t) on a manifold M:

$$\frac{\partial g_{\mu\nu}}{\partial t} = -2 R_{\mu\nu}$$

where R_{μν} is the Ricci tensor of g(t). This equation evolves the metric so that regions of high curvature spread out and regions of low curvature concentrate, analogous to heat flowing from hot to cold.

**Analogy**: Ricci flow is "the heat equation for the metric": the Ricci tensor is the natural second-order divergence of the metric, and −2R_{μν} drives the metric toward being "more constant" (i.e., more Einstein: R_{μν} ∝ g_{μν}).

**Short-time existence**: By Hamilton's work, for any smooth compact Riemannian manifold (M, g₀), Ricci flow has a unique smooth solution on [0, T) for some T > 0. This uses the maximum principle for parabolic systems — the same tool as for the heat equation.

**Perelman's breakthrough**: Grigori Perelman (2002–2003) proved that Ricci flow with surgery on a 3-manifold converges (after surgeries that cut out forming singularities) to a canonical metric, allowing the classification of 3-manifolds and proving the Poincaré conjecture. The key new tool was Perelman's **entropy functional**: a Lyapunov function for the Ricci flow, analogous to thermodynamic entropy, whose monotonic increase ensures the flow doesn't "bounce." [Perelman, G. (2002). "The entropy formula for the Ricci flow and its geometric applications." arXiv:math/0211159.]

---

## References

- Fourier, J.B.J. (1822). *Théorie analytique de la chaleur.* Paris. [The foundational treatise on heat conduction. Fourier introduces the heat equation and the method of separation of variables. He also introduces the Fourier series — the decomposition of an arbitrary periodic function into sinusoids — as the natural tool for solving the heat equation.]
- Evans, L.C. (2010). *Partial Differential Equations*, 2nd ed. AMS. [Chapter 2.3: heat equation; the fundamental solution; mean value formula; maximum principle; energy methods.]
- Hamilton, R.S. (1982). "Three-manifolds with positive Ricci curvature." *Journal of Differential Geometry*, 17, 255–306. [The founding paper of Ricci flow. Hamilton proves short-time existence and uses Ricci flow to classify compact 3-manifolds with positive Ricci curvature.]
- Perelman, G. (2002). "The entropy formula for the Ricci flow and its geometric applications." arXiv:math/0211159. [The first of Perelman's three preprints proving the Poincaré conjecture and geometrization conjecture. Introduces the entropy monotonicity — the key new idea.]
- Choquet-Bruhat, Y. and Geroch, R. (1969). "Global aspects of the Cauchy problem in general relativity." *Communications in Mathematical Physics*, 14, 329–335. [Proves existence of a unique maximal Cauchy development for the Einstein equations — the GR analogue of global well-posedness for parabolic PDEs.]
