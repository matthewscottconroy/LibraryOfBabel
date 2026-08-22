# Chapter 15 Exercises: Lagrangian and Hamiltonian Mechanics

---

## Section 15.1: Euler-Lagrange Equations

**15.1.1** *(Deriving equations of motion)*

(a) Write the Lagrangian for a particle of mass $m$ in 3D in spherical coordinates $(r, \theta, \phi)$ with potential $V(r)$. Derive the three Euler-Lagrange equations. Identify the conserved quantities for the case $V = V(r)$ (central potential).

(b) Write the Lagrangian for a bead of mass $m$ on a frictionless wire in the shape of a helix: $x = R\cos\theta$, $y = R\sin\theta$, $z = c\theta$, where $\theta$ is the generalized coordinate. Find the equation of motion and the normal force exerted by the wire.

(c) Two masses $m_1$ and $m_2$ are connected by a massless string over a frictionless pulley (Atwood machine). Use the constraint that the string length is constant to reduce to one degree of freedom and find the acceleration. Verify Newton's law result.

**15.1.2** *(The geodesic as an Euler-Lagrange problem)*

The geodesic equation extremizes the length $\int \sqrt{g_{ij}\dot{q}^i\dot{q}^j}\,dt$ (equivalently, the energy $\int g_{ij}\dot{q}^i\dot{q}^j\,dt$).

(a) For the metric on the sphere $S^2$: $ds^2 = d\theta^2 + \sin^2\theta\,d\phi^2$, write the Lagrangian $L = \dot\theta^2 + \sin^2\theta\,\dot\phi^2$ and derive the geodesic equations (the Euler-Lagrange equations). Show that great circles ($\phi =$ const) are solutions.

(b) For the flat metric in polar coordinates ($ds^2 = dr^2 + r^2\,d\phi^2$), derive the geodesic equations. Show that they are equivalent to straight lines in Cartesian coordinates.

(c) For the Schwarzschild metric (coordinates $(t, r, \theta, \phi)$ with $\theta = \pi/2$):

$$L = -\left(1 - \frac{2GM}{r}\right)\dot{t}^2 + \left(1 - \frac{2GM}{r}\right)^{-1}\dot{r}^2 + r^2\dot{\phi}^2$$

Identify the two conserved quantities from the cyclic coordinates. Derive the effective potential.

**15.1.3** *(Lagrange multipliers)*

A pendulum of mass $m$ and length $\ell$ swings in a vertical plane, with the pivot constrained to move horizontally (frictionless rail). The horizontal position of the pivot is $X(t)$ (given). Write the Lagrangian using $\theta$ (angle from vertical) as the generalized coordinate, eliminating the constraint. Derive the equation of motion and show it reduces to $\ddot\theta + (g/\ell)\sin\theta = -(\ddot X/\ell)\cos\theta$. Interpret the $\ddot X$ term as a fictitious force.

---

## Section 15.2: Noether's Theorem

**15.2.1** *(Deriving conservation laws from symmetries)*

(a) For a system of two particles with Lagrangian $L = \frac{1}{2}m_1|\dot{\mathbf{r}}_1|^2 + \frac{1}{2}m_2|\dot{\mathbf{r}}_2|^2 - V(|\mathbf{r}_1 - \mathbf{r}_2|)$:
- Apply Noether's theorem to the symmetry $\mathbf{r}_i \to \mathbf{r}_i + \varepsilon\mathbf{n}$ (translation in direction $\mathbf{n}$). What is the conserved Noether charge?
- Apply Noether's theorem to the symmetry $t \to t + \varepsilon$. What is the conserved charge?
- Is angular momentum conserved? Identify the corresponding symmetry.

(b) For the Lagrangian of a free particle in Minkowski spacetime: $L = -mc^2\sqrt{1 - \dot{\mathbf{r}}^2/c^2}$, apply Noether's theorem to time translation, spatial translation, rotation, and Lorentz boost. What are the four conserved charges?

