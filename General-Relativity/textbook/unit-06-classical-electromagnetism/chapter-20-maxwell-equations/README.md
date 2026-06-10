# Chapter 20: Maxwell's Equations

---

## Chapter Introduction

In 1865, James Clerk Maxwell published "A Dynamical Theory of the Electromagnetic Field" — a paper that unified electricity, magnetism, and optics into a single theory, predicted the existence of electromagnetic waves traveling at the speed of light, and set the stage for special relativity. It is one of the greatest theoretical achievements in the history of physics.

Maxwell's crucial contribution was not any individual equation — Gauss's law, Faraday's law, and Ampère's law were already known. His contribution was the **displacement current**: an extra term $\varepsilon_0\partial\mathbf{E}/\partial t$ in Ampère's law that makes the equations consistent with charge conservation and, crucially, allows for self-sustaining electromagnetic waves.

Without the displacement current, Ampère's law is inconsistent for time-varying fields. With it, the four Maxwell equations form a complete, consistent, and Lorentz-covariant description of electromagnetism. Together, they are to electromagnetism what Newton's laws are to mechanics — but unlike Newton's laws, they required no correction by relativity. They were already relativistic.

---

## The Four Maxwell Equations

In SI units, for fields in vacuum:

$$\nabla\cdot\mathbf{E} = \frac{\rho}{\varepsilon_0} \qquad \text{(Gauss's Law)}$$

$$\nabla\cdot\mathbf{B} = 0 \qquad \text{(No Magnetic Monopoles)}$$

$$\nabla\times\mathbf{E} = -\frac{\partial\mathbf{B}}{\partial t} \qquad \text{(Faraday's Law)}$$

$$\nabla\times\mathbf{B} = \mu_0\mathbf{J} + \mu_0\varepsilon_0\frac{\partial\mathbf{E}}{\partial t} \qquad \text{(Ampère-Maxwell Law)}$$

The four equations govern all classical electromagnetic phenomena: from the force between two charges to light propagation to the operation of transformers and antennas.

---

## The Displacement Current

The crucial new term is $\mu_0\varepsilon_0\partial\mathbf{E}/\partial t$ in Ampère's law. Maxwell called it the "displacement current."

**Motivation**: The static Ampère's law $\nabla\times\mathbf{B} = \mu_0\mathbf{J}$ is inconsistent for time-varying fields. Taking the divergence of both sides: $0 = \mu_0\nabla\cdot\mathbf{J}$. But charge conservation requires $\nabla\cdot\mathbf{J} + \partial_t\rho = 0$ — so $\nabla\cdot\mathbf{J} = -\partial_t\rho\neq 0$ in general.

The fix: add $\mu_0\varepsilon_0\partial\mathbf{E}/\partial t$ to Ampère's law. Then $\nabla\times\mathbf{B} = \mu_0(\mathbf{J} + \varepsilon_0\partial_t\mathbf{E})$, and taking the divergence gives $0 = \mu_0(\nabla\cdot\mathbf{J} + \varepsilon_0\partial_t(\nabla\cdot\mathbf{E})) = \mu_0(\nabla\cdot\mathbf{J} + \partial_t\rho) = 0$ — consistent with charge conservation.

**Charge conservation** (continuity equation):
$$\frac{\partial\rho}{\partial t} + \nabla\cdot\mathbf{J} = 0$$

This is automatically satisfied by Maxwell's equations: it is a consequence, not an independent postulate.

---

## Electromagnetic Waves in Vacuum

From Maxwell's equations in vacuum ($\rho = 0$, $\mathbf{J} = 0$):

Take $\nabla\times(\nabla\times\mathbf{E}) = \nabla(\nabla\cdot\mathbf{E}) - \nabla^2\mathbf{E} = -\nabla^2\mathbf{E}$

and $\nabla\times(\partial_t\mathbf{B}) = -\partial_t(\nabla\times\mathbf{B}) = -\mu_0\varepsilon_0\partial_t^2\mathbf{E}$.

From Faraday's law: $\nabla\times(\nabla\times\mathbf{E}) = -\partial_t(\nabla\times\mathbf{B})$. Combining:

$$\nabla^2\mathbf{E} = \mu_0\varepsilon_0\frac{\partial^2\mathbf{E}}{\partial t^2} = \frac{1}{c^2}\frac{\partial^2\mathbf{E}}{\partial t^2}$$

Similarly for $\mathbf{B}$. This is the **wave equation** for electromagnetic waves, with wave speed:
$$c = \frac{1}{\sqrt{\mu_0\varepsilon_0}} = 2.998\times 10^8\ \text{m/s}$$

Maxwell recognized this as the speed of light — thereby identifying light as an electromagnetic wave.

**Plane wave solutions**: $\mathbf{E} = \mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r} - \omega t)}$ with $\omega = ck$. The constraints from Maxwell's equations:
- $\mathbf{k}\cdot\mathbf{E}_0 = 0$: $\mathbf{E}$ is transverse to propagation
- $\mathbf{k}\cdot\mathbf{B}_0 = 0$: $\mathbf{B}$ is transverse
- $\mathbf{B} = (\hat{k}/c)\times\mathbf{E}$: $\mathbf{E}$, $\mathbf{B}$, and $\hat{k}$ are mutually perpendicular

Light is a transverse wave with two polarization states.

---

## Electromagnetic Energy and the Poynting Vector

The energy density of the electromagnetic field:
$$u = \frac{1}{2}\left(\varepsilon_0 E^2 + \frac{B^2}{\mu_0}\right)$$

The **Poynting vector** (energy flux):
$$\mathbf{S} = \frac{1}{\mu_0}\mathbf{E}\times\mathbf{B}$$

**Poynting's theorem** (energy conservation):
$$\frac{\partial u}{\partial t} + \nabla\cdot\mathbf{S} = -\mathbf{J}\cdot\mathbf{E}$$

The right-hand side is the work done on the currents (Ohmic heating for resistors, or energy input for generators). This is the electromagnetic analogue of the mechanical energy conservation equation.

**Radiation pressure**: Electromagnetic waves carry momentum with momentum density $\mathbf{g} = \mathbf{S}/c^2$. The radiation pressure on a perfectly absorbing surface: $P_{\rm rad} = u$ (energy density = pressure).

---

## Potentials and Gauge Invariance

The full time-dependent fields are expressed in terms of potentials $\phi(\mathbf{r},t)$ and $\mathbf{A}(\mathbf{r},t)$:
$$\mathbf{E} = -\nabla\phi - \frac{\partial\mathbf{A}}{\partial t}, \quad \mathbf{B} = \nabla\times\mathbf{A}$$

These automatically satisfy the two homogeneous Maxwell equations ($\nabla\cdot\mathbf{B} = 0$ and Faraday's law).

**Gauge invariance**: The transformation $\phi\to\phi - \partial_t\chi$, $\mathbf{A}\to\mathbf{A}+\nabla\chi$ leaves $\mathbf{E}$ and $\mathbf{B}$ unchanged.

**Lorenz gauge** ($\nabla\cdot\mathbf{A} + \partial_t\phi/c^2 = 0$): The Maxwell equations reduce to:
$$\Box\phi = -\frac{\rho}{\varepsilon_0}, \quad \Box\mathbf{A} = -\mu_0\mathbf{J}$$

where $\Box = \nabla^2 - \partial_t^2/c^2$ is the d'Alembertian. These are four decoupled wave equations with sources.

---

## Maxwell's Equations in Matter

Inside materials, introduce:
- **Electric displacement**: $\mathbf{D} = \varepsilon_0\mathbf{E} + \mathbf{P}$ (where $\mathbf{P}$ is the polarization)
- **Magnetic field** $\mathbf{H}$: $\mathbf{B} = \mu_0(\mathbf{H} + \mathbf{M})$ (where $\mathbf{M}$ is the magnetization)

Maxwell's equations in matter:
$$\nabla\cdot\mathbf{D} = \rho_f, \quad \nabla\times\mathbf{H} = \mathbf{J}_f + \frac{\partial\mathbf{D}}{\partial t}, \quad \nabla\cdot\mathbf{B} = 0, \quad \nabla\times\mathbf{E} = -\frac{\partial\mathbf{B}}{\partial t}$$

where $\rho_f$ and $\mathbf{J}_f$ are the free (non-polarization, non-magnetization) charge and current densities.

For linear media: $\mathbf{D} = \varepsilon\mathbf{E}$ and $\mathbf{B} = \mu\mathbf{H}$, with speed of light in the medium $v = 1/\sqrt{\varepsilon\mu} = c/n$ where $n = \sqrt{\varepsilon_r\mu_r}$ is the refractive index.

---

## Important Concepts

- **Four Maxwell equations**: Gauss (electric), Gauss (magnetic), Faraday, Ampère-Maxwell
- **Displacement current**: $\varepsilon_0\partial\mathbf{E}/\partial t$; added by Maxwell for consistency with charge conservation
- **Wave equation**: $\nabla^2\mathbf{E} = \partial_t^2\mathbf{E}/c^2$; speed $c = 1/\sqrt{\varepsilon_0\mu_0}$
- **Charge conservation**: $\partial_t\rho + \nabla\cdot\mathbf{J} = 0$; consequence of Maxwell's equations
- **Transverse waves**: $\mathbf{E}\perp\mathbf{k}$, $\mathbf{B}\perp\mathbf{k}$; two polarization states
- **Poynting vector**: $\mathbf{S} = \mathbf{E}\times\mathbf{B}/\mu_0$; energy flux
- **Gauge invariance**: $(\phi,\mathbf{A})\to(\phi-\partial_t\chi, \mathbf{A}+\nabla\chi)$; observables unchanged
- **Lorenz gauge**: Decouples wave equations for $\phi$ and $\mathbf{A}$

---

## Further Reading

**Primary Source**
- Maxwell, J.C. (1865). "A Dynamical Theory of the Electromagnetic Field." *Phil. Trans. Royal Society*, 155, 459.

**Textbooks**
- Griffiths, D.J. (2017). *Introduction to Electrodynamics* (4th ed.). Cambridge. — Chapters 7–9.
- Jackson, J.D. (1999). *Classical Electrodynamics* (3rd ed.). Wiley. — Chapters 6–7.

---

## Exercises

**20.1.** *Consistency and waves.*

(a) Show that charge conservation $\partial_t\rho + \nabla\cdot\mathbf{J} = 0$ follows directly from the full (including displacement current) Ampère-Maxwell law and Gauss's law.

(b) Derive the wave equation for $\mathbf{B}$ (not just $\mathbf{E}$) in vacuum.

(c) For a plane wave $\mathbf{E} = E_0\hat{x}e^{i(kz-\omega t)}$: find $\mathbf{B}$ using Faraday's law. Verify $|\mathbf{E}|/|\mathbf{B}| = c$.

---

**20.2.** *Energy and momentum.*

(a) A laser pointer outputs $P = 5$ mW in a beam of radius $r = 1$ mm. Compute the Poynting vector magnitude $|\mathbf{S}|$, the electric field amplitude $E_0$, and the radiation pressure $P_{\rm rad}$ on a perfect absorber.

(b) Verify Poynting's theorem for a resistor: current $I$ flows through resistance $R$. The electric field inside is $E = V/L$ (axial), the magnetic field just outside is $B = \mu_0 I/(2\pi r)$. Show $\mathbf{S}$ points inward through the resistor surface, and $\int\mathbf{S}\cdot d\mathbf{A} = -I^2 R$ (power dissipated).

---

**20.3.** *Gauge transformations.*

(a) Show that $\mathbf{E} = -\nabla\phi - \partial_t\mathbf{A}$ is unchanged under $\phi\to\phi-\partial_t\chi$, $\mathbf{A}\to\mathbf{A}+\nabla\chi$.

(b) Starting from Coulomb gauge ($\nabla\cdot\mathbf{A} = 0$), find the gauge function $\chi$ needed to transform to Lorenz gauge ($\nabla\cdot\mathbf{A} + \partial_t\phi/c^2 = 0$). What equation must $\chi$ satisfy?

---

**Thought Experiment T20.1.** *Light as an EM wave.*

Maxwell predicted that $c = 1/\sqrt{\varepsilon_0\mu_0}$ by calculating from constants measured in purely electrical experiments ($\varepsilon_0$) and purely magnetic experiments ($\mu_0$). He got a number equal to the known speed of light.

This was not a prediction made from a general principle — it was the discovery that two previously separate branches of physics were describing the same phenomenon. The speed of light was not *explained* by Maxwell; it was *identified* as a consequence of electromagnetic constants.

Why does this coincidence tell us something profound? What would it mean if $c_{\rm light}$ and $c_{\rm EM}$ were different? What experiment could detect such a difference, and what would it imply for the structure of physics?
