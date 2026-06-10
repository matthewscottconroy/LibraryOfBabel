# Section 12.4: Applications of Complex Analysis to Physics and GR

---

## Section Introduction

The tools of complex analysis — contour integrals, the residue theorem, conformal maps, analytic continuation — are not merely useful techniques for evaluating integrals. They reveal deep connections between the analytic structure of physical quantities (where their poles and branch cuts are) and the physics those quantities encode.

This section collects the most important applications of complex analysis in theoretical physics, with particular emphasis on those that directly enter GR and quantum field theory in curved spacetime.

---

## 12.4.1 Real Integrals by Contour Integration

The residue theorem converts contour integrals in ℂ to sums of residues. By closing a real integral as a contour in ℂ and invoking Cauchy's theorem, many real integrals become elementary.

**Type 1: ∫₋∞^∞ R(x) dx** where R(x) = P(x)/Q(x), deg Q ≥ deg P + 2.

Close in the upper half-plane: add a large semicircle Γ_R of radius R. For |z| large, |R(z)| ≤ C/|z|² → 0, so ∫_{Γ_R} R dz → 0 by the ML estimate. Applying the residue theorem:

$$\int_{-\infty}^\infty R(x) dx = 2\pi i \sum_{\text{Im}(z_k)>0} \text{Res}_{z=z_k} R(z)$$

**Type 2: ∫₋∞^∞ R(x) e^{iax} dx** with a > 0.

By Jordan's lemma, the integral over the large semicircle in the upper half-plane vanishes (e^{iaz} = e^{ia(x+iy)} = e^{iax}e^{-ay} decays in the upper half-plane for a > 0). The result is the Fourier transform of R evaluated at frequency a.

**Example** (Yukawa potential): The Fourier transform of 1/(p² + m²) in momentum space gives the Yukawa potential e^{−mr}/r in position space — the exponentially screened gravitational/electrostatic potential due to a massive mediator.

$$\int_{-\infty}^\infty \frac{e^{ipx}}{p^2 + m^2} dp = \frac{\pi}{m} e^{-m|x|}$$

Evaluated via residues: poles at p = ±im; for x > 0 close in the upper half-plane; residue at p = +im gives the result.

**Type 3: ∫₀^{2π} R(cos θ, sin θ) dθ**

Substitute z = e^{iθ}, cos θ = (z + z⁻¹)/2, sin θ = (z − z⁻¹)/(2i), dθ = dz/(iz). The integral becomes a contour integral around the unit circle.

---

## 12.4.2 Dispersion Relations (Kramers-Kronig)

A **dispersion relation** connects the real and imaginary parts of a response function via the Cauchy integral formula. These are physically fundamental: they encode causality.

**Setup**: A **response function** χ(ω) describes how a system responds at frequency ω to an applied force. Causality requires χ̃(t) = 0 for t < 0 (no response before the force is applied). In the frequency domain, causality implies χ(ω) is holomorphic in the upper half-plane Im(ω) > 0.

**Kramers-Kronig relations** (1926–1927): By the Cauchy integral formula, for χ holomorphic in the upper half-plane with χ(ω) → 0 as |ω| → ∞:

