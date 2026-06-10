# Section 24.2: The Ricci Tensor, Einstein Tensor, and the Field Equations

---

## Section Introduction

The Riemann tensor has 20 independent components in 4D — far more information than appears in the Einstein equations. The field equations involve only the **Ricci tensor** $R_{\mu\nu}$ (10 components) and the **Ricci scalar** $R$ (1 component). The remaining information (10 components of the Weyl tensor) propagates freely as gravitational waves.

This section defines the Ricci tensor and scalar, derives the contracted Bianchi identity, and presents the Einstein equations with their physical interpretation. The Einstein equations are not derived here — their derivation from the Einstein-Hilbert action is in Unit IX (Section 27.1). What we present here is the mathematical structure: what the equations say, and why the specific combination $G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$ (the Einstein tensor) is the right left-hand side.

---

## 24.2.1 The Ricci Tensor

**Definition**: The Ricci tensor is the trace of the Riemann tensor:

$$R_{\mu\nu} = R^\rho_{\ \mu\rho\nu}$$

(contraction of the first and third indices). This is a symmetric tensor: $R_{\mu\nu} = R_{\nu\mu}$ (follows from the symmetries of the Riemann tensor).

**Explicit formula**:

$$R_{\mu\nu} = \partial_\rho\Gamma^\rho_{\nu\mu} - \partial_\nu\Gamma^\rho_{\rho\mu} + \Gamma^\rho_{\rho\lambda}\Gamma^\lambda_{\nu\mu} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\rho\mu}$$

**Geometric interpretation**: The Ricci tensor measures the volume deviation of geodesics. A sphere of freely falling particles, if $R_{\mu\nu} \neq 0$ in some direction, will change its volume (focus or defocus) in that direction. This is the Raychaudhuri equation: $d\theta/d\tau = -\frac{1}{3}\theta^2 - \sigma_{\mu\nu}\sigma^{\mu\nu} + \omega_{\mu\nu}\omega^{\mu\nu} - R_{\mu\nu}u^\mu u^\nu$, where $\theta$ is the expansion (volume divergence of the geodesic congruence). The term $-R_{\mu\nu}u^\mu u^\nu$ is the geodesic focusing by matter — if the **energy conditions** hold (requiring $R_{\mu\nu}u^\mu u^\nu \geq 0$ for timelike $u^\mu$), the Ricci tensor always focuses geodesics. This is the key input to the Penrose-Hawking singularity theorems.

**In vacuum**: Einstein's vacuum equations are $R_{\mu\nu} = 0$ (no matter sources). A vacuum spacetime has vanishing Ricci tensor but can have nonzero Weyl curvature (e.g., the Schwarzschild and Kerr black hole solutions, gravitational waves). The 20 components of the Riemann tensor split into 10 (Ricci, constrained by vacuum equations to be zero) + 10 (Weyl, freely propagating).

---

## 24.2.2 The Ricci Scalar

**Definition**: The Ricci scalar (or scalar curvature) is the trace of the Ricci tensor:

$$R = g^{\mu\nu}R_{\mu\nu}$$

**Geometric interpretation**: For a small geodesic ball of radius $\varepsilon$ in $n$ dimensions, the volume is:

$$V = V_{\rm flat}\left(1 - \frac{R}{6(n+2)}\varepsilon^2 + O(\varepsilon^4)\right)$$

where $V_{\rm flat} = \omega_n\varepsilon^n$ is the flat-space volume. Positive $R$ means the ball is smaller than the flat-space prediction — curvature "focuses" geodesics. Negative $R$ means larger.

**Examples**:
- Unit 2-sphere $S^2$ (radius 1): $R = 2$ (positive, uniform curvature)
- Hyperbolic plane: $R = -2$ (negative, uniform curvature)
- Minkowski spacetime: $R = 0$
- Schwarzschild spacetime: $R = 0$ everywhere (vacuum solution of Einstein equations)
- FLRW cosmology: $R = -6(\ddot{a}/a + \dot{a}^2/a^2 + k/a^2)$ (varies with time)

