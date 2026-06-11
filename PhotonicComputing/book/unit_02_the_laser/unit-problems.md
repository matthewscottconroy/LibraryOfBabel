# Unit II Problem Set: The Laser

*Problems covering laser physics (Chapter 4) and photodetectors (Chapter 5). Ranges from fundamental laser dynamics to practical detector noise analysis.*

---

## Chapter 4: Laser Physics

**Problem 4.1** [Easy]
A HeNe laser ($\lambda = 632.8$ nm) has a gain medium of length $L_g = 20$ cm inside a cavity of total length $L = 30$ cm. The mirror reflectivities are $R_1 = 1.0$ and $R_2 = 0.98$. The gain medium has absorption loss coefficient $\alpha_\text{int} = 0.005$ cm⁻¹.

(a) Write the round-trip gain condition (threshold): the gain must equal all round-trip losses.

(b) Compute the threshold gain coefficient $g_\text{thresh}$ (per unit length).

(c) If the gain cross-section is $\sigma_{21} = 3\times10^{-13}$ cm², what population inversion density $\Delta N = N_2 - N_1$ is needed at threshold?

(d) What is the free spectral range $\Delta\nu_\text{FSR} = c/(2L)$ of this cavity? How many longitudinal modes can oscillate if the gain bandwidth is $1.5$ GHz?

**Problem 4.2** [Easy]
Laser rate equations (single-mode): Let $N$ be the population inversion density and $I$ be the intracavity photon density.

$$\frac{dN}{dt} = R_p - \frac{N}{\tau_s} - \sigma c\, NI$$

$$\frac{dI}{dt} = \sigma c\, NI - \frac{I}{\tau_p} + \frac{\beta N}{\tau_s}$$

where $R_p$ is the pump rate, $\tau_s$ is the spontaneous emission lifetime, $\tau_p$ is the photon lifetime, $\sigma$ is the cross-section, and $\beta$ is the spontaneous emission factor.

(a) Find the steady-state threshold inversion $N_\text{th}$ by setting $dI/dt = 0$ (ignoring the $\beta$ term).

(b) Above threshold, the steady-state photon density $I_\text{ss}$ increases linearly with pump rate. Show this by finding $I_\text{ss}(R_p)$.

(c) Why is the $\beta N/\tau_s$ term important below threshold but negligible well above threshold?

**Problem 4.3** [Medium]
Relaxation oscillations: linearize the laser rate equations around the steady-state operating point $(N_0, I_0)$. Let $N = N_0 + \delta N$, $I = I_0 + \delta I$.

(a) Derive the linearized equations for $\delta N$ and $\delta I$.

(b) Show that the perturbations oscillate at the relaxation oscillation frequency:
$$\Omega_R \approx \sqrt{\frac{I_0}{\tau_p N_\text{th}\tau_s}}$$

(c) What is the damping rate of these oscillations?

(d) For a diode laser with $\tau_p = 2$ ps, $\tau_s = 2$ ns, $I_0 = 10 I_\text{th}$ (where $I_0/N_\text{th}\tau_p$ is the stimulated emission rate): compute $\Omega_R/(2\pi)$.

**Problem 4.4** [Medium]
Semiconductor laser (diode laser):

(a) A quantum well laser at $\lambda = 980$ nm has gain bandwidth $\Delta\lambda_g = 40$ nm. Its active region length is $L = 300$ μm with reflective facets ($R = 0.32$ each, from Fresnel reflection at GaAs/air: $R = [(n-1)/(n+1)]^2$ with $n = 3.5$). Compute the number of cavity longitudinal modes within the gain bandwidth.

(b) Mode competition means only one or a few modes lase in practice. The threshold gain difference between modes is $\Delta g \approx 0.1$ cm⁻¹. For a gain material with gain spectrum $g(\lambda) = g_0\exp[-(\lambda-\lambda_0)^2/(2\sigma_\lambda^2)]$ with $\sigma_\lambda = 15$ nm: how far from the gain peak can a mode be before its gain falls below threshold by $\Delta g$?

(c) Distributed feedback (DFB) lasers have a Bragg grating etched into the waveguide. The Bragg condition is $\Lambda = \lambda/(2n_\text{eff})$. For $\lambda = 1550$ nm, $n_\text{eff} = 3.2$: find the grating period $\Lambda$.

(d) The DFB grating provides frequency-selective feedback that suppresses side modes. The side-mode suppression ratio (SMSR) is typically $> 30$ dB. Explain qualitatively why the grating selects a single longitudinal mode.

**Problem 4.5** [Medium]
Laser linewidth: the Schawlow-Townes linewidth formula for a laser:

$$\Delta\nu_\text{ST} = \frac{\pi h\nu (\Delta\nu_c)^2 n_\text{sp}}{P_\text{out}}$$

where $\Delta\nu_c = \nu/(2\mathcal{F})$ is the cold-cavity linewidth ($\mathcal{F}$ = finesse), $n_\text{sp}$ is the spontaneous emission factor ($\approx 1$–2 above threshold), and $P_\text{out}$ is the output power.

(a) For a HeNe laser: $\lambda = 633$ nm, cavity length $L = 30$ cm, mirror reflectivities $R_1 = 1$, $R_2 = 0.99$, $P_\text{out} = 1$ mW: compute $\Delta\nu_\text{ST}$.

