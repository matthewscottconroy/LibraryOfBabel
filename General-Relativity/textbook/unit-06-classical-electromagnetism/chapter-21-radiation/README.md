# Chapter 21: Electromagnetic Radiation

---

## Chapter Introduction

A static charge produces a static electric field. A uniformly moving charge (constant velocity) produces both electric and magnetic fields, but these remain "attached" to the charge — they fall off as $1/r^2$ and carry no net energy away. But an **accelerating charge** does something qualitatively different: it generates electromagnetic radiation — propagating waves that carry energy irreversibly away from the source at the speed of light.

This is a fundamental asymmetry of electrodynamics: you cannot shake a charge without broadcasting energy into the universe. The energy, once radiated, is gone — it does not return to the charge even if the charge is later brought to rest. Every oscillating electron in your radio antenna is broadcasting. Every proton in a synchrotron is radiating. Every charged particle in the early universe that was being heated and accelerated was radiating. The cosmic microwave background is, in a sense, the residue of all that radiation.

The mathematical description of radiation comes from the **Liénard-Wiechert potentials** — the exact solution to Maxwell's equations for a point charge in arbitrary motion. From these, we extract the radiation fields, the Larmor formula for radiated power, and the characteristic angular distribution and polarization of radiation.

---

## Retarded Potentials

For a source described by $\rho(\mathbf{r}',t')$ and $\mathbf{J}(\mathbf{r}',t')$, the **retarded (causal) solutions** to the wave equations in Lorenz gauge:

