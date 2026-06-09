# 35.4 Classifiable Subclasses

General classification is impossible. But there are many important subclasses where complete invariants do exist, and these results remain among the deepest in ergodic theory.

**Theorem 35.4.1 (Halmos-von Neumann, 1942).** Ergodic MPTs with *discrete spectrum* (all eigenvalues are countable, generating the full algebra) are classified by their group of eigenvalues $\text{Eig}(T) \subseteq S^1$.

Discrete spectrum is a very special property: the system looks like a rotation on a compact abelian group. The eigenvalue group tells you exactly which rotation. This was the first complete classification in ergodic theory.

**Theorem 35.4.2 (Bernoulli Shifts — Ornstein, 1970).** Bernoulli shifts are classified by their KS entropy.

The central theorem of Chapter 7. Two Bernoulli shifts $B(p_1, p_2, \ldots)$ and $B(q_1, q_2, \ldots)$ are isomorphic iff $H(p_1, p_2, \ldots) = H(q_1, q_2, \ldots)$.

**Theorem 35.4.3 (Ornstein-Weiss, 1987).** Bernoulli shifts of amenable groups are classified by entropy.

**Theorem 35.4.4 (Bowen, 2012).** Bernoulli shifts of sofic groups are classified by sofic entropy.

**Theorem 35.4.5 (Giordano-Putnam-Skau, 1995).** Minimal ${\mathbb Z}$-actions on the Cantor set are classified (up to orbit equivalence) by their ordered $K_0$ group.

Each of these is a complete classification in its domain. The ordered $K_0$ group is an algebraic object — a partially ordered abelian group with distinguished order unit — that can be computed from the dynamics. For minimal Cantor systems, this K-theoretic invariant is complete.

The pattern: the more structure the system has (pure Bernoulli, or Cantor minimal, or discrete spectrum), the more tractable the classification. It's precisely when you allow arbitrary ergodic systems that the problem becomes intractable. The Foreman-Rudolph-Weiss result lives in the gap between these structured cases.