---

## 24.2.3 The Einstein Tensor and Contracted Bianchi Identity

The **Einstein tensor** is defined as:

$$G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$$

It is symmetric ($G_{\mu\nu} = G_{\nu\mu}$) and has 10 independent components.

**The contracted Bianchi identity**: From the second Bianchi identity $\nabla_{[\lambda}R_{\rho\sigma]\mu\nu} = 0$, contracting twice:

$$\nabla_\mu G^{\mu\nu} = 0$$

*Proof*: Contract the Bianchi identity $\nabla_\lambda R_{\rho\sigma\mu\nu} + \nabla_\rho R_{\sigma\lambda\mu\nu} + \nabla_\sigma R_{\lambda\rho\mu\nu} = 0$ with $g^{\lambda\mu}$:

$$\nabla^\mu R_{\rho\sigma\mu\nu} + \nabla_\rho R_{\sigma\nu} - \nabla_\sigma R_{\rho\nu} = 0$$

Using $\nabla^\mu R_{\rho\sigma\mu\nu} = -\nabla^\mu R_{\sigma\rho\mu\nu} = \nabla^\mu R_{\mu\nu\rho\sigma}$ (symmetries), contract again with $g^{\rho\nu}$:

$$\nabla^\mu R_{\mu\nu} + \nabla_\nu R_{\mu}^{\ \mu} - \nabla^\mu R_{\mu\nu} = 0$$
$$2\nabla^\mu R_{\mu\nu} - \nabla_\nu R = 0$$
$$\nabla^\mu\left(R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R\right) = \nabla^\mu G_{\mu\nu} = 0 \qquad \square$$

This is an identity — true for any metric, regardless of whether Einstein's equations are satisfied.

**Physical significance**: The Einstein tensor $G_{\mu\nu}$ is the unique symmetric, divergence-free, rank-2 tensor built from the metric and its first two derivatives (up to a cosmological constant term). This uniqueness — proved by Lovelock (1971) — is what makes the Einstein equations essentially unique in 4D.

---

## 24.2.4 The Einstein Field Equations

The **Einstein field equations** are:

$$G_{\mu\nu} = 8\pi G T_{\mu\nu}$$

(in units $c = 1$). In full units: $G_{\mu\nu} = (8\pi G/c^4) T_{\mu\nu}$.

**The constant $8\pi G$**: Fixed by requiring that the Newtonian limit $\nabla^2\Phi = 4\pi G\rho$ is recovered. The factor of $8\pi G$ is determined by the normalization of $G_{\mu\nu}$ and the requirement that GR reduces to Newton in the appropriate limit.

**Content**: The equations say that the curvature of spacetime (left side, a geometric object) equals the stress-energy content of matter (right side, a physical object). This is the fundamental statement of GR: **matter tells spacetime how to curve; curved spacetime tells matter how to move.**

**Covariance**: $\nabla_\mu G^{\mu\nu} = 0$ (an identity) implies $\nabla_\mu T^{\mu\nu} = 0$ — local conservation of energy and momentum follows automatically from the Einstein equations. (See Section 20.2.2.)

**10 equations, 10 unknowns**: The metric $g_{\mu\nu}$ has 10 independent components. The Einstein equations are 10 equations. But 4 of them are constraints (the Bianchi identity removes 4 degrees of freedom), leaving 6 independent evolution equations. Of the 10 metric components, 4 are pure gauge (coordinate freedom = 4 functions), leaving 2 physical degrees of freedom — the two polarizations of gravitational waves.

**The vacuum Einstein equations**: $T_{\mu\nu} = 0$ gives $G_{\mu\nu} = 0$, equivalently $R_{\mu\nu} = 0$ (since $R_{\mu\nu} = 0 \Leftrightarrow R = 0 \Leftrightarrow G_{\mu\nu} = 0$ in vacuum). The Schwarzschild and Kerr metrics satisfy $R_{\mu\nu} = 0$ everywhere outside the central mass.

---

## 24.2.5 The Cosmological Constant

