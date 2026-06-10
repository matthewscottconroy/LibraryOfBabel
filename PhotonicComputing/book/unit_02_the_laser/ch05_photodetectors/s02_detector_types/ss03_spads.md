# 5.2.3 Single-Photon Avalanche Detectors (SPADs)

## Geiger Mode Operation

An APD biased above its avalanche breakdown voltage (Geiger mode) will, when triggered by a single photon, self-sustain an avalanche that saturates the output current. This is a binary event: a single photon causes a macroscopic current pulse of ~1 mA, easily detectable by electronics. The SPAD is reset by quenching the avalanche (either passively by a series resistor that drops the bias below breakdown, or actively by a fast electronic circuit).

**Key SPAD parameters**:
- **Photon detection efficiency (PDE)**: The probability that an incident photon fires the SPAD. PDE = QE × avalanche triggering probability. Typical values: 20–70% at 850 nm (Si SPADs); 10–25% at 1550 nm (InGaAs SPADs).
- **Dark count rate (DCR)**: The rate of false avalanche events triggered by thermal carriers or tunneling, in the absence of light. Typical: 100–1000 counts/second for cooled InGaAs SPADs; 10–100 counts/second for Si SPADs at room temperature.
- **Timing jitter**: The uncertainty in the photon arrival time measurement. Silicon SPADs: <50 ps FWHM; InGaAs SPADs: 50–200 ps FWHM.
- **Dead time**: The period after an avalanche during which the detector is resetting and cannot detect new photons. Typical: 10–100 ns, limiting count rates to 1–100 MHz.

## Silicon SPAD Arrays (SPAD Imagers)

Silicon SPADs can be fabricated in standard CMOS processes, enabling large 2D arrays with per-pixel electronics (quenching, counting, timing). SPAD imagers are used for:
- Time-of-flight LiDAR (photonic computing applications in robotics, autonomous vehicles)
- Fluorescence lifetime imaging microscopy (FLIM)
- Quantum key distribution (QKD) receivers

**2D SPAD arrays**: Current products achieve 1M+ pixels, 100+ kcps per pixel, and < 50 ps timing jitter. The "fill factor" (active area fraction) is limited by the per-pixel electronics (~5–30%), but optical microlens arrays improve effective collection.

## InGaAs SPADs for 1550 nm Quantum Photonics

Quantum photonic processors require detecting individual photons at telecom wavelengths (1310 or 1550 nm). InGaAs/InP SPADs are the standard detector for this:
- Operating temperature: −30 to −50°C (cooled to suppress dark counts)
- PDE: 15–25% at 1550 nm
- DCR: 100–10,000 counts/s
- Dead time: 10 μs (gated operation) to 25 ns (free-running with afterpulsing suppression)

The limited efficiency (15–25%) is a major obstacle for boson sampling, photonic quantum computing, and other quantum information processing protocols that require many successive photon detections, since the probability of detecting $n$ photons scales as $\eta^n$: with $\eta = 0.2$, detecting 20 photons has probability $0.2^{20} \approx 10^{-14}$.

This efficiency limitation is one reason SNSPDs (Section 5.2.4) are used in the most demanding quantum photonic experiments.
