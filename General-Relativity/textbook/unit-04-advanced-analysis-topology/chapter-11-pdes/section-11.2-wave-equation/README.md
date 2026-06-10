# Section 11.2: The Wave Equation

---

## Section Introduction

The wave equation $\partial^2 u / \partial t^2 = c^2 \nabla^2 u$ is the prototype of a hyperbolic PDE. Its solutions are propagating waves — disturbances that travel at speed c without dissipation. The wave equation governs sound, light, quantum probability amplitudes, and, crucially for this textbook, **gravitational waves**: ripples in the curvature of spacetime that travel at the speed of light.

Understanding the wave equation deeply — its d'Alembert solution, Huygens' principle, energy conservation, and the role of characteristics — prepares us directly for linearized GR. The linearized Einstein equations in harmonic gauge are exactly coupled wave equations for the metric perturbation. Gravitational waves are the solutions.

---

## 11.2.1 The 1D Wave Equation: d'Alembert's Solution

The one-dimensional wave equation:

$$\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$$

**d'Alembert's solution** (1747): Introduce characteristic coordinates ξ = x − ct, η = x + ct. Then:
- ∂/∂x = ∂/∂ξ + ∂/∂η
- ∂/∂t = −c ∂/∂ξ + c ∂/∂η

The wave equation becomes ∂²u/∂ξ∂η = 0, which integrates to:

$$u(x, t) = f(x - ct) + g(x + ct)$$

for arbitrary functions f and g. f(x − ct) is a right-traveling wave; g(x + ct) is a left-traveling wave. Together they give the general solution.

**With Cauchy data** u(x, 0) = u₀(x) and u_t(x, 0) = u₁(x):

$$u(x, t) = \frac{1}{2}[u_0(x - ct) + u_0(x + ct)] + \frac{1}{2c}\int_{x-ct}^{x+ct} u_1(s) \, ds$$

This is the explicit formula for the solution — it shows:
1. The solution at (x, t) depends only on the initial data in the interval [x − ct, x + ct] (the **domain of dependence**).
2. The initial data at x₀ affects the solution only at points x with |x − x₀| ≤ ct (the **domain of influence**).
3. Signals travel at exactly speed c (the wave equation has **sharp wavefronts** in 1+1 dimensions).

**Connection to GR**: In Minkowski spacetime, the d'Alembert wave equation is $\Box u = 0$ where $\Box = -\partial_t^2/c^2 + \nabla^2$ is the **d'Alembertian** (or wave operator). In curved spacetime, the d'Alembertian generalizes to $\Box_g u = g^{\mu\nu}\nabla_\mu\nabla_\nu u$, where ∇ is the covariant derivative. The linearized Einstein equations take the form $\Box_g \bar{h}_{\mu\nu} = -16\pi G T_{\mu\nu}$ in harmonic gauge.

---

## 11.2.2 Separation of Variables

On a bounded domain (say, a string of length L), separation of variables solves the wave equation with boundary conditions.

For u(x, t), t ≥ 0, 0 ≤ x ≤ L, with u(0, t) = u(L, t) = 0 (fixed ends):

**Separation**: Try u(x, t) = X(x)T(t). Then T'' /T = c²X''/X = −λ (separation constant).

**The eigenvalue problem**: X'' + λX = 0, X(0) = X(L) = 0. Solutions: λ_n = (nπ/L)², X_n = sin(nπx/L), for n = 1, 2, 3, ...

**Time factor**: T'' + c²λ_n T = 0, so T_n(t) = A_n cos(ω_n t) + B_n sin(ω_n t) with ω_n = cnπ/L.

**General solution** (normal mode expansion):

$$u(x, t) = \sum_{n=1}^\infty \left[A_n \cos(\omega_n t) + B_n \sin(\omega_n t)\right] \sin\left(\frac{n\pi x}{L}\right)$$

The coefficients A_n, B_n are determined by the Fourier expansion of the initial data.

**Normal modes**: The n-th normal mode oscillates at frequency ω_n = cnπ/L with a sinusoidal spatial pattern. This is the mathematical basis for:
- **Musical acoustics**: the harmonics of a vibrating string (n = 1 is the fundamental; n = 2, 3, ... are overtones).
- **Quasi-normal modes of black holes**: the black hole has discrete "ringing frequencies" ω_n = ω_{Rn} − iω_{In} (complex, due to the open boundaries and energy loss to infinity). The discrete spectrum arises from the same separation of variables, now with outgoing-wave boundary conditions.

---

## 11.2.3 The 3D Wave Equation and Huygens' Principle

The 3-dimensional wave equation $u_{tt} = c^2 \nabla^2 u$ with Cauchy data u(x, 0) = u₀(x), u_t(x, 0) = u₁(x):

**Kirchhoff's formula** (1882):

$$u(\mathbf{x}, t) = \frac{\partial}{\partial t}\left[\frac{1}{4\pi c^2 t} \oint_{|\mathbf{y}-\mathbf{x}|=ct} u_0(\mathbf{y}) \, dS\right] + \frac{1}{4\pi c^2 t} \oint_{|\mathbf{y}-\mathbf{x}|=ct} u_1(\mathbf{y}) \, dS$$

The integrals are over the sphere of radius ct centered at x.

**Huygens' principle** (strict form in 3D): u(x, t) depends only on the initial data on the sphere |y − x| = ct (not in the interior). Disturbances travel at exactly speed c — there is no "wake." A flash of light at the origin illuminates a sphere expanding at speed c; after the sphere passes, there is darkness behind it.

