# Chapter 19: Magnetostatics and Magnetic Fields

---

## Chapter Introduction

Magnetism seems, at first, to be a completely different phenomenon from electricity. Electric fields come from charges; magnetic fields come from moving charges (currents). The force law is different — magnetic force $\mathbf{F} = q\mathbf{v}\times\mathbf{B}$ involves the velocity and is perpendicular to both the velocity and the field. Electric fields can exist around stationary charges; magnetic fields require moving charges.

And yet, as we learned in Chapter 26 of Unit VII, electricity and magnetism are fundamentally the same thing — aspects of a single electromagnetic field. A pure magnetic field in one inertial frame appears as a combination of electric and magnetic fields in another. Magnetism is a relativistic effect of electricity. The Lorentz force $\mathbf{F} = q(\mathbf{E} + \mathbf{v}\times\mathbf{B})$ is a single covariant expression.

This chapter develops the classical theory of magnetic fields produced by steady currents — magnetostatics. The key tools are the Biot-Savart law, Ampère's law, and the vector potential. The mathematics closely parallels electrostatics: Ampère's law is the magnetic counterpart of Gauss's law, and the vector potential satisfies Poisson's equation.

---

## The Magnetic Field and Lorentz Force

The **magnetic force** on a charge $q$ moving with velocity $\mathbf{v}$ in field $\mathbf{B}$:
$$\mathbf{F} = q\mathbf{v}\times\mathbf{B}$$

Combined with the electric force: $\mathbf{F} = q(\mathbf{E} + \mathbf{v}\times\mathbf{B})$ — the **Lorentz force**.

**Properties of the magnetic force**:
- Perpendicular to $\mathbf{v}$: does no work, cannot change kinetic energy
- Perpendicular to $\mathbf{B}$: the motion curves but does not speed up
- Causes circular motion in a uniform $\mathbf{B}$: cyclotron frequency $\omega_c = qB/m$, radius $r_c = mv/(qB)$

**Force on a current-carrying wire**: $d\mathbf{F} = I\,d\mathbf{l}\times\mathbf{B}$ (since current = charge/time × number density × velocity).

---

## The Biot-Savart Law

