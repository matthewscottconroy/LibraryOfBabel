# 9.2.3 Spectral Efficiency and the Shannon Gap

## Spectral Efficiency as the Key Metric

Spectral efficiency (SE) is the ratio of bit rate to spectral bandwidth, measured in bits/s/Hz. It determines how much total capacity can be squeezed into a fixed fiber bandwidth (the C+L band spans ~10 THz; the C-band alone ~4.4 THz).

For a coherent system with Gaussian modulation (the Shannon limit):

$$\text{SE}_{\text{Shannon}} = 2\log_2(1 + \text{SNR}) \quad \text{bits/s/Hz (dual-polarization)}$$

For practical $M$-QAM with FEC overhead $r_{\text{FEC}}$:

$$\text{SE}_{\text{practical}} = \frac{2\log_2 M}{1 + r_{\text{FEC}}} \quad \text{bits/s/Hz}$$

where $r_{\text{FEC}}$ is the ratio of redundancy bits to information bits (e.g., 7% overhead: $r_{\text{FEC}} = 0.07$, so denominator = 1.07).

## The Shannon Gap

For each modulation format, the "Shannon gap" measures how far below the channel capacity the format operates:

$$\text{Gap} = \text{SE}_{\text{Shannon}}(\text{SNR}_{\text{required}}) - \text{SE}_{\text{practical}}$$

Equivalently, the SNR penalty is the excess SNR needed to achieve a given SE compared to the Shannon limit.

For DP-16QAM at SE = 7 bits/s/Hz (after FEC):
- Required SNR for DP-16QAM: ~17 dB
- Shannon SNR required for SE = 7: $\text{SNR}_{\text{min}} = 2^{7/2} - 1 \approx 10.6$ (10.3 dB)
- Shannon gap: 17 - 10.3 = 6.7 dB

This ~7 dB gap is the price of using a fixed-constellation format rather than Gaussian coding. It can be reduced by:
1. Higher-order QAM (closer to Gaussian distribution)
2. Geometric or probabilistic shaping (adjusting the constellation points or their probabilities to better match the Gaussian distribution)
3. Soft-decision FEC (soft decoding gains ~2 dB over hard-decision)

## Probabilistic Shaping

Probabilistic constellation shaping (PCS) is the technique of transmitting inner constellation points more frequently than outer ones, making the transmitted signal statistics approach a Gaussian distribution. The "Maxwell-Boltzmann distribution" over QAM symbols:

$$P(A_k) \propto e^{-\nu|A_k|^2}$$

where $\nu$ is a shaping parameter, reduces the required SNR by ~1–1.5 dB for typical operating conditions and allows fine adjustment of the SE (by changing $\nu$) to track the channel conditions.

Probabilistic shaping for DP-64QAM achieves SEs of 8–11 bits/s/Hz at SNRs of 20–30 dB — within ~2 dB of the Shannon limit. This was a breakthrough in coherent optical transmission, first demonstrated in Bocherer et al. 2015 [1] and now deployed in commercial systems.

## Nyquist WDM and Spectral Efficiency

To approach the Shannon limit in practice, WDM channels must be packed as densely as possible — ideally at Nyquist spacing, where adjacent channels touch without overlapping. For a symbol rate of $B_s$ GBaud, the minimum channel spacing is $B_s$ GHz (Nyquist spacing).

Nyquist WDM with 32 GBaud channels at 32 GHz spacing achieves 100% spectral utilization. With DP-16QAM: 8 bits/symbol × 32 GBaud = 256 Gbps gross; with 7% FEC: 239 Gbps net; at 32 GHz spacing: SE = 239/32 ≈ 7.5 bits/s/Hz.

State-of-art lab demonstrations (as of 2023) have achieved SE > 12 bits/s/Hz using DP-64QAM with probabilistic shaping at 100 GBaud [2].

---

## References

[1] Bocherer, G., Steiner, F., & Schulte, P. (2015). "Bandwidth efficient and rate-matched low-density parity-check coded modulation." *IEEE Transactions on Communications*, 63(12), 4651–4665. [The probabilistic shaping framework for QAM.]

[2] Renaudier, J., Brenot, R., Bigo, S., Mardoyan, H., Jouhno, M., Schmalen, L., ... & Charlet, G. (2023). "Recent advances on 1 Tb/s transmission over a single wavelength." *Journal of Lightwave Technology*, 41(3), 813–825. [State-of-art spectral efficiency demonstrations in coherent optical systems.]
