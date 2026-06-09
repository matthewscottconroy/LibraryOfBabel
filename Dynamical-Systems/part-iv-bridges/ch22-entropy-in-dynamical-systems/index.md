# Chapter 22 — Entropy in Dynamical Systems

> *The entropy of a dynamical system is Shannon entropy applied to the orbit structure. The variational principle says: topological entropy is the maximum, achieved by the measure of maximal entropy. This is the deepest single theorem connecting the two pillars of this book.*

**Prerequisites:** Chapter 7 (ergodic theory, KS entropy), Chapter 12 (symbolic dynamics, topological entropy), Chapter 16 (Shannon entropy, AEP).

---

## Overview

Here is the payoff.

For eleven chapters, we built the machinery of dynamical systems: flows, maps, attractors, Lyapunov exponents, symbolic dynamics, SFTs, ergodic measures. For six more chapters, we built the machinery of information theory: entropy, mutual information, channel capacity, source coding, AEP. Two pillars, constructed side by side, each internally coherent, each with its own theorems and its own vocabulary.

In Part IV, we find out they were describing the same thing.

This chapter makes the connection precise. The central theorem — the variational principle — says that the topological entropy of a dynamical system is the supremum of the KS entropy over all invariant measures. Topological entropy is a geometric property of the map; KS entropy is a measure-theoretic property of an orbit distribution. The variational principle says they're related by exactly the same duality as channel capacity and mutual information in Shannon's theory.

This isn't a metaphor. It's a theorem.

The chapters that follow will deepen this connection. Chapter 23 proves that the Shannon-McMillan-Breiman theorem — the almost-sure version of the AEP — is literally Birkhoff's ergodic theorem applied to the information function. Chapter 24 builds out the complete symbolic dynamics / information theory dictionary. But the heart of everything is here: orbit counting is information counting.

### What's in this chapter

We begin in Section 22.1 with the dictionary — a table matching each dynamical concept to its information-theoretic twin. Don't just read it; absorb it. Every row will be a theorem by the end of the book.

Section 22.2 introduces Bowen's definition of topological entropy, which makes the information-theoretic content explicit: $h_{\text{top}}$ counts the exponential growth rate of distinguishable orbits, measured at vanishing resolution. This is the number of bits per unit time needed to specify an orbit.

Section 22.3 proves the variational principle. This is the main event.

Sections 22.4 and 22.5 develop the theory of measures of maximal entropy and the pressure function. The pressure function generalizes entropy by allowing a "potential" — a real-valued weighting on orbits — and its Legendre structure connects directly to thermodynamics and the theory of Gibbs states.

Section 22.6 is Pesin's entropy formula: the KS entropy equals the sum of positive Lyapunov exponents (for smooth invariant measures). This is the statement that information production rate equals orbit-divergence rate — a theorem connecting two of the most important quantities in the subject.

Section 22.7 closes with zeta functions and thermodynamic formalism: the dynamical analogue of the Riemann zeta function, where periodic orbits play the role of primes.

### Sections

- [The Dictionary: Dynamics ↔ Information](the-dictionary.md)
- [Topological Entropy — Bowen's Definition](topological-entropy-bowens-definition.md)
- [The Variational Principle](the-variational-principle.md)
- [Measures of Maximal Entropy](measures-of-maximal-entropy.md)
- [The Pressure Function](the-pressure-function.md)
- [Pesin's Entropy Formula](pesins-entropy-formula.md)
- [Zeta Functions and Thermodynamic Formalism](zeta-functions-and-thermodynamic-formalism.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
