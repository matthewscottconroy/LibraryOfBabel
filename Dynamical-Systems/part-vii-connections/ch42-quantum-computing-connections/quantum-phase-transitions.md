# 42.3 Quantum Phase Transitions and Dynamical Phase Transitions

A classical phase transition occurs when a macroscopic system changes its qualitative behavior as a parameter (temperature, pressure) is varied — water freezing, a magnet demagnetizing. A quantum phase transition occurs at zero temperature when the ground state changes qualitatively as a coupling constant varies.

**Definition 42.3.1 (Quantum Phase Transition).** A *quantum phase transition* occurs at $T = 0$ when the ground state of $H(\lambda)$ changes qualitatively as the parameter $\lambda$ crosses a critical value $\lambda_c$. The transition is characterized by the spectral gap $\Delta(\lambda) \to 0$ as $\lambda \to \lambda_c$.

The closing of the spectral gap is the signature of a quantum phase transition: the ground state and first excited state become degenerate at the critical point. Near the critical point, the system has long-range correlations and quantum fluctuations dominate.

**Theorem 42.3.2 (Undecidability of Phase Transitions).** The question "does the quantum system $H(\lambda)$ have a phase transition at $\lambda_c$?" is undecidable in general (Cubitt-Pérez-García-Wolf, 2015). Specifically, the spectral gap problem for translationally invariant 2D Hamiltonians is undecidable.

This is the same undecidability result from Chapter 38. It says that the existence of a quantum phase transition cannot be determined algorithmically in general — there's no computer program that takes a Hamiltonian as input and outputs whether it's gapped or gapless in the thermodynamic limit.

**Connection to Dynamical Systems Bifurcations:** Quantum phase transitions are the quantum analogue of classical bifurcations (Chapter 10). At $\lambda_c$:
- Classically: a fixed point changes stability, new orbits emerge
- Quantum: the ground state changes topology (symmetry breaking), entanglement changes qualitatively

**Definition 42.3.3 (Dynamical Phase Transition — Heyl, 2013).** For a quantum quench (sudden change of Hamiltonian $H_0 \to H_1$ at $t=0$), the *Loschmidt echo* is:
$$\mathcal{L}(t) = |\langle\psi_0|e^{-iH_1 t}|\psi_0\rangle|^2.$$

A *dynamical phase transition* occurs when $\mathcal{L}(t) = 0$ — the "return amplitude" vanishes. These zeros are the quantum analogue of Lee-Yang zeros in thermodynamics.

The Loschmidt echo measures how much the initial state $|\psi_0\rangle$ overlaps with its time-evolved version. It starts at 1 and decays. A dynamical phase transition is where it hits zero — the system has "forgotten" its initial state entirely. These zeros in time are analogous to Lee-Yang zeros in the complex plane of coupling constants, and they signal a qualitative change in the dynamics.
