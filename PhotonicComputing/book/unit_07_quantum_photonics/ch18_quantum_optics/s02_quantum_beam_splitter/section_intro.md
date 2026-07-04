# Section 18.2: The Quantum Beam Splitter

The beam splitter is the humblest component in optics and, quantum-mechanically, one of the most consequential. Classically it just divides a beam; quantum-mechanically it is a two-mode unitary that mixes creation and annihilation operators, and everything interesting in linear optical quantum computing happens inside it. This section develops the beam splitter as an operator transformation and then unleashes its signature quantum effect.

We begin with the transformation itself (18.2.1): the constraints unitarity imposes on the reflection and transmission coefficients, why a single photon entering one port emerges *delocalized* across both, and why the "empty" second port can never be ignored — the vacuum that leaks in through it is mandatory bookkeeping, not an approximation. That vacuum port is the same one through which loss injects noise (Section 17.3.3) and through which squeezing is later injected into LIGO (Section 18.3.3).

Then the centerpiece: the Hong-Ou-Mandel effect (18.2.2), in which two indistinguishable photons entering opposite ports always leave together, and the coincidence rate drops to zero. We derive the cancellation explicitly and connect the depth of the HOM dip to photon indistinguishability. Finally we argue that this two-photon interference is the *only* interaction linear optics provides, and hence the primitive on which Chapter 20's entire architecture rests (18.2.3).

- **18.2.1** — The Quantum Beam Splitter Transformation
- **18.2.2** — The Hong-Ou-Mandel Effect
- **18.2.3** — HOM as the Primitive of Linear Optical Quantum Computing
