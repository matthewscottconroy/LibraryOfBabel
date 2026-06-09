# Chapter 12 — Symbolic Dynamics

> *Every orbit is a sequence. Symbolic dynamics makes this precise, turning the continuous into the combinatorial and connecting dynamical systems to information theory, automata theory, and combinatorics on words.*

**Prerequisites:** Chapters 6 (topological dynamics, conjugacy), 7 (ergodic theory, entropy), 9 (Markov partitions).

---

## What This Chapter Is About

There is a beautiful trick at the heart of symbolic dynamics. You take a complicated continuous dynamical system — say, a hyperbolic map on a manifold — and you encode its orbits as sequences of symbols. Each symbol corresponds to a piece of the phase space (a Markov partition); each orbit becomes an infinite sequence of symbols recording which piece the orbit visits at each step. Instead of studying the geometry of orbits directly, you study the combinatorics of sequences.

This trick is called symbolic coding, and it transforms a continuous problem into a combinatorial one. The "phase space" becomes a space of sequences; the "dynamics" becomes a shift map. And in this new setting, the tools of information theory, automata theory, and combinatorics on words become available.

The fundamental object is the *full shift*: all infinite sequences over a finite alphabet, with the dynamics given by the left-shift operator. This is simultaneously the simplest nontrivial dynamical system and the universal receptacle into which all other systems code. Every ergodic measure-theoretic system is a factor of a Bernoulli shift (by Krieger's theorem). Every hyperbolic system has a Markov partition and hence a symbolic model.

A *subshift* is any closed, shift-invariant subset of the full shift. The language of a subshift — the collection of finite words that appear in its sequences — characterizes it completely. Different constraints on the language give different classes of subshifts: subshifts of finite type (SFTs) are defined by finitely many forbidden words and correspond to finite directed graphs; sofic shifts are images of SFTs under sliding block codes and correspond to hidden Markov models; more general subshifts correspond to more complicated computational structures.

The entropy of a subshift is the exponential growth rate of its language: how many words of length $n$ does it contain? For SFTs, this is the logarithm of the Perron-Frobenius eigenvalue of the transition matrix — a clean algebraic answer to a geometric question. The Parry measure is the unique measure of maximal entropy, the natural "uniform" measure on the subshift, and it is a Markov measure with explicitly computable transition probabilities.

The zeta function counts periodic orbits: it is a formal power series whose coefficients are the number of periodic points of each period. For SFTs, this function is rational — in fact, it equals $1/\det(I - tA)$ where $A$ is the transition matrix. This rationality is a deep algebraic fact with strong dynamical consequences.

The classification problem — when are two SFTs conjugate? — is solved in principle by Williams' theorem (strong shift equivalence) but is computationally difficult: decidable over $\{0,1\}$ matrices, undecidable over $\mathbb{Z}$.

The information-theoretic connections are explicit and beautiful. A subshift is a stationary source; its topological entropy is the maximum entropy rate; its Parry measure is the maximum-entropy source; a sofic shift is a hidden Markov model; a sliding block code is a block encoder. The entire symbolic dynamics vocabulary has a direct translation into information theory, and we spell out this dictionary carefully in Section 12.9.

**What this chapter builds:** The full shift and subshifts as dynamical systems; subshifts of finite type (SFTs) and their transition matrices; sofic shifts as factors of SFTs; topological entropy via word growth; the Perron-Frobenius theorem in this setting; the zeta function counting periodic orbits; and the connection to automata theory and information theory.

---

## Sections

- [12.1 The Full Shift](the-full-shift.md) — Sequences, the shift map, and the Cantor set topology
- [12.2 Subshifts](subshifts.md) — Closed invariant sets, languages, and forbidden words
- [12.3 Subshifts of Finite Type](subshifts-of-finite-type.md) — Transition matrices, the golden mean shift, and the SFT structure theorem
- [12.4 Topological Entropy of Subshifts](topological-entropy-of-subshifts.md) — Word complexity, subadditivity, and Perron-Frobenius
- [12.5 Sofic Shifts](sofic-shifts.md) — Factors of SFTs, regular languages, and the Fischer cover
- [12.6 The Zeta Function](the-zeta-function.md) — Counting periodic orbits and the rationality theorem
- [12.7 Conjugacy and Classification](conjugacy-and-classification.md) — Williams' theorem and the decidability landscape
- [12.8 Automorphisms of Shifts](automorphisms-of-shifts.md) — Sliding block codes and the Curtis-Hedlund-Lyndon theorem
- [12.9 The Connection to Information Theory](the-connection-to-information-theory.md) — The complete symbolic dynamics / information theory dictionary

---

- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
