# 3.2 What RNNs Can Represent

## The Computational Power of Recurrent Networks

Before confronting the difficulties of training RNNs, we should appreciate what they can represent. The answer is: everything. This is not hyperbole — it is a theorem.

**Theorem (Siegelmann & Sontag, 1995) [Siegelmann1995]:** Every Turing machine can be simulated in real time by a finite recurrent neural network with sigmoidal activations and rational weights.

This is a stronger statement than the universal approximation theorem for feedforward networks (which concerns static functions). RNNs are Turing-complete: they can compute any computable function on sequence inputs, including functions that require unbounded memory and arbitrary control flow.

The proof constructs an explicit RNN that simulates a given Turing machine by encoding the machine's state and tape content in the network's recurrent state. The encoding uses the real-valued state of sigmoidal neurons to represent discrete symbols with rational-weight weights acting as precise switching functions.

## Finite Automata Simulation

A simpler but more intuitive result: every **finite automaton** — a machine that processes sequences of symbols and transitions between a finite number of internal states — can be exactly simulated by an RNN with integer weights.

A finite automaton has:
- A finite state set $Q = \{q_1, \ldots, q_K\}$
- An input alphabet $\Sigma$
- A transition function $\delta: Q \times \Sigma \to Q$
- An initial state $q_0$ and a set of accepting states $F \subseteq Q$

The simulation works by representing the current state $q_k$ as a one-hot vector $\mathbf{e}_k \in \{0,1\}^K$, and encoding the transition function as the recurrent weight matrix: $W^{rec}_{ij} = 1$ if the transition from state $j$ leads to state $i$ under the current input, $0$ otherwise. With appropriate input encoding, the network's state vector $\mathbf{x}_t$ tracks the automaton's state exactly.

**Why this matters:** Finite automata compute all **regular languages** — the simplest class in the Chomsky hierarchy [Chomsky1956]. RNNs can simulate them. More powerful formalisms (context-free grammars, context-sensitive grammars, unrestricted grammars) can also be simulated by RNNs with unbounded precision [Siegelmann1995], though with increasingly subtle constructions.

## The Gap: Expressiveness vs. Learnability

The Siegelmann-Sontag theorem establishes that RNNs have the full computational power of Turing machines. But this creates an apparent paradox: if RNNs can compute everything, why do we ever fail to train them to compute the things we want?

The answer is the gap between **expressiveness** (what a class of models can represent) and **learnability** (whether those representations can be found by efficient algorithms from finite data).

A function may be representable by an RNN, but:
1. The specific weight configuration that implements it may be unique or rare — a needle in a high-dimensional haystack.
2. Gradient descent may not be able to find that configuration, due to vanishing gradients, flat regions, or local minima.
3. Even if gradient descent converges, the required training time and data may be infeasible.

This gap between expressiveness and learnability is the central tension in the theory of learning. For RNNs, the expressive power is maximal — they are universal. The learnability is severely limited by the gradient problem.

**Reservoir computing resolves this tension differently.** Rather than training an expressive model via gradient descent (hard), it uses a fixed, random model (always available) and trains only the output layer (easy). The price is that the inductive bias of the random model may not match the task — but the Boyd-Chua theorem guarantees that, with a large enough reservoir, this price can be made arbitrarily small.

## What RNNs Can Represent That Feedforward Networks Cannot

For concreteness, here are temporal computations that RNNs can represent but feedforward networks with any fixed window cannot:

| Task | Why Feedforward Fails | Why RNN Succeeds |
|------|----------------------|------------------|
| Parity of all past bits | Requires infinite window | State encodes running XOR |
| Long-distance agreement (syntax) | Dependency length unbounded | State tracks open dependencies |
| Chaotic attractor prediction | Future depends on full trajectory | State tracks attractor position |
| Copy-repeat patterns | Repetition count unbounded | State encodes count and buffer |
| Formal language membership | Requires stack (pushdown automaton) | RNN can simulate pushdown [Siegelmann1995] |

Each of these is a real challenge in applications: long-distance syntactic dependencies in language, chaotic dynamics in physics, repetition patterns in music. The theoretical guarantee of RNN universality is the reason these applications are worth attempting.

## Practical Limits of Universality

Universality results must be interpreted carefully:

1. **The network may be very large.** The Siegelmann-Sontag construction requires a network that grows with the complexity of the simulated Turing machine. For practical tasks, the network size needed for exact representation may be astronomically large.

2. **Exact vs. approximate.** Real RNNs use floating-point arithmetic with limited precision. Turing completeness relies on arbitrary-precision computation that is not achievable in practice.

3. **Training dynamics are not guaranteed.** The existence of a weight configuration that represents the target function says nothing about whether gradient descent will find it.

4. **The construction is not unique.** There are many ways to represent a given function in an RNN. Gradient descent may find a different, less efficient representation, or no representation at all.

For reservoir computing, the relevant universality result is different: it is not about what weights an RNN can have, but about what functions a reservoir state can represent. This is the Boyd-Chua theorem applied to the reservoir's input-output relationship — and the universality it guarantees is approximation-theoretic, not computational.

---

## References

- [Siegelmann1995] Siegelmann, H.T. & Sontag, E.D. (1995). On the computational power of neural nets. *Journal of Computer and System Sciences*, 50(1), 132–150. **[The Turing-completeness proof. Essential.]**
- [Siegelmann1992] Siegelmann, H.T. (1992). *Computation Beyond the Turing Limit* (doctoral dissertation, Rutgers University).
- [Chomsky1956] Chomsky, N. (1956). Three models for the description of language. *IRE Transactions on Information Theory*, 2(3), 113–124.
- [Funahashi1993] Funahashi, K. & Nakamura, Y. (1993). Approximation of dynamical systems by continuous time recurrent neural networks. *Neural Networks*, 6(6), 801–806. **[Proves continuous-time RNNs are universal dynamical system approximators.]**
- [Maass2007] Maass, W. (2007). Computational aspects of feedback in neural circuits. *PLOS Computational Biology*, 3(1), e165. **[Extends universality results to networks with feedback.]**
