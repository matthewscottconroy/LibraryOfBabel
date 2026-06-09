# Chapter 27 — Computability Theory and Dynamical Systems

> *Every dynamical system is a computation. Every computation is a dynamical system. The Church-Turing thesis and the undecidability of the halting problem are theorems about dynamical systems — about what happens in the long run.*

**Prerequisites:** Chapter 18 (algorithmic information theory, Turing machines), Chapter 25 (undecidability in dynamics).

---

Here is a fact that took the theoretical computer science community a surprisingly long time to say out loud: a Turing machine is a dynamical system. Not metaphorically. The tape is the state space, the read/write head and internal registers form a compact control layer, and the step function — the machine's one-tick transition — is a continuous map on a product topology. Computation is iteration. The orbit of an initial configuration is the trace of a computation.

Once you see this, computability theory becomes a chapter of dynamics, and dynamics becomes a chapter of computability theory. The halting problem asks whether the orbit of a given point ever enters a certain target set. Rice's theorem says that any nontrivial asymptotic property of an orbit is undecidable. The arithmetic hierarchy — $\Sigma_1^0$, $\Pi_1^0$, $\Sigma_2^0$, and beyond — turns out to be exactly the right language for classifying how dynamical properties behave computationally.

This chapter builds that dictionary, then uses it. We begin with the formal connection between Turing machines and dynamical systems, move through cellular automata (where the connection becomes visually obvious), and then ask: what does it mean for an invariant measure to be computable? What is the Turing degree of the Mandelbrot set boundary? Where does Kleene's recursion theorem fit in the story of self-replication?

The short answer to all of these: the connections run deep, and understanding them forces you to be precise about what "long-time behavior" can and cannot be computed.

---

## Sections

- [27.1 — Turing Machines as Dynamical Systems](turing-machines-as-dynamical-systems.md)
- [27.2 — Computability of Invariant Measures](computability-of-invariant-measures.md)
- [27.3 — The Recursion Theorem and Fixed Points](the-recursion-theorem-and-fixed-points.md)
- [27.4 — Computable Real Analysis and Dynamics](computable-real-analysis-and-dynamics.md)
- [27.5 — Formal Verification of Dynamical Properties](formal-verification-of-dynamical-properties.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
