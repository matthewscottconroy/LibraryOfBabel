# Chapter 23: Exercises

---

## Section 23.1 — The Covariant Derivative

**23.1.1.** *The connection in polar coordinates.*

Consider the flat Euclidean plane with polar coordinates $(r,\theta)$, where the metric is $ds^2 = dr^2 + r^2 d\theta^2$.

(a) Compute all Christoffel symbols $\Gamma^\rho_{\mu\nu}$ directly from the formula $\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})$.

(b) The vector field $\mathbf{V} = \partial/\partial x$ (pointing in the $x$-direction) has components $V^r = \cos\theta$, $V^\theta = -\sin\theta/r$ in polar coordinates. Show directly that $\nabla_r V^\theta \neq \partial_r V^\theta$, and that the correction term $\Gamma^\theta_{r\theta}V^\theta$ is exactly what is needed for the covariant derivative to vanish (since $\partial/\partial x$ is a constant vector field in flat space).

(c) A vector $\mathbf{W}$ at $(r,\theta) = (1, 0)$ has components $(W^r, W^\theta) = (0, 1)$ (pointing in the $\theta$-direction). Parallel-transport this vector along the $r$-direction from $r=1$ to $r=2$ at fixed $\theta = 0$. Solve the parallel transport equations $dW^\mu/dr + \Gamma^\mu_{r\nu}W^\nu = 0$ explicitly. What direction does the vector point at $r=2$?

(d) Repeat (c) but transport along the circle $r = 1$ from $\theta = 0$ to $\theta = \pi/2$. Does the result depend on the path? What does this imply about the curvature of the flat plane?

---

**23.1.2.** *Verifying metric compatibility.*

Let $\mathbf{U}$ and $\mathbf{V}$ be two vector fields and $\gamma(\lambda)$ a curve with tangent $\mathbf{T}$. Metric compatibility states $\nabla_T(g(\mathbf{U},\mathbf{V})) = g(\nabla_T\mathbf{U}, \mathbf{V}) + g(\mathbf{U}, \nabla_T\mathbf{V})$.

(a) Show that this condition, written in components, is equivalent to $\nabla_\rho g_{\mu\nu} = 0$.

(b) Assume metric compatibility and show that parallel transport preserves the inner product: if $\mathbf{U}$ and $\mathbf{V}$ are parallel transported along $\gamma$, then $g_{\mu\nu}U^\mu V^\nu = \text{const}$.

(c) Show that metric compatibility also implies parallel transport preserves the norm $g_{\mu\nu}V^\mu V^\nu$, and therefore preserves the causal character (timelike/null/spacelike) of vectors.

(d) In the context of GR: a massive particle has 4-velocity $u^\mu$ with $g_{\mu\nu}u^\mu u^\nu = -c^2$. Explain why metric compatibility guarantees this condition is preserved along the particle's worldline.

---

**23.1.3.** *Torsion and symmetry.*

A general connection $\Gamma^\rho_{\mu\nu}$ need not be symmetric in $\mu\nu$. Define the torsion tensor $T^\rho_{\ \mu\nu} = \Gamma^\rho_{\mu\nu} - \Gamma^\rho_{\nu\mu}$.

(a) Show that the torsion tensor transforms as a tensor under coordinate changes, even though $\Gamma^\rho_{\mu\nu}$ does not.

(b) Show that any connection can be uniquely decomposed as $\Gamma^\rho_{\mu\nu} = \tilde{\Gamma}^\rho_{\mu\nu} + \frac{1}{2}T^\rho_{\ \mu\nu}$, where $\tilde{\Gamma}^\rho_{\mu\nu}$ is the symmetric part. (Here $\tilde{\Gamma}^\rho_{\mu\nu}$ is symmetric in $\mu\nu$.)

(c) For the Levi-Civita connection, $T^\rho_{\ \mu\nu} = 0$ by definition. Show that this symmetry condition, combined with metric compatibility, uniquely determines the Christoffel formula. (This is the fundamental theorem of Riemannian geometry.)

(d) Élie Cartan's torsion $T^a = de^a + \omega^a_{\ b}\wedge e^b$ (where $e^a$ are vielbeins and $\omega^a_{\ b}$ is the spin connection) generalizes this to non-symmetric connections. In Einstein-Cartan theory, torsion is sourced by spin density. Why does the torsion vanish in ordinary GR with spinless matter?

---

**23.1.4.** *The covariant derivative and the Lie derivative.*

The Lie derivative $\mathcal{L}_X Y = [X, Y]$ (the commutator of vector fields) and the covariant derivative $\nabla_X Y$ are two different ways to differentiate vector fields.

(a) Show that the difference $\nabla_X Y - \nabla_Y X - [X,Y] = T(X,Y)$ (the torsion). For the Levi-Civita connection, this difference vanishes: $\nabla_X Y - \nabla_Y X = [X,Y]$.

(b) The Lie derivative does not require a connection and is defined for any smooth manifold. The covariant derivative requires a choice of connection. A Killing vector field $\xi$ satisfies $\mathcal{L}_\xi g = 0$. Write this condition out in coordinates.

