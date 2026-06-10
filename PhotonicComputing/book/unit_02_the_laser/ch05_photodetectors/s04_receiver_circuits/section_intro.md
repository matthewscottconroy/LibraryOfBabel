# Section 5.4: Receiver Circuits

The photodetector produces a current signal. Converting that current to a usable voltage — and doing so with minimal added noise, adequate bandwidth, and good linearity — is the job of the receiver circuit. Two architectures dominate:

The **transimpedance amplifier (TIA)** is the universal first stage of any optical receiver, converting photocurrent to voltage with gain $Z_T = R_F$ and bandwidth determined by the feedback pole. Understanding TIA noise and bandwidth is prerequisite to understanding the precision of any photonic computing output stage.

**Coherent detection** replaces the simple square-law power detector with an optical hybrid that mixes the signal with a local oscillator, enabling phase-sensitive detection. Coherent receivers can achieve shot-noise-limited sensitivity even at low signal powers, and they detect both the amplitude and phase of the optical field — enabling more sophisticated signal processing and higher modulation efficiency.

- **5.4.1** — Transimpedance Amplifiers
- **5.4.2** — Coherent vs. Direct Detection