For a steady current $I$ in a circuit, the magnetic field at $\mathbf{r}$:
$$\mathbf{B}(\mathbf{r}) = \frac{\mu_0}{4\pi}\oint\frac{I\,d\mathbf{l}'\times(\mathbf{r}-\mathbf{r}')}{|\mathbf{r}-\mathbf{r}'|^3}$$

For a volume current density $\mathbf{J}(\mathbf{r}')$:
$$\mathbf{B}(\mathbf{r}) = \frac{\mu_0}{4\pi}\int\frac{\mathbf{J}(\mathbf{r}')\times(\mathbf{r}-\mathbf{r}')}{|\mathbf{r}-\mathbf{r}'|^3}d^3r'$$

where $\mu_0 = 4\pi\times 10^{-7}$ H/m is the permeability of free space, and $c = 1/\sqrt{\varepsilon_0\mu_0}$.

**Examples**:
- **Infinite straight wire** (current $I$): $B = \mu_0 I/(2\pi r)$, circumferential (right-hand rule)
- **Center of circular loop** (radius $R$): $B = \mu_0 I/(2R)$, along the axis
- **Solenoid** (infinite, $n$ turns/m): $B = \mu_0 n I$ inside, zero outside

---

## Gauss's Law for Magnetism and Ampère's Law

**Gauss's law for magnetism**: No magnetic monopoles:
$$\nabla\cdot\mathbf{B} = 0$$

(The second of Maxwell's equations.) The field lines of $\mathbf{B}$ always close — they have no sources or sinks.

**Ampère's law** (static case):
$$\nabla\times\mathbf{B} = \mu_0\mathbf{J}$$

or in integral form: $\oint_C\mathbf{B}\cdot d\mathbf{l} = \mu_0 I_{\rm enc}$.

Applications (by symmetry):
- Infinite wire: $B\cdot 2\pi r = \mu_0 I$ gives $B = \mu_0 I/(2\pi r)$
- Solenoid: $B\cdot L = \mu_0 n L I$ gives $B = \mu_0 n I$ inside, 0 outside
- Toroidal coil: $B = \mu_0 NI/(2\pi r)$ inside the torus, 0 outside

---

## The Vector Potential

Since $\nabla\cdot\mathbf{B} = 0$, by the converse of the divergence theorem, $\mathbf{B} = \nabla\times\mathbf{A}$ for some **vector potential** $\mathbf{A}$.

Substituting into Ampère's law:
$$\nabla\times(\nabla\times\mathbf{A}) = \nabla(\nabla\cdot\mathbf{A}) - \nabla^2\mathbf{A} = \mu_0\mathbf{J}$$

**Coulomb gauge**: Choose $\nabla\cdot\mathbf{A} = 0$ (this choice is always possible by a gauge transformation $\mathbf{A}\to\mathbf{A}+\nabla\chi$). Then:
$$\nabla^2\mathbf{A} = -\mu_0\mathbf{J}$$

This is three copies of Poisson's equation — one for each component. The solution:
$$\mathbf{A}(\mathbf{r}) = \frac{\mu_0}{4\pi}\int\frac{\mathbf{J}(\mathbf{r}')}{|\mathbf{r}-\mathbf{r}'|}d^3r'$$

**Gauge freedom**: $\mathbf{A}\to\mathbf{A}+\nabla\chi$, $\phi\to\phi-\partial_t\chi$ leaves $\mathbf{E}$ and $\mathbf{B}$ unchanged. In the full (time-dependent) theory, this becomes the electromagnetic gauge invariance, which extends to the U(1) gauge invariance of quantum electrodynamics.

---

## Magnetic Multipoles and Dipoles

The magnetic analogue of the electric multipole expansion: there are no magnetic monopoles ($\nabla\cdot\mathbf{B} = 0$), so the leading term is the **magnetic dipole**.

**Magnetic dipole moment**: For a current loop with area $A$ and current $I$:
$$\mathbf{m} = IA\hat{n}$$

Far from a magnetic dipole:
$$\mathbf{B} = \frac{\mu_0}{4\pi r^3}(3(\mathbf{m}\cdot\hat{r})\hat{r} - \mathbf{m}) + \frac{2\mu_0}{3}\mathbf{m}\delta^3(\mathbf{r})$$

**Earth's magnetic field**: Approximately a magnetic dipole with $m\approx 8\times 10^{22}$ A$\cdot$m$^2$. The magnetic axis is tilted $11°$ from the rotation axis.

**Torque on a dipole**: $\boldsymbol{\tau} = \mathbf{m}\times\mathbf{B}$. Potential energy: $U = -\mathbf{m}\cdot\mathbf{B}$.

---

## Magnetic Energy and Inductance

**Self-inductance** $L$: A current $I$ in a circuit creates a magnetic flux $\Phi_B = LI$ through its own circuit. The energy stored:
$$U = \frac{1}{2}LI^2 = \frac{1}{2\mu_0}\int B^2\,d^3r$$

The **magnetic energy density**: $u_B = B^2/(2\mu_0)$.

For an inductor in circuit theory: EMF = $-L\,dI/dt$ (Faraday's law). The inductance of a solenoid: $L = \mu_0 n^2 V$ where $V$ is the volume.

---

## Important Concepts

- **Lorentz force**: $\mathbf{F} = q(\mathbf{E}+\mathbf{v}\times\mathbf{B})$; magnetic force does no work
- **Biot-Savart law**: $\mathbf{B}$ from current distribution; inverse-square, crossproduct structure
- **Gauss's law for $\mathbf{B}$**: $\nabla\cdot\mathbf{B} = 0$; no magnetic monopoles; field lines close
- **Ampère's law** (static): $\nabla\times\mathbf{B} = \mu_0\mathbf{J}$; circuital law
- **Vector potential**: $\mathbf{B} = \nabla\times\mathbf{A}$; exists because $\nabla\cdot\mathbf{B} = 0$; gauge freedom
- **Magnetic dipole**: Leading multipole; $\mathbf{m} = IA\hat{n}$; $U = -\mathbf{m}\cdot\mathbf{B}$
- **Magnetic energy density**: $u_B = B^2/(2\mu_0)$

---

## Further Reading

- Griffiths, D.J. (2017). *Introduction to Electrodynamics*. Cambridge. — Chapters 5–6.
- Jackson, J.D. (1999). *Classical Electrodynamics*. Wiley. — Chapter 5.

---

## Exercises

**19.1.** *Biot-Savart and Ampère.*

(a) Use the Biot-Savart law to find $\mathbf{B}$ on the axis of a circular loop of radius $R$ carrying current $I$, at axial distance $z$ from the center.

(b) Use Ampère's law to find $\mathbf{B}$ inside and outside an infinite solenoid of $n$ turns/m carrying current $I$.

(c) The magnetic field inside a toroidal solenoid with $N$ total turns, inner radius $a$, outer radius $b$: use Ampère's law to find $\mathbf{B}(r)$ for $a<r<b$. Is the field uniform?

---

**19.2.** *Vector potential.*

(a) For a uniform magnetic field $\mathbf{B} = B\hat{z}$: find a vector potential $\mathbf{A}$ such that $\nabla\times\mathbf{A} = \mathbf{B}$. Is your answer unique?

(b) Apply the gauge transformation $\mathbf{A}\to\mathbf{A}+\nabla\chi$ with $\chi = Bxy/2$. Verify the new $\mathbf{A}$ still gives the same $\mathbf{B}$.

(c) In the Coulomb gauge $\nabla\cdot\mathbf{A} = 0$: for the infinite wire with current $I$ along $z$, find $\mathbf{A}$ satisfying $\nabla^2\mathbf{A} = -\mu_0\mathbf{J}$.

---

**19.3.** *Magnetic dipole.*

(a) A small magnetic dipole $\mathbf{m} = m\hat{z}$ is placed at the origin. The field far away: $\mathbf{B} = (\mu_0/4\pi r^3)(2\cos\theta\hat{r} + \sin\theta\hat\theta)m$. Verify $\nabla\cdot\mathbf{B} = 0$ and $\nabla\times\mathbf{B} = 0$ for $r > 0$.

(b) An electron has spin magnetic moment $\mu_B = e\hbar/(2m_e) = 9.27\times 10^{-24}$ J/T. In Earth's magnetic field ($B\approx 5\times 10^{-5}$ T): compute the energy splitting $\Delta E = 2\mu_B B$. Convert to frequency. This is the basis of electron spin resonance (ESR).

---

**Thought Experiment T19.1.** *Why no magnetic monopoles?*

Electric charges (electric monopoles) exist and are observed. Magnetic charges (monopoles) would be particles with $g_M$ such that $\oint\mathbf{B}\cdot d\mathbf{A} = \mu_0 g_M$. The Dirac quantization condition $e g_M = 2\pi\hbar c$ (in SI units) relates the magnetic and electric charges.

The existence of even one magnetic monopole in the universe would imply electric charge quantization — explaining why all electric charges are multiples of $e$. This is compelling.

Grand unified theories (GUTs) predict magnetic monopoles with mass $M \sim 10^{17}$ GeV — far too heavy to produce in any accelerator. Inflation is invoked to dilute the monopole density.

Why does nature apparently have electric charges but not magnetic charges? Is $\nabla\cdot\mathbf{B} = 0$ an exact law or an approximation? What experiment would you design to search for magnetic monopoles?
