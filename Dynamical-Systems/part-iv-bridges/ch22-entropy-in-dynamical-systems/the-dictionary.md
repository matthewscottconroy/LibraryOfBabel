# 22.1 The Dictionary: Dynamics ↔ Information

Two fields developed independently across the twentieth century — ergodic theory in the mathematics departments, information theory in the engineering schools — and arrived at the same set of ideas with different names. When Kolmogorov defined the entropy of a dynamical system in 1958, he was almost certainly unaware that Shannon had defined the entropy of a source in 1948. And yet the definitions are, under translation, identical.

This chapter makes that translation explicit. The goal isn't just to note the coincidence — it's to exploit it. Each side illuminates the other: ergodic theory gives information theory its deepest theorems (the AEP as a special case of Birkhoff's theorem, the SMB theorem as its almost-sure strengthening), while information theory gives dynamical systems its sharpest invariant (KS entropy as the complete invariant for Bernoulli shifts, by Ornstein's theorem — one of the most remarkable results of the 1970s).

Here is the dictionary in its simplest form. Every row is a theorem waiting to happen.

| Dynamical Concept | Information-Theoretic Concept |
|---|---|
| Partition $\xi$ of $(X, \mu)$ | Source alphabet |
| KS entropy $h_\mu(f, \xi)$ | Entropy rate of the coded process |
| Generating partition | Sufficient statistics |
| Topological entropy $h_{\text{top}}(f)$ | Max achievable entropy rate |
| Measure of maximal entropy | Capacity-achieving input distribution |
| Variational principle | Duality: channel capacity = max mutual information |
| Ornstein's theorem | Entropy classifies Bernoulli shifts (up to isomorphism) |
| Shannon-McMillan-Breiman | Ergodic AEP (see Chapter 23) |

Read this table slowly. The left column is everything we built in Parts I and II. The right column is everything we built in Part III. They're the same column.

The key to unlocking the dictionary is the partition. Given an ergodic measure-preserving transformation $(X, \mathcal{B}, \mu, f)$ and a finite measurable partition $\xi = \{A_1, \ldots, A_k\}$, we can *code* an orbit: assign to each point $x$ the sequence $(\xi(f^n(x)))_{n \geq 0}$, where $\xi(y) = A_i$ if $y \in A_i$. This is a stationary stochastic process on the alphabet $\{A_1, \ldots, A_k\}$. The entropy rate of this process is exactly $h_\mu(f, \xi)$, the KS entropy with respect to the partition.

When $\xi$ is a *generating* partition — when knowing the orbit code is equivalent to knowing the orbit itself — we get the full KS entropy $h_\mu(f) = h_\mu(f, \xi)$. The generating partition is the sufficient statistic: it captures all the information about the orbit.

The variational principle (Section 22.3) then says: the topological entropy is the supremum of KS entropy over all invariant measures. This is exactly Shannon's duality between channel capacity and mutual information — capacity is the maximum of mutual information over all input distributions. The maximum is achieved by the *measure of maximal entropy*, which plays the role of the capacity-achieving input distribution.

Ornstein's theorem (1970) is the crowning result: two Bernoulli shifts are isomorphic (as measure-preserving transformations) if and only if they have the same entropy. Entropy is a complete invariant for the most "random" class of dynamical systems. This is something Shannon's theory predicted — information content should determine equivalence — and ergodic theory confirmed.

As we move through this chapter, keep the dictionary in view. When we prove a theorem on the left side, ask what it says on the right. When we state a result in information-theoretic language, translate it back. The two languages are different ways of saying the same thing.
