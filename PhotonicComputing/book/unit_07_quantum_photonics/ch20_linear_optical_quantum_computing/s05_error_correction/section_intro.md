# Section 20.5: Quantum Error Correction for Photonic Systems

Every architecture in this chapter has deferred the same question: what happens when a photon goes missing? Photons decohere slowly — their great virtue as qubits — but they are readily *lost*, absorbed in a waveguide, reflected at a facet, or missed by a detector. In matter qubits the dominant error is a random Pauli flip whose location and nature are unknown until a syndrome reveals them. In dual-rail photonics the dominant error is loss, and loss has a redeeming property: it announces itself. A lost photon leaves the two rails holding vacuum instead of the single photon that $|0\rangle_L$ and $|1\rangle_L$ both carry, so the failure is a **located** error — an *erasure* — at a known position in the circuit. Knowing *where* an error struck, even without knowing what it did, is worth a great deal to a code.

This section builds the photonic error-correction story on that fact. It first models loss as a beam-splitter coupling to the environment and shows why dual-rail encoding converts it into a heralded erasure, with thresholds several times more forgiving than for Pauli noise. It then foliates topological codes into the very cluster states of Section 20.3, so that the measurements running the computation also correct it, and shows why measurement-based photonics maps onto surface codes with unusual grace. It closes with PsiQuantum's fault-tolerant roadmap — the attempt to assemble all of this, at the scale of millions of physical qubits, inside a CMOS photonics foundry.

- **20.5.1** — Photon Loss as Erasure
- **20.5.2** — Surface Codes on Photonic Graph States
- **20.5.3** — PsiQuantum's Fault-Tolerant Roadmap
