# Section 20.1: The Qubit in Photonics

Before any protocol, a representation: what, physically, is a photonic qubit? This section establishes the standard answer — the dual-rail encoding, one photon shared between two modes — and shows that single-qubit operations come essentially for free: every SU(2) rotation is a Mach-Zehnder interferometer, a device Unit 3 already taught us to build by the thousands with 99.9%+ fidelity.

Then the other shoe drops. Two-qubit gates require one photon's state to condition another's, and linear optics — by definition, mode transformations that never couple photons — cannot do it. Section 20.1.3 makes the impossibility precise and surveys the escape routes, setting up the KLM protocol of Section 20.2.

- **20.1.1** — Dual-Rail Encoding
- **20.1.2** — Single-Qubit Gates with Linear Optics
- **20.1.3** — The CNOT Problem
