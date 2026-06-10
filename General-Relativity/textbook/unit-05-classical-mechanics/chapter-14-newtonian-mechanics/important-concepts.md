# Chapter 14: Important Concepts

---

**Newton's Three Laws**
The foundation of classical mechanics: (I) a body continues in uniform motion unless acted on by a force; (II) $F = ma$; (III) action and reaction are equal and opposite. Law I distinguishes inertial frames; Law II defines force operationally; Law III ensures momentum conservation. In GR, freely-falling frames are inertial to first order (but not second — tidal forces), and the concept of "force" is replaced by spacetime geometry.

**Absolute vs. Relative Space and Time**
Newton's absolute space is a fixed background against which motion is defined; his absolute time flows uniformly everywhere. Leibniz and Mach objected: only relative positions and motions are physical. Einstein's GR implements Mach's insight: the metric $g_{\mu\nu}$ (the geometric structure of spacetime) is dynamical, determined by matter, and has no fixed background.

**Galilean Invariance**
The symmetry group of Newtonian mechanics: 10-parameter group (3 rotations, 3 translations, 3 velocity boosts, 1 time translation). Any two inertial observers related by a Galilean transformation make the same predictions. Maxwell's equations break Galilean invariance — they are invariant under the Lorentz group instead — which motivated the special theory of relativity.

**Weak Equivalence Principle (WEP)**
All bodies fall with the same acceleration in a gravitational field, regardless of their composition. Equivalently: gravitational mass = inertial mass. Tested to $10^{-13}$ (Schlamminger et al. 2008). WEP is the cornerstone of GR: it implies that gravity can be locally "cancelled" by free fall, and that spacetime curvature (not a force) is the correct description.

**Conservation of Energy**
For a conservative force $\mathbf{F} = -\nabla V$: the total mechanical energy $E = T + V$ is constant. Consequence of time-translation symmetry (Noether's theorem). In GR, energy is conserved for spacetimes with a timelike Killing vector (e.g., Schwarzschild), but not in general (e.g., FLRW cosmology).

**Conservation of Linear Momentum**
Total momentum $\mathbf{P} = \sum_i m_i \dot{\mathbf{r}}_i$ is conserved when the total external force vanishes. The center of mass moves uniformly. Consequence of spatial translation symmetry.

**Conservation of Angular Momentum**
$\mathbf{L} = \sum_i m_i \mathbf{r}_i \times \dot{\mathbf{r}}_i$ is conserved when total external torque vanishes. For a central force, $\mathbf{L}$ is conserved for each particle, confining its orbit to a plane. Consequence of rotational symmetry. In GR: Schwarzschild has a Killing vector for rotations, giving conserved angular momentum for geodesics.

**Virial Theorem**
For a time-averaged bounded system with $V \propto r^n$: $\langle T \rangle = \frac{n}{2}\langle V \rangle$. For gravity ($n = -1$): $\langle T \rangle = -E$ (kinetic energy = -total energy). Implies stars have negative heat capacity: as a star cools (loses energy), it heats up. Used to estimate cluster masses from velocity dispersions.

**Gravitational Potential $\Phi$**
Scalar field satisfying Poisson's equation $\nabla^2\Phi = 4\pi G\rho$; the force is $\mathbf{g} = -\nabla\Phi$. The GR analog: the metric component $g_{00} = -(1 + 2\Phi/c^2)$ in the weak-field limit, and the full nonlinear Einstein equations reduce to Poisson's equation when $v \ll c$.

**Poisson's Equation**
$\nabla^2\Phi = 4\pi G\rho$: the Newtonian field equation for gravity. Its solution is $\Phi = -G\int\rho(\mathbf{r}')/|\mathbf{r}-\mathbf{r}'|\,d^3r'$ via the Green's function. Compare to the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$, which reduce to Poisson's equation in the Newtonian limit.

**Multipole Expansion**
The potential far from a localized source decomposes as $\Phi = -GM/r$ (monopole) $-G\mathbf{D}\cdot\hat{\mathbf{r}}/r^2$ (dipole, zero in CM frame) $- \ldots$ The quadrupole term $J_2$ from Earth's oblateness causes satellite orbit precession. The time-varying quadrupole moment is the leading term in gravitational wave emission: $h \sim G\ddot{Q}_{ij}/c^4 r$.

**Tidal Forces**
The relative acceleration of two nearby freely-falling particles: $\ddot{\xi}^i = -(\partial^2\Phi/\partial r^i\partial r^j)\xi^j$. The tidal tensor is the Hessian of $\Phi$; it is traceless in vacuum (Laplace's equation). In GR, tidal forces become the geodesic deviation: $D^2\xi^\mu/d\tau^2 = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$. Tidal forces are curvature.

**Effective Potential**
In central force problems: $V_{\rm eff}(r) = V(r) + \ell^2/(2mr^2)$. The centrifugal term creates a potential barrier preventing $r \to 0$ (for $\ell \neq 0$). The minimum of $V_{\rm eff}$ gives circular orbits. In GR (Schwarzschild): $V_{\rm eff}^{\rm GR} = -GM/r + \ell^2/(2r^2) - GM\ell^2/r^3$, and the ISCO at $r = 6GM/c^2$ is where $V_{\rm eff}$ no longer has a minimum.

**Newton's Principia (1687)**
Newton's magnum opus: derives Kepler's laws from the inverse-square law, proves the shell theorem, develops the theory of tides, explains the precession of Earth's axis, and predicts the shape of the Earth. Sets the standard for mathematical physics for three centuries. Its Scholium contains Newton's definitions of absolute space and time — the metaphysical framework that Einstein would overturn.
