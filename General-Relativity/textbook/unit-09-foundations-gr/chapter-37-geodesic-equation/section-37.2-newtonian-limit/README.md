# Section 37.2: The Newtonian Limit and Post-Newtonian Expansion

---

## The Newtonian Limit of the Geodesic Equation

To recover Newtonian gravity from GR, we need three conditions:
1. **Slow motion:** $v \ll c$, so $\dot{x}^i \ll \dot{x}^0 = c\dot{t}$, i.e., $dx^i/d\tau \ll c\,dt/d\tau$
2. **Weak field:** $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$ with $|h_{\mu\nu}| \ll 1$
3. **Slow variation:** $\partial_t h_{\mu\nu} \ll \partial_i h_{\mu\nu}$ (time derivatives negligible)

Under these conditions, the geodesic equation $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$ simplifies.

The dominant term in $\Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho$ is the $\nu = \rho = 0$ term (since spatial velocities are small):
$$\Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho \approx \Gamma^\mu_{00}(\dot{t})^2$$

From the Christoffel formula with the slow-variation assumption:
$$\Gamma^\mu_{00} = \frac{1}{2}g^{\mu\nu}(2\partial_0 g_{\nu 0} - \partial_\nu g_{00}) \approx -\frac{1}{2}\eta^{\mu\nu}\partial_\nu h_{00}$$

For the spatial components ($\mu = i$):
$$\Gamma^i_{00} \approx -\frac{1}{2}\partial_i h_{00}$$

The spatial geodesic equation becomes:
$$\ddot{x}^i + \Gamma^i_{00}(\dot{t})^2 \approx 0 \implies \frac{d^2x^i}{dt^2} \approx -\frac{c^2}{2}\partial_i h_{00}$$

For this to be Newton's law $d^2x^i/dt^2 = -\partial_i\Phi$ (where $\Phi$ is the Newtonian potential), we need:
$$\boxed{h_{00} = -\frac{2\Phi}{c^2}} \implies g_{00} = -\left(1 + \frac{2\Phi}{c^2}\right)$$

This identifies the $00$-component of the metric perturbation as minus twice the Newtonian potential. For the Earth at the surface: $\Phi_\oplus/c^2 = -GM_\oplus/(R_\oplus c^2) \approx -7\times 10^{-10}$ — extremely small. This confirms the weak-field approximation.

---

## Gravitational Redshift Recovered

With $g_{00} = -(1 + 2\Phi/c^2)$, the gravitational redshift follows immediately. A static clock at position $r$ ticks at proper time $d\tau$:
$$d\tau = \sqrt{-g_{00}}dt = \sqrt{1 + 2\Phi/c^2}\,dt \approx \left(1 + \frac{\Phi}{c^2}\right)dt$$

A clock deeper in the potential ($\Phi$ more negative) ticks slower. This is the gravitational redshift, now derived from the metric without any separate argument. At the Earth's surface:
$$\frac{d\tau}{dt} = \sqrt{1 - \frac{2GM_\oplus}{R_\oplus c^2}} \approx 1 - 6.95\times 10^{-10}$$

A clock on Earth runs $\sim 6.95\times 10^{-10}$ slower than a clock infinitely far away. This is the gravitational time dilation used in GPS corrections.

---

## Post-Newtonian Expansion

The post-Newtonian (PN) expansion is a systematic approximation scheme for slowly-moving, weakly-gravitating systems. It expands in powers of $v/c \sim (GM/rc^2)^{1/2} \sim \epsilon^{1/2}$.

At Newtonian order (0PN): $\Phi = -GM/r$, geodesic equation gives Newton's gravity.

At 1PN ($\epsilon^1$, or $v^2/c^2$ corrections): The metric is:
$$g_{00} = -1 + \frac{2\Phi}{c^2} - \frac{2\Phi^2}{c^4} + O(c^{-6})$$
$$g_{0i} = -\frac{4\xi_i}{c^3} + O(c^{-5})$$
$$g_{ij} = \left(1 + \frac{2\Phi}{c^2}\right)\delta_{ij} + O(c^{-4})$$

where $\xi_i$ is a vector potential sourced by the mass current (like the magnetic vector potential is sourced by the electric current). The $g_{0i}$ terms give the **gravitomagnetic** effects: frame-dragging, Lense-Thirring precession, geodetic precession.

The equations of motion at 1PN are the **Einstein-Infeld-Hoffmann (EIH) equations** (1938):
$$m_a\ddot{\mathbf{x}}_a = -\sum_{b\neq a}\frac{Gm_a m_b(\mathbf{x}_a - \mathbf{x}_b)}{|\mathbf{x}_a - \mathbf{x}_b|^3}\left[1 + (\text{PN corrections})\right] + (\text{velocity-dependent terms})$$

