# Chapter 5 Exercises: Photodetectors

---

## Mathematical Exercises

**Exercise 5.1 — Responsivity and Quantum Efficiency**

(a) An InGaAs p-i-n photodetector has QE $\eta = 0.85$ at 1550 nm. Compute the responsivity $\mathcal{R}$ in A/W.

(b) The same detector receives 1 mW of optical power. Compute the photocurrent. At what reverse bias current does the shot noise from the photocurrent equal the shot noise from a dark current of 50 nA?

(c) The absorption coefficient of InGaAs at 1550 nm is $\alpha = 7000$ cm$^{-1}$. If the surface reflectivity is reduced to $R = 0.005$ by an AR coating, what depletion region thickness $d$ achieves $\eta = 0.85$ with $\eta_{coll} = 1$?

(d) Compute the maximum responsivity of an ideal detector at 1310 nm and at 850 nm. Why is higher wavelength better for responsivity?

---

**Exercise 5.2 — Bandwidth and the Speed-Efficiency Tradeoff**

A Ge-on-Si waveguide photodetector has:
- Ge intrinsic layer thickness: $d = 1$ μm
- Junction area: $A = 5 \times 10$ μm² (a rectangular waveguide detector)
- Ge relative permittivity: $\varepsilon_r = 16$
- Ge hole saturation velocity: $v_{h,s} = 6 \times 10^6$ cm/s
- Series resistance: $R_s = 20$ Ω
- Load: $R_L = 50$ Ω

(a) Compute the transit-time-limited bandwidth $f_{tr} = 0.45 v_s/d$.

(b) Compute the junction capacitance $C_j = \varepsilon_r \varepsilon_0 A/d$ and the RC-limited bandwidth $f_{RC} = 1/(2\pi(R_s+R_L)C_j)$.

(c) Compute the combined bandwidth $f_{3\text{dB}}$.

(d) If this detector is connected to a TIA with $R_F = 500$ Ω instead of 50 Ω, how does the bandwidth change? What happens to the shot-noise-limited SNR at 1 mW input?

---

**Exercise 5.3 — Shot Noise and Signal-to-Noise Ratio**

(a) A detector with $\mathcal{R} = 1$ A/W receives $P_{in} = 100$ μW of signal. The receiver bandwidth is $B = 5$ GHz. Compute the RMS shot noise current and compare to the RMS signal current.

(b) The TIA has feedback resistance $R_F = 200$ Ω at $T = 300$ K. Compute the RMS Johnson noise current in bandwidth $B$.

(c) The laser has RIN = −145 dBc/Hz. Compute the RMS RIN noise current.

(d) Add all noise sources in quadrature (they are uncorrelated), compute the total SNR, and convert to ENOB. Which noise source dominates?

---

**Exercise 5.4 — APD Sensitivity**

An InGaAs APD at 1550 nm has: $\mathcal{R}_0 = 0.9$ A/W (before gain), ionization ratio $k = 0.4$, dark current $I_d = 5$ nA, operating gain $M = 10$.

(a) Compute the excess noise factor $F(M) = kM + (1-k)(2-1/M)$.

(b) The total noise variance of the APD receiver (with TIA $R_F = 500$ Ω, $B = 1$ GHz):

$$\langle i^2_{noise}\rangle = 2eM^2F(M)(\mathcal{R}_0 P_{in} + I_d)B + \frac{4k_BTB}{R_F}$$

At $P_{in} = 10$ μW, compute the total noise and the SNR. Compare to a p-i-n detector with the same $\mathcal{R}_0$ and $R_F$.

(c) Find the optimum gain $M_{opt}$ that maximizes SNR. (Differentiate SNR with respect to $M$ and set to zero.) For this optimum gain, what is the sensitivity improvement over the p-i-n detector?

(d) As a rough rule, APDs improve receiver sensitivity by $10\log_{10}(\mathcal{R}_{APD}/\mathcal{R}_{pin}) = 10\log_{10}(M/\sqrt{F(M)})$. Compute this for $M = 10$ and compare to your full calculation.

---

**Exercise 5.5 — Coherent Detection**

A balanced homodyne receiver uses a 50:50 beamsplitter, two ideal p-i-n detectors ($\eta = 1$, $I_d = 0$), and a local oscillator power $P_{LO} = 1$ mW. The signal power is $P_s = 1$ μW.

(a) Compute the signal current $I_{diff} = 2\mathcal{R}\sqrt{P_sP_{LO}}$. Compare to the direct detection photocurrent $I_{direct} = \mathcal{R}P_s$.

(b) Compute the shot noise from the LO: $\langle i^2_{shot,LO}\rangle = 4eI_{LO}B$ with $B = 1$ GHz. Note the factor of 4 (two detectors, each with LO shot noise).

(c) Compute the coherent detection SNR: $\text{SNR}_{coh} = I_{diff}^2/\langle i^2_{shot,LO}\rangle$. Compare to the shot-noise-limited direct detection SNR.

(d) A laser RIN of −145 dBc/Hz contributes common-mode noise $\delta P_{RIN} = \sqrt{\text{RIN} \cdot B} \cdot P_{LO}$ to both detectors. After balanced subtraction, this cancels (in principle). By how many dB does perfect balancing suppress RIN noise compared to direct detection at the same $P_{in} = P_{LO}$?

---

## Conceptual Exercises

**Exercise 5.6 — Why Silicon Cannot Detect Telecom Light**

(a) Silicon has a bandgap of 1.12 eV. Compute the long-wavelength absorption edge $\lambda_c = hc/E_g$ for silicon. At 1550 nm, silicon is transparent. What is the photon energy at 1550 nm, and how does it compare to the silicon bandgap?

