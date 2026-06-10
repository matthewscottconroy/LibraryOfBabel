# Section 16.1: Central Force Motion

---

## Section Introduction

A central force is one that points radially toward or away from a fixed center, with magnitude depending only on distance: $\mathbf{F} = F(r)\hat{\mathbf{r}}$. Gravity is a central force. So is the Coulomb force, the spring force in 3D, and to a good approximation, any force between two spherically symmetric bodies.

Central force motion has a remarkable simplicity: the three-dimensional problem reduces to an effectively one-dimensional problem. Angular momentum conservation confines the orbit to a plane; the remaining radial motion is equivalent to a particle moving in an effective potential $V_{\rm eff}(r)$. The topological structure of this potential — how many minima it has, where they are, how deep they are — completely determines the character of the orbit.

This reduction is the prototype for the analogous reduction in GR. The Schwarzschild spacetime has two Killing vectors ($\partial_t$ and $\partial_\phi$), giving conserved energy E and angular momentum L. The geodesic equations then reduce to a one-dimensional problem in $r$, governed by the Schwarzschild effective potential. Every feature of this section has a direct GR analog.

---

## 16.1.1 Reduction to Two Dimensions

A central force $\mathbf{F} = F(r)\hat{\mathbf{r}}$ has zero torque: $\boldsymbol{\tau} = \mathbf{r} \times \mathbf{F} = r\hat{\mathbf{r}} \times F(r)\hat{\mathbf{r}} = 0$. Therefore angular momentum $\mathbf{L} = m\mathbf{r} \times \dot{\mathbf{r}}$ is conserved.

Conservation of $\mathbf{L}$ (as a vector, not just its magnitude) implies $\mathbf{r} \perp \mathbf{L}$ and $\dot{\mathbf{r}} \perp \mathbf{L}$ at all times. The orbit lies in the plane through the origin perpendicular to $\mathbf{L}$. We can therefore choose polar coordinates $(r, \phi)$ in this orbital plane, with:

- Position: $\mathbf{r} = r\hat{\mathbf{r}}$
- Velocity: $\dot{\mathbf{r}} = \dot{r}\hat{\mathbf{r}} + r\dot\phi\hat{\boldsymbol{\phi}}$
- Kinetic energy: $T = \frac{1}{2}m(\dot{r}^2 + r^2\dot\phi^2)$
- Angular momentum: $\ell = mr^2\dot\phi$ (the specific angular momentum per unit mass is sometimes written $h = r^2\dot\phi = \ell/m$)

---

## 16.1.2 The Effective Potential

The equations of motion in polar coordinates are:

$$m\ddot{r} - mr\dot\phi^2 = F(r)$$
$$\frac{d}{dt}(mr^2\dot\phi) = 0 \Rightarrow \ell = mr^2\dot\phi = \text{const}$$

Using $\dot\phi = \ell/(mr^2)$ to eliminate $\phi$ from the radial equation:

$$m\ddot{r} = F(r) + \frac{\ell^2}{mr^3}$$

This is equivalent to 1D motion in the **effective potential**:

$$V_{\rm eff}(r) = V(r) + \frac{\ell^2}{2mr^2}$$

where $V(r)$ is the potential energy (so $F(r) = -dV/dr$) and $\ell^2/(2mr^2)$ is the centrifugal potential energy. Energy conservation gives:

$$E = \frac{1}{2}m\dot{r}^2 + V_{\rm eff}(r)$$

The radial motion is that of a particle with energy $E$ in the 1D potential $V_{\rm eff}(r)$.

**Turning points**: The particle is constrained to $r$ values where $E \geq V_{\rm eff}(r)$. At a turning point, $\dot{r} = 0$ and $E = V_{\rm eff}(r)$.

---

## 16.1.3 Orbit Classification