(c) Show that for the Levi-Civita connection, $\mathcal{L}_\xi g_{\mu\nu} = 0$ is equivalent to $\nabla_{(\mu}\xi_{\nu)} = 0$ (the Killing equation).

(d) Schwarzschild has the Killing vector $\xi^\mu = (1,0,0,0)$ (time translation symmetry). Show that the conserved quantity along a geodesic is $E = -g_{\mu\nu}\xi^\mu\dot{x}^\nu = (1-2GM/rc^2)c\dot{t}$, the specific energy per unit mass. What physical quantity does this represent at large $r$?

---

## Section 23.2 — Christoffel Symbols and Geodesics

**23.2.1.** *Geodesics on the 2-sphere.*

The 2-sphere $S^2$ of radius $R$ has metric $ds^2 = R^2(d\theta^2 + \sin^2\theta\, d\phi^2)$.

(a) Compute all non-zero Christoffel symbols: $\Gamma^\theta_{\phi\phi} = -\sin\theta\cos\theta$ and $\Gamma^\phi_{\theta\phi} = \Gamma^\phi_{\phi\theta} = \cot\theta$. All others vanish.

(b) Write out the geodesic equations $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$ explicitly for $\theta$ and $\phi$.

(c) Verify that great circles satisfy the geodesic equations. Take the great circle $\theta = \pi/2$ (the equator): show that $\theta(\lambda) = \pi/2$ and $\dot{\phi} = \text{const}$ satisfies the equations.

(d) Now take any great circle. Show that without loss of generality it can be written as $\cos\theta = \tan(\phi_0 - \phi)\cdot C$ for constants $C$ and $\phi_0$. Verify this solves the geodesic equations by direct substitution.

(e) Parallel transport the vector $\mathbf{V} = \partial/\partial\theta$ around the equator ($\theta = \pi/2$, $\phi$ from $0$ to $2\pi$). Solve the parallel transport equations. What is the holonomy angle? How does this relate to the solid angle subtended by the hemisphere?

---

**23.2.2.** *Schwarzschild geodesics and classical tests.*

The Schwarzschild metric in natural units ($c = G = 1$) at fixed $\theta = \pi/2$ gives the effective 1D system with:
$$\frac{1}{2}\dot{r}^2 + V_{\rm eff}(r) = \frac{1}{2}E^2, \quad V_{\rm eff}(r) = \frac{1}{2}\left(-\frac{2M}{r} + \frac{L^2}{r^2} - \frac{2ML^2}{r^3}\right) + \frac{1}{2}$$
for timelike geodesics ($\epsilon = 1$, for null geodesics $\epsilon = 0$).

(a) Show that the innermost stable circular orbit (ISCO) is at $r = 6M$ for timelike geodesics. Find the orbital frequency $\Omega = d\phi/dt$ at the ISCO.

(b) For light bending: set $\epsilon = 0$ (null geodesics) and show that the total deflection angle for a photon passing at closest approach distance $b$ is $\delta\phi \approx 4M/b$ to leading order in $M/b$. Compare with the Newtonian prediction $\delta\phi_{\rm Newton} = 2M/b$.

(c) Perihelion precession: use the effective potential to show that nearly circular timelike geodesics precess at rate $\Delta\phi = 6\pi M/[a(1-e^2)]$ per orbit (in natural units), where $a$ is the semi-major axis and $e$ is the eccentricity.

(d) Gravitational redshift: two observers at rest at radii $r_{\rm emit}$ and $r_{\rm obs}$ exchange light signals. Show from the Killing vector argument that the frequency ratio is:
$$\frac{f_{\rm obs}}{f_{\rm emit}} = \sqrt{\frac{1 - 2M/r_{\rm emit}}{1 - 2M/r_{\rm obs}}}$$
What does this reduce to for $r_{\rm emit} \ll r_{\rm obs}$ at large $r_{\rm obs}$?

---

**23.2.3.** *Geodesic deviation and the tidal field.*

Two nearby geodesics are separated by the deviation vector $\xi^\mu$. The geodesic deviation equation is:
$$\frac{D^2\xi^\mu}{d\tau^2} = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$$

(a) In the Newtonian limit, the metric is $g_{00} = -(1 + 2\Phi/c^2)$ with all other components flat. The dominant Riemann components are $R^i_{\ 0j0}$. Show that these reduce to:
$$R^i_{\ 0j0} \approx \frac{1}{c^2}\frac{\partial^2\Phi}{\partial x^i\partial x^j}$$
and that the geodesic deviation equation becomes Newton's tidal force equation $\ddot{\xi}^i = -(\partial^2\Phi/\partial x^i\partial x^j)\xi^j$.

(b) For the Schwarzschild metric with two radially-separated free-falling observers, compute the tidal acceleration between them as a function of $r$ and the radial separation $\xi^r$.

(c) A spaghettified astronaut falls radially into a Schwarzschild black hole. Using the Schwarzschild Riemann tensor, compute the tidal force stretching the astronaut in the radial direction and squeezing them transversally. Show that the product (radial stretching) × (transverse squeezing) is constant — conservation of what?

