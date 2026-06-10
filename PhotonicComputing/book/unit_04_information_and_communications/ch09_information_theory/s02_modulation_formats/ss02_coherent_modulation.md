# 9.2.2 Coherent Modulation: BPSK, QPSK, and QAM

## Why Coherent?

Direct-detection formats (OOK, PAM4) measure the intensity of the received optical field — the square of the amplitude. They discard phase information. Coherent detection, by contrast, measures both the amplitude and phase of the field by mixing it with a local oscillator (LO) laser before detection.

The rewards are substantial:
1. **Full use of the complex plane**: Both I (in-phase) and Q (quadrature) components carry independent information, doubling capacity per bandwidth.
2. **Better receiver sensitivity**: Coherent detection fundamentally achieves shot-noise-limited sensitivity regardless of thermal noise, because the LO power amplifies the signal above the thermal noise floor.
3. **Polarization multiplexing**: Both polarization states can be independently modulated and separated at the receiver with a polarization-diversity hybrid, doubling capacity again.

The price: a narrow-linewidth LO laser is needed; frequency and phase locking (or digital carrier recovery) is required; the transmitter and receiver are more complex and expensive.

## Coherent Detection Principle

In a coherent receiver, the signal $E_s = A_s e^{i\phi_s}$ is mixed with LO $E_{\text{LO}} = A_{\text{LO}}e^{i\phi_{\text{LO}}}$ in a 90° optical hybrid (Section 5.4.2). The four output photodiodes measure:

$$I_I \propto A_s A_{\text{LO}}\cos(\phi_s - \phi_{\text{LO}})$$
$$I_Q \propto A_s A_{\text{LO}}\sin(\phi_s - \phi_{\text{LO}})$$

These directly measure the real and imaginary parts of the received complex field $E_s$ (relative to the LO phase reference). For high LO power ($A_{\text{LO}} \gg A_s$), the thermal noise is overwhelmed and the SNR is:

$$\text{SNR}_{\text{coh}} = \frac{(A_s A_{\text{LO}})^2}{\sigma^2_{\text{shot}}} = \frac{\mathcal{R}P_s}{\hbar\omega B}$$

This is the shot-noise-limited SNR: $\text{SNR}_{\text{coh}} = \bar{n}_s$ (mean signal photons per bit). This is 3 dB better than direct-detection OOK at the same received power.

## BPSK and QPSK

**BPSK** (binary PSK): Two symbols, phases $0$ and $\pi$. The constellation consists of two points on the real axis. Each symbol carries 1 bit. Minimum Euclidean distance between symbols $d = 2\sqrt{P}$.

BER for coherent BPSK with shot noise:

$$\text{BER}_{\text{BPSK}} = \frac{1}{2}\text{erfc}\left(\sqrt{\bar{n}_s}\right)$$

For BER = $10^{-3}$ (pre-FEC threshold): $\bar{n}_s \approx 7.5$ photons/bit. This is vastly more efficient than OOK's 43 photons/bit at the same BER — the advantage of coherent detection.

**QPSK** (quadrature PSK): Four symbols at phases $0, \pi/2, \pi, 3\pi/2$. The constellation consists of four points on a circle. Each symbol carries 2 bits. The minimum distance is $d = \sqrt{2P}$ (smaller than BPSK at the same power).

For DP-QPSK (dual-polarization QPSK): 2 bits/symbol × 2 polarizations = 4 bits per symbol. At 32 GBaud: 128 Gbps per channel. This was the first major coherent format deployed in long-haul networks (~2010).

BER for QPSK:
$$\text{BER}_{\text{QPSK}} \approx \frac{1}{2}\text{erfc}\left(\sqrt{\bar{n}_s/2}\right)$$

The factor of 2 penalty compared to BPSK comes from the reduced symbol spacing; DP-QPSK requires ~6 dB more SNR than DP-BPSK for the same BER, but carries twice the bits.

## QAM Constellations

Higher-order QAM uses both amplitude and phase to encode more bits per symbol. The $M$-QAM constellation has $M = 2^k$ points arranged in a rectangular grid in the complex plane, with $k = \log_2 M$ bits per symbol.

The minimum Euclidean distance for $M$-QAM with average power $P$:

$$d_{\min} = \sqrt{\frac{6P}{M-1}} \approx \sqrt{\frac{6P}{M}}$$

for large $M$. The BER approximation for $M$-QAM (Gray coding):

$$\text{BER}_M \approx \frac{4}{\log_2 M}\left(1 - \frac{1}{\sqrt{M}}\right)\frac{1}{2}\text{erfc}\left(\sqrt{\frac{3\bar{n}_s}{2(M-1)}}\right)$$

For DP-16QAM ($M = 16$, 4 bits/symbol, 8 bits per dual-pol symbol): at 32 GBaud → 256 Gbps per channel (800 Gbps with 3 channels per 100 GHz grid). This is the standard for 400G and 600G long-haul deployed since ~2018.

Required SNR for BER = $10^{-3}$ (pre-FEC) for common formats:

| Format | Bits/symbol | Required SNR (dB) | Spectral eff. (bits/s/Hz) |
|--------|-------------|-------------------|---------------------------|
| OOK (direct) | 1 | 14 dB | 1.0 |
| PAM4 (direct) | 2 | 20 dB | 2.0 |
| DP-BPSK | 2 | 7 dB | 2.0 |
| DP-QPSK | 4 | 10 dB | 4.0 |
| DP-16QAM | 8 | 16 dB | 8.0 |
| DP-64QAM | 12 | 22 dB | 12.0 |
| Shannon limit | — | — | $\log_2(1+\text{SNR})$ |

At SNR = 16 dB (40 linear): Shannon limit gives $\log_2(41) \approx 5.4$ bits/s/Hz. DP-16QAM achieves 8 bits/s/Hz — exceeding the Shannon limit per dimension because it uses dual-polarization (2 spatial modes). Per spatial mode, DP-16QAM achieves 4 bits/s/Hz at 16 dB SNR; the Shannon limit per mode is 5.4 bits/s/Hz — so DP-16QAM is within ~1.4 bits/s/Hz of the Shannon limit.

## Digital Signal Processing in Coherent Systems

Modern coherent receivers use powerful DSP to perform:
- **Chromatic dispersion compensation**: Linear FIR filter correcting for $D \cdot L$ dispersion.
- **Carrier phase recovery**: PLL-like algorithm tracking the LO-signal frequency offset and phase drift.
- **Polarization demux**: Adaptive MIMO equalizer separating the two polarizations.
- **Nonlinear compensation**: Back-propagation algorithms partially compensating SPM/XPM.

The DSP chip in a coherent transceiver is one of the most complex ASICs in commercial production — consuming 3–10 W in a 7-nm CMOS process for 400G operation. This electronic complexity is what makes coherent systems expensive for short-reach links (< 80 km), where direct-detection PAM4 is preferred despite its lower spectral efficiency.

---

## References

[1] Ip, E., Lau, A.P.T., Barros, D.J.F., & Kahn, J.M. (2008). "Coherent detection in optical fiber systems." *Optics Express*, 16(2), 753–791. [Comprehensive treatment of coherent detection, 90° hybrids, and DSP for coherent receivers.]

[2] Savory, S.J. (2010). "Digital coherent optical receivers: Algorithms and subsystems." *IEEE Journal of Selected Topics in Quantum Electronics*, 16(5), 1164–1179. [The DSP algorithms for chromatic dispersion compensation, carrier recovery, and polarization demux.]
