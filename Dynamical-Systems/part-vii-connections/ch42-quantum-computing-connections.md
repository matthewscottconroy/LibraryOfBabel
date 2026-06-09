# Chapter 42 — Quantum Computing: Connections to Dynamical Systems

> *Quantum computation is unitary dynamics on Hilbert space until measurement. Quantum error correction is the problem of maintaining a dynamical invariant (the code space) against noise. Quantum algorithms are structured dynamical systems designed to concentrate amplitude on the answer. The bridge runs in both directions.*

**Prerequisites:** Chapter 21 (quantum information theory), Chapter 38 (quantum complexity), Chapter 14 (Hamiltonian dynamics), Chapter 7 (ergodic theory).

---

## 42.1 Quantum Computation as Hamiltonian Dynamics

### 42.1.1 Gate-Based and Continuous-Time Models

**Definition 42.1.1.** A *quantum circuit* on $n$ qubits is a sequence of unitary operators $U = U_k \cdots U_1$ where each $U_i$ acts on at most 2 qubits (a *gate*). Computation: $|\psi_{out}\rangle = U|0\rangle^{\otimes n}$.

**Definition 42.1.2 (Hamiltonian Simulation).** A *Hamiltonian simulation* of $H$ for time $t$ implements $e^{-iHt}$ as a quantum circuit. By the Trotter-Suzuki formula:
$$e^{-i(H_1+H_2)t} \approx \left(e^{-iH_1 t/n}e^{-iH_2 t/n}\right)^n + O(t^2/n).$$

**Connection to Dynamical Systems:** Quantum computation is *literally* Hamiltonian dynamics on Hilbert space. The computation is the orbit $t \mapsto e^{-iHt}|\psi_0\rangle$ of the initial state $|\psi_0\rangle = |0\rangle^{\otimes n}$ under the Schrödinger flow.

**Theorem 42.1.3 (Universal Quantum Computation via Hamiltonian Simulation).** Any unitary $U$ can be approximated by $e^{-iHt}$ for a time-dependent Hamiltonian $H(t)$. Moreover, any Hamiltonian can be efficiently simulated on a quantum computer (Lloyd, 1996).

---

## 42.2 Adiabatic Quantum Computing

**Definition 42.2.1.** *Adiabatic quantum computation (AQC)* starts in the ground state of an easy Hamiltonian $H_0$ and slowly interpolates to a problem Hamiltonian $H_1$ whose ground state encodes the answer:
$$H(t) = (1 - t/T)H_0 + (t/T)H_1, \quad t \in [0, T].$$

**Theorem 42.2.2 (Adiabatic Theorem — Born-Fock, 1928).** If $H(t)$ changes slowly enough and the spectral gap $\Delta(t) = E_1(t) - E_0(t) > 0$ (between ground state and first excited state), then the system remains in the instantaneous ground state with high probability. The required adiabatic time is:
$$T = O\left(\frac{\|\dot{H}\|}{\Delta_{\min}^2}\right),$$
where $\Delta_{\min} = \min_{t \in [0,1]}\Delta(t)$ is the minimum gap.

**Theorem 42.2.3 (AQC = Gate-Based QC).** Adiabatic quantum computation is polynomially equivalent to gate-based quantum computation — they can simulate each other with polynomial overhead (Aharonov-van Dam-Kempe-Landau-Lloyd-Regev, 2007).

**Connection to KAM Theory:** The adiabatic theorem is the quantum analogue of KAM theory (Chapter 14): if the Hamiltonian changes slowly, the system follows the "adiabatic invariant" (the instantaneous energy level). The spectral gap $\Delta_{\min}$ is the quantum analogue of the Diophantine condition in KAM.

---

## 42.3 Quantum Phase Transitions and Dynamical Phase Transitions

**Definition 42.3.1 (Quantum Phase Transition).** A *quantum phase transition* occurs at $T = 0$ when the ground state of $H(\lambda)$ changes qualitatively as the parameter $\lambda$ crosses a critical value $\lambda_c$. The transition is characterized by the spectral gap $\Delta(\lambda) \to 0$ as $\lambda \to \lambda_c$.

