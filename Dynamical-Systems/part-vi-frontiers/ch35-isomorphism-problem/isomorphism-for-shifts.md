# 35.5 The Isomorphism Problem for Shifts

Symbolic dynamics — shifts of finite type, sofic shifts — has its own isomorphism problem. When are two SFTs topologically conjugate? This problem is old, deeply studied, and still mostly open.

**Theorem 35.5.1 (Williams' Theorem Failure).** Williams conjectured (1973) that two SFTs are isomorphic iff their transition matrices are related by "elementary strong shifts equivalence." This was disproved by Kim-Roush (1992).

Williams' conjecture was the central open problem in symbolic dynamics for twenty years. The disproof by Kim and Roush was a major shock — the algebraic condition Williams identified was necessary but not sufficient. The correct classification, if it exists, requires something else.

**Theorem 35.5.2 (Williams' Theorem for Flow Equivalence).** Two irreducible SFTs are *flow equivalent* iff their Bowen-Franks groups coincide: $\text{BF}(A) \cong \text{BF}(B)$ where $\text{BF}(A) = \text{coker}(I - A)$ (cokernel of $I - A: {\mathbb Z}^n \to {\mathbb Z}^n$).

Flow equivalence is a coarser equivalence than topological conjugacy — two shifts are flow equivalent if their suspension flows are homeomorphic. For flow equivalence, Williams' algebraic invariant (the cokernel of $I - A$) is complete. But for topological conjugacy itself, the story is more complicated.

**Theorem 35.5.3 (Sofic Shifts).** The isomorphism problem for sofic shifts (up to topological conjugacy) is undecidable: there is no algorithm that takes two sofic shift presentations and decides if they are conjugate.

**Remark 35.5.4.** The isomorphism problem for SFTs is one of the oldest open problems in symbolic dynamics. In dimension 1, it is open whether there exists a complete invariant. In dimension $\geq 2$, the problem is undecidable (Wang tiling connections from Chapter 25).

Here's where things stand: for 1D SFTs, we have strong algebraic invariants (Bowen-Franks groups, sign-gyration-compatibility conditions from Kim-Roush and Boyle-Huang), but no complete invariant. For 2D SFTs, there's not even hope for an algorithm — the problem is undecidable, connected to the unsolvability of the halting problem via Wang tiles.

This is a domain where measure theory (FRW anti-classification) and symbolic dynamics (Williams conjecture failure, 2D undecidability) converge on the same message: the isomorphism problem for dynamical systems is hard, in multiple precise senses.
