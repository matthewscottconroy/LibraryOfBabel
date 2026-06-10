# Chapter 37: Exercises

---

**37.1.** *Deriving the geodesic equation from the variational principle.*

The proper time functional is $\tau = \int\sqrt{-g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu}\,d\lambda$. It is easier to work with the equivalent Lagrangian $\mathcal{L} = -\frac{1}{2}g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu$.

(a) Compute $\partial\mathcal{L}/\partial\dot{x}^\sigma = -g_{\sigma\nu}\dot{x}^\nu$ and $\partial\mathcal{L}/\partial x^\sigma = -\frac{1}{2}\partial_\sigma g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu$.

(b) Write the Euler-Lagrange equation $d/d\lambda(\partial\mathcal{L}/\partial\dot{x}^\sigma) - \partial\mathcal{L}/\partial x^\sigma = 0$ and expand to get:
$$-g_{\sigma\nu}\ddot{x}^\nu - \partial_\mu g_{\sigma\nu}\dot{x}^\mu\dot{x}^\nu + \frac{1}{2}\partial_\sigma g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$$

(c) Multiply by $-g^{\rho\sigma}$ and use the Christoffel formula to show this is equivalent to $\ddot{x}^\rho + \Gamma^\rho_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$.

(d) Show that the normalization $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = \text{const}$ is preserved along a geodesic (i.e., $d/d\lambda(g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu) = 0$). This is a consequence of metric compatibility.

---

**37.2.** *Geodesics in Schwarzschild spacetime.*

The Schwarzschild metric (in units $G = c = 1$, equatorial plane $\theta = \pi/2$):
$$ds^2 = -(1-2M/r)dt^2 + (1-2M/r)^{-1}dr^2 + r^2d\phi^2$$

The two Killing vectors give conserved quantities $E = (1-2M/r)\dot{t}$ and $L = r^2\dot\phi$.

(a) From the normalization $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = -1$ (timelike), derive the effective radial equation:
$$\frac{1}{2}\dot{r}^2 + V_{\rm eff}(r) = \frac{E^2-1}{2}, \quad V_{\rm eff} = -\frac{M}{r} + \frac{L^2}{2r^2} - \frac{ML^2}{r^3}$$

(b) Find the circular orbit condition $dV_{\rm eff}/dr = 0$ and show it gives $r_\pm = L^2 \pm \sqrt{L^4 - 12M^2L^2}/(2M)$. Show the innermost stable circular orbit is at $r_{\rm ISCO} = 6M$ (by requiring also $d^2V_{\rm eff}/dr^2 = 0$).

(c) For a slightly non-circular orbit, the radial oscillation frequency $\Omega_r^2 = d^2V_{\rm eff}/dr^2|_{r_c}/(r_c^2)$ differs from the azimuthal frequency $\Omega_\phi = L/r_c^2$. Show that $\Omega_r < \Omega_\phi$ — the radial period is longer than the orbital period — giving retrograde precession of the pericenter. Compute the precession rate $\Delta\phi = 2\pi(\Omega_\phi/\Omega_r - 1)$ per orbit for a nearly circular orbit at $r_c \gg M$.

(d) For Mercury's orbit ($a = 5.79\times 10^{10}$ m, $e = 0.206$, $M_\odot = 1.989\times 10^{30}$ kg), compute $\Delta\phi$ per orbit and per century. Compare to the observed anomaly of $43''$/century.

---

**37.3.** *Null geodesics and light deflection.*

For null geodesics ($g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$) in Schwarzschild, with $E = (1-2M/r)\dot{t}$ and $L = r^2\dot\phi$:

(a) Show the effective potential for null geodesics is $V_{\rm null} = L^2(1-2M/r)/(2r^2)$.

(b) The photon sphere is at the maximum of $V_{\rm null}$. Show this is at $r = 3M$.

(c) For a light ray with impact parameter $b = L/E$ passing the Sun at closest approach $r_{\min} \approx R_\odot$, use a perturbative expansion in $M/r$ to show the total deflection is $\delta\phi = 4GM_\odot/(R_\odot c^2)$. (Hint: use the substitution $u = 1/r$ and the Binet-like equation $d^2u/d\phi^2 + u = 3Mu^2$.)

(d) The Event Horizon Telescope observed the shadow of M87*, whose mass is $M = 6.5\times 10^9 M_\odot$ at distance $D = 16.8$ Mpc. The shadow angular radius is $\sim 2.6\times r_{\rm photon}/D$ where $r_{\rm photon} = 3GM/c^2$. Compute the predicted shadow angular radius in microarcseconds. Compare to the EHT resolution of $\sim 25\,\mu$as.

---

**37.4.** *Lense-Thirring effect.*

