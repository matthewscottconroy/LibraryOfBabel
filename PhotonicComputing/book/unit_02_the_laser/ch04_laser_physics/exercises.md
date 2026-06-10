# Chapter 4 Exercises: Laser Physics

---

## Mathematical Exercises

**Exercise 4.1 — Three-Level Laser Threshold**

A three-level laser has upper level lifetime $\tau_2 = 3$ ms (ruby), total ion density $N_T = 1.58 \times 10^{19}$ cm$^{-3}$, stimulated emission cross-section $\sigma = 2 \times 10^{-20}$ cm², and cavity length $L = 5$ cm with mirror reflectivities $R_1 = 1.0$, $R_2 = 0.65$, internal loss $\alpha_i = 0.005$ cm$^{-1}$.

(a) Compute the threshold gain $g_{th} = \alpha_i + (1/2L)\ln(1/R_1R_2)$.

(b) Find the threshold inversion density $\Delta N_{th} = g_{th}/\sigma$.

(c) Since the three-level lower level is the ground state, threshold inversion requires $N_2 > N_T/2$. What fraction of ions must be in the upper level at threshold? Compare to the four-level case where no lower-level population constraint exists.

(d) The pump transition is at 550 nm (green), and the laser wavelength is 694.3 nm. Compute the quantum defect (ratio of lasing photon energy to pump photon energy). What fraction of the pump energy becomes heat?

---

**Exercise 4.2 — Semiconductor Laser Rate Equations**

A 1550 nm InGaAsP DFB laser has: differential gain $a = 2 \times 10^{-16}$ cm², transparency density $N_0 = 1.5 \times 10^{18}$ cm$^{-3}$, active volume $V = 50 \times 2 \times 0.1 \mu\text{m}^3$, confinement factor $\Gamma = 0.05$, photon lifetime $\tau_p = 2$ ps, carrier lifetime $\tau = 2$ ns, internal quantum efficiency $\eta_i = 0.9$.

(a) Compute the threshold carrier density $N_{th}$ from $\Gamma a (N_{th} - N_0) = 1/(v_g \tau_p)$ where $v_g = c/n_g$ with $n_g = 3.7$.

(b) Compute the threshold current $I_{th} = eV N_{th}/\tau \cdot (1/\eta_i)$.

(c) Compute the relaxation oscillation frequency at 2× threshold: $f_R = (1/2\pi)\sqrt{v_g a S_{th}/\tau_p}$ where $S_{th} = \Gamma \tau_p (I_{th}/eV)/(n_g/c \cdot \tau)$. [Use the approximation that $S$ scales linearly above threshold.]

(d) Estimate the −3 dB modulation bandwidth $\approx 1.55 f_R$. How does this compare to the photon and carrier lifetimes?

---

**Exercise 4.3 — Schawlow-Townes Linewidth**

A DFB laser at 1550 nm has: output power $P = 5$ mW, photon lifetime $\tau_p = 2$ ps, spontaneous emission factor $n_{sp} = 2$, linewidth enhancement factor $\alpha_H = 4$.

(a) Compute the Schawlow-Townes linewidth $\Delta\nu_{ST} = \hbar\omega/(\pi\tau_p^2 P) \cdot n_{sp}$ (note: using the equivalent form with photon lifetime, see Section 4.2.3).

(b) Compute the actual linewidth including the Henry factor enhancement: $\Delta\nu = \Delta\nu_{ST}(1 + \alpha_H^2)$.

(c) What is the coherence length $L_c = c/(\pi\Delta\nu n_g)$ for this laser in a Si waveguide ($n_g = 4.2$)?

(d) If the laser is used in an MZI photonic matrix processor with maximum arm-length difference $\Delta L$, what is the maximum $\Delta L$ that keeps interference visibility $V > 0.99$? Use $V = \exp(-\pi\Delta\nu \cdot n_g \Delta L/c)$.

---

**Exercise 4.4 — VCSEL Design**

