# Important Concepts — Chapter 1: Maxwell's Equations and Electromagnetic Waves

This section is a compressed summary of the key ideas developed in Chapter 1. It is intended for review and orientation, not for first encounter. The entries below are not definitions in the formal sense — they are conceptual summaries, with emphasis on the *meaning* of each concept and its *relevance* to photonic computing.

---

## Maxwell's Equations (Differential Form, Free Space)

$$\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0} \qquad \text{(Gauss — Electric)}$$

$$\nabla \cdot \mathbf{B} = 0 \qquad \text{(Gauss — Magnetic)}$$

$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t} \qquad \text{(Faraday)}$$

$$\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0 \varepsilon_0 \frac{\partial \mathbf{E}}{\partial t} \qquad \text{(Ampère-Maxwell)}$$

These four equations are the complete classical description of the electromagnetic field. Every result in classical optics and photonics is a consequence of these equations (plus appropriate boundary conditions and material properties).

**What each equation means physically:**
- Gauss (electric): electric field lines begin and end on charges; the total outward flux through any closed surface equals the enclosed charge per unit ε₀.
- Gauss (magnetic): magnetic field lines form closed loops; there are no magnetic monopoles.
- Faraday: a time-varying magnetic flux induces an EMF around any closed loop; this is the mechanism by which electromagnetic waves propagate.
- Ampère-Maxwell: magnetic field lines circulate around both current and time-varying electric flux; the displacement current term is Maxwell's addition and the reason electromagnetic waves exist.

**Relevance to photonic computing:** These equations govern every aspect of light propagation in photonic devices — waveguides, resonators, modulators, and detectors.

---

## The Displacement Current

Maxwell's addition to Ampère's law: $\mathbf{J}_D = \varepsilon_0 \partial \mathbf{E}/\partial t$.

Without this term, Ampère's law is inconsistent with charge conservation (the continuity equation $\partial \rho/\partial t + \nabla \cdot \mathbf{J} = 0$). The addition creates a current-like source for the magnetic field even in regions with no physical charge motion — including, crucially, in the fields themselves. This is what allows electromagnetic waves to propagate in vacuum: the changing electric field drives a changing magnetic field (via the displacement current) which drives a changing electric field (via Faraday's law), and so on.

---

## The Wave Equation and Speed of Light

Taking the curl of Faraday's law and substituting the Ampère-Maxwell law yields:

$$\nabla^2 \mathbf{E} = \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial t^2}$$

This is a wave equation. The wave speed is $c = 1/\sqrt{\mu_0 \varepsilon_0} \approx 2.998 \times 10^8$ m/s. Maxwell's recognition that this equaled the measured speed of light — to within experimental error — identified light as an electromagnetic wave. This was not an empirical discovery: it was a theoretical prediction, confirmed twenty-three years later by Hertz.

**In a medium:** The wave speed becomes $v = c/n$, where $n = \sqrt{\varepsilon_r \mu_r} \approx \sqrt{\varepsilon_r}$ for non-magnetic materials. In silicon ($n \approx 3.48$ at 1550 nm), light travels at roughly $c/3.48 \approx 8.6 \times 10^7$ m/s and has a wavelength of $1550/3.48 \approx 446$ nm.

---

## Plane Wave Solutions

A plane wave is the simplest solution to the wave equation:

$$\mathbf{E}(\mathbf{r},t) = \mathbf{E}_0 \, e^{i(\mathbf{k} \cdot \mathbf{r} - \omega t)}$$

Key properties derived from Maxwell's equations:
- **Transversality**: $\mathbf{k} \cdot \mathbf{E}_0 = 0$ (the field oscillates perpendicular to the propagation direction)
- **E-B relationship**: $\mathbf{B} = (\mathbf{k}/\omega) \times \mathbf{E}$ (E, B, and k form a right-handed orthogonal set)
- **Dispersion relation**: $|\mathbf{k}| = n\omega/c$ (relating frequency to wavevector)

The transversality of electromagnetic waves is what makes polarization a physical degree of freedom — and polarization is an information-carrying resource in photonic computing.

