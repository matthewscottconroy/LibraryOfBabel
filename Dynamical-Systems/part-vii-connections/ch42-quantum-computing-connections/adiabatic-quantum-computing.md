# 42.2 Adiabatic Quantum Computing

Adiabatic quantum computation is based on a beautiful physical principle: if you change a quantum system slowly enough, it stays in its ground state. This is the quantum adiabatic theorem, proved by Born and Fock in 1928. You can use it to solve optimization problems: encode the solution in the ground state of a complicated Hamiltonian, then slowly evolve from an easy Hamiltonian (whose ground state you know) to the complicated one.

**Definition 42.2.1.** *Adiabatic quantum computation (AQC)* starts in the ground state of an easy Hamiltonian $H_0$ and slowly interpolates to a problem Hamiltonian $H_1$ whose ground state encodes the answer:
$$H(t) = (1 - t/T)H_0 + (t/T)H_1, \quad t \in [0, T].$$

**Theorem 42.2.2 (Adiabatic Theorem — Born-Fock, 1928).** If $H(t)$ changes slowly enough and the spectral gap $\Delta(t) = E_1(t) - E_0(t) > 0$ (between ground state and first excited state), then the system remains in the instantaneous ground state with high probability. The required adiabatic time is:
$$T = O\left(\frac{\|\dot{H}\|}{\Delta_{\min}^2}\right),$$
where $\Delta_{\min} = \min_{t \in [0,1]}\Delta(t)$ is the minimum gap.

The adiabatic time scales as $1/\Delta_{\min}^2$. If the spectral gap closes to zero during the evolution — a quantum phase transition — the adiabatic time becomes infinite, and the algorithm fails. This is the bottleneck: hard optimization problems tend to correspond to Hamiltonians with exponentially small gaps.

**Theorem 42.2.3 (AQC = Gate-Based QC).** Adiabatic quantum computation is polynomially equivalent to gate-based quantum computation — they can simulate each other with polynomial overhead (Aharonov-van Dam-Kempe-Landau-Lloyd-Regev, 2007).

**Connection to KAM Theory:** The adiabatic theorem is the quantum analogue of KAM theory (Chapter 14): if the Hamiltonian changes slowly, the system follows the "adiabatic invariant" (the instantaneous energy level). The spectral gap $\Delta_{\min}$ is the quantum analogue of the Diophantine condition in KAM.

This is a precise analogy. In KAM theory, a quasi-periodic trajectory persists under perturbation if the frequency vector satisfies a Diophantine condition — it's far from resonances. In adiabatic computation, the ground state persists under the slow change of Hamiltonian if the spectral gap stays open — the ground state is "far from" excited states. The minimum gap is the quantum Diophantine condition.
