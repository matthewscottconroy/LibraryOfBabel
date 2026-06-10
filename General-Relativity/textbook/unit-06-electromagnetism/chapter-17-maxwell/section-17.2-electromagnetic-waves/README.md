# Section 17.2: Electromagnetic Waves

---

## Section Introduction

Maxwell's greatest discovery was hidden in plain sight: when he combined his four equations and eliminated either $\mathbf{E}$ or $\mathbf{B}$, he found that each component satisfied the wave equation with wave speed $c = 1/\sqrt{\mu_0\varepsilon_0}$. From the known values of $\mu_0$ and $\varepsilon_0$ (measured in static experiments), he computed $c \approx 3\times10^8$ m/s — and recognized it as the measured speed of light. Light is electromagnetic waves.

This section develops electromagnetic wave theory, the energy and momentum carried by electromagnetic fields, and the radiation from accelerating charges. The stress-energy tensor of the electromagnetic field — the source of gravity in the Einstein equations — appears naturally here.

---

## 17.2.1 The Wave Equation from Maxwell's Equations

In vacuum ($\rho = 0$, $\mathbf{J} = 0$), the Maxwell equations are:

$$\nabla\cdot\mathbf{E} = 0, \quad \nabla\times\mathbf{B} = \mu_0\varepsilon_0\partial_t\mathbf{E}$$
$$\nabla\cdot\mathbf{B} = 0, \quad \nabla\times\mathbf{E} = -\partial_t\mathbf{B}$$

Take the curl of Faraday's law:
$$\nabla\times(\nabla\times\mathbf{E}) = -\partial_t(\nabla\times\mathbf{B}) = -\mu_0\varepsilon_0\partial_{tt}\mathbf{E}$$

Using the vector identity $\nabla\times(\nabla\times\mathbf{E}) = \nabla(\nabla\cdot\mathbf{E}) - \nabla^2\mathbf{E} = -\nabla^2\mathbf{E}$ (since $\nabla\cdot\mathbf{E} = 0$ in vacuum):

$$\nabla^2\mathbf{E} - \mu_0\varepsilon_0\partial_{tt}\mathbf{E} = 0 \quad \Rightarrow \quad \Box\mathbf{E} = 0$$

where $\Box = \nabla^2 - (1/c^2)\partial_t^2$ is the d'Alembert wave operator. Similarly, $\Box\mathbf{B} = 0$. Each component of $\mathbf{E}$ and $\mathbf{B}$ satisfies the wave equation with speed:

$$c = \frac{1}{\sqrt{\mu_0\varepsilon_0}} = 2.998\times10^8 \text{ m/s}$$

Maxwell computed this in 1865 from the measured values of $\mu_0$ and $\varepsilon_0$ and recognized it as the speed of light.

**In covariant notation**: In vacuum, $\partial_\mu F^{\mu\nu} = 0$ with $F = dA$ (choosing Lorenz gauge $\partial_\mu A^\mu = 0$, Section 18.2) gives:

$$\Box A^\nu = 0$$

Each component of the 4-potential satisfies the scalar wave equation.

---

## 17.2.2 Plane Wave Solutions

The simplest solutions are **plane waves**: fields that depend only on $t$ and one spatial coordinate $z = \hat{\mathbf{k}}\cdot\mathbf{r}$ (the propagation direction):

$$\mathbf{E}(\mathbf{r}, t) = \mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r} - \omega t)}, \qquad \mathbf{B}(\mathbf{r}, t) = \mathbf{B}_0 e^{i(\mathbf{k}\cdot\mathbf{r} - \omega t)}$$

(taking the real part for physical fields).

**Dispersion relation**: Substituting into the wave equation: $-k^2 + \omega^2/c^2 = 0$, i.e., $\omega = ck$ (linear dispersion — photons are massless).

**Transversality**: From $\nabla\cdot\mathbf{E} = 0$: $\mathbf{k}\cdot\mathbf{E}_0 = 0$ (electric field perpendicular to wave vector). Similarly $\mathbf{k}\cdot\mathbf{B}_0 = 0$. From Faraday's law: $\mathbf{k}\times\mathbf{E}_0 = \omega\mathbf{B}_0$, so $\mathbf{B}_0 = (\hat{\mathbf{k}}/c)\times\mathbf{E}_0$. The fields are mutually perpendicular: $\mathbf{E} \perp \mathbf{B}$, $\mathbf{E} \perp \hat{\mathbf{k}}$, $\mathbf{B} \perp \hat{\mathbf{k}}$, and $|\mathbf{B}| = |\mathbf{E}|/c$.