The topology of $V_{\rm eff}(r)$ determines the orbit type. For an attractive force law $V(r) = -k/r^n$ ($k > 0$, $n > 0$) with $\ell \neq 0$, the centrifugal barrier always wins at small $r$ (for $n < 2$), creating a minimum in $V_{\rm eff}$.

**For Newtonian gravity** ($V = -GMm/r$, $n = 1$):

$$V_{\rm eff}(r) = -\frac{GMm}{r} + \frac{\ell^2}{2mr^2}$$

- Minimum at $r_0 = \ell^2/(GMm^2)$ with $V_{\rm eff}(r_0) = -G^2M^2m^3/(2\ell^2)$
- $V_{\rm eff} \to 0$ as $r \to \infty$
- $V_{\rm eff} \to +\infty$ as $r \to 0^+$ (centrifugal barrier; no particle reaches $r = 0$ unless $\ell = 0$)

| Energy condition | Orbit type |
|-----------------|------------|
| $E = V_{\rm eff}(r_0)$ | Circular orbit (both turning points coincide) |
| $V_{\rm eff}(r_0) < E < 0$ | Elliptic orbit (two turning points $r_{\min}$, $r_{\max}$) |
| $E = 0$ | Parabolic orbit ($r_{\min}$ finite, $r \to \infty$) |
| $E > 0$ | Hyperbolic orbit ($r_{\min}$ finite, $r \to \infty$) |
| $\ell = 0$ | Radial (straight-line) fall into center |

**Circular orbits**: At the minimum of $V_{\rm eff}$, $dV_{\rm eff}/dr = 0$:

$$\frac{dV_{\rm eff}}{dr} = -\frac{dV}{dr} - \frac{\ell^2}{mr^3} = 0 \Rightarrow -F(r_0) = \frac{\ell^2}{mr_0^3}$$

This says that the centripetal force $m v_\phi^2/r_0 = \ell^2/(mr_0^3)$ equals the gravitational pull $|F(r_0)|$. The circular orbit speed satisfies $v_\phi^2 = r_0|F(r_0)|/m$.

**Stability of circular orbits**: The circular orbit is stable if and only if $d^2V_{\rm eff}/dr^2 > 0$ at $r_0$ — i.e., if it sits in a potential minimum, not a maximum.

For a power law $V = -k/r^n$: circular orbits are stable when $n < 2$. For $n \geq 2$, the effective potential has no minimum and no stable circular orbits exist. This is why orbital stability is sensitive to the power law.

**The case $n = 2$ (harmonice oscillator in disguise)**: $V = \frac{1}{2}kr^2$. Circular orbits exist (and are stable), and the orbit is an ellipse centered on the force center (not at a focus). This is Bertrand's theorem territory — see Section 16.2.

---

## 16.1.4 Areal Velocity and Kepler's Second Law

The **areal velocity** is the rate at which the position vector sweeps out area:

$$\frac{dA}{dt} = \frac{1}{2}r^2\dot\phi = \frac{\ell}{2m}$$

Since $\ell$ is constant, the areal velocity is constant. This is **Kepler's second law**: equal areas are swept in equal times. Kepler discovered this empirically in 1609; Newton proved it in the *Principia* (1687) as a direct consequence of angular momentum conservation for any central force.

*Proof by geometry*: In time $dt$, the position vector sweeps out a triangle with base $r\,d\phi$ and height $r$, with area $dA = \frac{1}{2}r^2\,d\phi = \frac{\ell}{2m}\,dt$. □

Kepler's second law holds for any central force, regardless of the specific force law. It is a direct consequence of angular momentum conservation.

---

## 16.1.5 Orbit Equation and Binet's Formula

The orbit equation $r(\phi)$ is found by eliminating $t$ using $\ell = mr^2\dot\phi$. The trick is the substitution $u = 1/r$:

$$\dot{r} = \frac{dr}{dt} = \frac{dr}{d\phi}\cdot\frac{d\phi}{dt} = -\frac{1}{u^2}\frac{du}{d\phi}\cdot\frac{\ell u^2}{m} = -\frac{\ell}{m}\frac{du}{d\phi}$$

