# Unit IV Problem Set: Information Theory and Optical Communications

*Problems covering information theory fundamentals and their application to optical communication systems. Chapters 9–10.*

---

## Chapter 9: Information Theory and Optical Channels

**Problem 9.1** [Easy]
Shannon entropy:

(a) A source emits symbols $\{A, B, C, D\}$ with probabilities $\{0.5, 0.25, 0.125, 0.125\}$. Compute the entropy $H = -\sum p_i \log_2 p_i$ in bits per symbol.

(b) A Huffman code assigns A=0, B=10, C=110, D=111. Compute the average code length. How close is it to the Shannon entropy?

(c) A binary channel transmits bits with crossover probability $p = 0.01$ (binary symmetric channel). Compute the channel capacity $C = 1 - H_b(p) = 1 + p\log_2 p + (1-p)\log_2(1-p)$ in bits per channel use.

(d) What is the capacity in bits/s for a channel with $p = 0.01$ and symbol rate 10 Gbaud?

**Problem 9.2** [Easy]
The AWGN (Additive White Gaussian Noise) channel capacity:

$$C = B\log_2\!\left(1 + \frac{S}{N}\right) \text{ bits/s}$$

(a) A 50 GHz optical channel with SNR = 20 dB: compute $C$.

(b) A 100G coherent optical system uses 16-QAM (4 bits per symbol) with polarization multiplexing (2 polarizations) at 32 Gbaud. What is the spectral efficiency (bits/s/Hz)?

(c) At what SNR does 16-QAM achieve $C > 4$ bits/symbol/polarization? Is 32 Gbaud/16-QAM/polmux operating near the Shannon limit for a 50 GHz channel?

(d) The nonlinear Shannon limit for optical fiber accounts for Kerr nonlinearity. For a 1000 km link, the optimal launch power is $P_\text{opt} \approx (8B/3\gamma L)^{1/3}$ (order-of-magnitude estimate for a simplified model). For $\gamma = 1.3$ W⁻¹km⁻¹, $L = 1000$ km, $B = 50$ GHz: compute $P_\text{opt}$.

**Problem 9.3** [Medium]
Optical channel noise:

A WDM system carries 80 channels at 50 GHz spacing, each at power $P = 0$ dBm (1 mW). Erbium-doped fiber amplifiers (EDFAs) with gain $G = 20$ dB and noise figure NF = 6 dB are spaced 80 km apart along a 1600 km link (20 amplifiers).

(a) EDFA amplified spontaneous emission (ASE) power spectral density: $S_\text{ASE} = n_{sp}(G-1)h\nu$ per polarization mode per unit bandwidth. For NF $= 2n_{sp}(G-1)/G \approx 2n_{sp}$ in the high-gain limit: find $n_{sp}$ and $S_\text{ASE}$.

(b) After 20 amplifiers, total ASE power in bandwidth $B = 50$ GHz: $P_\text{ASE,total} = 20 \cdot 2S_\text{ASE} \cdot B$. Compute $P_\text{ASE}$ in dBm.

(c) Optical SNR (OSNR): $\text{OSNR} = P/(P_\text{ASE,total})$. Compute in dB.

(d) For coherent detection of 16-QAM, required OSNR per 32 Gbaud channel for BER $= 10^{-3}$: approximately 18 dB (0.1 nm reference bandwidth). Is there margin?

**Problem 9.4** [Medium]
Chromatic dispersion in WDM systems:

SMF-28 has dispersion $D = 17$ ps/(nm·km) at 1550 nm and dispersion slope $S = 0.057$ ps/(nm²·km).

(a) Over 1000 km, by how much does the 1530 nm channel (edge of C-band) differ in group delay from the 1565 nm channel (other edge)?

(b) A 10 Gbps NRZ signal has bit duration 100 ps. The dispersion limit (1 dB power penalty) is when $D\cdot L\cdot\Delta\lambda = 0.4$ ns where $\Delta\lambda$ is the spectral width ($\approx 0.1$ nm for NRZ at 10 Gbps). Compute the dispersion-limited distance.

(c) Dispersion-compensating fiber (DCF) has $D = -100$ ps/(nm·km). How long a DCF module is needed to compensate the dispersion of 80 km SMF at 1550 nm? What is the total insertion loss if DCF loss is 0.5 dB/km?

(d) In coherent systems, dispersion is compensated digitally using FIR filters. For 100 Gbps PM-QPSK at 32 Gbaud (symbol period 31.25 ps) after 1000 km with $D = 17$ ps/(nm·km): how many DSP taps are needed? ($N_\text{taps} \approx D\cdot L\cdot\Delta\lambda_\text{symbol}/T_s^2$, approximately.)

**Problem 9.5** [Hard]
*Hint: Use the Manakov equations — the fiber NLSE generalized to two polarizations. The nonlinear coefficient enters as $\gamma P$ (phase shift per unit length per unit power).*

Nonlinear phase shift (self-phase modulation, SPM): A Gaussian pulse $A(0,t) = P_0^{1/2}\exp(-t^2/(2T_0^2))$ propagates in an SMF fiber of length $L$ with nonlinear coefficient $\gamma = 1.3$ W⁻¹km⁻¹ and anomalous dispersion $\beta_2 = -21$ ps²/km.