(b) Why do semiconductor diode lasers have linewidths 100× to 1000× larger than the Schawlow-Townes prediction? (The Henry linewidth enhancement factor $\alpha_H$ is relevant.)

(c) Estimate the coherence length $L_c = c/(\pi\Delta\nu)$ for the HeNe laser and a typical diode laser with $\Delta\nu = 100$ MHz.

**Problem 4.6** [Hard]
*Hint: Use the transfer matrix for a coupled-cavity system (Section on coupled resonators). The eigenfrequencies shift when the cavities are coupled.*

Two microring resonators (radii $R_1 = 10$ μm, $R_2 = 10.05$ μm) are coupled with rate $\mu$. They are pumped to achieve gain (population inversion above the lasing threshold).

(a) Without coupling, the resonant frequencies are $\omega_1 = mc/(n_\text{eff}\cdot 2\pi R_1)$ and $\omega_2$. Compute $\omega_2 - \omega_1$ (assuming $n_\text{eff} = 2.5$, mode number $m$ such that $\lambda \approx 1550$ nm).

(b) With coupling $\mu$, the normal mode splitting is $2\sqrt{\mu^2 + (\delta\omega/2)^2}$ where $\delta\omega = \omega_2 - \omega_1$. For $\mu = \delta\omega/2$: what is the splitting?

(c) In a coupled-cavity laser, which normal mode lases preferentially? (Consider the spatial overlap of each normal mode with the gain region.)

---

## Chapter 5: Photodetectors

**Problem 5.1** [Easy]
A PIN photodiode detects 1550 nm light. Its quantum efficiency is $\eta = 0.85$ and its dark current is $I_d = 0.5$ nA.

(a) What is the responsivity $\mathcal{R} = \eta e\lambda/(hc)$ in A/W?

(b) At what power level does the photocurrent equal the dark current? This is the minimum detectable signal (in the absence of other noise).

(c) With a transimpedance amplifier gain $G = 10^4$ V/A and noise equivalent power (NEP) $= \sqrt{2eI_d}/(\mathcal{R}\sqrt{B})$ for bandwidth $B = 10$ MHz: compute the NEP in W/$\sqrt{\text{Hz}}$.

**Problem 5.2** [Medium]
Shot noise vs. thermal noise: A photodetector has photocurrent $I_\text{ph}$, dark current $I_d$, load resistance $R_L = 50\,\Omega$, temperature $T = 300$ K, and bandwidth $B$.

(a) Write expressions for: (i) shot noise current $I_s = \sqrt{2e(I_\text{ph}+I_d)B}$, (ii) thermal (Johnson) noise $I_t = \sqrt{4k_BT B/R_L}$.

(b) At what photocurrent does shot noise equal thermal noise?

(c) For $R_L = 50\,\Omega$, $T = 300$ K, $B = 1$ GHz: compute the thermal noise floor in dBm.

(d) For coherent detection (homodyne), the local oscillator power $P_\text{LO}$ amplifies the signal shot noise above the thermal noise floor. Show that coherent detection is shot-noise limited when $I_\text{LO} = \eta e P_\text{LO}/(h\nu) \gg I_t^2/(2eB)$. For $T = 300$ K, $B = 10$ GHz, $\eta = 0.8$, $\lambda = 1550$ nm: find the minimum $P_\text{LO}$ for shot-noise-limited detection.

**Problem 5.3** [Medium]
Avalanche photodiode (APD): an APD has multiplication factor $M = 10$, excess noise factor $F(M) = M^{0.5}$ (for silicon), quantum efficiency $\eta = 0.7$ at 850 nm, dark current $I_d = 10$ nA, and bandwidth $B = 1$ GHz.

(a) The signal current is $I_\text{sig} = M\eta eP/(h\nu)$. Compute $I_\text{sig}$ for $P = 1$ nW.

(b) The total noise current (rms): $\sigma^2 = 2eM^2 F(M)(I_\text{ph}/M + I_d)B + 4k_BTB/R_L$.

(c) Find the SNR for $P = 1$ nW. At what $M$ is SNR maximized? (Differentiate SNR with respect to $M$ and set to zero.)

**Problem 5.4** [Hard]
*Hint: The BER is related to the $Q$-factor: $\text{BER} = \frac{1}{2}\text{erfc}(Q/\sqrt{2})$ where $Q = (I_1 - I_0)/(\sigma_1 + \sigma_0)$.*

Bit error rate (BER) analysis for on-off-keying (OOK) at 10 Gb/s:

- Mark (1 bit): $P_1 = 1$ mW, mean photocurrent $I_1 = \mathcal{R}P_1$
- Space (0 bit): $P_0 = 0.1$ mW, mean photocurrent $I_0 = \mathcal{R}P_0$
- Quantum efficiency $\eta = 0.8$, $\lambda = 1550$ nm
- Shot noise limited, $B = 7.5$ GHz (matched filter)

(a) Compute $I_1$, $I_0$, and the noise standard deviations $\sigma_1 = \sqrt{2eI_1B}$, $\sigma_0 = \sqrt{2eI_0B}$.

(b) Compute the $Q$-factor.

(c) Estimate the BER (use $\text{erfc}(x) \approx e^{-x^2}/(\sqrt{\pi}x)$ for large $x$).

(d) FEC (forward error correction) codes allow operation at BER $= 10^{-3}$ before coding, achieving $< 10^{-15}$ after coding. What $Q$-factor is needed for pre-FEC BER $= 10^{-3}$?
