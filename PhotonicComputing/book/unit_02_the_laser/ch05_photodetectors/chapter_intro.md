# Chapter 5: Photodetectors

> *"The problem of the photoelectric effect: electrons are knocked out of metals by light, but only if the light frequency exceeds a threshold — independent of intensity. Einstein's 1905 explanation was simple and radical: light comes in quanta of energy $h\nu$."*
>
> — Physical context, 1905

---

## The Output Side of Photonic Computing

Every photonic computing system — whether a classical optical matrix multiplier, a neuromorphic photonic processor, or a quantum photonic circuit — ultimately produces an output that must be read out by an electronic circuit. The photodetector performs this transduction: it absorbs photons and converts their energy into an electrical current. The quality of this transduction — how efficiently, how quickly, and how quietly photons are converted to electrons — determines the output precision and bandwidth of the entire photonic system.

Understanding photodetectors means understanding three things:

1. **Quantum efficiency**: What fraction of incident photons produce a photoelectron? This determines the minimum detectable power.

2. **Bandwidth**: How quickly does the photocurrent respond to changes in optical power? This determines the maximum data rate or computation clock rate.

3. **Noise**: What unwanted fluctuations appear in the output current in the absence of (or in addition to) signal? This determines the signal-to-noise ratio and therefore the precision of every analog photonic computation.

---

## The Quantum Noise Floor

There is an irreducible noise source in any optical detection: *shot noise*, arising from the discrete nature of photons and electrons. Even if every photon produces exactly one electron (perfect quantum efficiency), and even if the detection circuit is perfectly noiseless, the randomness in the arrival times of photons — a consequence of quantum mechanics, not engineering imperfection — produces current fluctuations:

$$\langle i^2_{shot}\rangle = 2e I_{ph} B$$

where $I_{ph}$ is the photocurrent and $B$ is the bandwidth. This noise floor is fundamental. It cannot be reduced by better circuit design or lower-noise amplifiers. It can only be reduced by detecting more photons (higher power or lower bandwidth).

The shot noise limit is the quantum statement of the Heisenberg uncertainty principle applied to optical measurement: you cannot simultaneously know the exact amplitude and phase of a light field without incurring quantum noise. The minimum number of photons needed to distinguish two signal levels with error probability $< P_e$ follows from the Poisson statistics of photon arrival, and this sets the minimum signal power for reliable optical computation.

---

## Chapter Structure

**Section 5.1 — Detection Fundamentals**: The photoelectric effect, quantum efficiency, responsivity, and the bandwidth-speed tradeoff.

**Section 5.2 — Detector Types**: p-i-n photodiodes, avalanche photodetectors (APDs), single-photon avalanche detectors (SPADs), superconducting nanowire single-photon detectors (SNSPDs), and germanium-on-silicon photodetectors for integrated photonics.

**Section 5.3 — Noise**: Shot noise, Johnson (thermal) noise, dark current, and the complete noise model for an optical receiver. The sensitivity and signal-to-noise ratio of both direct detection and coherent detection are derived.

**Section 5.4 — Receiver Circuits**: Transimpedance amplifiers (TIAs) as the standard interface between detector and digital processing. Coherent vs. direct detection receiver architectures and their implications for photonic computing precision.

---

## Connection to Photonic Computing Precision

The fundamental question for photonic computing is: given that a photonic matrix multiplier encodes weight values as optical intensities and reads out results as photocurrents, what is the achievable precision?

The answer depends on the shot noise floor relative to the signal:

$$\text{SNR} = \frac{I_{ph}^2}{\langle i^2_{noise}\rangle} = \frac{(R P_{in})^2}{2eRI_{ph}B + 4k_BTB/R_L + \text{RIN} \cdot I_{ph}^2 \cdot B}$$

where the three noise terms are shot noise, Johnson noise, and laser relative intensity noise (RIN). The effective number of bits (ENOB) follows from:

$$\text{ENOB} \approx \frac{\text{SNR}(\text{dB}) - 1.76}{6.02}$$

For current state-of-the-art analog photonic processors, ENOB is limited to approximately 5–8 bits by a combination of shot noise, thermal noise, laser RIN, and modulator nonlinearity. This chapter provides the noise physics; later chapters (especially Unit V) discuss the engineering implications for photonic computing precision.