$$\ddot{r} = -\frac{\ell}{m}\frac{d^2u}{d\phi^2}\cdot\dot\phi = -\frac{\ell^2 u^2}{m^2}\frac{d^2u}{d\phi^2}$$

Substituting into $m\ddot{r} = F(1/u) + \ell^2 u^3/m$:

$$-\frac{\ell^2 u^2}{m}\frac{d^2u}{d\phi^2} = F(1/u) + \frac{\ell^2 u^3}{m}$$

Rearranging: **Binet's equation**:

$$\frac{d^2u}{d\phi^2} + u = -\frac{m}{\ell^2 u^2}F(1/u)$$

For a power-law force $F = -k/r^n = -ku^n$: right-hand side $= kmu^{n-2}/\ell^2$.

- **$n = 2$ (inverse square)**: RHS $= km/\ell^2 = $ const. Solution: $u = (1 + e\cos\phi)/p$ (conic, closed orbit).
- **$n = 3$ (inverse cube)**: RHS $= kmu/\ell^2$. Solution: $u \propto \cos(\alpha\phi)$ for some $\alpha$. This gives a spiral orbit — not closed.
- **$n = 1$ (linear)**: RHS $= kmu^{-1}/\ell^2 = km/(u\ell^2)$. Different character.

Binet's equation will be the key tool in Section 16.3 for computing the perihelion precession due to the GR correction.

---

## 16.1.6 The Schwarzschild Effective Potential

For completeness, the analogous reduction in GR (Schwarzschild spacetime, $c = G = 1$):

$$V_{\rm eff}^{\rm Schwarzschild}(r) = -\frac{M}{r} + \frac{\ell^2}{2r^2} - \frac{M\ell^2}{r^3}$$

The three terms are: Newtonian gravity, centrifugal barrier, and the GR correction. The GR correction term $-M\ell^2/r^3$ dominates at small $r$ and overwhelms the centrifugal barrier below $r = 3M$ (the photon sphere at $r = 3r_s/2$ for massless particles, and the ISCO at $r = 6M = 3r_s$ for massive particles).

Below the ISCO, $d^2V_{\rm eff}/dr^2 < 0$: no stable circular orbits exist. Particles perturbed inward of the ISCO spiral into the black hole. This has direct observational consequences for accretion disk physics and the X-ray spectrum of black holes — the inner edge of the accretion disk is predicted to terminate at the ISCO, and the corresponding spectral cutoff has been observed [Reynolds (2021), *Annual Review of Astronomy and Astrophysics*].

---

## References

- Newton, I. (1687). *Philosophiæ Naturalis Principia Mathematica.* Book I, Section II, Prop. I. London. [The proof that Kepler's second law (equal areas) is equivalent to zero torque, i.e., central force. Newton's proof precedes our angular momentum language by two centuries but is mathematically equivalent.]
- Bertrand, J. (1873). "Théorème relatif au mouvement d'un point attiré vers un centre fixe." *Comptes rendus de l'Académie des sciences*, 77, 849–853. [Only $F \propto r$ and $F \propto 1/r^2$ give all closed orbits. All other power laws give open (precessing) orbits for most energies.]
- Binet, J.P.M. (1841). "Mémoire sur l'intégration de l'équation qui donne la courbe décrite par une force quelconque." *Journal de l'École Polytechnique*, 17, 1–47. [Binet's formula (the orbit equation in terms of $u = 1/r$). A remarkable reduction that linearizes the orbit equation for inverse-square forces.]
- Reynolds, C.S. (2021). "Observational evidence for black hole spin." *Annual Review of Astronomy and Astrophysics*, 59, 107–154. [The ISCO as the inner edge of accretion disks; measurement of black hole spin from the disk's inner edge position.]
