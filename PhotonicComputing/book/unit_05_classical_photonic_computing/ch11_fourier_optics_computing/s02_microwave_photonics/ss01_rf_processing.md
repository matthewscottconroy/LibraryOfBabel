# Subsection 11.2.1: Photonic RF Signal Processing

## Orientation

When a radar or electronic warfare system needs to instantaneously characterize a wide band of RF spectrum — say, 1–40 GHz simultaneously — the fundamental challenge is that no single ADC can sample this bandwidth. A 40 GHz-bandwidth signal requires sampling at >80 Gsps (Nyquist), but high-resolution ADCs (>8 bits) saturate at ~10 Gsps due to aperture jitter (timing uncertainty in the sample clock). The photonic solution converts the RF signal to the optical domain, where the naturally narrow linewidth of laser light provides far better timing jitter than electronic oscillators.

---

## 11.2.1.1 The Photonic RF Link

### Basic Architecture

A photonic RF link performs:
1. **Modulation**: An RF signal modulates an optical carrier (via an EOM, Section 7.3.3)
2. **Transmission/processing**: The optical signal travels through fiber (with possible optical processing)
3. **Detection**: A photodetector converts the optical signal back to electrical

For a simple intensity-modulated direct-detection (IMDD) link with a Mach-Zehnder modulator biased at quadrature:

$$V_{\text{RF}}(t) \rightarrow P_{\text{opt}}(t) = \frac{P_0}{2}\left[1 + \sin\left(\frac{\pi V_{\text{RF}}}{V_\pi}\right)\right] \approx \frac{P_0}{2}\left[1 + \frac{\pi V_{\text{RF}}}{V_\pi}\right]$$

(for small $V_{\text{RF}} \ll V_\pi$). The photocurrent at the detector:

$$I(t) = \mathcal{R} P_{\text{opt}}(t) = \frac{\mathcal{R} P_0}{2}\left[1 + \frac{\pi V_{\text{RF}}}{V_\pi}\right]$$

where $\mathcal{R}$ is the responsivity. The RF gain of the link (in dB):

$$G_{\text{RF}} = 20\log_{10}\left(\frac{I_{\text{AC}}}{V_{\text{RF}}} \cdot Z_{\text{load}}\right) = 20\log_{10}\left(\frac{\mathcal{R} P_0 \pi}{2 V_\pi} \cdot Z\right)$$

For $\mathcal{R} = 0.9$ A/W, $P_0 = 100$ mW, $V_\pi = 3.5$ V (LNOI), $Z = 50$ Ω:
$$G_{\text{RF}} = 20\log_{10}\left(\frac{0.9 \times 0.1 \times \pi}{2 \times 3.5} \times 50\right) = 20\log_{10}(2.02) \approx 6 \text{ dB}$$

Modern photonic RF links achieve > 0 dB RF gain, meaning they amplify the RF signal as part of the photonic link — a capability that was impossible with early modulator technology (requiring many dB of optical power to compensate modulator insertion loss and link loss).

### Noise Figure of the Photonic Link

The noise figure (NF) is the ratio of signal-to-noise at input to SNR at output:

$$NF = \frac{S_{\text{in}} / N_{\text{in}}}{S_{\text{out}} / N_{\text{out}}} = \frac{P_{\text{shot}} + P_{\text{RIN}}}{G_{\text{RF}} \cdot k_B T_0 B}$$

Shot noise from the detector: $P_{\text{shot}} = 2 e I_{\text{DC}} Z B = 2 e \mathcal{R} P_0 Z B$

Relative intensity noise (RIN) from the laser: $P_{\text{RIN}} = \text{RIN} \cdot I_{\text{DC}}^2 Z^2 B$

For state-of-the-art low-RIN lasers ($\text{RIN} < -160$ dB/Hz) and high optical power:
- NF ≈ 10–15 dB for typical IMDD links
- NF ≈ 3–5 dB for carrier-suppressed single-sideband (CSSB) links
- Best demonstrated: NF = 2.5 dB for specialized high-power links [1]

Electronic amplifiers achieve NF = 1–3 dB; photonic links remain 2–5 dB noisier. The advantage is not noise figure but bandwidth, linearity, and the ability to transport RF signals over long distances (km) without power loss.

---

## 11.2.1.2 The Photonic Channelizer

### Problem Setup

A broadband RF receiver (e.g., a wideband radar ESM system) needs to identify signals anywhere from 1–40 GHz simultaneously. The conventional approach: a filter bank of electronic bandpass filters, each feeding a narrowband ADC. At 40 GHz, filters and ADCs are expensive, power-hungry, and available only from a handful of vendors.

The photonic channelizer uses the wavelength-division multiplexing capability of optics to parallelize the spectrum measurement:

