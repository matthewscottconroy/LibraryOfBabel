# Chapter 36 — The Zimmer Program

> *Can a lattice in $SL(n, {\mathbb R})$ act on a compact manifold of dimension less than $n-1$? Zimmer conjectured: no. This connects the deepest parts of Lie group theory, ergodic theory, and differential geometry — and was largely resolved by Brown-Fisher-Hurtado in 2020.*

**Prerequisites:** Chapter 7 (ergodic theory, cocycles), Chapter 33 (orbit equivalence, property (T)), Chapter 14 (Hamiltonian systems, Lie groups). Some familiarity with Lie groups helpful.

---

## What This Chapter Is About

The Zimmer conjecture is 35 years old. It was formulated in the mid-1980s by Robert Zimmer, who had just proved his cocycle superrigidity theorem and was trying to understand what it implied for smooth actions of large groups on manifolds.

The conjecture is this: lattices in high-rank Lie groups cannot act smoothly on small manifolds. More precisely, $SL(n, \mathbb{Z})$ — the group of $n \times n$ integer matrices of determinant 1 — cannot act faithfully by smooth volume-preserving diffeomorphisms on any compact manifold of dimension less than $n - 1$.

Why $n - 1$? Because $SL(n, \mathbb{Z})$ acts on the torus $\mathbb{T}^{n-1} = \mathbb{R}^{n-1}/\mathbb{Z}^{n-1}$ by the standard linear action. That's dimension $n-1$. Zimmer asked: can you do better? Can you find a compact manifold of smaller dimension that admits a faithful $SL(n, \mathbb{Z})$-action?

His conjecture: no, you cannot.

Brown, Fisher, and Hurtado proved this in a landmark 2020 paper. The proof uses KAM theory, Lyapunov exponents, and new techniques in "non-stationary normal forms" — a stunning combination of tools that nobody expected.

---

## Sections

- [36.1 Background: Lattices and Lie Groups](lattices-and-lie-groups.md)
- [36.2 The Zimmer Program](zimmer-program.md)
- [36.3 Cocycle Superrigidity](cocycle-superrigidity.md)
- [36.4 The Lyapunov Spectrum and Volume](lyapunov-spectrum-and-volume.md)
- [36.5 The Brown-Fisher-Hurtado Resolution](brown-fisher-hurtado.md)
- [36.6 Connections to Geometric Group Theory](geometric-group-theory-connections.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