A GaAs VCSEL at 850 nm uses 25 pairs of AlAs/GaAs DBR mirrors (top) and 30 pairs (bottom). The refractive indices are $n_{AlAs} = 2.95$ and $n_{GaAs} = 3.52$.

(a) Compute the reflectivity of the top mirror (25 pairs): $R = \tanh^2(N \Delta n/n_{avg})$ approximately, or use the exact formula $R = [1 - (n_L/n_H)^{2N}(n_s/n_0)]^2/[1 + (n_L/n_H)^{2N}(n_s/n_0)]^2$ where $n_0 = 1$ (air), $n_s = n_{GaAs}$ (substrate).

(b) The cavity length $L_{eff} = L_{cav} + L_{pen,top} + L_{pen,bot}$ includes penetration depth into each mirror: $L_{pen} = \lambda/(4\Delta n)$ where $\Delta n = n_H - n_L$. Compute $L_{pen}$ for each mirror. If the physical cavity length is 1.2 μm, compute $L_{eff}$.

(c) Compute the FSR: $\Delta\nu_{FSR} = c/(2n_{cav}L_{eff})$. Show that only one longitudinal mode falls under a gain bandwidth of ~30 nm for GaAs at 850 nm.

(d) The threshold current for an oxide-aperture VCSEL with 4 μm diameter aperture and threshold current density $J_{th} = 1 \times 10^3$ A/cm² is: $I_{th} = J_{th} \pi d^2/4$. Compute $I_{th}$. What is the power consumption at threshold at 1.5 V operating voltage?

---

**Exercise 4.5 — Mode-Locked Pulse Spectrum**

A mode-locked fiber laser at 1550 nm produces transform-limited sech² pulses with $T_{FWHM} = 200$ fs.

(a) Compute the spectral bandwidth $\Delta\nu_{FWHM}$ using the time-bandwidth product for sech² pulses: $\Delta t \cdot \Delta\nu = 0.315$.

(b) Convert $\Delta\nu_{FWHM}$ to $\Delta\lambda_{FWHM}$ using $\Delta\lambda \approx \lambda^2 \Delta\nu/c$.

(c) If the laser has a repetition rate of $f_{rep} = 250$ MHz, how many comb lines fall within the spectral bandwidth $\Delta\nu_{FWHM}$?

(d) The peak power of the pulse is $P_{peak} = E_{pulse}/(0.88 T_{FWHM})$ for sech². If the average power is 100 mW, compute the peak power.

---

**Exercise 4.6 — Microresonator Comb Threshold**

A Si₃N₄ ring resonator has: radius $R = 100$ μm, effective mode area $A_{eff} = 1$ μm², nonlinear index $n_2 = 2.4 \times 10^{-19}$ m²/W, loaded Q factor $Q_L = 5 \times 10^5$, coupling Q $Q_c = 10^6$ (so the resonator is under-coupled).

(a) Compute the resonance linewidth $\kappa_{tot} = \omega_0/Q_L$ and the FSR $= c/(2\pi n_g R)$ with $n_g = 1.9$.

(b) The threshold pump power for OPO in a microresonator is approximately:

$$P_{th} \approx \frac{1}{8} \frac{n_0^2 V_{eff}}{n_2 \omega_0 Q_c Q_L^2 / \omega_0^2}$$

A simpler estimate: $P_{th} \approx \pi n_0 A_{eff}/(n_2 \omega_0 \cdot Q_L \cdot \eta_{coup} L)$ where $L = 2\pi R$ and $\eta_{coup} = Q_L/Q_c$. Compute $P_{th}$ using this estimate.

(c) How many comb lines fit within the C-band (1530–1565 nm, bandwidth ~4.4 THz) at this FSR?

(d) If each comb line needs > 0.5 mW for modulation, and the pump provides 100 mW, what is the conversion efficiency per line that would be required? Comment on whether this is achievable with current DKS combs (typical total conversion ~5–10%).

---

## Conceptual Exercises

