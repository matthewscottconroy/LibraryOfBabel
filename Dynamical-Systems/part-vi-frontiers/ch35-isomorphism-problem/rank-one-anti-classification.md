# 35.3 Rank-One Systems and Anti-Classification

You might hope that the anti-classification result for general ergodic systems is a pathology — that it goes away for "nice" systems. Rank-one systems are the simplest class of ergodic systems: they can be approximated by single towers, each made of a level and its translates under $T$. Surely classification is tractable for these?

No.

**Definition 35.3.1.** An MPT $T$ is *rank-one* if it has a sequence of Rohlin towers $(B_n, TB_n, \ldots, T^{h_n-1}B_n)$ with $\mu(B_n) \to 0$ and $\mu\left(\bigcup_{j=0}^{h_n-1}T^jB_n\right) \to 1$.

Rank-one systems are the "simplest" ergodic systems — they are approximated by rotations on cyclic groups.

**Examples 35.3.2.**
- The Chacón system: rank-one, weakly mixing, not mixing
- The von Neumann-Kakutani adding machine: rank-one, non-ergodic (!)
- The Staircase transformation: rank-one, mixing

Rank-one systems can be mixing or non-mixing. They can be weakly mixing or have discrete spectrum. And within the mixing ones:

**Theorem 35.3.3 (King, 1988).** Within the class of rank-one systems, the set of mixing systems is dense. In other words, mixing is "not generic" but is "dense" — rank-one systems can approximate mixing behavior.

And then Foreman and Weiss showed:

**Theorem 35.3.4 (Foreman-Weiss, 2019).** The isomorphism relation restricted to rank-one systems is a complete $\Sigma^1_1$ equivalence relation. Thus even for the "simplest" class of ergodic systems, the isomorphism problem is maximally complex.

So there's no escape. Rank-one systems — the most structured, most tractable class of ergodic systems — still have a maximally complex isomorphism problem. Classification is genuinely hard, not just for pathological systems, but for the simplest ones we can think of.

The open question is: which invariants, if any, can distinguish some of the rank-one systems? Entropy is trivially zero for most rank-one systems. The spectrum might help. But we don't have a complete invariant, and we now know we cannot have one.
