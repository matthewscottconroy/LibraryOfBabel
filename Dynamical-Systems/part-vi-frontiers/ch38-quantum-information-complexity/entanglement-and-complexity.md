# 38.5 Quantum Entanglement and Complexity

## 38.5.1 Entanglement Entropy as Complexity Measure

Entanglement entropy measures how "quantum" a state is. A product state has zero entanglement entropy — it's essentially classical. A highly entangled state has large entanglement entropy and requires exponential classical resources to represent. The area law connects entanglement to computational complexity.

**Definition 38.5.1.** For a quantum state $|\psi\rangle$ of $n$ qubits, the *entanglement entropy across bipartition $(A, B)$* is $S(\rho_A) = -\text{Tr}[\rho_A\log\rho_A]$ where $\rho_A = \text{Tr}_B[|\psi\rangle\langle\psi|]$.

**Theorem 38.5.2 (Area Law for Ground States).** Ground states of gapped local Hamiltonians in $d$ dimensions satisfy an *area law*: $S(\rho_A) = O(|\partial A|)$ (entanglement entropy scales with boundary, not volume).

The area law says: in a gapped quantum system at zero temperature, the entanglement between a region $A$ and its complement is proportional to the boundary area $|\partial A|$, not to the volume. This is a dramatic constraint — generic quantum states satisfy a "volume law" where entanglement grows with volume.

**Conjecture 38.5.3 (Area Law in 2D).** Area laws hold for gapped 2D Hamiltonians. This is proved in 1D (Hastings 2007), open in 2D.

**Implication:** Area law states can be efficiently represented by tensor network states (MPS in 1D, PEPS in 2D). The complexity of quantum states is connected to their entanglement structure.

## 38.5.2 Tensor Networks and Dynamics

The area law tells us that ground states of gapped 1D Hamiltonians can be efficiently described by matrix product states.

**Definition 38.5.4.** A *matrix product state (MPS)* of bond dimension $D$ is:
$$|\psi\rangle = \sum_{i_1,\ldots,i_n} \text{Tr}[A^{i_1}_1 A^{i_2}_2 \cdots A^{i_n}_n] |i_1 \cdots i_n\rangle,$$
where each $A^{i_k}_k \in M_{D \times D}({\mathbb C})$.

**Theorem 38.5.5.** Every 1D ground state of a gapped local Hamiltonian is well-approximated by an MPS with bond dimension $D = e^{O(S)} = \text{poly}(n)$ for area-law states.

The bond dimension $D$ measures the amount of entanglement: if $D = 1$, the state is a product state (no entanglement). The area law guarantees $D$ is polynomial in $n$ for ground states of gapped systems.

This is directly connected to computational complexity: if we can efficiently find the MPS representation of the ground state, we can solve the Local Hamiltonian Problem efficiently. But this is QMA-hard — so either finding the MPS representation is hard even when it's known to be small, or the area law doesn't always give polynomial bond dimension. Current understanding: both are true in different regimes, and the boundary between them is not well understood.
