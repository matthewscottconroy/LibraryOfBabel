# 6.4.3 Distributed Raman Amplification

## Distributed vs. Lumped Amplification

An EDFA is a *lumped* amplifier: gain is concentrated in a 5–10 m coil of Er-doped fiber at one point in the span. Signal power varies greatly over the span — high at the transmitter, exponentially decaying to its minimum at the EDFA input (the noise-critical point).

Distributed Raman amplification spreads the gain throughout the fiber span by injecting backward-propagating pump light at a wavelength ~100 nm shorter than the signal. SRS in the transmission fiber itself provides gain. The signal experiences gain distributed along the 80–100 km span, maintaining higher average power and improving OSNR.

## Raman Gain in Standard Fiber

For a backward-propagating 1450 nm pump providing distributed gain for 1550 nm signal:
- Raman gain coefficient: $g_R \approx 0.4 \times 10^{-13}$ m/W (for backward pump in SMF-28, accounting for polarization averaging)
- Pump power needed for 0 dB total gain (transparent span): $P_p \approx \alpha_s/g_R = 0.046/(0.4\times10^{-13}) = 1.15$ MW·m — using effective length: $P_p L_{eff} = 1.15$ MW·m, so $P_p \approx 50$ mW for $L_{eff} = 22$ km

Typical Raman pump power: 200–500 mW to achieve 15–20 dB on-off gain with 80 km spans.

## Effective Noise Figure Improvement

Distributed Raman amplification improves the effective noise figure by the "distributed gain advantage" — signal power is maintained higher earlier in the span, so the noise floor at the EDFA input is lower. The effective noise figure of a Raman-pumped EDFA combination is:

$$F_{eff} = F_{EDFA} \cdot e^{-g_R P_p L_{eff}}$$

Typical improvement: 4–6 dB over a purely EDFA-amplified span. This allows either longer spans (100–120 km) or higher WDM channel count or higher-order modulation.

Submarine cables universally use Raman + EDFA hybrid amplification; terrestrial ultra-long-haul systems use Raman in high-performance configurations.

## Relevance to Photonic Computing

Raman amplification matters for photonic computing in the context of long optical interconnects: data center interconnects over 10–80 km, where purely EDFA-amplified links may not meet OSNR requirements for > 400 Gbps coherent transmission. Distributed Raman extends the reach while maintaining adequate SNR for data transmission to/from photonic computing nodes.
