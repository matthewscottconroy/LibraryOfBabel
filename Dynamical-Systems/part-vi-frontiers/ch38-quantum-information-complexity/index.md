# Chapter 38 — Quantum Information and Computational Complexity

> *P vs NP is the central question of classical complexity theory. QMA is its quantum analogue. The local Hamiltonian problem (QMA-complete) is the quantum version of 3-SAT. Quantum entanglement creates complexity — and complexity theory is the new thermodynamics.*

**Prerequisites:** Chapter 21 (quantum information, density matrices, channels), Chapter 26 (communication complexity, circuit complexity), Chapter 18 (algorithmic information theory).

---

## What This Chapter Is About

In 2020, Ji, Natarajan, Vidick, Wright, and Yuen proved MIP\* = RE. This is one of the most stunning results in the history of complexity theory.

What MIP\* = RE is actually saying: quantum provers with unlimited shared entanglement can convince a classical verifier of anything in RE — the recursively enumerable sets, the same complexity class as the halting problem. This is dramatically more powerful than anyone expected. Classical interactive proofs give you PSPACE. Classical interactive proofs with multiple provers (MIP) give you NEXP. But quantum entanglement between the provers boosts this all the way to RE — everything that's computable at all.

And MIP\* = RE implies Connes' embedding conjecture is false. A conjecture about von Neumann algebras that many people believed, tested against many examples over 40 years, turns out to be wrong — and the proof came from complexity theory.

This chapter covers quantum complexity classes (QMA, the quantum analogue of NP), the local Hamiltonian problem (QMA-complete, the quantum version of 3-SAT), quantum algorithms (Shor and Grover), and the connections between entanglement entropy and quantum complexity. Quantum channels as dynamical systems appear at the end — connecting back to ergodic theory via Lindblad equations.

---

## Sections

- [38.1 Quantum Complexity Classes](quantum-complexity-classes.md)
- [38.2 The Local Hamiltonian Problem](local-hamiltonian-problem.md)
- [38.3 Quantum PCP and Entanglement](quantum-pcp-entanglement.md)
- [38.4 Quantum Advantage: Shor and Grover](quantum-advantage.md)
- [38.5 Quantum Entanglement and Complexity](entanglement-and-complexity.md)
- [38.6 Quantum Channels as Dynamical Systems](quantum-channels-dynamics.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
