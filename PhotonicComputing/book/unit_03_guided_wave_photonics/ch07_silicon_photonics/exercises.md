# Chapter 7: Exercises

---

## Mathematical Exercises

**M7.1 — Waveguide Mode Cutoff**

A silicon strip waveguide (width $w$, height $h = 220$ nm) on SiO₂ is designed for single-mode TE operation at 1550 nm. Using the effective index method (treating the 2D geometry as two 1D slab problems):

(a) Find the cutoff condition for the second-order TE mode (TE₂₀) as a function of $w$. At what width $w$ does the second-order mode cut off?

(b) The standard width is $w = 450$ nm. What is the margin from the second-order mode cutoff? Express as a fraction of $\lambda$.

(c) At $w = 450$ nm, $h = 220$ nm, the effective index is $n_{\text{eff}} \approx 2.42$ and the group index is $n_g \approx 4.24$. Derive the group index from the dispersion relation $n_g = n_{\text{eff}} - \lambda(dn_{\text{eff}}/d\lambda)$ and explain physically why $n_g \gg n_{\text{eff}}$ in a tight waveguide.

**M7.2 — MZI Transfer Function**

(a) Derive the MZI transfer matrix $U_{\text{MZI}}$ by multiplying the three matrices: input 3-dB coupler, phase arms, and output 3-dB coupler. Show that the result is:

$$U_{\text{MZI}} = ie^{i(\phi_1+\phi_2)/2}\begin{pmatrix}\sin(\Delta\phi/2) & \cos(\Delta\phi/2) \\ \cos(\Delta\phi/2) & -\sin(\Delta\phi/2)\end{pmatrix}$$

(b) For push-pull operation ($\phi_1 = \phi_0 + \delta\phi$, $\phi_2 = \phi_0 - \delta\phi$), show that the output intensity is $I_{\text{out}} = I_{\text{in}}\cos^2(\delta\phi)$ and that the common-mode phase $e^{i\phi_0}$ does not depend on $\delta\phi$ (zero chirp).

(c) For an MZI biased at quadrature ($\delta\phi_0 = \pi/4$), compute the small-signal transfer function $dI/d(\delta\phi)|_{\pi/4}$ and show it is maximum at this point. At what bias point is the MZI most linear? Most nonlinear?

**M7.3 — Plasma Dispersion Efficiency**

A silicon PN junction modulator has the following parameters:
- Junction doping: $N_A = N_D = 5 \times 10^{17}$ cm⁻³
- Built-in voltage: $V_{bi} = 0.85$ V
- Reverse bias sweep: 0 to $-3$ V
- Depletion width within the 450-nm-wide waveguide: $W_d(V) = 150\sqrt{0.85-V}$ nm (with $V$ in volts)

(a) Using the Soref-Bennett equations, calculate $\Delta n$ as a function of reverse voltage $V$. Assume the depletion region sweeps uniformly across the waveguide, removing carriers of density $N = 5 \times 10^{17}$ cm⁻³.

(b) For a phase-shifter length of 2 mm with confinement factor $\Gamma = 0.8$, calculate the phase shift as a function of voltage and find $V_\pi$.

(c) The junction capacitance per unit length is $C_j = \varepsilon_{\text{Si}}/W_d$. For a 2-mm-long modulator at 0 V reverse bias, calculate the total capacitance and the RC-limited bandwidth with $R = 50\ \Omega$.

**M7.4 — Microring Modulator Thermal Sensitivity**

A silicon microring modulator has radius $R = 10$ μm, group index $n_g = 4.24$, and quality factor $Q = 10^4$ at 1550 nm.

(a) Calculate the FSR in GHz and in nm.

(b) The resonance linewidth (FWHM) is $\delta\lambda = \lambda_{\text{res}}/Q$. Calculate $\delta\lambda$ in pm.

(c) The thermal sensitivity is $d\lambda_{\text{res}}/dT = \lambda_{\text{res}}(dn/dT)/n_g = 69$ pm/K. How many degrees of temperature change corresponds to one resonance linewidth shift? Can this ring modulator operate without active temperature stabilization in a chip with $\pm 2°C$ temperature fluctuations?

(d) A local TiN heater can provide $P_\pi \approx 5$ mW to trim the resonance by one linewidth. How much total power would be needed to hold 64 rings at their resonance wavelengths, assuming each ring requires on average $P_\pi/4$ of correction?

**M7.5 — Pockels Effect in LNOI**

(a) Starting from $\Delta n_e = -\frac{1}{2}n_e^3 r_{33} E_z$ for LiNbO₃, and using $r_{33} = 30.9$ pm/V, $n_e = 2.138$, derive $V_\pi L$ for an LNOI modulator with electrode gap $d = 5$ μm at $\lambda = 1550$ nm. Compare to silicon PN depletion.

(b) For a push-pull LNOI MZI modulator ($V_\pi L = 2.2$ V·cm) of length $L = 5$ mm, what is $V_\pi$? What is the drive voltage required for a phase shift of 0.1 rad?

(c) An LNOI EO modulator has zero chirp ($\alpha_H = 0$) while a silicon depletion modulator has $\alpha_H = -2$. For a Gaussian pulse of duration $\tau_p = 10$ ps modulated onto a carrier at 1550 nm, calculate the bandwidth broadening due to chirp in the silicon case. Use $\Delta\nu_{\text{chirp}} \approx (1 + \alpha_H^2)^{1/2}\Delta\nu_{\text{transform-limited}}$ where $\Delta\nu_{\text{TL}} = 0.44/\tau_p$.

---

## Conceptual Exercises

**C7.1 — Why Not Build a Laser in Silicon?**

