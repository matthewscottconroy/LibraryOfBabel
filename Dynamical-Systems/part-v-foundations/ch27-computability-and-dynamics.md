# Chapter 27 — Computability Theory and Dynamical Systems

> *Every dynamical system is a computation. Every computation is a dynamical system. The Church-Turing thesis and the undecidability of the halting problem are theorems about dynamical systems — about what happens in the long run.*

**Prerequisites:** Chapter 18 (algorithmic information theory, Turing machines), Chapter 25 (undecidability in dynamics).

---

## 27.1 Turing Machines as Dynamical Systems

### 27.1.1 The Configuration Space

**Definition 27.1.1.** A *Turing machine* $M$ consists of:
- Finite state set $Q$ with distinguished initial state $q_0$ and halting states $Q_H$
- Tape alphabet $\Gamma$ with blank symbol $\sqcup$
- Transition function $\delta: (Q \setminus Q_H) \times \Gamma \to Q \times \Gamma \times \{L, R\}$

The *configuration* of $M$ is $(q, w, p)$ where $q \in Q$ is the current state, $w \in \Gamma^*$ is the tape content, and $p \in {\mathbb Z}$ is the head position.

**Definition 27.1.2.** The *configuration space* $\text{Conf}(M) = Q \times \Gamma^{\mathbb Z} \times {\mathbb Z}$ becomes a dynamical system under the *step map* $\delta^*: \text{Conf}(M) \to \text{Conf}(M)$ (one step of $M$'s execution).

**Theorem 27.1.3.** The step map $\delta^*$ is a partial computable function on $\text{Conf}(M)$. The *orbit* of an initial configuration $c_0$ under $\delta^*$ is the computation of $M$ on input $c_0$.

The halting problem asks: does the orbit of $c_0$ eventually reach $Q_H \times \Gamma^{\mathbb Z} \times {\mathbb Z}$?

### 27.1.2 Cellular Automata as Dynamical Systems

**Definition 27.1.4.** A *cellular automaton (CA)* is a quadruple $(d, Q, N, \delta)$ where:
- $d \geq 1$: dimension
- $Q$: finite state set
- $N \subset {\mathbb Z}^d$: finite neighborhood
- $\delta: Q^N \to Q$: local rule

The *global map* $F: Q^{{\mathbb Z}^d} \to Q^{{\mathbb Z}^d}$ defined by $(F(c))_i = \delta((c_{i+n})_{n \in N})$ is a continuous, shift-commuting map on the compact space $Q^{{\mathbb Z}^d}$.

**Theorem 27.1.5 (Curtis-Hedlund-Lyndon for CAs).** Every endomorphism of the full shift $Q^{{\mathbb Z}^d}$ commuting with all shifts is the global map of some cellular automaton.

**Example 27.1.6 (Game of Life).** Conway's Game of Life is a 2D CA with $Q = \{0,1\}$ and the standard Moore neighborhood ($|N| = 9$). It is known to be Turing-complete: the global map $F$ can simulate any Turing machine. Hence the halting problem for Game of Life initial configurations is undecidable.

---

## 27.2 Computability of Invariant Measures

**Definition 27.2.1.** A Borel probability measure $\mu$ on a compact metric space $X$ is *computable* if $\mu(U)$ is a computable real number for every computable open set $U$.

**Theorem 27.2.2 (Computability of Ergodic Averages).** For a computable dynamical system $(X, f)$ and a computable integrable function $\varphi$:
- If $\mu$ is a computable $f$-invariant measure and $x$ is $\mu$-generic (ML-random), then $\frac{1}{n}\sum_{k<n}\varphi(f^k(x))$ converges computably to $\int\varphi\,d\mu$.
- The rate of convergence is computable from the ML-randomness of $x$.

**Theorem 27.2.3 (Galatolo-Hoyrup-Rojas).** For a computable expanding map $f$ on the circle:
- The absolutely continuous invariant measure (ACIM) is computable.
- For Lebesgue-a.e. $x$, ergodic averages converge computably.
- There exist $x$ where the ergodic averages are not computable at all (e.g., $x$ is computable but not ML-random).

---

## 27.3 The Recursion Theorem and Fixed Points

**Theorem 27.3.1 (Kleene's Recursion Theorem).** For any computable function $f: \mathbb{N} \to \mathbb{N}$, there exists $e \in \mathbb{N}$ such that $\phi_e = \phi_{f(e)}$ (where $\phi_e$ is the function computed by Turing machine $e$).

**Dynamical Interpretation:** Let the space of Turing machines be $T = \{0,1,2,\ldots\}$ with the step function $e \mapsto f(e)$ (reindexing). The recursion theorem says $f$ has a "self-reproducing" fixed point — a program that outputs its own index.

**Application: Self-Replicating Programs.** Quines (programs that output their own source code) exist in any sufficiently powerful language, by the recursion theorem. This is the dynamical statement: the program is a fixed point of the "run and print" transformation.

**Theorem 27.3.2 (Rice's Theorem).** Let $P$ be any nontrivial property of computable functions (neither all functions have $P$ nor none do). Then the problem "does $\phi_e$ have property $P$?" is undecidable.

**Dynamical Reformulation:** Rice's theorem says that nontrivial asymptotic properties of the dynamical system $({\mathbb N}, e \mapsto \phi_e(0))$ are undecidable. "Does this orbit converge?", "Is this orbit eventually periodic?", "Is this orbit bounded?" — all undecidable for the universal Turing machine.

---

## 27.4 Computable Real Analysis and Dynamics

### 27.4.1 Type-2 Computability

**Definition 27.4.1 (Weihrauch, 2000).** A real number $x \in {\mathbb R}$ is *computable* if there is a Turing machine that, given $n \in \mathbb{N}$, outputs a rational $q_n$ with $|x - q_n| < 2^{-n}$.

A *computable function* $f: {\mathbb R} \to {\mathbb R}$ maps computable inputs to computable outputs uniformly in the approximation index.

**Theorem 27.4.2 (Every Computable Function is Continuous).** If $f: [0,1] \to [0,1]$ is computable, it is uniformly continuous (and the modulus of continuity is computable).

**Corollary 27.4.3.** Discontinuous functions (step functions, indicator functions of non-open sets) are not computable. The indicator function of a non-computable set is not computable.

### 27.4.2 Degrees of Computability in Dynamics

**Definition 27.4.4 (Turing Degree).** The *Turing degree* of a set $A \subseteq \mathbb{N}$ is the equivalence class of $A$ under Turing reducibility ($A \leq_T B$ if $A$ is computable from $B$).

**Theorem 27.4.5.** For the quadratic family $f_c(z) = z^2 + c$:
- The Mandelbrot set boundary $\partial\mathcal{M}$ contains points of every Turing degree (it is "computationally complete")
- The Julia set $\mathcal{J}(f_c)$ for $c \in \partial\mathcal{M}$ can have arbitrarily high Turing degree

These are consequences of the Braverman-Yampolsky noncomputability results (Section 25.5).

---

## 27.5 Formal Verification of Dynamical Properties

**Definition 27.5.1.** A property $P$ of a dynamical system is *$\Pi_1^0$* (or *co-c.e.*) if the set $\{(f, x) : (f,x) \text{ has } P\}$ is a countable intersection of computable open sets.

**Examples of $\Pi_1^0$ properties:**
- "The orbit of $x$ never enters the open set $U$" (requires checking all steps)
- "The system $f$ has no periodic orbit in $U$"
- "The system $f$ is nonexpansive on $U$"

**$\Sigma_1^0$ (c.e.) properties:**
- "The orbit of $x$ eventually enters $U$"
- "The system $f$ has a periodic orbit in $U$"

**Theorem 27.5.2 (Hierarchy of Dynamical Properties).** Let $f$ be a computable dynamical system on $[0,1]$:
- Transitivity is $\Pi_3^0$-complete: $h_{\text{top}}(f) > 0$ is $\Pi_1^0$-hard.
- Existence of a dense orbit is $\Sigma_2^0$ (c.e. in the limit).
- Minimality ($= $ all orbits dense) is $\Pi_2^0$.
- The set of recurrent points is $\Pi_2^0$.

---

## Exercises

**Exercise 27.1.** (Game of Life) Show that Conway's Game of Life can simulate a binary counter. Conclude that "does configuration $c$ eventually become all-zeros?" is undecidable.

**Exercise 27.2.** (Recursion Theorem) Construct an explicit quine in pseudocode (a program that prints its own source code), using the construction from Kleene's recursion theorem.

**Exercise 27.3.** For the tent map $T(x) = 2\min(x, 1-x)$: show that all rational $x$ have eventually periodic orbits. Show that the Lebesgue measure is a computable invariant measure. Is there a computable $x$ with a non-eventually-periodic orbit?

**Exercise 27.4.** (Research) The Collatz map $C: {\mathbb N} \to {\mathbb N}$ is a computable dynamical system. Using the hierarchy of dynamical properties, classify "all orbits reach 1" (the Collatz conjecture) into the arithmetic hierarchy. Is it $\Pi_2^0$? Can it be $\Pi_1^0$?

---

## Chapter Notes

The connection between computation and dynamical systems is developed in Moore's *Unpredictability and Undecidability in Dynamical Systems* (1990) and Koiran-Moore (1999). The computational complexity perspective is in Blum-Cucker-Shub-Smale's *Complexity and Real Computation*.

Weihrauch's *Computable Analysis* (2000) is the standard reference for Type-2 computability over real numbers. The Braverman-Yampolsky results on Julia sets are in their book *Computability of Julia Sets* (Springer, 2009).

The arithmetic hierarchy of dynamical properties is surveyed in Hoyrup-Rojas-Weihrauch (2012). Galatolo-Hoyrup-Rojas's work on computable ergodic theory appears in *Dynamics and abstract computability: computing invariant measures* (Discrete and Continuous Dynamical Systems, 2011).
