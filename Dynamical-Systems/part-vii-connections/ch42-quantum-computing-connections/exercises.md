# Exercises — Chapter 42

These exercises develop the key connections between quantum computation and dynamical systems, from Hamiltonian simulation through error correction. Exercise 42.3 requires direct computation with quantum states.

---

**Exercise 42.1.** (Hamiltonian Simulation) Implement the Trotter-Suzuki decomposition for $H = X \otimes I + I \otimes Z$ on 2 qubits for time $t = \pi/4$. Write the circuit (as a product of single-qubit and CNOT gates) and estimate the Trotter error for $n = 10$ steps.

**Exercise 42.2.** (Adiabatic Algorithm) For $H_0 = -\sum_i X_i$ (transverse field) and $H_1 = -\sum_i Z_i Z_{i+1}$ (Ising Hamiltonian) on 4 spins: compute the spectral gap $\Delta(t)$ for $t \in [0,1]$ and find the minimum gap $\Delta_{\min}$.

**Exercise 42.3.** Verify the ETH for the 2-qubit Hamiltonian $H = Z \otimes Z + 0.5(X \otimes I + I \otimes X)$: compute all energy eigenstates and check whether the expectation values of local observables $Z \otimes I$ match thermal averages at the corresponding temperature.

**Exercise 42.4.** (Toric Code) On a $2 \times 2$ torus (4 qubits): write down all vertex and plaquette stabilizers. Verify they commute. Describe what an "anyon" looks like as an error pattern.
