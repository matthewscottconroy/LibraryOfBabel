# Chapter 1: Exercises

The exercises below are organized into three categories:
- **Mathematical**: derivation and calculation, building technical mastery
- **Conceptual**: understanding and reasoning about physical meaning
- **Lab/Experimental**: hands-on investigation through simulation or physical experiment

Answers to starred (*) problems are provided in Appendix F.

---

## Mathematical Exercises

**1.1*** Verify Coulomb's law from Gauss's law.
Using Gauss's law in integral form, derive the electric field of a point charge $q$ at the origin. Show that your answer is consistent with Coulomb's force law.

**1.2** The field inside a uniform sphere of charge.
A sphere of radius $R$ carries a total charge $Q$ uniformly distributed throughout its volume. Using Gauss's law, find the electric field both inside ($r < R$) and outside ($r > R$) the sphere. Show that the field is continuous at $r = R$ (as it must be, since there is no surface charge). [Hint: the enclosed charge when $r < R$ is $Q(r/R)^3$.]

**1.3*** The energy stored in a parallel-plate capacitor.
A parallel-plate capacitor has plate area $A$, separation $d$, and is charged to voltage $V$. (a) Find the electric field $E$ between the plates using Gauss's law. (b) Find the energy density $u = \varepsilon_0 E^2/2$. (c) Show that the total energy stored in the field equals $\frac{1}{2}CV^2$ where $C = \varepsilon_0 A/d$.

**1.4** Deriving the magnetic field of an infinite wire.
A long straight wire carries current $I$. Using Ampère's law (the incomplete version without displacement current), find the magnetic field at distance $r$ from the wire. What is the direction of $\mathbf{B}$? What is its magnitude?

**1.5** Deriving the wave equation for $\mathbf{B}$.
Starting from Maxwell's equations in free space, derive the wave equation $\nabla^2\mathbf{B} = \mu_0\varepsilon_0\partial^2\mathbf{B}/\partial t^2$. [The approach is analogous to the derivation for $\mathbf{E}$ in Section 1.4.1, but start by taking the curl of the Ampère-Maxwell equation instead of Faraday's law.]

**1.6*** Phase accumulation in a photonic chip.
A silicon waveguide ($n = 3.476$) at 1550 nm has length $L = 500\ \mu\text{m}$. (a) What is the wavevector $k$ inside the waveguide? (b) What is the total phase accumulated? (c) Express this phase in turns (multiples of $2\pi$). (d) A thermo-optic tuner heats the waveguide by $\Delta T = 10\ \text{K}$, changing the index by $\Delta n = 1.8\times10^{-4}\ \text{K}^{-1} \times \Delta T$. What is the resulting phase shift in radians?

**1.7** The Poynting vector and intensity.
A 10 mW laser beam ($\lambda = 1550$ nm) is focused to a Gaussian spot with 1/e² radius $w_0 = 2\ \mu\text{m}$. For a Gaussian beam the peak intensity is $I_0 = 2P/(\pi w_0^2)$. (a) Find the peak intensity. (b) Find the peak electric field amplitude. (c) Find the peak magnetic field amplitude. (d) Calculate the radiation pressure on a perfectly reflecting mirror in this beam.

**1.8** Skin depth at different frequencies.
Calculate the skin depth $\delta = \sqrt{2/(\omega\mu_0\sigma)}$ for copper ($\sigma = 5.8\times10^7$ S/m) at: (a) 60 Hz (power line), (b) 1 GHz (microwave), (c) 193 THz (1550 nm optical). [Note: at optical frequencies, the Drude model predicts a skin depth on the order of the mean free path of electrons, and the formula above is not strictly valid — but compute it anyway and note the trend.]

**1.9** Kramers-Kronig and the plasma dispersion effect.
The carrier-induced absorption change in silicon is $\Delta\alpha = 8.5\times10^{-18}\Delta N_e$ [cm⁻¹] (for electrons). Using the Kramers-Kronig relation as a heuristic, argue qualitatively why there must also be a carrier-induced refractive index change. Is the sign of $\Delta n$ the same as or opposite to $\Delta\alpha$? [Hint: consider the integral in the Kramers-Kronig formula and the sign of $\Delta\kappa$ at frequencies below and above the plasma frequency.]

**1.10*** Boundary conditions at a Si/SiO₂ interface.
A plane electromagnetic wave propagates in silicon ($n_1 = 3.476$) and hits the interface with SiO₂ ($n_2 = 1.444$) at normal incidence. (a) Using boundary conditions, write down the Fresnel reflection coefficient $r = (n_1 - n_2)/(n_1 + n_2)$. (b) Calculate $|r|^2$ (the reflectance). (c) What fraction of the power is transmitted? (d) Would your answer change at the Si/air interface? Compute the reflectance there.

