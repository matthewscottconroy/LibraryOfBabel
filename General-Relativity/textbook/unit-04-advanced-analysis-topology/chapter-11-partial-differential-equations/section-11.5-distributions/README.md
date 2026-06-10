# Section 11.5: Distributions and Generalized Functions

---

## Section Introduction

Classical analysis requires functions to be well-behaved: continuous, differentiable, integrable. But physics constantly demands objects that violate these conditions. The **Dirac delta function** $\delta(x)$ is "zero everywhere except at $x=0$, with integral $1$" — clearly not a function in the classical sense. The step function $\theta(x)$ is not differentiable at $x=0$, yet its "derivative" $\theta'(x) = \delta(x)$ is needed in electrostatics (the surface charge density of a planar conductor is a delta function).

The **theory of distributions** (Schwartz, 1945) makes these objects rigorous. A distribution is a continuous linear functional on the space $\mathcal{D}$ of test functions (smooth functions with compact support). The delta distribution is defined by $\delta[\phi] = \phi(0)$ for all test functions $\phi$. Every locally integrable function $f$ defines a distribution $T_f[\phi] = \int f\phi\,dx$, so distributions generalize functions.

The key property: **every distribution is infinitely differentiable**. The derivative $T'$ of a distribution $T$ is defined by $T'[\phi] = -T[\phi']$ (integration by parts, but as a definition). This means we can differentiate $\theta(x)$ to get $\delta(x)$, differentiate $\delta(x)$ to get $\delta'(x)$, and so on indefinitely, without any smoothness conditions on the original distribution. Differential equations that have no classical solutions often have distributional solutions.

Distributions are essential for the rigorous treatment of Green's functions: the Green's function $G(x,x')$ satisfies $LG(x,x') = \delta(x-x')$ as a distributional equation. In field theory, the propagator (Green's function of the field equation) is a distribution. In GR, the geodesic equation for a point particle source produces a distributional stress-energy tensor $T^{\mu\nu}(\mathbf{x}) = m\int u^\mu u^\nu\delta^{(4)}(\mathbf{x}-\mathbf{x}(\tau))\,d\tau$.

---

## Subsections

- [11.5.1: Test Functions and the Space 𝒟](11.5.1-test-functions.md)
- [11.5.2: Distributions as Linear Functionals](11.5.2-distributions.md)
- [11.5.3: The Dirac Delta and Its Derivatives](11.5.3-delta.md)
- [11.5.4: Differentiation of Distributions](11.5.4-differentiation.md)
- [11.5.5: Distributional Green's Functions and Point Sources](11.5.5-greens.md)
