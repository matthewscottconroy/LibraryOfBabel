# Chapter 38 — Quantum Information and Computational Complexity

> *P vs NP is the central question of classical complexity theory. QMA is its quantum analogue. The local Hamiltonian problem (QMA-complete) is the quantum version of 3-SAT. Quantum entanglement creates complexity — and complexity theory is the new thermodynamics.*

**Prerequisites:** Chapter 21 (quantum information, density matrices, channels), Chapter 26 (communication complexity, circuit complexity), Chapter 18 (algorithmic information theory).

---

## 38.1 Quantum Complexity Classes

**Definition 38.1.1 (QMA — Quantum Merlin-Arthur).** $L \in$ QMA if there is a polynomial-time quantum verifier $V$ such that:
- (Completeness) If $x \in L$: $\exists$ quantum proof $|\psi\rangle$ with $P(V \text{ accepts}) \geq 2/3$
- (Soundness) If $x \notin L$: $\forall$ quantum proofs $|\psi\rangle$: $P(V \text{ accepts}) \leq 1/3$

QMA is the quantum analogue of NP (where the proof is classical and the verifier is classical).

**Hierarchy:**
$$\text{BPP} \subseteq \text{BQP} \subseteq \text{QMA} \subseteq \text{PP} \subseteq \text{PSPACE}.$$

**Theorem 38.1.2.** BQP $\neq$ NP is not known (but widely believed). BQP $\neq$ BPP is not proven. Quantum speedups (Shor, Grover) show BQP contains problems not known to be in P.

---

## 38.2 The Local Hamiltonian Problem

**Definition 38.2.1.** The *k-Local Hamiltonian Problem*: Given a collection $\{H_i\}$ of Hermitian operators each acting on $k$ qubits (of an $n$-qubit system), and real numbers $a < b$ with $b - a \geq 1/\text{poly}(n)$, decide:
- *Yes*: the ground state energy $E_0 = \lambda_{\min}(H) \leq a$
- *No*: $E_0 \geq b$

where $H = \sum_i H_i$ is the total Hamiltonian.

**Theorem 38.2.2 (Kitaev, 1999).** The 5-Local Hamiltonian Problem is QMA-complete. Subsequently: 2-Local Hamiltonian on a line is QMA-complete (Hallgren-Nagaj-Narayanaswami, 2013).

**Theorem 38.2.3 (Cubitt-Pérez-García-Wolf, 2015).** The spectral gap problem (deciding whether a quantum many-body system is gapless or gapped in the thermodynamic limit) is undecidable. This is a phase transition in computability — the gap can "suddenly appear" with no computable criterion.

---

## 38.3 Quantum PCP and Entanglement

**Classical PCP Theorem:** Every NP problem has a probabilistically checkable proof (PCP) where a verifier checks $O(1)$ bits of the proof and accepts with high probability.

**Quantum PCP Conjecture.** Every QMA problem has a quantum PCP: a quantum proof where a verifier measures $O(1)$ qubits and accepts with high probability.

**Theorem 38.3.1 (Known Quantum PCP Results).** The Quantum PCP conjecture is open. It implies:
- 2-Local Hamiltonian remains QMA-hard even for constant approximation factor
- There exist quantum codes with constant rate, constant distance, and constant LDPC constraint

**Conjecture 38.3.2 (No Low-Energy Trivial States — NLTS).** For a family of quantum codes with constant rate, the low-energy states cannot be prepared by constant-depth circuits.

**Theorem 38.3.3 (Anshu-Breuckmann-Nirkhe, 2022).** The NLTS conjecture is true: there exist local Hamiltonians where all low-energy states require logarithmic-depth circuits to prepare. This is a major step toward Quantum PCP.

---

## 38.4 Quantum Advantage: Shor and Grover

