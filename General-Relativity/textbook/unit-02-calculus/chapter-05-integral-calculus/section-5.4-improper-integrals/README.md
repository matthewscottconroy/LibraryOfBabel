# Section 5.4: Improper Integrals

---

## Section Introduction

The Riemann integral ∫ₐᵇ f(x) dx was defined for bounded functions on bounded intervals. Many important integrals fail one or both conditions: the integrand may blow up at some point (an unbounded integrand), or the limits of integration may be infinite. These are **improper integrals**, defined as limits of proper integrals.

Improper integrals appear everywhere in physics. The total energy in an electromagnetic field involves ∫₀^∞. Probability distributions must integrate to 1 over all of ℝ. Green's functions and propagators involve integrals over all space or time. The Fourier transform ∫_{−∞}^{∞} f(x)e^{ikx} dx is the foundation of spectral analysis in GR (Chapter 42). We must know when these integrals converge, and how to compute them when they do.

---

## 5.4.1 Improper Integrals of Type I (Infinite Limits)

**Definition**: If f is integrable on [a, b] for every b > a, we define:

$$\int_a^{\infty} f(x) \, dx = \lim_{b \to \infty} \int_a^b f(x) \, dx$$

if this limit exists (and is finite). Similarly for ∫_{-∞}^b and ∫_{-∞}^∞ = ∫_{-∞}^c + ∫_c^∞.

**Examples**:

1. $\int_1^\infty \frac{1}{x^p} dx$: The antiderivative of x^{-p} is x^{1-p}/(1-p) for p ≠ 1. Then $\int_1^b x^{-p} dx = \frac{b^{1-p} - 1}{1-p}$. As b → ∞: this → −1/(1-p) = 1/(p-1) if p > 1, and → ∞ if p < 1. For p = 1: ∫₁ᵇ 1/x dx = ln b → ∞. 

   *Conclusion*: $\int_1^\infty x^{-p} dx$ **converges iff p > 1**.

2. $\int_0^\infty e^{-x} dx = [-e^{-x}]_0^\infty = 0 - (-1) = 1$.

3. $\int_{-\infty}^\infty e^{-x^2} dx = \sqrt{\pi}$. This is the **Gaussian integral**, one of the most important in physics. It cannot be computed by elementary antiderivatives; the standard proof uses a polar coordinate trick (squaring the integral and converting to 2D). The result:

   $$\int_{-\infty}^\infty e^{-x^2} dx = \sqrt{\pi}$$

   This integral appears in the normalization of probability distributions, the path integral in quantum field theory, and the heat kernel in GR.

---

## 5.4.2 Improper Integrals of Type II (Unbounded Integrand)

**Definition**: If f is integrable on [a+ε, b] for every ε > 0 but f(a) is infinite (or undefined), we define:

$$\int_a^b f(x) \, dx = \lim_{\varepsilon \to 0^+} \int_{a+\varepsilon}^b f(x) \, dx$$

if this limit exists.

**Example**: $\int_0^1 x^{-p} dx$ for p > 0.

$\int_\varepsilon^1 x^{-p} dx = \left[\frac{x^{1-p}}{1-p}\right]_\varepsilon^1 = \frac{1 - \varepsilon^{1-p}}{1-p}$ (for p ≠ 1). As ε → 0⁺: εˁ⁻ᵖ → 0 if 1-p > 0 (i.e., p < 1). So the integral converges iff p < 1, to 1/(1−p). For p ≥ 1 it diverges.

Combining: $\int_0^1 x^{-p} dx$ converges iff p < 1; $\int_1^\infty x^{-p} dx$ converges iff p > 1. The "same" integrand x^{-p} has convergent integrals on both intervals only if... no value of p works for both! The function x^{-p} is integrable near 0 iff p < 1, and integrable near ∞ iff p > 1.

---

## 5.4.3 Convergence Tests

For improper integrals where direct computation is difficult, comparison tests give convergence/divergence without finding the exact value.