The EIH equations are used for high-precision solar system ephemeris calculations and for the data analysis of binary pulsar timing observations.

At 2.5PN: radiation reaction terms appear (the Schott-Burke-Thorne radiation damping). These cause the inspiral of compact binary systems — the first evidence for gravitational waves from the Taylor-Hulse binary pulsar.

**Gravitational waveform templates** for LIGO/Virgo use post-Newtonian expansions (for the early inspiral phase) matched to numerical relativity results (for the final merger plunge). Current templates go to 3.5PN in the phase and include spin-orbit coupling, spin-spin coupling, and quadrupole-monopole interactions.

---

## Gravitoelectromagnetism

In the weak-field, slow-motion limit, the linearized Einstein equations take a form analogous to Maxwell's equations. Define:
$$\mathbf{g} = -c^2\nabla h_{00}/2 \approx -\nabla\Phi \quad \text{(gravitoelectric field)}$$
$$\mathbf{H} = \text{(curl of gravitomagnetic potential from $g_{0i}$)} \quad \text{(gravitomagnetic field)}$$

The linearized Einstein equations give:
$$\nabla\cdot\mathbf{g} = -4\pi G\rho, \quad \nabla\times\mathbf{g} = -\frac{1}{c}\frac{\partial\mathbf{H}}{\partial t}$$
$$\nabla\cdot\mathbf{H} = 0, \quad \nabla\times\mathbf{H} = -\frac{16\pi G}{c}\mathbf{J} + \frac{1}{c}\frac{\partial\mathbf{g}}{\partial t}$$

where $\rho$ is the mass-energy density and $\mathbf{J} = \rho\mathbf{v}$ is the mass current. These are the **gravitoelectromagnetic equations** — structurally identical to Maxwell's equations, with $\rho\to\rho$, $\mathbf{J}\to\mathbf{J}$, but with the gravitomagnetic force being 4 times stronger (the factor of $-16\pi G$ vs. $\mu_0 = 4\pi/(\varepsilon_0 c^2)$ — gravity is a spin-2 theory, not spin-1, which changes the sign and strength of the magnetic-type force).

The equation of motion for a slowly moving particle in the GEM field is:
$$m\ddot{\mathbf{x}} = m\mathbf{g} + \frac{m}{c}\mathbf{v}\times\mathbf{H} \cdot (-4)$$

The factor of $-4$ (gravitomagnetic force is 4 times stronger than expected by direct EM analogy) is the hallmark of the spin-2 nature of gravity.

**Lense-Thirring precession:** A gyroscope orbiting a rotating body (mass $M$, angular momentum $J$) precesses at:
$$\boldsymbol{\Omega}_{\rm LT} = \frac{G}{c^2 r^3}\left[3(\mathbf{J}\cdot\hat{r})\hat{r} - \mathbf{J}\right]$$
Measured by Gravity Probe B: $37.2 \pm 7.2$ mas/yr (milliarcseconds per year), compared to the GR prediction of $39.2$ mas/yr. Confirmed at $\sim 19\%$ precision (limited by systematic errors in the gyroscope readout).

**Geodetic precession:** A gyroscope in orbit also precesses due to the curvature of the spatial metric:
$$\boldsymbol{\Omega}_{\rm dS} = \frac{3}{2}\frac{GM}{c^2 r^3}(\mathbf{v}\times\mathbf{r})$$
Measured by Gravity Probe B: $6601.8 \pm 18.3$ mas/yr, compared to the GR prediction of $6606.1$ mas/yr. Confirmed to $0.28\%$.

---

## Summary: What the Geodesic Equation Gives Us

The geodesic equation, combined with the weak-field metric $g_{00} = -(1 + 2\Phi/c^2)$, directly gives:
1. **Newton's gravitational force:** $d^2\mathbf{x}/dt^2 = -\nabla\Phi$ (0PN)
2. **Gravitational time dilation:** $d\tau/dt = \sqrt{1 + 2\Phi/c^2}$ (0PN metric)
3. **Light bending:** $4GM/(bc^2)$ (null geodesics, 0.5PN spatial metric)
4. **Shapiro delay:** from null geodesic travel time (0.5PN)
5. **Perihelion precession:** from corrections to the radial potential (1PN)
6. **Frame-dragging and Lense-Thirring:** from $g_{0i}$ (1PN gravitomagnetic)
7. **Gravitational waves:** from radiating oscillating mass distributions (2.5PN, dissipative)

Every classical test of GR is a prediction of the geodesic equation in the appropriate metric. The equation itself is a consequence of the field equations. The whole of classical GR — tests, solutions, predictions — flows from $G_{\mu\nu} = 8\pi G T_{\mu\nu}$.

