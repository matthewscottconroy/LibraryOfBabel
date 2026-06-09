# Chapter 5 — Notes

---

The Jordan canonical form is covered in Halmos' *Finite Dimensional Vector Spaces* — still one of the best books on the subject, despite its age. Halmos writes in a conversational style that makes the abstract feel concrete. Hoffman and Kunze's *Linear Algebra* is the other classic reference, more comprehensive and more systematic.

For the spectral theorem on infinite-dimensional Hilbert spaces, Rudin's *Functional Analysis* (Chapter 12) is the authoritative reference. It's dense, but everything is there. Reed and Simon's *Methods of Mathematical Physics, Vol. 1* (Chapters 6-7) covers the spectral theorem with more motivation and more attention to the subtleties of unbounded operators — which become important in quantum mechanics, where most observables are unbounded.

Horn and Johnson's *Matrix Analysis* is the comprehensive reference for matrix theory — singular values, inequalities, perturbation theory, and much more. It's a book you'll consult rather than read straight through, but it's invaluable to have on the shelf.

For Perron-Frobenius (Section 5.6): the finite-dimensional theorem is classical and covered in most advanced linear algebra books. The infinite-dimensional Ruelle-Perron-Frobenius theorem — which governs the spectral properties of transfer operators for expanding and hyperbolic maps — is covered in Katok-Hasselblatt's *Introduction to the Modern Theory of Dynamical Systems* (Chapter 14) and in Baladi's *Positive Transfer Operators and Decay of Correlations*. The latter is the definitive reference for the functional analytic approach to mixing rates.

Tensor products (Section 5.7) connect to two separate literatures. For the quantum information side — entanglement, quantum channels, Schmidt decomposition — Nielsen and Chuang's *Quantum Computation and Quantum Information* is the standard reference and very well written. For the dynamical systems side — Lyapunov exponents via exterior algebra — the key reference is Oseledets' original paper and its exposition in Katok-Hasselblatt.

One thing this chapter doesn't develop: the theory of *unbounded* operators, which is essential for quantum mechanics (where position and momentum operators are unbounded) and for PDEs (where differential operators are unbounded). If you need it, the right reference is Reed-Simon, Vol. 1-2.

The spectral gap mentioned in Section 5.4 is one of the central quantitative measures in dynamics: it determines exponential mixing rates, CLT variance, rates of decay of correlations. Much of the research in smooth ergodic theory since the 1990s has focused on computing and bounding spectral gaps for specific classes of maps. Sections 5.4 and 5.5 give you the vocabulary; Chapters 7, 12, and 13 develop the theory.
