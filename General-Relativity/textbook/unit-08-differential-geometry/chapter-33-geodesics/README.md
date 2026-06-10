# Chapter 33: Geodesics

---

## Chapter Introduction

A geodesic is the generalization of a straight line to a curved manifold. In Euclidean space, a straight line is the shortest path between two points and is characterized by having zero acceleration. On a curved surface, neither characterization is exactly right — but both provide useful intuitions.

On the surface of a sphere, the "straight lines" are great circles (the paths that planes through the Earth's center cut on the surface). An airplane flying from Los Angeles to Tokyo follows a great circle — the shortest path on the sphere — even though on a flat map it looks curved. The pilot is following a geodesic.

In spacetime, freely falling particles follow geodesics. The geodesic equation $\ddot{x}^\mu + \Gamma^\mu_{\rho\sigma}\dot{x}^\rho\dot{x}^\sigma = 0$ is the equation of motion in curved spacetime — it reduces to Newton's second law (with gravitational acceleration) in the Newtonian limit, and to force-free motion in a freely falling frame. This is the mathematical expression of the equivalence principle.

This chapter derives the geodesic equation from two perspectives (parallel transport and action principle), analyzes geodesics in specific spacetimes, and introduces the notion of geodesic completeness.

---

## Definition and Derivation

**Parallel-transport definition**: A geodesic is a curve $\gamma$ whose tangent vector is parallel-transported along itself:
$$\nabla_{\dot\gamma}\dot\gamma = 0 \quad\Leftrightarrow\quad \frac{D\dot\gamma^\mu}{d\lambda} = \ddot\gamma^\mu + \Gamma^\mu_{\rho\sigma}\dot\gamma^\rho\dot\gamma^\sigma = 0$$

This means: as you move along the curve, the "velocity" vector doesn't rotate — the curve "goes straight" as measured by the connection. 

**Variational definition**: A geodesic extremizes the length functional:
$$L[\gamma] = \int_a^b\sqrt{g_{\mu\nu}\dot\gamma^\mu\dot\gamma^\nu}\,d\lambda$$

(or the energy functional $E[\gamma] = \frac{1}{2}\int g_{\mu\nu}\dot\gamma^\mu\dot\gamma^\nu d\lambda$ for affinely parametrized geodesics). The Euler-Lagrange equations for $E[\gamma]$ give the geodesic equation with affine parametrization.

Both definitions agree for the Levi-Civita connection: the parallel-transport condition with the unique metric-compatible, torsion-free connection coincides with the extremization of arc length.

---

## Affine Parametrization

A geodesic can be reparametrized. Not all parametrizations satisfy the geodesic equation — only **affine parametrizations** $\lambda = a\tau + b$ do. An affinely parametrized geodesic satisfies:
$$\ddot\gamma^\mu + \Gamma^\mu_{\rho\sigma}\dot\gamma^\rho\dot\gamma^\sigma = 0$$

For timelike geodesics in Lorentzian geometry, the natural affine parameter is proper time $\tau$ (with $g_{\mu\nu}\dot\gamma^\mu\dot\gamma^\nu = -c^2$). For null geodesics ($g_{\mu\nu}\dot\gamma^\mu\dot\gamma^\nu = 0$), proper time is not defined, but affine parameter is still well-defined.

**First integrals**: For an affinely parametrized geodesic, $g_{\mu\nu}\dot\gamma^\mu\dot\gamma^\nu = \text{const}$ (the norm is conserved). This follows from:
$$\frac{d}{d\lambda}(g_{\mu\nu}\dot\gamma^\mu\dot\gamma^\nu) = 2g_{\mu\nu}\dot\gamma^\mu\frac{D\dot\gamma^\nu}{d\lambda} = 0$$

---

## Geodesics and Symmetry: Killing Vectors

If $K^\mu$ is a Killing vector ($\nabla_{(\mu}K_{\nu)} = 0$, or equivalently $\mathcal{L}_K g = 0$), then:
$$\frac{d}{d\lambda}(K_\mu\dot\gamma^\mu) = \dot\gamma^\nu\nabla_\nu(K_\mu\dot\gamma^\mu) = K_\mu\underbrace{\dot\gamma^\nu\nabla_\nu\dot\gamma^\mu}_{=0} + \dot\gamma^\mu\dot\gamma^\nu\nabla_\nu K_\mu = \dot\gamma^\mu\dot\gamma^\nu\nabla_{(\nu}K_{\mu)} = 0$$

**Killing vectors generate conservation laws**: $K_\mu\dot\gamma^\mu = \text{const}$ along geodesics.

**In Schwarzschild**: 
- Killing vector $\xi_{(t)} = \partial_t$ (time translation): $E = -g_{tt}\dot\gamma^t = (1-r_s/r)c^2\dot{t}$ (energy per unit mass, conserved)
- Killing vector $\xi_{(\phi)} = \partial_\phi$ (rotation): $L = g_{\phi\phi}\dot\gamma^\phi = r^2\sin^2\theta\,\dot\phi$ (angular momentum per unit mass, conserved)

With the mass-shell condition $g_{\mu\nu}\dot\gamma^\mu\dot\gamma^\nu = -c^2$, these three constants of motion reduce the geodesic equations to quadratures.

---

## Geodesics in Schwarzschild

For equatorial geodesics ($\theta = \pi/2$) in Schwarzschild, the geodesic equations with constants $E$ (energy/mass) and $L$ (angular momentum/mass) become:

$$\frac{1}{2}\dot{r}^2 + V_{\rm eff}(r) = \frac{E^2}{2c^2}$$

$$V_{\rm eff}(r) = \frac{1}{2}\left(1 - \frac{r_s}{r}\right)\left(c^2\epsilon + \frac{L^2}{r^2}\right)$$

where $\epsilon = 1$ for massive particles and $\epsilon = 0$ for photons.

**Circular orbits**: $\dot{r} = 0$ and $\ddot{r} = 0$ give $V_{\rm eff} = E^2/(2c^2)$ and $V'_{\rm eff} = 0$. Solving:
$$r_{\rm circ} = \frac{L^2}{Gm_\star/c^2\pm\sqrt{G^2m_\star^2/c^4 - 3L^2/c^2}}$$

for massive particles ($\epsilon = 1$). The ISCO is at $r = 6GM/c^2$.

**Precession**: For nearly circular orbits, the radial oscillation frequency differs from the azimuthal frequency, giving precession. The perihelion advance per orbit: $\Delta\phi = 6\pi GM/(a(1-e^2)c^2)$.

---

## Null Geodesics and the Photon Sphere

For photons ($\epsilon = 0$, $E$ and $b = L/E$ = impact parameter):
$$V_{\rm eff}^{\rm null}(r) = \frac{1}{2r^2}\left(1 - \frac{r_s}{r}\right)$$

The photon sphere at $r = 3GM/c^2$: the maximum of $V_{\rm eff}^{\rm null}$. For $b = b_c = 3\sqrt{3}GM/c^2$: unstable circular photon orbit.

For $b < b_c$: photon is captured (spirals into black hole).
For $b > b_c$: photon deflects and escapes.
For $b = b_c$: photon circles at the photon sphere (unstable).

**Light deflection**: For $b \gg r_s$, the total deflection angle is $\delta\phi = 4GM/(bc^2)$. For grazing incidence of solar light ($b = R_\odot$): $\delta\phi = 1.75$ arcseconds — confirmed by Eddington 1919.

---

## Geodesic Completeness and Singularities

A spacetime is **geodesically complete** if every geodesic (timelike, null, or spacelike) can be extended to infinite affine parameter. Geodesic incompleteness means some geodesic "ends" in finite affine parameter — which in GR is interpreted as a spacetime singularity.

**Examples of geodesically incomplete spacetimes**:
- Schwarzschild spacetime: geodesics falling into $r = 0$ are incomplete (the singularity is reached in finite proper time)
- FLRW spacetime at $t = 0$ (Big Bang): past-directed geodesics are incomplete

**Penrose-Hawking singularity theorems**: Under physically reasonable energy conditions and with trapped surfaces or closed Cauchy surfaces, every spacetime is geodesically incomplete. Singularities are generic.

---

## Geodesic Deviation

Two nearby geodesics with separation vector $\xi^\mu$ (connecting points at the same affine parameter) evolve as:
$$\frac{D^2\xi^\mu}{d\lambda^2} = -R^\mu_{\ \nu\rho\sigma}\dot\gamma^\nu\xi^\rho\dot\gamma^\sigma$$

This is the **geodesic deviation equation** (Jacobi equation). It says: the relative acceleration of nearby geodesics is determined by the Riemann tensor. In GR with the geodesic hypothesis (free particles follow geodesics), this is the tidal acceleration — the physical effect of curvature.

For gravitational wave detection (LIGO): two test masses are in free fall (following geodesics). A gravitational wave with strain $h$ causes differential acceleration $\ddot\xi = \frac{c^2}{2}\ddot{h}\xi$ — giving the measurable displacement $\delta L = \frac{1}{2}hL$.

---

## Exercises

**33.1.** *Geodesics on $S^2$.*

(a) Show that great circles (intersections of planes through the origin with $S^2$) are geodesics of the round metric on $S^2$.

(b) Write and solve the geodesic equations for the equatorial great circle $\theta = \pi/2$, $\phi = \phi(\lambda)$.

(c) Two geodesics starting at the north pole diverge and then reconverge at the south pole. Compute the separation $\xi(\lambda)$ between two nearby geodesics (both starting at the north pole but differing by a small angle) as a function of arc length. What is the curvature inferred from this divergence/convergence?

---

**33.2.** *The perihelion precession.*

For a nearly circular orbit in Schwarzschild ($r = r_0 + \delta r$, $\delta r\ll r_0$):

(a) Expand $V_{\rm eff}(r)$ around $r_0$ to second order to find the radial oscillation frequency $\omega_r$.

(b) The orbital (azimuthal) frequency is $\omega_\phi = d\phi/dt$. Compute both.

(c) Show that $\omega_r < \omega_\phi$, and the angle of precession per orbit is $\Delta\phi = 2\pi(\omega_\phi/\omega_r - 1) = 6\pi GM/(r_0 c^2(1-r_s/(2r_0)))$. For the nearly circular case, this gives Mercury's $43''$/century.

---

**33.3.** *Geodesic deviation and LIGO.*

For a gravitational wave with $+$-polarization $h_{xx} = -h_{yy} = h(t)$ (in linearized GR), two test masses separated by $\xi^x$ in the $x$-direction satisfy:
$$\frac{d^2\xi^x}{dt^2} = \frac{c^2}{2}\ddot{h}\xi^x$$

(a) For a sinusoidal wave $h = h_0\sin(\omega_{\rm GW}t)$, solve for $\xi^x(t)$ assuming small oscillations: $\xi^x(t) = L + \delta\xi^x(t)$ with $L = 4$ km.

(b) The LIGO sensitivity is $\delta L \gtrsim 10^{-18}$ m. For $h_0 = 10^{-21}$ and $f_{\rm GW} = 100$ Hz, compute $\delta\xi^x$ and verify it is detectable.

(c) Why is the sensitivity to the separation length $L$ — larger mirrors give better sensitivity? How does LIGO's power recycling (effectively multiplying $L$) work in this picture?

---

**Thought Experiment T33.1.** *The geodesic hypothesis.*

In Newtonian mechanics, $F = ma$: a particle moves in a straight line (zero acceleration) unless acted on by a force. In GR, a freely falling particle (no forces except gravity) follows a geodesic — but gravity is not a force; it's the curvature of spacetime.

A satellite orbiting Earth is in free fall — it experiences zero acceleration (weightless astronauts) and follows a geodesic in curved spacetime. The "force of gravity" is zero in the satellite's frame.

But the satellite's orbit is not a straight line in 3D. In what sense is the satellite "moving in a straight line"? How do you reconcile the satellite's curved orbit (seen from Earth) with its zero acceleration (felt by the astronauts)? What does this tell you about the nature of gravity as "curvature of spacetime" rather than a force?
