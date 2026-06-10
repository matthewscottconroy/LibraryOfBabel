# Chapter 20: Important Concepts

---

**4-Velocity**
$u^\mu = dx^\mu/d\tau = \gamma(c, \mathbf{v})$: the Lorentz covariant velocity vector. Normalized: $u_\mu u^\mu = \eta_{\mu\nu}u^\mu u^\nu = -c^2$ (constant). In the rest frame: $u^\mu = (c, 0, 0, 0)$. Always timelike; always "pointing into the future." For massless particles: not defined (use $k^\mu = dx^\mu/d\lambda$ with $k_\mu k^\mu = 0$).

**4-Momentum**
$p^\mu = mu^\mu = (E/c, \mathbf{p})$: encodes both energy and 3-momentum in a single 4-vector. Lorentz-covariant. Conservation: $\sum_i p^\mu_i = \text{const}$ in collisions (replaces separate energy and 3-momentum conservation). The mass-shell condition $p_\mu p^\mu = -m^2c^2$ is Lorentz-invariant.

**Energy-Momentum Relation**
$E^2 = \mathbf{p}^2c^2 + m^2c^4$: the relativistic relation between energy, momentum, and rest mass. Reduces to $E = mc^2$ at rest, $E = |\mathbf{p}|c$ for massless particles, $E \approx mc^2 + \mathbf{p}^2/(2m)$ for $|\mathbf{p}| \ll mc$. The Klein-Gordon equation $(\Box + m^2c^2/\hbar^2)\phi = 0$ is this equation in quantum mechanics (with $E \to i\hbar\partial_t$, $\mathbf{p} \to -i\hbar\nabla$).

**Mass-Energy Equivalence**
$E = mc^2$ (at rest): rest mass and rest energy are the same thing. All forms of energy have gravitational mass. Nuclear binding energy is mass-energy. Pair production converts photon energy into mass-energy. The mass of the proton ($938$ MeV/$c^2$) is mostly QCD binding energy, not quark rest masses.

**4-Force**
$f^\mu = dp^\mu/d\tau = ma^\mu$: the Lorentz covariant force. Satisfies $f_\mu u^\mu = 0$ (orthogonality to 4-velocity, for constant rest mass). The electromagnetic 4-force: $f^\mu = qF^{\mu\nu}u_\nu$. In GR, the geodesic equation $D^2x^\mu/d\tau^2 = 0$ replaces Newton's first law for freely falling particles.

**Geodesic Equation (Free Particle)**
$d^2x^\mu/d\tau^2 + \Gamma^\mu_{\nu\rho}(dx^\nu/d\tau)(dx^\rho/d\tau) = 0$: the covariant equation for a freely falling particle. Reduces to $d^2\mathbf{x}/dt^2 = 0$ in flat spacetime Cartesian coordinates. The Christoffel symbols encode the inertial forces in non-inertial frames and the gravitational "force" in curved spacetime (which GR reveals to be spacetime curvature, not a force at all).

**Stress-Energy Tensor**
$T^{\mu\nu}$: a symmetric rank-2 tensor encoding the density and flux of energy and momentum. $T^{00}$ = energy density; $T^{0i}$ = momentum density × $c^2$ = energy flux/c; $T^{ij}$ = stress (pressure + shear). The source of gravity in GR: $G_{\mu\nu} = 8\pi G T_{\mu\nu}$. Every matter field has a stress-energy tensor.

**Conservation Law $\partial_\mu T^{\mu\nu} = 0$**
Local conservation of energy and momentum in flat spacetime. For $\nu = 0$: energy conservation. For $\nu = i$: momentum conservation. In curved spacetime: $\nabla_\mu T^{\mu\nu} = 0$ — a consequence of the Einstein equations and the contracted Bianchi identity. The covariant conservation is what allows energy to be exchanged with the gravitational field.

**Dust**
$T^{\mu\nu}_{\rm dust} = \rho_0 u^\mu u^\nu$: the stress-energy of pressureless matter ($p = 0$). The simplest model. $\nabla_\mu T^{\mu\nu}_{\rm dust} = 0$ gives geodesic motion (dust particles are freely falling) and continuity (particle number conservation). Used in cosmological models as the stress-energy for "matter" after radiation-matter equality.

**Perfect Fluid**
$T^{\mu\nu}_{\rm fluid} = (\epsilon + p)u^\mu u^\nu + pg^{\mu\nu}$: the stress-energy of an ideal fluid with energy density $\epsilon$ and pressure $p$, no viscosity, no heat flow. The equation of state $p(\epsilon)$ determines the dynamics. Examples: dust ($p = 0$), radiation ($p = \epsilon/3$), cosmological constant ($p = -\epsilon$).

**Energy Conditions**
Constraints on $T_{\mu\nu}$: weak energy condition (WEC): $T_{\mu\nu}u^\mu u^\nu \geq 0$ for all timelike $u^\mu$ (energy density non-negative for all observers); null energy condition (NEC): $T_{\mu\nu}k^\mu k^\nu \geq 0$ for null $k^\mu$; strong energy condition (SEC): $(T_{\mu\nu} - \frac{1}{2}g_{\mu\nu}T)u^\mu u^\nu \geq 0$; dominant energy condition (DEC): WEC + energy flux is timelike or null. The singularity theorems require SEC or NEC. Dark energy (cosmological constant) violates SEC. Exotic matter for traversable wormholes would require NEC violation.

**Rapidity**
$\phi = \tanh^{-1}(v/c)$: parametrizes Lorentz boosts as hyperbolic rotations ($\gamma = \cosh\phi$, $\beta\gamma = \sinh\phi$). Rapidities add linearly when boosts are combined: $\phi_{12} = \phi_1 + \phi_2$. The analog of the rotation angle for spatial rotations. Used in high-energy physics where $v \approx c$ and velocities are nearly additive in rapidity space.
