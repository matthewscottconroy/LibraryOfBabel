# 41.1 The Collatz Map: Definitions and Basic Properties

The rule is simple enough to explain to a child. Take any positive integer. If it's even, divide by 2. If it's odd, multiply by 3 and add 1. Repeat. The conjecture says you always reach 1.

**Definition 41.1.1.** The *Collatz map* (also called the $3x+1$ map) on ${\mathbb N}$ is:
$$C(n) = \begin{cases}n/2 & \text{if } n \equiv 0 \pmod 2 \\ 3n+1 & \text{if } n \equiv 1 \pmod 2\end{cases}$$

The *Collatz conjecture*: for every $n \in {\mathbb N}^+$, there exists $k$ with $C^k(n) = 1$.

**Verified range:** All $n \leq 2^{68}$ have been verified computationally (as of 2023).

Two quantities measure the orbit behavior.

**Definition 41.1.2.** The *stopping time* $\sigma(n) = \min\{k : C^k(n) < n\}$ (first time the orbit drops below the starting value).

**Definition 41.1.3.** The *total stopping time* $\tau(n) = \min\{k : C^k(n) = 1\}$.

The stopping time asks how long before the orbit drops below $n$; the total stopping time asks how long to reach 1. These are different, and both are interesting.

**Theorem 41.1.4 (Terras, 1976).** For almost all $n$ (in the density sense):
$$\sigma(n) \sim \frac{\log n}{\log(4/3)}.$$

The stopping time grows logarithmically — orbits reach below $n$ in about $\log n / \log(4/3)$ steps.

Terras's theorem is proved using $2$-adic analysis. The key insight: the stopping time depends on the "parity pattern" of the orbit — the sequence of even/odd values along the orbit — and $2$-adic analysis is the natural tool for understanding parity patterns.

Note what Terras's theorem does and doesn't say. It says stopping times are logarithmic for most $n$ in the density sense. It doesn't say stopping times are logarithmic for all $n$, or that orbits always reach 1. The "almost all" is crucial: the exceptional set might be infinite (just density zero), and the exceptional cases might be exactly the counterexamples to the conjecture.
