# Chapter 7 — Ergodic Theory

> *Time averages equal space averages — when the system is ergodic. This is not a physical intuition but a theorem, and the proof reveals the mathematical structure underlying statistical mechanics, information theory, and number theory.*

---

## What This Chapter Is About

In 1931, George David Birkhoff proved a theorem that reorganized how mathematicians and physicists think about statistical equilibrium. The theorem says: for a system that preserves a probability measure and satisfies a certain irreducibility condition (ergodicity), the time average of any observable equals its space average — and this holds for almost every starting point.

This is the ergodic hypothesis, made rigorous. It's not obvious that it should be true. The time average is computed along a single orbit; the space average is computed over the whole space. For these to be equal, the single orbit has to "sample" the entire space in the right proportions. When that happens, we say the system is ergodic.

This chapter builds the theory from the ground up. We start with the central objects — measure-preserving transformations — and their fundamental examples. We prove both the von Neumann mean ergodic theorem (convergence in $L^2$) and Birkhoff's pointwise theorem (convergence almost everywhere). We characterize ergodicity in several equivalent ways, then move on to mixing (a stronger form of irreducibility). We introduce the Koopman operator, which turns dynamics into functional analysis, and use its spectral theory to classify mixing properties. We then define and compute Kolmogorov-Sinai entropy — the measure-theoretic isomorphism invariant — and state Ornstein's theorem: entropy *completely* classifies Bernoulli shifts. The chapter ends with joinings, a flexible framework for comparing two dynamical systems.

**Prerequisites:** Chapters 2 (measure theory, $L^p$ spaces, conditional expectation) and 6 (topological dynamics, invariant measures).

---

## What This Chapter Builds

- **Measure-preserving transformations** as the central object, with a gallery of examples.
- The **Poincaré Recurrence Theorem** in its measure-theoretic form (much sharper than the topological version).
- **Von Neumann's Mean Ergodic Theorem** and **Birkhoff's Pointwise Ergodic Theorem** — the two fundamental convergence results.
- **Ergodicity** and its multiple equivalent characterizations.
- **Mixing and weak mixing** as spectral properties of the Koopman operator.
- **Kolmogorov-Sinai entropy** — the dynamical analog of Shannon entropy.
- **Ornstein's classification theorem** — entropy is a complete invariant for Bernoulli shifts.
- **Joinings** — the language of independence and coupling between dynamical systems.

---

## Sections

1. [Measure-Preserving Transformations](measure-preserving-transformations.md)
2. [Poincaré Recurrence Theorem](poincare-recurrence-theorem.md)
3. [The Ergodic Theorems](the-ergodic-theorems.md)
4. [Ergodicity](ergodicity.md)
5. [Mixing](mixing.md)
6. [The Koopman Operator and Spectral Theory](the-koopman-operator-and-spectral-theory.md)
7. [Entropy](entropy.md)
8. [Ornstein Theory](ornstein-theory.md)
9. [Joinings](joinings.md)

---

[Exercises](exercises.md) | [Notes](notes.md)