**Theorem 38.4.1 (Shor's Algorithm, 1994).** Factoring an $n$-bit integer takes $O(n^3)$ quantum gate operations on a quantum computer, versus the best classical algorithm (NFS) which takes $e^{O(n^{1/3})}$.

*Key step*: Period finding on ${\mathbb Z}/N{\mathbb Z}$ via the Quantum Fourier Transform (QFT). The QFT is $U_{QFT}|j\rangle = \frac{1}{\sqrt{N}}\sum_{k=0}^{N-1}e^{2\pi ijk/N}|k\rangle$ — a quantum circuit implementing the discrete Fourier transform in $O(n^2)$ gates.

**Theorem 38.4.2 (Grover's Algorithm, 1996).** Unstructured database search (find one item in $N$ satisfying $f(x) = 1$) takes $O(\sqrt{N})$ quantum queries versus $O(N)$ classical.

**Theorem 38.4.3 (BBBV, 1994 — Quantum Search Lower Bound).** Any quantum algorithm for unstructured search requires $\Omega(\sqrt{N})$ queries. Grover's algorithm is optimal.

---

## 38.5 Quantum Entanglement and Complexity

### 38.5.1 Entanglement Entropy as Complexity Measure

**Definition 38.5.1.** For a quantum state $|\psi\rangle$ of $n$ qubits, the *entanglement entropy across bipartition $(A, B)$* is $S(\rho_A) = -\text{Tr}[\rho_A\log\rho_A]$ where $\rho_A = \text{Tr}_B[|\psi\rangle\langle\psi|]$.

**Theorem 38.5.2 (Area Law for Ground States).** Ground states of gapped local Hamiltonians in $d$ dimensions satisfy an *area law*: $S(\rho_A) = O(|\partial A|)$ (entanglement entropy scales with boundary, not volume).

**Conjecture 38.5.3 (Area Law in 2D).** Area laws hold for gapped 2D Hamiltonians. This is proved in 1D (Hastings 2007), open in 2D.

**Implication:** Area law states can be efficiently represented by tensor network states (MPS in 1D, PEPS in 2D). The complexity of quantum states is connected to their entanglement structure.

### 38.5.2 Tensor Networks and Dynamics

**Definition 38.5.4.** A *matrix product state (MPS)* of bond dimension $D$ is:
$$|\psi\rangle = \sum_{i_1,\ldots,i_n} \text{Tr}[A^{i_1}_1 A^{i_2}_2 \cdots A^{i_n}_n] |i_1 \cdots i_n\rangle,$$
where each $A^{i_k}_k \in M_{D \times D}({\mathbb C})$.

**Theorem 38.5.5.** Every 1D ground state of a gapped local Hamiltonian is well-approximated by an MPS with bond dimension $D = e^{O(S)} = \text{poly}(n)$ for area-law states.

---

## 38.6 Quantum Channels as Dynamical Systems

**Definition 38.6.1.** A *quantum Markov semigroup* is a family $(\mathcal{E}_t)_{t \geq 0}$ of CPTP maps satisfying $\mathcal{E}_0 = \text{id}$ and $\mathcal{E}_{t+s} = \mathcal{E}_t \circ \mathcal{E}_s$.

**Theorem 38.6.2 (Lindblad, 1976).** Every quantum Markov semigroup has a generator of the form:
$$\frac{d}{dt}\rho = \mathcal{L}(\rho) = -i[H, \rho] + \sum_k \left(L_k\rho L_k^\dagger - \frac{1}{2}\{L_k^\dagger L_k, \rho\}\right),$$
where $H$ is the Hamiltonian and $L_k$ are *Lindblad operators* (jump operators describing decoherence).

**Theorem 38.6.3 (Quantum Ergodicity).** A quantum Markov semigroup $(\mathcal{E}_t)$ has a unique stationary state $\sigma$ (satisfying $\mathcal{E}_t(\sigma) = \sigma$ for all $t$) iff the only observables commuting with all $L_k$ and $H$ are multiples of the identity (quantum ergodicity condition).

---

## Exercises

**Exercise 38.1.** (QFT Circuit) Write down the quantum circuit for the 3-qubit QFT (on $|j\rangle$ for $j \in \{0,\ldots,7\}$). How many Hadamard gates and CPHASE gates are needed? Verify it implements the DFT matrix.

**Exercise 38.2.** Verify Grover's algorithm for $N = 4$ (2 qubits): find the state $|11\rangle$ using one Grover iteration. Compute the state after the oracle and the inversion step.

**Exercise 38.3.** (Entanglement Entropy) Compute the entanglement entropy of the GHZ state $|GHZ\rangle = (|000\rangle + |111\rangle)/\sqrt{2}$ across the bipartition $A = \{1\}$, $B = \{2,3\}$. Does it satisfy an area law?

**Exercise 38.4.** (Lindblad Dynamics) For a single qubit with $H = 0$, $L = |0\rangle\langle 1|$ (amplitude damping), write down $\mathcal{L}(\rho)$ and solve the ODE $\dot\rho = \mathcal{L}(\rho)$ for initial state $\rho_0 = |+\rangle\langle+|$. What is the stationary state?

---

## Chapter Notes

Kitaev's QMA-completeness of Local Hamiltonian: *Classical and quantum computation* (AMS, 2002). The Cubitt-Pérez-García-Wolf undecidability result: *Undecidability of the spectral gap* (Nature, 2015).

Quantum PCP and NLTS: Anshu-Breuckmann-Nirkhe's *NLTS Hamiltonians from good quantum codes* (STOC 2023) resolves the NLTS conjecture. The Quantum PCP conjecture is surveyed in Aharonov-Arad's *The BCS-BKT transition from the complexity point of view* and in Bookatz et al.'s survey.

Hastings's area law: *An area law for one-dimensional quantum systems* (JSTAT, 2007). Tensor networks for quantum states: Orús's *A practical introduction to tensor networks* (Annals of Physics, 2014).

Lindblad's derivation: *On the generators of quantum dynamical semigroups* (Comm. Math. Phys., 1976). Quantum dynamical systems: Attal-Joye-Pillet (eds.), *Open Quantum Systems* (Springer, 2006).
