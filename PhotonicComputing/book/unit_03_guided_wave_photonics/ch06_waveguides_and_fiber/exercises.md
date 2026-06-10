# Chapter 6 Exercises: Waveguides and Fiber

**Exercise 6.1 — Slab Waveguide Mode Analysis**

(a) For a symmetric Si/SiO₂ slab waveguide at 1550 nm ($n_1 = 3.48$, $n_2 = 1.44$), compute the V-number as a function of core thickness $d$ for $d$ = 50, 100, 150, 200, 220 nm. How many TE modes does each thickness support?

(b) Solve the TE even mode eigenvalue equation $\kappa\tan(\kappa d/2) = \gamma$ graphically for $d = 220$ nm. Find the effective index $n_{eff} = \beta/k_0$.

(c) Compute the confinement factor $\Gamma$ for the TE₀ mode at $d = 220$ nm. How does it compare to the value quoted in the text (0.8 for the 2D strip waveguide)?

(d) Compute the evanescent field decay length $1/\gamma$ in the SiO₂ cladding. How far does the field extend into the cladding? This sets the minimum oxide thickness needed to prevent leakage to the Si substrate.

---

**Exercise 6.2 — Fiber Mode and Single-Mode Analysis**

(a) For a step-index fiber with $n_1 = 1.4677$, $n_2 = 1.4627$, $a = 4.1$ μm, compute the V-number at 1310 nm and 1550 nm. Is the fiber single-mode at both wavelengths?

(b) The LP$_{11}$ mode has cutoff at $V_c = 2.405$. Compute the cutoff wavelength for this fiber.

(c) For the LP$_{01}$ mode, the field profile is approximately Gaussian with $1/e$ radius $w \approx a(0.65 + 1.619/V^{1.5} + 2.879/V^6)$ (Marcuse approximation). Compute $w$ at 1550 nm and compare to the stated MFD of SMF-28.

(d) Two SMF-28 fibers are spliced with a transverse offset of 1 μm. Using the Gaussian approximation, compute the splice loss.

---

**Exercise 6.3 — Fiber Attenuation and Loss Budget**

(a) Using the Sellmeier equation for silica (from Chapter 3, Exercise 3.12), numerically compute the Rayleigh scattering loss at 800 nm, 1060 nm, 1310 nm, 1550 nm, and 2000 nm. Plot $\alpha_{Rayleigh}$ vs. $\lambda$.

(b) A transatlantic submarine cable is 7000 km long with EDFA amplifiers spaced every 80 km. At 1550 nm with 0.18 dB/km loss: how many amplifiers are needed, and what is the total fiber loss that must be compensated?

(c) If each EDFA has NF = 5 dB and gain = 14.4 dB (to compensate exactly 80 km × 0.18 dB/km), and the signal is at 0 dBm at each amplifier input, compute the OSNR (in 0.1 nm reference bandwidth) at the end of the link using the cascaded Friis formula.

(d) Modern coherent 400 Gbps DP-16QAM transmission requires OSNR > 23 dB. Is the OSNR from part (c) sufficient? What would you need to change (pump power, amplifier spacing, EDFA NF) to meet the requirement?

---

**Exercise 6.4 — Nonlinear Fiber Optics for WDM**

A WDM system uses SMF-28 with 80 channels at 100 GHz spacing. Each channel launches at +5 dBm.

(a) Compute the total launch power. Is this above the SBS threshold for a single channel? (Recall SBS threshold: ~1 mW for a narrow-linewidth cw source over 50 km.)

(b) Compute the FWM phase mismatch $|\Delta k|$ for three adjacent channels at 100 GHz spacing, using $\beta_2 = -21.7$ ps²/km at 1550 nm. Compare $|\Delta k|L_{eff}$ to 1 to determine whether FWM is significant.

(c) If the channel spacing is reduced to 25 GHz for higher spectral efficiency: does FWM become a problem? What does this imply for the minimum fiber dispersion needed for 25 GHz WDM?

(d) The Shannon capacity per WDM channel is $C = B\log_2(1 + \text{SNR})$. For OSNR = 25 dB in 100 GHz bandwidth, what is the maximum capacity per channel? What modulation format (QPSK, 16-QAM, 64-QAM) best approaches this capacity?

---

**Lab Exercise 6.1 — Waveguide Mode Solver**

Implement a finite difference mode solver for a 2D waveguide cross-section.

(a) Set up a finite difference grid for a 450 × 220 nm Si strip waveguide on SiO₂ substrate. Use a simulation window of 3 × 3 μm with perfectly matched layer (PML) absorbing boundary conditions.

(b) Solve the 2D eigenvalue problem (matrix form of Maxwell's equations) for the TE modes using `scipy.sparse.linalg.eigs`. Identify the TE₀₀ and TE₁₀ modes.

(c) Compute the effective indices $n_{eff}$ for both modes and compare to published results (~2.4 for TE₀₀ at 450 nm width).

(d) Vary the waveguide width from 300–600 nm and track $n_{eff}$ for TE₀₀ and TE₁₀. Identify the single-mode cutoff width. Compute the GVD coefficient $\beta_2 = d^2\beta/d\omega^2$ at 1550 nm for several waveguide widths by numerical differentiation of $\beta(\omega)$.