(d) LIGO measures gravitational waves by tracking the separation of mirrors. The relevant Riemann component for the $+$ polarization mode propagating in the $z$-direction is $R_{x0x0} = -\frac{1}{2}\ddot{h}_+$, where $h_+$ is the wave strain. For a 1 km arm length and peak strain $h_+ \sim 10^{-21}$ (like the first detection), what is the peak mirror displacement? How does this compare with the diameter of a proton ($\sim 10^{-15}$ m)?

---

## Thought Experiments

**T23.1.** *What does a vector "remember"?*

Imagine you are standing at the North Pole of the Earth, holding a javelin pointing south toward a particular meridian. You walk south along that meridian to the equator — the javelin still points south (in the direction you're walking). You then walk east along the equator by 90°, parallel-transporting the javelin (keeping it pointing in a fixed compass direction — but "parallel transport" on the sphere means keeping the angle between the javelin and your path constant). Finally, you walk back north to the Pole along the new meridian.

The javelin now points in a different direction than when you started, even though the Earth is locally flat at every point and you never rotated the javelin in your hands. How can a vector "rotate" without being rotated? What does this tell you about the relationship between curvature and path-dependence? If the Earth had twice the radius but you traveled the same triangle, would the rotation angle change?

---

**T23.2.** *The connection as a telephone network.*

The connection $\Gamma^\rho_{\mu\nu}$ can be thought of as a "telephone network" that tells vectors at one point how to communicate their identity to vectors at neighboring points. In flat space, the network is "perfect" — vectors at different points can be unambiguously compared. In curved space, the network depends on the path, so the comparison is ambiguous.

This is not a failure of the mathematics — it is a deep physical fact. In GR, there is no way to define the difference between two vectors at different spacetime events. You can only compare them if you parallel-transport one to the other, and the result depends on which path you choose.

Now consider: the electromagnetic vector potential $A_\mu$ plays exactly the same role for charged particles. The "phase" of a quantum wavefunction at one point cannot be compared to the phase at another point without choosing a path — and the phase acquired depends on $\oint A_\mu dx^\mu = \Phi_B$ (Aharonov-Bohm). Explain the precise analogy between $\Gamma^\rho_{\mu\nu}$ and $A_\mu$. What plays the role of the "curvature" $R^\rho_{\ \sigma\mu\nu}$ in electromagnetism?

---

**T23.3.** *Einstein's elevator, infinitely large.*

Einstein's equivalence principle says a freely-falling elevator is locally indistinguishable from flat spacetime. The word "locally" is crucial. Explain how the *size* of the region over which you can neglect curvature depends on (a) the strength of the gravitational field, (b) the precision of your measurements, and (c) the mass of the gravitating body. For the Earth, estimate the length scale over which tidal forces become detectable at 1% precision for freely-falling observers. For a stellar-mass black hole at the event horizon, what is this length scale? For a supermassive black hole ($M \sim 10^9 M_\odot$) at its horizon?

---

## Laboratory Exercise: Simulating Parallel Transport

**L23.1.** *Numerical parallel transport on $S^2$.*

Write a program (Python recommended) to simulate parallel transport on the 2-sphere.

**Setup:** Represent a vector $V = (V^\theta, V^\phi)$ at a point $(\theta,\phi)$ on the unit sphere. The parallel transport equations along a path $(\theta(\lambda),\phi(\lambda))$ are:
$$\frac{dV^\theta}{d\lambda} + \Gamma^\theta_{\phi\phi}\dot\phi V^\phi = 0, \quad \frac{dV^\phi}{d\lambda} + \Gamma^\phi_{\theta\phi}\dot\theta V^\phi + \Gamma^\phi_{\phi\theta}\dot\phi V^\theta = 0$$
with $\Gamma^\theta_{\phi\phi} = -\sin\theta\cos\theta$ and $\Gamma^\phi_{\theta\phi} = \Gamma^\phi_{\phi\theta} = \cot\theta$.

**Task 1:** Transport the vector $V = (1,0)$ (pointing "north") from $(\theta_0, 0)$ to $(\theta_0, 2\pi)$ around the latitude circle $\theta = \theta_0$. Use a 4th-order Runge-Kutta integrator with step size $\Delta\lambda = 0.001$. Measure the holonomy angle as a function of $\theta_0$. Plot the result.

**Task 2:** Prove (numerically) that for a triangle made of three great-circle arcs enclosing solid angle $\Omega$, the holonomy angle equals $\Omega$. (The Gauss-Bonnet theorem.)

**Task 3:** Embed the sphere in $\mathbb{R}^3$ and visualize the vector being transported as a 3D arrow attached to a moving point on the sphere. The vector should always be tangent to the sphere.

**Analysis:** The holonomy of a small loop of coordinate area $\delta\theta\,\delta\phi$ should be approximately $R_{\theta\phi}\delta\theta\,\delta\phi = \sin\theta\,\delta\theta\,\delta\phi$. Verify this numerically for small loops at several values of $\theta$.

