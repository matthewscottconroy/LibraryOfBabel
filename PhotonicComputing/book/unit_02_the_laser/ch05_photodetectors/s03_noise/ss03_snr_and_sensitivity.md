# 5.3.3 SNR and Receiver Sensitivity

## The Complete Noise Model

Assembling the noise sources:

$$\text{SNR} = \frac{I_{ph}^2}{\langle i^2_{shot}\rangle + \langle i^2_{dark}\rangle + \langle i^2_{thermal}\rangle + \langle i^2_{RIN}\rangle}$$

where:
- Signal: $I_{ph}^2 = (\mathcal{R}P_{in})^2$
- Shot noise: $2eI_{ph}B = 2e\mathcal{R}P_{in}B$
- Dark current shot noise: $2eI_dB$
- Johnson noise: $4k_BTB/R_F$
- Laser RIN: $\text{RIN}\cdot I_{ph}^2 \cdot B = \text{RIN}\cdot\mathcal{R}^2 P_{in}^2 B$

$$\text{SNR} = \frac{\mathcal{R}^2 P_{in}^2}{2e\mathcal{R}P_{in}B + 2eI_dB + 4k_BTB/R_F + \text{RIN}\cdot\mathcal{R}^2P_{in}^2B}$$

## Three Operating Regimes

**Thermally limited** (low power, large $R_F$ not yet optimal):
$$\text{SNR} \approx \frac{\mathcal{R}^2 P_{in}^2 R_F}{4k_BTB}$$

SNR $\propto P_{in}^2$; doubling power quadruples SNR (+6 dB per octave).

**Shot-noise limited** (moderate power, thermal noise smaller):
$$\text{SNR} \approx \frac{\mathcal{R}P_{in}}{2eB}$$

SNR $\propto P_{in}$; doubling power doubles SNR (+3 dB per octave).

**RIN limited** (high power, laser fluctuations dominate):
$$\text{SNR} \approx \frac{1}{\text{RIN}\cdot B}$$

SNR becomes *independent of power* — increasing power does not improve SNR. This is the ultimate floor set by laser noise. For a DFB with RIN = −145 dBc/Hz and $B = 10$ GHz: $\text{SNR}_{RIN} = 145 - 10 = 135$ dB → 22 bits (theoretical limit from RIN alone).

## Receiver Sensitivity

The **sensitivity** of an optical receiver is the minimum input power $P_{min}$ needed to achieve a specified BER (bit error rate) for digital links, or SNR for analog.

For a direct detection digital receiver (OOK modulation), the decision threshold is set at the midpoint between "0" (power 0) and "1" (power $P$). The BER is:

$$\text{BER} = \frac{1}{2}\text{erfc}\left(\frac{Q}{\sqrt{2}}\right)$$

where $Q = I_1/(2\sigma_1)$ and $\sigma_1$ is the rms noise for "1" bits. For BER = $10^{-12}$, $Q \approx 7.03$.

In the shot-noise-limited regime (dominant noise = shot noise of "1" bits):

$$Q \approx \sqrt{\frac{\mathcal{R}P_{min}}{2eB}}$$

$$P_{min} = \frac{2eQ^2 B}{\mathcal{R}} = \frac{2\times1.6\times10^{-19}\times49.4\times B}{\mathcal{R}}$$

For $\mathcal{R} = 1$ A/W, $B = 10$ GHz: $P_{min} = 1.6\times10^{-7}$ W = −37.9 dBm. This is the shot-noise-limited sensitivity — the quantum limit for direct detection.

Practical receivers are 10–15 dB less sensitive than this due to thermal noise.

## Minimum Detectable Power and ENOB for Photonic Computing

For analog photonic matrix multiplication, the relevant quantity is the SNR at a target signal power, which determines the effective number of bits (ENOB) in the computation:

$$\text{ENOB} = \frac{\text{SNR(dB)} - 1.76}{6.02}$$

For a silicon photonic matrix processor with:
- Signal power: $P_{in} = 1$ mW per detector
- $\mathcal{R} = 1$ A/W → $I_{ph} = 1$ mA
- Bandwidth: $B = 1$ GHz (matrix multiplication is not always at full modulation bandwidth)
- TIA $R_F = 10$ kΩ, $T = 300$ K
- Laser RIN = −150 dBc/Hz

Noise contributions:
- Shot: $\sqrt{2\times1.6\times10^{-19}\times10^{-3}\times10^9} = 0.566$ μA
- Thermal: $\sqrt{4\times1.38\times10^{-23}\times300\times10^9/10^4} = 1.27$ μA
- Dark: $\approx 0.18$ μA (negligible at $I_d = 100$ nA)
- RIN: $\sqrt{10^{-15}\times(10^{-3})^2\times10^9} = 31.6$ nA (negligible)

Total rms noise: $\sqrt{0.566^2 + 1.27^2} \approx 1.39$ μA

$\text{SNR} = 10^{-3}/1.39\times10^{-6} = 719 \Rightarrow 57.1$ dB

$\text{ENOB} \approx (57.1 - 1.76)/6.02 \approx 9.2$ bits

This optimistic calculation assumes no modulator nonlinearity, perfect calibration, and no crosstalk. In practice, modulator nonlinearity and fabrication variations reduce ENOB to 5–8 bits, consistent with experimental reports [1].

## Reference

[1] Bandyopadhyay, S., et al. (2022). "Single chip photonic deep neural network with accelerated training." *arXiv:2208.01623*. [Reports 6-bit equivalent SNR in a silicon photonic MZI matrix processor.]
