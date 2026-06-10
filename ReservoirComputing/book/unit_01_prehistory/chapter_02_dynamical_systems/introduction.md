# Chapter 2: Dynamical Systems — The Mathematics of Change Over Time

## Introduction

There is a particular kind of satisfaction that comes from watching a system evolve. Not from predicting it — though prediction matters — but from *understanding the geometry of how things change*. A pendulum swings and returns. A population of rabbits overshoots and crashes. A column of heated fluid suddenly breaks into swirling rolls. In each case, something is changing according to rules, and those rules carve out paths through an abstract space of possibilities.

This is what a dynamical system is: a set of rules for how a state changes over time.

That definition sounds almost trivially simple. But it conceals extraordinary depth. The universe itself is, at one level of description, a dynamical system — a colossal state vector evolving under the laws of physics, tracing an unimaginably long trajectory through phase space. Your brain is a dynamical system: roughly 86 billion neurons, each with a continuously varying voltage, interacting through synaptic weights that are themselves slowly changing. A reservoir computer is also a dynamical system — one that we deliberately design, or grow, or discover, and then *listen to* in order to read off information about the world.

That last sentence is worth pausing on. The deep insight of reservoir computing is not algorithmic: it is geometric. A reservoir is a dynamical system that is *driven* by an input signal, and the richness of the reservoir's state space — the fact that it contains many dimensions and complex transient dynamics — means that the history of the input is somehow encoded in the present state of the reservoir. To understand why this works, and when it fails, and how to make it work better, you need the language of dynamical systems.

This chapter builds that language carefully. We do not assume you have seen this material before. We will assume you are comfortable with calculus, linear algebra, and the basic flavor of differential equations. If some of those prerequisites are rusty, the material will sharpen them.

---

## The Central Metaphor

Imagine you are given a universe — not ours, but a small one, a mathematical toy. This universe has a *state*: a list of numbers $x = (x_1, x_2, \ldots, x_n)$ that describes everything about the universe at a given instant. The universe evolves according to a law, which is just a function $f$ that tells you how the state changes:

$$\frac{d}{dt} x(t) = f(x(t))$$

or, in discrete time:

$$x_{t+1} = f(x_t)$$

You know $f$. You know the initial state $x(0)$. From these, in principle, you can compute the entire future: $x(1), x(2), \ldots$, or $x(t)$ for any $t > 0$. The system is *deterministic*. There is no randomness, no free will, no intervention from outside.

And yet — and this is the miracle that the twentieth century slowly revealed — deterministic systems can produce behavior that looks, for all practical purposes, indistinguishable from random noise. They can be *sensitive* to their initial conditions in such a precise mathematical sense that predictions beyond a certain horizon become impossible, even in principle, unless you know the initial state with infinite precision. This is chaos. And chaos is not a breakdown of the law. Chaos *is* the law, operating without apology.

Now here is where the reservoir computing metaphor enters.

When you drive a reservoir with an input signal $u(t)$, you are adding a *perturbation* to this small universe. The reservoir's state $x(t)$ is no longer free to wander wherever its own dynamics take it — it is being *pushed* by the input, continuously. If the reservoir is designed well, its response to the input is *functional*: the current state $x(t)$ is a rich, nonlinear function of the entire history $u(t), u(t-1), u(t-2), \ldots$. The reservoir has become, in a sense, a *memory device* — not because we designed memory explicitly, but because the geometry of its dynamics compresses and encodes temporal structure.

The reservoir is a universe we design. We choose its laws ($f$), its dimension, its connectivity, its nonlinearities. We then listen to what the universe has to say about the input it has experienced.

---

## What This Chapter Covers

To understand reservoirs, we need several foundational ideas from the theory of dynamical systems. This chapter builds them in order.

**Section 1** introduces continuous-time and discrete-time dynamical systems at the level of definitions and examples. We look at pendulums, RC circuits, and population models. We introduce the phase portrait as the right geometric object for visualizing trajectories.

**Section 2** introduces the fixed points of a dynamical system — states where nothing changes — and the question of whether nearby trajectories move toward or away from these states. This leads us to linearization and the Jacobian matrix, and from there to eigenvalue stability analysis. This is the mathematical backbone of understanding when a reservoir will "settle" into a useful operating regime.

**Section 3** moves beyond fixed points to limit cycles — oscillatory attractors. The van der Pol oscillator appears here as a canonical example.

**Section 4** is the heart of the chapter for most readers: chaos and strange attractors. Edward Lorenz's 1963 paper on convection [Lorenz1963] described a three-dimensional ODE system that became the canonical example of deterministic chaos. We work through the Lorenz system in detail, introduce Lyapunov exponents as the precise measure of sensitivity to initial conditions, and discuss the fractal geometry of strange attractors.

**Section 5** covers bifurcations — moments where the qualitative behavior of a system changes suddenly as a parameter is varied. The logistic map's period-doubling cascade [Feigenbaum1978] is the central example.

**Section 6** examines attractors and their basins more carefully, with particular attention to multistability.

**Section 7** is the bridge to reservoir computing: input-driven dynamical systems, generalized synchronization [Pecora1990], and the echo state property [Jaeger2001]. By the end of Section 7, you will be able to state, in precise mathematical language, exactly what a reservoir computer is and why it works.

---

## Why This Matters for Machine Learning Engineers

If you come from a machine learning background, you may be wondering why a textbook on reservoir computing spends a full chapter on differential equations and chaos. The answer is that the language of dynamical systems is not decoration here — it is the primary tool of analysis.

When you ask whether a reservoir will generalize well, you are asking a question about attractors and their basins. When you ask whether your network's state is responding to input or has drifted off on its own trajectory, you are asking about the balance between the autonomous dynamics and the driven dynamics — a question with precise answers in terms of Lyapunov exponents. When you ask how far back in time your reservoir can "remember" an input, you are asking about the echo state property, which is itself a consequence of the contraction of phase space volume.

None of these questions have purely algorithmic answers. They are geometric questions, and their answers live in the phase space of the reservoir.

Take the time to learn this material. It will repay you many times over.

---

## A Note on Notation

Throughout this chapter, we use the following conventions:

- **State vectors** are bold lowercase: $\mathbf{x} \in \mathbb{R}^n$. For scalar systems we use plain $x$.
- **Time derivatives** are denoted $\dot{x} = dx/dt$ in continuous time.
- **Maps** use subscript notation: $x_{t+1} = f(x_t)$.
- **Equilibria and fixed points** are denoted with a star: $x^*$.
- **Jacobian matrices** are denoted $Df$ or $J$, with entries $J_{ij} = \partial f_i / \partial x_j$.
- **Lyapunov exponents** are denoted $\lambda_i$, ordered from largest to smallest.

When we want to be explicit that a quantity depends on a parameter (for bifurcation analysis), we write $f(x; \mu)$ or $f_\mu(x)$.

---

*The mathematics of dynamical systems is the mathematics of how things change. And things changing — states evolving, trajectories bending, attractors forming — is exactly what happens inside every reservoir computer. Let's begin.*
