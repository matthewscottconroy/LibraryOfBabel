# Chapter 25 — Chaos, Randomness, and Computation

> *Positive Lyapunov exponent = exponential divergence of orbits = information production. A chaotic system generates one bit per $1/\lambda$ time units. This is why long-term prediction is impossible and why chaos can be a source of pseudo-randomness.*

**Prerequisites:** Chapters 11 (chaos, Lyapunov exponents), 18 (algorithmic randomness, ML-randomness).

---

## Overview

Chaos and randomness look alike but they're not the same thing. A chaotic system is completely deterministic: given the initial condition, the orbit is fixed forever. A random process has genuine uncertainty. And yet chaotic orbits are, in a precise sense, just as unpredictable as random sequences.

The connection runs through information theory. A chaotic system with Lyapunov exponent $\lambda > 0$ produces information at rate $\lambda$ bits per time unit — not metaphorically, but in the sense of Pesin's formula ($h_\mu = \sum_{\lambda_i > 0} \lambda_i$). This information production is what makes long-term prediction impossible: to predict an orbit to accuracy $\varepsilon$ at time $T$, you need initial condition accuracy $\varepsilon e^{-\lambda T}$ — an exponentially fine specification, requiring $\lambda T / \log 2$ additional bits of precision.

The chapter also explores what happens at the boundary between chaos and computability. The algorithmic randomness of Chapter 18 — Martin-Löf randomness, Kolmogorov complexity — applies to the orbits of chaotic systems. For Lebesgue-almost-every initial condition, the symbolic orbit of the doubling map is ML-random. But if the initial condition is computable, the orbit is periodic and trivially not random. Chaos is a randomness generator only if you start with genuine randomness.

This leads to the deepest questions in the chapter: what dynamical properties are undecidable? What can be computed about a dynamical system, and what cannot? The answers involve Wang tiles, cellular automata, 2D subshifts, and the halting problem — a genuine convergence of dynamics and theoretical computer science.

### What's in this chapter

Section 25.1 establishes the central identity: chaos = information production, with precise rate given by the positive Lyapunov exponents.

Section 25.2 analyzes pseudo-randomness from chaotic maps — when chaos generates usable random bits, and when it doesn't.

Section 25.3 connects chaos to algorithmic randomness: which initial conditions give ML-random orbits? The effective Birkhoff theorem answers this precisely.

Section 25.4 presents the undecidability results for dynamical systems: properties that no algorithm can decide for all inputs.

Section 25.5 develops computable analysis for dynamical systems: what can and cannot be computed about Julia sets, Mandelbrot sets, and other fractal invariants.

Section 25.6 surveys computational complexity of dynamical quantities: how hard is it to compute entropy, Lyapunov exponents, and periodic point counts?

### Sections

- [Chaos as Information Production](chaos-as-information-production.md)
- [Pseudo-Randomness from Chaotic Maps](pseudo-randomness-from-chaotic-maps.md)
- [Algorithmic Randomness of Chaotic Orbits](algorithmic-randomness-of-chaotic-orbits.md)
- [Undecidability in Dynamical Systems](undecidability-in-dynamical-systems.md)
- [Computable Analysis and Dynamical Systems](computable-analysis-and-dynamical-systems.md)
- [Computational Complexity of Dynamical Properties](computational-complexity-of-dynamical-properties.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
