# 9.5 Markov Partitions

The horseshoe was described by a binary sequence — the sequence of visits to $H_0$ or $H_1$. We want to do the same for general Anosov diffeomorphisms: code the orbits by symbolic sequences in a way that captures the dynamics completely. The tool is a Markov partition.

**Definition 9.5.1.** A *Markov partition* of a hyperbolic set $\Lambda$ is a finite cover $\mathcal{R} = \{R_1, \ldots, R_k\}$ by "rectangles" (sets that are products of stable and unstable manifold pieces) with:
1. $R_i = \overline{\text{int}(R_i)}$ and $\text{int}(R_i) \cap \text{int}(R_j) = \emptyset$ for $i \neq j$
2. The Markov property: if $x \in \text{int}(R_i)$ and $f(x) \in \text{int}(R_j)$, then $f(W^u_{\text{loc}}(x) \cap R_i) \supseteq W^u_{\text{loc}}(f(x)) \cap R_j$

The Markov property is the key condition. It says: if a point in the interior of $R_i$ maps to the interior of $R_j$, then the entire unstable disk of $x$ within $R_i$ maps across the entire unstable disk of $f(x)$ within $R_j$. This "stretching across" property ensures that the symbolic coding is consistent — if your coding says "go from $R_i$ to $R_j$," then the full unstable disk in $R_i$ can make that transition.

**Theorem 9.5.2 (Sinai, Bowen).** Every Anosov diffeomorphism (and every hyperbolic attractor) has a Markov partition.

Sinai proved this in 1968, Bowen extended it in 1975. The existence of Markov partitions is a non-trivial theorem — the construction is delicate because the rectangles must be compatible with the (generally non-smooth) stable and unstable foliations.

**The Transition Matrix:** Define $A_{ij} = 1$ if $f(\text{int}(R_i)) \cap \text{int}(R_j) \neq \emptyset$, else $0$. The associated *subshift of finite type* $\Sigma_A \subseteq \{1,\ldots,k\}^{\mathbb Z}$ codes the dynamics: the coding map $\pi: \Lambda \to \Sigma_A$ defined by $\pi(x)_n = i$ iff $f^n(x) \in R_i$ is almost surjective (bijective on a residual set).

The transition matrix $A$ captures which rectangle-to-rectangle transitions are possible: $A_{ij} = 1$ means you can go from $R_i$ to $R_j$ in one step. The subshift $\Sigma_A$ consists of all bi-infinite sequences that only make allowed transitions — the set of "legal" symbolic orbits.

**Theorem 9.5.3.** For an Anosov diffeomorphism $f$ with Markov partition $\mathcal{R}$ and transition matrix $A$:
$$h_{\text{top}}(f) = \log \lambda_{\text{PF}}(A)$$
where $\lambda_{\text{PF}}(A)$ is the Perron-Frobenius eigenvalue of $A$.

This is the entropy formula in disguise. The Perron-Frobenius eigenvalue of the transition matrix measures the exponential growth rate of the number of allowed symbol sequences, which equals the exponential growth rate of the number of periodic orbits of length $n$ — which is the topological entropy.

For the horseshoe, the transition matrix is $A = \begin{pmatrix} 1 & 1 \\ 1 & 1 \end{pmatrix}$, with Perron-Frobenius eigenvalue 2. So $h_{\text{top}} = \log 2$, exactly as expected.
