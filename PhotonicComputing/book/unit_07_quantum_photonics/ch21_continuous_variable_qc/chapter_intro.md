# Chapter 21: Continuous-Variable and Xanadu's Quantum Computing

> *"We construct quantum error-correcting codes that embed a finite-dimensional code space in the infinite-dimensional Hilbert space of a system described by continuous quantum variables."*
>
> — Daniel Gottesman, Alexei Kitaev, and John Preskill, *Physical Review A*, 2001

---

## A Different Way to Be Quantum

The previous chapter treated the photon as a qubit: one photon, two modes, a two-dimensional Hilbert space. That is the *discrete-variable* (DV) paradigm, and its central difficulty is that single photons are hard to create deterministically, hard to interact with one another, and easy to lose.

This chapter develops the alternative: *continuous-variable* (CV) quantum information, in which the carriers of quantum information are not the discrete occupations of modes but the continuous quadrature amplitudes of the electromagnetic field — the same $\hat{x}$ and $\hat{p}$ observables measured by a homodyne detector. A single optical mode is an infinite-dimensional quantum system, a harmonic oscillator. Instead of asking "is the photon here or there?", CV quantum information asks "what are the field's amplitude and phase quadratures, and how are their fluctuations correlated with those of other modes?"

The CV paradigm has a striking practical advantage. Its basic resource — squeezed light — is produced *deterministically* by a parametric amplifier: turn on the pump laser, and squeezed vacuum comes out, every pulse, with no heralding and no post-selection. Its basic measurement — homodyne detection — operates at room temperature with quantum efficiency above 99% and gigahertz bandwidth, using ordinary photodiodes rather than cryogenic single-photon counters. Gaussian operations (beam splitters, phase shifts, squeezers) are exactly the operations integrated photonics performs well. This determinism is why CV experiments hold the records for the largest entangled states ever created on any platform: cluster states with more than one million entangled modes, generated in a tabletop optical setup.

The paradigm also has a precisely understood weakness. States, operations, and measurements that are all *Gaussian* — fully described by means and covariances — can be simulated efficiently on a classical computer. This is the CV analogue of the Gottesman-Knill theorem, and it means that squeezers, beam splitters, and homodyne detectors alone can never yield a quantum computational advantage. Some non-Gaussian element — photon-number-resolving detection, a cubic phase gate, or a non-Gaussian ancilla state — must be injected. The entire architecture question of CV quantum computing is *where and how to inject the non-Gaussianity*.

The deepest answer to that question is the Gottesman-Kitaev-Preskill (GKP) code quoted above: encode a qubit as a grid of quadrature spikes in the oscillator's phase space. GKP states are simultaneously the non-Gaussian resource, an error-correcting code that fights small displacement errors, and the bridge back to qubit-based fault tolerance. They demand extraordinary squeezing levels — of order 10 dB or more — which is why the world record for optical squeezing (15 dB, achieved in 2016) is not a curiosity but a roadmap milestone.

The second half of the chapter turns to the company that has bet its existence on this paradigm: Xanadu. Its Borealis machine (2022) time-multiplexed squeezed-light pulses from a single source into a 216-mode entangled state and demonstrated a quantum computational advantage in Gaussian boson sampling — with full programmability, a first for photonic advantage experiments. Its open-source software stack, PennyLane and Strawberry Fields, made differentiable quantum programming a standard tool. Its stated destination is a fault-tolerant photonic computer built from GKP qubits stitched together by CV cluster states.

## The Arc of This Chapter

**Section 21.1 — CV Quantum Information** builds the formalism. *21.1.1* defines quadrature operators, phase-space representations, vacuum noise, and squeezed states, together with the homodyne and heterodyne measurements that read them out. *21.1.2* develops Gaussian states and operations — covariance matrices, symplectic transformations, CV cluster states — and states the field's central no-go result: all-Gaussian circuits are classically simulable. *21.1.3* presents the GKP encoding: how a qubit hides inside an oscillator, how small displacement errors are detected and corrected, and what squeezing levels fault tolerance demands.

**Section 21.2 — Xanadu's Photonic Quantum Computer** examines the leading industrial embodiment. *21.2.1* dissects Borealis: time-domain multiplexing with fiber delay loops, programmable beam-splitter gates, photon-number-resolving readout, and the 2022 quantum-advantage result. *21.2.2* covers the PennyLane software framework and differentiable quantum programming. *21.2.3* surveys CV algorithms — Gaussian boson sampling applications, CV quantum neural networks, quantum kernels — and closes with a sober accounting of CV versus DV tradeoffs.

## Prerequisites

This chapter leans on Chapter 17 (quantum states, operators, measurement), Chapter 18 (field quantization, coherent states, squeezed light, parametric down-conversion), Chapter 19 (photon-number-resolving detectors), and Chapter 20 (cluster states, measurement-based computing, Gaussian boson sampling). The new mathematics — symplectic linear algebra on covariance matrices — is developed from scratch in Section 21.1.2 and requires only ordinary matrix manipulation.

---

## References for the Chapter Introduction

[1] Gottesman, D., Kitaev, A., & Preskill, J. (2001). Encoding a qubit in an oscillator. *Physical Review A*, 64(1), 012310. [DOI: 10.1103/PhysRevA.64.012310]

[2] Lloyd, S. & Braunstein, S.L. (1999). Quantum computation over continuous variables. *Physical Review Letters*, 82(8), 1784–1787. [DOI: 10.1103/PhysRevLett.82.1784] [The founding proposal of CV quantum computing.]

[3] Braunstein, S.L. & van Loock, P. (2005). Quantum information with continuous variables. *Reviews of Modern Physics*, 77(2), 513–577. [DOI: 10.1103/RevModPhys.77.513]

[4] Madsen, L.S., Laudenbach, F., Askarani, M.F., et al. (2022). Quantum computational advantage with a programmable photonic processor. *Nature*, 606, 75–81. [DOI: 10.1038/s41586-022-04725-x] [The Borealis experiment.]