**Exercise 4.7 — Why Silicon Cannot Lase (Yet)**

(a) Explain in terms of band structure why silicon's indirect bandgap makes stimulated emission extremely unlikely at the direct transition wavelength (~1100 nm). What is the role of phonons in non-radiative recombination?

(b) Tensile strain can shift the Ge conduction band toward a direct gap. Liu et al. (2010) demonstrated a Ge-on-Si laser with threshold current density $\sim 10^4$ A/cm² — about 10–100× higher than III-V lasers. What physical factor causes this high threshold, and why is it difficult to reduce?

(c) Erbium ions (Er³⁺) in silica emit at 1550 nm with near-unity quantum efficiency, even though silica is not a semiconductor. The Er³⁺ transition is $^4$I$_{13/2} \to ^4$I$_{15/2}$, a 4f intra-shell transition shielded from the host lattice by outer electron shells. Why does this shielding lead to low non-radiative quenching? What would happen if you tried to make an EDWA in silicon instead of silica or Si₃N₄?

(d) If a room-temperature, high-efficiency silicon laser is eventually demonstrated (e.g., through quantum confinement in nanowires or quantum dots), what would be the consequence for photonic computing chip architecture? What integration challenges would remain even with on-chip lasing?

---

**Exercise 4.8 — The α-Factor and Its Consequences**

The linewidth enhancement factor $\alpha_H = -(\partial n/\partial N)/(\partial g/\partial N) \cdot 2k_0$ couples amplitude (gain) changes to phase (refractive index) changes in a semiconductor laser.

(a) A DFB laser biased at threshold is switched on. The initial photon density is low; stimulated emission is negligible; carriers accumulate. As the photon density builds up, carrier density clamps at $N_{th}$. Before clamping, the carrier density spike causes a refractive index change $\Delta n = (\partial n/\partial N)\Delta N$. Using $\alpha_H = 4$ and $g_{th} = 50$ cm$^{-1}$, estimate the frequency chirp during the turn-on transient.

(b) In a directly modulated DFB transmitting on-off keying, adiabatic chirp causes the "1" bits to be at a slightly different wavelength than the "0" bits. For a 10 mW "1" bit and 0.1 mW "0" bit with $\alpha_H = 4$ and $\kappa = 10^{-12}$ W$^{-1}$s$^{-1}$ (adiabatic chirp coefficient), estimate the wavelength shift between "0" and "1" levels.

(c) In a coherent photonic computing MZI, the input laser is CW and there is no direct modulation — the laser is modulated by an external electro-optic modulator. In this case, does $\alpha_H$ of the laser contribute chirp to the signal? Explain why or why not.

(d) Quantum dot lasers have $\alpha_H$ close to zero, compared to $\alpha_H = 3$–5 for bulk and quantum well lasers. Explain physically why quantum dots (discrete energy levels, $\delta$-function density of states) suppress the linewidth enhancement factor.

---

**Exercise 4.9 — Laser Sources for Photonic Computing: Trade-Off Analysis**

Consider a photonic matrix processor that must drive 32 wavelength-division multiplexed channels, each at 100 GHz spacing across the C-band, with 1 mW power per channel. Compare the following source architectures:

(a) **32 discrete DFB lasers, TEC-stabilized**: Estimate the total power budget (laser + TEC), footprint, and cost (in terms of assembly complexity).

(b) **One mode-locked laser (repetition rate 100 GHz) with comb spectrum**: What average power is needed to provide 1 mW per line after splitting into 32 channels? What is the pulse width required to span the full C-band?

(c) **One CW pump laser + Si₃N₄ microresonator DKS comb (FSR = 100 GHz)**: With ~5% efficiency per comb line (total conversion ~5% of 100 mW pump), how much power per line is available? Is this sufficient for direct driving of modulators?

(d) Rank the three architectures on: power efficiency, footprint, coherence between channels, and suitability for reconfigurable (dynamically tunable wavelength) operation. Which would you choose for (i) a lab prototype, (ii) a production data center accelerator, (iii) a quantum photonic processor requiring maximally coherent, indistinguishable photons?

