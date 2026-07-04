# Unit VII: Quantum Photonics and Quantum Computing

> *"Nature isn't classical, dammit, and if you want to make a simulation of nature, you'd better make it quantum mechanical, and by golly it's a wonderful problem, because it doesn't look so easy."*
>
> — Richard Feynman, "Simulating Physics with Computers," 1982 [1]

---

## What This Unit Is About

Everything in this book so far has treated light classically. The electric field was a number (or a complex amplitude), intensity was proportional to its square, and computation meant manipulating those amplitudes with interferometers, resonators, and nonlinearities. That classical picture is astonishingly productive — it carries you through fiber communications, MZI meshes, and photonic neural networks without ever failing you. But it is not the whole truth, and the part it leaves out is not a small correction. It is a different kind of physics, with a different kind of computational power.

In 1935, Einstein, Podolsky, and Rosen described two particles that, once they had interacted, remained correlated in a way no classical theory seemed to permit — measure one, and you instantly learn something about the other, however distant [2]. Einstein called it "spooky action at a distance" and took it as evidence that quantum mechanics was incomplete. In 1964, John Bell proved that *no* local hidden-variable theory could reproduce the quantum correlations [3], turning a philosophical dispute into an experimental question. Photons answered it: Bell inequality violations were measured with entangled photon pairs in the 1970s and 1980s, and loophole-free tests closed the remaining escape routes in 2015. Today that spookiness is not a bug but a resource — the fuel of quantum computation, quantum cryptography, and quantum communication.

Among all physical platforms for quantum information, photons hold a special position. They travel at the speed of light. They barely interact with their environment, so a photonic qubit maintains its quantum state at room temperature over kilometers of fiber — no dilution refrigerator required for the qubit itself. And they are manipulated with exactly the hardware this book has already developed: waveguides, beam splitters, phase shifters, ring resonators, and detectors. The photon's weakness is the flip side of its strength: because photons barely interact with anything, they barely interact with *each other*, and two-qubit gates — the heart of quantum computation — cannot be performed deterministically with linear optics alone. Much of this unit is the story of how the field learned to work around that single obstacle.

---

## Six Chapters, from Postulates to the Quantum Internet

**Chapter 17: Quantum Mechanics Foundations for Photonics** builds the formalism: state vectors and Hilbert space, observables and the Born rule, unitary time evolution, density matrices, and the no-cloning theorem. It then quantizes the harmonic oscillator — the single most important calculation in this unit, because every optical mode *is* a harmonic oscillator — and introduces Fock states, coherent states, squeezed states, and the Wigner function. It closes with entanglement: tensor products, Bell states, Bell inequalities, and how entanglement is quantified.

**Chapter 18: Quantum Optics — From Photon Statistics to Squeezing** is where the formalism meets the laboratory. Correlation functions $g^{(1)}$ and $g^{(2)}$ classify light as thermal, coherent, or genuinely quantum; the Hanbury Brown–Twiss experiment measures them; antibunching certifies single photons. The quantum beam splitter produces the Hong-Ou-Mandel effect — two identical photons entering a 50/50 beam splitter always leave together — which is the physical primitive underlying photonic two-qubit gates. Parametric processes (OPA, SPDC) generate entangled pairs and squeezed light, culminating in LIGO's squeezed-light upgrade: quantum optics deployed on a kilometer-scale instrument.

**Chapter 19: Single-Photon Sources and Detectors** covers the device layer: quantum dots, color centers, and heralded SPDC sources, judged by brightness, purity ($g^{(2)}(0)$), and indistinguishability; superconducting nanowire single-photon detectors (SNSPDs) with >98% efficiency and picosecond timing jitter; and cavity QED — the Jaynes-Cummings model, strong coupling, and the Purcell effect — which is how emitters are made bright and indistinguishable at once.

**Chapter 20: Linear Optical Quantum Computing** confronts the two-qubit gate problem head-on. The Knill-Laflamme-Milburn (KLM) protocol shows that measurement plus ancilla photons creates effective nonlinearity; measurement-based and fusion-based architectures (the PsiQuantum approach) convert that insight into fault-tolerant designs; boson sampling demonstrates quantum computational advantage with photons; and photonic error correction treats photon loss as the dominant error channel.

**Chapter 21: Continuous-Variable and Xanadu's Quantum Computing** takes the other road: instead of discrete photon-number qubits, encode information in the continuous quadratures of the field. Gaussian states and operations, GKP encoding, and Xanadu's Borealis machine — a 216-mode Gaussian boson sampler — represent the squeezed-light route to quantum advantage.

**Chapter 22: Quantum Communication and the Quantum Internet** applies everything: quantum key distribution (BB84 and its photonic implementations, up to satellite links), quantum repeaters built on entanglement swapping and quantum memories — made necessary by the no-cloning theorem, which forbids amplifying an unknown quantum signal — and the staged roadmap toward a quantum internet.

---

## The Mathematical Language of This Unit

- **Linear algebra in Dirac notation**: kets $|\psi\rangle$, bras $\langle\phi|$, inner products $\langle\phi|\psi\rangle$, operators and their eigenvalue problems
- **Tensor products**: composite quantum systems and entanglement
- **Operator algebra**: commutators, creation/annihilation operators $\hat{a}$, $\hat{a}^\dagger$
- **Probability theory**: photon counting statistics, correlation functions
- **Phase-space methods**: Wigner functions and Gaussian states

None of this is assumed. Chapter 17 develops the formalism from scratch, always anchored to photonic examples: the polarization of a single photon is our first qubit, a phase shifter is our first unitary operator, a photodetector is our first measurement.

---

## A Note on Honesty

Quantum computing attracts hype the way high-index waveguides attract light. Throughout this unit we will be precise about what has been demonstrated (Bell violations, boson sampling advantage, 15 dB of squeezing, metropolitan QKD networks), what is engineering extrapolation (fault-tolerant photonic quantum computers, global quantum repeater networks), and what the honest obstacles are (photon loss, probabilistic gates, source indistinguishability, detector efficiency). The physics in this unit is among the most experimentally verified in all of science; the technology built on it spans the full range from deployed to speculative. Keeping those categories separate is part of the discipline.

---

## References for the Unit Introduction

[1] Feynman, R.P. (1982). "Simulating physics with computers." *International Journal of Theoretical Physics*, 21(6–7), 467–488. [The founding argument that simulating quantum nature requires quantum hardware.]

[2] Einstein, A., Podolsky, B., & Rosen, N. (1935). "Can quantum-mechanical description of physical reality be considered complete?" *Physical Review*, 47(10), 777–780. [The EPR paradox paper.]

[3] Bell, J.S. (1964). "On the Einstein Podolsky Rosen paradox." *Physics Physique Fizika*, 1(3), 195–200. [Bell's theorem: no local hidden-variable theory reproduces quantum correlations.]

[4] O'Brien, J.L. (2007). "Optical quantum computing." *Science*, 318(5856), 1567–1570. [A concise survey of why photons are attractive qubits and what stands in the way.]
