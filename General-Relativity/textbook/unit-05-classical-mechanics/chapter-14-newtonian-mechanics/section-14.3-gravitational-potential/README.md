# Section 14.3: The Newtonian Gravitational Potential

---

## Section Introduction

Newton's law of gravitation — a force that acts instantaneously across arbitrarily large distances — is mathematically elegant but physically puzzling. How does the Earth know the Sun is there? The potential theory introduced in this section provides a partial answer: the force can be encoded in a scalar field $\Phi(\mathbf{r})$ that permeates space, and the dynamics determined locally by $\mathbf{F} = -m\nabla\Phi$.

But this is not a complete answer. The potential still acts instantaneously — a change at the Sun propagates to Earth in zero time. This instantaneous action is Newtonian gravity's deepest flaw, and it is precisely what GR repairs. In GR, the gravitational potential is replaced by the metric field $g_{\mu\nu}$, perturbations in which travel at the speed of light (gravitational waves). The Newtonian potential $\Phi$ is recovered as the $h_{00}$ component of the metric in the weak-field slow-motion limit: $g_{00} = -(1 + 2\Phi/c^2)$.

Understanding the Poisson equation and multipole expansion here makes the transition to the linearized Einstein equations in Unit IX direct and natural.

---

## 14.3.1 The Gravitational Potential

**Newton's law**: The gravitational force on a test mass $m$ at $\mathbf{r}$ due to a source mass $M$ at the origin is:

$$\mathbf{F} = -\frac{GMm}{r^2}\hat{\mathbf{r}} = -\frac{GMm}{r^3}\mathbf{r}$$

This is conservative: $\mathbf{F} = -m\nabla\Phi$ where the **gravitational potential** is:

$$\Phi(\mathbf{r}) = -\frac{GM}{r}$$

**Superposition**: For a distribution of mass, the potential is linear in the source:

