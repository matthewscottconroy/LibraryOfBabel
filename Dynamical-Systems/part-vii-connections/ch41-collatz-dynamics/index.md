# Chapter 41 — The Collatz Conjecture as a Dynamical System

> *Take any positive integer. If even, halve it. If odd, triple it and add one. Repeat. The conjecture: you always reach 1. This is the simplest unsolved problem in mathematics — and it is a problem about the ergodic theory of a number-theoretic dynamical system.*

**Prerequisites:** Chapter 6 (topological dynamics), Chapter 7 (ergodic theory), Chapter 31 (ergodic number theory), Chapter 27 (computability and dynamics).

---

## What This Chapter Is About

The Collatz conjecture is 87 years old and still open. Paul Erdős said: "Mathematics is not yet ready for such problems." Every positive integer anyone has ever tried — all $2^{68}$ of them and more — eventually reaches 1. But we cannot prove it.

Part of what makes the Collatz conjecture hard is that it mixes two different mathematical worlds. It's a statement about positive integers — number theory. But the natural framework for understanding it is ergodic theory and $p$-adic analysis. The Collatz map extends continuously to the 2-adic integers, and on that completion, it is measure-preserving and ergodic. But the positive integers are a measure-zero subset of the 2-adic integers, and ergodicity on the 2-adic integers says nothing (directly) about what happens on positive integers.

The biggest breakthrough in decades came from Terence Tao in 2022. He proved that almost all Collatz orbits attain almost bounded values: for any function $f(n) \to \infty$, however slowly, a density-1 set of starting values $n$ has an orbit that reaches below $f(n)$. This is the strongest quantitative result ever proved for the Collatz conjecture.

The gap between Tao's theorem and the full conjecture: Tao's result says orbits reach "small" values, but the full conjecture requires reaching exactly 1. And nobody knows how to bridge that gap.

---

## Sections

- [41.1 The Collatz Map: Definitions and Basic Properties](collatz-map-basics.md)
- [41.2 2-Adic Extension and Ergodicity](2-adic-extension.md)
- [41.3 Statistical Model of the Collatz Map](statistical-model.md)
- [41.4 Tao's Progress (2022)](tao-progress.md)
- [41.5 Connections to Other Dynamical Systems](connections-other-systems.md)
- [41.6 Open Research Directions](open-research-directions.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
