# 12.3 Subshifts of Finite Type

The simplest and most important class of subshifts is defined by a *finite* list of forbidden words. These are the subshifts of finite type (SFTs), and they have a remarkably clean algebraic structure: every SFT is equivalent to a directed graph, and all of its dynamical properties — entropy, periodic orbit counts, classification — are encoded in the adjacency matrix of that graph.

**Definition 12.3.1.** A *subshift of finite type (SFT)* is a subshift defined by a finite set of forbidden words $\mathcal{F}$. Equivalently, after converting to the "higher block" representation (where each symbol represents a window of length $M$, for $M$ the maximum length of a forbidden word), an SFT can always be described by forbidden words of length exactly 2 — that is, by a *transition rule* on pairs of consecutive symbols.

**Theorem 12.3.2 (Vertex SFT).** Every SFT is conjugate (via a sliding block code) to a *vertex SFT*: given by a finite directed graph $G = (V, E)$ where the allowed sequences are exactly the bi-infinite paths in $G$. Explicitly:
$$X_G = \{(x_n)_{n \in \mathbb{Z}} : (x_n, x_{n+1}) \in E \text{ for all } n\}.$$

The *transition matrix* $A \in M_{|V|}(\{0,1\})$ records which transitions are allowed: $A_{ij} = 1$ iff $(i,j) \in E$.

What this is saying is: the entire combinatorial complexity of an SFT is encoded in a single 0-1 matrix. The allowed sequences are exactly the paths in the directed graph with adjacency matrix $A$. The matrix is the system.

**Example 12.3.3 (Golden Mean Shift).** Take $\mathcal{A} = \{0,1\}$ with the single forbidden word $\{11\}$ (no two consecutive 1s). This is the *golden mean shift*, and its transition matrix is:
$$A = \begin{pmatrix} 1 & 1 \\ 1 & 0 \end{pmatrix}.$$

Reading the matrix: from state 0, you can go to state 0 or state 1 ($A_{00} = A_{01} = 1$). From state 1, you can only go to state 0 ($A_{10} = 1$, $A_{11} = 0$ — since 11 is forbidden). The Perron-Frobenius eigenvalue of $A$ is $\lambda_+ = (1+\sqrt{5})/2$ — the golden ratio.

Why the golden ratio? Count the allowed words of length $n$. The number of such words satisfies the Fibonacci recurrence $p(n) = p(n-1) + p(n-2)$, and the growth rate is $\lim p(n)^{1/n} = \phi$. The golden ratio appears because Fibonacci growth is forced by the no-consecutive-1s constraint. The entropy is $h_{\text{top}} = \log \phi$.

This example illustrates the general principle: the structure of an SFT is algebraically transparent, and the Perron-Frobenius theorem — which describes the eigenvalue structure of non-negative matrices — is the key computational tool. We use it extensively in the next section.