**Theorem 42.3.2 (Undecidability of Phase Transitions).** The question "does the quantum system $H(\lambda)$ have a phase transition at $\lambda_c$?" is undecidable in general (Cubitt-Pérez-García-Wolf, 2015). Specifically, the spectral gap problem for translationally invariant 2D Hamiltonians is undecidable.

**Connection to Dynamical Systems Bifurcations:** Quantum phase transitions are the quantum analogue of classical bifurcations (Chapter 10). At $\lambda_c$:
- Classically: a fixed point changes stability, new orbits emerge
- Quantum: the ground state changes topology (symmetry breaking), entanglement changes qualitatively

**Definition 42.3.3 (Dynamical Phase Transition — Heyl, 2013).** For a quantum quench (sudden change of Hamiltonian $H_0 \to H_1$ at $t=0$), the *Loschmidt echo* is:
$$\mathcal{L}(t) = |\langle\psi_0|e^{-iH_1 t}|\psi_0\rangle|^2.$$

A *dynamical phase transition* occurs when $\mathcal{L}(t) = 0$ — the "return amplitude" vanishes. These zeros are the quantum analogue of Lee-Yang zeros in thermodynamics.

---

## 42.4 Quantum Ergodicity and Thermalization

**Eigenstate Thermalization Hypothesis (ETH — Deutsch, 1991; Srednicki, 1994):**

**Conjecture 42.4.1 (ETH).** For a non-integrable quantum system with Hamiltonian $H$ (energy eigenstates $|E_n\rangle$): for any local observable $A$:
$$\langle E_n | A | E_m \rangle = f_A(\bar{E})\delta_{nm} + e^{-S(\bar{E})/2}R_{nm}g_A(\bar{E}, \omega),$$
where $\bar{E} = (E_n+E_m)/2$, $\omega = E_n - E_m$, $S(\bar{E})$ is the thermodynamic entropy at energy $\bar{E}$, and $R_{nm}$ is a random variable with $\langle R_{nm}^2\rangle = 1$.

**Interpretation:** ETH says that individual energy eigenstates "look thermal" for local observables — their expectation values in an eigenstate equal the thermal average. This is the quantum version of the ergodic hypothesis.

**Theorem 42.4.2 (Berry-Tabor / Bohigas-Giannoni-Schmit Conjecture — Quantum Chaos).** For classically chaotic systems:
- Energy level spacings follow GUE (Gaussian Unitary Ensemble) statistics
- Eigenstates are approximately random unit vectors in the relevant Hilbert space subspace

For classically integrable systems:
- Energy level spacings are Poissonian (independent)
- Eigenstates are localized along classical tori

---

## 42.5 Quantum Error Correction as Dynamical Stability

**Definition 42.5.1.** A *quantum error correcting code* (QECC) is a subspace $\mathcal{C} \subseteq \mathcal{H}^{\otimes n}$ (the "code space") that is stable under a group of errors $\{E_k\}$.

**Connection to Dynamical Systems:** The code space $\mathcal{C}$ is the analogue of an *attractor* in a dynamical system. The *recovery map* $\mathcal{R}$ (error correction circuit) is a contraction mapping that maps perturbed states back to $\mathcal{C}$.

**Theorem 42.5.2 (Knill-Laflamme Conditions as Lyapunov Stability).** A code $\mathcal{C}$ corrects errors $\{E_k\}$ iff the conditions $\langle\psi|E_k^\dagger E_l|\phi\rangle = c_{kl}\langle\psi|\phi\rangle$ hold. This is equivalent to saying:
$$\mathcal{C} \text{ is invariant under } \{E_k^\dagger E_k\} \text{ up to a "gauge" } (c_{kl}).$$

The error correction conditions say $\mathcal{C}$ is "Lyapunov stable" with respect to the error group.

**Topological Quantum Error Correction:**

**Definition 42.5.3 (Toric Code — Kitaev, 1997).** The *toric code* is defined on a 2D torus with qubits on edges. Stabilizers are vertex operators $A_v = \prod_{e\ni v} X_e$ and plaquette operators $B_p = \prod_{e\in\partial p} Z_e$. The code space is the $+1$ eigenspace of all stabilizers.