**Comparison Test**: If 0 ≤ f(x) ≤ g(x) for all large x, and ∫g converges, then ∫f converges. If ∫f diverges, then ∫g diverges.

**Limit Comparison Test**: If f, g ≥ 0 and lim_{x→∞} f(x)/g(x) = L ∈ (0, ∞), then ∫f and ∫g converge or diverge together.

**Absolute Convergence**: If ∫ₐ^∞ |f(x)| dx converges, we say ∫f is **absolutely convergent**. Absolute convergence implies convergence. (Proof: f = f⁺ − f⁻ where f⁺ = max(f,0), f⁻ = max(-f,0); both are bounded by |f|.)

A conditionally convergent integral ∫f is one that converges but ∫|f| diverges. Example: $\int_1^\infty \frac{\sin x}{x} dx$ converges (conditionally) but $\int_1^\infty \frac{|\sin x|}{x} dx = \infty$.

---

## 5.4.4 The Gamma Function

A beautiful improper integral defines the **Gamma function**:

$$\Gamma(s) = \int_0^\infty t^{s-1} e^{-t} \, dt \quad (s > 0)$$

**Properties**:
- Γ(1) = ∫₀^∞ e^{-t} dt = 1
- Γ(n+1) = nΓ(n) (integration by parts: ∫₀^∞ tⁿ e^{-t} dt = n ∫₀^∞ t^{n-1} e^{-t} dt)
- Γ(n) = (n-1)! for positive integers n — so Γ extends the factorial to non-integers
- Γ(1/2) = √π (via the Gaussian integral)

The Gamma function appears in the volume of n-dimensional balls (V_n = πⁿ/² R^n / Γ(n/2 + 1)), in the solutions of differential equations in spherical coordinates, and in quantum field theory.

In GR: the solid angle subtended by a 2-sphere (S²) in 3D space is 4π = 2π^{3/2}/Γ(3/2). The generalization to arbitrary dimensions (S^{n−1} in ℝⁿ) involves the Gamma function: the surface area of the unit (n−1)-sphere is 2π^{n/2}/Γ(n/2). This is needed in Kaluza-Klein theories and string theory. [Nakahara, M. (2003). *Geometry, Topology and Physics*, 2nd ed. IOP Publishing. Chapter 5.]

---

## 5.4.5 Cauchy Principal Value

Some integrals that are not properly defined as limits can be assigned a value by taking a symmetric limit:

$$\text{P.V.} \int_{-\infty}^{\infty} f(x) \, dx = \lim_{R \to \infty} \int_{-R}^{R} f(x) \, dx$$

or, for a singularity at c ∈ (a, b):

$$\text{P.V.} \int_a^b f(x) \, dx = \lim_{\varepsilon \to 0^+} \left[\int_a^{c-\varepsilon} f + \int_{c+\varepsilon}^b f\right]$$

**Example**: P.V. ∫_{−∞}^∞ x dx = lim_{R→∞} [x²/2]_{-R}^R = 0. But ∫_{−∞}^∞ x dx is not properly defined (both ∫₀^∞ x dx and ∫_{-∞}^0 x dx diverge).

The Cauchy principal value appears in the **Kramers-Kronig relations** (relating real and imaginary parts of response functions in physics) and in the proper treatment of singular integrals in quantum field theory.

---

## References

- Arfken, G.B., Weber, H.J., and Harris, F.E. (2013). *Mathematical Methods for Physicists*, 7th ed. Academic Press. [Chapter 2 on Gamma functions and Chapter 1 on integration techniques used in physics.]
- Nakahara, M. (2003). *Geometry, Topology and Physics*, 2nd ed. IOP Publishing. [Chapter 5 on homology and the n-sphere volumes involving Gamma functions.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 6 for convergence of improper integrals.]
- Whittaker, E.T. and Watson, G.N. (1927). *A Course of Modern Analysis*, 4th ed. Cambridge University Press. [Classic reference for the Gamma function and related special functions; Chapter 12.]
