# Section 20.3: Measurement-Based Quantum Computing

Section 20.2 ended by inverting the usual picture of computation: in linear optics, gates *are* teleportations. The nondeterministic CZ never touches data; an offline factory prepares an entangled resource state by trial and error, and the gate is delivered by teleporting the data through it. Measurement-based quantum computing (MBQC) carries this inversion to its conclusion. If every gate is already a teleportation through pre-built entanglement, why assemble that entanglement one gate at a time? Prepare a single large entangled state up front — a **cluster state** — and let the entire computation be nothing but a schedule of adaptive single-qubit measurements on it. No entangling gate is ever performed on data: the two-qubit entanglement was all manufactured, probabilistically and offline, before the computation began (Raussendorf & Briegel, 2001).

This is an unusually good fit for photonics. Photons are measured destructively regardless, so a model in which measurement *is* the computation wastes nothing. The probabilistic entangling operations that cripple the circuit model are demoted to the resource-preparation stage, where a failure costs only a few ancilla photons. And "build a quantum computer" becomes "grow a large entangled graph from small pieces" — a percolation problem (Section 20.3.3) rather than a sequential-gate problem. This section develops cluster states and their stabilizer description, the one-way computer that runs on them, and fusion-based quantum computing, the architecture that carried MBQC out of theory and into a semiconductor foundry.

- **20.3.1** — Cluster States
- **20.3.2** — The One-Way Quantum Computer
- **20.3.3** — Fusion-Based Quantum Computing