1. A wideband RF signal modulates an optical comb (a laser with evenly spaced frequency teeth, e.g., from a mode-locked laser or microresonator comb — Section 6.3.3).
2. Each tooth of the comb carries the same RF sidebands, but at a different optical frequency.
3. The comb passes through a dispersive element (a chirped fiber Bragg grating or dispersive fiber). Different optical frequencies arrive at the detector at different times.
4. A single high-speed photodetector samples the output: each time slot corresponds to one comb tooth, i.e., one RF measurement at the frequency corresponding to that time slot.

**Channelizer bandwidth**: For a comb with $N$ teeth, each at frequency $\nu_0 + n \Delta\nu$ ($n = 0, 1, \ldots, N-1$), and RF modulation bandwidth $B_{\text{RF}}$ per comb tooth, the total channelized bandwidth is:

$$B_{\text{total}} = N \times B_{\text{RF}}$$

For $N = 100$ comb teeth and $B_{\text{RF}} = 1$ GHz per tooth: $B_{\text{total}} = 100$ GHz. The entire 100 GHz band is analyzed with one detector.

**Resolution**: The spectral resolution of each channel is set by the comb tooth bandwidth and the optical filter bandwidth used to select channels. For DWDM-spacing combs (12.5 GHz):
$$\Delta f_{\text{channel}} = \text{min}(B_{\text{RF}}, \Delta\nu_{\text{comb}}) \approx 1 \text{ GHz}$$

State-of-the-art photonic channelizers (Cundiff group, NIST; Capmany group, Valencia) achieve:
- Bandwidth: DC to 40+ GHz simultaneous
- Resolution: 50 MHz [2]
- Dynamic range: 45–50 dB (compared to ~70–80 dB for high-end electronic spectrum analyzers — still a gap, but useful for military ESM where speed matters more than dynamic range)

### Defense and Commercial Applications

**Electronic warfare (EW)**: Intercepting radar emissions requires instantaneously measuring a wide RF band to characterize hostile radar waveforms. Photonic channelizers enable this at bandwidths electronic ESM receivers cannot achieve.

**Cognitive radio**: Sensing the RF spectrum over wide bandwidth to find unused channels; photonic channelizers provide the sensing capability.

**Astronomical receivers**: Radio astronomy at mm-wave frequencies (70–115 GHz) uses photonic down-conversion and channelization. ALMA (Atacama Large Millimeter Array) uses photonic local oscillator distribution [3].

---

## 11.2.1.3 Photonic Filtering

### Microwave Photonic Filters (MPF)

A microwave photonic filter processes an RF signal in the optical domain, with the filter response determined by optical path lengths and coupling ratios. The basic architecture:

1. Modulate the RF signal onto multiple optical taps (using a splitter)
2. Apply different delays $\tau_k$ to each tap (using different fiber lengths)
3. Detect and sum all taps

The transfer function is the z-transform of the tap coefficients:

$$H(f) = \sum_{k=0}^{N-1} a_k e^{-i2\pi f k \tau}$$

where $\tau$ is the tap spacing and $a_k$ is the amplitude of the $k$-th tap. This is a finite impulse response (FIR) filter implemented in the photonic domain.

**Reconfigurability**: By varying the tap amplitudes $a_k$ (using VOAs or EOMs) and tap delays (using optical delay lines), the filter response is programmable. Demonstrated filters: bandpass filters from DC to 40 GHz with >60 dB stopband rejection, tunable bandwidth from 100 MHz to 10 GHz, all-optical reconfiguration in < 1 μs [4].

**Advantage over electronic filters**: At frequencies > 10 GHz, electronic filters use distributed LC structures that are physically large, difficult to integrate, and limited in tunability. Photonic filters operate at the same frequencies using compact integrated photonics, with >100 GHz bandwidth and electronic reconfigurability.

---

## References

[1] Urick, V.J., McKinney, J.D., & Williams, K.J. (2015). *Fundamentals of Microwave Photonics*. Wiley. [The comprehensive textbook on the field; Urick et al. are from the Naval Research Laboratory, the leading US government research group in microwave photonics.]

[2] Valley, G.C. (2007). "Photonic analog-to-digital converters." *Optics Express*, 15(5), 1955–1982. [Comprehensive review of photonic ADC approaches; provides the performance data quoted here.]

[3] ALMA Partnership (2015). "The 2014 ALMA long baseline campaign." *Astrophysical Journal Letters*, 808, L1. [ALMA uses photonic local oscillator distribution; this paper describes the system architecture.]

[4] Capmany, J., Ortega, B., & Pastor, D. (2006). "A tutorial on microwave photonic filters." *Journal of Lightwave Technology*, 24(1), 201–229. [The tutorial that defined the MPF as a distinct topic; Capmany and collaborators are the primary contributors to this field.]
