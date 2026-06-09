# 42.1 Quantum Computation as Hamiltonian Dynamics

## 42.1.1 Gate-Based and Continuous-Time Models

A quantum circuit applies a sequence of unitary operations to qubits. At the end, you measure, and the measurement collapses the quantum state to a classical outcome. Everything up to measurement is Hamiltonian dynamics — reversible, unitary, deterministic.

**Definition 42.1.1.** A *quantum circuit* on $n$ qubits is a sequence of unitary operators $U = U_k \cdots U_1$ where each $U_i$ acts on at most 2 qubits (a *gate*). Computation: $|\psi_{out}\rangle = U|0\rangle^{\otimes n}$.

**Definition 42.1.2 (Hamiltonian Simulation).** A *Hamiltonian simulation* of $H$ for time $t$ implements $e^{-iHt}$ as a quantum circuit. By the Trotter-Suzuki formula:
$$e^{-i(H_1+H_2)t} \approx \left(e^{-iH_1 t/n}e^{-iH_2 t/n}\right)^n + O(t^2/n).$$

**Connection to Dynamical Systems:** Quantum computation is *literally* Hamiltonian dynamics on Hilbert space. The computation is the orbit $t \mapsto e^{-iHt}|\psi_0\rangle$ of the initial state $|\psi_0\rangle = |0\rangle^{\otimes n}$ under the Schrödinger flow.

The Trotter-Suzuki formula is the quantum analogue of splitting methods in numerical analysis of ODEs. You approximate $e^{-i(H_1+H_2)t}$ by alternating small steps of $e^{-iH_1 \Delta t}$ and $e^{-iH_2 \Delta t}$. The error is $O((\Delta t)^2)$ per step — exactly as in classical numerical integration.

**Theorem 42.1.3 (Universal Quantum Computation via Hamiltonian Simulation).** Any unitary $U$ can be approximated by $e^{-iHt}$ for a time-dependent Hamiltonian $H(t)$. Moreover, any Hamiltonian can be efficiently simulated on a quantum computer (Lloyd, 1996).

Lloyd's theorem is the quantum computing version of the universality of quantum mechanics: any physical system can be efficiently simulated by a quantum computer. This is in sharp contrast to classical computers, where simulating quantum systems requires exponential resources (the original motivation for quantum computing, due to Feynman).

From the dynamical systems perspective: the set of quantum circuits is dense in the set of unitary operators on $\mathcal{H}^{\otimes n}$. Any unitary flow can be approximated by a discrete-time quantum circuit — a stroboscopic map of the continuous flow.