$$\phi(\mathbf{r},t) = \frac{1}{4\pi\varepsilon_0}\int\frac{\rho(\mathbf{r}', t_{\rm ret})}{|\mathbf{r}-\mathbf{r}'|}d^3r'$$

$$\mathbf{A}(\mathbf{r},t) = \frac{\mu_0}{4\pi}\int\frac{\mathbf{J}(\mathbf{r}', t_{\rm ret})}{|\mathbf{r}-\mathbf{r}'|}d^3r'$$

where $t_{\rm ret} = t - |\mathbf{r}-\mathbf{r}'|/c$ is the **retarded time** — the time at which the source must have emitted in order for the signal to arrive at $\mathbf{r}$ at time $t$.

The causal structure is explicit: the field now depends on what the source was doing in the past, not the present. Influence travels at the speed of light.

---

## Liénard-Wiechert Potentials

For a point charge $q$ at position $\mathbf{r}_s(t)$ moving with velocity $\mathbf{v}(t) = \dot{\mathbf{r}}_s$:

$$\phi(\mathbf{r},t) = \frac{q}{4\pi\varepsilon_0}\frac{c}{\mathcal{R}c - \mathbf{R}\cdot\mathbf{v}}\Bigg|_{\rm ret}$$

$$\mathbf{A}(\mathbf{r},t) = \frac{\mathbf{v}}{c^2}\phi$$

where $\mathbf{R} = \mathbf{r} - \mathbf{r}_s(t_{\rm ret})$ is the displacement from the retarded source position to the field point, $\mathcal{R} = |\mathbf{R}|$, all evaluated at the retarded time $t_{\rm ret}$.

The electric and magnetic fields from Liénard-Wiechert split into two parts:

$$\mathbf{E} = \mathbf{E}_{\rm vel} + \mathbf{E}_{\rm acc}$$

**Velocity (Coulomb) field**: $\mathbf{E}_{\rm vel} \propto 1/\mathcal{R}^2$ — falls off as inverse square, stays "attached" to the charge, does not radiate. For a charge at rest: reduces to Coulomb's law.

**Acceleration (radiation) field**: 
$$\mathbf{E}_{\rm acc} = \frac{q}{4\pi\varepsilon_0}\frac{\hat{\mathbf{R}}\times[(\hat{\mathbf{R}}-\boldsymbol{\beta})\times\dot{\boldsymbol{\beta}}]}{(1-\hat{\mathbf{R}}\cdot\boldsymbol{\beta})^3\mathcal{R}c^2}\Bigg|_{\rm ret}$$

where $\boldsymbol{\beta} = \mathbf{v}/c$. This falls off as $\mathbf{E}_{\rm acc}\propto 1/\mathcal{R}$ — it propagates to infinity carrying energy.

The radiation field $\mathbf{B}_{\rm acc} = \hat{\mathbf{R}}\times\mathbf{E}_{\rm acc}/c$ — both fields are perpendicular to $\hat{\mathbf{R}}$, perpendicular to each other, and equal in magnitude (in appropriate units). This is a transverse electromagnetic wave.

---

## The Larmor Formula

For a non-relativistic charge ($v\ll c$) with acceleration $\dot{\mathbf{v}}$, the total radiated power:

$$P = \frac{q^2\dot{v}^2}{6\pi\varepsilon_0 c^3} = \frac{q^2a^2}{6\pi\varepsilon_0 c^3} = \frac{\mu_0 q^2 a^2}{6\pi c}$$

This is the **Larmor formula**. It has several important features:
- Power $\propto q^2$: heavier particles (same $q$) radiate less for the same acceleration
- Power $\propto a^2$: quadratic in acceleration — radiation is a second-order effect
- Power $\propto c^{-3}$: signals the electromagnetic coupling to the speed of light

**Angular distribution**: $dP/d\Omega = (q^2a^2\sin^2\theta)/(16\pi^2\varepsilon_0 c^3)$ — radiation is maximum perpendicular to the acceleration ($\theta = \pi/2$), zero along the acceleration axis ($\theta = 0$). The radiation pattern is a **donut**.

---

## Relativistic Generalization: Liénard Formula

The relativistic generalization of the Larmor formula:

$$P = \frac{q^2\gamma^6}{6\pi\varepsilon_0 m^2 c^3}\left(a_\parallel^2/c^2 - (a_\perp/c)^2/\gamma^2\right)^{-1} = \frac{\mu_0 q^2 c}{6\pi}\gamma^6\left(|\dot{\boldsymbol{\beta}}|^2 - |\boldsymbol{\beta}\times\dot{\boldsymbol{\beta}}|^2\right)$$

Or, more elegantly using 4-vectors:
$$P = -\frac{q^2}{6\pi\varepsilon_0 m^2 c^3}\frac{dp_\mu}{d\tau}\frac{dp^\mu}{d\tau}$$

This is Lorentz-invariant, as it must be.

**Bremsstrahlung** (braking radiation): A charge decelerating in matter emits radiation. The energy radiated per unit frequency (in quantum theory) is roughly flat — a broad spectrum. This is the mechanism behind X-rays from an X-ray tube.

**Synchrotron radiation**: A relativistic charge moving in a circle (perpendicular acceleration only). Larmor: $P = q^2a^2/(6\pi\varepsilon_0 c^3)$, with $a = v^2/r = \gamma^2 c^2/(r)$ (relativistic). Energy loss per revolution significant for electron synchrotrons; this is the key limitation on circular collider energies.

---

## Dipole Radiation

For a distribution of charges undergoing small oscillations, the radiation fields in the far zone ($r\gg\lambda$) are dominated by the changing electric dipole moment $\mathbf{p}(t) = \sum_i q_i\mathbf{r}_i$.

**Radiation fields** (far zone, $r\to\infty$, non-relativistic):
$$\mathbf{E}_{\rm rad} = \frac{\mu_0}{4\pi c r}\hat{r}\times(\hat{r}\times\ddot{\mathbf{p}})$$

Power radiated by an oscillating dipole:
$$P = \frac{\mu_0}{6\pi c}|\ddot{\mathbf{p}}|^2 = \frac{q^2a^2}{6\pi\varepsilon_0 c^3}$$

(same as Larmor — consistent). For a dipole oscillating at frequency $\omega$: $\mathbf{p} = p_0\hat{z}\cos(\omega t)$, $\ddot{\mathbf{p}} = -\omega^2 p_0\hat{z}\cos(\omega t)$:

$$P = \frac{\mu_0\omega^4 p_0^2}{12\pi c}$$

Power scales as $\omega^4$ — higher frequencies radiate much more strongly. This is why:
- Blue light scatters more than red (Rayleigh scattering $\propto\omega^4$) → sky is blue
- Radio antennas need large currents to radiate efficiently at low $\omega$

**Magnetic dipole and electric quadrupole radiation**: The next terms in the multipole expansion. For a magnetic dipole $\mathbf{m}$: $P_{\rm md} = \mu_0|\ddot{\mathbf{m}}|^2/(6\pi c^3)$ — same form as electric dipole radiation. Electric quadrupole: $P_{\rm eq}\propto |\dddot{Q}_{ij}|^2$. This is the prototype for gravitational wave radiation (Chapter 44), where only quadrupole radiation occurs because there are no gravitational dipoles (conservation of momentum).

---

## Radiation Reaction

An accelerating charge radiates energy. But where does this energy come from? The charge must experience a **radiation reaction force** that opposes the motion producing the radiation.

**Abraham-Lorentz force** (non-relativistic):
$$\mathbf{F}_{\rm rad} = \frac{\mu_0 q^2}{6\pi c}\dot{\mathbf{a}} = \frac{q^2}{6\pi\varepsilon_0 c^3}\dot{\mathbf{a}}$$

(proportional to the time derivative of acceleration, or the "jerk"). This is the reaction force that ensures energy conservation: the work done against $\mathbf{F}_{\rm rad}$ equals the radiated power.

**Problems with the Abraham-Lorentz force**:
- **Runaway solutions**: $m\mathbf{a} = \mathbf{F}_{\rm ext} + (q^2/6\pi\varepsilon_0 c^3)\dot{\mathbf{a}}$ has solutions that accelerate exponentially even with $\mathbf{F}_{\rm ext} = 0$
- **Pre-acceleration**: The correct causal solution requires the particle to start accelerating before the force is applied (violation of causality at timescales $t \sim q^2/(m c^3) \sim r_e/c$ — the light crossing time of the classical electron radius)

These pathologies are cured in QED, where the electron's self-energy is renormalized. The Abraham-Lorentz force is the classical limit of the radiative corrections to electron propagation.

---

## Important Concepts

- **Retarded time**: $t_{\rm ret} = t - |\mathbf{r}-\mathbf{r}'|/c$; causal structure of EM fields
- **Liénard-Wiechert potentials**: Exact potentials for a moving point charge
- **Velocity field**: $\sim 1/r^2$; stays with the charge; no energy transport to infinity
- **Radiation (acceleration) field**: $\sim 1/r$; propagates to infinity; carries energy
- **Larmor formula**: $P = q^2a^2/(6\pi\varepsilon_0 c^3)$; radiated power from accelerating charge
- **Angular distribution**: $\propto\sin^2\theta$ (dipole pattern); donut shape; max perpendicular to acceleration
- **Dipole radiation**: $P\propto\omega^4|\ddot{\mathbf{p}}|^2$; dominant at low velocities; Rayleigh scattering, antennas
- **Quadrupole radiation**: Next order; $P\propto|\dddot{Q}_{ij}|^2$; analogue of gravitational wave radiation
- **Abraham-Lorentz force**: Radiation reaction; runaway solutions; cured by QED renormalization

---

## Further Reading

- Griffiths, D.J. (2017). *Introduction to Electrodynamics*. Cambridge. — Chapters 10–11.
- Jackson, J.D. (1999). *Classical Electrodynamics*. Wiley. — Chapters 9–14.
- Landau, L.D. & Lifshitz, E.M. (1975). *Classical Theory of Fields*. Pergamon. — Chapters 8–9.

---

## Exercises

**21.1.** *Larmor radiation.*

(a) An electron in a hydrogen atom (classical Bohr orbit, radius $a_0 = 0.529$ Å) has centripetal acceleration $a = v^2/a_0$ where $v = e^2/(4\pi\varepsilon_0\hbar m_e a_0)^{1/2}$. Compute $a$ and the radiated power $P$.

(b) The total energy of the electron is $E = -e^2/(8\pi\varepsilon_0 a_0) = -13.6$ eV. Using $dE/dt = -P$, estimate the time for the electron to spiral into the nucleus. Compare to the observed stability of hydrogen.

(c) This classical instability of hydrogen was a major crisis for pre-quantum physics. What assumption of classical EM fails in quantum mechanics? (This was Bohr's motivation for his quantum model.)

---

**21.2.** *Dipole radiation pattern.*

(a) An antenna consists of a short oscillating dipole of length $\ell$ carrying current $I = I_0\cos(\omega t)$. The effective dipole moment: $\ddot{p} = -I_0\omega\ell\sin(\omega t)$. Write the time-averaged radiated power and the radiation resistance $R_{\rm rad} = 2P/\langle I^2\rangle$.

(b) For $\ell = \lambda/2$ (half-wave dipole): $R_{\rm rad}\approx 73\,\Omega$. For $\ell = \lambda/100$ (short dipole): compute $R_{\rm rad}$. Why is a short antenna inefficient?

(c) Why does the sky appear blue (Rayleigh scattering $\propto\omega^4$) and sunsets red ($\omega^4$ scattering depletes blue from the long path)?

---

**21.3.** *Gravitational analogy.*

The gravitational wave power formula is $P_{\rm GW} = G|\dddot{Q}_{ij}|^2/(5c^5)$ (Chapter 44). For EM, the electric quadrupole power is $P_{\rm eq} = |\dddot{Q}_{ij}|^2/(c^5\cdot 360\pi\varepsilon_0)$.

(a) Write the ratio $P_{\rm GW}/P_{\rm EM}$ for equal quadrupole moments. What suppresses gravitational radiation relative to EM?

(b) Why is there no gravitational dipole radiation? (Hint: what conservation law forbids it?)

(c) The EM analogue of the Hulse-Taylor pulsar: two electrons in a circular orbit of radius $r$. Compute the orbital decay rate $dr/dt$ from the radiated power. At what separation does the orbital period equal 1 second?

---

**Thought Experiment T21.1.** *Does a free-falling charge radiate?*

A stationary charge in a gravitational field: is it accelerating? Yes — by Newton's second law, $F = ma = mg$, so $a = g$. The charge should radiate by the Larmor formula. But by the equivalence principle, a free-falling observer sees the charge as stationary (zero acceleration) and non-radiating.

Which description is correct? Does the charge radiate or not?

The resolution involves the concept of "radiation" being frame-dependent in the same way that particle number is (Unruh effect). A stationary charge in a gravitational field is equivalent, by the equivalence principle, to an accelerating charge in flat spacetime. But a free-falling observer is in an inertial frame — they do not see radiation. This apparent paradox is the classical electromagnetic version of the Unruh effect, and it remains subtly controversial. The key is that "radiation" and "absorption" are relative concepts when horizons are involved.
