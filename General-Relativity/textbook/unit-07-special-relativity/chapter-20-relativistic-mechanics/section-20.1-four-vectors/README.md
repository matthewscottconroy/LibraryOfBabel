# Section 20.1: Four-Vectors and Relativistic Kinematics

---

## Section Introduction

A 4-vector is any quantity that transforms like the position 4-vector $x^\mu = (ct, \mathbf{r})$ under Lorentz transformations: $V'^\mu = \Lambda^\mu_{\ \nu}V^\nu$. The set of 4-vectors is exactly the right language for special relativity: Lorentz-invariant quantities are built from 4-vector dot products, and the laws of physics are covariant when written in terms of 4-vectors.

The 4-velocity, 4-momentum, and 4-acceleration are the natural relativistic extensions of their 3D counterparts. The 4-momentum $p^\mu = (E/c, \mathbf{p})$ has the beautiful property that its squared norm gives the rest mass: $p_\mu p^\mu = -m^2c^2$. This is the mass-shell condition, which in quantum mechanics becomes the Klein-Gordon equation.

---

## 20.1.1 The 4-Velocity

For a particle with worldline $x^\mu(\tau)$ (parametrized by proper time $\tau$), the **4-velocity** is:

$$u^\mu = \frac{dx^\mu}{d\tau}$$

Since $d\tau = dt\sqrt{1-v^2/c^2} = dt/\gamma$, and $dx^\mu = (c\,dt, d\mathbf{r})$:

$$u^\mu = \frac{dx^\mu}{d\tau} = \gamma\frac{dx^\mu}{dt} = \gamma\left(c, \mathbf{v}\right) = (\gamma c, \gamma\mathbf{v})$$

**Normalization**: The norm of the 4-velocity is:

$$u_\mu u^\mu = \eta_{\mu\nu}u^\mu u^\nu = -\gamma^2 c^2 + \gamma^2|\mathbf{v}|^2 = -\gamma^2 c^2\left(1 - \frac{v^2}{c^2}\right) = -c^2$$