**15.2.2** *(Gauge symmetry and Noether's second theorem)*

The electromagnetic Lagrangian density $\mathcal{L} = -\frac{1}{4}F_{\mu\nu}F^{\mu\nu}$ where $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$. Under the gauge transformation $A_\mu \to A_\mu + \partial_\mu\chi$ (for any function $\chi$):

(a) Show $F_{\mu\nu}$ is gauge invariant.
(b) The Euler-Lagrange equations give $\partial_\mu F^{\mu\nu} = 0$. Show these are not independent — specifically, show $\partial_\nu(\partial_\mu F^{\mu\nu}) = 0$ is an identity. This is Noether's *second* theorem applied to gauge symmetry.
(c) This identity corresponds to charge conservation when a charged source is added: $\partial_\mu F^{\mu\nu} = J^\nu$ implies $\partial_\nu J^\nu = 0$. Verify.

**15.2.3** *(Energy conservation and Killing vectors)*

In GR, the conserved energy along a geodesic in a spacetime with a Killing vector $\xi^\mu = (\partial/\partial t)^\mu$ is $E = -g_{\mu\nu}u^\mu\xi^\nu$.

(a) Show that $E$ is constant along a geodesic: $\frac{d}{d\tau}(g_{\mu\nu}u^\mu\xi^\nu) = 0$, using the geodesic equation and the Killing equation $\nabla_{(\mu}\xi_{\nu)} = 0$.

(b) For the Schwarzschild metric, $\xi^\mu = (1, 0, 0, 0)$. Compute $E$ explicitly and show it equals $-(1-2GM/r)c^2\dot{t}$ (where the dot is $d/d\tau$).

(c) In the Newtonian limit $GM/r \ll c^2$, show $E \approx mc^2 + T + V$ (rest energy plus kinetic plus potential). What happens to the $mc^2$ term?

---

## Section 15.3: Hamiltonian Mechanics

**15.3.1** *(From Lagrangian to Hamiltonian)*

(a) For the simple harmonic oscillator $L = \frac{1}{2}m\dot{q}^2 - \frac{1}{2}k q^2$, compute $p = \partial L/\partial \dot{q}$, invert to get $\dot{q}(p)$, and compute $H = p\dot{q} - L$. Write Hamilton's equations and verify they give $\ddot{q} + \omega^2 q = 0$.

(b) For the Lagrangian of a charged particle in electromagnetic fields: $L = \frac{1}{2}m\dot{\mathbf{r}}^2 + q\dot{\mathbf{r}}\cdot\mathbf{A} - q\phi$ (Gaussian units). Compute the canonical momentum $\mathbf{p} = \partial L/\partial\dot{\mathbf{r}}$. Note $\mathbf{p} \neq m\dot{\mathbf{r}}$ — it includes the field contribution. Compute $H$.

(c) For a relativistic free particle: $L = -mc^2/\gamma = -mc^2\sqrt{1-v^2/c^2}$. Find $\mathbf{p} = \partial L/\partial \mathbf{v}$. Find $H$ by Legendre transform. Express $H$ as a function of $\mathbf{p}$ (not $\mathbf{v}$) — you should get $H = \sqrt{m^2c^4 + p^2c^2}$.

**15.3.2** *(Poisson brackets)*

(a) Verify the Poisson bracket relations $\{q^i, q^j\} = 0$, $\{p_i, p_j\} = 0$, $\{q^i, p_j\} = \delta^i_j$.

(b) For $H = p^2/(2m) + V(q)$: compute $\{q, H\}$ and $\{p, H\}$ and verify they give Hamilton's equations.

(c) For the angular momentum components $L_x = yp_z - zp_y$, $L_y = zp_x - xp_z$, $L_z = xp_y - yp_x$: compute $\{L_x, L_y\}$, $\{L_y, L_z\}$, $\{L_z, L_x\}$. The algebra you find is $\mathfrak{so}(3)$ — the Lie algebra of rotations. In quantum mechanics, this becomes $[L_x, L_y] = i\hbar L_z$, etc. (the angular momentum algebra).

(d) Compute $\{L^2, L_z\}$ where $L^2 = L_x^2 + L_y^2 + L_z^2$. What does this imply about simultaneous measurement of $L^2$ and $L_z$ in quantum mechanics?

**15.3.3** *(Integrable systems and the Kepler problem)*

The Kepler problem has the Hamiltonian $H = p^2/(2m) - GMm/r$ and two additional integrals: $\mathbf{L} = \mathbf{r} \times \mathbf{p}$ (angular momentum) and $\mathbf{A} = \mathbf{p} \times \mathbf{L} - GMm^2\hat{\mathbf{r}}$ (LRL vector).

(a) Show $\{H, L_z\} = 0$ and $\{H, A_z\} = 0$ (these are conserved).
(b) Show $\{L_i, A_j\} = \varepsilon_{ijk}A_k$ (the Poisson bracket between $\mathbf{L}$ and $\mathbf{A}$ generates $\mathbf{A}$, reflecting the symmetry of the Kepler problem under $SO(4)$).
(c) In action-angle variables for the Kepler problem, the three action variables are $J_r$ (radial), $J_\theta$ (polar), $J_\phi$ (azimuthal). The Hamiltonian depends only on $J = J_r + J_\theta + J_\phi$ (total), giving frequencies $\omega_r = \omega_\theta = \omega_\phi$. What does this equal-frequency condition imply about the orbit?

---

## Thought Experiments

**TE 15.1: What is the Action?**
The action $S = \int_{t_1}^{t_2} L\,dt$ is not energy, momentum, or any other obviously physical quantity. Yet the laws of physics are determined by making $S$ stationary.

(a) What are the dimensions of $S$? (It has dimensions of energy × time = angular momentum.) In quantum mechanics, $S/\hbar$ appears as a phase — this is the connection between the classical principle of stationary action and Feynman's path integral.

(b) Consider two paths from $(t_1, q_1)$ to $(t_2, q_2)$: the physical path and a nearby non-physical path. The physical path makes $S$ stationary (not necessarily minimal). Give an example where the physical path is a saddle point of $S$, not a minimum.

(c) In the semiclassical (WKB) approximation, quantum amplitudes receive contributions from all paths, weighted by $e^{iS/\hbar}$. Paths near the classical trajectory contribute coherently (they all have nearly the same phase); paths far away cancel. This is why the classical path is the most probable one in the quantum limit. How large must $S/\hbar$ be for the classical approximation to be valid?

**TE 15.2: Phase Space and the Liouville Theorem**
A swarm of 10⁶ particles all start in a small box in phase space at $t = 0$. By Liouville's theorem, the phase-space volume they occupy is constant in time.

(a) Does this mean the spatial density of particles is constant? (Consider: a Hamiltonian system can focus particles in position space by defocusing them in momentum space.)
(b) A laser beam is a highly ordered system (small phase-space volume). A thermal source emits in all directions (large phase-space volume). Liouville's theorem says you cannot decrease phase-space volume. What does this imply about the possibility of focusing thermal light to the intensity of laser light using optical elements (mirrors, lenses)?
(c) Connect to the second law of thermodynamics: why does entropy increase even though Liouville's theorem says phase-space volume is conserved?

---

## Laboratory Explorations

**Lab 15.1: Testing Conservation Laws with a Pendulum**
A compound pendulum (rigid rod, not point mass) has a period that depends on its moment of inertia. (a) Derive the Lagrangian for a uniform rod of length $L$ and mass $M$ pivoting about one end. The kinetic energy involves the moment of inertia $I = ML^2/3$. Find the period for small oscillations. (b) Measure the period as a function of length by building pendula of different lengths. (c) Verify energy conservation: measure the maximum angle $\theta_{\max}$ and verify that the speed at the bottom $v_{\max} = \sqrt{2gL(1-\cos\theta_{\max})}$.

**Lab 15.2: Phase Portraits**
Using a pendulum or oscillator, map out the phase portrait $(q, \dot{q})$: (a) For small amplitudes, the phase portrait should be approximately elliptical (harmonic oscillator). (b) For large amplitudes, it should distort. (c) The separatrix (boundary between libration and rotation) passes through the unstable equilibrium $(\pi, 0)$. Measure the period as a function of amplitude and look for divergence as $\theta_{\max} \to \pi$. The period diverges logarithmically — the pendulum takes infinite time to reach the top.
