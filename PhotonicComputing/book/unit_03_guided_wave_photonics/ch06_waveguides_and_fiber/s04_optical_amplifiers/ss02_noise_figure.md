# 6.4.2 Noise Figure and Amplified Spontaneous Emission

## The Origin of ASE

Every optical amplifier generates spontaneous emission that is amplified along with the signal — **amplified spontaneous emission (ASE)**. ASE occupies the full gain bandwidth of the amplifier and adds noise to all signal channels. Unlike shot noise (which scales as $\sqrt{P_{signal}}$), ASE power scales with gain and bandwidth.

The ASE power in one polarization mode over optical bandwidth $\Delta\nu_{opt}$:

$$P_{ASE} = n_{sp}(G-1)\hbar\omega\Delta\nu_{opt}$$

where $n_{sp} = N_2/(N_2 - N_1)$ is the spontaneous emission factor (=1 for full inversion, >1 for partial inversion). For a fully inverted EDFA ($n_{sp} = 1$) with $G = 20$ dB = 100, $\Delta\nu_{opt} = 30$ nm $\approx 3.75$ THz:

$$P_{ASE} = 1 \times (100-1) \times 6.63\times10^{-34} \times 1.93\times10^{14} \times 3.75\times10^{12} \approx 47 \text{ μW}$$

Both polarizations contribute, so total $P_{ASE} \approx 94$ μW = −10.3 dBm per 30 nm bandwidth.

## Noise Figure Definition

The noise figure of an amplifier is defined as the degradation of signal-to-noise ratio:

$$F = \frac{\text{SNR}_{in}}{\text{SNR}_{out}}$$

where SNR is measured in terms of the photon statistics (shot-noise-limited SNR). For a linear optical amplifier:

$$F = 2 n_{sp} \frac{G-1}{G} \approx 2 n_{sp}$$

The **quantum limit** for a phase-insensitive linear amplifier is $F = 2n_{sp} = 2$ (3 dB), reached only for full inversion ($n_{sp} = 1$). This is the standard quantum limit (SQL): every linear amplifier adds at least 3 dB of noise in a phase-insensitive amplification scheme.

**Practical EDFA NF**: 4–7 dB. The deviation from the 3 dB limit arises from:
- Incomplete population inversion at the signal input end of the fiber (before the pump reaches full inversion)
- Residual absorption of the pump and signal at the fiber connector interfaces
- Signal-ASE beat noise from prior amplifier stages in cascaded chains

## Cascaded Amplifier Noise: Friis Formula

In a chain of $N$ identical amplifiers (gain $G$, noise figure $F$, span loss $L = G$), the total noise figure is given by the Friis formula:

$$F_{total} = F + \frac{F-1}{G} + \frac{F-1}{G^2} + \ldots \approx F \cdot N$$

For $N = 100$ amplifiers, each $F = 6$ dB: $F_{total} \approx 100 \times 4 = 400 = 26$ dB. The total system noise figure of a 10,000 km link with 100 EDFAs is ~26 dB — meaning the effective signal quality degrades by 26 dB from the quantum shot noise baseline.

**Optical signal-to-noise ratio (OSNR)**: The end-of-link OSNR in a 0.1 nm reference bandwidth is:

$$\text{OSNR} = \frac{P_s}{P_{ASE,total}} = \frac{P_s}{N \cdot P_{ASE,span}}$$

For 10,000 km with 100 amplifiers at $G = 20$ dB, $F = 5$ dB, 0.1 nm BW, $P_s = 0$ dBm at each amplifier input:

$$\text{OSNR} \approx 10 \text{ dB}$$

This is barely sufficient for 100 Gbps QPSK. Higher-order modulation (16-QAM, 64-QAM) requires OSNR > 20–25 dB, which is why long submarine cables use EDFAs with low NF and carefully manage launch power.
