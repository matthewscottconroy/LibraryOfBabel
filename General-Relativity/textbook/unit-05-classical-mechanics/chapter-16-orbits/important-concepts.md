# Chapter 16: Important Concepts

---

**Central Force**
A force $\mathbf{F} = F(r)\hat{\mathbf{r}}$ directed radially, with magnitude depending only on $r$. Zero torque; angular momentum conserved; orbit confined to a plane. Prototype for all two-body gravitational and electromagnetic problems. In GR, the "central force" analog is the Schwarzschild effective potential — also radially symmetric but with an extra GR term.

**Areal Velocity and Kepler's Second Law**
$dA/dt = \ell/(2m)$ is constant for any central force. The planet sweeps equal areas in equal times — not because of the specific force law, but because of angular momentum conservation. Kepler discovered this empirically (1609); Newton proved it (1687) as a consequence of $\dot{\mathbf{L}} = 0$.

**Effective Potential**
$V_{\rm eff}(r) = V(r) + \ell^2/(2mr^2)$: the sum of gravitational potential energy and centrifugal potential energy. Reduces the 2D central force problem to 1D motion. The structure of $V_{\rm eff}$ (number and stability of minima) completely determines orbit type: bound/unbound, circular/elliptic/hyperbolic.

**Binet's Equation**
$d^2u/d\phi^2 + u = -mF(1/u)/(\ell^2 u^2)$, where $u = 1/r$. Converts the 2D orbit equation to an ODE in $u(\phi)$. For the Newtonian force $F = -GMm/r^2$: RHS is constant, giving a simple harmonic oscillator with solution $u = (1 + e\cos\phi)/p$ — a conic section. The key tool for computing perihelion precession from perturbed potentials.

**Kepler's First Law**
Planetary orbits are ellipses with the Sun at one focus. Derived from Binet's equation for the inverse-square force. The shape of the orbit (eccentricity $e$) is determined by the energy and angular momentum: $e = \sqrt{1 + 2E\ell^2/(G^2M^2m^3)}$.

**Kepler's Third Law**
$T^2 = (4\pi^2/GM)a^3$: the period-squared is proportional to the semi-major axis cubed. The constant $4\pi^2/GM$ encodes the central mass. Used in reverse to measure masses: from $T$ and $a$, compute $M$. Applied to the galactic center stars, it gives the mass of Sagittarius A* ($4\times10^6 M_\odot$).

**Vis-Viva Equation**
$v^2 = GM(2/r - 1/a)$: the orbital speed at any point, depending only on the distance $r$ and the semi-major axis $a$ (not on the direction or the eccentricity separately). Derives from energy conservation. The escape velocity ($a \to \infty$): $v_{\rm esc} = \sqrt{2GM/r}$.

**Laplace-Runge-Lenz Vector**
$\mathbf{A} = \mathbf{p}\times\mathbf{L} - GMm^2\hat{\mathbf{r}}$: a conserved vector pointing from the focus to the perihelion. Its conservation is equivalent to orbit closure and the $SO(4)$ symmetry of the Kepler problem. Any perturbation to the $1/r$ potential breaks conservation of $\mathbf{A}$, causing the perihelion to rotate — perihelion precession is the rate of rotation of $\mathbf{A}$.

**Bertrand's Theorem**
Only two central force laws produce closed orbits for all energies: $F \propto r$ (harmonic oscillator) and $F \propto 1/r^2$ (Newtonian gravity). Any other power law gives open (precessing) orbits. Proved by Bertrand (1873). Both exceptions correspond to systems with enhanced symmetry ($U(n)$ and $SO(4)$ respectively) and extra conserved quantities.

**GR Correction to Newtonian Potential**
From the Schwarzschild geodesic equations, the effective potential acquires a GR term $-GM\ell^2/(c^2 r^3)$. This term dominates at small $r$, destroys the stability of all circular orbits inside the ISCO ($r < 6GM/c^2$), and causes a secular perihelion advance for all bound orbits.

**Perihelion Precession Formula**
$\Delta\phi = 6\pi GM/(c^2 a(1-e^2))$ per orbit: the GR precession of the perihelion due to the $-GM\ell^2/(c^2 r^3)$ correction. Derived via first-order perturbation theory in Binet's equation. For Mercury: 42.98 arcsec/century. First computed by Einstein in November 1915 — his result matched Le Verrier's unexplained residual exactly.

**Mercury's Precession Budget**
Total observed precession: 574.1 arcsec/century. Newtonian contributions (mainly Venus, Jupiter, Earth): 530.8 arcsec/century. GR: 42.98 arcsec/century. Solar oblateness: 0.025 arcsec/century. Total predicted: 573.8 arcsec/century. Agreement with observation: $\lesssim 0.5$ arcsec/century. One of the most precise tests of GR in the solar system.

**Resonant Forcing**
In perturbation theory for Binet's equation: forcing at frequency 1 (resonant forcing) produces a secularly growing response $\phi\sin\phi$ rather than an oscillatory correction. This secular growth corresponds to a slow precession of the perihelion. The resonant term in the GR perturbation is $6(GM/c^2)u_0^2 e\cos\phi$ — the $\cos\phi$ forcing drives the secular precession.

**Innermost Stable Circular Orbit (ISCO)**
The minimum radius for a stable circular orbit in Schwarzschild spacetime: $r_{\rm ISCO} = 6GM/c^2 = 3r_s$ (three Schwarzschild radii). Below this radius, the effective potential has no local minimum; circular orbits exist but are unstable. For a black hole, the ISCO marks the inner edge of an accretion disk. Its location (measurable from X-ray spectroscopy) depends on black hole spin and tests GR near the event horizon.
