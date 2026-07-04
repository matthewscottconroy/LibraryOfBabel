# Section 20.4: Boson Sampling

The first three sections chased a universal, fault-tolerant quantum computer — a hard, decades-long engineering program. Boson sampling asks a different question: what is the *easiest* thing linear optics can do that a classical computer provably cannot? Aaronson and Arkhipov (2011) found it by giving up almost everything. Abandon universality, abandon error correction, abandon even the ability to choose the computation: just inject $n$ indistinguishable photons into a fixed Haar-random $m$-mode interferometer and record where they come out. The device computes nothing useful. Yet the probability of each outcome is set by the **permanent** of a submatrix of the interferometer — a quantity that is #P-hard to compute (Valiant, 1979) — and this is enough to make classically simulating the machine's *samples* intractable, unless the polynomial hierarchy collapses.

Boson sampling reframed the case for quantum advantage. It does not require a single logical qubit, a single gate teleportation, or a single error-corrected operation; it needs only good photon sources, a low-loss interferometer, and fast detectors — precisely the hardware Chapter 19 delivers. That lowered bar is why photonics, not superconducting qubits, produced some of the field's most scrutinized advantage claims. This section develops the Aaronson-Arkhipov hardness argument, its experimentally friendlier cousin Gaussian boson sampling, and the decade-long arms race between ever-larger photonic samplers and ever-cleverer classical simulations.

- **20.4.1** — The Aaronson-Arkhipov Result
- **20.4.2** — Gaussian Boson Sampling
- **20.4.3** — Experimental Demonstrations
