# 9.3.2 WDM System Design

## The System Margin Budget

A WDM optical transmission system must be designed with sufficient margin to ensure reliable operation throughout its 20–25 year lifetime, accounting for component aging, splices, connectors, and amplifier gain tilt. The system margin budget has the form:

$$\text{Received SNR} = \text{Launched power} - \text{Span loss} \times N_{\text{spans}} + \text{Amp gain} \times N_{\text{spans}} - \text{ASE accumulation}$$

In decibels:

$$\text{OSNR}_{\text{received}} = P_{\text{launch}} - \alpha L N_{\text{spans}} + G N_{\text{spans}} - 10\log_{10}(N_{\text{spans}} F_n \hbar\omega\Delta\nu)$$

where $G = \alpha L$ (gain equals span loss), and the last term is the accumulated ASE noise.

For a typical 80-km span with 0.2 dB/km loss:
- Span loss: 16 dB
- EDFA gain: 16 dB
- EDFA noise figure: 5 dB
- ASE power per span: $P_{\text{ASE}} = (F_n - 1)\hbar\omega G \Delta\nu \approx 10^{0.5-1} \times \hbar\omega\Delta\nu \times 10^{1.6}$

For $N = 20$ spans (1600 km), the accumulated OSNR:

$$\text{OSNR} \approx \frac{P_{\text{launch}}}{N \cdot F_n \cdot G \cdot \hbar\omega\Delta\nu} = \frac{P}{20 \times 3.16 \times 40 \times \hbar\omega \times 12.5 \times 10^9}$$

At $P = 0$ dBm (1 mW), $\lambda = 1550$ nm: $\hbar\omega = 1.28 \times 10^{-19}$ J. The OSNR (in 0.1 nm = 12.5 GHz) is approximately 20–22 dB for this scenario.

A typical OSNR requirement for DP-16QAM at BER = $10^{-3}$ (before FEC) is ~17–18 dB. The margin available is 2–5 dB — tight but sufficient.

## Power Equalization and Spectral Flatness

Different WDM channels experience slightly different gains through each EDFA (because the EDFA gain spectrum is not perfectly flat). After many cascaded EDFAs, the power difference between strongest and weakest channels can grow to 10–20 dB, making reliable reception impossible for the weakest channels.

Solutions:
1. **Dynamic gain equalization** (DGE): A programmable optical attenuator (wavelength selective switch, or spatial light modulator) after each EDFA equalizes the channel powers.
2. **Gain-flattening filters** (GFF): Passive filters that pre-distort the input spectrum to compensate for EDFA gain tilt; simpler but not adaptive.
3. **Raman amplification**: Adds counter-propagating Raman pump to equalize the effective noise figure across channels.

In modern systems, automatic DGE is standard in each ROADM node; the OSNR per channel is monitored by performance monitoring optics and the equalization adjusted in real time.

## Dispersion Management

Chromatic dispersion ($D = 17$ ps/(nm·km) for SMF-28) accumulates along the fiber. After $L$ km:

$$\Delta\tau = D \times L \times \Delta\lambda$$

For $\Delta\lambda = 0.3$ nm (spectral width of a 10 GBaud signal), $L = 80$ km:

$$\Delta\tau = 17 \times 80 \times 0.3 = 408 \text{ ps}$$

A symbol period at 10 GBaud is 100 ps; 408 ps of broadening causes severe ISI (inter-symbol interference).

In legacy direct-detection systems, this was compensated by alternating positive-dispersion fiber (SMF, $D = +17$) and negative-dispersion fiber (DCF, $D \approx -100$) to achieve near-zero net dispersion at each amplifier site. This "dispersion mapping" required careful balancing of fiber lengths.

In modern coherent systems, a digital FIR filter in the DSP chip compensates chromatic dispersion entirely in the electrical domain. A 1600-km link with SMF-28 has ~27,200 ps/nm of accumulated dispersion, requiring a filter with ~1700 taps at 64 GBaud — computationally expensive but feasible in 7-nm CMOS. The digital approach eliminates the need for DCF modules, saving significant insertion loss (DCF has ~0.5 dB/km loss compared to SMF's 0.2 dB/km) and simplifying system design.

---

## References

[1] Agrawal, G.P. (2012). *Fiber-Optic Communication Systems*, 5th ed. Wiley. [Chapters 7–9 cover WDM system design, EDFA cascades, and dispersion management.]

[2] Savory, S.J. (2010). "Digital coherent optical receivers: Algorithms and subsystems." *IEEE Journal of Selected Topics in Quantum Electronics*, 16(5), 1164–1179. [Digital chromatic dispersion compensation and carrier recovery algorithms.]
