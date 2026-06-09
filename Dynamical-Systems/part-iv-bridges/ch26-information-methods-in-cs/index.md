# Chapter 26 — Information-Theoretic Methods in Computer Science

> *Entropy is not just a measure of uncertainty — it is a combinatorial weapon. Shearer's lemma gives submodular bounds; the entropy method proves the Loomis-Whitney inequality; communication complexity uses information to lower-bound computation. Information pervades the foundations of computer science.*

**Prerequisites:** Chapter 16 (Shannon entropy, mutual information), Chapter 18 (algorithmic information theory), Chapter 25 (chaos and computation).

---

## Overview

The previous chapters used information theory to understand dynamical systems. This chapter goes the other direction: information theory as a tool in pure computer science and combinatorics.

The applications range from elementary to deep. At one end: entropy proves the Loomis-Whitney inequality (a beautiful geometric fact about projections of finite sets) in two lines, via Shearer's lemma. At the other end: communication complexity uses mutual information to prove lower bounds on distributed computation, establishing that some problems require linear communication no matter how clever the protocol.

What unifies these applications is the core property of entropy: it is submodular and satisfies the chain rule. These two facts, combined with the operational interpretation (entropy measures information content), give entropy its combinatorial power. When you need a lower bound on how big something must be, or how much information must be communicated, or how complex a circuit must be — entropy is often the right tool.

This chapter is also, in a sense, a coming-home for the whole book. We started with dynamical systems, crossed into information theory, and used information theory to understand dynamics. Now we see that information theory, at its foundations, is a tool for understanding computation — which is itself a kind of discrete dynamical system. The circle closes.

### What's in this chapter

Section 26.1 develops the entropy method in combinatorics. Shearer's lemma gives the key inequality; applications include the Loomis-Whitney inequality, counting triangle-free graphs, and Turán's theorem.

Section 26.2 develops communication complexity: the model, information complexity as a lower bound tool, and the major results (equality, disjointness, direct sum).

Section 26.3 connects information complexity to circuit lower bounds via the Karchmer-Wigderson theorem.

Section 26.4 covers expander graphs, pseudorandom generators, and randomness extractors — the information-theoretic side of pseudorandomness.

Section 26.5 connects coding theory to dynamical systems, closing the loop on Part IV.

### Sections

- [The Entropy Method in Combinatorics](the-entropy-method-in-combinatorics.md)
- [Communication Complexity](communication-complexity.md)
- [Information Complexity and Circuit Lower Bounds](information-complexity-and-circuit-lower-bounds.md)
- [Expander Graphs and Information](expander-graphs-and-information.md)
- [Coding Theory and Dynamical Systems](coding-theory-and-dynamical-systems.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
