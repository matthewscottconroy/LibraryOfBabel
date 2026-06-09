# Chapter 21 — Quantum Information Theory

> *Quantum information theory is the extension of Shannon's theory to quantum mechanics. The key insight: quantum states carry more information than classical states — and the rules for manipulating that information are fundamentally different.*

**Prerequisites:** Chapter 5 (Hilbert spaces, tensor products), Chapter 16 (classical information theory), Chapter 17 (von Neumann entropy).

---

Shannon's theory is about bits — discrete, classical, perfectly readable. Quantum mechanics offers something richer: *qubits*, which can be in superpositions of 0 and 1, can be entangled with other qubits, and collapse to classical bits only upon measurement. The rules for manipulating quantum information are different from the classical rules — sometimes more powerful, sometimes more restricted.

Quantum information theory asks: what are the capacity limits of quantum systems for storing and transmitting information? The question has the same shape as Shannon's but the answers are genuinely new. Quantum compression (Schumacher's theorem) achieves rates down to the von Neumann entropy — exactly as you would hope. Quantum channel capacity requires a regularization over multiple channel uses and is still not fully understood. Entanglement is a resource that has no classical analogue, and its operational theory — how much entanglement can you distill from a state, how much do you need to create it — is a rich subject.

Throughout, the technical heart is the strong subadditivity of von Neumann entropy (introduced in Chapter 17), which drives most of the positive results in quantum information theory. The negative results — the impossibility of certain quantum operations — follow from more structural arguments about quantum mechanics.

This chapter assumes you are comfortable with basic quantum mechanics at the level of Hilbert spaces, density matrices, and unitary evolution. The presentation is information-theoretic rather than physics-first: we care about what quantum systems can do computationally and communicationally, not about their physical interpretation.

**What this chapter builds:**
- Density matrices, quantum channels, and entanglement
- Schumacher's quantum source coding theorem
- Holevo's bound and the HSW classical capacity theorem
- The LSD theorem for quantum capacity
- Entanglement theory: measures, distillation, and dilution
- Quantum error correction and stabilizer codes
- Strong subadditivity and the quantum data processing inequality
- The Hastings counterexample to channel additivity

**Sections:**
- [21.1 Quantum States and Operations](quantum-states-and-operations.md)
- [21.2 Quantum Source Coding — Schumacher's Theorem](schumacher-theorem.md)
- [21.3 Quantum Channel Capacity](quantum-channel-capacity.md)
- [21.4 Entanglement Theory](entanglement-theory.md)
- [21.5 Quantum Error Correction](quantum-error-correction.md)
- [21.6 Quantum Information Inequalities](quantum-information-inequalities.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