The 4-velocity has constant norm $-c^2$. Geometrically: it is always a unit timelike vector (pointing along the particle's worldline).

**At rest**: $v = 0$, $\gamma = 1$: $u^\mu = (c, 0, 0, 0)$. The 4-velocity points along the time direction.

**Massless particles** (photons): $d\tau = 0$ along null worldlines, so $u^\mu = dx^\mu/d\tau$ is not defined. Instead, parametrize by an affine parameter $\lambda$: $k^\mu = dx^\mu/d\lambda$, with $k_\mu k^\mu = 0$ (null condition). The photon 4-momentum is $p^\mu = \hbar k^\mu = (\hbar\omega/c, \hbar\mathbf{k})$.

---

## 20.1.2 The 4-Momentum and Energy-Momentum Relation

The **4-momentum** of a particle of rest mass $m$ is:

$$p^\mu = mu^\mu = m\gamma(c, \mathbf{v}) = \left(\frac{E}{c}, \mathbf{p}\right)$$

where:
- $\mathbf{p} = m\gamma\mathbf{v}$ is the relativistic 3-momentum
- $E = m\gamma c^2$ is the relativistic energy (rest energy + kinetic energy)

**The mass-shell condition**: Since $u_\mu u^\mu = -c^2$:

$$p_\mu p^\mu = m^2 u_\mu u^\mu = -m^2c^2$$

Expanding: $-(E/c)^2 + |\mathbf{p}|^2 = -m^2c^2$, i.e.:

$$\boxed{E^2 = \mathbf{p}^2c^2 + m^2c^4}$$

This is the **relativistic energy-momentum relation**. It unifies energy and momentum and determines the dispersion relation for relativistic particles.

**Special cases**:
- $\mathbf{p} = 0$ (rest): $E = mc^2$ — rest energy.
- $m = 0$ (photon): $E = |\mathbf{p}|c$ — massless particles.
- $|\mathbf{p}| \gg mc$ (ultra-relativistic): $E \approx |\mathbf{p}|c$ — ultra-relativistic particles behave like massless particles.
- $|\mathbf{p}| \ll mc$ (non-relativistic): $E \approx mc^2 + \mathbf{p}^2/(2m)$ — rest energy plus Newtonian kinetic energy.

**The kinetic energy**: $K = E - mc^2 = (\gamma - 1)mc^2$. For $v \ll c$: $K \approx \frac{1}{2}mv^2$. ✓

---

## 20.1.3 Mass-Energy Equivalence

The formula $E = mc^2$ (for a particle at rest) has profound consequences: mass and energy are the same physical quantity. Examples:

**Nuclear binding energy**: The mass of a helium-4 nucleus is less than the sum of the masses of 2 protons + 2 neutrons by $\Delta m = 0.0304$ atomic mass units. This mass defect corresponds to binding energy $\Delta E = \Delta mc^2 = 28.3$ MeV — the energy released in nuclear fusion (or required to disassemble the nucleus).

**Pair production and annihilation**: A photon with $E_\gamma \geq 2m_e c^2 = 1.022$ MeV can produce an electron-positron pair ($\gamma \to e^+ + e^-$). Conversely, $e^+ + e^- \to 2\gamma$. Mass-energy is conserved.

**QCD and nucleon mass**: 99% of the proton's mass comes from the kinetic and field energy of its constituent quarks and gluons — not from the quark rest masses. The proton is mostly "pure energy."

**Gravitational mass of binding energy**: The gravitational mass of a nucleus is $m_{\rm nucleus} = (M_{\rm protons} + M_{\rm neutrons} - \Delta m)$ where $\Delta m$ is the binding energy deficit. GR requires that all forms of energy (including binding energy) contribute to gravity. Precision Eötvös experiments testing the equivalence principle verify this at the level of $10^{-3}$ — and future experiments aim for $10^{-6}$ [Will (2014)].

---

## 20.1.4 The 4-Force and Relativistic Dynamics

The equation of motion for a relativistic particle:

$$f^\mu = \frac{dp^\mu}{d\tau} = m\frac{du^\mu}{d\tau} = ma^\mu$$

where $a^\mu = du^\mu/d\tau$ is the **4-acceleration**.

**Constraint**: Since $u_\mu u^\mu = -c^2$ (constant): differentiate with respect to $\tau$: $2a_\mu u^\mu = 0$. The 4-acceleration is always orthogonal to the 4-velocity. This means the 4-force satisfies $f_\mu u^\mu = 0$ for a particle of constant rest mass.

**Components**: In the rest frame ($u^\mu = (c, 0, 0, 0)$):
- $f^\mu = (0, \mathbf{F})$ where $\mathbf{F}$ is the ordinary 3-force (in the rest frame, $dp^0/d\tau = 0$ since the force does no work in the rest frame if it's purely spatial — for electromagnetic force on a charged particle at rest, this is satisfied).

**Electromagnetic 4-force**: The Lorentz force in covariant form:

$$f^\mu = qF^{\mu\nu}u_\nu$$

where $F^{\mu\nu}$ is the Faraday tensor and $u_\nu$ is the 4-velocity. In components:
- $f^0 = qF^{0\nu}u_\nu = q(\mathbf{E}/c)\cdot(\gamma\mathbf{v}) = \gamma q\mathbf{E}\cdot\mathbf{v}/c$: power delivered = rate of energy gain.
- $f^i = qF^{i\nu}u_\nu = \gamma q(E^i + \varepsilon^{ijk}v_jB_k) = \gamma q(\mathbf{E} + \mathbf{v}\times\mathbf{B})^i$: the Lorentz force.

**Geodesic equation**: In GR, a freely falling particle experiences no force — it follows a geodesic:

$$\frac{d^2x^\mu}{d\tau^2} + \Gamma^\mu_{\nu\rho}\frac{dx^\nu}{d\tau}\frac{dx^\rho}{d\tau} = 0$$

This is the relativistic generalization of $\mathbf{F} = 0$ in Newton's first law. The Christoffel symbols $\Gamma^\mu_{\nu\rho}$ encode the inertial forces due to curved coordinates — and in GR, also the gravitational force.

---

## 20.1.5 Threshold Phenomena and Invariant Mass

4-momentum conservation is the relativistic generalization of 3-momentum + energy conservation. Since $p^\mu$ is a 4-vector, the quantity $p_\mu p^\mu = -m^2c^2$ is frame-independent — it is the invariant mass squared.

**Compton scattering**: $\gamma + e^- \to \gamma + e^-$ (photon scattering off an electron). The photon wavelength shift:

$$\lambda' - \lambda = \frac{h}{m_e c}(1 - \cos\theta)$$

where $\theta$ is the scattering angle. *Derived from 4-momentum conservation*: $p_\gamma + p_e = p'_\gamma + p'_e$. Square $(p_\gamma - p'_\gamma)^2$; use $p_e^2 = -m_e^2c^2$; use $p_\gamma^2 = 0$. The Compton wavelength $\lambda_C = h/(m_e c) = 2.43\times10^{-12}$ m is the characteristic scale of relativistic quantum effects for the electron.

**Threshold energy**: The minimum energy for a reaction to occur is when all products are created at rest in the center-of-mass frame. For pion production $p + p \to p + p + \pi^0$:

$$\sqrt{s} = (2m_p + m_\pi)c^2 \quad \Rightarrow \quad s = (2m_p + m_\pi)^2c^4$$

In the lab frame (one proton at rest), $s = (p_1 + p_2)^2c^2 = -2m_p^2c^4 + 2E_1 m_p c^2$ (in natural units). Setting $s = (2m_p + m_\pi)^2c^4$: the threshold kinetic energy is $T_{\rm thresh} = 2m_\pi c^2(1 + m_\pi/(4m_p)) \approx 279.6$ MeV. This is exact — derived from 4-momentum conservation alone.

---

## References

- Einstein, A. (1905). "Ist die Trägheit eines Körpers von seinem Energieinhalt abhängig?" *Annalen der Physik*, 18, 639–641. [The paper deriving $E = mc^2$: a body that emits radiation $L$ loses mass $L/c^2$.]
- Compton, A.H. (1923). "A quantum theory of the scattering of X-rays by light elements." *Physical Review*, 21, 483–502. [The Compton effect: X-ray scattering changes wavelength by $h/(m_e c)(1-\cos\theta)$. One of the key experiments establishing the particle nature of light.]
- Dirac, P.A.M. (1928). "The quantum theory of the electron." *Proceedings of the Royal Society A*, 117, 610–624. [Derives the relativistic equation for the electron: the Dirac equation $(\gamma^\mu\partial_\mu + im)\psi = 0$. Predicts antiparticles. The 4-momentum structure of relativistic quantum mechanics.]
- Taylor, E.F. and Wheeler, J.A. (1992). *Spacetime Physics*, 2nd ed. W.H. Freeman. [Chapter 7 on 4-momentum, the energy-momentum relation, and relativistic collision kinematics. Excellent on physical intuition.]
