# Subsection 11.2.2: Photonic Analog-to-Digital Conversion

## Orientation

The analog-to-digital converter (ADC) is one of the critical interfaces between the continuous physical world and the discrete digital processor. For wideband RF systems, electronic ADCs face a fundamental performance wall: as sampling rate increases, the effective number of bits (ENOB) decreases. This limitation arises from aperture jitter — timing uncertainty in the sampling clock. Photonic ADCs exploit the sub-femtosecond timing stability of mode-locked laser pulses to achieve both high sampling rate and high resolution.

---

## 11.2.2.1 The Electronic ADC Limitation

### Aperture Jitter

An ideal ADC samples the input voltage at precisely defined instants $t_k = k/f_s$. A real ADC samples at $t_k + \delta t_k$, where $\delta t_k$ is the aperture jitter — random timing error from clock noise, substrate coupling, and device mismatch.

For a sinusoidal input $V(t) = A\sin(2\pi f_{\text{in}} t)$, the voltage error due to jitter $\delta t$ is:

$$\delta V = \frac{dV}{dt}\bigg|_{t} \cdot \delta t = 2\pi f_{\text{in}} A \cos(2\pi f_{\text{in}} t) \cdot \delta t$$

The RMS voltage error for a sinusoidal input:
$$\sigma_V = \frac{A}{\sqrt{2}} \cdot 2\pi f_{\text{in}} \cdot \sigma_{\delta t}$$

The signal-to-noise ratio:
$$\text{SNR}_{\text{jitter}} = 20\log_{10}\left(\frac{A/\sqrt{2}}{\sigma_V}\right) = -20\log_{10}(2\pi f_{\text{in}} \sigma_{\delta t})$$

The ENOB:
$$\text{ENOB} = \frac{\text{SNR} - 1.76}{6.02} = -\log_2(2\pi f_{\text{in}} \sigma_{\delta t}) - 0.29$$

For a target ENOB = 8 bits and $f_{\text{in}} = 40$ GHz:
$$\sigma_{\delta t} < \frac{1}{2^{8.29} \times 2\pi \times 40\times10^9} = \frac{1}{390 \times 251 \times 10^9} \approx 10 \text{ fs}$$

10 femtosecond jitter at 40 GHz for 8-bit ENOB. State-of-the-art electronic oscillators achieve ~30 fs RMS jitter at best. At 40 GHz input frequency with 30 fs jitter:
$$\text{ENOB} = -\log_2(2\pi \times 40\times10^9 \times 30\times10^{-15}) - 0.29 = -\log_2(0.00754) - 0.29 = 7.1 - 0.29 = 6.8 \text{ bits}$$

Achieving 8-bit ENOB at 40 GHz input requires <10 fs jitter — at the limit of what electronic oscillators can achieve.

**The scaling law**: ENOB = $-\log_2(2\pi f_{\text{in}} \sigma_{\delta t}) - 0.29$. For fixed jitter $\sigma_{\delta t}$, each octave increase in $f_{\text{in}}$ costs exactly 1 ENOB. The "jitter wall" in the Walden plot [1] of commercial ADC performance (ENOB vs. Nyquist frequency) shows the degradation: every ADC family in the world falls below the jitter-limited bound as frequency increases.

---

## 11.2.2.2 Mode-Locked Laser Pulse Sampling

### Why Photons Have Lower Jitter

A mode-locked laser (Section 4.5, if it were included — the relevant physics is: a laser cavity with many longitudinal modes locked in phase produces a train of short pulses) generates pulses at a repetition rate $f_{\text{rep}} = c/(2nL)$ with timing jitter $\sigma_{\delta t} < 1$ fs for the best fiber mode-locked lasers [2]. This is 10–100 times better than the best electronic oscillators.

The physical reason: mode-locked laser timing noise is limited by the spontaneous emission noise from the gain medium (quantum noise), which is extremely small. Electronic oscillator noise is limited by thermal and 1/f noise in transistors, which is much larger.

### The Photonic Sampling Architecture

A photonic ADC uses mode-locked laser pulses as the sampling gate:

1. **Mode-locked laser** produces pulses at $f_{\text{rep}} = 1$ GHz, duration $\tau_p \sim 1$ ps, with <1 fs jitter.
2. **Modulator**: An electro-optic modulator imprints the RF input signal $V_{\text{RF}}(t)$ onto the optical pulse amplitude. Each pulse samples $V_{\text{RF}}$ at the pulse arrival time.
3. **Optical stretch** (time-lens or dispersive stretch): The sampled pulses are optionally stretched in time to ease electronic readout.
4. **Detector + electronic ADC**: The stretched pulses are detected and quantized at a lower effective rate.

The sampling jitter is determined by the laser pulse timing jitter ($< 1$ fs), not by the electronic ADC clock jitter. The quantization (digitization) is performed at a lower rate, where electronic ADCs have adequate ENOB.