$$\chi(\omega) = \frac{1}{\pi i} \text{P.V.} \int_{-\infty}^\infty \frac{\chi(\omega')}{\omega' - \omega} d\omega'$$

Taking real and imaginary parts:

$$\text{Re}[\chi(\omega)] = \frac{1}{\pi} \text{P.V.} \int_{-\infty}^\infty \frac{\text{Im}[\chi(\omega')]}{\omega' - \omega} d\omega'$$
$$\text{Im}[\chi(\omega)] = -\frac{1}{\pi} \text{P.V.} \int_{-\infty}^\infty \frac{\text{Re}[\chi(\omega')]}{\omega' - \omega} d\omega'$$

**Physical meaning**: The real part (dispersion — how the medium changes wave speed) is completely determined by the imaginary part (absorption — how the medium dissipates energy), and vice versa. These relations are a consequence of causality alone.

**GR application**: The **quasi-normal modes** of a black hole have complex frequencies ω_n = ω_{Rn} − iω_{In}. They are the poles of the **black hole scattering matrix** (the Green's function in the frequency domain). Dispersion relations for the scattering matrix encode the causal structure of the black hole spacetime — information about what happens inside can in principle be inferred from the structure of the quasi-normal mode spectrum. [Leung, P.T. et al. (1997). "Completeness and orthogonality of quasinormal modes in leaky optical cavities." *Physical Review A*, 49, 3068.]

---

## 12.4.3 The Laplace Transform and ODEs

The **Laplace transform** is a complex analysis tool for solving ODEs with initial conditions:

$$\mathcal{L}[f](s) = \int_0^\infty f(t) e^{-st} dt, \quad \text{Re}(s) > s_0$$

Key property: $\mathcal{L}[f'](s) = s\mathcal{L}[f](s) - f(0)$. This converts differentiation to multiplication by s.

**Solving ODEs**: The ODE y'' + py' + qy = r(t), y(0) = y₀, y'(0) = y₁ becomes:

$$(s^2 + ps + q)Y(s) = R(s) + (s + p)y_0 + y_1$$

where Y = ℒ[y] and R = ℒ[r]. This is an algebraic equation for Y(s). The solution y(t) is the inverse Laplace transform:

$$y(t) = \frac{1}{2\pi i} \int_{c-i\infty}^{c+i\infty} Y(s) e^{st} ds$$

(a contour integral in the complex s-plane, called the Bromwich integral). The poles of Y(s) are the quasi-normal modes of the ODE.

**GR application**: The Laplace transform is used to analyze the response of spacetime to perturbations in the frequency domain. The black hole "transfer function" from source to gravitational wave output is the Laplace transform of the Green's function, and its poles are the quasi-normal mode frequencies.

---

## 12.4.4 Conformal Maps and Penrose Diagrams

A **conformal map** f: U → V is a holomorphic bijection with f'(z) ≠ 0. It preserves angles but distorts distances by a position-dependent factor |f'(z)|.

**Application to potential theory**: If u is harmonic on V (∇²u = 0) and f: U → V is a conformal map, then u ∘ f is harmonic on U. This allows boundary value problems in complicated domains to be reduced to simpler ones.

**Conformal compactification** in GR: The **Penrose diagram** is a conformal map that brings the infinite extent of spacetime into a finite diagram. The transformation is:

For Minkowski spacetime in coordinates (t, r):
- Define null coordinates u = t − r, v = t + r.
- Apply U = arctan(u), V = arctan(v) (compactification).
- New "time" T = V + U and "radius" R = V − U.

The entire Minkowski spacetime maps to the finite triangle 0 ≤ R ≤ π, |T| ≤ π − R in (T, R) space. The metric becomes ds² = (overall conformal factor) × (−dT² + dR² + sin²R dΩ²). The boundaries of the diagram represent:
- i⁰: spacelike infinity (r → ∞ at fixed t)
- i⁺, i⁻: future/past timelike infinity (t → ±∞ at fixed r)
- ℐ⁺, ℐ⁻: future/past null infinity (the boundary of the spacetime for outgoing/incoming light rays)

The conformal factor doesn't affect causal structure (light cones are preserved by conformal transformations), so the Penrose diagram displays the causal structure of spacetime exactly, while fitting infinite spacetime into a finite picture.

**Penrose diagram for Schwarzschild**: The Kruskal-Szekeres coordinates map the Schwarzschild metric, and conformal compactification of Kruskal coordinates gives the Penrose diagram. It reveals: two exterior regions (connected by a non-traversable wormhole), a future singularity, and a past singularity. [Penrose, R. (1963). "Asymptotic properties of fields and spacetimes." *Physical Review Letters*, 10, 66–68.]

---

## 12.4.5 Zeta Function Regularization

In quantum field theory in curved spacetime, the zero-point energy (vacuum energy) is a divergent sum:

$$E_{\text{vac}} = \frac{\hbar}{2} \sum_{n=1}^\infty \omega_n$$

For a quantum field in a box, ω_n = nπc/L (normal mode frequencies). The sum diverges:

$$\sum_{n=1}^\infty n = 1 + 2 + 3 + \cdots = ?$$

**Zeta function regularization**: Define the spectral zeta function:

$$\zeta_L(s) = \sum_{n=1}^\infty \frac{1}{\omega_n^s}$$

This converges for Re(s) large. By analytic continuation (using the Hurwitz/Riemann zeta function), it extends to a meromorphic function on ℂ. The regularized vacuum energy is:

$$E_{\text{vac}} = \frac{\hbar}{2} \zeta_L(-1) = \frac{\hbar}{2} \cdot \zeta(-1) \cdot \left(\frac{\pi c}{L}\right)^{-(-1)} = \frac{\hbar \pi c}{2L} \cdot \zeta(-1)$$

where ζ(s) is the Riemann zeta function. Using the analytic continuation ζ(−1) = −1/12:

$$E_{\text{vac}} = -\frac{\hbar \pi c}{24 L}$$

**Casimir effect**: The force between two conducting plates separated by distance L is F = −∂E_{vac}/∂L = −ℏπc/(24L²) per unit area. This attractive force between uncharged plates — arising from quantum vacuum fluctuations constrained by the boundary conditions — was predicted by Casimir (1948) and measured experimentally (Lamoreaux, 1997). The calculation uses complex analysis (analytic continuation of the zeta function) to give a finite physical result from a formally divergent sum.

**In GR**: The same zeta function regularization appears in the one-loop quantum corrections to the Einstein-Hilbert action in curved spacetime. The DeWitt-Seeley heat kernel expansion gives the divergent terms, and zeta regularization (analytic continuation in the parameter s of the heat kernel) extracts finite physical predictions.

[Casimir, H.B.G. (1948). "On the attraction between two perfectly conducting plates." *Proceedings of the Koninklijke Nederlandse Akademie van Wetenschappen*, 51, 793–795.]

---

## References

- Ahlfors, L.V. (1979). *Complex Analysis*, 3rd ed. McGraw-Hill. [The complete reference for complex analysis techniques, including contour integration, the residue theorem, and conformal maps.]
- Toll, J.S. (1956). "Causality and the dispersion relation: Logical foundations." *Physical Review*, 104, 1760–1770. [A careful derivation of the Kramers-Kronig relations from causality, making explicit the connection to analyticity in the upper half-plane.]
- Penrose, R. (1963). "Asymptotic properties of fields and spacetimes." *Physical Review Letters*, 10, 66–68. [Introduces conformal infinity (ℐ⁺ and ℐ⁻) and the Penrose diagram.]
- Hawking, S.W. and Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time.* Cambridge University Press. [Chapter 5: conformal transformation and the causal structure of spacetime; Penrose diagrams for all standard exact solutions.]
- Casimir, H.B.G. (1948). "On the attraction between two perfectly conducting plates." *Proceedings of the Koninklijke Nederlandse Akademie van Wetenschappen*, 51, 793–795. [The original Casimir effect paper; the calculation using mode sums regularized by zeta continuation.]
- Hawking, S.W. (1975). "Particle creation by black holes." *Communications in Mathematical Physics*, 43, 199–220. [Hawking's derivation of black hole radiation, using the Green's function of the wave equation analytically continued across the horizon — a direct application of the analytic continuation techniques of this chapter.]
