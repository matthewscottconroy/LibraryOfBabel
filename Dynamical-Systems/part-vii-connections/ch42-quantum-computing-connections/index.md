# Chapter 42 — Quantum Computing: Connections to Dynamical Systems

> *Quantum computation is unitary dynamics on Hilbert space until measurement. Quantum error correction is the problem of maintaining a dynamical invariant (the code space) against noise. Quantum algorithms are structured dynamical systems designed to concentrate amplitude on the answer. The bridge runs in both directions.*

**Prerequisites:** Chapter 21 (quantum information theory), Chapter 38 (quantum complexity), Chapter 14 (Hamiltonian dynamics), Chapter 7 (ergodic theory).

---

## What This Chapter Is About

Quantum computation is Hamiltonian dynamics. A quantum circuit is a sequence of unitary operators acting on $n$ qubits; the computation proceeds as the state $|\psi\rangle$ evolves under these unitaries. From the dynamical systems point of view, the computation is the orbit of the initial state $|0\rangle^{\otimes n}$ under a time-dependent Hamiltonian $H(t)$.

This is not just an analogy. The connections are precise and deep.

Adiabatic quantum computation — the model where you slowly evolve from an easy Hamiltonian to a problem Hamiltonian — is the quantum analogue of KAM theory. The adiabatic theorem (the system stays in the ground state if the Hamiltonian changes slowly enough) is the quantum analogue of adiabatic invariants in classical mechanics. The spectral gap $\Delta_{\min}$ plays the role of the Diophantine condition in KAM.

Quantum error correction is the problem of maintaining an invariant subspace (the code space) against noise. The code space is an attractor — every state, after error correction, is pulled back to the code space. The Knill-Laflamme error correction conditions are a stability condition analogous to Lyapunov stability.

The Eigenstate Thermalization Hypothesis (ETH) is the quantum ergodic hypothesis: individual energy eigenstates of a chaotic quantum system look thermal for local observables. This is a quantum analogue of the ergodic hypothesis from Chapter 7, and it's still a conjecture — not proved in general.

---

## Sections

- [42.1 Quantum Computation as Hamiltonian Dynamics](quantum-computation-hamiltonian.md)
- [42.2 Adiabatic Quantum Computing](adiabatic-quantum-computing.md)
- [42.3 Quantum Phase Transitions and Dynamical Phase Transitions](quantum-phase-transitions.md)
- [42.4 Quantum Ergodicity and Thermalization](quantum-ergodicity-thermalization.md)
- [42.5 Quantum Error Correction as Dynamical Stability](error-correction-stability.md)
- [42.6 Variational Quantum Algorithms and Optimization](variational-quantum-algorithms.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