---

## Conceptual Exercises

**1.11** The meaning of the displacement current.
Maxwell added the displacement current to Ampère's law without any direct experimental evidence for it at the time. Explain in your own words: (a) What inconsistency in Ampère's original law did Maxwell identify? (b) Why does adding the displacement current fix the inconsistency? (c) What experimental prediction does the displacement current make, and how was it later confirmed?

**1.12** Thinking about field lines.
(a) Can the electric field lines of a static charge distribution form closed loops? Explain using Maxwell's equations. (b) Can the electric field lines of a time-varying field configuration form closed loops? Explain. (c) Can the magnetic field lines of a magnetized bar magnet form closed loops? Where do they go inside the magnet?

**1.13** The speed of light as a consequence.
Before Maxwell, the speed of light was a measured quantity with no theoretical explanation. After Maxwell, it became a derived quantity — a prediction from $\varepsilon_0$ and $\mu_0$. Discuss the epistemological difference between a measured quantity and a derived quantity in physics. What does it mean for a theory to "explain" a numerical value?

**1.14** The "size" of a photon.
A photon at 1550 nm has wavelength 1550 nm. (a) If you were asked "how big is a photon?", what would you say, and why is the question tricky? (b) In the wave picture, what is the spatial extent of a monochromatic wave? In the particle picture, what is the "size" of a photon? (c) How does the uncertainty principle relate photon frequency bandwidth to spatial localization?

**1.15** Transversality and computing.
Electromagnetic waves are transverse: $\mathbf{E}$ and $\mathbf{B}$ are both perpendicular to $\mathbf{k}$. (a) How many independent polarization directions does this leave for a plane wave propagating in a fixed direction? (b) Explain why this means that a single optical beam carries at least two independent "channels" of information. (c) How does polarization multiplexing exploit this in fiber-optic communications?

**1.16** Energy conservation in a waveguide.
Light propagates without loss in an ideal silicon waveguide. (a) From Poynting's theorem, explain why there is no net energy flow in the direction *perpendicular* to the waveguide axis. (b) What does this imply about the relationship between the field components at the waveguide walls? (c) Is the energy density uniform across the waveguide cross-section? Explain.

---

## Lab / Experimental Exercises

**Lab 1.1: FDTD Simulation of a Plane Wave**
*Tools: Python with Meep (free, open source) or any 1D FDTD code you write yourself*

Implement a 1D finite-difference time-domain (FDTD) simulation of a Gaussian pulse propagating through vacuum and then entering a dielectric slab (n = 3.47, simulating silicon).

(a) Implement the 1D FDTD update equations:
$$E_i^{n+1} = E_i^n + \frac{\Delta t}{\varepsilon\Delta x}\left(H_{i-1/2}^{n+1/2} - H_{i+1/2}^{n+1/2}\right)$$
$$H_{i+1/2}^{n+3/2} = H_{i+1/2}^{n+1/2} + \frac{\Delta t}{\mu_0\Delta x}\left(E_i^{n+1} - E_{i+1}^{n+1}\right)$$
Ensure the Courant stability condition $\Delta t < \Delta x / c$ is satisfied.

(b) Add a Gaussian pulse source at one end. Run the simulation and visualize the propagating field.

(c) Add a dielectric interface at the midpoint. Observe and measure the reflection coefficient. Compare your measured value to the Fresnel formula $r = (n_1 - n_2)/(n_1 + n_2)$.

(d) Repeat for different incident angles (in a 2D simulation) and verify Snell's law.

*What you learn*: The FDTD method is the workhorse of photonic device simulation. Understanding its connection to Maxwell's equations is essential for using commercial photonic design tools (Lumerical FDTD, Meep).

**Lab 1.2: Measuring the Refractive Index with a Fabry-Pérot Etalon**
*Tools: Spectrometer, laser, glass slide or etalon*

A glass slide of thickness $d$ and refractive index $n$ acts as a Fabry-Pérot etalon at normal incidence. The transmitted intensity has maxima when $2nd = m\lambda$ (constructive interference).

(a) Set up a spectrometer to measure the transmission spectrum of a glass slide as a function of wavelength. Identify the fringe pattern.

(b) From the fringe spacing $\Delta\lambda$ and the known plate thickness $d$, calculate the refractive index $n = \lambda^2 / (2d\Delta\lambda)$. Compare to the known value for glass.

(c) If you have access to a tunable laser (or a swept-wavelength source), measure the free spectral range (FSR) directly and compute $n$ from $FSR = \lambda^2/(2nd)$.

*What you learn*: Direct application of the interference condition derived from the wave equation. This measurement technique is used in photonics labs to characterize the effective index of waveguide modes.

**Lab 1.3: Visualizing Electromagnetic Fields with Python**
*Tools: Python, NumPy, Matplotlib*

