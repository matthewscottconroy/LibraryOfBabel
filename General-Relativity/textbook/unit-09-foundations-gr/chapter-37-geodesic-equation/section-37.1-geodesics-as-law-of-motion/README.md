# Section 37.1: Geodesics as the Law of Motion

---

## Derivation from the Variational Principle

The proper time along a timelike worldline is:
$$\tau = \int\sqrt{-g_{\mu\nu}\frac{dx^\mu}{d\lambda}\frac{dx^\nu}{d\lambda}}\,d\lambda$$
where $\lambda$ is any affine parameter. A geodesic is a stationary point of $\tau$ (or equivalently of $\int(-g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu)^{1/2}d\lambda$).

Working with the simpler Lagrangian $\mathcal{L} = -\frac{1}{2}g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu$ (which has the same stationary points as $\sqrt{-\mathcal{L}}$ but is easier to vary), the Euler-Lagrange equations give:
$$\frac{d}{d\lambda}\frac{\partial\mathcal{L}}{\partial\dot{x}^\mu} - \frac{\partial\mathcal{L}}{\partial x^\mu} = 0$$

Computing:
$$\frac{\partial\mathcal{L}}{\partial\dot{x}^\mu} = -g_{\mu\nu}\dot{x}^\nu, \quad \frac{\partial\mathcal{L}}{\partial x^\mu} = -\frac{1}{2}\partial_\mu g_{\nu\rho}\dot{x}^\nu\dot{x}^\rho$$

Substituting:
$$-\frac{d}{d\lambda}(g_{\mu\nu}\dot{x}^\nu) + \frac{1}{2}\partial_\mu g_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$$
$$-g_{\mu\nu}\ddot{x}^\nu - \partial_\rho g_{\mu\nu}\dot{x}^\rho\dot{x}^\nu + \frac{1}{2}\partial_\mu g_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$$

Multiplying by $g^{\sigma\mu}$ and using $\partial_\rho g_{\mu\nu} = \frac{1}{2}(\partial_\rho g_{\mu\nu} + \partial_\nu g_{\mu\rho} + \partial_\rho g_{\mu\nu} - \partial_\nu g_{\mu\rho})$ (symmetrizing and antisymmetrizing):
$$\ddot{x}^\sigma + \Gamma^\sigma_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$$

where $\Gamma^\sigma_{\nu\rho} = \frac{1}{2}g^{\sigma\mu}(\partial_\nu g_{\rho\mu} + \partial_\rho g_{\nu\mu} - \partial_\mu g_{\nu\rho})$ are the Christoffel symbols. This is the geodesic equation.

---

## Conserved Quantities from Killing Vectors

If the metric has a symmetry — a Killing vector $\xi^\mu$ satisfying $\nabla_{(\mu}\xi_{\nu)} = 0$ — then the quantity:
$$Q = g_{\mu\nu}\xi^\mu\dot{x}^\nu = \xi_\mu\dot{x}^\mu$$
is conserved along any geodesic. This follows from:
$$\frac{dQ}{d\lambda} = \frac{d}{d\lambda}(\xi_\mu\dot{x}^\mu) = \nabla_\nu(\xi_\mu\dot{x}^\mu)\dot{x}^\nu = \dot{x}^\mu\dot{x}^\nu\nabla_\nu\xi_\mu + \xi_\mu\underbrace{\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho}_{= 0} = 0$$
The last step uses the symmetry of $\dot{x}^\mu\dot{x}^\nu$ and the antisymmetry of $\nabla_\nu\xi_\mu - \nabla_\mu\xi_\nu$ (from the Killing equation $\nabla_{(\mu}\xi_{\nu)} = 0$ implies $\nabla_\nu\xi_\mu = -\nabla_\mu\xi_\nu$ combined with metric compatibility).

**Schwarzschild conserved quantities:**

The Schwarzschild metric has two relevant Killing vectors for equatorial orbits ($\theta = \pi/2$):
- Time translation: $\xi^\mu_t = (1, 0, 0, 0)$. Conserved: $E/c = -g_{tt}\dot{t} = (1-2GM/(rc^2))\dot{t}c$. Physically: specific energy per unit mass.
- Rotation: $\xi^\mu_\phi = (0, 0, 0, 1)$. Conserved: $L = g_{\phi\phi}\dot{\phi} = r^2\dot{\phi}$. Physically: specific angular momentum per unit mass.

With the normalization $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = -c^2$ (timelike) or $= 0$ (null), these two conserved quantities plus the normalization reduce the geodesic equations to a single first-order ODE:
$$\frac{1}{2}\dot{r}^2 + V_{\rm eff}(r) = \frac{E^2}{2c^2}$$
where the effective potential encodes all the orbital physics (perihelion precession, ISCO, photon sphere, etc.).

