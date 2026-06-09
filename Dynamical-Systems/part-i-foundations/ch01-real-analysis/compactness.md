# 1.2 Compactness

Compactness is one of the most powerful properties a metric space can have, and most of the existence theorems in dynamical systems rely on it. But it's also one of the most initially mysterious concepts in analysis — because the formal definition doesn't look like what it's saying.

Here's the intuition. Compact spaces behave like finite sets in many key respects: continuous functions on them are bounded and uniformly continuous, sequences in them have convergent subsequences, and covering arguments that would fail in infinite-dimensional or unbounded spaces work cleanly on them. Compact spaces are "small" in a topological sense, even when they contain infinitely many points.

## 1.2.1 Equivalent Definitions

The official definition is open-cover based:

**Definition 1.2.1.** A subset $K \subseteq X$ of a metric space is *compact* if every open cover has a finite subcover: whenever $K \subseteq \bigcup_{\alpha \in I} U_\alpha$ for open sets $U_\alpha$, there exist finitely many $\alpha_1, \ldots, \alpha_n$ with $K \subseteq U_{\alpha_1} \cup \cdots \cup U_{\alpha_n}$.

This is not how most people think about compactness day-to-day, but it's the definition that generalizes best to non-metric spaces. Fortunately, in metric spaces, there are equivalent formulations that are often more tractable:

**Theorem 1.2.2 (Equivalent Characterizations of Compactness in Metric Spaces).** For a subset $K$ of a metric space, the following are equivalent:
1. $K$ is compact (every open cover has a finite subcover).
2. $K$ is *sequentially compact* (every sequence in $K$ has a convergent subsequence with limit in $K$).
3. $K$ is *complete and totally bounded* (for every $\varepsilon > 0$, $K$ is covered by finitely many balls of radius $\varepsilon$).

*(proof sketch)* The equivalence $(1) \Leftrightarrow (3)$ is the key step. Total boundedness gives a way to extract convergent subsequences via a diagonal argument.

What this is really saying: in a compact space, you can never "escape to infinity" in any direction, and every infinite sequence has to accumulate somewhere. Total boundedness is the rigorous version of "you can always cover the set with finitely many small balls" — and together with completeness, this prevents sequences from running off the edge of the space.

The most concrete version, for $\mathbb{R}^n$, is classical:

**Theorem 1.2.3 (Heine-Borel Theorem).** A subset $K \subseteq \mathbb{R}^n$ is compact if and only if it is closed and bounded.

This is the theorem most people encounter first: a set in $\mathbb{R}^n$ is compact iff it's closed and bounded. Every continuous function on $[0,1]$ attains its maximum? Heine-Borel. Bolzano-Weierstrass (every bounded sequence has a convergent subsequence)? Sequential compactness of $[0,1]$.

**Remark 1.2.4.** Heine-Borel fails in infinite-dimensional spaces. The closed unit ball in an infinite-dimensional Banach space is *never* compact. This distinction is crucial when dynamical systems act on function spaces — where the "state space" is infinite-dimensional and compactness arguments require more care (see Arzelà-Ascoli in Section 1.3).

## 1.2.2 Properties of Compact Spaces

Compact spaces come with a package of properties that make them extremely useful:

**Proposition 1.2.5.** Let $K$ be a compact metric space.
1. $K$ is complete and bounded.
2. Every closed subset of $K$ is compact.
3. If $f: K \to Y$ is continuous, then $f(K)$ is compact.
4. If $f: K \to \mathbb{R}$ is continuous, then $f$ attains its maximum and minimum.
5. If $f: K \to Y$ is continuous and bijective with $Y$ Hausdorff, then $f^{-1}$ is continuous.

Properties (3) and (4) are the workhorses. Compact sets are "preserved" by continuous maps, and real-valued functions on compact sets are bounded and attain their bounds. Property (5) is surprisingly non-trivial: in general, a continuous bijection need not have a continuous inverse, but compactness forces the issue.

Property (5) will come back in a key way when we study conjugacies between dynamical systems: if we can find a continuous bijection between a compact phase space and a known system, compactness guarantees it's actually a homeomorphism.

With compactness established, we can now ask: what happens to continuous functions on compact spaces? The answers — uniform continuity, the Arzelà-Ascoli theorem — are the beginning of the functional analysis that makes dynamical systems tick.
