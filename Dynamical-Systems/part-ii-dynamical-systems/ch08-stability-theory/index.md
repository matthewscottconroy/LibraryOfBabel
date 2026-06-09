# Chapter 8 — Stability Theory and Lyapunov Methods

> *Lyapunov's direct method asks: can you find a function that decreases along trajectories? If yes, the system is stable — without solving the equations.*

---

## What This Chapter Is About

The central question in stability theory is deceptively simple: if you start near an equilibrium and perturb the system slightly, do you stay near the equilibrium, or do you drift away?

The answer depends entirely on the system, and finding it by solving the equations of motion is usually impossible. Lyapunov's great insight — formalized in his 1892 thesis — was that you don't need to solve the equations. You only need to find a function that decreases along trajectories. If such a function exists and is shaped like a bowl around the equilibrium, then the trajectories can't climb out of the bowl. The equilibrium is stable.

This "direct method" (as Lyapunov called it, to distinguish it from linearization) transforms stability analysis into a search for a useful function. It sounds like a trick, and in some ways it is — but it's a trick that works over an enormous range of problems, from engineering control systems to nonlinear PDEs to billiards.

This chapter develops Lyapunov's theory for both continuous-time and discrete-time systems. We prove the basic stability theorem, then LaSalle's invariance principle (which handles the case when the Lyapunov function doesn't strictly decrease). We discuss converse theorems — which guarantee a Lyapunov function exists whenever the equilibrium is stable, making the method not just sufficient but necessary. We then develop Lyapunov exponents, which generalize eigenvalues to nonlinear systems and measure the asymptotic rate of separation of nearby trajectories. And we connect everything back to the ergodic theory of Chapter 7 through Pesin's formula, which equates entropy with the sum of positive Lyapunov exponents.

**Prerequisites:** Chapter 4 (ODEs, flows, equilibria) and Chapter 5 (spectral theory).

---

## What This Chapter Builds

- **Stability definitions**: Lyapunov stable, asymptotically stable, exponentially stable, GAS.
- **Lyapunov's direct method**: the main stability theorem and its proof.
- **LaSalle's invariance principle**: extending Lyapunov to the case of non-strict decrease.
- **Converse Lyapunov theorems**: stability is equivalent to the existence of a Lyapunov function.
- **Lyapunov exponents and the Oseledec theorem**: the nonlinear, global generalization of eigenvalues.
- **Pesin's formula**: entropy equals the sum of positive Lyapunov exponents.
- **Floquet theory**: stability of periodic orbits.

---

## Sections

1. [Stability Definitions](stability-definitions.md)
2. [Lyapunov's Direct Method](lyapunovs-direct-method.md)
3. [LaSalle's Invariance Principle](lasalles-invariance-principle.md)
4. [Converse Lyapunov Theorems](converse-lyapunov-theorems.md)
5. [Lyapunov Exponents](lyapunov-exponents.md)
6. [Stability of Periodic Orbits — Floquet Theory](stability-of-periodic-orbits-floquet-theory.md)
7. [Stability in Discrete-Time Systems](stability-in-discrete-time-systems.md)

---

[Exercises](exercises.md) | [Notes](notes.md)
