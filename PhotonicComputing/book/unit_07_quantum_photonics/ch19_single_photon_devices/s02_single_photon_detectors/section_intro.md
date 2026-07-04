# Section 19.2: Single-Photon Detectors

Detecting a single photon means registering an energy deposit of order $10^{-19}$ J — and doing it with high probability, at a precisely known time, without being fooled by thermal fluctuations. Every single-photon detector is judged on the same scorecard:

- **System detection efficiency (SDE)**: probability that a photon arriving in the input fiber/waveguide produces an electrical click.
- **Dark count rate (DCR)**: clicks per second with no light present.
- **Timing jitter**: the FWHM uncertainty of the click time relative to photon arrival.
- **Dead time / maximum count rate**: how quickly the detector can fire again.
- **Photon-number resolution (PNR)**: whether the detector can distinguish 1 photon from 2.
- **Operating temperature**: room temperature, 2–4 K cryocooler, or 100 mK dilution/ADR stage.

This section covers the three technology families in order of increasing performance and decreasing convenience: semiconductor avalanche photodiodes (room temperature to Peltier-cooled, mature, modest efficiency), superconducting nanowire single-photon detectors (2–4 K, the all-around champions: >98% SDE, few-ps jitter, sub-Hz dark counts), and transition-edge sensors plus waveguide-integrated detection (the photon-number-resolving and on-chip endgame).

- **19.2.1** — SPADs and Avalanche Photodiodes: Semiconductor Click Detectors
- **19.2.2** — Superconducting Nanowire Single-Photon Detectors (SNSPDs)
- **19.2.3** — Photon-Number Resolution and Waveguide-Integrated Detection