**Contrast with 2D**: In 2 spatial dimensions, u(x, t) depends on initial data in the disk |y − x| ≤ ct (not just the boundary circle). There is a wake: even after the wavefront passes, the field remains disturbed. This is why a stone dropped in a pond leaves ripples that persist (2D), but an electromagnetic pulse in 3D is sharp.

**GR significance**: Gravitational waves propagate at exactly c in flat spacetime (Huygens' principle holds). In a region of strong curvature (near a black hole), waves propagate on and inside the light cone — there can be a "tail" (backscattering off the curvature). The gravitational wave signal from a binary merger therefore has both a sharp leading edge (at the light cone) and a trailing tail (from scattering off curvature). [Price, R.H. (1972). "Nonspherical perturbations of relativistic gravitational collapse. I." *Physical Review D*, 5, 2419–2438.]

---

## 11.2.4 Energy and Dispersion

**Energy for the wave equation**: Define the energy density $e = \frac{1}{2}(u_t^2 + c^2|\nabla u|^2)$ and energy flux J = −c² u_t ∇u. Then the energy continuity equation is:

$$\frac{\partial e}{\partial t} + \nabla \cdot \mathbf{J} = 0$$

The total energy E = ∫ e dV is conserved. This is the PDE analogue of energy conservation in mechanics.

**Fourier analysis**: Taking the Fourier transform û(k, t) of the wave equation gives:

$$\frac{\partial^2 \hat{u}}{\partial t^2} = -c^2 |k|^2 \hat{u}$$

Each Fourier mode oscillates at frequency ω = c|k| — the **dispersion relation** ω = ck. The wave speed ω/k = c is the same for all frequencies: the wave equation is **non-dispersive**. All Fourier components travel at the same speed; wavepackets do not spread.

**Dispersive waves**: If the dispersion relation is ω = ω(k) with ω/k depending on k (e.g., ω² = k² + m² for the Klein-Gordon equation), different Fourier modes travel at different speeds. Wavepackets spread. The group velocity v_g = dω/dk is the speed of information propagation.

**For gravitational waves in GR**: In vacuum, gravitational waves are non-dispersive (ω = ck exactly). Any observation of frequency-dependent wave speed would indicate new physics (massive gravitons, modifications of GR). The LIGO observation of GW150914 (the first gravitational wave detection) confirmed that the gravitational wave signal arrived simultaneously across different frequencies — consistent with massless gravitons and ω = ck, constraining the graviton mass to m_g < 1.2 × 10⁻²² eV/c². [Abbott, B.P. et al. (LIGO and Virgo) (2016). "Observation of gravitational waves from a binary black hole merger." *Physical Review Letters*, 116, 061102.]

---

## 11.2.5 The Wave Equation in Curved Spacetime

In curved spacetime with metric g_{μν}, the covariant wave equation for a scalar field φ is:

$$\Box_g \phi = g^{\mu\nu} \nabla_\mu \nabla_\nu \phi = \frac{1}{\sqrt{-g}} \partial_\mu(\sqrt{-g} g^{\mu\nu} \partial_\nu \phi) = 0$$

The factor √(−g) arises from the covariant volume element (Section 7.4). This equation is the wave equation modified by the curvature of spacetime.

**Near a Schwarzschild black hole**: In Schwarzschild coordinates, the wave equation for a massless scalar field becomes, after separation in spherical harmonics:

$$\frac{\partial^2 \Phi}{\partial r_*^2} - \frac{\partial^2 \Phi}{\partial t^2} = V_\ell(r) \Phi$$

where r* is the tortoise coordinate and V_ℓ(r) = (1 − r_s/r)[ℓ(ℓ+1)/r² + r_s/r³] is the effective potential (Section 10.4). This is a wave equation with a potential barrier — the same structure as the Schrödinger equation.

The transmission coefficient of the potential barrier determines how much of an incident wave passes through the potential versus reflects back. This **scattering matrix** encodes the black hole's response to external perturbations and is directly measured in gravitational wave observations.

---

## References

- d'Alembert, J.L.R. (1747). "Recherches sur la courbe que forme une corde tendue mise en vibration." *Mémoires de l'Académie Royale des Sciences et Belles Lettres de Berlin*, 214–249. [The first paper solving the wave equation; introduces d'Alembert's formula and separates the solution into traveling waves.]
- Kirchhoff, G. (1882). "Zur Theorie der Lichtstrahlen." *Annalen der Physik*, 18, 663–695. [Kirchhoff's formula for the 3D wave equation; rigorous derivation of Huygens' principle.]
- Abbott, B.P. et al. (LIGO Scientific Collaboration and Virgo Collaboration) (2016). "Observation of gravitational waves from a binary black hole merger." *Physical Review Letters*, 116, 061102. [The discovery paper for gravitational waves. The dispersion constraint on the graviton mass appears in the supplemental material.]
- Evans, L.C. (2010). *Partial Differential Equations*, 2nd ed. AMS. [Chapter 2.4: the wave equation; d'Alembert's formula; energy methods; Huygens' principle.]
- Price, R.H. (1972). "Nonspherical perturbations of relativistic gravitational collapse. I–II." *Physical Review D*, 5, 2419–2454. [Price's law: late-time tails of gravitational wave signals decay as t^{−(2ℓ+2)}, due to backscattering off the Schwarzschild potential. A quantitative result about wave propagation in curved spacetime.]