Einstein originally derived the field equations as $G_{\mu\nu} = 8\pi G T_{\mu\nu}$. However, motivated by the desire for a static universe (which is impossible without modification), he added the **cosmological constant** $\Lambda$:

$$G_{\mu\nu} + \Lambda g_{\mu\nu} = 8\pi G T_{\mu\nu}$$

The $\Lambda g_{\mu\nu}$ term is consistent with $\nabla_\mu G^{\mu\nu} = 0$ since $\nabla_\mu(g^{\mu\nu}) = 0$ (metric compatibility). Lovelock's theorem allows this term. Physically, it acts as a constant energy density and negative pressure: $\rho_\Lambda = \Lambda/(8\pi G)$, $p_\Lambda = -\rho_\Lambda$.

**The discovery of cosmic acceleration**: In 1998, supernova surveys showed that the expansion of the universe is accelerating [Perlmutter et al. (1999); Riess et al. (1998)]. This requires $\Lambda > 0$ (or some equivalent "dark energy"). The observed value $\Lambda \approx 1.1\times10^{-52}$ m$^{-2}$ corresponds to an energy density $\rho_\Lambda \approx 6.9\times10^{-27}$ kg/m$^3$ — about 68% of the total energy budget of the universe.

**The cosmological constant problem**: Quantum field theory predicts $\rho_\Lambda \sim (E_{\rm Planck}/c^2)^4 \approx 5\times10^{96}$ kg/m$^3$ from vacuum fluctuations — roughly $10^{123}$ times larger than observed. This factor-of-$10^{123}$ discrepancy is the worst prediction in the history of physics, and one of the deepest unsolved problems.

---

## 24.2.6 Counting and Classifying Solutions

The Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ are 10 coupled, nonlinear PDEs. Finding exact solutions is extremely difficult. Strategies:

**Symmetry reduction**: Impose symmetry (spherical, axial, homogeneous and isotropic) to reduce to ODEs. This gives the Schwarzschild, Kerr, FLRW, and Kasner solutions.

**Linearization** (gravitational waves): Write $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$ with $h_{\mu\nu}$ small, and linearize. The linearized Einstein equations are wave equations for $h_{\mu\nu}$, which propagate at speed $c$ — gravitational waves.

**Numerical relativity**: For two merging black holes or neutron stars (no useful symmetry), integrate the full nonlinear PDEs numerically. This is the computational backbone of LIGO waveform templates. The first successful binary black hole merger simulation was by Pretorius (2005).

**Perturbation theory**: Expand around known exact solutions. The Regge-Wheeler (1957) and Teukolsky (1973) equations govern perturbations of the Schwarzschild and Kerr solutions, respectively.

---

## References

- Einstein, A. (1915). "Die Feldgleichungen der Gravitation." *Sitzungsberichte der Königlich Preußischen Akademie der Wissenschaften*, 844–847. [Einstein's paper presenting the field equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ on November 25, 1915 — the culmination of a decade of work.]
- Hilbert, D. (1915). "Die Grundlagen der Physik." *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen*, 395–407. [Derived the field equations independently via the variational principle (Einstein-Hilbert action) on November 20, 1915 — five days before Einstein's own paper. The attribution of priority remains debated.]
- Lovelock, D. (1971). "The Einstein tensor and its generalizations." *Journal of Mathematical Physics*, 12, 498–501. [Proves that the Einstein tensor (plus cosmological constant) is the unique symmetric, divergence-free, rank-2 tensor constructed from the metric and its first two derivatives in 4D.]
- Perlmutter, S. et al. (1999). "Measurements of $\Omega$ and $\Lambda$ from 42 high-redshift supernovae." *Astrophysical Journal*, 517, 565–586. [Discovery of cosmic acceleration with Type Ia supernovae. Led to the 2011 Nobel Prize in Physics.]
- Pretorius, F. (2005). "Evolution of binary black-hole spacetimes." *Physical Review Letters*, 95, 121101. [The first successful numerical simulation of two black holes merging — the breakthrough that made LIGO waveform templates possible.]
