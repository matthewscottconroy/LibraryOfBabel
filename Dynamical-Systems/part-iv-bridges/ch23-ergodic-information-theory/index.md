# Chapter 23 — Ergodic Information Theory

> *The Shannon-McMillan-Breiman theorem is the ergodic-theoretic AEP: for a stationary ergodic process, the information per symbol concentrates at the entropy rate — almost surely. This is Birkhoff's theorem for the information function.*

**Prerequisites:** Chapter 7 (ergodic theory, Birkhoff's theorem), Chapter 16 (Shannon entropy, AEP).

---

## Overview

The asymptotic equipartition property — the AEP — is the central limit theorem of information theory. It says that for a stationary ergodic source, almost every long sequence is "typical": its probability is close to $2^{-nh}$, where $h$ is the entropy rate and $n$ is the length. Most sequences look alike, in the sense that they all have the same probability to first order.

We proved the AEP in Chapter 16 for i.i.d. sources, using the law of large numbers. But the law of large numbers has a more powerful generalization: Birkhoff's ergodic theorem, which applies to any ergodic measure-preserving transformation. The Shannon-McMillan-Breiman theorem is what you get when you apply Birkhoff's theorem to the *information function* — the function $x \mapsto -\log\mu(\text{atom of } x)$.

This is not a metaphor or an analogy. It is literally the same proof, applied to a different function.

The consequence is powerful: the AEP holds not just for i.i.d. sources, but for any stationary ergodic source. This includes Markov chains, hidden Markov processes, functions of ergodic measure-preserving transformations — any source whose past doesn't "remember" arbitrarily far back in a pathological way. The typical set has size $\approx 2^{nh}$, regardless of the specific correlation structure of the source.

And from this comes the most important application: the Lempel-Ziv algorithm achieves optimal compression — without knowing $h$ in advance — for any stationary ergodic source. The algorithm is universal; the SMB theorem is why.

### What's in this chapter

Section 23.1 sets up the framework: stationary processes, entropy rates, and the connection to KS entropy via the generating partition. This is the translation between the ergodic theory language and the information theory language.

Section 23.2 states and proves the Shannon-McMillan-Breiman theorem, then unpacks its consequence: the ergodic typical set and the structure of long typical sequences.

Section 23.3 applies the SMB theorem to prove the universality of Lempel-Ziv compression. This is the punchline of source coding theory: one algorithm compresses everything optimally.

Section 23.4 situates the SMB theorem in a hierarchy of ergodic results, from the law of large numbers to Birkhoff's theorem.

Section 23.5 connects to the Lempel-Ziv complexity as an empirical entropy estimator — one that works without knowledge of the source distribution.

### Sections

- [Stationary Processes and Their Entropy](stationary-processes-and-their-entropy.md)
- [The Shannon-McMillan-Breiman Theorem](the-shannon-mcmillan-breiman-theorem.md)
- [Universal Source Coding for Stationary Ergodic Sources](universal-source-coding.md)
- [The Ergodic Theorem as a Generalization](the-ergodic-theorem-as-generalization.md)
- [Entropy Rate and the Ziv-Lempel Complexity](entropy-rate-and-ziv-lempel-complexity.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