**Theorem 42.5.4.** The toric code has distance $d = O(\sqrt{n})$ (for $n$ physical qubits) and encodes 2 logical qubits. Errors correspond to anyonic excitations; error correction corresponds to bringing anyons together to annihilate them (topological operations).

**Dynamical Interpretation:** The toric code dynamics under noise is a 2D classical statistical mechanics model (a random-bond Ising model on the dual lattice). The threshold theorem for the toric code is equivalent to a phase transition in this statistical model.

---

## 42.6 Variational Quantum Algorithms and Optimization

**Definition 42.6.1 (QAOA — Quantum Approximate Optimization Algorithm).** QAOA is a variational algorithm with circuit:
$$|\boldsymbol{\gamma},\boldsymbol{\beta}\rangle = \prod_{k=1}^p e^{-i\beta_k H_B}e^{-i\gamma_k H_C}|+\rangle^{\otimes n},$$
where $H_C$ is the cost Hamiltonian (encoding the optimization problem), $H_B = \sum_i X_i$ is the mixing Hamiltonian, and $(\boldsymbol{\gamma}, \boldsymbol{\beta})$ are variational parameters optimized classically.

**Connection to Dynamical Systems:** QAOA is a discrete-time dynamical system in parameter space. The optimization landscape $E(\boldsymbol{\gamma}, \boldsymbol{\beta}) = \langle\boldsymbol{\gamma},\boldsymbol{\beta}|H_C|\boldsymbol{\gamma},\boldsymbol{\beta}\rangle$ is a function on a manifold, and classical gradient descent on this function is the "outer loop." The quantum circuit is the "inner loop" evaluating $E$.

**Theorem 42.6.2 (QAOA as Trotterized Adiabatic Evolution).** For large $p$, QAOA approximates the adiabatic evolution from $H_B$ to $H_C$. The QAOA parameters $\gamma_k \approx t_k/p$ and $\beta_k \approx (T-t_k)/p$ match the adiabatic interpolation time schedule.

---

## Exercises

**Exercise 42.1.** (Hamiltonian Simulation) Implement the Trotter-Suzuki decomposition for $H = X \otimes I + I \otimes Z$ on 2 qubits for time $t = \pi/4$. Write the circuit (as a product of single-qubit and CNOT gates) and estimate the Trotter error for $n = 10$ steps.

**Exercise 42.2.** (Adiabatic Algorithm) For $H_0 = -\sum_i X_i$ (transverse field) and $H_1 = -\sum_i Z_i Z_{i+1}$ (Ising Hamiltonian) on 4 spins: compute the spectral gap $\Delta(t)$ for $t \in [0,1]$ and find the minimum gap $\Delta_{\min}$.

**Exercise 42.3.** Verify the ETH for the 2-qubit Hamiltonian $H = Z \otimes Z + 0.5(X \otimes I + I \otimes X)$: compute all energy eigenstates and check whether the expectation values of local observables $Z \otimes I$ match thermal averages at the corresponding temperature.

**Exercise 42.4.** (Toric Code) On a $2 \times 2$ torus (4 qubits): write down all vertex and plaquette stabilizers. Verify they commute. Describe what an "anyon" looks like as an error pattern.

---

## Chapter Notes

Adiabatic quantum computation: Farhi-Goldstone-Gutmann-Sipser *Quantum computation by adiabatic evolution* (MIT report, 2000). The equivalence of AQC and gate-based QC: Aharonov et al. (SIAM J. Comp., 2007).

ETH and quantum chaos: Deutsch (1991), Srednicki (1994). The review: D'Alessio-Kafri-Polkovnikov-Rigol *From quantum chaos and eigenstate thermalization to statistical mechanics and thermodynamics* (Advances in Physics, 2016).

Topological quantum codes: Kitaev's toric code in *Fault-tolerant quantum computation by anyons* (Annals of Physics, 2003). The surface code review: Fowler-Martinis et al. (2012). The connection to statistical mechanics: Dennis-Kitaev-Landahl-Preskill (2002).

QAOA: Farhi-Goldstone-Gutmann (2014). The connection to adiabatic evolution: Roland-Cerf (2002) and the recent review by Blekos et al. (2024).
