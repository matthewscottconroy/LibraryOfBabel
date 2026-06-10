# Chapter 1 — Key Concepts

---

## Causal Functional

A mapping $H$ from sequences of inputs $\mathbf{u} = (\ldots, u_{t-1}, u_t)$ to sequences of outputs $\mathbf{y} = (\ldots, y_{t-1}, y_t)$ such that $y_t$ depends only on inputs up to and including time $t$. The central object of study in temporal computation. Feedforward networks can only compute causal functionals of finite window width; dynamical systems can compute causal functionals of potentially infinite (but fading) memory.

---

## Fading Memory

A property of a causal functional: the influence of past inputs on the current output decays as those inputs recede in time. Formally, two input sequences that agree on recent inputs but differ on distant past inputs produce nearly identical outputs. The rate of decay is characterized by a weighting sequence $w_k \to 0$.

Fading memory is the critical condition that makes a functional approximable by finite-dimensional dynamical systems (Boyd-Chua theorem). It is also a physically necessary property of any realizable system, since perfect memory would require infinite energy.

**Why it matters for reservoir computing:** A reservoir computer has fading memory by design (via the echo state property, Chapter 5). This is not merely a convenient property — it is the mathematical foundation of why reservoir training with linear readouts works.

---

## The Boyd-Chua Theorem

The theorem (Boyd & Chua, 1985) stating that any causal, time-invariant functional with the fading memory property can be approximated to arbitrary accuracy by a finite-dimensional driven dynamical system. This is the theoretical license for reservoir computing: it guarantees that a sufficiently rich reservoir can approximate any target temporal computation.

**Proof sketch:** Fading memory implies that $H[\mathbf{u}]_t$ depends effectively only on a finite window of recent inputs. Stone-Weierstrass guarantees that polynomial functionals of this finite window are dense in the class of continuous functionals. A driven dynamical system with polynomial interactions approximates any such functional.

---

## Volterra Series

A functional Taylor series for nonlinear systems with memory:

$$y(t) = h_0 + \int h_1(\tau) u(t-\tau)\, d\tau + \iint h_2(\tau_1, \tau_2) u(t-\tau_1) u(t-\tau_2)\, d\tau_1 d\tau_2 + \cdots$$

The Volterra series is the exact (infinite-order) representation of any analytic fading-memory functional. Truncated Volterra series are polynomial approximators to temporal functionals, with parameter counts growing as $O(K^p)$ where $K$ is the window length and $p$ is the truncation order. This exponential growth is the principal practical limitation of Volterra methods and motivates the compact state representation of reservoir computing.

---

## Universal Approximation (Temporal Version)

The temporal generalization of the classical universal approximation theorem: any continuous causal functional with fading memory can be approximated by a driven dynamical system with a trainable readout. The reservoir provides the state; the readout provides the approximation. Together, they constitute a universal temporal approximator.

**Contrast with feedforward UAT:** The classical UAT (Cybenko, Hornik) guarantees approximation of *static* functions. The temporal UAT (Boyd-Chua) guarantees approximation of *functionals*. The two results together explain why reservoir computing (which uses both a static readout and a dynamical state) is a universal system for temporal computation.

---

## Sufficient State

The minimal information about the past that is necessary to compute the target output $y_t$, given future inputs. For a Markov process, the sufficient state is the current state of the process. For a general fading-memory functional, the sufficient state has finite effective dimension (because distant inputs have vanishing influence) but may be infinite-dimensional in principle.

A good reservoir computes an *approximation* of the sufficient state. The quality of this approximation — how much of the relevant past it captures, and how faithfully — determines the quality of the reservoir's predictions. This is formalized in Chapter 7 as information processing capacity.

---

## The Sliding Window Approach

A practical method for handling temporal dependencies in feedforward networks: concatenate the last $W$ inputs as a vector and feed it to a static network. The method is simple and effective for short-range dependencies, but fails for:
- Long-range dependencies (requiring $W$ too large)
- Unknown or variable memory requirements (requiring $W$ to be chosen in advance)
- High-dimensional inputs (since the input size scales as $dW$)

The sliding window approach is the naive solution to the temporal processing problem; reservoir computing is the principled one.

---

## Memory-Computation Tradeoff

The fundamental tradeoff between a system's temporal memory (how far into the past it can effectively reach) and its nonlinear computational capacity (how complex a function of the past it can implement). In a reservoir of fixed size, increasing one tends to decrease the other. This tradeoff is quantified precisely in Chapter 7 using the information processing capacity framework (Dambre et al., 2012).
