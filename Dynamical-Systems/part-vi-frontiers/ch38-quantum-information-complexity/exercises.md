# Exercises — Chapter 38

These exercises span quantum circuits, algorithms, entanglement, and Lindblad dynamics. Exercise 38.4 requires solving a differential equation.

---

**Exercise 38.1.** (QFT Circuit) Write down the quantum circuit for the 3-qubit QFT (on $|j\rangle$ for $j \in \{0,\ldots,7\}$). How many Hadamard gates and CPHASE gates are needed? Verify it implements the DFT matrix.

**Exercise 38.2.** Verify Grover's algorithm for $N = 4$ (2 qubits): find the state $|11\rangle$ using one Grover iteration. Compute the state after the oracle and the inversion step.

**Exercise 38.3.** (Entanglement Entropy) Compute the entanglement entropy of the GHZ state $|GHZ\rangle = (|000\rangle + |111\rangle)/\sqrt{2}$ across the bipartition $A = \{1\}$, $B = \{2,3\}$. Does it satisfy an area law?

**Exercise 38.4.** (Lindblad Dynamics) For a single qubit with $H = 0$, $L = |0\rangle\langle 1|$ (amplitude damping), write down $\mathcal{L}(\rho)$ and solve the ODE $\dot\rho = \mathcal{L}(\rho)$ for initial state $\rho_0 = |+\rangle\langle+|$. What is the stationary state?