---

## Laboratory and Computational Exercises

**Lab 4.1 — Semiconductor Laser Rate Equation Simulation**

Implement the coupled rate equations for a semiconductor laser and simulate the turn-on transient and small-signal modulation response.

(a) Use parameters from Exercise 4.2. Start below threshold ($I = 0.5 I_{th}$) and step to $I = 2 I_{th}$. Simulate $S(t)$ and $N(t)$ using `scipy.integrate.odeint` or `solve_ivp`. Plot the relaxation oscillations in the turn-on transient.

(b) For small-signal modulation $I(t) = I_{DC}(1 + m\cos(2\pi f t))$ with $m = 0.1$, compute the modulation response $|P(f)|/|P(0)|$ by Fourier transforming the output power $P(t) \propto S(t)$. Plot the modulation response vs. $f$ from DC to 50 GHz. Identify the relaxation oscillation frequency and −3 dB bandwidth.

(c) Add gain saturation: replace $g(N)$ with $g(N)/(1 + \varepsilon S)$ where $\varepsilon = 2 \times 10^{-17}$ cm³. How does this change the modulation bandwidth and relaxation oscillation peak?

(d) Simulate the RIN: add Langevin noise terms to the rate equations (white noise with variance $\sqrt{2D \cdot dt}$ at each time step, where $D$ is the noise diffusion coefficient). Compute the RIN spectrum from the power spectral density of $\delta S(t)/S$.

---

**Lab 4.2 — DFB Grating Design**

Design a quarter-wave-shifted DFB grating for single-mode operation at 1550 nm.

(a) For an InGaAsP waveguide with $n_{eff} = 3.2$ at 1550 nm, compute the grating period $\Lambda$ for first-order Bragg reflection.

(b) The coupling coefficient $\kappa$ for a corrugated grating with depth $h$ and duty cycle 50% is approximately $\kappa \approx |\Delta n|/\lambda$ where $\Delta n$ is the effective index modulation. For $\Delta n = 0.05$ (typical for surface grating), compute $\kappa$ and the required grating length for $\kappa L = 2$ (good single-mode discrimination).

(c) Compute the SMSR (side-mode suppression ratio) as a function of $\kappa L$ using the transfer matrix method for a QWS-DFB. The SMSR is approximately $\exp(2(\kappa L - 1))$ for $\kappa L > 1$.

(d) Analyze the effect of temperature tuning: at $d\lambda/dT = 0.1$ nm/°C, over what temperature range does the DFB maintain > 30 dB SMSR? What limits the tuning range?

---

**Lab 4.3 — Microresonator Comb Simulation (Lugiato-Lefever Equation)**

Simulate the Lugiato-Lefever equation (LLE) for microresonator frequency comb generation.

(a) Implement the LLE in normalized form using split-step Fourier method (FFT for dispersion, pointwise multiplication for nonlinearity and driving/loss):

$$\frac{\partial \psi}{\partial \tau} = -(1 + i\alpha)\psi + i|\psi|^2\psi - \frac{id_2}{2}\frac{\partial^2\psi}{\partial\theta^2} + F$$

where $\tau$ is normalized time, $\alpha$ is pump detuning, $d_2$ is normalized dispersion, $F$ is pump amplitude. Use Python with `numpy.fft`.

(b) Start with small noise, pump below threshold, and gradually increase the pump $F$. Observe the primary comb formation (first pair of sidebands appear), then secondary comb, and eventually (with anomalous $d_2 < 0$ and appropriate detuning) transition to a DKS state with a sech-shaped pulse circulating in the resonator.

(c) Plot the intracavity power spectrum at each stage. Compare the DKS spectrum to a sech² envelope.

(d) Compute the number of comb lines within 3 dB of the peak. How does this depend on $|d_2|$ (dispersion) and $|F|^2$ (pump power)?