(a) In the gravitomagnetic formalism, the gravitomagnetic field around a rotating Earth is:
$$\mathbf{H} = \frac{G}{c^2r^3}\left[3(\mathbf{J}\cdot\hat{r})\hat{r} - \mathbf{J}\right]$$
A gyroscope precesses at $d\hat{S}/d\tau = \boldsymbol\Omega_{\rm LT}\times\hat{S}$ where $\boldsymbol\Omega_{\rm LT} = \mathbf{H}/2$. Compute $|\boldsymbol\Omega_{\rm LT}|$ for a gyroscope at the Gravity Probe B orbital radius ($r = 7020$ km) at the poles (where $\hat{r} \parallel \mathbf{J}$).

(b) The geodetic (de Sitter) precession rate for GP-B is $\boldsymbol\Omega_{\rm dS} = \frac{3GM_\oplus}{2c^2r}(\mathbf{v}/r)$ where $|\mathbf{v}| = v_{\rm orb}$. Compute $v_{\rm orb}$ for the circular orbit at $r = 7020$ km and then $|\boldsymbol\Omega_{\rm dS}|$.

(c) Compare $|\boldsymbol\Omega_{\rm LT}|$ and $|\boldsymbol\Omega_{\rm dS}|$. Which is larger? How does the ratio compare to the ratio of their measured values in the GP-B experiment ($37.2''$/yr and $6601.8''$/yr)?

(d) The LAGEOS satellites (laser-ranging geodetic satellites) have measured the Lense-Thirring effect on their orbital planes, finding agreement with GR to $\sim 10\%$. Why are satellite orbits (rather than gyroscopes) a valid probe of the Lense-Thirring effect?

---

## Thought Experiments

**T37.1.** *The geodesic as "natural motion."*

Aristotle held that natural motion (in the absence of force) is circular (for heavenly bodies) or along straight lines toward the element's natural place (for earthly matter). Newton said natural motion in the absence of force is along a straight line at constant speed. Einstein says natural motion in the absence of non-gravitational force is along a geodesic of curved spacetime.

Each refinement makes the concept of "natural motion" more precise and more encompassing. In Einstein's picture, a planet orbiting the Sun is in *natural motion* — no force is acting on it. The Earth is in natural motion too (geodesic = ellipse, approximately). You sitting in a chair are *not* in natural motion — the chair exerts a normal force on you, accelerating you away from the geodesic (which would be free fall through the floor).

From this perspective, who is "really" accelerating: the astronaut in free fall, or you sitting in a chair? (Answer: you. Free-fall is inertial; the chair forces you into non-inertial motion.) How does this reframe our intuition about "feeling weightless"?

**T37.2.** *Can you escape a black hole by accelerating?*

Inside the event horizon of a Schwarzschild black hole ($r < 2GM/c^2$), the radial coordinate $r$ becomes timelike — all timelike and null geodesics have $dr/d\tau < 0$ (decreasing $r$). Can a rocket engine allow an observer inside the horizon to escape?

The answer is no: even with arbitrary acceleration, the future light cone of any event inside the horizon is entirely contained within the horizon. No causal signal can escape. But exactly at $r = 2GM/c^2$ (the horizon), a radially outward light ray stays at $r = \text{const}$ — it neither escapes nor falls in (in Schwarzschild coordinates). The horizon is the boundary between "can escape" and "cannot escape."

Construct the causal argument carefully using the null geodesic equation and the Schwarzschild metric. Why does Schwarzschild coordinate time diverge at the horizon even though proper time does not?

---

## Laboratory Exercise: Geodesic Simulation

**L37.1.** *Numerically integrating geodesics in Schwarzschild spacetime.*

**Setup:** The Schwarzschild geodesic equations in the equatorial plane ($\theta = \pi/2$, $G = c = M = 1$) are:
$$\dot{E} = 0, \quad \dot{L} = 0$$
$$\dot{r}^2 = E^2 - (1-2/r)(1 + L^2/r^2)$$
$$\dot\phi = L/r^2, \quad \dot{t} = E/(1-2/r)$$

**Task 1 (timelike geodesics):** For $E = 0.96$ (slightly bound, $E < 1$) and $L = 4M$ (close to circular), integrate the geodesic equations numerically using 4th-order Runge-Kutta. Start at $r(0) = 10M$, $\phi(0) = 0$, $t(0) = 0$. Plot the orbit in the $(x,y) = (r\cos\phi, r\sin\phi)$ plane. Measure the precession angle per orbit and compare to $\Delta\phi = 6\pi M/[L^2(1-e^2)]$.

**Task 2 (null geodesics):** Repeat for a photon with impact parameter $b = L/E = 5.2M$ (slightly above the critical value $b_c = 3\sqrt{3}M \approx 5.196M$). Plot the deflection angle as the photon passes the central mass. Verify $\delta\phi \approx 4M/b$ for large $b$.

**Task 3 (capture):** For $b < b_c$, show that the photon spirals into the black hole. Find numerically the critical impact parameter $b_c$ and compare to the analytic value $3\sqrt{3}M$.

**Task 4 (ISCO):** Starting from circular orbit initial conditions at $r = 6M$ (ISCO), add a small perturbation in $r$ and show the orbit is marginally stable (neither strongly bound nor unbound). Compare to $r = 5M$ (inside ISCO, unstable — plunges) and $r = 8M$ (stable circular orbit).