(a) Silicon has an indirect bandgap. Explain in physical terms (using momentum conservation) why this makes efficient light emission essentially impossible in bulk silicon, while GaAs (direct bandgap) emits efficiently.

(b) The Purcell effect can enhance spontaneous emission rates. Could a very high-Q silicon resonator be used to make a silicon laser by enhancing the phonon-assisted radiative rate? Estimate the enhancement factor needed and assess whether this is practical.

(c) Silicon has a Raman gain coefficient $g_R \approx 76$ cm/GW at 1550 nm. Can Raman gain be used to make a silicon laser? What external component is required and why? (See Boyraz & Jalali, *Optics Express*, 2004 and Jones et al., *Optics Express*, 2005.)

**C7.2 — The Scalability Wall**

Consider a photonic matrix multiplier based on MZI mesh (Clements decomposition) for an $N \times N$ unitary matrix:

(a) How many MZI elements are needed for exact $N \times N$ unitary decomposition? (The Clements architecture requires $N(N-1)/2$ MZIs.)

(b) If each MZI occupies 50 μm × 100 μm and each thermo-optic phase shifter consumes 20 mW static power, calculate the chip area and total static power for $N = 4, 8, 16, 32$.

(c) At what $N$ does the chip area exceed 1 cm² (a typical reticle limit)? At what $N$ does the static power exceed 50 W (a practical cooling limit)?

(d) What three hardware innovations would most relieve these bottlenecks, and what are the realistic timescales for their development?

**C7.3 — Resonant vs. Non-Resonant Modulators**

An engineer must choose between an MZI modulator (non-resonant) and a ring modulator (resonant) for a WDM photonic matrix multiplier. List the key tradeoffs and for each of the following application scenarios, argue which is preferable:

(a) A matrix with 64 weights that are updated once per second (slow reconfiguration).
(b) A system with 100 GHz bandwidth per channel and ambient temperature variations of ±5°C.
(c) A chip where power budget is limited to 100 mW total including all components.
(d) A system where weight precision must be 8 bits.

**C7.4 — Comparing Platforms**

You are designing a photonic tensor processing unit (TPU) for a data center. Your design requires:
- Matrix-vector multiplication of 64×64 complex matrices
- Input data rate: 100 Gbps per port (64 ports)
- Reconfiguration rate: 1 MHz (weights updated every microsecond)
- Operating environment: datacenter (25°C ± 3°C ambient, tight power budget)

For each of the four platforms (Si, Si₃N₄, LNOI, InP), identify one critical capability it contributes and one critical limitation that prevents using it exclusively. Sketch a heterogeneous integration strategy that combines three of the four platforms to meet all requirements.

---

## Lab/Experimental Exercises

**L7.1 — Silicon Waveguide Simulation (Python)**

Using the open-source tools `lumopt`, `MEEP`, or `EMpy` (install via pip):

(a) Simulate the mode profiles (field distributions) of the TE and TM fundamental modes in a 450×220 nm silicon waveguide at 1550 nm. Plot the $|E_y|^2$ field for the TE mode and compare the confinement factor $\Gamma$ to the analytical approximation $\Gamma \approx 1 - e^{-2\kappa d}$ (which applies to symmetric slabs).

(b) Vary the waveguide width from 300 nm to 600 nm and plot $n_{\text{eff}}$ vs. width for the fundamental TE mode. At what width does the second-order TE mode appear?

(c) Calculate the group index $n_g = n_{\text{eff}} - \lambda \frac{dn_{\text{eff}}}{d\lambda}$ from your simulation by computing $n_{\text{eff}}$ at three wavelengths near 1550 nm. Compare to the literature value of $n_g = 4.24$.

**L7.2 — MZI Transfer Function Measurement (or Simulation)**

If you have access to a silicon photonic chip (or to a fiber-optic bench-top MZI), measure (or simulate):

(a) The transmission spectrum of one arm of an unbalanced MZI (path length difference $\Delta L = 50$ μm) as a function of wavelength. Fit the transmission $T(\lambda) = \cos^2(\pi n_g \Delta L / \lambda)$ to extract $n_g$.

(b) Apply a DC voltage to a thermo-optic heater on one arm and measure the transmission shift as a function of voltage. Extract the $V_\pi$ for this device.

(c) At quadrature bias, measure the small-signal transfer function (frequency response from electrical to optical domain) using a vector network analyzer (VNA) or equivalent. From the 3 dB bandwidth, extract the RC time constant and compare to the known heater capacitance and resistance.

**L7.3 — Plasma Dispersion Numerical Integration (Python)**

Write a Python script to:

(a) Implement the Soref-Bennett equations as a function `delta_n(N_e, N_h, wavelength)` and `delta_alpha(N_e, N_h, wavelength)` with all numerical coefficients.

(b) Plot $\Delta n$ and $\Delta\alpha$ as a function of carrier density $N$ (equal electrons and holes) from $10^{16}$ to $10^{18}$ cm⁻³ at $\lambda = 1550$ nm and $\lambda = 1310$ nm. Verify the $\lambda^2$ scaling of $\Delta n$.

(c) For a depletion modulator with junction profile $W_d(V) = 150\sqrt{0.85-V}$ nm (with $V$ in volts, $W_d$ in nm, and waveguide width = 450 nm), plot $\Delta n_{\text{eff}} = \Gamma \Delta n_{\text{bulk}} \times (W_d/w)$ vs. $V$ for $V \in [-3, 0]$ V. Integrate over a 2-mm modulator length to get the total phase shift and voltage-phase curve $\Delta\phi(V)$.

(d) Add the absorption change $\Delta\alpha$ to compute the extinction ratio available from the carrier-induced absorption alone (for a segment of length $L = 100$ μm), and compare to the phase-only extinction ratio when using this segment in an MZI.
