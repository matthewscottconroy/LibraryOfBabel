# Chapter 31 — Ergodic Methods in Number Theory

> *The primes are equidistributed in arithmetic progressions (Dirichlet). The digits of $\pi$ are normal (expected, not proved). The Collatz conjecture is about ergodic properties of a number-theoretic map. Dynamical systems methods — equidistribution, recurrence, combinatorial number theory — have transformed additive combinatorics.*

**Prerequisites:** Chapter 7 (ergodic theory, Birkhoff's theorem), Chapter 6 (topological dynamics, equicontinuity), Chapter 9 (symbolic dynamics).

---

Number theory and dynamical systems feel like distant subjects. One is about the integers — a discrete, algebraic structure. The other is about continuous evolution in time. Yet some of the deepest results in number theory have been proved by ergodic-theoretic methods, and the connection runs both ways: number-theoretic questions have motivated central developments in ergodic theory.

The bridge is equidistribution. When you ask "how often does the sequence $n\alpha \pmod 1$ visit the interval $[1/3, 2/3)$?", you're asking about the long-run statistics of an orbit. Birkhoff's theorem says orbits spend time proportional to measure. Weyl's theorem says irrational rotations visit every interval proportionally. These are ergodic-theoretic facts, but their number-theoretic content is: for irrational $\alpha$, the sequence $(n\alpha)$ mod 1 is uniformly distributed.

Furstenberg's 1977 correspondence principle is the most striking step in this direction. It converts questions about arithmetic progressions in subsets of the integers — the subject of Szemerédi's theorem — into questions about return times of orbits in dynamical systems. Once you're in the dynamical setting, the tools of ergodic theory (mixing, multiple recurrence, compact extensions) become available. Furstenberg proved Szemerédi's theorem this way, and Green and Tao extended the method to prove the primes contain arbitrarily long arithmetic progressions.

This chapter traces that journey: from Weyl's classical theorem, through Furstenberg's correspondence principle and Szemerédi's theorem, to the Green-Tao theorem and the Collatz conjecture.

---

## Sections

- [31.1 — Equidistribution and Weyl's Theorem](equidistribution-and-weyls-theorem.md)
- [31.2 — Furstenberg's Correspondence Principle](furstenbergs-correspondence-principle.md)
- [31.3 — Normal Numbers](normal-numbers.md)
- [31.4 — The Collatz Map as Ergodic System](the-collatz-map-as-ergodic-system.md)
- [31.5 — van der Waerden, Hales-Jewett, and Recurrence](van-der-waerden-hales-jewett-and-recurrence.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
