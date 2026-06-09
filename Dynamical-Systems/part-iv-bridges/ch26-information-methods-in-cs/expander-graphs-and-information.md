# 26.4 Expander Graphs and Information

An expander graph is a sparse graph that is paradoxically well-connected: every subset of vertices has many neighbors outside the subset. Expanders are used everywhere in theoretical computer science — in error-correcting codes, randomness extractors, pseudorandom generators, derandomization, and algorithm design. The information-theoretic perspective explains why: expanders are graphs on which information diffuses rapidly.

## 26.4.1 Spectral Expansion and Information Diffusion

**Definition 26.4.1.** A $d$-regular graph $G$ on $n$ vertices is an $\varepsilon$-*expander* if for every set $S$ with $|S| \leq n/2$:
$$|N(S)| \geq (1+\varepsilon)|S|.$$

Equivalently, the spectral gap $\lambda = \lambda_1 - \lambda_2 \geq \varepsilon d$ (where $\lambda_1 = d$ is the largest eigenvalue of the adjacency matrix).

The vertex expansion condition says: you can't "trap" information in a small set. Any set has many connections to the outside, so information (modeled as a random walk) spreads throughout the graph.

The spectral condition makes this precise: the second eigenvalue $\lambda_2$ controls how fast the random walk mixes. If $\lambda_2$ is much smaller than $\lambda_1 = d$, the walk mixes fast. If $\lambda_2 \approx \lambda_1$, the walk is slow to mix (like a disconnected or nearly disconnected graph).

**Theorem 26.4.2 (Expander Mixing Lemma).** For a $d$-regular expander with second eigenvalue $\lambda_2$, and any sets $A, B \subseteq V$:
$$\left|E(A,B) - \frac{d|A||B|}{n}\right| \leq \lambda_2 \sqrt{|A||B|}.$$

In words: the number of edges between $A$ and $B$ is close to what you'd expect in a random $d$-regular graph (which is $d|A||B|/n$), with error controlled by $\lambda_2$. The smaller $\lambda_2$, the more "random-like" the graph.

**Information-Theoretic Interpretation:** Random walks on expanders mix rapidly (in $O(\log n)$ steps), meaning information propagates quickly. The spectral gap controls the mixing time: $\tau_{\text{mix}} = O(\log n / \lambda)$.

The mixing time $O(\log n / \lambda)$ means that after $O(\log n)$ steps of a random walk, the distribution over vertices is close to uniform, regardless of the starting point. This is the key property for applications: an expander rapidly "forgets" where you started, which is the information-theoretic signature of fast entropy production and fast information diffusion.

## 26.4.2 Pseudorandom Generators and Extractors

The connection between expanders and pseudorandomness is deep.

**Definition 26.4.3.** A *pseudorandom generator* $G: \{0,1\}^s \to \{0,1\}^n$ is a function such that $G(U_s)$ is computationally indistinguishable from $U_n$ (where $U_k$ denotes uniform on $\{0,1\}^k$).

A pseudorandom generator stretches a short random seed into a long pseudorandom string. The key property is computational indistinguishability: no efficient algorithm can tell $G(U_s)$ from a truly uniform string. This is different from information-theoretic indistinguishability (which would require $n = s$) — it's the computational version.

**Definition 26.4.4 (Extractor).** An $(k, \varepsilon)$-*extractor* is a function $\text{Ext}: \{0,1\}^n \times \{0,1\}^d \to \{0,1\}^m$ such that for any source $X$ with min-entropy $H_\infty(X) \geq k$:
$$\|\text{Ext}(X, U_d) - U_m\|_1 \leq \varepsilon.$$

An extractor takes a "weak" random source $X$ (one with min-entropy $k$, meaning every outcome has probability $\leq 2^{-k}$) and a short uniform seed $U_d$, and produces $m$ nearly-uniform bits. This is information-theoretically non-trivial: the source $X$ has $k$ bits of "real" randomness buried in the $n$ bits, and the extractor finds and outputs it.

**Theorem 26.4.5 (Expanders as Extractors).** Expander graphs yield near-optimal randomness extractors. A random walk of length $t$ on a $d$-regular $\lambda_2$-expander starting from a high-entropy source extracts $\approx t \log d - O(t\lambda_2/d)$ nearly-uniform bits.

The idea: take a random starting vertex (drawn from the high-entropy source), perform a random walk of length $t$, and output the sequence of edge labels traversed. The expander mixing lemma ensures that the distribution of the walk mixes rapidly, so after $O(\log n)$ steps the walk is nearly uniform over all $d^t$ possible edge-label sequences. The spectral gap controls the approximation error.

This connection is beautiful: expanders allow you to turn "weak" randomness into "good" randomness, at the cost of a short uniform seed (to drive the random walk). The information-theoretic content: the high-entropy source provides the "location" of the walk, while the expander's fast mixing ensures the walk explores the graph almost uniformly, extracting nearly-uniform bits.

Explicit constructions of expanders (the Ramanujan graphs of Lubotzky-Phillips-Sarnak, the Cayley graphs of $SL_2(\mathbb{F}_p)$) yield explicit extractors. The existence of Ramanujan graphs — expanders achieving the Alon-Boppana bound $\lambda_2 \geq 2\sqrt{d-1}$ — is proved using algebraic number theory. The best possible expanders are algebraic objects, and their information-theoretic applications are the practical payoff.
