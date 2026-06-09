# 38.3 Quantum PCP and Entanglement

One of the deepest questions in classical complexity theory is the PCP theorem: every NP problem has a "probabilistically checkable proof" where you only need to read a constant number of bits to verify (with high probability) that the proof is correct. Does the quantum analogue hold?

**Classical PCP Theorem:** Every NP problem has a probabilistically checkable proof (PCP) where a verifier checks $O(1)$ bits of the proof and accepts with high probability.

The PCP theorem is equivalent to saying that approximating certain optimization problems (like MAX-3-SAT) is NP-hard. It's one of the foundational results of complexity theory.

**Quantum PCP Conjecture.** Every QMA problem has a quantum PCP: a quantum proof where a verifier measures $O(1)$ qubits and accepts with high probability.

This is open. And it's hard. The main difficulty: classical PCP uses randomness and derandomization, but quantum states can be entangled in ways that make "looking at a few qubits" much more complex.

**Theorem 38.3.1 (Known Quantum PCP Results).** The Quantum PCP conjecture is open. It implies:
- 2-Local Hamiltonian remains QMA-hard even for constant approximation factor
- There exist quantum codes with constant rate, constant distance, and constant LDPC constraint

A major step toward quantum PCP was the NLTS conjecture — "No Low-Energy Trivial States." A state is "trivial" if it can be prepared by a constant-depth quantum circuit. NLTS says low-energy states of certain Hamiltonians cannot be trivial.

**Conjecture 38.3.2 (No Low-Energy Trivial States — NLTS).** For a family of quantum codes with constant rate, the low-energy states cannot be prepared by constant-depth circuits.

**Theorem 38.3.3 (Anshu-Breuckmann-Nirkhe, 2022).** The NLTS conjecture is true: there exist local Hamiltonians where all low-energy states require logarithmic-depth circuits to prepare. This is a major step toward Quantum PCP.

This is very recent work — the preprint appeared in 2022 and was the breakthrough of that year in quantum complexity theory. The proof uses "good quantum LDPC codes" — error correcting codes with constant rate and constant distance that were themselves only recently constructed. The connection: a good quantum code has many "frustrated" constraints, and the low-energy states of the corresponding Hamiltonian must be complex.

The Quantum PCP conjecture remains open. But NLTS was the key intermediate step that many people thought was the hard part.
