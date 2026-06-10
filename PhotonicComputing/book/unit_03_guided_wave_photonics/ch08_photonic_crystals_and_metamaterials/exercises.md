# Chapter 8: Exercises

---

## Mathematical Exercises

**M8.1 — Bragg Reflector Bandgap**

A Bragg reflector consists of alternating layers of TiO₂ ($n_A = 2.35$) and SiO₂ ($n_B = 1.46$), each at quarter-wave thickness for $\lambda_0 = 1550$ nm.

(a) Calculate the layer thicknesses $d_A$ and $d_B$ for the quarter-wave condition.

(b) Using the formula $\Delta\omega/\omega_0 \approx (4/\pi)\arcsin[(n_A-n_B)/(n_A+n_B)]$, calculate the bandgap width as a fraction of the center frequency.

(c) For $N = 10$ layer pairs on a glass substrate, calculate the peak reflectance at the Bragg wavelength using $R_N = [(1-(n_A/n_B)^{2N})/(1+(n_A/n_B)^{2N})]^2$.

(d) Compare to a Si/SiO₂ Bragg reflector with the same number of pairs. Which has higher reflectance? Which has a wider stopband? Explain the tradeoff.

**M8.2 — Photonic Crystal Band Structure**

The Bragg dispersion relation for a 1D photonic crystal with period $a = d_A + d_B$ and layer indices $n_A$, $n_B$ is:

$$\cos(Ka) = \cos(k_A d_A)\cos(k_B d_B) - \frac{1}{2}\left(\frac{n_A}{n_B}+\frac{n_B}{n_A}\right)\sin(k_A d_A)\sin(k_B d_B)$$

where $k_i = n_i \omega/c$ and $K$ is the Bloch wavevector.

(a) For a Si ($n_A = 3.478$, $d_A = 111$ nm) / air ($n_B = 1$, $d_B = 775$ nm) crystal with $a = 886$ nm, numerically solve this equation to plot $\omega a/c$ vs. $Ka/\pi$ for $Ka/\pi \in [0, 1]$ (first Brillouin zone). Show the band structure for the first three bands.

(b) Identify the bandgap(s) and calculate their width in dimensionless units $\Delta(\omega a/c)$.

(c) At the Brillouin zone edge ($Ka = \pi/a$), the dispersion relation requires $\cos(Ka) = -1$. Find the frequencies $\omega_1^{\text{gap}}$ and $\omega_2^{\text{gap}}$ bounding the first gap by solving the dispersion relation numerically.

**M8.3 — Metasurface Phase Profile for Focusing**

Design a metasurface metalens to focus a normally incident plane wave at $\lambda = 1550$ nm to a point $f = 100$ μm above the surface.

(a) Derive the required phase profile $\phi(r)$ where $r = \sqrt{x^2+y^2}$ is the radial coordinate.

(b) The phase is sampled at discrete meta-atom positions with spacing $\Lambda = 500$ nm. How many meta-atoms are needed to cover a 50-μm-diameter aperture? What is the maximum phase gradient $d\phi/dr$ at the edge of the lens, and verify it satisfies the sub-wavelength sampling requirement $|d\phi/dr| < 2\pi/\Lambda$.

(c) For a PB phase implementation, what range of rotation angles $\alpha$ is required across the aperture? A single meta-atom orientation provides a phase of $2\alpha$ for RCP incidence. What rotation angle provides the maximum phase ($\phi = 2\pi$)?

(d) The metalens has diameter $D = 50$ μm and focal length $f = 100$ μm. Calculate the numerical aperture NA = $D/(2f)$ and the diffraction-limited focal spot size $d = 0.51\lambda/\text{NA}$.

**M8.4 — SPP Dispersion**

For a gold/air interface at $\lambda = 1550$ nm, with gold dielectric function $\varepsilon_m = -114 + 11i$:

(a) Calculate $k_{\text{SPP}} = k'_{\text{SPP}} + ik''_{\text{SPP}}$ using the SPP dispersion relation $k_{\text{SPP}} = (\omega/c)\sqrt{\varepsilon_m\varepsilon_d/(\varepsilon_m+\varepsilon_d)}$ with $\varepsilon_d = 1$ (air).

(b) The propagation length is $L_{\text{SPP}} = 1/(2k''_{\text{SPP}})$. Calculate $L_{\text{SPP}}$ in μm.