Electromagnetic waves are **transverse waves** with two independent polarization states.

**Polarization**: Choose $\hat{\mathbf{k}} = \hat{\mathbf{z}}$. The two independent polarizations are:
- **Linear polarization**: $\mathbf{E} = E_0\hat{\mathbf{x}}\cos(kz - \omega t)$ or $\mathbf{E} = E_0\hat{\mathbf{y}}\cos(kz - \omega t)$
- **Circular polarization**: $\mathbf{E} = E_0(\hat{\mathbf{x}} \pm i\hat{\mathbf{y}})e^{i(kz-\omega t)}$ (right/left circular, with $\pm$ corresponding to helicity $\pm 1$)

The two circular polarizations are eigenstates of the helicity operator; they correspond to photons with spin $\pm\hbar$ (angular momentum $\pm\hbar$ along the propagation direction). The graviton (if it exists) has spin 2 and helicity $\pm 2$ — analogous to the photon's helicity $\pm 1$.

---

## 17.2.3 Energy and the Poynting Vector

The energy density in the electromagnetic field is:

$$u = \frac{1}{2}\left(\varepsilon_0 |\mathbf{E}|^2 + \frac{1}{\mu_0}|\mathbf{B}|^2\right)$$

The energy flux (power per unit area) is the **Poynting vector**:

$$\mathbf{S} = \frac{1}{\mu_0}\mathbf{E}\times\mathbf{B}$$

*Derivation*: Take the dot product of the Ampère-Maxwell law with $\mathbf{E}$ and of Faraday's law with $\mathbf{B}/\mu_0$, and add:

$$\mathbf{E}\cdot(\nabla\times\mathbf{B}/\mu_0) - \mathbf{B}/\mu_0\cdot(\nabla\times\mathbf{E}) = \mathbf{J}\cdot\mathbf{E} + \partial_t\left(\frac{\varepsilon_0|\mathbf{E}|^2}{2} + \frac{|\mathbf{B}|^2}{2\mu_0}\right)$$

The left side is $\nabla\cdot(\mathbf{E}\times\mathbf{B}/\mu_0) = \nabla\cdot\mathbf{S}$ (by the vector identity $\nabla\cdot(\mathbf{A}\times\mathbf{B}) = \mathbf{B}\cdot(\nabla\times\mathbf{A}) - \mathbf{A}\cdot(\nabla\times\mathbf{B})$). Rearranging:

$$\frac{\partial u}{\partial t} + \nabla\cdot\mathbf{S} = -\mathbf{J}\cdot\mathbf{E}$$

This is **Poynting's theorem**: the rate of change of field energy plus the energy flux equals the work done on the charges (with a minus sign — work done on charges reduces field energy).

For a plane wave: $|\mathbf{E}| = c|\mathbf{B}|$, so $u = \varepsilon_0|\mathbf{E}|^2$ (equal electric and magnetic contributions). The Poynting vector points in the propagation direction: $\mathbf{S} = uc\hat{\mathbf{k}}$ — energy travels at speed $c$.

**Radiation pressure**: Electromagnetic waves carry momentum. The momentum density is $\mathbf{g} = \mathbf{S}/c^2 = \varepsilon_0\mathbf{E}\times\mathbf{B}$. A beam absorbed by a surface exerts pressure $P = I/c$ where $I = |\mathbf{S}|$ is the intensity (Watt/m²). Solar radiation pressure ($I_\odot \approx 1361$ W/m² at Earth) can be used for solar sails.

---

## 17.2.4 The Electromagnetic Stress-Energy Tensor

In special relativity (and as a source for GR), the energy density, momentum density, and stress of the electromagnetic field are unified into the **electromagnetic stress-energy tensor**:

$$T^{\mu\nu}_{\rm EM} = \frac{1}{\mu_0}\left(F^{\mu\alpha}F^\nu_{\ \alpha} - \frac{1}{4}\eta^{\mu\nu}F_{\alpha\beta}F^{\alpha\beta}\right)$$

Its components are:

- $T^{00} = u$ (energy density)
- $T^{0i} = T^{i0} = S^i/c$ (energy flux = momentum density $\times c$)
- $T^{ij}$ = Maxwell stress tensor (electromagnetic stress/pressure)

**Tracelessness**: $\eta_{\mu\nu}T^{\mu\nu}_{\rm EM} = \frac{1}{\mu_0}(F^{\mu\alpha}F_{\mu\alpha} - \frac{4}{4}F_{\alpha\beta}F^{\alpha\beta}) = 0$. The electromagnetic stress-energy tensor is traceless — a signature of conformal invariance (masslessness of the photon).