---

## The Geodesic Hypothesis

The geodesic equation is often stated as a postulate of GR: "free particles follow geodesics." But it is actually a theorem — it follows from the field equations.

The **geodesic hypothesis** states that, in the limit of a small test body (one whose own stress-energy is negligible compared to the background), the body's center of mass follows a geodesic of the background spacetime. This was shown rigorously by:
- Einstein and Grommer (1927): first proof for the point-particle case.
- Dixon (1970): general extended body, to leading order in the mass.
- Gralla and Wald (2008): rigorous proof for small but finite bodies.

The key idea: the conservation equation $\nabla_\mu T^{\mu\nu} = 0$ for the matter, combined with the Einstein equations (which are self-consistent), implies that the center of mass of the body follows a geodesic to leading order. The body's finite size produces corrections (Papapetrou force for spinning bodies, quadrupole force for tidally deformed bodies).

This is physically profound: the **equations of motion follow from the field equations**. In Newton's gravity, the law $F = ma$ and the gravitational force law $F = -Gm M/r^2$ are separate postulates. In GR, the motion of matter is determined by the curvature of spacetime, which is determined by the matter distribution through the Einstein equations. There is a single unified framework.

---

## Null Geodesics and Light Rays

A photon follows a null geodesic: the same equation $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$ with the constraint $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$.

Since photons have zero rest mass, proper time is undefined ($d\tau = 0$ along a null path). Instead, we use an **affine parameter** $\lambda$ — any parameter related to proper time of a nearby massive particle by a linear transformation.

The null geodesic equation combined with the Schwarzschild metric gives:
$$\frac{d^2u}{d\phi^2} + u = \frac{3GM}{c^2}u^2 \quad (u = 1/r)$$
This is the same as the massive particle Binet equation but without the $u$ term on the right (no rest-mass potential for photons). The correction $3GMu^2/c^2$ gives the light deflection and the Shapiro time delay.

The **photon sphere** is the unstable circular orbit for photons, at $r = 3GM/c^2 = 1.5\,r_s$ for Schwarzschild. Photons on the photon sphere orbit the black hole indefinitely (but unstably). The image of this orbit, scattered toward a distant observer, creates the bright ring seen around a black hole shadow.

The **Event Horizon Telescope** images (2019, 2022) show the shadow of M87* and Sgr A* — regions where null geodesics do not escape to infinity. The shadow boundary is determined by the photon sphere of the Kerr metric.

---

## Geodesic Completeness and Singularities

A spacetime is **geodesically complete** if every geodesic can be extended to arbitrarily large affine parameter in both directions. A spacetime is **geodesically incomplete** if some geodesic terminates at a finite affine parameter value without reaching a boundary.

The Penrose-Hawking singularity theorems (Chapter 47) prove that under physically reasonable conditions (energy conditions + trapped surfaces or cosmological conditions), GR spacetimes are geodesically incomplete. The incompleteness is interpreted as a singularity — a breakdown of the spacetime manifold.

In Schwarzschild, radial geodesics reach $r = 0$ in finite proper time (for timelike geodesics): a particle falling freely through the horizon reaches the singularity in $\tau = \pi GM/c^3$ of proper time (for a particle starting from rest at infinity, approximately $6.5\,\mu$s for a solar-mass black hole). At $r = 0$, tidal forces diverge and the classical description breaks down.

The singularity is *not* a "point in space" but a moment in *time* — a spacelike surface in the Kruskal-Szekeres extension of the Schwarzschild metric. Every future-directed timelike or null geodesic inside the event horizon hits the singularity at a finite proper time.

---

## The Mathisson-Papapetrou-Dixon Equations

For a body with nonzero spin $S^{\mu\nu}$, the center of mass does not follow a geodesic. The **Mathisson-Papapetrou-Dixon equations** (1937, 1951, 1970) give the equations of motion for an extended spinning body in a curved background:

$$\frac{DP^\mu}{d\tau} = -\frac{1}{2}R^\mu_{\ \nu\rho\sigma}u^\nu S^{\rho\sigma}$$
$$\frac{DS^{\mu\nu}}{d\tau} = P^\mu u^\nu - P^\nu u^\mu$$

The first equation shows that a spinning body experiences a **Papapetrou force** proportional to the Riemann tensor — the curvature coupling between spin and tidal forces. For an astrophysical compact object, this force is of order $(r_s/r)^2$ times the geodesic force, which is negligible for most purposes.

For compact binaries (merging neutron stars or black holes), the spin-orbit coupling matters for gravitational waveform modeling — LIGO/Virgo templates include post-Newtonian spin terms.

