# 20.1.1 Dual-Rail Encoding

## One Photon, Two Modes

The standard photonic qubit is **dual-rail**: a single photon delocalized over two optical modes $a$ and $b$ (two waveguides, two fiber paths, or two polarizations of one spatial mode). The logical basis states are

$$|0\rangle_L = |1\rangle_a|0\rangle_b = \hat{a}^\dagger|\text{vac}\rangle, \qquad |1\rangle_L = |0\rangle_a|1\rangle_b = \hat{b}^\dagger|\text{vac}\rangle,$$

and an arbitrary qubit state is the photon created in a superposition of the two modes:

$$|\psi\rangle_L = \alpha|0\rangle_L + \beta|1\rangle_L = \left(\alpha\,\hat{a}^\dagger + \beta\,\hat{b}^\dagger\right)|\text{vac}\rangle.$$

The qubit is not "which state the photon is in" but *which mode the photon occupies* — the quantum information lives in the photon's address. Superposition means the photon takes both rails at once; measuring which rail contains the photon (a detector on each waveguide) is a computational-basis measurement.

**Polarization encoding is the same encoding.** Horizontal and vertical polarization are two orthogonal modes of one spatial channel: $|0\rangle_L = |H\rangle$, $|1\rangle_L = |V\rangle$. A polarizing beam splitter converts between polarization and path versions losslessly, and experiments switch representations freely — polarization for transmission through fiber or bulk optics, path for integrated circuits (waveguides preserve path superbly but scramble polarization). **Time-bin encoding** — early vs. late arrival, the choice for long fiber links where polarization drifts — is again the same idea in temporal modes. Throughout this chapter, "dual-rail" covers all three.

## Why Dual-Rail Rather Than Photon Number?

The obvious alternative — **single-rail** encoding, $|0\rangle_L = |\text{vac}\rangle$, $|1\rangle_L = |1\rangle$ in one mode — is used in specific protocols but fails as a general-purpose qubit for two instructive reasons:

1. **Superpositions require a phase reference.** The state $\alpha|\text{vac}\rangle + \beta|1\rangle$ has a phase between different photon-number states, which is only defined relative to a shared optical local oscillator; distributing and stabilizing that reference across a machine is a serious burden (this is the natural home of the continuous-variable framework, Chapter 21).
2. **Loss is invisible.** If the photon vanishes, single-rail $|1\rangle_L$ *becomes* $|0\rangle_L$ — a silent, undetectable logical bit-flip.

Dual-rail elegantly fixes both. All logical states have exactly one photon, so no number-superposition phase reference is needed. And loss maps the qubit *outside* the code space:

$$\text{loss}: \quad \alpha|10\rangle + \beta|01\rangle \ \longrightarrow\ |00\rangle,$$

which is orthogonal to both logical states. Losing the photon does not corrupt the qubit into a wrong answer — it flags itself as *no answer*, detectable (in principle) by total-photon-number parity and, crucially, convertible into an **erasure error with known location**. Section 20.5 builds photonic error correction on exactly this property; it is the single most important structural fact about the dual-rail qubit.

## The Merits and the Tax

As qubits go, the dual-rail photon has an enviable list of native advantages:

- **Decoherence:** photons barely couple to the environment — no charge noise, no phonons, no magnetic field sensitivity. A photonic qubit's coherence is limited by loss and by interferometric phase stability, not by $T_1/T_2$ physics. Waveguide path-encoded qubits on a chip are interferometrically stable by construction (both rails share the same substrate).
- **Temperature:** the qubit itself works at 300 K (only the detectors, per Chapter 19, want cryogenics).
- **Speed:** gates are passive optical elements; a photon traverses a component in picoseconds, and clock rates are set by source and detector rates (GHz-scale), not by gate durations.
- **Connectivity and networking:** a photonic qubit *is* a flying qubit; chip-to-chip and node-to-node links are the same technology as the gates (Chapter 22).
- **Manufacturability:** the components are the silicon photonics of Unit 3 — waveguides, MZIs, phase shifters — in a CMOS foundry.

The tax, and it is enormous: photons do not interact, so the two-qubit gate — trivially available in any matter-qubit platform via Coulomb or exchange interactions — must be conjured from interference and measurement (Sections 20.1.3–20.2). And photons cannot be stopped: there is no idle state, no memory beyond a delay line, so architectures must be *ballistic* — entanglement and logic flow through the machine at the speed of light, consumed as produced. Every design decision in this chapter traces back to these two facts.

## Notation for What Follows

For two dual-rail qubits we write $|xy\rangle_L$ with $x, y \in \{0,1\}$, occupying four modes $(a_0, a_1, b_0, b_1)$: qubit $A$'s photon in $a_x$, qubit $B$'s in $b_y$. The state $|11\rangle_L = \hat{a}_1^\dagger \hat{b}_1^\dagger|\text{vac}\rangle$ will play the starring role in the CZ gate: it is the only logical basis state in which two photons can meet in the same interferometer arm — the opening that measurement-induced nonlinearity exploits.