**Conservation**: In vacuum, $\partial_\mu T^{\mu\nu}_{\rm EM} = 0$ — electromagnetic energy and momentum are locally conserved.

**As source of gravity**: In the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$, the electromagnetic stress-energy contributes. An electromagnetic field curves spacetime. The electromagnetic field of a charged black hole (Reissner-Nordström metric) includes the gravitational effect of its electromagnetic energy.

---

## 17.2.5 Radiation from Accelerating Charges

An accelerating charge radiates electromagnetic energy. This is the fundamental mechanism for all radiation: radio waves, X-rays, synchrotron radiation, bremsstrahlung.

**The Larmor formula**: A charge $q$ with acceleration $a$ (non-relativistic) radiates power:

$$P = \frac{q^2 a^2}{6\pi\varepsilon_0 c^3} = \frac{\mu_0 q^2 a^2}{6\pi c}$$

*Derivation sketch*: Far from the charge (in the "radiation zone" $r \gg \lambda$), the fields fall as $1/r$ (radiation fields) rather than $1/r^2$ (Coulomb fields). The radiation fields are:

$$\mathbf{E}_{\rm rad} = \frac{q}{4\pi\varepsilon_0}\frac{1}{c^2 r}\hat{\mathbf{r}}\times(\hat{\mathbf{r}}\times\dot{\mathbf{v}})|_{t_{\rm ret}}$$

where $t_{\rm ret} = t - r/c$ is the retarded time (fields travel at $c$). The Poynting flux integrated over a sphere at large $r$ gives Larmor's formula.

**Relativistic generalization** (Liénard formula):

$$P = \frac{q^2\gamma^6}{6\pi\varepsilon_0 c^3}\left(|\dot{\mathbf{v}}|^2 - \left|\frac{\mathbf{v}\times\dot{\mathbf{v}}}{c}\right|^2\right)$$

where $\gamma = (1 - v^2/c^2)^{-1/2}$ and $\dot{\mathbf{v}}$ is the 3-acceleration.

In covariant form: $P = \frac{q^2}{6\pi\varepsilon_0 m^2 c^3}\frac{dp_\mu}{d\tau}\frac{dp^\mu}{d\tau}$ — the power depends only on the invariant norm of the 4-acceleration.

**Synchrotron radiation**: A relativistic charge moving in a magnetic field radiates at power $P \propto \gamma^2 B^2 v_\perp^2$. In radio astronomy: electrons in magnetic fields emit synchrotron radiation from radio waves to X-rays, depending on electron energy and field strength. The Crab Nebula pulsar's emission is synchrotron radiation.

**Gravitational wave analogy**: The gravitational wave luminosity from a binary system (Section 38) is analogous to electromagnetic radiation from an accelerating charge, with the mass quadrupole moment $\ddot{Q}_{ij}$ playing the role of the electric dipole moment $\ddot{p}$:

$$P_{\rm GW} = \frac{G}{5c^5}\langle\dddot{Q}_{ij}\dddot{Q}^{ij}\rangle$$

(The third time derivative appears because gravity is quadrupole radiation, not dipole radiation — momentum conservation forbids a mass dipole term.)

---

## References

- Maxwell, J.C. (1865). "A dynamical theory of the electromagnetic field." *Philosophical Transactions of the Royal Society*, 155, 459–512. [Contains the prediction of electromagnetic waves and the calculation $c = 1/\sqrt{\mu_0\varepsilon_0}$.]
- Hertz, H. (1888). "Über die Ausbreitung der elektrischen Kraft." *Annalen der Physik*, 36, 1–22. [The experimental confirmation of electromagnetic waves: generation and detection of radio waves in the laboratory, 22 years after Maxwell's prediction.]
- Larmor, J. (1897). "On the theory of the magnetic influence on spectra, and on the radiation from moving ions." *Philosophical Magazine*, 44, 503–512. [The Larmor formula for radiation from accelerating charges.]
- Poynting, J.H. (1884). "On the transfer of energy in the electromagnetic field." *Philosophical Transactions*, 175, 343–361. [Derives the Poynting vector and the energy conservation equation for electromagnetic fields.]
- Jackson, J.D. (1999). *Classical Electrodynamics*, 3rd ed. Wiley. [The standard advanced reference for electrodynamics. Chapters 6–9 on waves; Chapter 14 on radiation from moving charges. Thorough, rigorous, and comprehensive.]
