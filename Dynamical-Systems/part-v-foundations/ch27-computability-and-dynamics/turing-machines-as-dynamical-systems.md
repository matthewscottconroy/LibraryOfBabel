# 27.1 Turing Machines as Dynamical Systems

Here's a fact that took the theoretical computer science community decades to fully appreciate: a Turing machine is a dynamical system. Not metaphorically — literally. The tape is the state space (an element of $\Sigma^{\mathbb{Z}}$ for a finite alphabet $\Sigma$), the read/write head and internal state form a compact "control" layer, and the step function of the machine is a continuous map in the product topology. Computation is iteration.

This observation reframes computability theory in the language of dynamics. The halting problem asks whether the orbit of a given initial condition ever enters a certain set (the "halted" configurations). Undecidability says that no algorithm can determine this in general — but it's a statement about the dynamical system's long-time behavior. The $\omega$-limit sets of Turing machine dynamics are, in general, incomputable.

## 27.1.1 The Configuration Space

To make this precise, we need to define both the space and the map.

**Definition 27.1.1.** A *Turing machine* $M$ consists of:
- Finite state set $Q$ with distinguished initial state $q_0$ and halting states $Q_H$
- Tape alphabet $\Gamma$ with blank symbol $\sqcup$
- Transition function $\delta: (Q \setminus Q_H) \times \Gamma \to Q \times \Gamma \times \{L, R\}$

The *configuration* of $M$ is $(q, w, p)$ where $q \in Q$ is the current state, $w \in \Gamma^*$ is the tape content, and $p \in \mathbb{Z}$ is the head position.

Think of the configuration as the "phase point" of the computation. The machine doesn't move from state to state in the old finite-automaton sense — it moves through an enormous, infinite-dimensional configuration space, one step at a time.

**Definition 27.1.2.** The *configuration space* $\text{Conf}(M) = Q \times \Gamma^{\mathbb{Z}} \times \mathbb{Z}$ becomes a dynamical system under the *step map* $\delta^*: \text{Conf}(M) \to \text{Conf}(M)$ (one step of $M$'s execution).

**Theorem 27.1.3.** The step map $\delta^*$ is a partial computable function on $\text{Conf}(M)$. The *orbit* of an initial configuration $c_0$ under $\delta^*$ is the computation of $M$ on input $c_0$.

The halting problem asks: does the orbit of $c_0$ eventually reach $Q_H \times \Gamma^{\mathbb{Z}} \times \mathbb{Z}$?

This is not just a cute restatement. It means the halting problem is genuinely a problem about $\omega$-limit sets. Whether the orbit of $c_0$ eventually enters the halting configurations is exactly the question of whether that target set intersects the orbit closure. And the Turing-undecidability of halting translates directly: no computable algorithm can decide this intersection for all initial conditions.

## 27.1.2 Cellular Automata as Dynamical Systems

If Turing machines are somewhat abstract as dynamical systems, cellular automata make the dynamical nature impossible to miss. Here the state space is visibly a product space, the map is visibly local, and the iteration is visibly spatial dynamics playing out in time.

**Definition 27.1.4.** A *cellular automaton (CA)* is a quadruple $(d, Q, N, \delta)$ where:
- $d \geq 1$: dimension
- $Q$: finite state set
- $N \subset \mathbb{Z}^d$: finite neighborhood
- $\delta: Q^N \to Q$: local rule

The *global map* $F: Q^{\mathbb{Z}^d} \to Q^{\mathbb{Z}^d}$ defined by $(F(c))_i = \delta((c_{i+n})_{n \in N})$ is a continuous, shift-commuting map on the compact space $Q^{\mathbb{Z}^d}$.

The Curtis-Hedlund-Lyndon theorem closes the loop: every "nice" endomorphism of the shift is a cellular automaton.

**Theorem 27.1.5 (Curtis-Hedlund-Lyndon for CAs).** Every endomorphism of the full shift $Q^{\mathbb{Z}^d}$ commuting with all shifts is the global map of some cellular automaton.

This means the study of CA dynamics and the study of shift-commuting maps are literally the same subject.

**Example 27.1.6 (Game of Life).** Conway's Game of Life is a 2D CA with $Q = \{0,1\}$ and the standard Moore neighborhood ($|N| = 9$). It is known to be Turing-complete: the global map $F$ can simulate any Turing machine. Hence the halting problem for Game of Life initial configurations is undecidable.

Game of Life is remarkable here: a stunningly simple local rule — live cells survive with two or three neighbors, dead cells are born with exactly three — generates dynamics complex enough to simulate all of computation. Any undecidable question about Turing machines translates into an undecidable question about whether certain Game of Life configurations eventually die out.

In the next section, we push further: not just whether orbits behave in certain ways, but whether the natural invariant measures attached to dynamical systems are themselves computable.