Write a Python script to:

(a) Visualize the electric field $\mathbf{E}(\mathbf{r})$ of a point charge as a vector field in 2D (the $xy$-plane). Verify that the field lines radiate outward and that the Gauss's law flux integral through a circle of radius $r$ gives a constant value independent of $r$.

(b) Visualize the magnetic field of an infinite straight wire (along $z$) as a 2D vector field in the $xy$-plane. Verify Ampère's law by numerically computing the line integral of $\mathbf{B}$ around a circle.

(c) Visualize a linearly polarized plane wave $E_x(z, t) = E_0\cos(kz - \omega t)$ and its corresponding magnetic field. Animate the propagation in time. Add a dielectric boundary at $z = 0$ and show the reflected and transmitted waves.

(d) **Extension**: Compute and animate the Poynting vector $\mathbf{S} = \mathbf{E}\times\mathbf{H}/\mu_0$ for the plane wave. Show that it always points in the direction of propagation.

*What you learn*: Building physical intuition for electromagnetic fields through visualization. The ability to compute and visualize fields is essential for designing and understanding photonic devices.

**Lab 1.4: The Electromagnetic Spectrum in Context**
*Tools: Python, matplotlib; optional: oscilloscope, function generator*

(a) Compute and plot the complete electromagnetic spectrum from 1 Hz to $10^{24}$ Hz on a logarithmic frequency axis. On the same axis, label: ELF radio waves, AM radio (MHz), FM radio and TV (100 MHz), WiFi (2.4 GHz, 5 GHz), 5G mmWave (28 GHz), infrared (THz), near-infrared telecom (193 THz), visible (400–750 THz), UV, X-rays, gamma rays.

(b) For each labeled band, compute the wavelength, photon energy (in eV and in Joules), and the period of oscillation $T = 1/f$.

(c) Mark the "telecom C-band" on your spectrum (1530–1565 nm = 191.6–196.1 THz). What is the bandwidth of the C-band in THz? In nm? In GHz?

(d) If a single 10-GHz-bandwidth channel in the C-band can carry 400 Gbps of data using coherent 64-QAM modulation, how much total capacity (Tbps) could the entire C-band support if each 10 GHz slot were filled?

*What you learn*: The context of photonic computing wavelengths within the electromagnetic spectrum, and the enormous information-carrying capacity of optical frequencies.

**Thought Experiment 1.5: What If $c$ Were Different?**

Imagine a universe where the permittivity of free space were $\varepsilon_0' = 100\varepsilon_0$, so that $c' = 1/\sqrt{\mu_0\varepsilon_0'} = c/10$.

(a) What would be the wavelength of "1550 nm light" in this universe — that is, light at the same frequency $f = 193$ THz?

(b) Would silicon still be a good waveguide material? Recall that silicon is transparent below its bandgap energy $E_g = 1.1$ eV, which corresponds to $\lambda_g = hc/E_g$. In this universe, what does $\lambda_g$ become, and what does this imply for which materials are transparent?

(c) The information capacity of an optical link scales as $\Delta f / f_{\text{channel}}$. How does the fiber capacity change in this universe, if the fiber loss window remains at the same absolute wavelength range?

(d) What does this thought experiment reveal about the physical assumptions underlying photonic computing? Which properties of photonic computing depend on $c$, and which are wavelength-independent?

*What you learn*: Distinguishing which aspects of photonic computing are fundamental (wavelength-independent, arising from Maxwell's equations) from which are contingent (depending on the specific values of $\varepsilon_0$, $\mu_0$, and material properties). This kind of counterfactual reasoning is how physicists identify which aspects of a theory are deep and which are accidental.

**Thought Experiment 1.6: Optical vs. Electronic Interconnects — A First-Principles Comparison**

Without using any numbers from the photonic computing literature, derive from Maxwell's equations why optical interconnects might be more energy-efficient than electrical interconnects for long-distance information transfer.

Specifically:
(a) In an electrical interconnect (copper wire), the current $I$ creates a magnetic field, and the signal carries energy at rate $P = I^2 R$. The energy dissipated per bit is $E_{\text{bit}} \propto CV^2$. How does $E_{\text{bit}}$ scale with wire length?

(b) In an optical interconnect (optical fiber), a photon at 1550 nm carries energy $E_{\text{photon}} = 0.8$ eV and can propagate for kilometers with loss ~0.2 dB/km. What is the minimum energy to transmit one bit optically? How does this scale with distance?

(c) What limits the minimum number of photons per bit in practice (noise, detection, encoding)?

(d) Based on your analysis, for what interconnect distances does optical become more efficient than electrical? What engineering factors determine the crossover point?

*What you learn*: The fundamental basis for the energy efficiency argument for optical interconnects — an argument that is central to the motivation for photonic computing.