(b) If you operate a photonic computing chip at 1064 nm instead of 1550 nm, silicon can be used as the detector. What changes in the chip design? What happens to the waveguide loss (hint: silicon absorbs strongly above the two-photon absorption edge at ~2200 nm and has free-carrier absorption above ~1100 nm)?

(c) Germanium has $E_g = 0.67$ eV (direct), 0.80 eV (indirect). At 1550 nm, which transition is relevant for absorption? How does the absorption coefficient of Ge compare to InGaAs at 1550 nm, and what does this imply for the required detector length?

(d) Si₁₋ₓGeₓ alloys can be tuned between Si and Ge bandgaps. For a target absorption wavelength of 2 μm (for mid-infrared sensing), what Ge fraction is needed? What challenges arise in integrating high-Ge-content SiGe on SOI platforms?

---

**Exercise 5.7 — The Quantum Limit and Photonic Computing Precision**

The shot-noise-limited SNR for direct detection is $\text{SNR} = \mathcal{R}P_{in}/(2eB)$.

(a) For an MZI matrix processor with matrix size $N = 64$, the input optical power per channel is $P_{in}/N$ (the total power is divided across $N$ channels). If total source power is $P_{total} = 100$ mW and $B = 1$ GHz: compute the per-channel SNR and ENOB at the shot-noise limit.

(b) A photonic neural network layer requires ENOB of 8 bits for inference at ImageNet accuracy comparable to a 32-bit floating-point software model. What minimum optical power per channel is needed (assuming shot-noise-limited operation at $B = 100$ MHz)?

(c) The state of the art for analog photonic processors reports ENOB of 5–6 bits in practice. Identify three physical mechanisms (beyond shot noise) that limit the ENOB to this range.

(d) Propose two techniques that could improve ENOB from 6 bits toward 8 bits in a silicon photonic matrix processor. For each, identify the tradeoff.

---

**Exercise 5.8 — SNSPD and Quantum Computing**

A quantum photonic boson sampling experiment uses 20 SNSPD detectors at 1550 nm.

(a) With SNSPD system detection efficiency $\eta_{SDE} = 95\%$ per detector, and input of $N$-photon Fock states (all photons must be detected), what is the probability of detecting all 20 photons? Compare to $\eta_{SDE} = 20\%$ (InGaAs SPAD).

(b) The experiment aims to demonstrate quantum computational advantage at $N = 50$ photons, requiring $\sim10^6$ successful detection events. With SNSPD at $\eta = 95\%$ and a photon repetition rate of $f_{rep} = 100$ kHz (source-limited), how long would data collection take? With $\eta = 20\%$ SPADs?

(c) Dark counts at rate $r_{dc} = 10$ cts/s (SNSPD) add false detection events. If the detector gate window is 10 ns (matched to the photon pulse), what is the dark count probability per gate? For a 50-photon boson sampling experiment, how does this affect the fidelity of the output distribution?

(d) SNSPDs require cooling to ~1 K (dilution refrigerator) consuming ~1 kW of electrical power. If the quantum photonic processor chip operates at room temperature (or at 4 K if photonic components are superconducting), describe the packaging challenge of getting > 1000 optical fibers from room temperature to 1 K with acceptable thermal isolation and transmission.

---

## Laboratory and Computational Exercises

**Lab 5.1 — Receiver Noise Model**

Implement a complete optical receiver noise model in Python.

(a) Write a function `receiver_snr(P_in, R, eta, I_dark, R_F, T, B, RIN_dBc_Hz)` that computes the total SNR and ENOB for a direct detection receiver with TIA.

(b) Plot SNR (dB) vs. $P_{in}$ from −60 dBm to +10 dBm for $B = 1$ GHz, $R_F = 1$ kΩ, $T = 300$ K, RIN = −150 dBc/Hz. Mark the transition from thermal-limited to shot-limited to RIN-limited regimes.

(c) Repeat for $B = 10$ MHz and $B = 10$ GHz. How does bandwidth affect the three regimes?

(d) For an ENOB = 7 bits requirement, find the minimum signal power as a function of bandwidth $B$ (from 1 MHz to 10 GHz). Plot the "ENOB budget" curve.

---

**Lab 5.2 — Ge-on-Si Detector Design**

Design a waveguide-integrated Ge-on-Si photodetector for a silicon photonic processor.

(a) Using the absorption data for Ge at 1550 nm ($\alpha = 3000$ cm$^{-1}$ for unstrained Ge-on-Si), compute the detector length required for 90% absorption as a function of optical mode overlap factor $\Gamma_{Ge}$ (fraction of power in the Ge region, typically 0.3–0.6).

(b) For a $5 \times L$ μm cross-section Ge detector with length $L$ from part (a), compute the junction capacitance $C_j$ as a function of bias voltage $V_R$ using $C_j = \varepsilon_0 \varepsilon_r A / d(V_R)$ where $d(V_R) = \sqrt{2\varepsilon V_R/(qN_D)}$ for a one-sided abrupt junction with doping $N_D = 10^{17}$ cm$^{-3}$.

(c) Compute the transit-time-limited bandwidth and RC-limited bandwidth (with 50 Ω) as functions of depletion width $d$ at −2 V bias. Plot both and identify the combined bandwidth.

(d) The dark current scales with the depletion volume: $I_d \approx q n_i V_{dep}/\tau$, where $n_i = 2.4\times10^{13}$ cm$^{-3}$ is the intrinsic carrier density of Ge and $\tau = 1$ ns is the carrier lifetime. Compute $I_d$ for your design and assess whether it limits SNR at 1 μW signal power.