$$\Phi(\mathbf{r}) = -G \int \frac{\rho(\mathbf{r}')}{|\mathbf{r} - \mathbf{r}'|}\, d^3r'$$

where $\rho(\mathbf{r}')$ is the mass density. (Linearity fails in GR: the Einstein equations are nonlinear, and the mass-energy of the gravitational field itself contributes to gravity.)

**Units**: $[\Phi] = $ energy/mass = m²/s². The gravitational potential at Earth's surface is $\Phi \approx -6.25 \times 10^7$ m²/s², i.e., $|\Phi|/c^2 \approx 7 \times 10^{-10}$ — a very weak field.

---

## 14.3.2 Poisson's Equation

The gravitational field $\mathbf{g} = -\nabla\Phi$ satisfies:

$$\nabla \cdot \mathbf{g} = -4\pi G\rho$$

*Derivation*: For a point mass, $\mathbf{g} = -GM\hat{\mathbf{r}}/r^2$. We know (from the vector calculus of Section 8) that $\nabla \cdot (\hat{\mathbf{r}}/r^2) = 4\pi\delta^3(\mathbf{r})$. So:

$$\nabla \cdot \mathbf{g} = -GM \cdot 4\pi\delta^3(\mathbf{r}) = -4\pi G \cdot M\delta^3(\mathbf{r}) = -4\pi G\rho$$

For a continuous distribution, the same reasoning applies by superposition. Since $\mathbf{g} = -\nabla\Phi$:

$$\boxed{\nabla^2\Phi = 4\pi G\rho}$$

This is **Poisson's equation** for the gravitational potential. In vacuum ($\rho = 0$): $\nabla^2\Phi = 0$ (Laplace's equation).

**Comparison to Einstein's equations**: Poisson's equation is the Newtonian limit of Einstein's equations. The linearized metric $h_{00} = -2\Phi/c^2$ satisfies:

$$\nabla^2 h_{00} = \frac{16\pi G}{c^2}\rho$$

which matches $\nabla^2\Phi = 4\pi G\rho$ (with $h_{00} = -2\Phi/c^2$). The full nonlinear Einstein equations reduce to Poisson's equation in the limit $v \ll c$, $\Phi \ll c^2$.

**Solution via Green's function** (Section 11.4): The fundamental solution to $\nabla^2 G = \delta^3(\mathbf{r})$ is $G(\mathbf{r}) = -1/(4\pi|\mathbf{r}|)$. So:

$$\Phi(\mathbf{r}) = -G \int \frac{\rho(\mathbf{r}')}{|\mathbf{r} - \mathbf{r}'|}\, d^3r'$$

which is exactly the superposition formula. The gravitational potential is the convolution of the Green's function with the mass density.

---

## 14.3.3 The Multipole Expansion

For a localized mass distribution (all mass within $r' < R$), the potential at $r \gg R$ can be expanded in inverse powers of $r$. Using the expansion:

$$\frac{1}{|\mathbf{r} - \mathbf{r}'|} = \frac{1}{r}\sum_{\ell=0}^{\infty}\left(\frac{r'}{r}\right)^\ell P_\ell(\cos\theta')$$

where $\theta'$ is the angle between $\mathbf{r}$ and $\mathbf{r}'$, and $P_\ell$ are Legendre polynomials:

$$\Phi(\mathbf{r}) = -\frac{G}{r}\sum_{\ell=0}^{\infty}\frac{1}{r^\ell}\int \rho(\mathbf{r}')(r')^\ell P_\ell(\cos\theta')\, d^3r'$$

The terms are:

**Monopole** ($\ell = 0$): $\Phi_0 = -GM/r$ where $M = \int\rho\, d^3r$ is the total mass. This dominates at large distances.

**Dipole** ($\ell = 1$): $\Phi_1 = -G\mathbf{D}\cdot\hat{\mathbf{r}}/r^2$ where $\mathbf{D} = \int\rho\mathbf{r}'\, d^3r'$ is the mass dipole. If the origin is at the center of mass, $\mathbf{D} = 0$.

**Quadrupole** ($\ell = 2$): $\Phi_2 = -G Q_{ij}\hat{r}^i\hat{r}^j/(2r^3)$ where $Q_{ij} = \int\rho(3r'_ir'_j - r'^2\delta_{ij})\, d^3r'$ is the quadrupole moment tensor.

**Solar oblateness**: The Sun is slightly oblate (equatorial bulge due to rotation). Its gravitational potential includes a quadrupole term parametrized by $J_2$:

$$\Phi_\odot = -\frac{GM_\odot}{r}\left[1 - J_2\left(\frac{R_\odot}{r}\right)^2 P_2(\cos\theta)\right]$$

where $J_2 \approx 2.2 \times 10^{-7}$ is the solar quadrupole moment. This contributes 0.025 arcseconds/century to Mercury's perihelion precession (Section 16.3.6) — tiny compared to the 42.98 from GR.

---

## 14.3.4 Energy of the Gravitational Field

The gravitational potential energy stored in a mass distribution is:

$$U = \frac{1}{2}\int\rho(\mathbf{r})\Phi(\mathbf{r})\, d^3r = -\frac{1}{8\pi G}\int|\nabla\Phi|^2\, d^3r$$

The second form (derived by integration by parts using Poisson's equation) expresses the energy as a **field energy density** $u = -|\mathbf{g}|^2/(8\pi G)$ — the energy is stored in the field, not in the masses. The negative sign reflects the fact that gravitational binding energy is negative (it costs energy to pull bound masses apart).

**Self-energy problem**: The field energy of a point mass ($\rho = M\delta^3(\mathbf{r})$) is infinite: $U = -\int_0^\infty (GM/r)^2 4\pi r^2 dr/(8\pi G) \propto \int_0^\infty dr/r^2 \to \infty$. This ultraviolet divergence is the Newtonian counterpart of the self-energy problems that plague classical field theories (and are regulated in QFT by renormalization).

**GR comparison**: In GR, gravitational energy cannot be localized — there is no gauge-invariant gravitational energy density. The total energy (ADM energy) of an asymptotically flat spacetime is well-defined and conserved, but it cannot be attributed to any specific region. This is the gravitational version of the gauge non-invariance of $|\mathbf{g}|^2/(8\pi G)$.

---

## 14.3.5 Tidal Forces and Geodesic Deviation

When two freely-falling particles are nearby, they accelerate relative to each other due to the gradient of the gravitational field. This **tidal acceleration** is:

$$\delta\ddot{r}^i = -\frac{\partial^2\Phi}{\partial r^i \partial r^j}\delta r^j = T^i_{\ j}\,\delta r^j$$

where $T_{ij} = -\partial^2\Phi/\partial r^i\partial r^j$ is the **tidal tensor** (Hessian of the potential).

For a point mass $\Phi = -GM/r$:

$$T_{ij} = \frac{GM}{r^3}\left(\delta_{ij} - 3\hat{r}_i\hat{r}_j\right)$$

A sphere of particles in free fall is distorted: compressed tangentially and stretched radially. This is the origin of ocean tides: the Moon stretches the Earth along the Earth-Moon line and compresses it transversely.

**GR counterpart**: The geodesic deviation equation describes the relative acceleration of nearby geodesics in curved spacetime:

$$\frac{D^2\xi^\mu}{d\tau^2} = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$$

where $\xi^\mu$ is the separation vector, $u^\nu$ is the 4-velocity, and $R^\mu_{\ \nu\rho\sigma}$ is the Riemann tensor. In the Newtonian limit, this reduces exactly to the tidal acceleration equation with $R^i_{\ 0j0} = \partial^2\Phi/\partial r^i\partial r^j$. **Tidal forces are curvature** — this is the direct physical interpretation of the Riemann tensor.

The tidal deformation of a laser interferometer by a passing gravitational wave (LIGO) is exactly the geodesic deviation equation in action. The metric perturbation $h_{\mu\nu}$ of a gravitational wave acts on the arm of the interferometer as a tidal force, stretching and compressing it with amplitude $\Delta L/L \sim h/2$.

---

## References

- Poisson, S.D. (1813). "Remarques sur une équation qui se présente dans la théorie des attractions des sphéroïdes." *Nouveau Bulletin des Sciences par la Société Philomathique de Paris*, 3, 388–392. [Poisson's equation: the generalization of Laplace's equation to include sources.]
- Laplace, P.S. (1799). *Mécanique Céleste*, Vol. 1. Paris. [The celestial mechanics of the solar system; the Laplace equation; the multipole expansion and spherical harmonics. The founding text of mathematical astronomy.]
- Gauss, C.F. (1840). *Allgemeine Lehrsätze in Beziehung auf die im verkehrten Verhältnisse des Quadrats der Entfernung wirkenden Anziehungs- und Abstossungs-Kräfte.* Leipzig. [Gauss's theorem and its application to gravitation — the first systematic treatment of potential theory.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [§1.3 on geodesic deviation and tidal forces; §17.4 on the Newtonian limit of the Einstein equations; §19.1 on Poisson's equation from Einstein's equations.]
- Chandrasekhar, S. (1969). *Ellipsoidal Figures of Equilibrium.* Yale University Press. [The multipole expansion and self-gravitating fluid equilibria; the J₂ coefficient and oblateness.]
