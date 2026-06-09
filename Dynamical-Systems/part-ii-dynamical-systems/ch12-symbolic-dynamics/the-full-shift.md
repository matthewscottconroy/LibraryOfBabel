# 12.1 The Full Shift

The starting point for symbolic dynamics is not a theorem — it is a setup. We need to define the basic space and the basic map, and understand their elementary properties. Once we have done that, everything else is built on top.

The idea is simple: take a finite alphabet $\mathcal{A}$ and consider all doubly-infinite sequences of symbols from $\mathcal{A}$. These sequences are our "phase space." The dynamics is given by the left-shift operator: slide the sequence one step to the left. This is the full shift.

**Definition 12.1.1.** Let $\mathcal{A}$ be a finite *alphabet* (e.g., $\mathcal{A} = \{0, 1, \ldots, k-1\}$). The *full shift* on $\mathcal{A}$ is:
$$\mathcal{A}^{\mathbb Z} = \{(x_n)_{n \in \mathbb{Z}} : x_n \in \mathcal{A}\} \quad \text{with the shift map } \sigma: \mathcal{A}^{\mathbb Z} \to \mathcal{A}^{\mathbb Z}, \quad \sigma(x)_n = x_{n+1}.$$

The *product topology* on $\mathcal{A}^{\mathbb Z}$ (where $\mathcal{A}$ has the discrete topology) is metrizable by:
$$d(x, y) = 2^{-\min\{|n| : x_n \neq y_n\}}.$$

The metric says: two sequences are close if they agree on a long central block. Sequences that agree at positions $-N, \ldots, N$ are at distance at most $2^{-N}$. This is the right topology for dynamics: nearby sequences are those with similar near-term behavior.

**Properties of $(\mathcal{A}^{\mathbb Z}, \sigma)$:**

- $\mathcal{A}^{\mathbb Z}$ is compact (by Tychonoff's theorem), perfect (no isolated points), and totally disconnected (any two distinct points can be separated by clopen sets). For $|\mathcal{A}| \geq 2$, this means $\mathcal{A}^{\mathbb Z}$ is homeomorphic to the Cantor set.
- $\sigma$ is a homeomorphism (it has the inverse $\sigma^{-1}(x)_n = x_{n-1}$).
- $(\mathcal{A}^{\mathbb Z}, \sigma)$ is topologically mixing: for any two nonempty open sets $U, V$, there exists $N$ such that $\sigma^n(U) \cap V \neq \emptyset$ for all $n \geq N$.
- The periodic points $\{x : \sigma^n(x) = x\}$ are dense (periodic sequences are dense in the product topology). There are exactly $k^n$ periodic points of period $n$ (or dividing $n$), where $k = |\mathcal{A}|$.
- The topological entropy is $h_{\text{top}}(\sigma) = \log |\mathcal{A}|$ — which makes intuitive sense: at each step, the sequence "chooses" from $k$ symbols, contributing $\log k$ bits of new information.

The full shift is chaotic in every sense of Definition 11.1: it is topologically mixing (stronger than transititive), has dense periodic points, and has sensitive dependence. It is the "maximally chaotic" symbolic system.

Why doubly-infinite sequences $\mathcal{A}^{\mathbb Z}$ rather than one-sided sequences $\mathcal{A}^{\mathbb N}$? For invertibility: the shift on $\mathcal{A}^{\mathbb Z}$ is a homeomorphism, while the shift on $\mathcal{A}^{\mathbb N}$ is not invertible (information is lost). When our underlying dynamical system is invertible (a diffeomorphism), the two-sided shift is the right model. One-sided shifts appear when we model non-invertible maps.

In the next section, we close the full shift down: by imposing constraints on which sequences are allowed, we get the rich zoo of subshifts.
