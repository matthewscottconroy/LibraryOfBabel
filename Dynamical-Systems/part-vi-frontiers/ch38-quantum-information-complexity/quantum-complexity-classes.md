# 38.1 Quantum Complexity Classes

The classical complexity hierarchy — P, NP, PSPACE, EXP — has a quantum counterpart. BQP (bounded-error quantum polynomial time) is the quantum version of P: problems solvable efficiently on a quantum computer. QMA is the quantum version of NP.

**Definition 38.1.1 (QMA — Quantum Merlin-Arthur).** $L \in$ QMA if there is a polynomial-time quantum verifier $V$ such that:
- (Completeness) If $x \in L$: $\exists$ quantum proof $|\psi\rangle$ with $P(V \text{ accepts}) \geq 2/3$
- (Soundness) If $x \notin L$: $\forall$ quantum proofs $|\psi\rangle$: $P(V \text{ accepts}) \leq 1/3$

QMA is the quantum analogue of NP (where the proof is classical and the verifier is classical).

The name "Quantum Merlin-Arthur" comes from the interactive proof model: Merlin (all-powerful) sends a proof; Arthur (efficient) verifies it. In QMA, the proof is a quantum state.

**Hierarchy:**
$$\text{BPP} \subseteq \text{BQP} \subseteq \text{QMA} \subseteq \text{PP} \subseteq \text{PSPACE}.$$

**Theorem 38.1.2.** BQP $\neq$ NP is not known (but widely believed). BQP $\neq$ BPP is not proven. Quantum speedups (Shor, Grover) show BQP contains problems not known to be in P.

Notice what we don't know: we can't prove quantum computers are more powerful than classical computers. We have strong evidence (Shor's algorithm factors efficiently; classical factoring is believed hard), but no proof. This is a fundamental limitation of current complexity theory — we can't prove separations between most complexity classes.

The placement of QMA relative to PP is interesting: quantum proofs are verified by quantum computers in polynomial time, but the class still lives below PSPACE. The power of quantum proofs is real but bounded.
