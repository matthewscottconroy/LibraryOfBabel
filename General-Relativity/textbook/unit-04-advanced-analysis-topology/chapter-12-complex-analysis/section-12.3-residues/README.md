# Section 12.3: Laurent Series, Poles, and the Residue Theorem

---

## Section Introduction

Not all complex functions are holomorphic everywhere — singularities arise where the function is undefined or blows up. The **Laurent series** extends Taylor series to include negative powers, capturing the behavior near singularities. The **residue theorem** converts contour integrals around singularities into simple algebra — computing the "residue" (the coefficient of 1/(z−z₀)) at each pole. This transforms many otherwise intractable real integrals into elementary calculations.

In GR, residues and contour integrals appear in: Green's functions (propagators), the quasi-normal mode spectrum of black holes (poles in the frequency domain), Hawking radiation (analytic continuation across a singularity of the metric), and the zeta function regularization of quantum field theory.

---

## 12.3.1 Laurent Series

**Theorem**: If f is holomorphic on the annulus r < |z − z₀| < R, then f has a unique expansion:

$$f(z) = \sum_{n=-\infty}^{\infty} c_n (z - z_0)^n$$

called a **Laurent series**. The coefficients are:

$$c_n = \frac{1}{2\pi i} \oint_{|z-z_0|=\rho} \frac{f(z)}{(z-z_0)^{n+1}} dz \quad (r < \rho < R)$$

The **principal part** of the Laurent series is the sum of terms with n < 0.

**Classification of isolated singularities** at z₀:
- **Removable singularity**: principal part = 0 (Laurent series has no negative powers). f is bounded near z₀; the singularity disappears if we define f(z₀) = c₀.
- **Pole of order m**: principal part has finitely many terms; the lowest power is 1/(z−z₀)^m. Near a pole: f(z) ~ c_{-m}/(z−z₀)^m + ⋯ + c_{-1}/(z−z₀) + c₀ + ⋯.
- **Essential singularity**: principal part has infinitely many terms. By Picard's great theorem: near an essential singularity, f takes every complex value (with at most one exception) in every punctured disk.

---

## 12.3.2 Residues

**Definition**: The **residue** of f at an isolated singularity z₀ is the coefficient c_{-1} of the Laurent series:

$$\text{Res}_{z=z_0} f(z) = c_{-1}$$

**Computing residues**:
- For a **simple pole** (order 1): Res = lim_{z→z₀} (z−z₀)f(z).
- For a **pole of order m**: Res = lim_{z→z₀} [d^{m-1}/dz^{m-1} (z−z₀)^m f(z)]/(m−1)!.
- For f = g/h with g(z₀) ≠ 0 and h(z₀) = 0, h'(z₀) ≠ 0 (simple pole): Res = g(z₀)/h'(z₀).

---

## 12.3.3 The Residue Theorem

**Theorem** (Residue Theorem): If f is meromorphic on and inside a simple closed contour C (except for finitely many poles z₁, ..., zₙ inside C), then:

$$\oint_C f(z) \, dz = 2\pi i \sum_{k=1}^n \text{Res}_{z=z_k} f(z)$$

*Proof*: By Cauchy's theorem, the integral over C equals the sum of integrals over small circles around each pole. Each small circle integral equals 2πi times the residue by the Laurent series formula for c_{-1}. □

**Example** (real integral via residues): Compute $\int_{-\infty}^\infty \frac{dx}{1 + x^4}$.

Extend to a contour in the upper half-plane: close with a large semicircle. The poles of 1/(1+z⁴) are at z = e^{iπ(2k+1)/4} for k = 0, 1, 2, 3. Those in the upper half-plane are z₁ = e^{iπ/4} and z₂ = e^{3iπ/4}.

Residue at z₁: 1/(4z₁³) = e^{-3iπ/4}/4. Residue at z₂: 1/(4z₂³) = e^{-9iπ/4}/4 = e^{-iπ/4}/4.

Sum of residues: (e^{-3iπ/4} + e^{-iπ/4})/4 = (−(1+i)/√2 + (1+i)/√2 ... let me compute directly: e^{-3iπ/4} + e^{-iπ/4} = −1/√2 − i/√2 + 1/√2 − i/√2 = −2i/√2 = −i√2.

Integral = 2πi · (−i√2)/4 = 2π√2/4 = π/√2. ✓

---

## 12.3.4 Analytic Continuation and Hawking Radiation

**Analytic continuation**: If f is holomorphic on domain D and g is holomorphic on D' ⊃ D with g = f on D, then g is the **analytic continuation** of f. By the identity theorem, the analytic continuation is unique.

This allows extending real-variable functions to larger domains in ℂ. For instance, the Gamma function Γ(s) = ∫₀^∞ t^{s-1} e^{-t} dt is defined for Re(s) > 0, but can be analytically continued to all of ℂ except the non-positive integers.

**Hawking radiation via analytic continuation**: The Schwarzschild metric in Euclidean signature (t → iτ) becomes:

$$ds^2 = \left(1 - \frac{r_s}{r}\right) d\tau^2 + \left(1 - \frac{r_s}{r}\right)^{-1} dr^2 + r^2 d\Omega^2$$

Near r = rₛ, this looks like flat space in polar coordinates (R, θ) with R² ≈ (r − rₛ)/(rₛ/4) and θ = τ/(2rₛ). Flat Euclidean space has θ with period 2π. So τ must have period 4πrₛ.

In quantum field theory, a field theory in Euclidean space with imaginary time of period β = 1/(k_B T) corresponds to a thermal state at temperature T. So the Hawking temperature is:

$$T_H = \frac{\hbar c^3}{8\pi G M k_B}$$

This derivation — analytic continuation of the metric and periodicity of imaginary time — is due to Gibbons and Hawking (1977). [Gibbons, G.W. and Hawking, S.W. (1977). "Action integrals and partition functions in quantum gravity." *Physical Review D*, 15, 2752–2756.]

The complex analysis tool — identifying periodicity in imaginary time with temperature — connects to the Matsubara formalism in condensed matter physics and quantum field theory at finite temperature.

---

## References

- Ahlfors, L.V. (1979). *Complex Analysis*, 3rd ed. McGraw-Hill. [Chapters 4–5 on Laurent series and residues.]
- Gibbons, G.W. and Hawking, S.W. (1977). "Action integrals and partition functions in quantum gravity." *Physical Review D*, 15, 2752–2756. [Hawking temperature via Euclidean analytic continuation.]
- Wald, R.M. (1984). *General Relativity*. University of Chicago Press. [Appendix on analytic continuation and the Unruh/Hawking effects.]
