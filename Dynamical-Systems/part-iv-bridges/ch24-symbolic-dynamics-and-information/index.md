# Chapter 24 — Symbolic Dynamics as Information Theory

> *A subshift is a constrained source. The topological entropy is the maximum entropy rate. A factor map is a noisy channel. The full symbolic dynamics / information theory dictionary, made rigorous.*

**Prerequisites:** Chapters 12 (symbolic dynamics), 16 (channel capacity, source coding), 23 (ergodic information theory).

---

## Overview

We have been building a dictionary between dynamical systems and information theory since Chapter 22. Here we complete it — systematically, rigorously, and with applications.

The setting is symbolic dynamics: sequences on a finite alphabet, constrained to live in some shift-invariant subset. This is simultaneously the most combinatorial part of dynamical systems and the most concrete part of information theory. A subshift is a constrained channel. A factor map is a sliding block code. The topological entropy is the channel capacity. The Parry measure is the capacity-achieving input distribution.

These aren't analogies. They're theorems.

What makes symbolic dynamics so useful here is that everything is explicit. For a subshift of finite type, the topological entropy is $\log \lambda_{\text{PF}}(A)$ — the logarithm of the Perron-Frobenius eigenvalue of the transition matrix. The measure of maximal entropy is the Parry measure, which we can write down explicitly. The KS entropy of any invariant measure is the entropy rate of the corresponding Markov measure. And factor maps are exactly the sliding block codes studied in coding theory.

The chapter also introduces hidden Markov processes — the outputs of hidden Markov models — and shows they are exactly the sofic processes of symbolic dynamics. Computing the entropy rate of a hidden Markov process is hard (it requires knowledge of the Blackwell measure on belief states), but the connection to sofic shifts gives it a clean mathematical home.

The applications are real: constrained codes for magnetic recording, DNA sequence modeling, natural language statistics. Symbolic dynamics is the mathematical language for all of these.

### What's in this chapter

Section 24.1 establishes subshifts as constrained sources and works out the source coding implications.

Section 24.2 develops factor maps as channels — the information-theoretic meaning of conjugacy, factor, and the various notions of code in symbolic dynamics.

Section 24.3 shows that hidden Markov processes are exactly sofic processes — closing a loop between the engineering and mathematical communities.

Section 24.4 presents the complete symbolic dynamics / information theory dictionary.

Section 24.5 gives the application to data compression via constrained codes.

### Sections

- [Subshifts as Stationary Sources](subshifts-as-stationary-sources.md)
- [Factor Maps as Channels](factor-maps-as-channels.md)
- [Hidden Markov Processes and Sofic Shifts](hidden-markov-processes-and-sofic-shifts.md)
- [The Complete Dictionary](the-complete-dictionary.md)
- [Data Compression via Subshift Constraints](data-compression-via-subshift-constraints.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
