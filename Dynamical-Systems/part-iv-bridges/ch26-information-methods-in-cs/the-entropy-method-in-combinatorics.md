# 26.1 The Entropy Method in Combinatorics

Entropy is a combinatorial weapon. This might seem strange — entropy is defined probabilistically, in terms of probability distributions, not combinatorial objects. But the connection is simple: if you have a combinatorial object (a set, a graph, a function), define a random variable that is uniformly distributed over it. Then the entropy of the random variable is the logarithm of the number of objects. And entropy's inequalities — the chain rule, submodularity, conditioning reduces entropy — become inequalities about sizes of combinatorial objects.

The cleanest expression of this is Shearer's lemma, which is a general inequality about entropy over a collection of overlapping subsets.

## 26.1.1 Shearer's Lemma

**Theorem 26.1.1 (Shearer's Lemma).** Let $X_1, \ldots, X_n$ be random variables and $\mathcal{F} \subseteq 2^{[n]}$ a collection of subsets of $[n]$ such that each $i \in [n]$ belongs to at least $k$ sets in $\mathcal{F}$. Then:
$$H(X_1, \ldots, X_n) \leq \frac{1}{k} \sum_{S \in \mathcal{F}} H(X_S),$$
where $X_S = (X_i)_{i \in S}$.

*(proof)* By submodularity of entropy: $H(X_1, \ldots, X_n) = \sum_i H(X_i | X_1, \ldots, X_{i-1})$. Weighted averaging over the sets in $\mathcal{F}$, each $i$ appears $\geq k$ times, giving the bound.

Let's unpack the proof. The key step is the chain rule: $H(X_1, \ldots, X_n) = \sum_{i=1}^n H(X_i | X_1, \ldots, X_{i-1})$. For each set $S \in \mathcal{F}$, we also have $H(X_S) = \sum_{i \in S} H(X_i | X_j, j \in S, j < i) \geq \sum_{i \in S} H(X_i | X_1, \ldots, X_{i-1})$ (conditioning on more reduces entropy). Summing over $S \in \mathcal{F}$: each $i$ appears in at least $k$ sets, so the right side is at least $k \cdot H(X_1, \ldots, X_n)$. Dividing by $k$ gives Shearer.

## 26.1.2 The Loomis-Whitney Inequality

The Loomis-Whitney inequality is a geometric fact: a finite set in $\mathbb{Z}^d$ can't be much larger than the product of its projections onto coordinate hyperplanes. The entropy proof is four lines.

**Application 26.1.2 (Loomis-Whitney Inequality).** Let $A \subseteq {\mathbb Z}^d$ be a finite set. Let $A_{-i}$ denote the projection of $A$ onto the hyperplane perpendicular to the $i$-th coordinate. Then:
$$|A|^{d-1} \leq \prod_{i=1}^d |A_{-i}|.$$

*(proof)* Choose $X = (X_1, \ldots, X_d)$ uniformly from $A$. Then $H(X) = \log|A|$ and $H(X_{-i}) \leq \log|A_{-i}|$. Shearer's lemma with the collection $\{[d] \setminus \{i\}\}_{i=1}^d$ (each element appears $d-1$ times) gives $(d-1)\log|A| \leq \sum_i \log|A_{-i}|$.

Taking exponentials: $|A|^{d-1} \leq \prod_i |A_{-i}|$. The proof is complete.

Compare this to the classical proof of Loomis-Whitney, which uses geometric arguments about projections and measures. The entropy proof is, arguably, more transparent: it says that the information in a point of $A$ (which is $\log |A|$ bits) can be reconstructed from its projections, and each projection contributes at most $\log|A_{-i}|$ bits, and the Shearer inequality controls how these contributions combine.

## 26.1.2 The Entropy Method for Counting

Shearer's lemma extends to counting problems in graph theory and combinatorics. The strategy: define a random variable on the combinatorial object, bound its entropy using the collection structure, and translate back to a count.

**Example 26.1.3 (Counting Graphs).** The number of triangle-free graphs on $n$ vertices is at most $2^{n^2/4}$ (Kruskal-Katona, proved by entropy method).

*(proof sketch)* Let $G$ be a triangle-free graph. The Shannon entropy of the edge set $E(G)$ satisfies:
$$H(E(G)) \leq \frac{1}{3}\sum_{\text{triangles}} H(E(\triangle)) \leq \frac{\binom{n}{3}}{3} \cdot \log 8.$$

**Theorem 26.1.4 (Entropy Method for Graph Coloring).** For a $k$-colorable graph $G$ on $n$ vertices with $m$ edges:
$$m \leq \frac{n^2}{2}\left(1 - \frac{1}{k}\right).$$

This is Turán's theorem, proved via entropy.

The Turán theorem proof goes: if $G$ is $k$-colorable with color classes $V_1, \ldots, V_k$, choose a random vertex $X$ uniformly from $V$. The entropy of $X$ is $\log n$. But $X$ can be described by its color class (which contributes $\log k$ bits) plus its identity within the class (at most $\log(n/k)$ bits by uniformity within the class). This forces the number of edges between classes to be at most the maximum achieved by the complete $k$-partite graph — the Turán graph.

The entropy method works, broadly, because entropy is a universal measure of information content: bounding the entropy of the object bounds the log-number of such objects, or the log-size of the object, or the complexity of its description. In combinatorics, these bounds give the extremal results.