**Effective sampling rate**: By using multiple optical channels (WDM) with different delays, the effective sampling rate is multiplied:

$$f_{\text{effective}} = N_{\text{channels}} \times f_{\text{rep}}$$

For $N_{\text{channels}} = 16$ and $f_{\text{rep}} = 1$ GHz: $f_{\text{effective}} = 16$ Gsps.

---

## 11.2.2.3 The Time-Stretch ADC

### Photonic Time Stretching

The time-stretch ADC, developed by Jalali's group at UCLA [3], uses dispersive optical fiber to time-stretch the RF-modulated optical waveform before detection:

1. The RF signal modulates a chirped optical pulse (whose frequency varies linearly with time).
2. The modulated pulse propagates through dispersive fiber with group delay dispersion $\beta_2 L$.
3. Different frequency components travel at different speeds, stretching the pulse in time by a stretch factor $M$:

$$M = 1 + \frac{|\beta_{2f} L_f|}{|\beta_{2c} L_c|}$$

where $\beta_{2c} L_c$ is the chirp in the input pulse and $\beta_{2f} L_f$ is the dispersion of the fiber.

After stretching by $M$, a signal at RF frequency $f_{\text{RF}}$ appears at frequency $f_{\text{RF}}/M$ in the electrical domain. An electronic ADC sampling at $f_s$ effectively captures RF signals up to $M \times f_s / 2$.

For $M = 100$ and an electronic ADC running at 1 Gsps: effective RF bandwidth $= 100 \times 1 \text{ GHz}/2 = 50$ GHz.

**Demonstrated performance**: Jalali group demonstrated capture of a 10 THz optical bandwidth signal at the equivalent of 10 petasamples/second [3] — the highest effective sampling rate ever demonstrated. For practical RF processing:
- Bandwidth: DC to 100 GHz
- ENOB: 5–7 bits at 100 GHz (limited by dispersive distortion, not jitter)
- Real-time operation: demonstrated at 165 Gsps [4]

### Noise and Distortion

The time-stretch ADC trades bandwidth for SNR: the stretching process amplifies some noise components. The dominant noise terms are:

**Shot noise**: Unchanged by stretching (the SNR due to shot noise is $\bar{n} = P \tau_{\text{det}} / (\hbar\omega)$; stretching increases $\tau_{\text{det}}$ and decreases $P$ per pulse, leaving the product approximately constant).

**Relative intensity noise (RIN)**: Appears as amplitude modulation of the sampled pulse; becomes a dominant noise at high stretch factors.

**Nonlinear distortion**: At high RF modulation depths, the modulator transfer function $\cos^2(\pi V / 2V_\pi)$ introduces harmonics that limit spurious-free dynamic range (SFDR). State-of-the-art: 90–100 dBc at 1 GHz, degrading to ~60 dBc at 40 GHz.

---

## 11.2.2.4 The Current State and Genuine Advantage

Electronic ADCs are improving. The Walden plot has advanced by ~1–1.5 bits/decade since 1960 [1]. However, jitter-limited performance is a hard physical wall, and mode-locked laser jitter is intrinsically superior.

The photonic ADC is genuinely superior for:
1. Input frequencies > 40 GHz (electronic ADCs run out of dynamic range)
2. Applications requiring instantaneous wide-bandwidth capture (wideband radar, EW)
3. Systems where the signal is already optical (OCT, LiDAR)

The photonic ADC is NOT superior for:
1. Narrow-band signals at moderate frequencies (electronic ADCs are cheaper and more integrated)
2. Applications requiring > 12-bit resolution (electronic ADCs achieve 14–16 bits at low frequencies)
3. Low-power applications (mode-locked lasers consume watts; low-power electronic ADCs consume milliwatts)

---

## References

[1] Walden, R.H. (1999). "Analog-to-digital converter survey and analysis." *IEEE Journal on Selected Areas in Communications*, 17(4), 539–550. [The Walden plot paper; defines the performance bound for electronic ADCs vs. jitter and frequency.]

[2] Fortier, T.M., et al. (2011). "Generation of ultrastable microwaves via optical frequency division." *Nature Photonics*, 5, 425–429. [Demonstrates <0.3 fs jitter for optical frequency division signals; the best demonstration of laser-based low-jitter oscillators.]

[3] Coppinger, F., Bhushan, A.S., & Jalali, B. (1999). "Photonic time stretch and its application to analog-to-digital conversion." *IEEE Transactions on Microwave Theory and Techniques*, 47(7), 1309–1314. [The original time-stretch ADC paper by Jalali's group.]

[4] Han, Y., & Jalali, B. (2003). "Photonic time-stretched analog-to-digital converter: Fundamental concepts and practical considerations." *Journal of Lightwave Technology*, 21(12), 3085–3103. [Comprehensive analysis of the time-stretch ADC including noise and distortion; 165 Gsps demonstration referenced here.]