---

## Phase Velocity and Group Velocity

**Phase velocity**: $v_p = \omega/k = c/n$ — the speed at which a surface of constant phase moves.

**Group velocity**: $v_g = d\omega/dk = c/n_g$ — the speed at which the envelope of a wave packet moves (the speed of energy propagation and information transmission).

**Group index**: $n_g = n - \lambda \, dn/d\lambda = n + \omega \, dn/d\omega$

**Group velocity dispersion (GVD)**: $\beta_2 = d^2k/d\omega^2$ — determines pulse spreading in dispersive media. In silica fiber at 1310 nm, $\beta_2 = 0$ (zero-dispersion wavelength). In silicon waveguides, GVD can be engineered by adjusting waveguide geometry.

**Why this matters for computing**: Dispersion limits pulse bandwidth and hence the data rate that can be transmitted through a photonic channel. GVD engineering is essential for optical communications and for ultrashort pulse lasers used in some photonic computing schemes.

---

## Boundary Conditions

At an interface between two dielectric media (surface charge density σf, surface current density **Kf**):

| Field component | Condition |
|-----------------|-----------|
| Normal **D** | $D_{1n} - D_{2n} = \sigma_f$ |
| Normal **B** | $B_{1n} = B_{2n}$ |
| Tangential **E** | $E_{1t} = E_{2t}$ |
| Tangential **H** | $H_{1t} - H_{2t} = K_f \times \hat{n}$ |

For the Si/SiO₂ interface relevant to silicon photonics (no free surface charges or currents): all four conditions take their simple forms. The critical angle for total internal reflection: $\theta_c = \arcsin(n_2/n_1) \approx 24.5°$ for Si ($n_1 = 3.48$) on SiO₂ ($n_2 = 1.44$). This is why silicon-on-insulator waveguides confine light effectively — any ray hitting the cladding at an angle less than 65.5° from normal (more than 24.5° from the interface) is totally internally reflected.

---

## The Poynting Vector and Energy Flow

$$\mathbf{S} = \mathbf{E} \times \mathbf{H} \quad \text{(W/m}^2\text{)}$$

Poynting's theorem: $\partial u / \partial t + \nabla \cdot \mathbf{S} = -\mathbf{E} \cdot \mathbf{J}$

where $u = \varepsilon_0 E^2/2 + B^2/(2\mu_0)$ is the electromagnetic energy density.

This equation is an energy conservation statement: the rate of change of electromagnetic energy in a volume plus the energy flux out through the surface equals the negative of the work done on charges. The Poynting vector tells you where electromagnetic energy flows.

**Time-averaged intensity** for a plane wave: $I = \langle S \rangle = \frac{1}{2} \varepsilon_0 c E_0^2 = \frac{E_0^2}{2Z_0}$, where $Z_0 = \sqrt{\mu_0/\varepsilon_0} \approx 377 \, \Omega$ is the impedance of free space.

**Relevance**: The Poynting vector determines optical beam intensity, laser power, signal strength in waveguides, and detector responsivity. The factor of 1/2 in the time-averaged intensity is critical — missing it leads to errors of a factor of 2 in power calculations.

---

## Complex Refractive Index and Absorption

$$\tilde{n} = n + i\kappa$$

where $n$ is the phase refractive index and $\kappa$ is the extinction coefficient (imaginary part).

**Beer-Lambert law**: intensity attenuates as $I(z) = I_0 e^{-\alpha z}$, where the absorption coefficient $\alpha = 2\omega\kappa/c = 4\pi\kappa/\lambda$.

**In silicon at 1550 nm**: $\kappa \approx 0$ (transparent), $\alpha \approx 0$ for intrinsic silicon. Doping introduces free carriers that increase $\alpha$ via the Soref-Bennett relation:

$$\Delta\alpha = 8.5 \times 10^{-18} \Delta N_e + 6.0 \times 10^{-18} \Delta N_h \quad \text{(cm}^{-1}\text{)}$$

This is why silicon modulator waveguides must be short: the same carrier injection that changes the phase also introduces absorption.

