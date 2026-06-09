# Chapter 2 — Measure Theory and Probability

> *Probability is measure theory with a normalization condition. Ergodic theory is the study of measure-preserving maps. Before either, there must be measures.*

---

There's a question lurking behind almost everything in dynamical systems: when we say an orbit "typically" behaves in some way, what do we mean by "typically"? The Baire category theorem from Chapter 1 gives one answer — generic in the topological sense. But that notion doesn't play well with probability or statistics.

Measure theory gives the other answer. A measure is a way of assigning "size" to subsets of a space, consistently enough that you can add up sizes over countable collections of disjoint sets. Probability theory is measure theory where the total size is 1. Ergodic theory is the study of what happens to measures under iteration — when does a dynamical system mix them, and how fast?

This chapter builds the rigorous foundation for all of that. We assume you've seen the material from Chapter 1 — metric spaces, Banach spaces, and the basic topology of limits.

**What this chapter builds:**

The $\sigma$-algebra formalism is the foundation for defining what "measurable" means, and Carathéodory's extension theorem is how we construct measures from simpler data (like the length of intervals). The Lebesgue integral and its convergence theorems — monotone convergence, dominated convergence, Fatou's lemma — are the computational workhorses of analysis. The $L^p$ spaces built from the integral are the Banach spaces on which dynamical systems act. The Radon-Nikodym theorem and conditional expectation are essential for entropy theory. Prokhorov's theorem is the compactness theorem for probability measures and the primary tool for proving that invariant measures exist. And the law of large numbers is the classical predecessor of the Birkhoff Ergodic Theorem, which is one of the central results in Part II.

**Sections:**

- [2.1 σ-Algebras and Measurable Spaces](sigma-algebras-and-measurable-spaces.md)
- [2.2 Measures and Their Construction](measures-and-their-construction.md)
- [2.3 Integration](integration.md)
- [2.4 $L^p$ Spaces](lp-spaces.md)
- [2.5 Radon-Nikodym Theorem](radon-nikodym-theorem.md)
- [2.6 Probability Theory](probability-theory.md)
- [2.7 The Law of Large Numbers and Its Ergodic Generalization](law-of-large-numbers.md)
- [2.8 Measures on Topological Spaces](measures-on-topological-spaces.md)
- [2.9 Product Measures and Fubini's Theorem](product-measures-and-fubini.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
