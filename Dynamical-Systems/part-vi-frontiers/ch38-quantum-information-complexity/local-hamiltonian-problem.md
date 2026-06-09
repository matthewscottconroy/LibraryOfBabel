# 38.2 The Local Hamiltonian Problem

In classical complexity, 3-SAT is the canonical NP-complete problem: decide whether a Boolean formula with 3-literal clauses is satisfiable. The quantum analogue is the Local Hamiltonian Problem, proved QMA-complete by Kitaev in 1999.

**Definition 38.2.1.** The *k-Local Hamiltonian Problem*: Given a collection $\{H_i\}$ of Hermitian operators each acting on $k$ qubits (of an $n$-qubit system), and real numbers $a < b$ with $b - a \geq 1/\text{poly}(n)$, decide:
- *Yes*: the ground state energy $E_0 = \lambda_{\min}(H) \leq a$
- *No*: $E_0 \geq b$

where $H = \sum_i H_i$ is the total Hamiltonian.

The ground state energy of a Hamiltonian is the lowest eigenvalue — the energy of the most stable configuration. The Local Hamiltonian Problem asks: is this energy below threshold $a$ or above threshold $b$? (If it's between $a$ and $b$, you don't have to answer.)

**Theorem 38.2.2 (Kitaev, 1999).** The 5-Local Hamiltonian Problem is QMA-complete. Subsequently: 2-Local Hamiltonian on a line is QMA-complete (Hallgren-Nagaj-Narayanaswami, 2013).

The analogy with 3-SAT: a SAT clause says "at least one of these 3 variables must be True." A local Hamiltonian term says "the local energy contribution from these $k$ qubits must be in a certain range." The satisfying assignment corresponds to the ground state.

This is not just a theoretical curiosity. It directly connects complexity theory to quantum physics: the problem of finding the ground state energy of a quantum system is QMA-hard. This means quantum physics contains problems that are computationally intractable, even for quantum computers.

Then came an even more dramatic result:

**Theorem 38.2.3 (Cubitt-Pérez-García-Wolf, 2015).** The spectral gap problem (deciding whether a quantum many-body system is gapless or gapped in the thermodynamic limit) is undecidable. This is a phase transition in computability — the gap can "suddenly appear" with no computable criterion.

Not just hard — undecidable. No algorithm, classical or quantum, can decide whether an infinite quantum lattice system is gapped. This is a phase transition from computability to incomputability, and it's sitting right in the middle of condensed matter physics.
