# Chapter 17: Quantum Mechanics Foundations for Photonics

> *"Quantum phenomena do not occur in a Hilbert space. They occur in a laboratory."*
>
> — Asher Peres, *Quantum Theory: Concepts and Methods*, 1993 [1]

---

## Why This Chapter Exists

This book has already used the word "photon" many times — in the photoelectric effect, in laser rate equations, in detector shot noise. In every case we got away with a minimal picture: light energy arrives in lumps of $\hbar\omega$. That picture is enough for classical photonic computing. It is nowhere near enough for quantum photonic computing, where the *state* of one or a few photons — its superpositions, its correlations with other photons, the way measurement disturbs it — is the computational substrate itself.

This chapter builds the necessary machinery honestly, from the postulates up, with two commitments. First, everything is aimed at photonics: our canonical two-level system is the polarization of a single photon, not a spin in a magnetic field; our canonical continuous system is a mode of the electromagnetic field, not a particle in a box. Second, nothing is left as formal decoration. Every mathematical object introduced here — the density matrix, the annihilation operator, the Wigner function, the Bell state — is a working tool that reappears in Chapters 18 through 22 attached to laboratory hardware: beam splitters, parametric crystals, homodyne detectors, entangled-pair sources.

A reader who has met quantum mechanics before will find the postulates familiar but the emphasis different. Textbook quantum mechanics is organized around massive particles: wavefunctions in position space, potentials, tunneling. Quantum *optics* is organized around the harmonic oscillator and its ladder of number states, because a mode of light *is* a harmonic oscillator — that identification, made precise in Section 17.3, is the single most consequential fact in this unit. A reader meeting quantum mechanics for the first time is in surprisingly good shape: the linear algebra of Hilbert spaces is the same mathematics as the Jones calculus of Chapter 2 and the transfer matrices of MZI meshes, now reinterpreted as the exact description of nature rather than a convenient formalism for fields.

---

## The Arc of This Chapter

**Section 17.1 — The Postulates of Quantum Mechanics** states the rules of the game. States are vectors in Hilbert space (17.1.1), and mixed states — statistical ensembles, or subsystems of entangled wholes — are density matrices. Observables are Hermitian operators whose eigenvalues are the possible measurement outcomes (17.1.2). Measurement is governed by the Born rule and changes the state (17.1.3); a corollary with enormous engineering consequences is the no-cloning theorem, which forbids copying an unknown quantum state and thereby forbids amplifying a quantum signal. Closed systems evolve unitarily under the Schrödinger equation (17.1.4) — and every lossless linear-optical element is exactly such a unitary.

**Section 17.2 — The Quantum Harmonic Oscillator** solves the one problem that quantum optics is built on. The algebraic solution by creation and annihilation operators (17.2.2) replaces differential equations with commutator gymnastics, yields the energy ladder $E_n = \hbar\omega(n + \tfrac{1}{2})$, and defines the Fock states $|n\rangle$ (17.2.3) that will become states of definite photon number.

**Section 17.3 — Quantization of the Electromagnetic Field** performs the central construction: expanding the field in modes and quantizing each mode as an oscillator (17.3.1). Photons emerge as excitations of modes, not little bullets. We then meet the three most important families of field states: coherent states (17.3.2), the quantum description of ideal laser light; squeezed states (17.3.3), which redistribute vacuum noise between quadratures and enable sub-shot-noise metrology; and the Wigner function (17.3.4), a phase-space portrait whose negativity is a bona fide signature of non-classicality.

**Section 17.4 — Quantum Entanglement** assembles composite systems by tensor products (17.4.1), exhibits the Bell states (17.4.2) — the maximally entangled two-qubit states that photonic experiments generate by the billions per second — derives the CHSH inequality and its quantum violation (17.4.3), and quantifies entanglement through entropy, concurrence, and negativity (17.4.4).

---

## Mathematical Prerequisites for This Chapter

- **Complex linear algebra**: vector spaces, inner products, eigenvalues and eigenvectors, unitary and Hermitian matrices (used throughout; developed as needed)
- **Basic probability**: expectation values, variance, discrete distributions
- **The classical harmonic oscillator and Maxwell mode decompositions** from Chapters 1–2

Dirac notation is introduced from scratch in Section 17.1.1. No prior quantum mechanics is assumed, though prior exposure will make Section 17.1 faster reading.

---

## The Computing Connection

Every abstraction in this chapter is a hardware statement in disguise:

- **A qubit** → one photon in two modes (polarization, path, or time-bin): the information carrier of Chapters 20 and 22
- **Unitary evolution** → lossless linear optics; an MZI mesh implements a unitary on modes (the same Reck/Clements decomposition as in classical photonic processors, now acting on quantum amplitudes)
- **The Born rule and state collapse** → single-photon detection; heralding; measurement-induced nonlinearity in the KLM protocol (Chapter 20)
- **The no-cloning theorem** → no amplifiers in quantum links; the security of QKD and the necessity of quantum repeaters (Chapter 22)
- **Fock, coherent, and squeezed states** → the resource states of discrete-variable and continuous-variable photonic quantum computing (Chapters 20–21)
- **Wigner negativity** → the dividing line between classically simulable Gaussian optics and genuinely quantum resources (Chapter 21)
- **Bell states and CHSH** → entangled-pair sources, device-independent QKD, and fusion measurements (Chapters 20 and 22)

---

## References for the Chapter Introduction

[1] Peres, A. (1993). *Quantum Theory: Concepts and Methods*. Kluwer Academic Publishers. [The source of the epigraph and a model of operational clarity about what the formalism does and does not say.]

[2] Dirac, P.A.M. (1930). *The Principles of Quantum Mechanics*. Oxford University Press. [The book that codified the bra-ket formalism used in every page of this unit.]

[3] Nielsen, M.A. & Chuang, I.L. (2010). *Quantum Computation and Quantum Information*, 10th anniversary ed. Cambridge University Press. [The standard reference for the quantum information perspective adopted here.]

[4] Gerry, C.C. & Knight, P.L. (2005). *Introductory Quantum Optics*. Cambridge University Press. [The gentlest rigorous path from the postulates to quantized fields; the closest companion to Sections 17.2–17.3.]
