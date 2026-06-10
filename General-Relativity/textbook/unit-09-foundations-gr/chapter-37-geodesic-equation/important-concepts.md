# Chapter 37: Important Concepts

---

**Geodesic**
A curve on a manifold that parallel-transports its own tangent vector: $Du^\mu/d\tau = u^\nu\nabla_\nu u^\mu = 0$. For a Lorentzian manifold: timelike geodesics are worldlines of freely falling massive particles (maximum proper time); null geodesics are worldlines of light (affinely parameterized, zero interval); spacelike geodesics are instantaneous spatial curves (minimum proper distance, not physically realized by particles).

**Geodesic Equation**
$\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$: the equation of motion for a freely falling test body in curved spacetime. Contains no reference to the particle's mass — all freely falling bodies follow the same geodesic, regardless of their properties. This is the geometric expression of the equivalence principle.

**Geodesic Hypothesis (Theorem)**
The equations of motion for small test bodies follow from the Einstein equations — they need not be postulated separately. The center of mass of a test body follows a geodesic to leading order in its mass. Proved rigorously by Gralla and Wald (2008) for finite-size bodies.

**Affine Parameter**
For null geodesics, proper time is not defined ($d\tau^2 = 0$). An affine parameter $\lambda$ is defined by the condition that the geodesic equation holds without a non-affine term: $d^2x^\mu/d\lambda^2 + \Gamma^\mu_{\nu\rho}(dx^\nu/d\lambda)(dx^\rho/d\lambda) = 0$. Any two affine parameters are related by $\lambda' = a\lambda + b$.

**Killing Vectors and Conserved Quantities**
If $\xi^\mu$ is a Killing vector ($\nabla_{(\mu}\xi_{\nu)} = 0$), then $Q = g_{\mu\nu}\xi^\mu\dot{x}^\nu$ is conserved along geodesics. Schwarzschild has 4 Killing vectors: time translation (conserved energy $E$) and 3 rotations (conserved angular momentum $L$). Kerr has the same 4 plus a hidden (Carter constant $C$).

**Photon Sphere**
Unstable circular null geodesic in Schwarzschild at $r = 3GM/c^2 = 1.5\,r_s$. The image of the photon sphere (scattered toward a distant observer) forms the bright ring around the black hole shadow seen in EHT images. The innermost stable circular orbit (ISCO) for massive particles is at $r = 6GM/c^2 = 3\,r_s$.

**Geodesic Completeness**
A spacetime is geodesically complete if every geodesic can be extended to arbitrarily large affine parameter. Geodesically incomplete spacetimes contain singularities (where geodesics terminate at finite parameter value). The Penrose-Hawking singularity theorems show GR generically produces incomplete spacetimes.

**Newtonian Limit**
The geodesic equation reduces to Newton's second law $d^2\mathbf{x}/dt^2 = -\nabla\Phi$ when: $v \ll c$ (slow motion), $|h_{\mu\nu}| \ll 1$ (weak field), time derivatives negligible. The identification $g_{00} = -(1 + 2\Phi/c^2)$ connects the metric perturbation to the Newtonian potential.

**Post-Newtonian Expansion**
Systematic expansion of the geodesic equation in powers of $v/c \sim (GM/rc^2)^{1/2}$. 1PN corrections give gravitomagnetic effects (Lense-Thirring, geodetic precession); 2PN give further corrections; 2.5PN give radiation damping (orbital inspiral). Templates for LIGO gravitational waveforms use 3.5PN phase evolution.

**Gravitoelectromagnetism (GEM)**
In the weak-field limit, the linearized Einstein equations take the form of Maxwell-like equations for gravitoelectric field $\mathbf{g} = -\nabla\Phi$ and gravitomagnetic field $\mathbf{H}$ (from the $g_{0i}$ metric components). The gravitomagnetic force on a slowly-moving particle is $4\times$ larger than the EM analogy would suggest — a consequence of gravity being spin-2.

**Lense-Thirring Precession**
A gyroscope in the gravitomagnetic field of a rotating body precesses at $\boldsymbol\Omega_{\rm LT} = G[3(\mathbf{J}\cdot\hat{r})\hat{r} - \mathbf{J}]/(c^2r^3)$. Confirmed by Gravity Probe B (2011): $37.2\pm 7.2$ mas/yr (predicted: $39.2$ mas/yr). Also observed indirectly in the Lense-Thirring orbital precession of the LAGEOS satellites.

**Geodetic Precession**
A gyroscope orbiting a massive body precesses due to the curvature of the spatial part of the metric: $\boldsymbol\Omega_{\rm dS} = \frac{3}{2}GM(\mathbf{v}\times\hat{r})/(c^2r^2)$. For Earth: $6606$ mas/yr. Confirmed by Gravity Probe B to $0.28\%$ precision.

**Mathisson-Papapetrou-Dixon Equations**
Equations of motion for a spinning extended body in curved spacetime. The Papapetrou force $\sim R_{\mu\nu\rho\sigma}u^\nu S^{\rho\sigma}$ couples the body's spin $S^{\mu\nu}$ to the Riemann tensor. For compact binary systems, spin-orbit coupling is included in gravitational waveform templates from the 1.5PN order.