(a) The nonlinear length $L_\text{NL} = 1/(\gamma P_0)$ and dispersion length $L_D = T_0^2/|\beta_2|$. For $P_0 = 10$ mW, $T_0 = 20$ ps: compute $L_\text{NL}$ and $L_D$.

(b) In the absence of dispersion ($L \ll L_D$), SPM causes a phase shift $\phi_\text{NL}(t) = \gamma P_0 L_\text{eff} e^{-t^2/T_0^2}$ where $L_\text{eff} = (1-e^{-\alpha L})/\alpha$. The instantaneous frequency chirp $\delta\omega(t) = -d\phi_\text{NL}/dt$. Find the maximum frequency deviation $\delta\omega_\text{max}$.

(c) When $L_\text{NL} = L_D$ and the dispersion is anomalous ($\beta_2 < 0$), the pulse forms a **soliton** with no pulse broadening. Write the soliton condition $N = \sqrt{L_D/L_\text{NL}} = 1$ and find the required peak power $P_0$ for soliton propagation at $T_0 = 20$ ps.

**Problem 9.6** [Hard]
*Hint: The capacity of the AWGN optical channel is the Holevo capacity; for coherent detection it reduces to the classical formula.*

Quantum limits on optical communication: a coherent state $|\alpha\rangle$ carries $\bar{n} = |\alpha|^2$ mean photons. For on-off-keying (OOK) with signal state $|\alpha\rangle$ and vacuum state $|0\rangle$:

(a) The quantum bit error probability for homodyne detection is $\text{BEP} = \frac{1}{2}\text{erfc}(\sqrt{\bar{n}})$. For a target BEP $= 10^{-9}$: find the minimum required $\bar{n}$ (photons per bit).

(b) The Helstrom bound (optimal quantum measurement) gives $\text{BEP}_\text{min} = \frac{1}{2}(1 - \sqrt{1-e^{-4\bar{n}}})$. For the same $\bar{n}$, is homodyne detection optimal?

(c) The photocount distribution for a coherent state $|\alpha\rangle$ is Poissonian: $P(k) = e^{-\bar{n}}\bar{n}^k/k!$. For direct detection with a photon-number-resolving detector and the discrimination strategy "output 0 if no photons, output 1 if $\geq 1$ photon" (Kennedy receiver): find the BEP and compare to homodyne.

---

## Chapter 10: Optical Interconnects

**Problem 10.1** [Easy]
Bandwidth-distance product: a multimode fiber (MMF) with 50 μm core has modal bandwidth $BW_\text{modal} = 2000$ MHz·km (OM4 fiber).

(a) What is the maximum bandwidth for a 100 m link? 300 m link?

(b) The chromatic dispersion adds in quadrature: total $\Delta\tau = \sqrt{\Delta\tau_\text{modal}^2 + \Delta\tau_\text{chromatic}^2}$. For LED source ($\Delta\lambda = 15$ nm) at 850 nm with $D_\text{material} = -80$ ps/(nm·km) over 100 m: is modal or chromatic dispersion dominant?

(c) A VCSEL (vertical-cavity surface-emitting laser) source with $\Delta\lambda = 0.3$ nm replaces the LED. Now which dominates?

**Problem 10.2** [Medium]
Silicon photonic link budget for a 2 m board-level optical interconnect:

| Component | Loss |
|---|---|
| Laser (VCSEL, $P_\text{out} = 0$ dBm) | — |
| Fiber-chip grating coupler | 2.5 dB per coupler |
| Silicon waveguide, 5 cm long, 2 dB/cm | 10 dB |
| MMI beamsplitter (if used) | 3 dB |
| Photodetector ($\mathcal{R} = 0.85$ A/W, NEP $= 10$ pW/$\sqrt{\text{Hz}}$, $B = 25$ GHz) | — |

(a) Compute the received power at the detector.

(b) Compute the detector SNR for the received power.

(c) What is the minimum required received power for BER $= 10^{-12}$ (use the formula from Problem 5.4)?

(d) Is there a power margin? If not, identify which component loss dominates and propose one specific improvement.

**Problem 10.3** [Medium]
Dense WDM interconnects: an on-chip WDM system uses 8 wavelength channels spaced by 100 GHz ($\approx$ 0.8 nm at 1550 nm), each modulated at 25 Gbps. Wavelength (de)multiplexers are arrayed waveguide gratings (AWGs) with 0.5 dB per channel insertion loss and $-30$ dB crosstalk.

(a) What is the aggregate bandwidth?

(b) The crosstalk from adjacent channels causes a power penalty. For crosstalk level $\epsilon = -30$ dB: the power penalty $\approx -10\log_{10}(1-10^{\epsilon/10})$ dB. Compute the penalty.

(c) Temperature shifts the AWG passband by $0.1$ nm/K. For a $\pm 10$ K temperature range, does the channel shift cause crosstalk with adjacent channels? (Channel spacing 0.8 nm, passband FWHM 0.3 nm.)

(d) What is the total chip area for the AWG if the waveguide pitch is 5 μm, and the AWG design has 100 arrayed waveguides of length 1 cm each?
