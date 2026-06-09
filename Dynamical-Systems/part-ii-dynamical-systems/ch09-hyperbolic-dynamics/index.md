# Chapter 9 — Hyperbolic Dynamics

> *Hyperbolic systems are the richest, best-understood class of chaotic systems. The theory is essentially complete — and it was built in the 1960s and 70s by Smale, Anosov, Sinai, Ruelle, Bowen, and their contemporaries.*

---

## What This Chapter Is About

There is a class of dynamical systems where chaos is not just present but *organized*. These are the hyperbolic systems, and the theory describing them is a remarkable achievement: it explains why chaotic systems can be coded by symbolic sequences, why numerical simulations of chaotic systems are reliable, and how to identify the "physical" probability measure that governs what a typical observer sees.

The starting point is Smale's horseshoe — a geometric construction from the early 1960s that showed how simple operations (stretch, fold) produce extraordinary complexity. The horseshoe is the archetype of chaos: its orbits are in bijection with binary sequences, its topological entropy is $\log 2$, and its structure is completely described by a Cantor set.

From the horseshoe, Smale extracted the key abstraction: a *hyperbolic set*, where the tangent space splits into contracting and expanding directions. Anosov diffeomorphisms take this to the extreme — the entire manifold is hyperbolic. These systems have unique ergodic measures (the SRB measures), dense periodic orbits, and are structurally stable under perturbation.

The machinery developed to study hyperbolic systems — Markov partitions, symbolic dynamics, SRB measures — became the template for all subsequent work on chaotic systems. Even systems that are not hyperbolic are often studied by asking "how close to hyperbolic are they?"

**Prerequisites:** Chapters 4, 7, 8 (flows, ergodic theory, stability and Lyapunov exponents).

---

## What This Chapter Builds

- The **Smale horseshoe**: the geometric prototype of chaotic dynamics, and its symbolic coding.
- **Hyperbolic sets**: the abstract framework, with examples.
- **Anosov diffeomorphisms**: the extreme case where the entire manifold is hyperbolic.
- The **Shadowing Lemma**: pseudo-orbits (numerical orbits with small errors) are approximated by true orbits.
- **Markov partitions**: the bridge from geometry to symbolic dynamics.
- **SRB measures**: the physically relevant invariant measures for hyperbolic attractors.
- **Axiom A and structural stability**: Smale's classification theorem and the meaning of stability for the whole system.
- **Homoclinic orbits and the genesis of chaos**: how a single transverse intersection generates a horseshoe.
- **Partial hyperbolicity**: what happens when you relax the hyperbolicity conditions.

---

## Sections

1. [The Smale Horseshoe](the-smale-horseshoe.md)
2. [Hyperbolic Sets](hyperbolic-sets.md)
3. [Anosov Diffeomorphisms](anosov-diffeomorphisms.md)
4. [The Shadowing Lemma](the-shadowing-lemma.md)
5. [Markov Partitions](markov-partitions.md)
6. [SRB Measures](srb-measures.md)
7. [Axiom A and Structural Stability](axiom-a-and-structural-stability.md)
8. [Homoclinic Orbits and the Genesis of Chaos](homoclinic-orbits-and-the-genesis-of-chaos.md)
9. [Partial Hyperbolicity](partial-hyperbolicity.md)

---

[Exercises](exercises.md) | [Notes](notes.md)