---

## Kramers-Kronig Relations

The real and imaginary parts of the complex susceptibility (or complex refractive index) are not independent:

$$n(\omega) - 1 = \frac{2}{\pi} \mathcal{P} \int_0^\infty \frac{\omega' \kappa(\omega')}{\omega'^2 - \omega^2} d\omega'$$

$$\kappa(\omega) = -\frac{2\omega}{\pi} \mathcal{P} \int_0^\infty \frac{n(\omega') - 1}{\omega'^2 - \omega^2} d\omega'$$

These relations follow from causality alone: the response of any physical medium cannot precede its cause. Their importance is that they constrain the optical properties of materials: you cannot have absorption at one frequency without altering the refractive index at all other frequencies, and you cannot have an arbitrary refractive index spectrum without the corresponding absorption spectrum.

**Application**: The Soref-Bennett equations for silicon are a specific instance of the Kramers-Kronig relations: free-carrier injection simultaneously changes both $n$ and $\kappa$. This tradeoff — a refractive index change accompanied by absorption — is a fundamental limit on silicon modulator performance.

---

## Radiation Pressure and Optical Forces

Electromagnetic fields carry momentum. For a plane wave, the momentum density is $\mathbf{g} = \mathbf{S}/c^2$. The radiation pressure on an absorbing surface is $P_{rad} = I/c$; on a perfectly reflecting surface, $P_{rad} = 2I/c$.

Radiation pressure is small — a 1 mW beam exerts $\sim 3 \times 10^{-12}$ N — but it is detectable. More importantly for photonics, *gradient forces* (optical trapping, optical tweezers) allow light to manipulate micron-scale particles. In photonic circuits, radiation pressure couples optical fields to mechanical modes of the structure (cavity optomechanics), which is both a noise source and an engineering resource.

---

## Angular Momentum of Light

- **Spin angular momentum (SAM)**: $\pm \hbar$ per photon for left/right circular polarization. Responsible for the torque on birefringent materials; basis of optical isolators and wave plates.
- **Orbital angular momentum (OAM)**: $\ell \hbar$ per photon for beams with helical phase fronts ($\ell$ = topological charge, any integer). Laguerre-Gaussian modes carry OAM. An unbounded discrete set of orthogonal states, potentially useful for high-dimensional optical information encoding.

Both SAM and OAM are real, measurable properties of the electromagnetic field. They are not restricted to quantum optics — they appear in classical Maxwell theory.

---

## Key Numbers for Photonic Computing Reference

| Quantity | Value | Context |
|----------|-------|---------|
| Speed of light in vacuum | $c = 2.998 \times 10^8$ m/s | |
| Optical frequency at 1550 nm | 193 THz | Telecom C-band center |
| Photon energy at 1550 nm | 0.80 eV = $1.28 \times 10^{-19}$ J | Below Si bandgap (1.12 eV) |
| Si refractive index at 1550 nm | 3.48 | SOI waveguide core |
| SiO₂ refractive index at 1550 nm | 1.44 | SOI waveguide cladding |
| Si₃N₄ refractive index at 1550 nm | 2.00 | Low-loss waveguide platform |
| LiNbO₃ refractive index at 1550 nm | 2.21 (extraordinary) | Modulator platform |
| Silica fiber loss at 1550 nm | 0.2 dB/km | Fundamental Rayleigh limit |
| Silicon waveguide loss | 2–3 dB/cm | State-of-art Si strip waveguide |
| Si₃N₄ waveguide loss | $\sim$0.1 dB/m | Record low-loss platform |
| C-band bandwidth | 4.4 THz | 1530–1565 nm |
| Critical angle Si/SiO₂ | 24.5° | TIR in SOI waveguides |
| Free-space impedance $Z_0$ | 377 Ω | $\sqrt{\mu_0/\varepsilon_0}$ |

---

*These are the essential concepts from Chapter 1. Each concept is developed fully in the chapter text, with derivations, physical interpretation, and worked examples. The numbers in the reference table appear repeatedly throughout this book — they are worth committing to memory.*
