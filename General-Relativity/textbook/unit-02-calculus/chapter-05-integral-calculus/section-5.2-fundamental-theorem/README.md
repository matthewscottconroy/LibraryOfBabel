# Section 5.2: The Fundamental Theorem of Calculus

---

## Section Introduction

The Fundamental Theorem of Calculus is the deepest result in elementary analysis. It reveals that differentiation and integration — defined by completely different limiting processes — are inverse operations. This connection is not obvious and is, historically, what made calculus a unified and powerful theory rather than a collection of isolated techniques.

Newton and Leibniz both discovered the Fundamental Theorem, and it is the reason they are credited with inventing calculus (rather than, say, Archimedes, who could compute areas, or Fermat, who could find tangent lines). The theorem is what links the two problems.

---

## 5.2.1 The Antiderivative

**Definition**: A function F is an **antiderivative** (or **primitive**) of f on (a, b) if F'(x) = f(x) for all x ∈ (a, b).

**Uniqueness up to constant**: If F and G are both antiderivatives of f on (a, b), then F − G is constant. (Proof: (F − G)' = f − f = 0; a function with zero derivative is constant, by the MVT.)

**Notation**: The **indefinite integral** ∫ f(x) dx denotes the family of all antiderivatives of f: F(x) + C, where C is an arbitrary constant.

**Examples**:
- ∫ xⁿ dx = xⁿ⁺¹/(n+1) + C (n ≠ −1)
- ∫ x⁻¹ dx = ln|x| + C
- ∫ eˣ dx = eˣ + C
- ∫ sin x dx = −cos x + C
- ∫ cos x dx = sin x + C
- ∫ 1/(1+x²) dx = arctan x + C

These follow immediately from the differentiation rules of Chapter 4.

---

## 5.2.2 The Fundamental Theorem: Part I

**Theorem** (FTC, Part I): Let f be integrable on [a, b] and continuous at c ∈ (a, b). Define:

$$F(x) = \int_a^x f(t) \, dt \quad \text{for } x \in [a, b]$$

Then F is differentiable at c and F'(c) = f(c).

**Proof**: We compute F'(c) from the definition:

$$\frac{F(c+h) - F(c)}{h} = \frac{1}{h} \int_c^{c+h} f(t) \, dt$$

Since f is continuous at c, for any ε > 0 there exists δ > 0 such that |t − c| < δ ⟹ |f(t) − f(c)| < ε. For |h| < δ:

$$\left| \frac{1}{h} \int_c^{c+h} f(t) \, dt - f(c) \right| = \left| \frac{1}{h} \int_c^{c+h} [f(t) - f(c)] \, dt \right| \leq \frac{1}{|h|} \cdot \varepsilon |h| = \varepsilon$$

(using the MVT for integrals and |h| cancellation). Taking h → 0: F'(c) = f(c). □

**Meaning**: The function F(x) = ∫ₐˣ f(t) dt is the "area accumulation function" — it accumulates area under the curve from a to x. The theorem says: the rate at which area accumulates at x is f(x). If f(x) is large (the curve is high), area accumulates fast; if f(x) is small, area accumulates slowly. This is intuitively obvious — and the theorem makes it precise.

---

## 5.2.3 The Fundamental Theorem: Part II

**Theorem** (FTC, Part II): Let f be continuous on [a, b] and let F be any antiderivative of f. Then:

$$\int_a^b f(x) \, dx = F(b) - F(a)$$

This is often written F(b) − F(a) = [F(x)]ₐᵇ.

**Proof**: Let G(x) = ∫ₐˣ f(t) dt. By FTC Part I, G'(x) = f(x) = F'(x). So F − G has zero derivative on (a, b), hence F(x) − G(x) = C for some constant. At x = a: G(a) = ∫ₐᵃ f dt = 0, so F(a) − 0 = C, meaning C = F(a). At x = b: F(b) − G(b) = F(a), so G(b) = F(b) − F(a). But G(b) = ∫ₐᵇ f(t) dt. □

**This theorem makes integration practical.** Instead of computing limits of Riemann sums (which is tedious except for the simplest functions), we simply find an antiderivative and evaluate it at the endpoints.

**Examples**:

$$\int_0^1 x^2 \, dx = \left[\frac{x^3}{3}\right]_0^1 = \frac{1}{3} - 0 = \frac{1}{3}$$

$$\int_0^\pi \sin x \, dx = [-\cos x]_0^\pi = (-\cos \pi) - (-\cos 0) = 1 + 1 = 2$$

$$\int_1^e \frac{1}{x} \, dx = [\ln x]_1^e = \ln e - \ln 1 = 1 - 0 = 1$$

---

## 5.2.4 The Connection to Physics

The Fundamental Theorem has a direct physical interpretation that goes beyond mathematics.

*Velocity and displacement*: If v(t) is the velocity of a particle, then the displacement over [t₁, t₂] is ∫_{t₁}^{t₂} v(t) dt = x(t₂) − x(t₁), where x is position. This is FTC Part II.

*Conservation laws*: Many conservation laws in physics have the form: the rate of change of a quantity equals the flux through the boundary. For a 1D region [a, b], this says dQ/dt = J(b) − J(a) = [J]ₐᵇ = ∫ₐᵇ dJ/dx dx. This is FTC. In higher dimensions, the generalization is Stokes' theorem (Chapter 7) and ultimately the divergence theorem and its differential geometry generalization — the generalized Stokes' theorem for differential forms on manifolds (Chapter 28).

*The action in GR*: The Einstein-Hilbert action S = ∫ R√(−g) d⁴x involves integration over a 4-dimensional region. The variation of S that yields Einstein's equations is a version of the FTC applied in 4 dimensions: boundary terms vanish (when fields fall off at infinity), and we are left with the Euler-Lagrange equations for the metric. The conceptual structure is identical to what we develop here.

---

## 5.2.5 Integration as the Inverse of Differentiation

Part I says: $\frac{d}{dx} \int_a^x f(t) \, dt = f(x)$. Differentiation undoes integration.

Part II says: $\int_a^b F'(x) \, dx = F(b) - F(a)$. Integration of the derivative gives back the original function (up to boundary evaluation).

These are not merely algebraically inverse — they express a deep duality between local (pointwise) and global (accumulated) information. The derivative is a local object; the integral is global. The Fundamental Theorem is the precise statement that they are inverse operations, connecting the local rate of change to the global accumulation.

In differential geometry, this duality appears as the relationship between differential forms and their integrals — the exterior derivative is the generalization of differentiation, and Stokes' theorem is the generalization of the FTC. In homological algebra, it appears as the relationship between coboundaries and boundaries. The idea reverberates through all of mathematics.

---

## References

- Apostol, T.M. (1967). *Calculus*, Vol. 1, 2nd ed. Wiley. [Chapters 5–6 on the integral and the fundamental theorem.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 6, Theorem 6.20 and 6.21 for FTC.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapters 14–15; Spivak's proof of the FTC is particularly clear, and his discussion of what the theorem *means* is superb.]
- Newton, I. (1671/1736). *Method of Fluxions and Infinite Series*. London. [Newton's original treatment, translated posthumously. The Fundamental Theorem appears here in the language of fluxions.]
- Leibniz, G.W. (1686). "De geometria recondita et analysi indivisibilium atque infinitorum." *Acta Eruditorum*, 292–300. [Leibniz's integral notation and his version of the Fundamental Theorem.]