(c) The confinement in the dielectric is $1/\kappa_d$ where $\kappa_d = \sqrt{k'^2_{\text{SPP}} - \varepsilon_d(\omega/c)^2}$. Calculate the SPP field decay length in air.

(d) Compare the propagation length to the confinement depth: is this SPP "tightly confined" (propagation length << confinement depth) or "loosely confined" (propagation length >> confinement depth)? What does this imply for its utility as a waveguide at telecom wavelengths?

---

## Conceptual Exercises

**C8.1 — Photonic Crystal vs. Silicon Waveguide**

(a) A silicon strip waveguide (450×220 nm) achieves single-mode confinement by total internal reflection. A 2D photonic crystal waveguide achieves confinement by the photonic bandgap. List three fundamental differences between the two confinement mechanisms.

(b) A photonic crystal waveguide has a group velocity $v_g = c/30$. Explain in physical terms (Bloch modes, standing waves, energy oscillation) why the group velocity is reduced.

(c) A photonic crystal waveguide has propagation loss 30 dB/cm at $v_g = c/30$. Estimate the loss at $v_g = c/100$ using the $v_g^{-2}$ loss scaling. Is this waveguide useful for an optical interconnect?

(d) The slow-light enhancement of nonlinear phase shift scales as $S^2 = (c/v_g)^2$, but so does the loss. Show that for a device with length chosen so that the insertion loss is fixed at 3 dB, the nonlinear phase shift accumulated does not benefit from slow light in the loss-limited regime.

**C8.2 — The Linearity Problem in D²NNs**

(a) Explain why a stack of linear optical layers (diffractive or otherwise) is mathematically equivalent to a single linear optical transformation. What algebraic structure does this follow from?

(b) To add nonlinearity to a D²NN, one could detect the optical signal after each layer and re-illuminate the next layer using a spatial light modulator. Describe the energy cost of this approach compared to a fully passive D²NN, and identify the bottleneck that prevents it from being "optical" computation in any useful sense.

(c) An alternative is to use nonlinear optical materials between layers. The intensity-dependent phase shift in silicon for a 100-μm length is $\delta\phi_{\text{NL}} = \gamma P L = 290 \times P \times 10^{-4}$ rad for power $P$ in watts. For an on-chip D²NN operating at 1 mW average power, what is $\delta\phi_{\text{NL}}$? Is this sufficient for a useful nonlinear activation function?

(d) What intensity would be needed for a "significant" nonlinear phase shift (say $\delta\phi_{\text{NL}} = 0.1$ rad)? Is this achievable without damaging silicon via TPA?

**C8.3 — Plasmonics and the Loss Problem**

(a) The SPP propagation length scales as $L_{\text{SPP}} \propto |\varepsilon_m'|^2/\varepsilon_m''$. For a material with $\varepsilon_m' = -100$ and $\varepsilon_m'' = 1$ (an ideal material with 10× lower loss than gold at 1550 nm), calculate $L_{\text{SPP}}$.

(b) Does such a material exist? Using the Kramers-Kronig relations (Section 3.1), argue that a material with $|\varepsilon_m'| = 100$ at a given frequency must also have large $\varepsilon_m''$ in some frequency range. Can the $\varepsilon_m''$ be moved to a different frequency, away from the operating wavelength?

(c) Noble metals (gold, silver) already represent near-optimal choices for room-temperature optical plasmonics. Propose a specific material or operating condition that might substantially improve the FOM = $|\varepsilon_m'|^2/\varepsilon_m''$, and assess its practicality.

---

## Lab/Experimental Exercises

**L8.1 — Bragg Reflector Simulation (Python/TMM)**

Using the `tmm` Python package (pip install tmm) or the transfer matrix code:

(a) Simulate the reflectance spectrum of a TiO₂/SiO₂ Bragg reflector ($n_A = 2.35$, $n_B = 1.46$) with $N = 5$, 10, and 20 layer pairs, designed for $\lambda_0 = 1550$ nm. Plot $R(\lambda)$ from 1000–2000 nm.

(b) Show that as $N$ increases, the stopband becomes flatter (more rectangular) and the side lobes become more numerous. Explain this in terms of the $N$-slit interference analogy.

(c) Simulate a chirped Bragg reflector: vary the period linearly from $a_1 = 400$ nm to $a_2 = 600$ nm across 20 pairs. How does the reflection spectrum change? What might this be useful for?

**L8.2 — Photonic Crystal Band Structure with MPB (Python)**

Install the MIT Photonic Bands package (mpb) or use its Python bindings (meep's `mpb` interface):

(a) Compute the photonic band structure of a 2D triangular lattice of air holes ($r/a = 0.3$) in silicon ($\varepsilon = 12$) for both TE and TM polarizations. Plot the band diagram along $\Gamma$-$M$-$K$-$\Gamma$.

(b) Identify any complete photonic bandgap (existing for both TE and TM) and partial gaps (existing for only one polarization).

(c) Design a W1 waveguide (one row of missing holes) and compute the guided mode dispersion. At the Brillouin zone edge, estimate the group velocity from $v_g = d\omega/dk$.

**L8.3 — Metasurface Phase Profile Visualization (Python)**

(a) Write a Python script to compute and visualize the required phase profile $\phi(x, y)$ for a metalens focusing light at $f = 200$ μm, $\lambda = 1550$ nm, over a 100 × 100 μm aperture.

(b) Discretize the phase to the nearest allowed PB rotation angle, using a discrete set of angles $\alpha_k = k\pi/8$ for $k = 0, 1, \ldots, 7$ (8 levels). Plot the discretized rotation map and the corresponding quantized phase.

(c) Simulate the far-field diffraction pattern produced by this discretized metalens using the angular spectrum method (2D FFT of the aperture field with phase $e^{i\phi_{\text{discretized}}(x,y)}$). Compare the focused spot size and peak intensity to the ideal (continuous phase) case.
